// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $039B58: lea.l $39b6a(pc), a0
pub fn i_039b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39B5C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39B6A;
    16
}
/// $039B5C: movea.l (a0, d0.w), a0
pub fn i_039b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39B60;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039B60: jsr $28cd4.l
pub fn i_039b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39B66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39B66);
    cpu.pc = 0x28CD4;
    16
}
/// $039B66: bra.w $39b92
pub fn i_039b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39B6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x39B92;
    10
}
/// $039B92: rts
pub fn i_039b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39B94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Stand_039e54  ($039E54, 62 B, player/arm_weapon_fx_0388f0.s)
/// $039E54: bclr.b #$2, $8c(a6)
pub fn i_039e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E5A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039E5A: movea.l $c(a6), a0
pub fn i_039e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E5E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039E5E: cmpi.w #$1, $72(a0)
pub fn i_039e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E64;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $039E64: bne.w $39e6c
pub fn i_039e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E68;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x39E6C; return 10; }
    8
}
/// $039E68: jmp $39f56(pc)
pub fn i_039e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E6C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x39F56;
    8
}
/// $039E6C: cmpi.b #$41, $79(a0)
pub fn i_039e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E72;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x41, Size::Byte, false);
    16
}
/// $039E72: bne.w $39ebe
pub fn i_039e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x39EBE; return 10; }
    8
}
/// $039E76: bclr.b #$2, $8c(a6)
pub fn i_039e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039E7C: jsr $3933a(pc)
pub fn i_039e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E80;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39E80);
    cpu.pc = 0x3933A;
    16
}
/// $039E80: lea.l $39e92(pc), a0
pub fn i_039e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E84;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39E92;
    16
}
/// $039E84: movea.l (a0, d0.w), a0
pub fn i_039e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E88;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039E88: jsr $28cd4.l
pub fn i_039e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39E8E);
    cpu.pc = 0x28CD4;
    16
}
/// $039E8E: bra.w $39eba
pub fn i_039e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E92;
    cpu.ir = 0x6000;
    cpu.pc = 0x39EBA;
    10
}
/// $039EBA: bra.w $39f54
pub fn i_039eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39EBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x39F54;
    10
}
/// $039EBE: jsr $2abcc.l
pub fn i_039ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39EC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39EC4);
    cpu.pc = 0x2ABCC;
    16
}
/// $039EC4: bcs.w $39f10
pub fn i_039ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39EC8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x39F10; return 10; }
    8
}
/// $039EC8: bclr.b #$2, $8c(a6)
pub fn i_039ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39ECE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039ECE: jsr $3933a(pc)
pub fn i_039ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39ED2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39ED2);
    cpu.pc = 0x3933A;
    16
}
/// $039ED2: lea.l $39ee4(pc), a0
pub fn i_039ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39ED6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39EE4;
    16
}
/// $039ED6: movea.l (a0, d0.w), a0
pub fn i_039ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39EDA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039EDA: jsr $28cd4.l
pub fn i_039eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39EE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39EE0);
    cpu.pc = 0x28CD4;
    16
}
/// $039EE0: bra.w $39f0c
pub fn i_039ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39EE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x39F0C;
    10
}
/// $039F0C: bra.w $39f54
pub fn i_039f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F10;
    cpu.ir = 0x6000;
    cpu.pc = 0x39F54;
    10
}
/// $039F10: bclr.b #$2, $8c(a6)
pub fn i_039f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F16;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039F16: jsr $3933a(pc)
pub fn i_039f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39F1A);
    cpu.pc = 0x3933A;
    16
}
/// $039F1A: lea.l $39f2c(pc), a0
pub fn i_039f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F1E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39F2C;
    16
}
/// $039F1E: movea.l (a0, d0.w), a0
pub fn i_039f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F22;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039F22: jsr $28cd4.l
pub fn i_039f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39F28);
    cpu.pc = 0x28CD4;
    16
}
/// $039F28: bra.w $39f54
pub fn i_039f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x39F54;
    10
}
/// $039F54: rts
pub fn i_039f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_StandAlt_039f56  ($039F56, 62 B, player/arm_weapon_fx_0388f0.s)
/// $039F56: bclr.b #$2, $8c(a6)
pub fn i_039f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F5C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039F5C: movea.l $c(a6), a0
pub fn i_039f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039F60: cmpi.w #$0, $72(a0)
pub fn i_039f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F66;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $039F66: bne.w $39f6e
pub fn i_039f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F6A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x39F6E; return 10; }
    8
}
/// $039F6A: jmp $39e54(pc)
pub fn i_039f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F6E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x39E54;
    8
}
/// $039F6E: cmpi.b #$14, $79(a0)
pub fn i_039f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F74;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}
/// $039F74: bne.w $39fc0
pub fn i_039f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x39FC0; return 10; }
    8
}
/// $039F78: bclr.b #$2, $8c(a6)
pub fn i_039f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F7E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039F7E: jsr $3933a(pc)
pub fn i_039f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39F82);
    cpu.pc = 0x3933A;
    16
}
/// $039F82: lea.l $39f94(pc), a0
pub fn i_039f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F86;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39F94;
    16
}
/// $039F86: movea.l (a0, d0.w), a0
pub fn i_039f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F8A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039F8A: jsr $28cd4.l
pub fn i_039f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39F90);
    cpu.pc = 0x28CD4;
    16
}
/// $039F90: bra.w $39fbc
pub fn i_039f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39F94;
    cpu.ir = 0x6000;
    cpu.pc = 0x39FBC;
    10
}
/// $039FBC: bra.w $3a056
pub fn i_039fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A056;
    10
}
/// $039FC0: jsr $2abcc.l
pub fn i_039fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39FC6);
    cpu.pc = 0x2ABCC;
    16
}
/// $039FC6: bcs.w $3a012
pub fn i_039fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FCA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3A012; return 10; }
    8
}
/// $039FCA: bclr.b #$2, $8c(a6)
pub fn i_039fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FD0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039FD0: jsr $3933a(pc)
pub fn i_039fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39FD4);
    cpu.pc = 0x3933A;
    16
}
/// $039FD4: lea.l $39fe6(pc), a0
pub fn i_039fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FD8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39FE6;
    16
}
/// $039FD8: movea.l (a0, d0.w), a0
pub fn i_039fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FDC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039FDC: jsr $28cd4.l
pub fn i_039fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39FE2);
    cpu.pc = 0x28CD4;
    16
}
/// $039FE2: bra.w $3a00e
pub fn i_039fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39FE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A00E;
    10
}
/// $03A00E: bra.w $3a056
pub fn i_03a00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A012;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A056;
    10
}
/// $03A012: bclr.b #$2, $8c(a6)
pub fn i_03a012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A018;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A018: jsr $3933a(pc)
pub fn i_03a018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A01C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A01C);
    cpu.pc = 0x3933A;
    16
}
/// $03A01C: lea.l $3a02e(pc), a0
pub fn i_03a01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A020;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A02E;
    16
}
/// $03A020: movea.l (a0, d0.w), a0
pub fn i_03a020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A024;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A024: jsr $28cd4.l
pub fn i_03a024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A02A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A02A);
    cpu.pc = 0x28CD4;
    16
}
/// $03A02A: bra.w $3a056
pub fn i_03a02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A02E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A056;
    10
}
/// $03A056: rts
pub fn i_03a056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A058;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Walk_03a058  ($03A058, 62 B, player/arm_weapon_fx_0388f0.s)
/// $03A058: bclr.b #$2, $8c(a6)
pub fn i_03a058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A05E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A05E: movea.l $c(a6), a0
pub fn i_03a05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A062;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03A062: cmpi.w #$1, $72(a0)
pub fn i_03a062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A068;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03A068: bne.w $3a070
pub fn i_03a068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A06C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A070; return 10; }
    8
}
/// $03A06C: jmp $3a108(pc)
pub fn i_03a06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A070;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3A108;
    8
}
/// $03A070: cmpi.b #$41, $79(a0)
pub fn i_03a070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A076;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x41, Size::Byte, false);
    16
}
/// $03A076: bne.w $3a0c2
pub fn i_03a076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A07A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A0C2; return 10; }
    8
}
/// $03A07A: bclr.b #$2, $8c(a6)
pub fn i_03a07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A080;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A080: jsr $3933a(pc)
pub fn i_03a080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A084;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A084);
    cpu.pc = 0x3933A;
    16
}
/// $03A084: lea.l $3a096(pc), a0
pub fn i_03a084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A088;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A096;
    16
}
/// $03A088: movea.l (a0, d0.w), a0
pub fn i_03a088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A08C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A08C: jsr $28cd4.l
pub fn i_03a08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A092);
    cpu.pc = 0x28CD4;
    16
}
/// $03A092: bra.w $3a0be
pub fn i_03a092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A096;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A0BE;
    10
}
/// $03A0BE: bra.w $3a106
pub fn i_03a0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A0C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A106;
    10
}
/// $03A0C2: bclr.b #$2, $8c(a6)
pub fn i_03a0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A0C8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A0C8: jsr $3933a(pc)
pub fn i_03a0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A0CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A0CC);
    cpu.pc = 0x3933A;
    16
}
/// $03A0CC: lea.l $3a0de(pc), a0
pub fn i_03a0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A0D0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A0DE;
    16
}
/// $03A0D0: movea.l (a0, d0.w), a0
pub fn i_03a0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A0D4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A0D4: jsr $28cd4.l
pub fn i_03a0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A0DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A0DA);
    cpu.pc = 0x28CD4;
    16
}
/// $03A0DA: bra.w $3a106
pub fn i_03a0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A0DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A106;
    10
}
/// $03A106: rts
pub fn i_03a106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A108;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WalkAlt_03a108  ($03A108, 62 B, player/arm_weapon_fx_0388f0.s)
/// $03A108: bclr.b #$2, $8c(a6)
pub fn i_03a108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A10E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A10E: movea.l $c(a6), a0
pub fn i_03a10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A112;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03A112: cmpi.w #$0, $72(a0)
pub fn i_03a112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A118;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03A118: bne.w $3a120
pub fn i_03a118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A11C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A120; return 10; }
    8
}
/// $03A11C: jmp $3a058(pc)
pub fn i_03a11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A120;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3A058;
    8
}
/// $03A120: cmpi.b #$14, $79(a0)
pub fn i_03a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A126;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}
/// $03A126: bne.w $3a172
pub fn i_03a126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A12A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A172; return 10; }
    8
}
/// $03A12A: bclr.b #$2, $8c(a6)
pub fn i_03a12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A130;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A130: jsr $3933a(pc)
pub fn i_03a130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A134;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A134);
    cpu.pc = 0x3933A;
    16
}
/// $03A134: lea.l $3a146(pc), a0
pub fn i_03a134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A138;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A146;
    16
}
/// $03A138: movea.l (a0, d0.w), a0
pub fn i_03a138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A13C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A13C: jsr $28cd4.l
pub fn i_03a13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A142;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A142);
    cpu.pc = 0x28CD4;
    16
}
/// $03A142: bra.w $3a16e
pub fn i_03a142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A146;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A16E;
    10
}
/// $03A16E: bra.w $3a1b6
pub fn i_03a16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A172;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A1B6;
    10
}
/// $03A172: bclr.b #$2, $8c(a6)
pub fn i_03a172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A178;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A178: jsr $3933a(pc)
pub fn i_03a178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A17C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A17C);
    cpu.pc = 0x3933A;
    16
}
/// $03A17C: lea.l $3a18e(pc), a0
pub fn i_03a17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A180;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A18E;
    16
}
/// $03A180: movea.l (a0, d0.w), a0
pub fn i_03a180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A184;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A184: jsr $28cd4.l
pub fn i_03a184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A18A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A18A);
    cpu.pc = 0x28CD4;
    16
}
/// $03A18A: bra.w $3a1b6
pub fn i_03a18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A18E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A1B6;
    10
}
/// $03A1B6: rts
pub fn i_03a1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WalkLoop_03a1b8  ($03A1B8, 62 B, player/arm_weapon_fx_0388f0.s)
/// $03A1B8: bclr.b #$2, $8c(a6)
pub fn i_03a1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1BE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A1BE: movea.l $c(a6), a0
pub fn i_03a1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1C2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03A1C2: cmpi.w #$1, $72(a0)
pub fn i_03a1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1C8;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03A1C8: bne.w $3a1d0
pub fn i_03a1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A1D0; return 10; }
    8
}
/// $03A1CC: jmp $3a268(pc)
pub fn i_03a1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1D0;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3A268;
    8
}
/// $03A1D0: cmpi.b #$41, $79(a0)
pub fn i_03a1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1D6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x41, Size::Byte, false);
    16
}
/// $03A1D6: bne.w $3a222
pub fn i_03a1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A222; return 10; }
    8
}
/// $03A1DA: bclr.b #$2, $8c(a6)
pub fn i_03a1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1E0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A1E0: jsr $3933a(pc)
pub fn i_03a1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A1E4);
    cpu.pc = 0x3933A;
    16
}
/// $03A1E4: lea.l $3a1f6(pc), a0
pub fn i_03a1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1E8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A1F6;
    16
}
/// $03A1E8: movea.l (a0, d0.w), a0
pub fn i_03a1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1EC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A1EC: jsr $28cd4.l
pub fn i_03a1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A1F2);
    cpu.pc = 0x28CD4;
    16
}
/// $03A1F2: bra.w $3a21e
pub fn i_03a1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A1F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A21E;
    10
}
/// $03A21E: bra.w $3a266
pub fn i_03a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A222;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A266;
    10
}
/// $03A222: bclr.b #$2, $8c(a6)
pub fn i_03a222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A228;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A228: jsr $3933a(pc)
pub fn i_03a228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A22C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A22C);
    cpu.pc = 0x3933A;
    16
}
/// $03A22C: lea.l $3a23e(pc), a0
pub fn i_03a22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A230;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A23E;
    16
}
/// $03A230: movea.l (a0, d0.w), a0
pub fn i_03a230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A234;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A234: jsr $28cd4.l
pub fn i_03a234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A23A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A23A);
    cpu.pc = 0x28CD4;
    16
}
/// $03A23A: bra.w $3a266
pub fn i_03a23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A23E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A266;
    10
}
/// $03A266: rts
pub fn i_03a266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A268;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WalkLoopAlt_03a268  ($03A268, 62 B, player/arm_weapon_fx_0388f0.s)
/// $03A268: bclr.b #$2, $8c(a6)
pub fn i_03a268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A26E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A26E: movea.l $c(a6), a0
pub fn i_03a26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A272;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03A272: cmpi.w #$0, $72(a0)
pub fn i_03a272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A278;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03A278: bne.w $3a280
pub fn i_03a278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A27C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A280; return 10; }
    8
}
/// $03A27C: jmp $3a1b8(pc)
pub fn i_03a27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A280;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3A1B8;
    8
}
/// $03A280: cmpi.b #$14, $79(a0)
pub fn i_03a280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A286;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}
/// $03A286: bne.w $3a2d2
pub fn i_03a286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A28A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A2D2; return 10; }
    8
}
/// $03A28A: bclr.b #$2, $8c(a6)
pub fn i_03a28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A290;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A290: jsr $3933a(pc)
pub fn i_03a290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A294;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A294);
    cpu.pc = 0x3933A;
    16
}
/// $03A294: lea.l $3a2a6(pc), a0
pub fn i_03a294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A298;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A2A6;
    16
}
/// $03A298: movea.l (a0, d0.w), a0
pub fn i_03a298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A29C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A29C: jsr $28cd4.l
pub fn i_03a29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A2A2);
    cpu.pc = 0x28CD4;
    16
}
/// $03A2A2: bra.w $3a2ce
pub fn i_03a2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A2CE;
    10
}
/// $03A2CE: bra.w $3a316
pub fn i_03a2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A316;
    10
}
/// $03A2D2: bclr.b #$2, $8c(a6)
pub fn i_03a2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2D8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A2D8: jsr $3933a(pc)
pub fn i_03a2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A2DC);
    cpu.pc = 0x3933A;
    16
}
/// $03A2DC: lea.l $3a2ee(pc), a0
pub fn i_03a2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2E0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A2EE;
    16
}
/// $03A2E0: movea.l (a0, d0.w), a0
pub fn i_03a2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2E4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A2E4: jsr $28cd4.l
pub fn i_03a2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A2EA);
    cpu.pc = 0x28CD4;
    16
}
/// $03A2EA: bra.w $3a316
pub fn i_03a2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A2EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A316;
    10
}
/// $03A316: rts
pub fn i_03a316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A318;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WalkShootB_03a35e  ($03A35E, 28 B, player/arm_weapon_fx_0388f0.s)
/// $03A35E: bclr.b #$2, $8c(a6)
pub fn i_03a35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A364;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A364: jsr $3933a(pc)
pub fn i_03a364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A368;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A368);
    cpu.pc = 0x3933A;
    16
}
/// $03A368: lea.l $3a37a(pc), a0
pub fn i_03a368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A36C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A37A;
    16
}
/// $03A36C: movea.l (a0, d0.w), a0
pub fn i_03a36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A370;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A370: jsr $28cd4.l
pub fn i_03a370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A376);
    cpu.pc = 0x28CD4;
    16
}
/// $03A376: bra.w $3a3a2
pub fn i_03a376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A37A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A3A2;
    10
}
/// $03A3A2: rts
pub fn i_03a3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_TurnA_03a4c8  ($03A4C8, 34 B, player/arm_weapon_fx_0388f0.s)
/// $03A4C8: bclr.b #$2, $8c(a6)
pub fn i_03a4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A4CE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A4CE: bclr.b #$2, $8c(a6)
pub fn i_03a4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A4D4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A4D4: jsr $3933a(pc)
pub fn i_03a4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A4D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A4D8);
    cpu.pc = 0x3933A;
    16
}
/// $03A4D8: lea.l $3a4ea(pc), a0
pub fn i_03a4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A4DC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A4EA;
    16
}
/// $03A4DC: movea.l (a0, d0.w), a0
pub fn i_03a4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A4E0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A4E0: jsr $28cd4.l
pub fn i_03a4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A4E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A4E6);
    cpu.pc = 0x28CD4;
    16
}
/// $03A4E6: bra.w $3a512
pub fn i_03a4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A4EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A512;
    10
}
/// $03A512: rts
pub fn i_03a512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_JumpMelee_03a560  ($03A560, 42 B, player/arm_weapon_fx_0388f0.s)
/// $03A560: addq.b #$1, $30(a6)
pub fn i_03a560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A564;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03A564: btst.b #$0, $30(a6)
pub fn i_03a564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A56A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03A56A: bne.w $3a5b6
pub fn i_03a56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A56E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A5B6; return 10; }
    8
}
/// $03A56E: bclr.b #$2, $8c(a6)
pub fn i_03a56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A574;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A574: jsr $3933a(pc)
pub fn i_03a574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A578;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A578);
    cpu.pc = 0x3933A;
    16
}
/// $03A578: lea.l $3a58a(pc), a0
pub fn i_03a578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A57C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A58A;
    16
}
/// $03A57C: movea.l (a0, d0.w), a0
pub fn i_03a57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A580;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A580: jsr $28cd4.l
pub fn i_03a580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A586;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A586);
    cpu.pc = 0x28CD4;
    16
}
/// $03A586: bra.w $3a5b2
pub fn i_03a586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A58A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A5B2;
    10
}
/// $03A5B2: bra.w $3a5fa
pub fn i_03a5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A5B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A5FA;
    10
}
/// $03A5B6: bclr.b #$2, $8c(a6)
pub fn i_03a5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A5BC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A5BC: jsr $3933a(pc)
pub fn i_03a5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A5C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A5C0);
    cpu.pc = 0x3933A;
    16
}
/// $03A5C0: lea.l $3a5d2(pc), a0
pub fn i_03a5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A5C4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A5D2;
    16
}
/// $03A5C4: movea.l (a0, d0.w), a0
pub fn i_03a5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A5C8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A5C8: jsr $28cd4.l
pub fn i_03a5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A5CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A5CE);
    cpu.pc = 0x28CD4;
    16
}
/// $03A5CE: bra.w $3a5fa
pub fn i_03a5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A5D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A5FA;
    10
}
/// $03A5FA: lea.l $392a4(pc), a0
pub fn i_03a5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A5FE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x392A4;
    16
}
/// $03A5FE: move.l a0, $4c(a6)
pub fn i_03a5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A602;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03A602: jsr $283ca.l
pub fn i_03a602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A608;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A608);
    cpu.pc = 0x283CA;
    16
}
/// $03A608: rts
pub fn i_03a608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A60A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Jump_03a60a  ($03A60A, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03A60A: bclr.b #$2, $8c(a6)
pub fn i_03a60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A610;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A610: bclr.b #$2, $8c(a6)
pub fn i_03a610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A616;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A616: jsr $3933a(pc)
pub fn i_03a616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A61A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A61A);
    cpu.pc = 0x3933A;
    16
}
/// $03A61A: lea.l $3a62c(pc), a0
pub fn i_03a61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A61E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A62C;
    16
}
/// $03A61E: movea.l (a0, d0.w), a0
pub fn i_03a61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A622;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A622: jsr $28cd4.l
pub fn i_03a622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A628;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A628);
    cpu.pc = 0x28CD4;
    16
}
/// $03A628: bra.w $3a654
pub fn i_03a628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A62C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A654;
    10
}
/// $03A654: rts
pub fn i_03a654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A656;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Fall_03a656  ($03A656, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03A656: bclr.b #$2, $8c(a6)
pub fn i_03a656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A65C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A65C: bclr.b #$2, $8c(a6)
pub fn i_03a65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A662;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A662: jsr $3933a(pc)
pub fn i_03a662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A666;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A666);
    cpu.pc = 0x3933A;
    16
}
/// $03A666: lea.l $3a678(pc), a0
pub fn i_03a666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A66A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A678;
    16
}
/// $03A66A: movea.l (a0, d0.w), a0
pub fn i_03a66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A66E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A66E: jsr $28cd4.l
pub fn i_03a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A674;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A674);
    cpu.pc = 0x28CD4;
    16
}
/// $03A674: bra.w $3a6a0
pub fn i_03a674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A678;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A6A0;
    10
}
/// $03A6A0: rts
pub fn i_03a6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_JumpShootA_03aae0  ($03AAE0, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03AAE0: cmpi.w #$1, $72(a6)
pub fn i_03aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAE6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03AAE6: bne.w $3ab32
pub fn i_03aae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AB32; return 10; }
    8
}
/// $03AAEA: bclr.b #$2, $8c(a6)
pub fn i_03aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAF0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AAF0: jsr $3933a(pc)
pub fn i_03aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AAF4);
    cpu.pc = 0x3933A;
    16
}
/// $03AAF4: lea.l $3ab06(pc), a0
pub fn i_03aaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAF8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AB06;
    16
}
/// $03AAF8: movea.l (a0, d0.w), a0
pub fn i_03aaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAFC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AAFC: jsr $28cd4.l
pub fn i_03aafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AB02);
    cpu.pc = 0x28CD4;
    16
}
/// $03AB02: bra.w $3ab2e
pub fn i_03ab02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB06;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AB2E;
    10
}
/// $03AB2E: bra.w $3ac1a
pub fn i_03ab2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB32;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AC1A;
    10
}
/// $03AB32: cmpi.w #$2, $72(a6)
pub fn i_03ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB38;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03AB38: bne.w $3ab84
pub fn i_03ab38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AB84; return 10; }
    8
}
/// $03AB3C: bclr.b #$2, $8c(a6)
pub fn i_03ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB42;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AB42: jsr $3933a(pc)
pub fn i_03ab42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AB46);
    cpu.pc = 0x3933A;
    16
}
/// $03AB46: lea.l $3ab58(pc), a0
pub fn i_03ab46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB4A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AB58;
    16
}
/// $03AB4A: movea.l (a0, d0.w), a0
pub fn i_03ab4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB4E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AB4E: jsr $28cd4.l
pub fn i_03ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AB54);
    cpu.pc = 0x28CD4;
    16
}
/// $03AB54: bra.w $3ab80
pub fn i_03ab54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB58;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AB80;
    10
}
/// $03AB80: bra.w $3ac1a
pub fn i_03ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB84;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AC1A;
    10
}
/// $03AB84: cmpi.w #$3, $72(a6)
pub fn i_03ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB8A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03AB8A: bne.w $3abd6
pub fn i_03ab8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3ABD6; return 10; }
    8
}
/// $03AB8E: bclr.b #$2, $8c(a6)
pub fn i_03ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB94;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AB94: jsr $3933a(pc)
pub fn i_03ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AB98);
    cpu.pc = 0x3933A;
    16
}
/// $03AB98: lea.l $3abaa(pc), a0
pub fn i_03ab98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AB9C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3ABAA;
    16
}
/// $03AB9C: movea.l (a0, d0.w), a0
pub fn i_03ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABA0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03ABA0: jsr $28cd4.l
pub fn i_03aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ABA6);
    cpu.pc = 0x28CD4;
    16
}
/// $03ABA6: bra.w $3abd2
pub fn i_03aba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3ABD2;
    10
}
/// $03ABD2: bra.w $3ac1a
pub fn i_03abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AC1A;
    10
}
/// $03ABD6: bclr.b #$2, $8c(a6)
pub fn i_03abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABDC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03ABDC: jsr $3933a(pc)
pub fn i_03abdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3ABE0);
    cpu.pc = 0x3933A;
    16
}
/// $03ABE0: lea.l $3abf2(pc), a0
pub fn i_03abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABE4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3ABF2;
    16
}
/// $03ABE4: movea.l (a0, d0.w), a0
pub fn i_03abe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABE8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03ABE8: jsr $28cd4.l
pub fn i_03abe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ABEE);
    cpu.pc = 0x28CD4;
    16
}
/// $03ABEE: bra.w $3ac1a
pub fn i_03abee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ABF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AC1A;
    10
}
/// $03AC1A: rts
pub fn i_03ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FallShootA_03ac1c  ($03AC1C, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03AC1C: cmpi.w #$1, $72(a6)
pub fn i_03ac1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC22;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03AC22: bne.w $3ac6e
pub fn i_03ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AC6E; return 10; }
    8
}
/// $03AC26: bclr.b #$2, $8c(a6)
pub fn i_03ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC2C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AC2C: jsr $3933a(pc)
pub fn i_03ac2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AC30);
    cpu.pc = 0x3933A;
    16
}
/// $03AC30: lea.l $3ac42(pc), a0
pub fn i_03ac30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC34;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AC42;
    16
}
/// $03AC34: movea.l (a0, d0.w), a0
pub fn i_03ac34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC38;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AC38: jsr $28cd4.l
pub fn i_03ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AC3E);
    cpu.pc = 0x28CD4;
    16
}
/// $03AC3E: bra.w $3ac6a
pub fn i_03ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC42;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AC6A;
    10
}
/// $03AC6A: bra.w $3ad56
pub fn i_03ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AD56;
    10
}
/// $03AC6E: cmpi.w #$2, $72(a6)
pub fn i_03ac6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC74;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03AC74: bne.w $3acc0
pub fn i_03ac74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3ACC0; return 10; }
    8
}
/// $03AC78: bclr.b #$2, $8c(a6)
pub fn i_03ac78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC7E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AC7E: jsr $3933a(pc)
pub fn i_03ac7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AC82);
    cpu.pc = 0x3933A;
    16
}
/// $03AC82: lea.l $3ac94(pc), a0
pub fn i_03ac82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC86;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AC94;
    16
}
/// $03AC86: movea.l (a0, d0.w), a0
pub fn i_03ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC8A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AC8A: jsr $28cd4.l
pub fn i_03ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AC90);
    cpu.pc = 0x28CD4;
    16
}
/// $03AC90: bra.w $3acbc
pub fn i_03ac90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AC94;
    cpu.ir = 0x6000;
    cpu.pc = 0x3ACBC;
    10
}
/// $03ACBC: bra.w $3ad56
pub fn i_03acbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AD56;
    10
}
/// $03ACC0: cmpi.w #$3, $72(a6)
pub fn i_03acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACC6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03ACC6: bne.w $3ad12
pub fn i_03acc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AD12; return 10; }
    8
}
/// $03ACCA: bclr.b #$2, $8c(a6)
pub fn i_03acca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACD0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03ACD0: jsr $3933a(pc)
pub fn i_03acd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3ACD4);
    cpu.pc = 0x3933A;
    16
}
/// $03ACD4: lea.l $3ace6(pc), a0
pub fn i_03acd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACD8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3ACE6;
    16
}
/// $03ACD8: movea.l (a0, d0.w), a0
pub fn i_03acd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACDC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03ACDC: jsr $28cd4.l
pub fn i_03acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ACE2);
    cpu.pc = 0x28CD4;
    16
}
/// $03ACE2: bra.w $3ad0e
pub fn i_03ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ACE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AD0E;
    10
}
/// $03AD0E: bra.w $3ad56
pub fn i_03ad0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD12;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AD56;
    10
}
/// $03AD12: bclr.b #$2, $8c(a6)
pub fn i_03ad12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD18;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AD18: jsr $3933a(pc)
pub fn i_03ad18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AD1C);
    cpu.pc = 0x3933A;
    16
}
/// $03AD1C: lea.l $3ad2e(pc), a0
pub fn i_03ad1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD20;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AD2E;
    16
}
/// $03AD20: movea.l (a0, d0.w), a0
pub fn i_03ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD24;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AD24: jsr $28cd4.l
pub fn i_03ad24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AD2A);
    cpu.pc = 0x28CD4;
    16
}
/// $03AD2A: bra.w $3ad56
pub fn i_03ad2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AD56;
    10
}
/// $03AD56: rts
pub fn i_03ad56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FallShootDownC_03b952  ($03B952, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B952: cmpi.w #$0, $72(a6)
pub fn i_03b952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B958;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B958: bne.w $3b9a4
pub fn i_03b958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B95C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B9A4; return 10; }
    8
}
/// $03B95C: bclr.b #$2, $8c(a6)
pub fn i_03b95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B962;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B962: jsr $3933a(pc)
pub fn i_03b962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B966;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B966);
    cpu.pc = 0x3933A;
    16
}
/// $03B966: lea.l $3b978(pc), a0
pub fn i_03b966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B96A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B978;
    16
}
/// $03B96A: movea.l (a0, d0.w), a0
pub fn i_03b96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B96E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B96E: jsr $28cd4.l
pub fn i_03b96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B974;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B974);
    cpu.pc = 0x28CD4;
    16
}
/// $03B974: bra.w $3b9a0
pub fn i_03b974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B978;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B9A0;
    10
}
/// $03B9A0: bra.w $3ba3a
pub fn i_03b9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BA3A;
    10
}
/// $03B9A4: cmpi.w #$2, $72(a6)
pub fn i_03b9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9AA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03B9AA: bne.w $3b9f6
pub fn i_03b9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B9F6; return 10; }
    8
}
/// $03B9AE: bclr.b #$2, $8c(a6)
pub fn i_03b9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9B4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B9B4: jsr $3933a(pc)
pub fn i_03b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B9B8);
    cpu.pc = 0x3933A;
    16
}
/// $03B9B8: lea.l $3b9ca(pc), a0
pub fn i_03b9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9BC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B9CA;
    16
}
/// $03B9BC: movea.l (a0, d0.w), a0
pub fn i_03b9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9C0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B9C0: jsr $28cd4.l
pub fn i_03b9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B9C6);
    cpu.pc = 0x28CD4;
    16
}
/// $03B9C6: bra.w $3b9f2
pub fn i_03b9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B9F2;
    10
}
/// $03B9F2: bra.w $3ba3a
pub fn i_03b9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BA3A;
    10
}
/// $03B9F6: bclr.b #$2, $8c(a6)
pub fn i_03b9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B9FC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B9FC: jsr $3933a(pc)
pub fn i_03b9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BA00);
    cpu.pc = 0x3933A;
    16
}
/// $03BA00: lea.l $3ba12(pc), a0
pub fn i_03ba00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA04;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BA12;
    16
}
/// $03BA04: movea.l (a0, d0.w), a0
pub fn i_03ba04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA08;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BA08: jsr $28cd4.l
pub fn i_03ba08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BA0E);
    cpu.pc = 0x28CD4;
    16
}
/// $03BA0E: bra.w $3ba3a
pub fn i_03ba0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA12;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BA3A;
    10
}
/// $03BA3A: rts
pub fn i_03ba3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_DeathE_03bd58  ($03BD58, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03BD58: bclr.b #$2, $8c(a6)
pub fn i_03bd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD5E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BD5E: bset.b #$0, $13(a6)
pub fn i_03bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD64;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BD64: bclr.b #$2, $8c(a6)
pub fn i_03bd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD6A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BD6A: jsr $3933a(pc)
pub fn i_03bd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BD6E);
    cpu.pc = 0x3933A;
    16
}
/// $03BD6E: lea.l $3bd80(pc), a0
pub fn i_03bd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD72;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BD80;
    16
}
/// $03BD72: movea.l (a0, d0.w), a0
pub fn i_03bd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD76;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BD76: jsr $28cd4.l
pub fn i_03bd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BD7C);
    cpu.pc = 0x28CD4;
    16
}
/// $03BD7C: bra.w $3bda8
pub fn i_03bd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD80;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BDA8;
    10
}
/// $03BDA8: rts
pub fn i_03bda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_DeathF_03bdaa  ($03BDAA, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03BDAA: bclr.b #$2, $8c(a6)
pub fn i_03bdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDB0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BDB0: bset.b #$0, $13(a6)
pub fn i_03bdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDB6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BDB6: bclr.b #$2, $8c(a6)
pub fn i_03bdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDBC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BDBC: jsr $3933a(pc)
pub fn i_03bdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BDC0);
    cpu.pc = 0x3933A;
    16
}
/// $03BDC0: lea.l $3bdd2(pc), a0
pub fn i_03bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDC4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BDD2;
    16
}
/// $03BDC4: movea.l (a0, d0.w), a0
pub fn i_03bdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDC8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BDC8: jsr $28cd4.l
pub fn i_03bdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BDCE);
    cpu.pc = 0x28CD4;
    16
}
/// $03BDCE: bra.w $3bdfa
pub fn i_03bdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BDFA;
    10
}
/// $03BDFA: rts
pub fn i_03bdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BDFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_SpawnFall_03bea0  ($03BEA0, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03BEA0: bclr.b #$2, $8c(a6)
pub fn i_03bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEA6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BEA6: bclr.b #$2, $8c(a6)
pub fn i_03bea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEAC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BEAC: jsr $3933a(pc)
pub fn i_03beac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BEB0);
    cpu.pc = 0x3933A;
    16
}
/// $03BEB0: lea.l $3bec2(pc), a0
pub fn i_03beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEB4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BEC2;
    16
}
/// $03BEB4: movea.l (a0, d0.w), a0
pub fn i_03beb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEB8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BEB8: jsr $28cd4.l
pub fn i_03beb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BEBE);
    cpu.pc = 0x28CD4;
    16
}
/// $03BEBE: bra.w $3beea
pub fn i_03bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BEEA;
    10
}
/// $03BEEA: rts
pub fn i_03beea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrouchEnterA_03beec  ($03BEEC, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03BEEC: bclr.b #$2, $8c(a6)
pub fn i_03beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEF2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BEF2: bclr.b #$2, $8c(a6)
pub fn i_03bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEF8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BEF8: jsr $3933a(pc)
pub fn i_03bef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BEFC);
    cpu.pc = 0x3933A;
    16
}
/// $03BEFC: lea.l $3bf0e(pc), a0
pub fn i_03befc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF00;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BF0E;
    16
}
/// $03BF00: movea.l (a0, d0.w), a0
pub fn i_03bf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF04;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BF04: jsr $28cd4.l
pub fn i_03bf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BF0A);
    cpu.pc = 0x28CD4;
    16
}
/// $03BF0A: bra.w $3bf36
pub fn i_03bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BF36;
    10
}
/// $03BF36: rts
pub fn i_03bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrouchEnterB_03bf84  ($03BF84, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03BF84: bclr.b #$2, $8c(a6)
pub fn i_03bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF8A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BF8A: bclr.b #$2, $8c(a6)
pub fn i_03bf8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF90;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BF90: jsr $3933a(pc)
pub fn i_03bf90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BF94);
    cpu.pc = 0x3933A;
    16
}
/// $03BF94: lea.l $3bfa6(pc), a0
pub fn i_03bf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF98;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BFA6;
    16
}
/// $03BF98: movea.l (a0, d0.w), a0
pub fn i_03bf98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF9C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BF9C: jsr $28cd4.l
pub fn i_03bf9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BFA2);
    cpu.pc = 0x28CD4;
    16
}
/// $03BFA2: bra.w $3bfce
pub fn i_03bfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BFCE;
    10
}
/// $03BFCE: rts
pub fn i_03bfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrouchIdleA_03bfd0  ($03BFD0, 44 B, player/arm_air_death_crouch_03a60a.s)
/// $03BFD0: bclr.b #$2, $8c(a6)
pub fn i_03bfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFD6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BFD6: jsr $2abcc.l
pub fn i_03bfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BFDC);
    cpu.pc = 0x2ABCC;
    16
}
/// $03BFDC: bcs.w $3c028
pub fn i_03bfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFE0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3C028; return 10; }
    8
}
/// $03BFE0: bclr.b #$2, $8c(a6)
pub fn i_03bfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFE6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BFE6: jsr $3933a(pc)
pub fn i_03bfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BFEA);
    cpu.pc = 0x3933A;
    16
}
/// $03BFEA: lea.l $3bffc(pc), a0
pub fn i_03bfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFEE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BFFC;
    16
}
/// $03BFEE: movea.l (a0, d0.w), a0
pub fn i_03bfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFF2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BFF2: jsr $28cd4.l
pub fn i_03bff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BFF8);
    cpu.pc = 0x28CD4;
    16
}
/// $03BFF8: bra.w $3c024
pub fn i_03bff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BFFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C024;
    10
}
/// $03C024: bra.w $3c06c
pub fn i_03c024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C028;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C06C;
    10
}
/// $03C028: bclr.b #$2, $8c(a6)
pub fn i_03c028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C02E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C02E: jsr $3933a(pc)
pub fn i_03c02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C032;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C032);
    cpu.pc = 0x3933A;
    16
}
/// $03C032: lea.l $3c044(pc), a0
pub fn i_03c032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C036;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C044;
    16
}
/// $03C036: movea.l (a0, d0.w), a0
pub fn i_03c036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C03A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C03A: jsr $28cd4.l
pub fn i_03c03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C040);
    cpu.pc = 0x28CD4;
    16
}
/// $03C040: bra.w $3c06c
pub fn i_03c040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C044;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C06C;
    10
}
/// $03C06C: rts
pub fn i_03c06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C06E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrouchShoot_03c10c  ($03C10C, 44 B, player/arm_air_death_crouch_03a60a.s)
/// $03C10C: bclr.b #$2, $8c(a6)
pub fn i_03c10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C112;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C112: jsr $2abcc.l
pub fn i_03c112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C118;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C118);
    cpu.pc = 0x2ABCC;
    16
}
/// $03C118: bcs.w $3c164
pub fn i_03c118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C11C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3C164; return 10; }
    8
}
/// $03C11C: bclr.b #$2, $8c(a6)
pub fn i_03c11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C122;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C122: jsr $3933a(pc)
pub fn i_03c122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C126;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C126);
    cpu.pc = 0x3933A;
    16
}
/// $03C126: lea.l $3c138(pc), a0
pub fn i_03c126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C12A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C138;
    16
}
/// $03C12A: movea.l (a0, d0.w), a0
pub fn i_03c12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C12E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C12E: jsr $28cd4.l
pub fn i_03c12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C134;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C134);
    cpu.pc = 0x28CD4;
    16
}
/// $03C134: bra.w $3c160
pub fn i_03c134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C138;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C160;
    10
}
/// $03C160: bra.w $3c1a8
pub fn i_03c160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C164;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C1A8;
    10
}
/// $03C164: bclr.b #$2, $8c(a6)
pub fn i_03c164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C16A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C16A: jsr $3933a(pc)
pub fn i_03c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C16E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C16E);
    cpu.pc = 0x3933A;
    16
}
/// $03C16E: lea.l $3c180(pc), a0
pub fn i_03c16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C172;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C180;
    16
}
/// $03C172: movea.l (a0, d0.w), a0
pub fn i_03c172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C176;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C176: jsr $28cd4.l
pub fn i_03c176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C17C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C17C);
    cpu.pc = 0x28CD4;
    16
}
/// $03C17C: bra.w $3c1a8
pub fn i_03c17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C180;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C1A8;
    10
}
/// $03C1A8: rts
pub fn i_03c1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrouchGrenade_03c28e  ($03C28E, 28 B, player/arm_air_death_crouch_03a60a.s)
/// $03C28E: bclr.b #$2, $8c(a6)
pub fn i_03c28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C294;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C294: jsr $3933a(pc)
pub fn i_03c294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C298;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C298);
    cpu.pc = 0x3933A;
    16
}
/// $03C298: lea.l $3c2aa(pc), a0
pub fn i_03c298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C29C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C2AA;
    16
}
/// $03C29C: movea.l (a0, d0.w), a0
pub fn i_03c29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2A0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C2A0: jsr $28cd4.l
pub fn i_03c2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C2A6);
    cpu.pc = 0x28CD4;
    16
}
/// $03C2A6: bra.w $3c2d2
pub fn i_03c2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C2D2;
    10
}
/// $03C2D2: rts
pub fn i_03c2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_MeleeC_03c2d4  ($03C2D4, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03C2D4: movea.l $c(a6), a0
pub fn i_03c2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2D8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03C2D8: move.b $80(a0), d0
pub fn i_03c2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2DC;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03C2DC: bne.w $3c328
pub fn i_03c2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C328; return 10; }
    8
}
/// $03C2E0: bclr.b #$2, $8c(a6)
pub fn i_03c2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2E6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C2E6: jsr $3933a(pc)
pub fn i_03c2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C2EA);
    cpu.pc = 0x3933A;
    16
}
/// $03C2EA: lea.l $3c2fc(pc), a0
pub fn i_03c2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2EE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C2FC;
    16
}
/// $03C2EE: movea.l (a0, d0.w), a0
pub fn i_03c2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2F2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C2F2: jsr $28cd4.l
pub fn i_03c2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C2F8);
    cpu.pc = 0x28CD4;
    16
}
/// $03C2F8: bra.w $3c324
pub fn i_03c2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C2FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C324;
    10
}
/// $03C324: bra.w $3c36c
pub fn i_03c324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C328;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C36C;
    10
}
/// $03C328: bclr.b #$2, $8c(a6)
pub fn i_03c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C32E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C32E: jsr $3933a(pc)
pub fn i_03c32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C332;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C332);
    cpu.pc = 0x3933A;
    16
}
/// $03C332: lea.l $3c344(pc), a0
pub fn i_03c332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C336;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C344;
    16
}
/// $03C336: movea.l (a0, d0.w), a0
pub fn i_03c336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C33A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C33A: jsr $28cd4.l
pub fn i_03c33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C340);
    cpu.pc = 0x28CD4;
    16
}
/// $03C340: bra.w $3c36c
pub fn i_03c340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C344;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C36C;
    10
}
/// $03C36C: rts
pub fn i_03c36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C36E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_AirMeleeA_03c36e  ($03C36E, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03C36E: cmpi.w #$1, $72(a6)
pub fn i_03c36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C374;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03C374: bne.w $3c3c0
pub fn i_03c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C378;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C3C0; return 10; }
    8
}
/// $03C378: bclr.b #$2, $8c(a6)
pub fn i_03c378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C37E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C37E: jsr $3933a(pc)
pub fn i_03c37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C382;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C382);
    cpu.pc = 0x3933A;
    16
}
/// $03C382: lea.l $3c394(pc), a0
pub fn i_03c382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C386;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C394;
    16
}
/// $03C386: movea.l (a0, d0.w), a0
pub fn i_03c386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C38A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C38A: jsr $28cd4.l
pub fn i_03c38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C390);
    cpu.pc = 0x28CD4;
    16
}
/// $03C390: bra.w $3c3bc
pub fn i_03c390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C394;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C3BC;
    10
}
/// $03C3BC: bra.w $3c456
pub fn i_03c3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C456;
    10
}
/// $03C3C0: cmpi.w #$2, $72(a6)
pub fn i_03c3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3C6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03C3C6: bne.w $3c412
pub fn i_03c3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C412; return 10; }
    8
}
/// $03C3CA: bclr.b #$2, $8c(a6)
pub fn i_03c3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3D0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C3D0: jsr $3933a(pc)
pub fn i_03c3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C3D4);
    cpu.pc = 0x3933A;
    16
}
/// $03C3D4: lea.l $3c3e6(pc), a0
pub fn i_03c3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3D8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C3E6;
    16
}
/// $03C3D8: movea.l (a0, d0.w), a0
pub fn i_03c3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3DC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C3DC: jsr $28cd4.l
pub fn i_03c3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C3E2);
    cpu.pc = 0x28CD4;
    16
}
/// $03C3E2: bra.w $3c40e
pub fn i_03c3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C3E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C40E;
    10
}
/// $03C40E: bra.w $3c456
pub fn i_03c40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C412;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C456;
    10
}
/// $03C412: bclr.b #$2, $8c(a6)
pub fn i_03c412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C418;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C418: jsr $3933a(pc)
pub fn i_03c418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C41C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C41C);
    cpu.pc = 0x3933A;
    16
}
/// $03C41C: lea.l $3c42e(pc), a0
pub fn i_03c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C420;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C42E;
    16
}
/// $03C420: movea.l (a0, d0.w), a0
pub fn i_03c420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C424;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C424: jsr $28cd4.l
pub fn i_03c424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C42A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C42A);
    cpu.pc = 0x28CD4;
    16
}
/// $03C42A: bra.w $3c456
pub fn i_03c42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C42E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C456;
    10
}
/// $03C456: rts
pub fn i_03c456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C458;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_SlugRideA_03c632  ($03C632, 34 B, player/fire_shells_03c62a.s)
/// $03C632: bclr.b #$2, $8c(a6)
pub fn i_03c632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C638;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C638: bclr.b #$2, $8c(a6)
pub fn i_03c638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C63E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C63E: jsr $3933a(pc)
pub fn i_03c63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C642;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C642);
    cpu.pc = 0x3933A;
    16
}
/// $03C642: lea.l $3c654(pc), a0
pub fn i_03c642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C646;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C654;
    16
}
/// $03C646: movea.l (a0, d0.w), a0
pub fn i_03c646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C64A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C64A: jsr $28cd4.l
pub fn i_03c64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C650);
    cpu.pc = 0x28CD4;
    16
}
/// $03C650: bra.w $3c67c
pub fn i_03c650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C654;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C67C;
    10
}
/// $03C67C: rts
pub fn i_03c67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C67E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Pistol_Spawn_03c710  ($03C710, 102 B, player/fire_shells_03c62a.s)
/// $03C710: cmpa.l #$100440, a6
pub fn i_03c710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C716;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03C716: beq.b $3c722
pub fn i_03c716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C718;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3C722; return 10; }
    8
}
/// $03C718: cmpa.l #$1004e0, a6
pub fn i_03c718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C71E;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03C71E: bne.w $3c728
pub fn i_03c71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C722;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C728; return 10; }
    8
}
/// $03C722: movea.l a6, a2
pub fn i_03c722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C724;
    cpu.ir = 0x244E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $03C724: bra.w $3c72c
pub fn i_03c724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C728;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C72C;
    10
}
/// $03C728: movea.l $c(a6), a2
pub fn i_03c728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C72C;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $03C72C: addq.b #$1, $7a(a6)
pub fn i_03c72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C730;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03C730: cmpi.w #$0, $82(a2)
pub fn i_03c730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C736;
    cpu.ir = 0xC6A;
    let ad2 = cpu.a[2].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03C736: beq.w $3c740
pub fn i_03c736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C73A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3C740; return 10; }
    8
}
/// $03C73A: subi.w #$1, $82(a2)
pub fn i_03c73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C740;
    cpu.ir = 0x46A;
    let ad1 = cpu.a[2].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03C740: move.w #$10f7, d0
pub fn i_03c740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C744;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03C744: jsr $2352.l
pub fn i_03c744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C74A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C74A);
    cpu.pc = 0x2352;
    16
}
/// $03C74A: lea.l $3093a.l, a1
pub fn i_03c74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C750;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3093A;
    20
}
/// $03C750: jsr $6fe.l
pub fn i_03c750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C756;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C756);
    cpu.pc = 0x6FE;
    16
}
/// $03C756: move.w $22(a2), $22(a0)
pub fn i_03c756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C75C;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03C75C: move.w $24(a2), $24(a0)
pub fn i_03c75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C762;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03C762: move.w $38(a2), $38(a0)
pub fn i_03c762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C768;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03C768: jsr $517fe.l
pub fn i_03c768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C76E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C76E);
    cpu.pc = 0x517FE;
    16
}
/// $03C76E: addi.w #$1c, $24(a0)
pub fn i_03c76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C774;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03C774: rts
pub fn i_03c774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C776;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Pistol_Fwd_03c776  ($03C776, 52 B, player/fire_shells_03c62a.s)
/// $03C776: jsr $3c710(pc)
pub fn i_03c776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C77A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C77A);
    cpu.pc = 0x3C710;
    16
}
/// $03C77A: btst.b #$0, $3a(a6)
pub fn i_03c77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C780;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C780: bne.w $3c790
pub fn i_03c780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C784;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C790; return 10; }
    8
}
/// $03C784: move.b #$0, d0
pub fn i_03c784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C788;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C788: move.b #$0, d1
pub fn i_03c788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C78C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C78C: bra.w $3c798
pub fn i_03c78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C790;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C798;
    10
}
/// $03C790: move.b #$0, d0
pub fn i_03c790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C794;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C794: move.b #$2, d1
pub fn i_03c794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C798;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C798: lsl.b #$3, d0
pub fn i_03c798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C79A;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03C79A: move.b d0, $98(a0)
pub fn i_03c79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C79E;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C79E: move.b $7a(a6), $9a(a0)
pub fn i_03c79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7A4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03C7A4: clr.b $9b(a0)
pub fn i_03c7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7A8;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03C7A8: rts
pub fn i_03c7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Pistol_Up_03c7aa  ($03C7AA, 30 B, player/fire_shells_03c62a.s)
/// $03C7AA: jsr $3c710(pc)
pub fn i_03c7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C7AE);
    cpu.pc = 0x3C710;
    16
}
/// $03C7AE: move.b #$8, d0
pub fn i_03c7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7B2;
    cpu.ir = 0x103C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C7B2: move.b #$1, d1
pub fn i_03c7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7B6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C7B6: lsl.b #$3, d0
pub fn i_03c7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7B8;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03C7B8: move.b d0, $98(a0)
pub fn i_03c7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7BC;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C7BC: move.b $7a(a6), $9a(a0)
pub fn i_03c7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7C2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03C7C2: clr.b $9b(a0)
pub fn i_03c7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7C6;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03C7C6: rts
pub fn i_03c7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Pistol_FwdLow_03c7fc  ($03C7FC, 58 B, player/fire_shells_03c62a.s)
/// $03C7FC: jsr $3c710(pc)
pub fn i_03c7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C800;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C800);
    cpu.pc = 0x3C710;
    16
}
/// $03C800: subi.w #$c, $24(a0)
pub fn i_03c800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C806;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03C806: btst.b #$0, $3a(a6)
pub fn i_03c806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C80C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C80C: bne.w $3c81c
pub fn i_03c80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C810;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C81C; return 10; }
    8
}
/// $03C810: move.b #$0, d0
pub fn i_03c810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C814;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C814: move.b #$0, d1
pub fn i_03c814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C818;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C818: bra.w $3c824
pub fn i_03c818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C81C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C824;
    10
}
/// $03C81C: move.b #$0, d0
pub fn i_03c81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C820;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C820: move.b #$2, d1
pub fn i_03c820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C824;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C824: lsl.b #$3, d0
pub fn i_03c824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C826;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03C826: move.b d0, $98(a0)
pub fn i_03c826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C82A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C82A: move.b $7a(a6), $9a(a0)
pub fn i_03c82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C830;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03C830: clr.b $9b(a0)
pub fn i_03c830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C834;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03C834: rts
pub fn i_03c834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C836;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Pistol_Down_03c836  ($03C836, 30 B, player/fire_shells_03c62a.s)
/// $03C836: jsr $3c710(pc)
pub fn i_03c836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C83A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C83A);
    cpu.pc = 0x3C710;
    16
}
/// $03C83A: move.b #$18, d0
pub fn i_03c83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C83E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C83E: move.b #$0, d1
pub fn i_03c83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C842;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C842: lsl.b #$3, d0
pub fn i_03c842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C844;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03C844: move.b d0, $98(a0)
pub fn i_03c844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C848;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C848: move.b $7a(a6), $9a(a0)
pub fn i_03c848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C84E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03C84E: clr.b $9b(a0)
pub fn i_03c84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C852;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03C852: rts
pub fn i_03c852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C854;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Debris_Spawn_03d616  ($03D616, 42 B, player/fire_shells_03c62a.s)
/// $03D616: cmpi.b #$3, $106ece.l
pub fn i_03d616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D61E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $03D61E: bne.w $3d63e
pub fn i_03d61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D622;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D63E; return 10; }
    8
}
/// $03D622: lea.l $3d76e(pc), a1
pub fn i_03d622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D626;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D76E;
    16
}
/// $03D626: jsr $4ae.l
pub fn i_03d626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D62C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D62C);
    cpu.pc = 0x4AE;
    16
}
/// $03D62C: jsr $5dd02.l
pub fn i_03d62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D632;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D632);
    cpu.pc = 0x5DD02;
    16
}
/// $03D632: andi.w #$ffe3, $38(a0)
pub fn i_03d632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D638;
    cpu.ir = 0x268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D638: ori.w #$10, $38(a0)
pub fn i_03d638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D63E;
    cpu.ir = 0x68;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D63E: rts
pub fn i_03d63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D640;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SlugCannon_ArmOverlay_03d944  ($03D944, 190 B, player/fire_shells_03c62a.s)
/// $03D944: lea.l $100440.l, a0
pub fn i_03d944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D94A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $03D94A: jsr $2ac0e.l
pub fn i_03d94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D950);
    cpu.pc = 0x2AC0E;
    16
}
/// $03D950: bcc.w $3d98e
pub fn i_03d950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D954;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3D98E; return 10; }
    8
}
/// $03D954: move.w #$176, d1
pub fn i_03d954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D958;
    cpu.ir = 0x323C;
    let v1: u32 = 0x176;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D958: jsr $236e.l
pub fn i_03d958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D95E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D95E);
    cpu.pc = 0x236E;
    16
}
/// $03D95E: move.w #$190, d1
pub fn i_03d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D962;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D962: jsr $236e.l
pub fn i_03d962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D968;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D968);
    cpu.pc = 0x236E;
    16
}
/// $03D968: move.w #$191, d1
pub fn i_03d968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D96C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x191;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D96C: jsr $236e.l
pub fn i_03d96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D972;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D972);
    cpu.pc = 0x236E;
    16
}
/// $03D972: move.w #$1b, $1c(a6)
pub fn i_03d972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D978;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03D978: jsr $138fe.l
pub fn i_03d978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D97E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D97E);
    cpu.pc = 0x138FE;
    16
}
/// $03D97E: bclr.b #$4, $12(a6)
pub fn i_03d97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D984;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03D984: lea.l $100440.l, a0
pub fn i_03d984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D98A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $03D98A: bra.w $3d9dc
pub fn i_03d98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D98E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D9DC;
    10
}
/// $03D98E: lea.l $1004e0.l, a0
pub fn i_03d98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D994;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $03D994: jsr $2ac0e.l
pub fn i_03d994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D99A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D99A);
    cpu.pc = 0x2AC0E;
    16
}
/// $03D99A: bcc.w $3d9d8
pub fn i_03d99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D99E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3D9D8; return 10; }
    8
}
/// $03D99E: move.w #$177, d1
pub fn i_03d99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9A2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x177;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D9A2: jsr $236e.l
pub fn i_03d9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D9A8);
    cpu.pc = 0x236E;
    16
}
/// $03D9A8: move.w #$190, d1
pub fn i_03d9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9AC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D9AC: jsr $236e.l
pub fn i_03d9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D9B2);
    cpu.pc = 0x236E;
    16
}
/// $03D9B2: move.w #$192, d1
pub fn i_03d9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x192;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D9B6: jsr $236e.l
pub fn i_03d9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D9BC);
    cpu.pc = 0x236E;
    16
}
/// $03D9BC: move.w #$1c, $1c(a6)
pub fn i_03d9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03D9C2: jsr $138fe.l
pub fn i_03d9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D9C8);
    cpu.pc = 0x138FE;
    16
}
/// $03D9C8: bset.b #$4, $12(a6)
pub fn i_03d9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9CE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03D9CE: lea.l $1004e0.l, a0
pub fn i_03d9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $03D9D4: bra.w $3d9dc
pub fn i_03d9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D9DC;
    10
}
/// $03D9D8: jmp $3daa0(pc)
pub fn i_03d9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9DC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3DAA0;
    8
}
/// $03D9DC: moveq #$0, d0
pub fn i_03d9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9DE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03D9DE: move.b $71(a0), d0
pub fn i_03d9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9E2;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03D9E2: cmpa.l #$1004e0, a0
pub fn i_03d9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9E8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03D9E8: bne.w $3d9ee
pub fn i_03d9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D9EE; return 10; }
    8
}
/// $03D9EC: addq.w #$5, d0
pub fn i_03d9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9EE;
    cpu.ir = 0x5A40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 5, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03D9EE: lsl.w #$2, d0
pub fn i_03d9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9F0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $03D9F0: lea.l $3da02(pc), a0
pub fn i_03d9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9F4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3DA02;
    16
}
/// $03D9F4: movea.l (a0, d0.w), a0
pub fn i_03d9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9F8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03D9F8: jsr $28cd4.l
pub fn i_03d9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D9FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D9FE);
    cpu.pc = 0x28CD4;
    16
}
/// $03D9FE: bra.w $3da2a
pub fn i_03d9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA02;
    cpu.ir = 0x6000;
    cpu.pc = 0x3DA2A;
    10
}
/// $03DA2A: jsr $2ff8e.l
pub fn i_03da2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DA30);
    cpu.pc = 0x2FF8E;
    16
}
/// $03DA30: lsl.w #$2, d0
pub fn i_03da30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA32;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $03DA32: lea.l $3daa8(pc), a0
pub fn i_03da32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA36;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3DAA8;
    16
}
/// $03DA36: move.w (a0, d0.w), d1
pub fn i_03da36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA3A;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $03DA3A: move.w $2(a0, d0.w), d2
pub fn i_03da3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA3E;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $03DA3E: move.w d1, $5c(a6)
pub fn i_03da3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA42;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DA42: move.w d2, $5e(a6)
pub fn i_03da42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA46;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DA46: lea.l $3da4c(pc), a1
pub fn i_03da46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DA4C;
    16
}
/// $03DA4A: move.l a1, (a6)
pub fn i_03da4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DA4C: jsr $2783a.l
pub fn i_03da4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DA52);
    cpu.pc = 0x2783A;
    16
}
/// $03DA52: lea.l $100580.l, a0
pub fn i_03da52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $03DA58: move.w $22(a0), d0
pub fn i_03da58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA5C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03DA5C: move.w $24(a0), d1
pub fn i_03da5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA60;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03DA60: move.w $38(a0), d2
pub fn i_03da60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA64;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03DA64: add.w $5c(a6), d0
pub fn i_03da64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA68;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03DA68: add.w $5e(a6), d1
pub fn i_03da68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA6C;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03DA6C: move.w d0, $22(a6)
pub fn i_03da6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA70;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DA70: move.w d1, $24(a6)
pub fn i_03da70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA74;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DA74: move.w d2, $38(a6)
pub fn i_03da74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA78;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DA78: andi.w #$ffe3, $38(a6)
pub fn i_03da78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA7E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03DA7E: ori.w #$4, $38(a6)
pub fn i_03da7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA84;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03DA84: jsr $28d70.l
pub fn i_03da84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DA8A);
    cpu.pc = 0x28D70;
    16
}
/// $03DA8A: bcc.w $3da9e
pub fn i_03da8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3DA9E; return 10; }
    8
}
/// $03DA8E: lea.l $3daa0(pc), a1
pub fn i_03da8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DAA0;
    16
}
/// $03DA92: move.l a1, (a6)
pub fn i_03da92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DA94: move.w #$108b, d0
pub fn i_03da94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_03da98  ($03DA98, 8 B, thunks/jsr_abs_thunks.c)
/// $03DA98: jsr $2352.l
pub fn i_03da98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DA9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DA9E);
    cpu.pc = 0x2352;
    16
}
/// $03DA9E: rts
pub fn i_03da9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DAA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_03daa0  ($03DAA0, 8 B, kernel/jmp_to_scheduler.c)
/// $03DAA0: jmp $518.l
pub fn i_03daa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DAA6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Subsystem_ScoresInit_03EE3A  ($03EE3A, 14 B, pow/results_squadleader_03daa8.s)
/// $03EE3A: clr.w $106f4c.l
pub fn i_03ee3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE40;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F4C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F4C, r2 as u16);
    16
}
/// $03EE40: clr.w $106f4e.l
pub fn i_03ee40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE46;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F4E) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F4E, r2 as u16);
    16
}
/// $03EE46: rts
pub fn i_03ee46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_CountIfPending_03ee48  ($03EE48, 10 B, pow/results_squadleader_03daa8.s)
/// $03EE48: tst.b $106ed2.l
pub fn i_03ee48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE4E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $03EE4E: bne.w $3ee58
pub fn i_03ee4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EE58; return 10; }
    8
}
// ===== ClearC_03ee52  ($03EE52, 6 B, thunks/ccr_helpers.c)
/// $03EE52: andi.b #$fe, ccr
pub fn i_03ee52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE56;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EE56: rts
pub fn i_03ee56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RescueCredit_03ee58  ($03EE58, 84 B, pow/results_squadleader_03daa8.s)
/// $03EE58: move.l #$ffffffff, -(a7)
pub fn i_03ee58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE5E;
    cpu.ir = 0x2F3C;
    let v1: u32 = 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $03EE5E: moveq #$0, d0
pub fn i_03ee5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE60;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EE60: jsr $5e3a2.l
pub fn i_03ee60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EE66);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EE66: bcc.w $3ee74
pub fn i_03ee66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EE74; return 10; }
    8
}
/// $03EE6A: bsr.w $3ef5c
pub fn i_03ee6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE6E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3EE6E);
    cpu.pc = 0x3EF5C;
    18
}
/// $03EE6E: bcc.w $3ee74
pub fn i_03ee6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EE74; return 10; }
    8
}
/// $03EE72: move.l a0, (a7)
pub fn i_03ee72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE74;
    cpu.ir = 0x2E88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EE74: moveq #$1, d0
pub fn i_03ee74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE76;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EE76: jsr $5e3a2.l
pub fn i_03ee76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EE7C);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EE7C: bcc.w $3eea0
pub fn i_03ee7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EEA0; return 10; }
    8
}
/// $03EE80: bsr.w $3ef5c
pub fn i_03ee80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE84;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3EE84);
    cpu.pc = 0x3EF5C;
    18
}
/// $03EE84: bcc.w $3eea0
pub fn i_03ee84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EEA0; return 10; }
    8
}
/// $03EE88: move.l (a7), d0
pub fn i_03ee88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE8A;
    cpu.ir = 0x2017;
    let v2 = bus.read32(cpu.a[7]);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EE8A: bmi.w $3ee9e
pub fn i_03ee8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE8E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x3EE9E; return 10; }
    8
}
/// $03EE8E: movea.l d0, a2
pub fn i_03ee8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE90;
    cpu.ir = 0x2440;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $03EE90: move.w $22(a0), d0
pub fn i_03ee90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE94;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03EE94: cmp.w $22(a2), d0
pub fn i_03ee94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE98;
    cpu.ir = 0xB06A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03EE98: bcc.w $3ee9e
pub fn i_03ee98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EE9E; return 10; }
    8
}
/// $03EE9C: movea.l a2, a0
pub fn i_03ee9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EE9E;
    cpu.ir = 0x204A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $03EE9E: move.l a0, (a7)
pub fn i_03ee9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEA0;
    cpu.ir = 0x2E88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EEA0: moveq #$0, d0
pub fn i_03eea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEA2;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EEA2: move.b #$1, d2
pub fn i_03eea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEA6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EEA6: move.l (a7)+, d1
pub fn i_03eea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEA8;
    cpu.ir = 0x221F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EEA8: bpl.w $3eeb6
pub fn i_03eea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEAC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x3EEB6; return 10; }
    8
}
// ===== ClearC_03eeac  ($03EEAC, 6 B, thunks/ccr_helpers.c)
/// $03EEAC: andi.b #$fe, ccr
pub fn i_03eeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEB0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EEB0: rts
pub fn i_03eeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RescueCreditP2_03eeb6  ($03EEB6, 16 B, pow/results_squadleader_03daa8.s)
/// $03EEB6: cmpi.l #$100440, d1
pub fn i_03eeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEBC;
    cpu.ir = 0xC81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100440, Size::Long, false);
    8
}
/// $03EEBC: beq.w $3eec6
pub fn i_03eebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3EEC6; return 10; }
    8
}
/// $03EEC0: moveq #$1, d0
pub fn i_03eec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEC2;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EEC2: move.b #$2, d2
pub fn i_03eec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEC6;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== SetC_03eec6  ($03EEC6, 6 B, thunks/ccr_helpers.c)
/// $03EEC6: ori.b #$1, ccr
pub fn i_03eec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EECA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $03EECA: rts
pub fn i_03eeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EECC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_CountIfPendingB_03eecc  ($03EECC, 10 B, pow/results_squadleader_03daa8.s)
/// $03EECC: tst.b $106ed2.l
pub fn i_03eecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EED2;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $03EED2: bne.w $3eedc
pub fn i_03eed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EED6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EEDC; return 10; }
    8
}
// ===== ClearC_03eed6  ($03EED6, 6 B, thunks/ccr_helpers.c)
/// $03EED6: andi.b #$fe, ccr
pub fn i_03eed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEDA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EEDA: rts
pub fn i_03eeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RescueCreditB_03eedc  ($03EEDC, 44 B, pow/results_squadleader_03daa8.s)
/// $03EEDC: move.l #$ffffffff, -(a7)
pub fn i_03eedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEE2;
    cpu.ir = 0x2F3C;
    let v1: u32 = 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $03EEE2: moveq #$0, d0
pub fn i_03eee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEE4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EEE4: jsr $5e3a2.l
pub fn i_03eee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EEEA);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EEEA: bcc.w $3eefc
pub fn i_03eeea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EEFC; return 10; }
    8
}
/// $03EEEE: lea.l $3ef9a(pc), a1
pub fn i_03eeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EF9A;
    16
}
/// $03EEF2: bsr.w $3ef5c
pub fn i_03eef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEF6;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3EEF6);
    cpu.pc = 0x3EF5C;
    18
}
/// $03EEF6: bcc.w $3eefc
pub fn i_03eef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EEFC; return 10; }
    8
}
/// $03EEFA: move.l a0, (a7)
pub fn i_03eefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEFC;
    cpu.ir = 0x2E88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EEFC: moveq #$0, d0
pub fn i_03eefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EEFE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EEFE: move.b #$1, d2
pub fn i_03eefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF02;
    cpu.ir = 0x143C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EF02: move.l (a7)+, d1
pub fn i_03ef02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF04;
    cpu.ir = 0x221F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EF04: bpl.w $3ef0e
pub fn i_03ef04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF08;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x3EF0E; return 10; }
    8
}
// ===== ClearC_03ef08  ($03EF08, 6 B, thunks/ccr_helpers.c)
/// $03EF08: andi.b #$fe, ccr
pub fn i_03ef08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF0C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EF0C: rts
pub fn i_03ef0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_03ef0e  ($03EF0E, 6 B, thunks/ccr_helpers.c)
/// $03EF0E: ori.b #$1, ccr
pub fn i_03ef0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF12;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $03EF12: rts
pub fn i_03ef12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_CountIfPendingC_03ef14  ($03EF14, 10 B, pow/results_squadleader_03daa8.s)
/// $03EF14: tst.b $106ed2.l
pub fn i_03ef14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF1A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x106ED2) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $03EF1A: bne.w $3ef24
pub fn i_03ef1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EF24; return 10; }
    8
}
// ===== ClearC_03ef1e  ($03EF1E, 6 B, thunks/ccr_helpers.c)
/// $03EF1E: andi.b #$fe, ccr
pub fn i_03ef1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF22;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EF22: rts
pub fn i_03ef22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RescueCreditC_03ef24  ($03EF24, 44 B, pow/results_squadleader_03daa8.s)
/// $03EF24: move.l #$ffffffff, -(a7)
pub fn i_03ef24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF2A;
    cpu.ir = 0x2F3C;
    let v1: u32 = 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $03EF2A: moveq #$1, d0
pub fn i_03ef2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF2C;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EF2C: jsr $5e3a2.l
pub fn i_03ef2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EF32);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EF32: bcc.w $3ef44
pub fn i_03ef32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EF44; return 10; }
    8
}
/// $03EF36: lea.l $3ef9a(pc), a1
pub fn i_03ef36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EF9A;
    16
}
/// $03EF3A: bsr.w $3ef5c
pub fn i_03ef3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF3E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3EF3E);
    cpu.pc = 0x3EF5C;
    18
}
/// $03EF3E: bcc.w $3ef44
pub fn i_03ef3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EF44; return 10; }
    8
}
/// $03EF42: move.l a0, (a7)
pub fn i_03ef42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF44;
    cpu.ir = 0x2E88;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(cpu.a[7], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EF44: moveq #$1, d0
pub fn i_03ef44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF46;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EF46: move.b #$2, d2
pub fn i_03ef46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF4A;
    cpu.ir = 0x143C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EF4A: move.l (a7)+, d1
pub fn i_03ef4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF4C;
    cpu.ir = 0x221F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EF4C: bpl.w $3ef56
pub fn i_03ef4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF50;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x3EF56; return 10; }
    8
}
// ===== ClearC_03ef50  ($03EF50, 6 B, thunks/ccr_helpers.c)
/// $03EF50: andi.b #$fe, ccr
pub fn i_03ef50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF54;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EF54: rts
pub fn i_03ef54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_03ef56  ($03EF56, 6 B, thunks/ccr_helpers.c)
/// $03EF56: ori.b #$1, ccr
pub fn i_03ef56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF5A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $03EF5A: rts
pub fn i_03ef5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_RectOverlap_03ef5c  ($03EF5C, 56 B, pow/results_squadleader_03daa8.s)
/// $03EF5C: btst.b #$3, $13(a0)
pub fn i_03ef5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF62;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03EF62: bne.w $3ef94
pub fn i_03ef62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EF94; return 10; }
    8
}
/// $03EF66: btst.b #$0, $13(a0)
pub fn i_03ef66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF6C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03EF6C: bne.w $3ef94
pub fn i_03ef6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EF94; return 10; }
    8
}
/// $03EF70: move.w $22(a0), d0
pub fn i_03ef70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF74;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03EF74: sub.w $22(a6), d0
pub fn i_03ef74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF78;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EF78: add.w (a1), d0
pub fn i_03ef78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF7A;
    cpu.ir = 0xD051;
    let v2 = bus.read16(cpu.a[1]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03EF7A: cmp.w $2(a1), d0
pub fn i_03ef7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF7E;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03EF7E: bcc.w $3ef92
pub fn i_03ef7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EF92; return 10; }
    8
}
/// $03EF82: move.w $24(a0), d0
pub fn i_03ef82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF86;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03EF86: sub.w $24(a6), d0
pub fn i_03ef86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF8A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EF8A: add.w $4(a1), d0
pub fn i_03ef8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF8E;
    cpu.ir = 0xD069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EF8E: cmp.w $6(a1), d0
pub fn i_03ef8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF92;
    cpu.ir = 0xB069;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03EF92: rts
pub fn i_03ef92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_03ef94  ($03EF94, 6 B, thunks/ccr_helpers.c)
/// $03EF94: andi.b #$fe, ccr
pub fn i_03ef94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF98;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $03EF98: rts
pub fn i_03ef98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EF9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03EFA2: movem.w d0, -(a7)
pub fn i_03efa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFA6;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $03EFA6: jsr $5e3a2.l
pub fn i_03efa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EFAC);
    cpu.pc = 0x5E3A2;
    16
}
/// $03EFAC: movem.w (a7)+, d0
pub fn i_03efac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFB0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $03EFB0: bcc.w $3efcc
pub fn i_03efb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EFCC; return 10; }
    8
}
/// $03EFB4: tst.b d0
pub fn i_03efb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFB6;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $03EFB6: bne.w $3efc4
pub fn i_03efb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFBA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EFC4; return 10; }
    8
}
/// $03EFBA: lea.l $106f4c.l, a1
pub fn i_03efba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFC0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x106F4C;
    20
}
/// $03EFC0: bra.w $3efca
pub fn i_03efc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EFCA;
    10
}
/// $03EFC4: lea.l $106f4e.l, a1
pub fn i_03efc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFCA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x106F4E;
    20
}
/// $03EFCA: addq.w #$1, (a1)
pub fn i_03efca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFCC;
    cpu.ir = 0x5251;
    let ad1 = cpu.a[1];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $03EFCC: rts
pub fn i_03efcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_OffworldFree_03efce  ($03EFCE, 22 B, pow/results_squadleader_03daa8.s)
/// $03EFCE: lea.l $3efe4(pc), a0
pub fn i_03efce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFD2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3EFE4;
    16
}
/// $03EFD2: jsr $5dd56.l
pub fn i_03efd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EFD8);
    cpu.pc = 0x5DD56;
    16
}
/// $03EFD8: bcc.w $3efe2
pub fn i_03efd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EFE2; return 10; }
    8
}
/// $03EFDC: jmp $518.l
pub fn i_03efdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFE2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03EFE2: rts
pub fn i_03efe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03EFEC: move.b $9b(a6), d0
pub fn i_03efec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFF0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03EFF0: move.w #$9c, d1
pub fn i_03eff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFF4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EFF4: jmp $9a7cc.l
pub fn i_03eff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EFFA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x9A7CC;
    8
}
/// $03F9B8: tst.b $9a(a6)
pub fn i_03f9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9BC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03F9BC: beq.w $3f9c6
pub fn i_03f9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3F9C6; return 10; }
    8
}
/// $03F9C0: bset.b #$0, $3a(a6)
pub fn i_03f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9C6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03F9C6: move.w #$182, d1
pub fn i_03f9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x182;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03F9CA: jsr $236e.l
pub fn i_03f9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3F9D0);
    cpu.pc = 0x236E;
    16
}
/// $03F9D0: move.w #$8000, d0
pub fn i_03f9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03F9D4: jsr $28134.l
pub fn i_03f9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3F9DA);
    cpu.pc = 0x28134;
    16
}
/// $03F9DA: andi.w #$ffe3, $38(a6)
pub fn i_03f9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9E0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03F9E0: ori.w #$18, $38(a6)
pub fn i_03f9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9E6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03F9E6: clr.w $28(a6)
pub fn i_03f9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03F9EA: clr.w $2a(a6)
pub fn i_03f9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9EE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03F9EE: clr.w $2c(a6)
pub fn i_03f9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9F2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03F9F2: clr.w $2e(a6)
pub fn i_03f9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9F6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03F9F6: jsr $27f60.l
pub fn i_03f9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3F9FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3F9FC);
    cpu.pc = 0x27F60;
    16
}
/// $03F9FC: scc.b $70(a6)
pub fn i_03f9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FA00;
    cpu.ir = 0x54EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $03FA00: lea.l $776e2.l, a1
pub fn i_03fa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FA06;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
// ===== JsrAbsThunk_03fa06  ($03FA06, 8 B, thunks/jsr_abs_thunks.c)
/// $03FA06: jsr $4ae.l
pub fn i_03fa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FA0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FA0C);
    cpu.pc = 0x4AE;
    16
}
/// $03FA0C: rts
pub fn i_03fa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FA0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03FC38: bsr.w $3f9b8
pub fn i_03fc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC3C;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FC3C);
    cpu.pc = 0x3F9B8;
    18
}
/// $03FC3C: lea.l $3f292(pc), a0
pub fn i_03fc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC40;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F292;
    16
}
/// $03FC40: jsr $28cd4.l
pub fn i_03fc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC46);
    cpu.pc = 0x28CD4;
    16
}
/// $03FC46: lea.l $3fc4c(pc), a1
pub fn i_03fc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FC4C;
    16
}
/// $03FC4A: move.l a1, (a6)
pub fn i_03fc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FC4C: jsr $3fece(pc)
pub fn i_03fc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3FC50);
    cpu.pc = 0x3FECE;
    16
}
/// $03FC50: jsr $28d70.l
pub fn i_03fc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC56);
    cpu.pc = 0x28D70;
    16
}
/// $03FC56: jsr $2870a.l
pub fn i_03fc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC5C);
    cpu.pc = 0x2870A;
    16
}
/// $03FC5C: bcc.w $3fc66
pub fn i_03fc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FC66; return 10; }
    8
}
/// $03FC60: lea.l $3fc6a(pc), a1
pub fn i_03fc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FC6A;
    16
}
/// $03FC64: move.l a1, (a6)
pub fn i_03fc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FC66: jmp $3efce(pc)
pub fn i_03fc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC6A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_Freed_03fc6a  ($03FC6A, 58 B, pow/results_squadleader_03daa8.s)
/// $03FC6A: lea.l $3f39e(pc), a0
pub fn i_03fc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC6E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F39E;
    16
}
/// $03FC6E: jsr $28cd4.l
pub fn i_03fc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC74);
    cpu.pc = 0x28CD4;
    16
}
/// $03FC74: lea.l $3fc7a(pc), a1
pub fn i_03fc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FC7A;
    16
}
/// $03FC78: move.l a1, (a6)
pub fn i_03fc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FC7A: jsr $2783a.l
pub fn i_03fc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC80);
    cpu.pc = 0x2783A;
    16
}
/// $03FC80: jsr $28d70.l
pub fn i_03fc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC86);
    cpu.pc = 0x28D70;
    16
}
/// $03FC86: bcc.w $3fca0
pub fn i_03fc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FCA0; return 10; }
    8
}
/// $03FC8A: lea.l $3ff14(pc), a1
pub fn i_03fc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF14;
    16
}
/// $03FC8E: move.l a1, (a6)
pub fn i_03fc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FC90: lea.l $3fca4(pc), a1
pub fn i_03fc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FCA4;
    16
}
/// $03FC94: jsr $6fe.l
pub fn i_03fc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FC9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FC9A);
    cpu.pc = 0x6FE;
    16
}
/// $03FC9A: jsr $5dd02.l
pub fn i_03fc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCA0);
    cpu.pc = 0x5DD02;
    16
}
/// $03FCA0: jmp $3efce(pc)
pub fn i_03fca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCA4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_FreedCheer_03fca4  ($03FCA4, 20 B, pow/results_squadleader_03daa8.s)
/// $03FCA4: move.w #$182, d1
pub fn i_03fca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCA8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x182;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FCA8: jsr $236e.l
pub fn i_03fca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCAE);
    cpu.pc = 0x236E;
    16
}
/// $03FCAE: lea.l $3f3c2(pc), a0
pub fn i_03fcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCB2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F3C2;
    16
}
/// $03FCB2: jsr $28cd4.l
pub fn i_03fcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCB8);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_03fcb8  ($03FCB8, 8 B, kernel/task_handlers.c)
/// $03FCB8: lea.l $3fcc0(pc), a1
pub fn i_03fcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FCC0;
    16
}
/// $03FCBC: move.l a1, (a6)
pub fn i_03fcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FCBE: rts
pub fn i_03fcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_FreedWait_03fcc0  ($03FCC0, 24 B, pow/results_squadleader_03daa8.s)
/// $03FCC0: jsr $2783a.l
pub fn i_03fcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCC6);
    cpu.pc = 0x2783A;
    16
}
/// $03FCC6: jsr $28d70.l
pub fn i_03fcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FCCC);
    cpu.pc = 0x28D70;
    16
}
/// $03FCCC: bcc.w $3fcd6
pub fn i_03fccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCD0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FCD6; return 10; }
    8
}
/// $03FCD0: jmp $518.l
pub fn i_03fcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCD6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03FCD6: rts
pub fn i_03fcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FCD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_Entry_03fe5a  ($03FE5A, 4 B, pow/results_squadleader_03daa8.s)
/// $03FE5A: bsr.w $3f9b8
pub fn i_03fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE5E;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FE5E);
    cpu.pc = 0x3F9B8;
    18
}
// ===== SetTaskHandler_03fe5e  ($03FE5E, 8 B, kernel/task_handlers.c)
/// $03FE5E: lea.l $3fe66(pc), a1
pub fn i_03fe5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FE66;
    16
}
/// $03FE62: move.l a1, (a6)
pub fn i_03fe62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FE64: rts
pub fn i_03fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_Idle_03fe66  ($03FE66, 54 B, pow/results_squadleader_03daa8.s)
/// $03FE66: lea.l $3f070(pc), a0
pub fn i_03fe66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE6A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F070;
    16
}
/// $03FE6A: jsr $28cd4.l
pub fn i_03fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE70);
    cpu.pc = 0x28CD4;
    16
}
/// $03FE70: clr.w $28(a6)
pub fn i_03fe70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE74;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03FE74: clr.w $2a(a6)
pub fn i_03fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE78;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03FE78: lea.l $3fe7e(pc), a1
pub fn i_03fe78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FE7E;
    16
}
/// $03FE7C: move.l a1, (a6)
pub fn i_03fe7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FE7E: jsr $3fece(pc)
pub fn i_03fe7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3FE82);
    cpu.pc = 0x3FECE;
    16
}
/// $03FE82: jsr $28d70.l
pub fn i_03fe82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE88);
    cpu.pc = 0x28D70;
    16
}
/// $03FE88: jsr $2870a.l
pub fn i_03fe88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FE8E);
    cpu.pc = 0x2870A;
    16
}
/// $03FE8E: bcc.w $3fe98
pub fn i_03fe8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FE98; return 10; }
    8
}
/// $03FE92: lea.l $3fe9c(pc), a1
pub fn i_03fe92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FE9C;
    16
}
/// $03FE96: move.l a1, (a6)
pub fn i_03fe96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FE98: jmp $3efce(pc)
pub fn i_03fe98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FE9C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_IdleWave_03fe9c  ($03FE9C, 42 B, pow/results_squadleader_03daa8.s)
/// $03FE9C: lea.l $3f0cc(pc), a0
pub fn i_03fe9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEA0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F0CC;
    16
}
/// $03FEA0: jsr $28cd4.l
pub fn i_03fea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FEA6);
    cpu.pc = 0x28CD4;
    16
}
/// $03FEA6: lea.l $3feac(pc), a1
pub fn i_03fea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FEAC;
    16
}
/// $03FEAA: move.l a1, (a6)
pub fn i_03feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FEAC: jsr $28364.l
pub fn i_03feac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FEB2);
    cpu.pc = 0x28364;
    16
}
/// $03FEB2: jsr $28d70.l
pub fn i_03feb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FEB8);
    cpu.pc = 0x28D70;
    16
}
/// $03FEB8: bcc.w $3fec2
pub fn i_03feb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FEC2; return 10; }
    8
}
/// $03FEBC: lea.l $3ff14(pc), a1
pub fn i_03febc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF14;
    16
}
/// $03FEC0: move.l a1, (a6)
pub fn i_03fec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FEC2: jmp $3efce(pc)
pub fn i_03fec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEC6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3EFCE;
    8
}
// ===== Pow_Physics_03fece  ($03FECE, 62 B, pow/results_squadleader_03daa8.s)
/// $03FECE: jsr $27f08.l
pub fn i_03fece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FED4);
    cpu.pc = 0x27F08;
    16
}
/// $03FED4: bcc.w $3fee0
pub fn i_03fed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FED8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FEE0; return 10; }
    8
}
/// $03FED8: move.b d3, $70(a6)
pub fn i_03fed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEDC;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FEDC: bra.w $3fef6
pub fn i_03fedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FEF6;
    10
}
/// $03FEE0: cmp.b $70(a6), d0
pub fn i_03fee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEE4;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $03FEE4: bne.w $3fef0
pub fn i_03fee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FEF0; return 10; }
    8
}
/// $03FEE8: move.b d3, $70(a6)
pub fn i_03fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEEC;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FEEC: bra.w $3fef6
pub fn i_03feec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FEF6;
    10
}
/// $03FEF0: move.b #$ff, $70(a6)
pub fn i_03fef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FEF6: tst.b $70(a6)
pub fn i_03fef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEFA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03FEFA: bne.w $3ff0c
pub fn i_03fefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FEFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FF0C; return 10; }
    8
}
/// $03FEFE: jsr $28364.l
pub fn i_03fefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF04);
    cpu.pc = 0x28364;
    16
}
/// $03FF04: scs.b $70(a6)
pub fn i_03ff04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF08;
    cpu.ir = 0x55EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Cs) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $03FF08: bra.w $3ff12
pub fn i_03ff08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FF12;
    10
}
// ===== JsrAbsThunk_03ff0c  ($03FF0C, 8 B, thunks/jsr_abs_thunks.c)
/// $03FF0C: jsr $28292.l
pub fn i_03ff0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF12);
    cpu.pc = 0x28292;
    16
}
/// $03FF12: rts
pub fn i_03ff12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Pow_Despawn_03ff14  ($03FF14, 16 B, pow/results_squadleader_03daa8.s)
/// $03FF14: jsr $13600.l
pub fn i_03ff14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF1A);
    cpu.pc = 0x13600;
    16
}
/// $03FF1A: move.w #$181, d1
pub fn i_03ff1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF1E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x181;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FF1E: jsr $236e.l
pub fn i_03ff1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF24);
    cpu.pc = 0x236E;
    16
}
// ===== Pow_PickWalk_03ff24  ($03FF24, 62 B, pow/results_squadleader_03daa8.s)
/// $03FF24: clr.b $70(a6)
pub fn i_03ff24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF28;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03FF28: lea.l $3f01c(pc), a0
pub fn i_03ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF2C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F01C;
    16
}
/// $03FF2C: move.l a0, $48(a6)
pub fn i_03ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF30;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03FF30: bclr.b #$3, $13(a6)
pub fn i_03ff30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF36;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03FF36: bclr.b #$0, $13(a6)
pub fn i_03ff36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF3C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03FF3C: move.b $98(a6), d0
pub fn i_03ff3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF40;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03FF40: beq.w $3ff5c
pub fn i_03ff40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF44;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3FF5C; return 10; }
    8
}
/// $03FF44: cmpi.b #$1, d0
pub fn i_03ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF48;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $03FF48: bne.w $3ff56
pub fn i_03ff48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF4C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FF56; return 10; }
    8
}
/// $03FF4C: bclr.b #$0, $3a(a6)
pub fn i_03ff4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF52;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03FF52: bra.w $3ff5c
pub fn i_03ff52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF56;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FF5C;
    10
}
/// $03FF56: bset.b #$0, $3a(a6)
pub fn i_03ff56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF5C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03FF5C: move.w #$f, $5c(a6)
pub fn i_03ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF62;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Pow_Walk_03ff62  ($03FF62, 178 B, pow/results_squadleader_03daa8.s)
/// $03FF62: lea.l $3f40a(pc), a0
pub fn i_03ff62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF66;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3F40A;
    16
}
/// $03FF66: jsr $28cd4.l
pub fn i_03ff66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FF6C);
    cpu.pc = 0x28CD4;
    16
}
/// $03FF6C: move.w #$180, d0
pub fn i_03ff6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03FF70: btst.b #$0, $3a(a6)
pub fn i_03ff70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF76;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03FF76: bne.w $3ff7c
pub fn i_03ff76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF7A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FF7C; return 10; }
    8
}
/// $03FF7A: neg.w d0
pub fn i_03ff7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF7C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03FF7C: move.w d0, $28(a6)
pub fn i_03ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF80;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03FF80: lea.l $3ff86(pc), a1
pub fn i_03ff80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3FF86;
    16
}
/// $03FF84: move.l a1, (a6)
pub fn i_03ff84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FF86: bsr.w $3fece
pub fn i_03ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF8A;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FF8A);
    cpu.pc = 0x3FECE;
    18
}
/// $03FF8A: tst.b $70(a6)
pub fn i_03ff8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF8E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03FF8E: bne.w $3ff9c
pub fn i_03ff8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FF9C; return 10; }
    8
}
/// $03FF92: lea.l $40014(pc), a1
pub fn i_03ff92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40014;
    16
}
/// $03FF96: move.l a1, (a6)
pub fn i_03ff96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FF98: bra.w $3ffbc
pub fn i_03ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FF9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFBC;
    10
}
/// $03FF9C: btst.b #$5, $5a(a6)
pub fn i_03ff9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFA2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03FFA2: bne.w $3ffb6
pub fn i_03ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FFB6; return 10; }
    8
}
/// $03FFA6: tst.b $98(a6)
pub fn i_03ffa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFAA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $03FFAA: bne.w $3ffbc
pub fn i_03ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3FFBC; return 10; }
    8
}
/// $03FFAE: subq.w #$1, $5c(a6)
pub fn i_03ffae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFB2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03FFB2: bcc.w $3ffbc
pub fn i_03ffb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3FFBC; return 10; }
    8
}
/// $03FFB6: lea.l $40054(pc), a1
pub fn i_03ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40054;
    16
}
/// $03FFBA: move.l a1, (a6)
pub fn i_03ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FFBC: jsr $770cc.l
pub fn i_03ffbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FFC2);
    cpu.pc = 0x770CC;
    16
}
/// $03FFC2: tst.w d0
pub fn i_03ffc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFC4;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $03FFC4: bmi.w $3ffe4
pub fn i_03ffc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFC8;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x3FFE4; return 10; }
    8
}
/// $03FFC8: move.b d0, $72(a6)
pub fn i_03ffc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFCC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FFCC: andi.b #$1, d1
pub fn i_03ffcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFD0;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03FFD0: beq.w $3ffde
pub fn i_03ffd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3FFDE; return 10; }
    8
}
/// $03FFD4: lea.l $4008c(pc), a1
pub fn i_03ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4008C;
    16
}
/// $03FFD8: move.l a1, (a6)
pub fn i_03ffd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FFDA: bra.w $3ffe4
pub fn i_03ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3FFE4;
    10
}
/// $03FFDE: lea.l $401a8(pc), a1
pub fn i_03ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x401A8;
    16
}
/// $03FFE2: move.l a1, (a6)
pub fn i_03ffe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03FFE4: jsr $28d70.l
pub fn i_03ffe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3FFEA);
    cpu.pc = 0x28D70;
    16
}
/// $03FFEA: lea.l $3ef9a(pc), a1
pub fn i_03ffea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EF9A;
    16
}
/// $03FFEE: bsr.w $3ee48
pub fn i_03ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFF2;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x3FFF2);
    cpu.pc = 0x3EE48;
    18
}
/// $03FFF2: bcc.w $40000
pub fn i_03fff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x40000; return 10; }
    8
}
/// $03FFF6: move.b d0, $71(a6)
pub fn i_03fff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFFA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03FFFA: lea.l $40268(pc), a1
pub fn i_03fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3FFFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x40268;
    16
}
/// $03FFFE: move.l a1, (a6)
pub fn i_03fffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x40000;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x039B58, i_039b58),
    (0x039B5C, i_039b5c),
    (0x039B60, i_039b60),
    (0x039B66, i_039b66),
    (0x039B92, i_039b92),
    (0x039E54, i_039e54),
    (0x039E5A, i_039e5a),
    (0x039E5E, i_039e5e),
    (0x039E64, i_039e64),
    (0x039E68, i_039e68),
    (0x039E6C, i_039e6c),
    (0x039E72, i_039e72),
    (0x039E76, i_039e76),
    (0x039E7C, i_039e7c),
    (0x039E80, i_039e80),
    (0x039E84, i_039e84),
    (0x039E88, i_039e88),
    (0x039E8E, i_039e8e),
    (0x039EBA, i_039eba),
    (0x039EBE, i_039ebe),
    (0x039EC4, i_039ec4),
    (0x039EC8, i_039ec8),
    (0x039ECE, i_039ece),
    (0x039ED2, i_039ed2),
    (0x039ED6, i_039ed6),
    (0x039EDA, i_039eda),
    (0x039EE0, i_039ee0),
    (0x039F0C, i_039f0c),
    (0x039F10, i_039f10),
    (0x039F16, i_039f16),
    (0x039F1A, i_039f1a),
    (0x039F1E, i_039f1e),
    (0x039F22, i_039f22),
    (0x039F28, i_039f28),
    (0x039F54, i_039f54),
    (0x039F56, i_039f56),
    (0x039F5C, i_039f5c),
    (0x039F60, i_039f60),
    (0x039F66, i_039f66),
    (0x039F6A, i_039f6a),
    (0x039F6E, i_039f6e),
    (0x039F74, i_039f74),
    (0x039F78, i_039f78),
    (0x039F7E, i_039f7e),
    (0x039F82, i_039f82),
    (0x039F86, i_039f86),
    (0x039F8A, i_039f8a),
    (0x039F90, i_039f90),
    (0x039FBC, i_039fbc),
    (0x039FC0, i_039fc0),
    (0x039FC6, i_039fc6),
    (0x039FCA, i_039fca),
    (0x039FD0, i_039fd0),
    (0x039FD4, i_039fd4),
    (0x039FD8, i_039fd8),
    (0x039FDC, i_039fdc),
    (0x039FE2, i_039fe2),
    (0x03A00E, i_03a00e),
    (0x03A012, i_03a012),
    (0x03A018, i_03a018),
    (0x03A01C, i_03a01c),
    (0x03A020, i_03a020),
    (0x03A024, i_03a024),
    (0x03A02A, i_03a02a),
    (0x03A056, i_03a056),
    (0x03A058, i_03a058),
    (0x03A05E, i_03a05e),
    (0x03A062, i_03a062),
    (0x03A068, i_03a068),
    (0x03A06C, i_03a06c),
    (0x03A070, i_03a070),
    (0x03A076, i_03a076),
    (0x03A07A, i_03a07a),
    (0x03A080, i_03a080),
    (0x03A084, i_03a084),
    (0x03A088, i_03a088),
    (0x03A08C, i_03a08c),
    (0x03A092, i_03a092),
    (0x03A0BE, i_03a0be),
    (0x03A0C2, i_03a0c2),
    (0x03A0C8, i_03a0c8),
    (0x03A0CC, i_03a0cc),
    (0x03A0D0, i_03a0d0),
    (0x03A0D4, i_03a0d4),
    (0x03A0DA, i_03a0da),
    (0x03A106, i_03a106),
    (0x03A108, i_03a108),
    (0x03A10E, i_03a10e),
    (0x03A112, i_03a112),
    (0x03A118, i_03a118),
    (0x03A11C, i_03a11c),
    (0x03A120, i_03a120),
    (0x03A126, i_03a126),
    (0x03A12A, i_03a12a),
    (0x03A130, i_03a130),
    (0x03A134, i_03a134),
    (0x03A138, i_03a138),
    (0x03A13C, i_03a13c),
    (0x03A142, i_03a142),
    (0x03A16E, i_03a16e),
    (0x03A172, i_03a172),
    (0x03A178, i_03a178),
    (0x03A17C, i_03a17c),
    (0x03A180, i_03a180),
    (0x03A184, i_03a184),
    (0x03A18A, i_03a18a),
    (0x03A1B6, i_03a1b6),
    (0x03A1B8, i_03a1b8),
    (0x03A1BE, i_03a1be),
    (0x03A1C2, i_03a1c2),
    (0x03A1C8, i_03a1c8),
    (0x03A1CC, i_03a1cc),
    (0x03A1D0, i_03a1d0),
    (0x03A1D6, i_03a1d6),
    (0x03A1DA, i_03a1da),
    (0x03A1E0, i_03a1e0),
    (0x03A1E4, i_03a1e4),
    (0x03A1E8, i_03a1e8),
    (0x03A1EC, i_03a1ec),
    (0x03A1F2, i_03a1f2),
    (0x03A21E, i_03a21e),
    (0x03A222, i_03a222),
    (0x03A228, i_03a228),
    (0x03A22C, i_03a22c),
    (0x03A230, i_03a230),
    (0x03A234, i_03a234),
    (0x03A23A, i_03a23a),
    (0x03A266, i_03a266),
    (0x03A268, i_03a268),
    (0x03A26E, i_03a26e),
    (0x03A272, i_03a272),
    (0x03A278, i_03a278),
    (0x03A27C, i_03a27c),
    (0x03A280, i_03a280),
    (0x03A286, i_03a286),
    (0x03A28A, i_03a28a),
    (0x03A290, i_03a290),
    (0x03A294, i_03a294),
    (0x03A298, i_03a298),
    (0x03A29C, i_03a29c),
    (0x03A2A2, i_03a2a2),
    (0x03A2CE, i_03a2ce),
    (0x03A2D2, i_03a2d2),
    (0x03A2D8, i_03a2d8),
    (0x03A2DC, i_03a2dc),
    (0x03A2E0, i_03a2e0),
    (0x03A2E4, i_03a2e4),
    (0x03A2EA, i_03a2ea),
    (0x03A316, i_03a316),
    (0x03A35E, i_03a35e),
    (0x03A364, i_03a364),
    (0x03A368, i_03a368),
    (0x03A36C, i_03a36c),
    (0x03A370, i_03a370),
    (0x03A376, i_03a376),
    (0x03A3A2, i_03a3a2),
    (0x03A4C8, i_03a4c8),
    (0x03A4CE, i_03a4ce),
    (0x03A4D4, i_03a4d4),
    (0x03A4D8, i_03a4d8),
    (0x03A4DC, i_03a4dc),
    (0x03A4E0, i_03a4e0),
    (0x03A4E6, i_03a4e6),
    (0x03A512, i_03a512),
    (0x03A560, i_03a560),
    (0x03A564, i_03a564),
    (0x03A56A, i_03a56a),
    (0x03A56E, i_03a56e),
    (0x03A574, i_03a574),
    (0x03A578, i_03a578),
    (0x03A57C, i_03a57c),
    (0x03A580, i_03a580),
    (0x03A586, i_03a586),
    (0x03A5B2, i_03a5b2),
    (0x03A5B6, i_03a5b6),
    (0x03A5BC, i_03a5bc),
    (0x03A5C0, i_03a5c0),
    (0x03A5C4, i_03a5c4),
    (0x03A5C8, i_03a5c8),
    (0x03A5CE, i_03a5ce),
    (0x03A5FA, i_03a5fa),
    (0x03A5FE, i_03a5fe),
    (0x03A602, i_03a602),
    (0x03A608, i_03a608),
    (0x03A60A, i_03a60a),
    (0x03A610, i_03a610),
    (0x03A616, i_03a616),
    (0x03A61A, i_03a61a),
    (0x03A61E, i_03a61e),
    (0x03A622, i_03a622),
    (0x03A628, i_03a628),
    (0x03A654, i_03a654),
    (0x03A656, i_03a656),
    (0x03A65C, i_03a65c),
    (0x03A662, i_03a662),
    (0x03A666, i_03a666),
    (0x03A66A, i_03a66a),
    (0x03A66E, i_03a66e),
    (0x03A674, i_03a674),
    (0x03A6A0, i_03a6a0),
    (0x03AAE0, i_03aae0),
    (0x03AAE6, i_03aae6),
    (0x03AAEA, i_03aaea),
    (0x03AAF0, i_03aaf0),
    (0x03AAF4, i_03aaf4),
    (0x03AAF8, i_03aaf8),
    (0x03AAFC, i_03aafc),
    (0x03AB02, i_03ab02),
    (0x03AB2E, i_03ab2e),
    (0x03AB32, i_03ab32),
    (0x03AB38, i_03ab38),
    (0x03AB3C, i_03ab3c),
    (0x03AB42, i_03ab42),
    (0x03AB46, i_03ab46),
    (0x03AB4A, i_03ab4a),
    (0x03AB4E, i_03ab4e),
    (0x03AB54, i_03ab54),
    (0x03AB80, i_03ab80),
    (0x03AB84, i_03ab84),
    (0x03AB8A, i_03ab8a),
    (0x03AB8E, i_03ab8e),
    (0x03AB94, i_03ab94),
    (0x03AB98, i_03ab98),
    (0x03AB9C, i_03ab9c),
    (0x03ABA0, i_03aba0),
    (0x03ABA6, i_03aba6),
    (0x03ABD2, i_03abd2),
    (0x03ABD6, i_03abd6),
    (0x03ABDC, i_03abdc),
    (0x03ABE0, i_03abe0),
    (0x03ABE4, i_03abe4),
    (0x03ABE8, i_03abe8),
    (0x03ABEE, i_03abee),
    (0x03AC1A, i_03ac1a),
    (0x03AC1C, i_03ac1c),
    (0x03AC22, i_03ac22),
    (0x03AC26, i_03ac26),
    (0x03AC2C, i_03ac2c),
    (0x03AC30, i_03ac30),
    (0x03AC34, i_03ac34),
    (0x03AC38, i_03ac38),
    (0x03AC3E, i_03ac3e),
    (0x03AC6A, i_03ac6a),
    (0x03AC6E, i_03ac6e),
    (0x03AC74, i_03ac74),
    (0x03AC78, i_03ac78),
    (0x03AC7E, i_03ac7e),
    (0x03AC82, i_03ac82),
    (0x03AC86, i_03ac86),
    (0x03AC8A, i_03ac8a),
    (0x03AC90, i_03ac90),
    (0x03ACBC, i_03acbc),
    (0x03ACC0, i_03acc0),
    (0x03ACC6, i_03acc6),
    (0x03ACCA, i_03acca),
    (0x03ACD0, i_03acd0),
    (0x03ACD4, i_03acd4),
    (0x03ACD8, i_03acd8),
    (0x03ACDC, i_03acdc),
    (0x03ACE2, i_03ace2),
    (0x03AD0E, i_03ad0e),
    (0x03AD12, i_03ad12),
    (0x03AD18, i_03ad18),
    (0x03AD1C, i_03ad1c),
    (0x03AD20, i_03ad20),
    (0x03AD24, i_03ad24),
    (0x03AD2A, i_03ad2a),
    (0x03AD56, i_03ad56),
    (0x03B952, i_03b952),
    (0x03B958, i_03b958),
    (0x03B95C, i_03b95c),
    (0x03B962, i_03b962),
    (0x03B966, i_03b966),
    (0x03B96A, i_03b96a),
    (0x03B96E, i_03b96e),
    (0x03B974, i_03b974),
    (0x03B9A0, i_03b9a0),
    (0x03B9A4, i_03b9a4),
    (0x03B9AA, i_03b9aa),
    (0x03B9AE, i_03b9ae),
    (0x03B9B4, i_03b9b4),
    (0x03B9B8, i_03b9b8),
    (0x03B9BC, i_03b9bc),
    (0x03B9C0, i_03b9c0),
    (0x03B9C6, i_03b9c6),
    (0x03B9F2, i_03b9f2),
    (0x03B9F6, i_03b9f6),
    (0x03B9FC, i_03b9fc),
    (0x03BA00, i_03ba00),
    (0x03BA04, i_03ba04),
    (0x03BA08, i_03ba08),
    (0x03BA0E, i_03ba0e),
    (0x03BA3A, i_03ba3a),
    (0x03BD58, i_03bd58),
    (0x03BD5E, i_03bd5e),
    (0x03BD64, i_03bd64),
    (0x03BD6A, i_03bd6a),
    (0x03BD6E, i_03bd6e),
    (0x03BD72, i_03bd72),
    (0x03BD76, i_03bd76),
    (0x03BD7C, i_03bd7c),
    (0x03BDA8, i_03bda8),
    (0x03BDAA, i_03bdaa),
    (0x03BDB0, i_03bdb0),
    (0x03BDB6, i_03bdb6),
    (0x03BDBC, i_03bdbc),
    (0x03BDC0, i_03bdc0),
    (0x03BDC4, i_03bdc4),
    (0x03BDC8, i_03bdc8),
    (0x03BDCE, i_03bdce),
    (0x03BDFA, i_03bdfa),
    (0x03BEA0, i_03bea0),
    (0x03BEA6, i_03bea6),
    (0x03BEAC, i_03beac),
    (0x03BEB0, i_03beb0),
    (0x03BEB4, i_03beb4),
    (0x03BEB8, i_03beb8),
    (0x03BEBE, i_03bebe),
    (0x03BEEA, i_03beea),
    (0x03BEEC, i_03beec),
    (0x03BEF2, i_03bef2),
    (0x03BEF8, i_03bef8),
    (0x03BEFC, i_03befc),
    (0x03BF00, i_03bf00),
    (0x03BF04, i_03bf04),
    (0x03BF0A, i_03bf0a),
    (0x03BF36, i_03bf36),
    (0x03BF84, i_03bf84),
    (0x03BF8A, i_03bf8a),
    (0x03BF90, i_03bf90),
    (0x03BF94, i_03bf94),
    (0x03BF98, i_03bf98),
    (0x03BF9C, i_03bf9c),
    (0x03BFA2, i_03bfa2),
    (0x03BFCE, i_03bfce),
    (0x03BFD0, i_03bfd0),
    (0x03BFD6, i_03bfd6),
    (0x03BFDC, i_03bfdc),
    (0x03BFE0, i_03bfe0),
    (0x03BFE6, i_03bfe6),
    (0x03BFEA, i_03bfea),
    (0x03BFEE, i_03bfee),
    (0x03BFF2, i_03bff2),
    (0x03BFF8, i_03bff8),
    (0x03C024, i_03c024),
    (0x03C028, i_03c028),
    (0x03C02E, i_03c02e),
    (0x03C032, i_03c032),
    (0x03C036, i_03c036),
    (0x03C03A, i_03c03a),
    (0x03C040, i_03c040),
    (0x03C06C, i_03c06c),
    (0x03C10C, i_03c10c),
    (0x03C112, i_03c112),
    (0x03C118, i_03c118),
    (0x03C11C, i_03c11c),
    (0x03C122, i_03c122),
    (0x03C126, i_03c126),
    (0x03C12A, i_03c12a),
    (0x03C12E, i_03c12e),
    (0x03C134, i_03c134),
    (0x03C160, i_03c160),
    (0x03C164, i_03c164),
    (0x03C16A, i_03c16a),
    (0x03C16E, i_03c16e),
    (0x03C172, i_03c172),
    (0x03C176, i_03c176),
    (0x03C17C, i_03c17c),
    (0x03C1A8, i_03c1a8),
    (0x03C28E, i_03c28e),
    (0x03C294, i_03c294),
    (0x03C298, i_03c298),
    (0x03C29C, i_03c29c),
    (0x03C2A0, i_03c2a0),
    (0x03C2A6, i_03c2a6),
    (0x03C2D2, i_03c2d2),
    (0x03C2D4, i_03c2d4),
    (0x03C2D8, i_03c2d8),
    (0x03C2DC, i_03c2dc),
    (0x03C2E0, i_03c2e0),
    (0x03C2E6, i_03c2e6),
    (0x03C2EA, i_03c2ea),
    (0x03C2EE, i_03c2ee),
    (0x03C2F2, i_03c2f2),
    (0x03C2F8, i_03c2f8),
    (0x03C324, i_03c324),
    (0x03C328, i_03c328),
    (0x03C32E, i_03c32e),
    (0x03C332, i_03c332),
    (0x03C336, i_03c336),
    (0x03C33A, i_03c33a),
    (0x03C340, i_03c340),
    (0x03C36C, i_03c36c),
    (0x03C36E, i_03c36e),
    (0x03C374, i_03c374),
    (0x03C378, i_03c378),
    (0x03C37E, i_03c37e),
    (0x03C382, i_03c382),
    (0x03C386, i_03c386),
    (0x03C38A, i_03c38a),
    (0x03C390, i_03c390),
    (0x03C3BC, i_03c3bc),
    (0x03C3C0, i_03c3c0),
    (0x03C3C6, i_03c3c6),
    (0x03C3CA, i_03c3ca),
    (0x03C3D0, i_03c3d0),
    (0x03C3D4, i_03c3d4),
    (0x03C3D8, i_03c3d8),
    (0x03C3DC, i_03c3dc),
    (0x03C3E2, i_03c3e2),
    (0x03C40E, i_03c40e),
    (0x03C412, i_03c412),
    (0x03C418, i_03c418),
    (0x03C41C, i_03c41c),
    (0x03C420, i_03c420),
    (0x03C424, i_03c424),
    (0x03C42A, i_03c42a),
    (0x03C456, i_03c456),
    (0x03C632, i_03c632),
    (0x03C638, i_03c638),
    (0x03C63E, i_03c63e),
    (0x03C642, i_03c642),
    (0x03C646, i_03c646),
    (0x03C64A, i_03c64a),
    (0x03C650, i_03c650),
    (0x03C67C, i_03c67c),
    (0x03C710, i_03c710),
    (0x03C716, i_03c716),
    (0x03C718, i_03c718),
    (0x03C71E, i_03c71e),
    (0x03C722, i_03c722),
    (0x03C724, i_03c724),
    (0x03C728, i_03c728),
    (0x03C72C, i_03c72c),
    (0x03C730, i_03c730),
    (0x03C736, i_03c736),
    (0x03C73A, i_03c73a),
    (0x03C740, i_03c740),
    (0x03C744, i_03c744),
    (0x03C74A, i_03c74a),
    (0x03C750, i_03c750),
    (0x03C756, i_03c756),
    (0x03C75C, i_03c75c),
    (0x03C762, i_03c762),
    (0x03C768, i_03c768),
    (0x03C76E, i_03c76e),
    (0x03C774, i_03c774),
    (0x03C776, i_03c776),
    (0x03C77A, i_03c77a),
    (0x03C780, i_03c780),
    (0x03C784, i_03c784),
    (0x03C788, i_03c788),
    (0x03C78C, i_03c78c),
    (0x03C790, i_03c790),
    (0x03C794, i_03c794),
    (0x03C798, i_03c798),
    (0x03C79A, i_03c79a),
    (0x03C79E, i_03c79e),
    (0x03C7A4, i_03c7a4),
    (0x03C7A8, i_03c7a8),
    (0x03C7AA, i_03c7aa),
    (0x03C7AE, i_03c7ae),
    (0x03C7B2, i_03c7b2),
    (0x03C7B6, i_03c7b6),
    (0x03C7B8, i_03c7b8),
    (0x03C7BC, i_03c7bc),
    (0x03C7C2, i_03c7c2),
    (0x03C7C6, i_03c7c6),
    (0x03C7FC, i_03c7fc),
    (0x03C800, i_03c800),
    (0x03C806, i_03c806),
    (0x03C80C, i_03c80c),
    (0x03C810, i_03c810),
    (0x03C814, i_03c814),
    (0x03C818, i_03c818),
    (0x03C81C, i_03c81c),
    (0x03C820, i_03c820),
    (0x03C824, i_03c824),
    (0x03C826, i_03c826),
    (0x03C82A, i_03c82a),
    (0x03C830, i_03c830),
    (0x03C834, i_03c834),
    (0x03C836, i_03c836),
    (0x03C83A, i_03c83a),
    (0x03C83E, i_03c83e),
    (0x03C842, i_03c842),
    (0x03C844, i_03c844),
    (0x03C848, i_03c848),
    (0x03C84E, i_03c84e),
    (0x03C852, i_03c852),
    (0x03D616, i_03d616),
    (0x03D61E, i_03d61e),
    (0x03D622, i_03d622),
    (0x03D626, i_03d626),
    (0x03D62C, i_03d62c),
    (0x03D632, i_03d632),
    (0x03D638, i_03d638),
    (0x03D63E, i_03d63e),
    (0x03D944, i_03d944),
    (0x03D94A, i_03d94a),
    (0x03D950, i_03d950),
    (0x03D954, i_03d954),
    (0x03D958, i_03d958),
    (0x03D95E, i_03d95e),
    (0x03D962, i_03d962),
    (0x03D968, i_03d968),
    (0x03D96C, i_03d96c),
    (0x03D972, i_03d972),
    (0x03D978, i_03d978),
    (0x03D97E, i_03d97e),
    (0x03D984, i_03d984),
    (0x03D98A, i_03d98a),
    (0x03D98E, i_03d98e),
    (0x03D994, i_03d994),
    (0x03D99A, i_03d99a),
    (0x03D99E, i_03d99e),
    (0x03D9A2, i_03d9a2),
    (0x03D9A8, i_03d9a8),
    (0x03D9AC, i_03d9ac),
    (0x03D9B2, i_03d9b2),
    (0x03D9B6, i_03d9b6),
    (0x03D9BC, i_03d9bc),
    (0x03D9C2, i_03d9c2),
    (0x03D9C8, i_03d9c8),
    (0x03D9CE, i_03d9ce),
    (0x03D9D4, i_03d9d4),
    (0x03D9D8, i_03d9d8),
    (0x03D9DC, i_03d9dc),
    (0x03D9DE, i_03d9de),
    (0x03D9E2, i_03d9e2),
    (0x03D9E8, i_03d9e8),
    (0x03D9EC, i_03d9ec),
    (0x03D9EE, i_03d9ee),
    (0x03D9F0, i_03d9f0),
    (0x03D9F4, i_03d9f4),
    (0x03D9F8, i_03d9f8),
    (0x03D9FE, i_03d9fe),
    (0x03DA2A, i_03da2a),
    (0x03DA30, i_03da30),
    (0x03DA32, i_03da32),
    (0x03DA36, i_03da36),
    (0x03DA3A, i_03da3a),
    (0x03DA3E, i_03da3e),
    (0x03DA42, i_03da42),
    (0x03DA46, i_03da46),
    (0x03DA4A, i_03da4a),
    (0x03DA4C, i_03da4c),
    (0x03DA52, i_03da52),
    (0x03DA58, i_03da58),
    (0x03DA5C, i_03da5c),
    (0x03DA60, i_03da60),
    (0x03DA64, i_03da64),
    (0x03DA68, i_03da68),
    (0x03DA6C, i_03da6c),
    (0x03DA70, i_03da70),
    (0x03DA74, i_03da74),
    (0x03DA78, i_03da78),
    (0x03DA7E, i_03da7e),
    (0x03DA84, i_03da84),
    (0x03DA8A, i_03da8a),
    (0x03DA8E, i_03da8e),
    (0x03DA92, i_03da92),
    (0x03DA94, i_03da94),
    (0x03DA98, i_03da98),
    (0x03DA9E, i_03da9e),
    (0x03DAA0, i_03daa0),
    (0x03EE3A, i_03ee3a),
    (0x03EE40, i_03ee40),
    (0x03EE46, i_03ee46),
    (0x03EE48, i_03ee48),
    (0x03EE4E, i_03ee4e),
    (0x03EE52, i_03ee52),
    (0x03EE56, i_03ee56),
    (0x03EE58, i_03ee58),
    (0x03EE5E, i_03ee5e),
    (0x03EE60, i_03ee60),
    (0x03EE66, i_03ee66),
    (0x03EE6A, i_03ee6a),
    (0x03EE6E, i_03ee6e),
    (0x03EE72, i_03ee72),
    (0x03EE74, i_03ee74),
    (0x03EE76, i_03ee76),
    (0x03EE7C, i_03ee7c),
    (0x03EE80, i_03ee80),
    (0x03EE84, i_03ee84),
    (0x03EE88, i_03ee88),
    (0x03EE8A, i_03ee8a),
    (0x03EE8E, i_03ee8e),
    (0x03EE90, i_03ee90),
    (0x03EE94, i_03ee94),
    (0x03EE98, i_03ee98),
    (0x03EE9C, i_03ee9c),
    (0x03EE9E, i_03ee9e),
    (0x03EEA0, i_03eea0),
    (0x03EEA2, i_03eea2),
    (0x03EEA6, i_03eea6),
    (0x03EEA8, i_03eea8),
    (0x03EEAC, i_03eeac),
    (0x03EEB0, i_03eeb0),
    (0x03EEB6, i_03eeb6),
    (0x03EEBC, i_03eebc),
    (0x03EEC0, i_03eec0),
    (0x03EEC2, i_03eec2),
    (0x03EEC6, i_03eec6),
    (0x03EECA, i_03eeca),
    (0x03EECC, i_03eecc),
    (0x03EED2, i_03eed2),
    (0x03EED6, i_03eed6),
    (0x03EEDA, i_03eeda),
    (0x03EEDC, i_03eedc),
    (0x03EEE2, i_03eee2),
    (0x03EEE4, i_03eee4),
    (0x03EEEA, i_03eeea),
    (0x03EEEE, i_03eeee),
    (0x03EEF2, i_03eef2),
    (0x03EEF6, i_03eef6),
    (0x03EEFA, i_03eefa),
    (0x03EEFC, i_03eefc),
    (0x03EEFE, i_03eefe),
    (0x03EF02, i_03ef02),
    (0x03EF04, i_03ef04),
    (0x03EF08, i_03ef08),
    (0x03EF0C, i_03ef0c),
    (0x03EF0E, i_03ef0e),
    (0x03EF12, i_03ef12),
    (0x03EF14, i_03ef14),
    (0x03EF1A, i_03ef1a),
    (0x03EF1E, i_03ef1e),
    (0x03EF22, i_03ef22),
    (0x03EF24, i_03ef24),
    (0x03EF2A, i_03ef2a),
    (0x03EF2C, i_03ef2c),
    (0x03EF32, i_03ef32),
    (0x03EF36, i_03ef36),
    (0x03EF3A, i_03ef3a),
    (0x03EF3E, i_03ef3e),
    (0x03EF42, i_03ef42),
    (0x03EF44, i_03ef44),
    (0x03EF46, i_03ef46),
    (0x03EF4A, i_03ef4a),
    (0x03EF4C, i_03ef4c),
    (0x03EF50, i_03ef50),
    (0x03EF54, i_03ef54),
    (0x03EF56, i_03ef56),
    (0x03EF5A, i_03ef5a),
    (0x03EF5C, i_03ef5c),
    (0x03EF62, i_03ef62),
    (0x03EF66, i_03ef66),
    (0x03EF6C, i_03ef6c),
    (0x03EF70, i_03ef70),
    (0x03EF74, i_03ef74),
    (0x03EF78, i_03ef78),
    (0x03EF7A, i_03ef7a),
    (0x03EF7E, i_03ef7e),
    (0x03EF82, i_03ef82),
    (0x03EF86, i_03ef86),
    (0x03EF8A, i_03ef8a),
    (0x03EF8E, i_03ef8e),
    (0x03EF92, i_03ef92),
    (0x03EF94, i_03ef94),
    (0x03EF98, i_03ef98),
    (0x03EFA2, i_03efa2),
    (0x03EFA6, i_03efa6),
    (0x03EFAC, i_03efac),
    (0x03EFB0, i_03efb0),
    (0x03EFB4, i_03efb4),
    (0x03EFB6, i_03efb6),
    (0x03EFBA, i_03efba),
    (0x03EFC0, i_03efc0),
    (0x03EFC4, i_03efc4),
    (0x03EFCA, i_03efca),
    (0x03EFCC, i_03efcc),
    (0x03EFCE, i_03efce),
    (0x03EFD2, i_03efd2),
    (0x03EFD8, i_03efd8),
    (0x03EFDC, i_03efdc),
    (0x03EFE2, i_03efe2),
    (0x03EFEC, i_03efec),
    (0x03EFF0, i_03eff0),
    (0x03EFF4, i_03eff4),
    (0x03F9B8, i_03f9b8),
    (0x03F9BC, i_03f9bc),
    (0x03F9C0, i_03f9c0),
    (0x03F9C6, i_03f9c6),
    (0x03F9CA, i_03f9ca),
    (0x03F9D0, i_03f9d0),
    (0x03F9D4, i_03f9d4),
    (0x03F9DA, i_03f9da),
    (0x03F9E0, i_03f9e0),
    (0x03F9E6, i_03f9e6),
    (0x03F9EA, i_03f9ea),
    (0x03F9EE, i_03f9ee),
    (0x03F9F2, i_03f9f2),
    (0x03F9F6, i_03f9f6),
    (0x03F9FC, i_03f9fc),
    (0x03FA00, i_03fa00),
    (0x03FA06, i_03fa06),
    (0x03FA0C, i_03fa0c),
    (0x03FC38, i_03fc38),
    (0x03FC3C, i_03fc3c),
    (0x03FC40, i_03fc40),
    (0x03FC46, i_03fc46),
    (0x03FC4A, i_03fc4a),
    (0x03FC4C, i_03fc4c),
    (0x03FC50, i_03fc50),
    (0x03FC56, i_03fc56),
    (0x03FC5C, i_03fc5c),
    (0x03FC60, i_03fc60),
    (0x03FC64, i_03fc64),
    (0x03FC66, i_03fc66),
    (0x03FC6A, i_03fc6a),
    (0x03FC6E, i_03fc6e),
    (0x03FC74, i_03fc74),
    (0x03FC78, i_03fc78),
    (0x03FC7A, i_03fc7a),
    (0x03FC80, i_03fc80),
    (0x03FC86, i_03fc86),
    (0x03FC8A, i_03fc8a),
    (0x03FC8E, i_03fc8e),
    (0x03FC90, i_03fc90),
    (0x03FC94, i_03fc94),
    (0x03FC9A, i_03fc9a),
    (0x03FCA0, i_03fca0),
    (0x03FCA4, i_03fca4),
    (0x03FCA8, i_03fca8),
    (0x03FCAE, i_03fcae),
    (0x03FCB2, i_03fcb2),
    (0x03FCB8, i_03fcb8),
    (0x03FCBC, i_03fcbc),
    (0x03FCBE, i_03fcbe),
    (0x03FCC0, i_03fcc0),
    (0x03FCC6, i_03fcc6),
    (0x03FCCC, i_03fccc),
    (0x03FCD0, i_03fcd0),
    (0x03FCD6, i_03fcd6),
    (0x03FE5A, i_03fe5a),
    (0x03FE5E, i_03fe5e),
    (0x03FE62, i_03fe62),
    (0x03FE64, i_03fe64),
    (0x03FE66, i_03fe66),
    (0x03FE6A, i_03fe6a),
    (0x03FE70, i_03fe70),
    (0x03FE74, i_03fe74),
    (0x03FE78, i_03fe78),
    (0x03FE7C, i_03fe7c),
    (0x03FE7E, i_03fe7e),
    (0x03FE82, i_03fe82),
    (0x03FE88, i_03fe88),
    (0x03FE8E, i_03fe8e),
    (0x03FE92, i_03fe92),
    (0x03FE96, i_03fe96),
    (0x03FE98, i_03fe98),
    (0x03FE9C, i_03fe9c),
    (0x03FEA0, i_03fea0),
    (0x03FEA6, i_03fea6),
    (0x03FEAA, i_03feaa),
    (0x03FEAC, i_03feac),
    (0x03FEB2, i_03feb2),
    (0x03FEB8, i_03feb8),
    (0x03FEBC, i_03febc),
    (0x03FEC0, i_03fec0),
    (0x03FEC2, i_03fec2),
    (0x03FECE, i_03fece),
    (0x03FED4, i_03fed4),
    (0x03FED8, i_03fed8),
    (0x03FEDC, i_03fedc),
    (0x03FEE0, i_03fee0),
    (0x03FEE4, i_03fee4),
    (0x03FEE8, i_03fee8),
    (0x03FEEC, i_03feec),
    (0x03FEF0, i_03fef0),
    (0x03FEF6, i_03fef6),
    (0x03FEFA, i_03fefa),
    (0x03FEFE, i_03fefe),
    (0x03FF04, i_03ff04),
    (0x03FF08, i_03ff08),
    (0x03FF0C, i_03ff0c),
    (0x03FF12, i_03ff12),
    (0x03FF14, i_03ff14),
    (0x03FF1A, i_03ff1a),
    (0x03FF1E, i_03ff1e),
    (0x03FF24, i_03ff24),
    (0x03FF28, i_03ff28),
    (0x03FF2C, i_03ff2c),
    (0x03FF30, i_03ff30),
    (0x03FF36, i_03ff36),
    (0x03FF3C, i_03ff3c),
    (0x03FF40, i_03ff40),
    (0x03FF44, i_03ff44),
    (0x03FF48, i_03ff48),
    (0x03FF4C, i_03ff4c),
    (0x03FF52, i_03ff52),
    (0x03FF56, i_03ff56),
    (0x03FF5C, i_03ff5c),
    (0x03FF62, i_03ff62),
    (0x03FF66, i_03ff66),
    (0x03FF6C, i_03ff6c),
    (0x03FF70, i_03ff70),
    (0x03FF76, i_03ff76),
    (0x03FF7A, i_03ff7a),
    (0x03FF7C, i_03ff7c),
    (0x03FF80, i_03ff80),
    (0x03FF84, i_03ff84),
    (0x03FF86, i_03ff86),
    (0x03FF8A, i_03ff8a),
    (0x03FF8E, i_03ff8e),
    (0x03FF92, i_03ff92),
    (0x03FF96, i_03ff96),
    (0x03FF98, i_03ff98),
    (0x03FF9C, i_03ff9c),
    (0x03FFA2, i_03ffa2),
    (0x03FFA6, i_03ffa6),
    (0x03FFAA, i_03ffaa),
    (0x03FFAE, i_03ffae),
    (0x03FFB2, i_03ffb2),
    (0x03FFB6, i_03ffb6),
    (0x03FFBA, i_03ffba),
    (0x03FFBC, i_03ffbc),
    (0x03FFC2, i_03ffc2),
    (0x03FFC4, i_03ffc4),
    (0x03FFC8, i_03ffc8),
    (0x03FFCC, i_03ffcc),
    (0x03FFD0, i_03ffd0),
    (0x03FFD4, i_03ffd4),
    (0x03FFD8, i_03ffd8),
    (0x03FFDA, i_03ffda),
    (0x03FFDE, i_03ffde),
    (0x03FFE2, i_03ffe2),
    (0x03FFE4, i_03ffe4),
    (0x03FFEA, i_03ffea),
    (0x03FFEE, i_03ffee),
    (0x03FFF2, i_03fff2),
    (0x03FFF6, i_03fff6),
    (0x03FFFA, i_03fffa),
    (0x03FFFE, i_03fffe),
];
