// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $039C98: beq.w $39cea
pub fn i_039c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39C9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39CEA; return 10; }
    8
}
/// $039C9C: bclr.b #$2, $8c(a6)
pub fn i_039c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CA2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039CA2: bclr.b #$2, $8c(a6)
pub fn i_039ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CA8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039CA8: jsr $3933a(pc)
pub fn i_039ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CAC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39CAC);
    cpu.pc = 0x3933A;
    16
}
/// $039CAC: lea.l $39cbe(pc), a0
pub fn i_039cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CB0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39CBE;
    16
}
/// $039CB0: movea.l (a0, d0.w), a0
pub fn i_039cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CB4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039CB4: jsr $28cd4.l
pub fn i_039cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39CBA);
    cpu.pc = 0x28CD4;
    16
}
/// $039CBA: bra.w $39ce6
pub fn i_039cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x39CE6;
    10
}
/// $039CE6: bra.w $39d80
pub fn i_039ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x39D80;
    10
}
/// $039CEA: jsr $2abcc.l
pub fn i_039cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39CF0);
    cpu.pc = 0x2ABCC;
    16
}
/// $039CF0: bcs.w $39d3c
pub fn i_039cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CF4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x39D3C; return 10; }
    8
}
/// $039CF4: bclr.b #$2, $8c(a6)
pub fn i_039cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CFA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039CFA: jsr $3933a(pc)
pub fn i_039cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39CFE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39CFE);
    cpu.pc = 0x3933A;
    16
}
/// $039CFE: lea.l $39d10(pc), a0
pub fn i_039cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D02;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39D10;
    16
}
/// $039D02: movea.l (a0, d0.w), a0
pub fn i_039d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D06;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039D06: jsr $28cd4.l
pub fn i_039d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39D0C);
    cpu.pc = 0x28CD4;
    16
}
/// $039D0C: bra.w $39d38
pub fn i_039d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D10;
    cpu.ir = 0x6000;
    cpu.pc = 0x39D38;
    10
}
/// $039D38: bra.w $39d80
pub fn i_039d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x39D80;
    10
}
/// $039D3C: bclr.b #$2, $8c(a6)
pub fn i_039d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D42;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039D42: jsr $3933a(pc)
pub fn i_039d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39D46);
    cpu.pc = 0x3933A;
    16
}
/// $039D46: lea.l $39d58(pc), a0
pub fn i_039d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D4A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39D58;
    16
}
/// $039D4A: movea.l (a0, d0.w), a0
pub fn i_039d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D4E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039D4E: jsr $28cd4.l
pub fn i_039d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39D54);
    cpu.pc = 0x28CD4;
    16
}
/// $039D54: bra.w $39d80
pub fn i_039d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D58;
    cpu.ir = 0x6000;
    cpu.pc = 0x39D80;
    10
}
/// $039D80: rts
pub fn i_039d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_MeleeA_039d82  ($039D82, 28 B, player/arm_weapon_fx_0388f0.s)
/// $039D82: bclr.b #$2, $8c(a6)
pub fn i_039d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039D88: jsr $3933a(pc)
pub fn i_039d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39D8C);
    cpu.pc = 0x3933A;
    16
}
/// $039D8C: lea.l $39d9e(pc), a0
pub fn i_039d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D90;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39D9E;
    16
}
/// $039D90: movea.l (a0, d0.w), a0
pub fn i_039d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D94;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039D94: jsr $28cd4.l
pub fn i_039d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39D9A);
    cpu.pc = 0x28CD4;
    16
}
/// $039D9A: bra.w $39dc6
pub fn i_039d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39D9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x39DC6;
    10
}
/// $039DC6: rts
pub fn i_039dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39DC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_MeleeB_039dc8  ($039DC8, 28 B, player/arm_weapon_fx_0388f0.s)
/// $039DC8: bclr.b #$2, $8c(a6)
pub fn i_039dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39DCE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039DCE: jsr $3933a(pc)
pub fn i_039dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39DD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39DD2);
    cpu.pc = 0x3933A;
    16
}
/// $039DD2: lea.l $39de4(pc), a0
pub fn i_039dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39DD6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39DE4;
    16
}
/// $039DD6: movea.l (a0, d0.w), a0
pub fn i_039dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39DDA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039DDA: jsr $28cd4.l
pub fn i_039dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39DE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39DE0);
    cpu.pc = 0x28CD4;
    16
}
/// $039DE0: bra.w $39e0c
pub fn i_039de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39DE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x39E0C;
    10
}
/// $039E0C: rts
pub fn i_039e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Pose_039e0e  ($039E0E, 28 B, player/arm_weapon_fx_0388f0.s)
/// $039E0E: bclr.b #$2, $8c(a6)
pub fn i_039e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E14;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039E14: jsr $3933a(pc)
pub fn i_039e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39E18);
    cpu.pc = 0x3933A;
    16
}
/// $039E18: lea.l $39e2a(pc), a0
pub fn i_039e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E1C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x39E2A;
    16
}
/// $039E1C: movea.l (a0, d0.w), a0
pub fn i_039e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E20;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039E20: jsr $28cd4.l
pub fn i_039e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39E26);
    cpu.pc = 0x28CD4;
    16
}
/// $039E26: bra.w $39e52
pub fn i_039e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x39E52;
    10
}
/// $039E52: rts
pub fn i_039e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39E54;
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
// ===== PlayerArm_WalkShootA_03a318  ($03A318, 28 B, player/arm_weapon_fx_0388f0.s)
/// $03A318: bclr.b #$2, $8c(a6)
pub fn i_03a318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A31E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A31E: jsr $3933a(pc)
pub fn i_03a31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A322;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A322);
    cpu.pc = 0x3933A;
    16
}
/// $03A322: lea.l $3a334(pc), a0
pub fn i_03a322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A326;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A334;
    16
}
/// $03A326: movea.l (a0, d0.w), a0
pub fn i_03a326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A32A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A32A: jsr $28cd4.l
pub fn i_03a32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A330;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A330);
    cpu.pc = 0x28CD4;
    16
}
/// $03A330: bra.w $3a35c
pub fn i_03a330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A334;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A35C;
    10
}
/// $03A35C: rts
pub fn i_03a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A35E;
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
// ===== PlayerArm_WalkShootC_03a3a4  ($03A3A4, 34 B, player/arm_weapon_fx_0388f0.s)
/// $03A3A4: bclr.b #$2, $8c(a6)
pub fn i_03a3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3AA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A3AA: bclr.b #$2, $8c(a6)
pub fn i_03a3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3B0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A3B0: jsr $3933a(pc)
pub fn i_03a3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A3B4);
    cpu.pc = 0x3933A;
    16
}
/// $03A3B4: lea.l $3a3c6(pc), a0
pub fn i_03a3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3B8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A3C6;
    16
}
/// $03A3B8: movea.l (a0, d0.w), a0
pub fn i_03a3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3BC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A3BC: jsr $28cd4.l
pub fn i_03a3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A3C2);
    cpu.pc = 0x28CD4;
    16
}
/// $03A3C2: bra.w $3a3ee
pub fn i_03a3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A3EE;
    10
}
/// $03A3EE: rts
pub fn i_03a3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WalkShootD_03a3f0  ($03A3F0, 34 B, player/arm_weapon_fx_0388f0.s)
/// $03A3F0: bclr.b #$2, $8c(a6)
pub fn i_03a3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3F6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A3F6: bclr.b #$2, $8c(a6)
pub fn i_03a3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A3FC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A3FC: jsr $3933a(pc)
pub fn i_03a3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A400;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A400);
    cpu.pc = 0x3933A;
    16
}
/// $03A400: lea.l $3a412(pc), a0
pub fn i_03a400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A404;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A412;
    16
}
/// $03A404: movea.l (a0, d0.w), a0
pub fn i_03a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A408;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A408: jsr $28cd4.l
pub fn i_03a408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A40E);
    cpu.pc = 0x28CD4;
    16
}
/// $03A40E: bra.w $3a43a
pub fn i_03a40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A412;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A43A;
    10
}
/// $03A43A: rts
pub fn i_03a43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A43C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WalkShootE_03a43c  ($03A43C, 28 B, player/arm_weapon_fx_0388f0.s)
/// $03A43C: bclr.b #$2, $8c(a6)
pub fn i_03a43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A442;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A442: jsr $3933a(pc)
pub fn i_03a442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A446;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A446);
    cpu.pc = 0x3933A;
    16
}
/// $03A446: lea.l $3a458(pc), a0
pub fn i_03a446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A44A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A458;
    16
}
/// $03A44A: movea.l (a0, d0.w), a0
pub fn i_03a44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A44E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A44E: jsr $28cd4.l
pub fn i_03a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A454);
    cpu.pc = 0x28CD4;
    16
}
/// $03A454: bra.w $3a480
pub fn i_03a454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A458;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A480;
    10
}
/// $03A480: rts
pub fn i_03a480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WalkShootF_03a482  ($03A482, 28 B, player/arm_weapon_fx_0388f0.s)
/// $03A482: bclr.b #$2, $8c(a6)
pub fn i_03a482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A488;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A488: jsr $3933a(pc)
pub fn i_03a488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A48C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A48C);
    cpu.pc = 0x3933A;
    16
}
/// $03A48C: lea.l $3a49e(pc), a0
pub fn i_03a48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A490;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A49E;
    16
}
/// $03A490: movea.l (a0, d0.w), a0
pub fn i_03a490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A494;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A494: jsr $28cd4.l
pub fn i_03a494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A49A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A49A);
    cpu.pc = 0x28CD4;
    16
}
/// $03A49A: bra.w $3a4c6
pub fn i_03a49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A49E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A4C6;
    10
}
/// $03A4C6: rts
pub fn i_03a4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A4C8;
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
// ===== PlayerArm_TurnB_03a514  ($03A514, 34 B, player/arm_weapon_fx_0388f0.s)
/// $03A514: bclr.b #$2, $8c(a6)
pub fn i_03a514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A51A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A51A: bclr.b #$2, $8c(a6)
pub fn i_03a51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A520;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A520: jsr $3933a(pc)
pub fn i_03a520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A524;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A524);
    cpu.pc = 0x3933A;
    16
}
/// $03A524: lea.l $3a536(pc), a0
pub fn i_03a524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A528;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A536;
    16
}
/// $03A528: movea.l (a0, d0.w), a0
pub fn i_03a528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A52C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A52C: jsr $28cd4.l
pub fn i_03a52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A532);
    cpu.pc = 0x28CD4;
    16
}
/// $03A532: bra.w $3a55e
pub fn i_03a532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A536;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A55E;
    10
}
/// $03A55E: rts
pub fn i_03a55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A560;
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
// ===== PlayerArm_AirUnref_03a6a2  ($03A6A2, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03A6A2: bclr.b #$2, $8c(a6)
pub fn i_03a6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A6A8: bclr.b #$2, $8c(a6)
pub fn i_03a6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A6AE: jsr $3933a(pc)
pub fn i_03a6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A6B2);
    cpu.pc = 0x3933A;
    16
}
/// $03A6B2: lea.l $3a6c4(pc), a0
pub fn i_03a6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6B6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A6C4;
    16
}
/// $03A6B6: movea.l (a0, d0.w), a0
pub fn i_03a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6BA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A6BA: jsr $28cd4.l
pub fn i_03a6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A6C0);
    cpu.pc = 0x28CD4;
    16
}
/// $03A6C0: bra.w $3a6ec
pub fn i_03a6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A6EC;
    10
}
/// $03A6EC: rts
pub fn i_03a6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_AirB_03a6ee  ($03A6EE, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03A6EE: bclr.b #$2, $8c(a6)
pub fn i_03a6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A6F4: bclr.b #$2, $8c(a6)
pub fn i_03a6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6FA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A6FA: jsr $3933a(pc)
pub fn i_03a6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A6FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A6FE);
    cpu.pc = 0x3933A;
    16
}
/// $03A6FE: lea.l $3a710(pc), a0
pub fn i_03a6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A702;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A710;
    16
}
/// $03A702: movea.l (a0, d0.w), a0
pub fn i_03a702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A706;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A706: jsr $28cd4.l
pub fn i_03a706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A70C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A70C);
    cpu.pc = 0x28CD4;
    16
}
/// $03A70C: bra.w $3a738
pub fn i_03a70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A710;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A738;
    10
}
/// $03A738: rts
pub fn i_03a738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A73A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_AirShootA_03a73a  ($03A73A, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03A73A: cmpi.w #$1, $72(a6)
pub fn i_03a73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A740;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03A740: bne.w $3a78c
pub fn i_03a740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A744;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A78C; return 10; }
    8
}
/// $03A744: bclr.b #$2, $8c(a6)
pub fn i_03a744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A74A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A74A: jsr $3933a(pc)
pub fn i_03a74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A74E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A74E);
    cpu.pc = 0x3933A;
    16
}
/// $03A74E: lea.l $3a760(pc), a0
pub fn i_03a74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A752;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A760;
    16
}
/// $03A752: movea.l (a0, d0.w), a0
pub fn i_03a752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A756;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A756: jsr $28cd4.l
pub fn i_03a756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A75C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A75C);
    cpu.pc = 0x28CD4;
    16
}
/// $03A75C: bra.w $3a788
pub fn i_03a75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A760;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A788;
    10
}
/// $03A788: bra.w $3a822
pub fn i_03a788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A78C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A822;
    10
}
/// $03A78C: cmpi.w #$3, $72(a6)
pub fn i_03a78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A792;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03A792: bne.w $3a7de
pub fn i_03a792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A796;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A7DE; return 10; }
    8
}
/// $03A796: bclr.b #$2, $8c(a6)
pub fn i_03a796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A79C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A79C: jsr $3933a(pc)
pub fn i_03a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A7A0);
    cpu.pc = 0x3933A;
    16
}
/// $03A7A0: lea.l $3a7b2(pc), a0
pub fn i_03a7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7A4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A7B2;
    16
}
/// $03A7A4: movea.l (a0, d0.w), a0
pub fn i_03a7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7A8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A7A8: jsr $28cd4.l
pub fn i_03a7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A7AE);
    cpu.pc = 0x28CD4;
    16
}
/// $03A7AE: bra.w $3a7da
pub fn i_03a7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A7DA;
    10
}
/// $03A7DA: bra.w $3a822
pub fn i_03a7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A822;
    10
}
/// $03A7DE: bclr.b #$2, $8c(a6)
pub fn i_03a7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7E4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A7E4: jsr $3933a(pc)
pub fn i_03a7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A7E8);
    cpu.pc = 0x3933A;
    16
}
/// $03A7E8: lea.l $3a7fa(pc), a0
pub fn i_03a7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A7FA;
    16
}
/// $03A7EC: movea.l (a0, d0.w), a0
pub fn i_03a7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7F0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A7F0: jsr $28cd4.l
pub fn i_03a7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A7F6);
    cpu.pc = 0x28CD4;
    16
}
/// $03A7F6: bra.w $3a822
pub fn i_03a7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A7FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A822;
    10
}
/// $03A822: rts
pub fn i_03a822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A824;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03A824: cmpi.w #$1, $72(a6)
pub fn i_03a824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A82A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03A82A: bne.w $3a874
pub fn i_03a82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A82E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A874; return 10; }
    8
}
/// $03A82E: bclr.b #$2, $8c(a6)
pub fn i_03a82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A834;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A834: jsr $3933a(pc)
pub fn i_03a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A838;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A838);
    cpu.pc = 0x3933A;
    16
}
/// $03A838: lea.l $3a84a(pc), a0
pub fn i_03a838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A83C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A84A;
    16
}
/// $03A83C: movea.l (a0, d0.w), a0
pub fn i_03a83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A840;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A840: jsr $28cd4.l
pub fn i_03a840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A846);
    cpu.pc = 0x28CD4;
    16
}
/// $03A846: bra.w $3a872
pub fn i_03a846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A84A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A872;
    10
}
/// $03A872: bra.b $3a822
pub fn i_03a872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A874;
    cpu.ir = 0x60AE;
    cpu.pc = 0x3A822;
    10
}
/// $03A874: cmpi.w #$3, $72(a6)
pub fn i_03a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A87A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03A87A: bne.w $3a8c6
pub fn i_03a87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A87E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A8C6; return 10; }
    8
}
/// $03A87E: bclr.b #$2, $8c(a6)
pub fn i_03a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A884;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A884: jsr $3933a(pc)
pub fn i_03a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A888;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A888);
    cpu.pc = 0x3933A;
    16
}
/// $03A888: lea.l $3a89a(pc), a0
pub fn i_03a888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A88C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A89A;
    16
}
/// $03A88C: movea.l (a0, d0.w), a0
pub fn i_03a88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A890;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A890: jsr $28cd4.l
pub fn i_03a890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A896);
    cpu.pc = 0x28CD4;
    16
}
/// $03A896: bra.w $3a8c2
pub fn i_03a896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A89A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A8C2;
    10
}
/// $03A8C2: bra.w $3a90a
pub fn i_03a8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A8C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A90A;
    10
}
/// $03A8C6: bclr.b #$2, $8c(a6)
pub fn i_03a8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A8CC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A8CC: jsr $3933a(pc)
pub fn i_03a8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A8D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A8D0);
    cpu.pc = 0x3933A;
    16
}
/// $03A8D0: lea.l $3a8e2(pc), a0
pub fn i_03a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A8D4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A8E2;
    16
}
/// $03A8D4: movea.l (a0, d0.w), a0
pub fn i_03a8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A8D8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A8D8: jsr $28cd4.l
pub fn i_03a8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A8DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A8DE);
    cpu.pc = 0x28CD4;
    16
}
/// $03A8DE: bra.w $3a90a
pub fn i_03a8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A8E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A90A;
    10
}
/// $03A90A: rts
pub fn i_03a90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A90C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_AirShootB_03a90c  ($03A90C, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03A90C: cmpi.w #$0, $72(a6)
pub fn i_03a90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A912;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03A912: bne.w $3a95e
pub fn i_03a912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A916;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A95E; return 10; }
    8
}
/// $03A916: bclr.b #$2, $8c(a6)
pub fn i_03a916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A91C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A91C: jsr $3933a(pc)
pub fn i_03a91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A920;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A920);
    cpu.pc = 0x3933A;
    16
}
/// $03A920: lea.l $3a932(pc), a0
pub fn i_03a920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A924;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A932;
    16
}
/// $03A924: movea.l (a0, d0.w), a0
pub fn i_03a924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A928;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A928: jsr $28cd4.l
pub fn i_03a928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A92E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A92E);
    cpu.pc = 0x28CD4;
    16
}
/// $03A92E: bra.w $3a95a
pub fn i_03a92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A932;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A95A;
    10
}
/// $03A95A: bra.w $3a9f4
pub fn i_03a95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A95E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A9F4;
    10
}
/// $03A95E: cmpi.w #$1, $72(a6)
pub fn i_03a95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A964;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03A964: bne.w $3a9b0
pub fn i_03a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A968;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3A9B0; return 10; }
    8
}
/// $03A968: bclr.b #$2, $8c(a6)
pub fn i_03a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A96E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A96E: jsr $3933a(pc)
pub fn i_03a96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A972;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A972);
    cpu.pc = 0x3933A;
    16
}
/// $03A972: lea.l $3a984(pc), a0
pub fn i_03a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A976;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A984;
    16
}
/// $03A976: movea.l (a0, d0.w), a0
pub fn i_03a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A97A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A97A: jsr $28cd4.l
pub fn i_03a97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A980;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A980);
    cpu.pc = 0x28CD4;
    16
}
/// $03A980: bra.w $3a9ac
pub fn i_03a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A984;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A9AC;
    10
}
/// $03A9AC: bra.w $3a9f4
pub fn i_03a9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A9F4;
    10
}
/// $03A9B0: bclr.b #$2, $8c(a6)
pub fn i_03a9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9B6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03A9B6: jsr $3933a(pc)
pub fn i_03a9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3A9BA);
    cpu.pc = 0x3933A;
    16
}
/// $03A9BA: lea.l $3a9cc(pc), a0
pub fn i_03a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9BE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3A9CC;
    16
}
/// $03A9BE: movea.l (a0, d0.w), a0
pub fn i_03a9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9C2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03A9C2: jsr $28cd4.l
pub fn i_03a9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3A9C8);
    cpu.pc = 0x28CD4;
    16
}
/// $03A9C8: bra.w $3a9f4
pub fn i_03a9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3A9F4;
    10
}
/// $03A9F4: rts
pub fn i_03a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_AirShootC_03a9f6  ($03A9F6, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03A9F6: cmpi.w #$0, $72(a6)
pub fn i_03a9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3A9FC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03A9FC: bne.w $3aa48
pub fn i_03a9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AA48; return 10; }
    8
}
/// $03AA00: bclr.b #$2, $8c(a6)
pub fn i_03aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA06;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AA06: jsr $3933a(pc)
pub fn i_03aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AA0A);
    cpu.pc = 0x3933A;
    16
}
/// $03AA0A: lea.l $3aa1c(pc), a0
pub fn i_03aa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA0E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AA1C;
    16
}
/// $03AA0E: movea.l (a0, d0.w), a0
pub fn i_03aa0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA12;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AA12: jsr $28cd4.l
pub fn i_03aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AA18);
    cpu.pc = 0x28CD4;
    16
}
/// $03AA18: bra.w $3aa44
pub fn i_03aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AA44;
    10
}
/// $03AA44: bra.w $3aade
pub fn i_03aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA48;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AADE;
    10
}
/// $03AA48: cmpi.w #$1, $72(a6)
pub fn i_03aa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA4E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03AA4E: bne.w $3aa9a
pub fn i_03aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AA9A; return 10; }
    8
}
/// $03AA52: bclr.b #$2, $8c(a6)
pub fn i_03aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA58;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AA58: jsr $3933a(pc)
pub fn i_03aa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AA5C);
    cpu.pc = 0x3933A;
    16
}
/// $03AA5C: lea.l $3aa6e(pc), a0
pub fn i_03aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA60;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AA6E;
    16
}
/// $03AA60: movea.l (a0, d0.w), a0
pub fn i_03aa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA64;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AA64: jsr $28cd4.l
pub fn i_03aa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AA6A);
    cpu.pc = 0x28CD4;
    16
}
/// $03AA6A: bra.w $3aa96
pub fn i_03aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AA96;
    10
}
/// $03AA96: bra.w $3aade
pub fn i_03aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AA9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AADE;
    10
}
/// $03AA9A: bclr.b #$2, $8c(a6)
pub fn i_03aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAA0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AAA0: jsr $3933a(pc)
pub fn i_03aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AAA4);
    cpu.pc = 0x3933A;
    16
}
/// $03AAA4: lea.l $3aab6(pc), a0
pub fn i_03aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAA8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AAB6;
    16
}
/// $03AAA8: movea.l (a0, d0.w), a0
pub fn i_03aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAAC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AAAC: jsr $28cd4.l
pub fn i_03aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AAB2);
    cpu.pc = 0x28CD4;
    16
}
/// $03AAB2: bra.w $3aade
pub fn i_03aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AADE;
    10
}
/// $03AADE: rts
pub fn i_03aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AAE0;
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
// ===== PlayerArm_JumpShootB_03ad58  ($03AD58, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03AD58: cmpi.w #$1, $72(a6)
pub fn i_03ad58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD5E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03AD5E: bne.w $3adaa
pub fn i_03ad5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3ADAA; return 10; }
    8
}
/// $03AD62: bclr.b #$2, $8c(a6)
pub fn i_03ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD68;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AD68: jsr $3933a(pc)
pub fn i_03ad68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AD6C);
    cpu.pc = 0x3933A;
    16
}
/// $03AD6C: lea.l $3ad7e(pc), a0
pub fn i_03ad6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD70;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AD7E;
    16
}
/// $03AD70: movea.l (a0, d0.w), a0
pub fn i_03ad70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD74;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AD74: jsr $28cd4.l
pub fn i_03ad74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AD7A);
    cpu.pc = 0x28CD4;
    16
}
/// $03AD7A: bra.w $3ada6
pub fn i_03ad7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AD7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3ADA6;
    10
}
/// $03ADA6: bra.w $3ae92
pub fn i_03ada6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AE92;
    10
}
/// $03ADAA: cmpi.w #$2, $72(a6)
pub fn i_03adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADB0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03ADB0: bne.w $3adfc
pub fn i_03adb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3ADFC; return 10; }
    8
}
/// $03ADB4: bclr.b #$2, $8c(a6)
pub fn i_03adb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADBA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03ADBA: jsr $3933a(pc)
pub fn i_03adba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADBE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3ADBE);
    cpu.pc = 0x3933A;
    16
}
/// $03ADBE: lea.l $3add0(pc), a0
pub fn i_03adbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADC2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3ADD0;
    16
}
/// $03ADC2: movea.l (a0, d0.w), a0
pub fn i_03adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADC6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03ADC6: jsr $28cd4.l
pub fn i_03adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3ADCC);
    cpu.pc = 0x28CD4;
    16
}
/// $03ADCC: bra.w $3adf8
pub fn i_03adcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3ADF8;
    10
}
/// $03ADF8: bra.w $3ae92
pub fn i_03adf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3ADFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AE92;
    10
}
/// $03ADFC: cmpi.w #$3, $72(a6)
pub fn i_03adfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE02;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03AE02: bne.w $3ae4e
pub fn i_03ae02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AE4E; return 10; }
    8
}
/// $03AE06: bclr.b #$2, $8c(a6)
pub fn i_03ae06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE0C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AE0C: jsr $3933a(pc)
pub fn i_03ae0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AE10);
    cpu.pc = 0x3933A;
    16
}
/// $03AE10: lea.l $3ae22(pc), a0
pub fn i_03ae10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE14;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AE22;
    16
}
/// $03AE14: movea.l (a0, d0.w), a0
pub fn i_03ae14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE18;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AE18: jsr $28cd4.l
pub fn i_03ae18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AE1E);
    cpu.pc = 0x28CD4;
    16
}
/// $03AE1E: bra.w $3ae4a
pub fn i_03ae1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE22;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AE4A;
    10
}
/// $03AE4A: bra.w $3ae92
pub fn i_03ae4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AE92;
    10
}
/// $03AE4E: bclr.b #$2, $8c(a6)
pub fn i_03ae4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE54;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AE54: jsr $3933a(pc)
pub fn i_03ae54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AE58);
    cpu.pc = 0x3933A;
    16
}
/// $03AE58: lea.l $3ae6a(pc), a0
pub fn i_03ae58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE5C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AE6A;
    16
}
/// $03AE5C: movea.l (a0, d0.w), a0
pub fn i_03ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE60;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AE60: jsr $28cd4.l
pub fn i_03ae60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AE66);
    cpu.pc = 0x28CD4;
    16
}
/// $03AE66: bra.w $3ae92
pub fn i_03ae66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AE92;
    10
}
/// $03AE92: rts
pub fn i_03ae92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FallShootB_03ae94  ($03AE94, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03AE94: cmpi.w #$1, $72(a6)
pub fn i_03ae94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE9A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03AE9A: bne.w $3aee6
pub fn i_03ae9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AE9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AEE6; return 10; }
    8
}
/// $03AE9E: bclr.b #$2, $8c(a6)
pub fn i_03ae9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEA4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AEA4: jsr $3933a(pc)
pub fn i_03aea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AEA8);
    cpu.pc = 0x3933A;
    16
}
/// $03AEA8: lea.l $3aeba(pc), a0
pub fn i_03aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEAC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AEBA;
    16
}
/// $03AEAC: movea.l (a0, d0.w), a0
pub fn i_03aeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEB0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AEB0: jsr $28cd4.l
pub fn i_03aeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AEB6);
    cpu.pc = 0x28CD4;
    16
}
/// $03AEB6: bra.w $3aee2
pub fn i_03aeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEBA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AEE2;
    10
}
/// $03AEE2: bra.w $3afce
pub fn i_03aee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AFCE;
    10
}
/// $03AEE6: cmpi.w #$2, $72(a6)
pub fn i_03aee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEEC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03AEEC: bne.w $3af38
pub fn i_03aeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEF0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AF38; return 10; }
    8
}
/// $03AEF0: bclr.b #$2, $8c(a6)
pub fn i_03aef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEF6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AEF6: jsr $3933a(pc)
pub fn i_03aef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AEFA);
    cpu.pc = 0x3933A;
    16
}
/// $03AEFA: lea.l $3af0c(pc), a0
pub fn i_03aefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AEFE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AF0C;
    16
}
/// $03AEFE: movea.l (a0, d0.w), a0
pub fn i_03aefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF02;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AF02: jsr $28cd4.l
pub fn i_03af02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AF08);
    cpu.pc = 0x28CD4;
    16
}
/// $03AF08: bra.w $3af34
pub fn i_03af08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AF34;
    10
}
/// $03AF34: bra.w $3afce
pub fn i_03af34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF38;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AFCE;
    10
}
/// $03AF38: cmpi.w #$3, $72(a6)
pub fn i_03af38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF3E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03AF3E: bne.w $3af8a
pub fn i_03af3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF42;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3AF8A; return 10; }
    8
}
/// $03AF42: bclr.b #$2, $8c(a6)
pub fn i_03af42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF48;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AF48: jsr $3933a(pc)
pub fn i_03af48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AF4C);
    cpu.pc = 0x3933A;
    16
}
/// $03AF4C: lea.l $3af5e(pc), a0
pub fn i_03af4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF50;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AF5E;
    16
}
/// $03AF50: movea.l (a0, d0.w), a0
pub fn i_03af50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF54;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AF54: jsr $28cd4.l
pub fn i_03af54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AF5A);
    cpu.pc = 0x28CD4;
    16
}
/// $03AF5A: bra.w $3af86
pub fn i_03af5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF5E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AF86;
    10
}
/// $03AF86: bra.w $3afce
pub fn i_03af86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF8A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AFCE;
    10
}
/// $03AF8A: bclr.b #$2, $8c(a6)
pub fn i_03af8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF90;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AF90: jsr $3933a(pc)
pub fn i_03af90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AF94);
    cpu.pc = 0x3933A;
    16
}
/// $03AF94: lea.l $3afa6(pc), a0
pub fn i_03af94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF98;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AFA6;
    16
}
/// $03AF98: movea.l (a0, d0.w), a0
pub fn i_03af98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AF9C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AF9C: jsr $28cd4.l
pub fn i_03af9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AFA2);
    cpu.pc = 0x28CD4;
    16
}
/// $03AFA2: bra.w $3afce
pub fn i_03afa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3AFCE;
    10
}
/// $03AFCE: rts
pub fn i_03afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_JumpShootC_03afd0  ($03AFD0, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03AFD0: cmpi.w #$1, $72(a6)
pub fn i_03afd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFD6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03AFD6: bne.w $3b022
pub fn i_03afd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFDA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B022; return 10; }
    8
}
/// $03AFDA: bclr.b #$2, $8c(a6)
pub fn i_03afda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFE0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03AFE0: jsr $3933a(pc)
pub fn i_03afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3AFE4);
    cpu.pc = 0x3933A;
    16
}
/// $03AFE4: lea.l $3aff6(pc), a0
pub fn i_03afe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFE8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3AFF6;
    16
}
/// $03AFE8: movea.l (a0, d0.w), a0
pub fn i_03afe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFEC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03AFEC: jsr $28cd4.l
pub fn i_03afec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3AFF2);
    cpu.pc = 0x28CD4;
    16
}
/// $03AFF2: bra.w $3b01e
pub fn i_03aff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3AFF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B01E;
    10
}
/// $03B01E: bra.w $3b10a
pub fn i_03b01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B022;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B10A;
    10
}
/// $03B022: cmpi.w #$0, $72(a6)
pub fn i_03b022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B028;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B028: bne.w $3b074
pub fn i_03b028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B02C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B074; return 10; }
    8
}
/// $03B02C: bclr.b #$2, $8c(a6)
pub fn i_03b02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B032;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B032: jsr $3933a(pc)
pub fn i_03b032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B036;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B036);
    cpu.pc = 0x3933A;
    16
}
/// $03B036: lea.l $3b048(pc), a0
pub fn i_03b036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B03A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B048;
    16
}
/// $03B03A: movea.l (a0, d0.w), a0
pub fn i_03b03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B03E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B03E: jsr $28cd4.l
pub fn i_03b03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B044;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B044);
    cpu.pc = 0x28CD4;
    16
}
/// $03B044: bra.w $3b070
pub fn i_03b044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B048;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B070;
    10
}
/// $03B070: bra.w $3b10a
pub fn i_03b070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B074;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B10A;
    10
}
/// $03B074: cmpi.w #$3, $72(a6)
pub fn i_03b074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B07A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03B07A: bne.w $3b0c6
pub fn i_03b07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B07E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B0C6; return 10; }
    8
}
/// $03B07E: bclr.b #$2, $8c(a6)
pub fn i_03b07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B084;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B084: jsr $3933a(pc)
pub fn i_03b084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B088;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B088);
    cpu.pc = 0x3933A;
    16
}
/// $03B088: lea.l $3b09a(pc), a0
pub fn i_03b088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B08C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B09A;
    16
}
/// $03B08C: movea.l (a0, d0.w), a0
pub fn i_03b08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B090;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B090: jsr $28cd4.l
pub fn i_03b090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B096);
    cpu.pc = 0x28CD4;
    16
}
/// $03B096: bra.w $3b0c2
pub fn i_03b096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B09A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B0C2;
    10
}
/// $03B0C2: bra.w $3b10a
pub fn i_03b0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B0C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B10A;
    10
}
/// $03B0C6: bclr.b #$2, $8c(a6)
pub fn i_03b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B0CC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B0CC: jsr $3933a(pc)
pub fn i_03b0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B0D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B0D0);
    cpu.pc = 0x3933A;
    16
}
/// $03B0D0: lea.l $3b0e2(pc), a0
pub fn i_03b0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B0D4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B0E2;
    16
}
/// $03B0D4: movea.l (a0, d0.w), a0
pub fn i_03b0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B0D8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B0D8: jsr $28cd4.l
pub fn i_03b0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B0DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B0DE);
    cpu.pc = 0x28CD4;
    16
}
/// $03B0DE: bra.w $3b10a
pub fn i_03b0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B0E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B10A;
    10
}
/// $03B10A: rts
pub fn i_03b10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B10C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FallShootC_03b10c  ($03B10C, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B10C: cmpi.w #$1, $72(a6)
pub fn i_03b10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B112;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03B112: bne.w $3b15e
pub fn i_03b112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B116;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B15E; return 10; }
    8
}
/// $03B116: bclr.b #$2, $8c(a6)
pub fn i_03b116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B11C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B11C: jsr $3933a(pc)
pub fn i_03b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B120;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B120);
    cpu.pc = 0x3933A;
    16
}
/// $03B120: lea.l $3b132(pc), a0
pub fn i_03b120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B124;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B132;
    16
}
/// $03B124: movea.l (a0, d0.w), a0
pub fn i_03b124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B128;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B128: jsr $28cd4.l
pub fn i_03b128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B12E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B12E);
    cpu.pc = 0x28CD4;
    16
}
/// $03B12E: bra.w $3b15a
pub fn i_03b12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B132;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B15A;
    10
}
/// $03B15A: bra.w $3b246
pub fn i_03b15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B15E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B246;
    10
}
/// $03B15E: cmpi.w #$0, $72(a6)
pub fn i_03b15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B164;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B164: bne.w $3b1b0
pub fn i_03b164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B168;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B1B0; return 10; }
    8
}
/// $03B168: bclr.b #$2, $8c(a6)
pub fn i_03b168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B16E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B16E: jsr $3933a(pc)
pub fn i_03b16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B172;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B172);
    cpu.pc = 0x3933A;
    16
}
/// $03B172: lea.l $3b184(pc), a0
pub fn i_03b172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B176;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B184;
    16
}
/// $03B176: movea.l (a0, d0.w), a0
pub fn i_03b176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B17A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B17A: jsr $28cd4.l
pub fn i_03b17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B180;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B180);
    cpu.pc = 0x28CD4;
    16
}
/// $03B180: bra.w $3b1ac
pub fn i_03b180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B184;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B1AC;
    10
}
/// $03B1AC: bra.w $3b246
pub fn i_03b1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B246;
    10
}
/// $03B1B0: cmpi.w #$3, $72(a6)
pub fn i_03b1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1B6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03B1B6: bne.w $3b202
pub fn i_03b1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B202; return 10; }
    8
}
/// $03B1BA: bclr.b #$2, $8c(a6)
pub fn i_03b1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1C0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B1C0: jsr $3933a(pc)
pub fn i_03b1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B1C4);
    cpu.pc = 0x3933A;
    16
}
/// $03B1C4: lea.l $3b1d6(pc), a0
pub fn i_03b1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1C8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B1D6;
    16
}
/// $03B1C8: movea.l (a0, d0.w), a0
pub fn i_03b1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1CC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B1CC: jsr $28cd4.l
pub fn i_03b1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B1D2);
    cpu.pc = 0x28CD4;
    16
}
/// $03B1D2: bra.w $3b1fe
pub fn i_03b1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B1D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B1FE;
    10
}
/// $03B1FE: bra.w $3b246
pub fn i_03b1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B202;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B246;
    10
}
/// $03B202: bclr.b #$2, $8c(a6)
pub fn i_03b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B208;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B208: jsr $3933a(pc)
pub fn i_03b208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B20C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B20C);
    cpu.pc = 0x3933A;
    16
}
/// $03B20C: lea.l $3b21e(pc), a0
pub fn i_03b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B210;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B21E;
    16
}
/// $03B210: movea.l (a0, d0.w), a0
pub fn i_03b210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B214;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B214: jsr $28cd4.l
pub fn i_03b214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B21A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B21A);
    cpu.pc = 0x28CD4;
    16
}
/// $03B21A: bra.w $3b246
pub fn i_03b21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B21E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B246;
    10
}
/// $03B246: rts
pub fn i_03b246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B248;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_JumpShootD_03b248  ($03B248, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B248: cmpi.w #$1, $72(a6)
pub fn i_03b248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B24E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03B24E: bne.w $3b29a
pub fn i_03b24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B252;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B29A; return 10; }
    8
}
/// $03B252: bclr.b #$2, $8c(a6)
pub fn i_03b252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B258;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B258: jsr $3933a(pc)
pub fn i_03b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B25C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B25C);
    cpu.pc = 0x3933A;
    16
}
/// $03B25C: lea.l $3b26e(pc), a0
pub fn i_03b25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B260;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B26E;
    16
}
/// $03B260: movea.l (a0, d0.w), a0
pub fn i_03b260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B264;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B264: jsr $28cd4.l
pub fn i_03b264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B26A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B26A);
    cpu.pc = 0x28CD4;
    16
}
/// $03B26A: bra.w $3b296
pub fn i_03b26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B26E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B296;
    10
}
/// $03B296: bra.w $3b382
pub fn i_03b296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B29A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B382;
    10
}
/// $03B29A: cmpi.w #$0, $72(a6)
pub fn i_03b29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2A0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B2A0: bne.w $3b2ec
pub fn i_03b2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B2EC; return 10; }
    8
}
/// $03B2A4: bclr.b #$2, $8c(a6)
pub fn i_03b2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2AA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B2AA: jsr $3933a(pc)
pub fn i_03b2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B2AE);
    cpu.pc = 0x3933A;
    16
}
/// $03B2AE: lea.l $3b2c0(pc), a0
pub fn i_03b2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2B2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B2C0;
    16
}
/// $03B2B2: movea.l (a0, d0.w), a0
pub fn i_03b2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2B6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B2B6: jsr $28cd4.l
pub fn i_03b2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B2BC);
    cpu.pc = 0x28CD4;
    16
}
/// $03B2BC: bra.w $3b2e8
pub fn i_03b2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B2E8;
    10
}
/// $03B2E8: bra.w $3b382
pub fn i_03b2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B382;
    10
}
/// $03B2EC: cmpi.w #$3, $72(a6)
pub fn i_03b2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2F2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03B2F2: bne.w $3b33e
pub fn i_03b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B33E; return 10; }
    8
}
/// $03B2F6: bclr.b #$2, $8c(a6)
pub fn i_03b2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B2FC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B2FC: jsr $3933a(pc)
pub fn i_03b2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B300;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B300);
    cpu.pc = 0x3933A;
    16
}
/// $03B300: lea.l $3b312(pc), a0
pub fn i_03b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B304;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B312;
    16
}
/// $03B304: movea.l (a0, d0.w), a0
pub fn i_03b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B308;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B308: jsr $28cd4.l
pub fn i_03b308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B30E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B30E);
    cpu.pc = 0x28CD4;
    16
}
/// $03B30E: bra.w $3b33a
pub fn i_03b30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B312;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B33A;
    10
}
/// $03B33A: bra.w $3b382
pub fn i_03b33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B33E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B382;
    10
}
/// $03B33E: bclr.b #$2, $8c(a6)
pub fn i_03b33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B344;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B344: jsr $3933a(pc)
pub fn i_03b344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B348;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B348);
    cpu.pc = 0x3933A;
    16
}
/// $03B348: lea.l $3b35a(pc), a0
pub fn i_03b348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B34C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B35A;
    16
}
/// $03B34C: movea.l (a0, d0.w), a0
pub fn i_03b34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B350;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B350: jsr $28cd4.l
pub fn i_03b350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B356;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B356);
    cpu.pc = 0x28CD4;
    16
}
/// $03B356: bra.w $3b382
pub fn i_03b356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B35A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B382;
    10
}
/// $03B382: rts
pub fn i_03b382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B384;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FallShootD_03b384  ($03B384, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B384: cmpi.w #$1, $72(a6)
pub fn i_03b384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B38A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03B38A: bne.w $3b3d6
pub fn i_03b38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B38E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B3D6; return 10; }
    8
}
/// $03B38E: bclr.b #$2, $8c(a6)
pub fn i_03b38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B394;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B394: jsr $3933a(pc)
pub fn i_03b394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B398;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B398);
    cpu.pc = 0x3933A;
    16
}
/// $03B398: lea.l $3b3aa(pc), a0
pub fn i_03b398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B39C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B3AA;
    16
}
/// $03B39C: movea.l (a0, d0.w), a0
pub fn i_03b39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3A0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B3A0: jsr $28cd4.l
pub fn i_03b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B3A6);
    cpu.pc = 0x28CD4;
    16
}
/// $03B3A6: bra.w $3b3d2
pub fn i_03b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B3D2;
    10
}
/// $03B3D2: bra.w $3b4be
pub fn i_03b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B4BE;
    10
}
/// $03B3D6: cmpi.w #$0, $72(a6)
pub fn i_03b3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3DC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B3DC: bne.w $3b428
pub fn i_03b3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B428; return 10; }
    8
}
/// $03B3E0: bclr.b #$2, $8c(a6)
pub fn i_03b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3E6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B3E6: jsr $3933a(pc)
pub fn i_03b3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B3EA);
    cpu.pc = 0x3933A;
    16
}
/// $03B3EA: lea.l $3b3fc(pc), a0
pub fn i_03b3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3EE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B3FC;
    16
}
/// $03B3EE: movea.l (a0, d0.w), a0
pub fn i_03b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3F2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B3F2: jsr $28cd4.l
pub fn i_03b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B3F8);
    cpu.pc = 0x28CD4;
    16
}
/// $03B3F8: bra.w $3b424
pub fn i_03b3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B3FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B424;
    10
}
/// $03B424: bra.w $3b4be
pub fn i_03b424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B428;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B4BE;
    10
}
/// $03B428: cmpi.w #$3, $72(a6)
pub fn i_03b428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B42E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $03B42E: bne.w $3b47a
pub fn i_03b42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B432;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B47A; return 10; }
    8
}
/// $03B432: bclr.b #$2, $8c(a6)
pub fn i_03b432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B438;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B438: jsr $3933a(pc)
pub fn i_03b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B43C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B43C);
    cpu.pc = 0x3933A;
    16
}
/// $03B43C: lea.l $3b44e(pc), a0
pub fn i_03b43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B440;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B44E;
    16
}
/// $03B440: movea.l (a0, d0.w), a0
pub fn i_03b440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B444;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B444: jsr $28cd4.l
pub fn i_03b444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B44A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B44A);
    cpu.pc = 0x28CD4;
    16
}
/// $03B44A: bra.w $3b476
pub fn i_03b44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B44E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B476;
    10
}
/// $03B476: bra.w $3b4be
pub fn i_03b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B47A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B4BE;
    10
}
/// $03B47A: bclr.b #$2, $8c(a6)
pub fn i_03b47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B480;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B480: jsr $3933a(pc)
pub fn i_03b480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B484;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B484);
    cpu.pc = 0x3933A;
    16
}
/// $03B484: lea.l $3b496(pc), a0
pub fn i_03b484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B488;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B496;
    16
}
/// $03B488: movea.l (a0, d0.w), a0
pub fn i_03b488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B48C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B48C: jsr $28cd4.l
pub fn i_03b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B492;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B492);
    cpu.pc = 0x28CD4;
    16
}
/// $03B492: bra.w $3b4be
pub fn i_03b492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B496;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B4BE;
    10
}
/// $03B4BE: rts
pub fn i_03b4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_JumpShootDownA_03b4c0  ($03B4C0, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B4C0: cmpi.w #$0, $72(a6)
pub fn i_03b4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4C6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B4C6: bne.w $3b512
pub fn i_03b4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B512; return 10; }
    8
}
/// $03B4CA: bclr.b #$2, $8c(a6)
pub fn i_03b4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4D0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B4D0: jsr $3933a(pc)
pub fn i_03b4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B4D4);
    cpu.pc = 0x3933A;
    16
}
/// $03B4D4: lea.l $3b4e6(pc), a0
pub fn i_03b4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4D8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B4E6;
    16
}
/// $03B4D8: movea.l (a0, d0.w), a0
pub fn i_03b4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4DC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B4DC: jsr $28cd4.l
pub fn i_03b4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B4E2);
    cpu.pc = 0x28CD4;
    16
}
/// $03B4E2: bra.w $3b50e
pub fn i_03b4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B4E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B50E;
    10
}
/// $03B50E: bra.w $3b5a8
pub fn i_03b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B512;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B5A8;
    10
}
/// $03B512: cmpi.w #$2, $72(a6)
pub fn i_03b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B518;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03B518: bne.w $3b564
pub fn i_03b518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B51C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B564; return 10; }
    8
}
/// $03B51C: bclr.b #$2, $8c(a6)
pub fn i_03b51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B522;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B522: jsr $3933a(pc)
pub fn i_03b522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B526;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B526);
    cpu.pc = 0x3933A;
    16
}
/// $03B526: lea.l $3b538(pc), a0
pub fn i_03b526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B52A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B538;
    16
}
/// $03B52A: movea.l (a0, d0.w), a0
pub fn i_03b52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B52E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B52E: jsr $28cd4.l
pub fn i_03b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B534;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B534);
    cpu.pc = 0x28CD4;
    16
}
/// $03B534: bra.w $3b560
pub fn i_03b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B538;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B560;
    10
}
/// $03B560: bra.w $3b5a8
pub fn i_03b560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B564;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B5A8;
    10
}
/// $03B564: bclr.b #$2, $8c(a6)
pub fn i_03b564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B56A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B56A: jsr $3933a(pc)
pub fn i_03b56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B56E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B56E);
    cpu.pc = 0x3933A;
    16
}
/// $03B56E: lea.l $3b580(pc), a0
pub fn i_03b56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B572;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B580;
    16
}
/// $03B572: movea.l (a0, d0.w), a0
pub fn i_03b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B576;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B576: jsr $28cd4.l
pub fn i_03b576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B57C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B57C);
    cpu.pc = 0x28CD4;
    16
}
/// $03B57C: bra.w $3b5a8
pub fn i_03b57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B580;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B5A8;
    10
}
/// $03B5A8: rts
pub fn i_03b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FallShootDownA_03b5aa  ($03B5AA, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B5AA: cmpi.w #$0, $72(a6)
pub fn i_03b5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B5B0: bne.w $3b5fc
pub fn i_03b5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B5FC; return 10; }
    8
}
/// $03B5B4: bclr.b #$2, $8c(a6)
pub fn i_03b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B5BA: jsr $3933a(pc)
pub fn i_03b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B5BE);
    cpu.pc = 0x3933A;
    16
}
/// $03B5BE: lea.l $3b5d0(pc), a0
pub fn i_03b5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B5D0;
    16
}
/// $03B5C2: movea.l (a0, d0.w), a0
pub fn i_03b5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5C6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B5C6: jsr $28cd4.l
pub fn i_03b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B5CC);
    cpu.pc = 0x28CD4;
    16
}
/// $03B5CC: bra.w $3b5f8
pub fn i_03b5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B5F8;
    10
}
/// $03B5F8: bra.w $3b692
pub fn i_03b5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B5FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B692;
    10
}
/// $03B5FC: cmpi.w #$2, $72(a6)
pub fn i_03b5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B602;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03B602: bne.w $3b64e
pub fn i_03b602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B606;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B64E; return 10; }
    8
}
/// $03B606: bclr.b #$2, $8c(a6)
pub fn i_03b606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B60C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B60C: jsr $3933a(pc)
pub fn i_03b60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B610;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B610);
    cpu.pc = 0x3933A;
    16
}
/// $03B610: lea.l $3b622(pc), a0
pub fn i_03b610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B614;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B622;
    16
}
/// $03B614: movea.l (a0, d0.w), a0
pub fn i_03b614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B618;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B618: jsr $28cd4.l
pub fn i_03b618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B61E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B61E);
    cpu.pc = 0x28CD4;
    16
}
/// $03B61E: bra.w $3b64a
pub fn i_03b61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B622;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B64A;
    10
}
/// $03B64A: bra.w $3b692
pub fn i_03b64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B64E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B692;
    10
}
/// $03B64E: bclr.b #$2, $8c(a6)
pub fn i_03b64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B654;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B654: jsr $3933a(pc)
pub fn i_03b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B658;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B658);
    cpu.pc = 0x3933A;
    16
}
/// $03B658: lea.l $3b66a(pc), a0
pub fn i_03b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B65C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B66A;
    16
}
/// $03B65C: movea.l (a0, d0.w), a0
pub fn i_03b65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B660;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B660: jsr $28cd4.l
pub fn i_03b660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B666;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B666);
    cpu.pc = 0x28CD4;
    16
}
/// $03B666: bra.w $3b692
pub fn i_03b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B66A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B692;
    10
}
/// $03B692: rts
pub fn i_03b692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B694;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_JumpShootDownB_03b694  ($03B694, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B694: cmpi.w #$0, $72(a6)
pub fn i_03b694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B69A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B69A: bne.w $3b6e6
pub fn i_03b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B69E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B6E6; return 10; }
    8
}
/// $03B69E: bclr.b #$2, $8c(a6)
pub fn i_03b69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6A4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B6A4: jsr $3933a(pc)
pub fn i_03b6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B6A8);
    cpu.pc = 0x3933A;
    16
}
/// $03B6A8: lea.l $3b6ba(pc), a0
pub fn i_03b6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6AC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B6BA;
    16
}
/// $03B6AC: movea.l (a0, d0.w), a0
pub fn i_03b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6B0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B6B0: jsr $28cd4.l
pub fn i_03b6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B6B6);
    cpu.pc = 0x28CD4;
    16
}
/// $03B6B6: bra.w $3b6e2
pub fn i_03b6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B6E2;
    10
}
/// $03B6E2: bra.w $3b77c
pub fn i_03b6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B77C;
    10
}
/// $03B6E6: cmpi.w #$2, $72(a6)
pub fn i_03b6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03B6EC: bne.w $3b738
pub fn i_03b6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B738; return 10; }
    8
}
/// $03B6F0: bclr.b #$2, $8c(a6)
pub fn i_03b6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6F6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B6F6: jsr $3933a(pc)
pub fn i_03b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B6FA);
    cpu.pc = 0x3933A;
    16
}
/// $03B6FA: lea.l $3b70c(pc), a0
pub fn i_03b6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B6FE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B70C;
    16
}
/// $03B6FE: movea.l (a0, d0.w), a0
pub fn i_03b6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B702;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B702: jsr $28cd4.l
pub fn i_03b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B708);
    cpu.pc = 0x28CD4;
    16
}
/// $03B708: bra.w $3b734
pub fn i_03b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B70C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B734;
    10
}
/// $03B734: bra.w $3b77c
pub fn i_03b734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B738;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B77C;
    10
}
/// $03B738: bclr.b #$2, $8c(a6)
pub fn i_03b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B73E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B73E: jsr $3933a(pc)
pub fn i_03b73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B742);
    cpu.pc = 0x3933A;
    16
}
/// $03B742: lea.l $3b754(pc), a0
pub fn i_03b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B746;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B754;
    16
}
/// $03B746: movea.l (a0, d0.w), a0
pub fn i_03b746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B74A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B74A: jsr $28cd4.l
pub fn i_03b74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B750);
    cpu.pc = 0x28CD4;
    16
}
/// $03B750: bra.w $3b77c
pub fn i_03b750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B754;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B77C;
    10
}
/// $03B77C: rts
pub fn i_03b77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B77E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FallShootDownB_03b77e  ($03B77E, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B77E: cmpi.w #$0, $72(a6)
pub fn i_03b77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B784;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B784: bne.w $3b7d0
pub fn i_03b784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B788;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B7D0; return 10; }
    8
}
/// $03B788: bclr.b #$2, $8c(a6)
pub fn i_03b788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B78E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B78E: jsr $3933a(pc)
pub fn i_03b78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B792;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B792);
    cpu.pc = 0x3933A;
    16
}
/// $03B792: lea.l $3b7a4(pc), a0
pub fn i_03b792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B796;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B7A4;
    16
}
/// $03B796: movea.l (a0, d0.w), a0
pub fn i_03b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B79A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B79A: jsr $28cd4.l
pub fn i_03b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B7A0);
    cpu.pc = 0x28CD4;
    16
}
/// $03B7A0: bra.w $3b7cc
pub fn i_03b7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B7CC;
    10
}
/// $03B7CC: bra.w $3b866
pub fn i_03b7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B866;
    10
}
/// $03B7D0: cmpi.w #$2, $72(a6)
pub fn i_03b7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7D6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03B7D6: bne.w $3b822
pub fn i_03b7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B822; return 10; }
    8
}
/// $03B7DA: bclr.b #$2, $8c(a6)
pub fn i_03b7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7E0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B7E0: jsr $3933a(pc)
pub fn i_03b7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B7E4);
    cpu.pc = 0x3933A;
    16
}
/// $03B7E4: lea.l $3b7f6(pc), a0
pub fn i_03b7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7E8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B7F6;
    16
}
/// $03B7E8: movea.l (a0, d0.w), a0
pub fn i_03b7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7EC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B7EC: jsr $28cd4.l
pub fn i_03b7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B7F2);
    cpu.pc = 0x28CD4;
    16
}
/// $03B7F2: bra.w $3b81e
pub fn i_03b7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B7F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B81E;
    10
}
/// $03B81E: bra.w $3b866
pub fn i_03b81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B822;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B866;
    10
}
/// $03B822: bclr.b #$2, $8c(a6)
pub fn i_03b822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B828;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B828: jsr $3933a(pc)
pub fn i_03b828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B82C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B82C);
    cpu.pc = 0x3933A;
    16
}
/// $03B82C: lea.l $3b83e(pc), a0
pub fn i_03b82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B830;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B83E;
    16
}
/// $03B830: movea.l (a0, d0.w), a0
pub fn i_03b830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B834;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B834: jsr $28cd4.l
pub fn i_03b834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B83A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B83A);
    cpu.pc = 0x28CD4;
    16
}
/// $03B83A: bra.w $3b866
pub fn i_03b83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B83E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B866;
    10
}
/// $03B866: rts
pub fn i_03b866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B868;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_JumpShootDownC_03b868  ($03B868, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03B868: cmpi.w #$0, $72(a6)
pub fn i_03b868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B86E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03B86E: bne.w $3b8ba
pub fn i_03b86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B872;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B8BA; return 10; }
    8
}
/// $03B872: bclr.b #$2, $8c(a6)
pub fn i_03b872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B878;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B878: jsr $3933a(pc)
pub fn i_03b878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B87C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B87C);
    cpu.pc = 0x3933A;
    16
}
/// $03B87C: lea.l $3b88e(pc), a0
pub fn i_03b87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B880;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B88E;
    16
}
/// $03B880: movea.l (a0, d0.w), a0
pub fn i_03b880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B884;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B884: jsr $28cd4.l
pub fn i_03b884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B88A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B88A);
    cpu.pc = 0x28CD4;
    16
}
/// $03B88A: bra.w $3b8b6
pub fn i_03b88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B88E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B8B6;
    10
}
/// $03B8B6: bra.w $3b950
pub fn i_03b8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B950;
    10
}
/// $03B8BA: cmpi.w #$2, $72(a6)
pub fn i_03b8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8C0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03B8C0: bne.w $3b90c
pub fn i_03b8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3B90C; return 10; }
    8
}
/// $03B8C4: bclr.b #$2, $8c(a6)
pub fn i_03b8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8CA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B8CA: jsr $3933a(pc)
pub fn i_03b8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B8CE);
    cpu.pc = 0x3933A;
    16
}
/// $03B8CE: lea.l $3b8e0(pc), a0
pub fn i_03b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8D2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B8E0;
    16
}
/// $03B8D2: movea.l (a0, d0.w), a0
pub fn i_03b8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8D6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B8D6: jsr $28cd4.l
pub fn i_03b8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B8DC);
    cpu.pc = 0x28CD4;
    16
}
/// $03B8DC: bra.w $3b908
pub fn i_03b8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B8E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B908;
    10
}
/// $03B908: bra.w $3b950
pub fn i_03b908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B90C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B950;
    10
}
/// $03B90C: bclr.b #$2, $8c(a6)
pub fn i_03b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B912;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03B912: jsr $3933a(pc)
pub fn i_03b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B916;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3B916);
    cpu.pc = 0x3933A;
    16
}
/// $03B916: lea.l $3b928(pc), a0
pub fn i_03b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B91A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3B928;
    16
}
/// $03B91A: movea.l (a0, d0.w), a0
pub fn i_03b91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B91E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03B91E: jsr $28cd4.l
pub fn i_03b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3B924);
    cpu.pc = 0x28CD4;
    16
}
/// $03B924: bra.w $3b950
pub fn i_03b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B928;
    cpu.ir = 0x6000;
    cpu.pc = 0x3B950;
    10
}
/// $03B950: rts
pub fn i_03b950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3B952;
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
// ===== PlayerArm_ShootDownUnrefA_03ba3c  ($03BA3C, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03BA3C: cmpi.w #$0, $72(a6)
pub fn i_03ba3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA42;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03BA42: bne.w $3ba8e
pub fn i_03ba42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3BA8E; return 10; }
    8
}
/// $03BA46: bclr.b #$2, $8c(a6)
pub fn i_03ba46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA4C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BA4C: jsr $3933a(pc)
pub fn i_03ba4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BA50);
    cpu.pc = 0x3933A;
    16
}
/// $03BA50: lea.l $3ba62(pc), a0
pub fn i_03ba50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA54;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BA62;
    16
}
/// $03BA54: movea.l (a0, d0.w), a0
pub fn i_03ba54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA58;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BA58: jsr $28cd4.l
pub fn i_03ba58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BA5E);
    cpu.pc = 0x28CD4;
    16
}
/// $03BA5E: bra.w $3ba8a
pub fn i_03ba5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA62;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BA8A;
    10
}
/// $03BA8A: bra.w $3bb24
pub fn i_03ba8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BB24;
    10
}
/// $03BA8E: cmpi.w #$2, $72(a6)
pub fn i_03ba8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA94;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03BA94: bne.w $3bae0
pub fn i_03ba94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3BAE0; return 10; }
    8
}
/// $03BA98: bclr.b #$2, $8c(a6)
pub fn i_03ba98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BA9E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BA9E: jsr $3933a(pc)
pub fn i_03ba9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BAA2);
    cpu.pc = 0x3933A;
    16
}
/// $03BAA2: lea.l $3bab4(pc), a0
pub fn i_03baa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAA6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BAB4;
    16
}
/// $03BAA6: movea.l (a0, d0.w), a0
pub fn i_03baa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAAA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BAAA: jsr $28cd4.l
pub fn i_03baaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BAB0);
    cpu.pc = 0x28CD4;
    16
}
/// $03BAB0: bra.w $3badc
pub fn i_03bab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BADC;
    10
}
/// $03BADC: bra.w $3bb24
pub fn i_03badc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BB24;
    10
}
/// $03BAE0: bclr.b #$2, $8c(a6)
pub fn i_03bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAE6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BAE6: jsr $3933a(pc)
pub fn i_03bae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BAEA);
    cpu.pc = 0x3933A;
    16
}
/// $03BAEA: lea.l $3bafc(pc), a0
pub fn i_03baea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAEE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BAFC;
    16
}
/// $03BAEE: movea.l (a0, d0.w), a0
pub fn i_03baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAF2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BAF2: jsr $28cd4.l
pub fn i_03baf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BAF8);
    cpu.pc = 0x28CD4;
    16
}
/// $03BAF8: bra.w $3bb24
pub fn i_03baf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BAFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BB24;
    10
}
/// $03BB24: rts
pub fn i_03bb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_ShootDownUnrefB_03bb26  ($03BB26, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03BB26: cmpi.w #$0, $72(a6)
pub fn i_03bb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB2C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03BB2C: bne.w $3bb78
pub fn i_03bb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3BB78; return 10; }
    8
}
/// $03BB30: bclr.b #$2, $8c(a6)
pub fn i_03bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB36;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BB36: jsr $3933a(pc)
pub fn i_03bb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BB3A);
    cpu.pc = 0x3933A;
    16
}
/// $03BB3A: lea.l $3bb4c(pc), a0
pub fn i_03bb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB3E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BB4C;
    16
}
/// $03BB3E: movea.l (a0, d0.w), a0
pub fn i_03bb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB42;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BB42: jsr $28cd4.l
pub fn i_03bb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BB48);
    cpu.pc = 0x28CD4;
    16
}
/// $03BB48: bra.w $3bb74
pub fn i_03bb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BB74;
    10
}
/// $03BB74: bra.w $3bc0e
pub fn i_03bb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB78;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BC0E;
    10
}
/// $03BB78: cmpi.w #$2, $72(a6)
pub fn i_03bb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB7E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $03BB7E: bne.w $3bbca
pub fn i_03bb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB82;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3BBCA; return 10; }
    8
}
/// $03BB82: bclr.b #$2, $8c(a6)
pub fn i_03bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB88;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BB88: jsr $3933a(pc)
pub fn i_03bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BB8C);
    cpu.pc = 0x3933A;
    16
}
/// $03BB8C: lea.l $3bb9e(pc), a0
pub fn i_03bb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB90;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BB9E;
    16
}
/// $03BB90: movea.l (a0, d0.w), a0
pub fn i_03bb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB94;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BB94: jsr $28cd4.l
pub fn i_03bb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BB9A);
    cpu.pc = 0x28CD4;
    16
}
/// $03BB9A: bra.w $3bbc6
pub fn i_03bb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BB9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BBC6;
    10
}
/// $03BBC6: bra.w $3bc0e
pub fn i_03bbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BBCA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BC0E;
    10
}
/// $03BBCA: bclr.b #$2, $8c(a6)
pub fn i_03bbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BBD0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BBD0: jsr $3933a(pc)
pub fn i_03bbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BBD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BBD4);
    cpu.pc = 0x3933A;
    16
}
/// $03BBD4: lea.l $3bbe6(pc), a0
pub fn i_03bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BBD8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BBE6;
    16
}
/// $03BBD8: movea.l (a0, d0.w), a0
pub fn i_03bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BBDC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BBDC: jsr $28cd4.l
pub fn i_03bbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BBE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BBE2);
    cpu.pc = 0x28CD4;
    16
}
/// $03BBE2: bra.w $3bc0e
pub fn i_03bbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BBE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BC0E;
    10
}
/// $03BC0E: rts
pub fn i_03bc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_DeathA_03bc10  ($03BC10, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03BC10: bclr.b #$2, $8c(a6)
pub fn i_03bc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC16;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BC16: bset.b #$0, $13(a6)
pub fn i_03bc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC1C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BC1C: bclr.b #$2, $8c(a6)
pub fn i_03bc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC22;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BC22: jsr $3933a(pc)
pub fn i_03bc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BC26);
    cpu.pc = 0x3933A;
    16
}
/// $03BC26: lea.l $3bc38(pc), a0
pub fn i_03bc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC2A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BC38;
    16
}
/// $03BC2A: movea.l (a0, d0.w), a0
pub fn i_03bc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC2E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BC2E: jsr $28cd4.l
pub fn i_03bc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BC34);
    cpu.pc = 0x28CD4;
    16
}
/// $03BC34: bra.w $3bc60
pub fn i_03bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC38;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BC60;
    10
}
/// $03BC60: rts
pub fn i_03bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_DeathB_03bc62  ($03BC62, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03BC62: bclr.b #$2, $8c(a6)
pub fn i_03bc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC68;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BC68: bset.b #$0, $13(a6)
pub fn i_03bc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC6E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BC6E: bclr.b #$2, $8c(a6)
pub fn i_03bc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC74;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BC74: jsr $3933a(pc)
pub fn i_03bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BC78);
    cpu.pc = 0x3933A;
    16
}
/// $03BC78: lea.l $3bc8a(pc), a0
pub fn i_03bc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC7C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BC8A;
    16
}
/// $03BC7C: movea.l (a0, d0.w), a0
pub fn i_03bc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC80;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BC80: jsr $28cd4.l
pub fn i_03bc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BC86);
    cpu.pc = 0x28CD4;
    16
}
/// $03BC86: bra.w $3bcb2
pub fn i_03bc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BC8A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BCB2;
    10
}
/// $03BCB2: rts
pub fn i_03bcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_DeathC_03bcb4  ($03BCB4, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03BCB4: bclr.b #$2, $8c(a6)
pub fn i_03bcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCBA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BCBA: bset.b #$0, $13(a6)
pub fn i_03bcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCC0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BCC0: bclr.b #$2, $8c(a6)
pub fn i_03bcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCC6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BCC6: jsr $3933a(pc)
pub fn i_03bcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BCCA);
    cpu.pc = 0x3933A;
    16
}
/// $03BCCA: lea.l $3bcdc(pc), a0
pub fn i_03bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCCE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BCDC;
    16
}
/// $03BCCE: movea.l (a0, d0.w), a0
pub fn i_03bcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCD2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BCD2: jsr $28cd4.l
pub fn i_03bcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BCD8);
    cpu.pc = 0x28CD4;
    16
}
/// $03BCD8: bra.w $3bd04
pub fn i_03bcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BCDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BD04;
    10
}
/// $03BD04: rts
pub fn i_03bd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_DeathD_03bd06  ($03BD06, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03BD06: bclr.b #$2, $8c(a6)
pub fn i_03bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD0C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BD0C: bset.b #$0, $13(a6)
pub fn i_03bd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD12;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BD12: bclr.b #$2, $8c(a6)
pub fn i_03bd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD18;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BD18: jsr $3933a(pc)
pub fn i_03bd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BD1C);
    cpu.pc = 0x3933A;
    16
}
/// $03BD1C: lea.l $3bd2e(pc), a0
pub fn i_03bd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD20;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BD2E;
    16
}
/// $03BD20: movea.l (a0, d0.w), a0
pub fn i_03bd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD24;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BD24: jsr $28cd4.l
pub fn i_03bd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BD2A);
    cpu.pc = 0x28CD4;
    16
}
/// $03BD2A: bra.w $3bd56
pub fn i_03bd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BD56;
    10
}
/// $03BD56: rts
pub fn i_03bd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BD58;
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
// ===== PlayerArm_DeathG_03bdfc  ($03BDFC, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03BDFC: bclr.b #$2, $8c(a6)
pub fn i_03bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE02;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BE02: bset.b #$0, $13(a6)
pub fn i_03be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE08;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BE08: bclr.b #$2, $8c(a6)
pub fn i_03be08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE0E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BE0E: jsr $3933a(pc)
pub fn i_03be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BE12);
    cpu.pc = 0x3933A;
    16
}
/// $03BE12: lea.l $3be24(pc), a0
pub fn i_03be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE16;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BE24;
    16
}
/// $03BE16: movea.l (a0, d0.w), a0
pub fn i_03be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE1A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BE1A: jsr $28cd4.l
pub fn i_03be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BE20);
    cpu.pc = 0x28CD4;
    16
}
/// $03BE20: bra.w $3be4c
pub fn i_03be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE24;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BE4C;
    10
}
/// $03BE4C: rts
pub fn i_03be4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_DeathH_03be4e  ($03BE4E, 40 B, player/arm_air_death_crouch_03a60a.s)
/// $03BE4E: bclr.b #$2, $8c(a6)
pub fn i_03be4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE54;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BE54: bset.b #$0, $13(a6)
pub fn i_03be54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE5A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BE5A: bclr.b #$2, $8c(a6)
pub fn i_03be5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE60;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BE60: jsr $3933a(pc)
pub fn i_03be60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BE64);
    cpu.pc = 0x3933A;
    16
}
/// $03BE64: lea.l $3be76(pc), a0
pub fn i_03be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE68;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BE76;
    16
}
/// $03BE68: movea.l (a0, d0.w), a0
pub fn i_03be68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE6C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BE6C: jsr $28cd4.l
pub fn i_03be6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BE72);
    cpu.pc = 0x28CD4;
    16
}
/// $03BE72: bra.w $3be9e
pub fn i_03be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BE76;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BE9E;
    10
}
/// $03BE9E: rts
pub fn i_03be9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BEA0;
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
// ===== PlayerArm_CrouchUnref_03bf38  ($03BF38, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03BF38: bclr.b #$2, $8c(a6)
pub fn i_03bf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF3E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BF3E: bclr.b #$2, $8c(a6)
pub fn i_03bf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF44;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03BF44: jsr $3933a(pc)
pub fn i_03bf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3BF48);
    cpu.pc = 0x3933A;
    16
}
/// $03BF48: lea.l $3bf5a(pc), a0
pub fn i_03bf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF4C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3BF5A;
    16
}
/// $03BF4C: movea.l (a0, d0.w), a0
pub fn i_03bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF50;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03BF50: jsr $28cd4.l
pub fn i_03bf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3BF56);
    cpu.pc = 0x28CD4;
    16
}
/// $03BF56: bra.w $3bf82
pub fn i_03bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3BF82;
    10
}
/// $03BF82: rts
pub fn i_03bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3BF84;
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
// ===== PlayerArm_CrouchIdleB_03c06e  ($03C06E, 44 B, player/arm_air_death_crouch_03a60a.s)
/// $03C06E: bclr.b #$2, $8c(a6)
pub fn i_03c06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C074;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C074: jsr $2abcc.l
pub fn i_03c074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C07A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C07A);
    cpu.pc = 0x2ABCC;
    16
}
/// $03C07A: bcs.w $3c0c6
pub fn i_03c07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C07E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3C0C6; return 10; }
    8
}
/// $03C07E: bclr.b #$2, $8c(a6)
pub fn i_03c07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C084;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C084: jsr $3933a(pc)
pub fn i_03c084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C088;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C088);
    cpu.pc = 0x3933A;
    16
}
/// $03C088: lea.l $3c09a(pc), a0
pub fn i_03c088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C08C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C09A;
    16
}
/// $03C08C: movea.l (a0, d0.w), a0
pub fn i_03c08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C090;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C090: jsr $28cd4.l
pub fn i_03c090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C096);
    cpu.pc = 0x28CD4;
    16
}
/// $03C096: bra.w $3c0c2
pub fn i_03c096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C09A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C0C2;
    10
}
/// $03C0C2: bra.w $3c10a
pub fn i_03c0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C0C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C10A;
    10
}
/// $03C0C6: bclr.b #$2, $8c(a6)
pub fn i_03c0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C0CC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C0CC: jsr $3933a(pc)
pub fn i_03c0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C0D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C0D0);
    cpu.pc = 0x3933A;
    16
}
/// $03C0D0: lea.l $3c0e2(pc), a0
pub fn i_03c0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C0D4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C0E2;
    16
}
/// $03C0D4: movea.l (a0, d0.w), a0
pub fn i_03c0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C0D8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C0D8: jsr $28cd4.l
pub fn i_03c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C0DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C0DE);
    cpu.pc = 0x28CD4;
    16
}
/// $03C0DE: bra.w $3c10a
pub fn i_03c0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C0E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C10A;
    10
}
/// $03C10A: rts
pub fn i_03c10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C10C;
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
// ===== PlayerArm_CrawlA_03c1aa  ($03C1AA, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03C1AA: bclr.b #$2, $8c(a6)
pub fn i_03c1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1B0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C1B0: bclr.b #$2, $8c(a6)
pub fn i_03c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1B6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C1B6: jsr $3933a(pc)
pub fn i_03c1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C1BA);
    cpu.pc = 0x3933A;
    16
}
/// $03C1BA: lea.l $3c1cc(pc), a0
pub fn i_03c1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1BE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C1CC;
    16
}
/// $03C1BE: movea.l (a0, d0.w), a0
pub fn i_03c1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1C2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C1C2: jsr $28cd4.l
pub fn i_03c1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C1C8);
    cpu.pc = 0x28CD4;
    16
}
/// $03C1C8: bra.w $3c1f4
pub fn i_03c1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C1F4;
    10
}
/// $03C1F4: rts
pub fn i_03c1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrawlUnref_03c1f6  ($03C1F6, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03C1F6: bclr.b #$2, $8c(a6)
pub fn i_03c1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C1FC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C1FC: bclr.b #$2, $8c(a6)
pub fn i_03c1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C202;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C202: jsr $3933a(pc)
pub fn i_03c202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C206;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C206);
    cpu.pc = 0x3933A;
    16
}
/// $03C206: lea.l $3c218(pc), a0
pub fn i_03c206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C20A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C218;
    16
}
/// $03C20A: movea.l (a0, d0.w), a0
pub fn i_03c20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C20E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C20E: jsr $28cd4.l
pub fn i_03c20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C214);
    cpu.pc = 0x28CD4;
    16
}
/// $03C214: bra.w $3c240
pub fn i_03c214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C218;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C240;
    10
}
/// $03C240: rts
pub fn i_03c240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C242;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrawlB_03c242  ($03C242, 34 B, player/arm_air_death_crouch_03a60a.s)
/// $03C242: bclr.b #$2, $8c(a6)
pub fn i_03c242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C248;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C248: bclr.b #$2, $8c(a6)
pub fn i_03c248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C24E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C24E: jsr $3933a(pc)
pub fn i_03c24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C252;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C252);
    cpu.pc = 0x3933A;
    16
}
/// $03C252: lea.l $3c264(pc), a0
pub fn i_03c252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C256;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C264;
    16
}
/// $03C256: movea.l (a0, d0.w), a0
pub fn i_03c256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C25A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C25A: jsr $28cd4.l
pub fn i_03c25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C260;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C260);
    cpu.pc = 0x28CD4;
    16
}
/// $03C260: bra.w $3c28c
pub fn i_03c260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C264;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C28C;
    10
}
/// $03C28C: rts
pub fn i_03c28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C28E;
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
// ===== PlayerArm_AirMeleeB_03c458  ($03C458, 38 B, player/arm_air_death_crouch_03a60a.s)
/// $03C458: cmpi.w #$1, $72(a6)
pub fn i_03c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C45E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03C45E: bne.w $3c4aa
pub fn i_03c45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C462;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C4AA; return 10; }
    8
}
/// $03C462: bclr.b #$2, $8c(a6)
pub fn i_03c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C468;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C468: jsr $3933a(pc)
pub fn i_03c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C46C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C46C);
    cpu.pc = 0x3933A;
    16
}
/// $03C46C: lea.l $3c47e(pc), a0
pub fn i_03c46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C470;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C47E;
    16
}
/// $03C470: movea.l (a0, d0.w), a0
pub fn i_03c470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C474;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C474: jsr $28cd4.l
pub fn i_03c474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C47A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C47A);
    cpu.pc = 0x28CD4;
    16
}
/// $03C47A: bra.w $3c4a6
pub fn i_03c47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C47E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C4A6;
    10
}
/// $03C4A6: bra.w $3c540
pub fn i_03c4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C540;
    10
}
/// $03C4AA: cmpi.w #$0, $72(a6)
pub fn i_03c4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03C4B0: bne.w $3c4fc
pub fn i_03c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C4FC; return 10; }
    8
}
/// $03C4B4: bclr.b #$2, $8c(a6)
pub fn i_03c4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4BA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C4BA: jsr $3933a(pc)
pub fn i_03c4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C4BE);
    cpu.pc = 0x3933A;
    16
}
/// $03C4BE: lea.l $3c4d0(pc), a0
pub fn i_03c4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4C2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C4D0;
    16
}
/// $03C4C2: movea.l (a0, d0.w), a0
pub fn i_03c4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4C6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C4C6: jsr $28cd4.l
pub fn i_03c4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C4CC);
    cpu.pc = 0x28CD4;
    16
}
/// $03C4CC: bra.w $3c4f8
pub fn i_03c4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C4F8;
    10
}
/// $03C4F8: bra.w $3c540
pub fn i_03c4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C4FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C540;
    10
}
/// $03C4FC: bclr.b #$2, $8c(a6)
pub fn i_03c4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C502;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C502: jsr $3933a(pc)
pub fn i_03c502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C506;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C506);
    cpu.pc = 0x3933A;
    16
}
/// $03C506: lea.l $3c518(pc), a0
pub fn i_03c506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C50A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C518;
    16
}
/// $03C50A: movea.l (a0, d0.w), a0
pub fn i_03c50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C50E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C50E: jsr $28cd4.l
pub fn i_03c50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C514);
    cpu.pc = 0x28CD4;
    16
}
/// $03C514: bra.w $3c540
pub fn i_03c514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C518;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C540;
    10
}
/// $03C540: rts
pub fn i_03c540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C542;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrouchMelee_03c542  ($03C542, 28 B, player/arm_air_death_crouch_03a60a.s)
/// $03C542: bclr.b #$2, $8c(a6)
pub fn i_03c542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C548;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C548: jsr $3933a(pc)
pub fn i_03c548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C54C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C54C);
    cpu.pc = 0x3933A;
    16
}
/// $03C54C: lea.l $3c55e(pc), a0
pub fn i_03c54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C550;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C55E;
    16
}
/// $03C550: movea.l (a0, d0.w), a0
pub fn i_03c550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C554;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C554: jsr $28cd4.l
pub fn i_03c554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C55A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C55A);
    cpu.pc = 0x28CD4;
    16
}
/// $03C55A: bra.w $3c586
pub fn i_03c55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C55E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C586;
    10
}
/// $03C586: rts
pub fn i_03c586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C588;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_CrouchReload_03c588  ($03C588, 42 B, player/arm_air_death_crouch_03a60a.s)
/// $03C588: addq.b #$1, $30(a6)
pub fn i_03c588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C58C;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03C58C: btst.b #$0, $30(a6)
pub fn i_03c58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C592;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C592: bne.w $3c5de
pub fn i_03c592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C596;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C5DE; return 10; }
    8
}
/// $03C596: bclr.b #$2, $8c(a6)
pub fn i_03c596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C59C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C59C: jsr $3933a(pc)
pub fn i_03c59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C5A0);
    cpu.pc = 0x3933A;
    16
}
/// $03C5A0: lea.l $3c5b2(pc), a0
pub fn i_03c5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5A4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C5B2;
    16
}
/// $03C5A4: movea.l (a0, d0.w), a0
pub fn i_03c5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5A8;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C5A8: jsr $28cd4.l
pub fn i_03c5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C5AE);
    cpu.pc = 0x28CD4;
    16
}
/// $03C5AE: bra.w $3c5da
pub fn i_03c5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C5DA;
    10
}
/// $03C5DA: bra.w $3c622
pub fn i_03c5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C622;
    10
}
/// $03C5DE: bclr.b #$2, $8c(a6)
pub fn i_03c5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5E4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C5E4: jsr $3933a(pc)
pub fn i_03c5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C5E8);
    cpu.pc = 0x3933A;
    16
}
/// $03C5E8: lea.l $3c5fa(pc), a0
pub fn i_03c5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C5FA;
    16
}
/// $03C5EC: movea.l (a0, d0.w), a0
pub fn i_03c5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5F0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C5F0: jsr $28cd4.l
pub fn i_03c5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C5F6);
    cpu.pc = 0x28CD4;
    16
}
/// $03C5F6: bra.w $3c622
pub fn i_03c5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C5FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C622;
    10
}
/// $03C622: lea.l $392a4(pc), a0
pub fn i_03c622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C626;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x392A4;
    16
}
/// $03C626: move.l a0, $4c(a6)
pub fn i_03c626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C62A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== PlayerArm_JsrAttack_03c62a  ($03C62A, 8 B, player/fire_shells_03c62a.s)
/// $03C62A: jsr $283ca.l
pub fn i_03c62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C630);
    cpu.pc = 0x283CA;
    16
}
/// $03C630: rts
pub fn i_03c630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C632;
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
// ===== PlayerArm_SlugRideB_03c67e  ($03C67E, 28 B, player/fire_shells_03c62a.s)
/// $03C67E: bclr.b #$2, $8c(a6)
pub fn i_03c67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C684;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C684: jsr $3933a(pc)
pub fn i_03c684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C688;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C688);
    cpu.pc = 0x3933A;
    16
}
/// $03C688: lea.l $3c69a(pc), a0
pub fn i_03c688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C68C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C69A;
    16
}
/// $03C68C: movea.l (a0, d0.w), a0
pub fn i_03c68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C690;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C690: jsr $28cd4.l
pub fn i_03c690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C696;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C696);
    cpu.pc = 0x28CD4;
    16
}
/// $03C696: bra.w $3c6c2
pub fn i_03c696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C69A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C6C2;
    10
}
/// $03C6C2: rts
pub fn i_03c6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C6C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Fall2_03c6c4  ($03C6C4, 34 B, player/fire_shells_03c62a.s)
/// $03C6C4: bclr.b #$2, $8c(a6)
pub fn i_03c6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C6CA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C6CA: bclr.b #$2, $8c(a6)
pub fn i_03c6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C6D0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C6D0: jsr $3933a(pc)
pub fn i_03c6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C6D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C6D4);
    cpu.pc = 0x3933A;
    16
}
/// $03C6D4: lea.l $3c6e6(pc), a0
pub fn i_03c6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C6D8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3C6E6;
    16
}
/// $03C6D8: movea.l (a0, d0.w), a0
pub fn i_03c6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C6DC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $03C6DC: jsr $28cd4.l
pub fn i_03c6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C6E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C6E2);
    cpu.pc = 0x28CD4;
    16
}
/// $03C6E2: bra.w $3c70e
pub fn i_03c6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C6E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C70E;
    10
}
/// $03C70E: rts
pub fn i_03c70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C710;
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
// ===== PlayerFire_Pistol_Back_03c7c8  ($03C7C8, 52 B, player/fire_shells_03c62a.s)
/// $03C7C8: jsr $3c710(pc)
pub fn i_03c7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C7CC);
    cpu.pc = 0x3C710;
    16
}
/// $03C7CC: btst.b #$0, $3a(a6)
pub fn i_03c7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7D2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C7D2: beq.w $3c7e2
pub fn i_03c7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3C7E2; return 10; }
    8
}
/// $03C7D6: move.b #$10, d0
pub fn i_03c7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7DA;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C7DA: move.b #$0, d1
pub fn i_03c7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7DE;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C7DE: bra.w $3c7ea
pub fn i_03c7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C7EA;
    10
}
/// $03C7E2: move.b #$10, d0
pub fn i_03c7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7E6;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C7E6: move.b #$2, d1
pub fn i_03c7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7EA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C7EA: lsl.b #$3, d0
pub fn i_03c7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7EC;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03C7EC: move.b d0, $98(a0)
pub fn i_03c7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7F0;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C7F0: move.b $7a(a6), $9a(a0)
pub fn i_03c7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7F6;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03C7F6: clr.b $9b(a0)
pub fn i_03c7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7FA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03C7FA: rts
pub fn i_03c7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C7FC;
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
// ===== PlayerFire_Pistol_DownB_03c854  ($03C854, 30 B, player/fire_shells_03c62a.s)
/// $03C854: jsr $3c710(pc)
pub fn i_03c854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C858;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C858);
    cpu.pc = 0x3C710;
    16
}
/// $03C858: move.b #$18, d0
pub fn i_03c858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C85C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C85C: move.b #$0, d1
pub fn i_03c85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C860;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C860: lsl.b #$3, d0
pub fn i_03c860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C862;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03C862: move.b d0, $98(a0)
pub fn i_03c862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C866;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C866: move.b $7a(a6), $9a(a0)
pub fn i_03c866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C86C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03C86C: clr.b $9b(a0)
pub fn i_03c86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C870;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03C870: rts
pub fn i_03c870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C872;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_HMG_Spawn_03c872  ($03C872, 108 B, player/fire_shells_03c62a.s)
/// $03C872: cmpa.l #$100440, a6
pub fn i_03c872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C878;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03C878: beq.b $3c884
pub fn i_03c878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C87A;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3C884; return 10; }
    8
}
/// $03C87A: cmpa.l #$1004e0, a6
pub fn i_03c87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C880;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03C880: bne.w $3c88a
pub fn i_03c880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C884;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C88A; return 10; }
    8
}
/// $03C884: movea.l a6, a2
pub fn i_03c884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C886;
    cpu.ir = 0x244E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $03C886: bra.w $3c88e
pub fn i_03c886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C88A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C88E;
    10
}
/// $03C88A: movea.l $c(a6), a2
pub fn i_03c88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C88E;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $03C88E: cmpi.w #$0, $82(a2)
pub fn i_03c88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C894;
    cpu.ir = 0xC6A;
    let ad2 = cpu.a[2].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03C894: beq.w $3c8d8
pub fn i_03c894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C898;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3C8D8; return 10; }
    8
}
/// $03C898: subi.w #$1, $82(a2)
pub fn i_03c898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C89E;
    cpu.ir = 0x46A;
    let ad1 = cpu.a[2].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03C89E: cmpi.b #$0, $85(a2)
pub fn i_03c89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8A4;
    cpu.ir = 0xC2A;
    let ad2 = cpu.a[2].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03C8A4: beq.w $3c8ae
pub fn i_03c8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3C8AE; return 10; }
    8
}
/// $03C8A8: subi.b #$1, $85(a2)
pub fn i_03c8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8AE;
    cpu.ir = 0x42A;
    let ad1 = cpu.a[2].wrapping_add(0x85);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $03C8AE: move.w #$10f6, d0
pub fn i_03c8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03C8B2: jsr $2352.l
pub fn i_03c8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C8B8);
    cpu.pc = 0x2352;
    16
}
/// $03C8B8: lea.l $9c4d4.l, a1
pub fn i_03c8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x9C4D4;
    20
}
/// $03C8BE: jsr $5eaa4.l
pub fn i_03c8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C8C4);
    cpu.pc = 0x5EAA4;
    16
}
/// $03C8C4: move.w $22(a2), $22(a0)
pub fn i_03c8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8CA;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03C8CA: move.w $24(a2), $24(a0)
pub fn i_03c8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8D0;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03C8D0: jsr $517fe.l
pub fn i_03c8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C8D6);
    cpu.pc = 0x517FE;
    16
}
/// $03C8D6: rts
pub fn i_03c8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03C8D8: movem.l (a7)+, a0
pub fn i_03c8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8DC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $03C8DC: rts
pub fn i_03c8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_HMG_Fwd_03c8de  ($03C8DE, 38 B, player/fire_shells_03c62a.s)
/// $03C8DE: jsr $3c872(pc)
pub fn i_03c8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C8E2);
    cpu.pc = 0x3C872;
    16
}
/// $03C8E2: btst.b #$0, $3a(a6)
pub fn i_03c8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8E8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C8E8: bne.w $3c8f4
pub fn i_03c8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C8F4; return 10; }
    8
}
/// $03C8EC: move.b #$0, d0
pub fn i_03c8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8F0;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C8F0: bra.w $3c8f8
pub fn i_03c8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C8F8;
    10
}
/// $03C8F4: move.b #$ff, d0
pub fn i_03c8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8F8;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C8F8: clr.b d1
pub fn i_03c8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8FA;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $03C8FA: move.b d1, $98(a0)
pub fn i_03c8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C8FE;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C8FE: move.b d0, $99(a0)
pub fn i_03c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C902;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C902: rts
pub fn i_03c902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C904;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_HMG_Up_03c904  ($03C904, 22 B, player/fire_shells_03c62a.s)
/// $03C904: jsr $3c872(pc)
pub fn i_03c904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C908;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C908);
    cpu.pc = 0x3C872;
    16
}
/// $03C908: move.b #$1, d1
pub fn i_03c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C90C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C90C: move.b #$0, d0
pub fn i_03c90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C910;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C910: move.b d1, $98(a0)
pub fn i_03c910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C914;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C914: move.b d0, $99(a0)
pub fn i_03c914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C918;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C918: rts
pub fn i_03c918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C91A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_HMG_Down_03c91a  ($03C91A, 40 B, player/fire_shells_03c62a.s)
/// $03C91A: jsr $3c872(pc)
pub fn i_03c91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C91E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C91E);
    cpu.pc = 0x3C872;
    16
}
/// $03C91E: btst.b #$0, $3a(a6)
pub fn i_03c91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C924;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C924: bne.w $3c930
pub fn i_03c924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C928;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C930; return 10; }
    8
}
/// $03C928: move.b #$0, d0
pub fn i_03c928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C92C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C92C: bra.w $3c934
pub fn i_03c92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C930;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C934;
    10
}
/// $03C930: move.b #$ff, d0
pub fn i_03c930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C934;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C934: move.b #$2, d1
pub fn i_03c934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C938;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C938: move.b d1, $98(a0)
pub fn i_03c938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C93C;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C93C: move.b d0, $99(a0)
pub fn i_03c93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C940;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C940: rts
pub fn i_03c940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C942;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_HMG_FwdB_03c942  ($03C942, 38 B, player/fire_shells_03c62a.s)
/// $03C942: jsr $3c872(pc)
pub fn i_03c942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C946;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C946);
    cpu.pc = 0x3C872;
    16
}
/// $03C946: btst.b #$0, $3a(a6)
pub fn i_03c946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C94C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C94C: bne.w $3c958
pub fn i_03c94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C950;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C958; return 10; }
    8
}
/// $03C950: move.b #$0, d0
pub fn i_03c950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C954;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C954: bra.w $3c95c
pub fn i_03c954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C958;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C95C;
    10
}
/// $03C958: move.b #$ff, d0
pub fn i_03c958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C95C;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C95C: clr.b d1
pub fn i_03c95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C95E;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $03C95E: move.b d1, $98(a0)
pub fn i_03c95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C962;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C962: move.b d0, $99(a0)
pub fn i_03c962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C966;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C966: rts
pub fn i_03c966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C968;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_HMG_Diag_03c968  ($03C968, 46 B, player/fire_shells_03c62a.s)
/// $03C968: jsr $3c872(pc)
pub fn i_03c968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C96C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C96C);
    cpu.pc = 0x3C872;
    16
}
/// $03C96C: subi.w #$c, $24(a0)
pub fn i_03c96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C972;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03C972: btst.b #$0, $3a(a6)
pub fn i_03c972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C978;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C978: bne.w $3c984
pub fn i_03c978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C97C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C984; return 10; }
    8
}
/// $03C97C: move.b #$0, d0
pub fn i_03c97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C980;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C980: bra.w $3c988
pub fn i_03c980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C984;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C988;
    10
}
/// $03C984: move.b #$ff, d0
pub fn i_03c984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C988;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C988: move.b #$3, d1
pub fn i_03c988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C98C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C98C: move.b d1, $98(a0)
pub fn i_03c98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C990;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C990: move.b d0, $99(a0)
pub fn i_03c990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C994;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C994: rts
pub fn i_03c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C996;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_HMG_DiagB_03c996  ($03C996, 46 B, player/fire_shells_03c62a.s)
/// $03C996: jsr $3c872(pc)
pub fn i_03c996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C99A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3C99A);
    cpu.pc = 0x3C872;
    16
}
/// $03C99A: subi.w #$c, $24(a0)
pub fn i_03c99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9A0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03C9A0: btst.b #$0, $3a(a6)
pub fn i_03c9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9A6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03C9A6: bne.w $3c9b2
pub fn i_03c9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9AA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C9B2; return 10; }
    8
}
/// $03C9AA: move.b #$0, d0
pub fn i_03c9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9AE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C9AE: bra.w $3c9b6
pub fn i_03c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C9B6;
    10
}
/// $03C9B2: move.b #$ff, d0
pub fn i_03c9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9B6;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C9B6: move.b #$3, d1
pub fn i_03c9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9BA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03C9BA: move.b d1, $98(a0)
pub fn i_03c9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9BE;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C9BE: move.b d0, $99(a0)
pub fn i_03c9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9C2;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03C9C2: rts
pub fn i_03c9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Shotgun_Spawn_03c9c4  ($03C9C4, 98 B, player/fire_shells_03c62a.s)
/// $03C9C4: cmpa.l #$100440, a6
pub fn i_03c9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9CA;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03C9CA: beq.b $3c9d6
pub fn i_03c9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9CC;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3C9D6; return 10; }
    8
}
/// $03C9CC: cmpa.l #$1004e0, a6
pub fn i_03c9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9D2;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03C9D2: bne.w $3c9dc
pub fn i_03c9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9D6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3C9DC; return 10; }
    8
}
/// $03C9D6: movea.l a6, a2
pub fn i_03c9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9D8;
    cpu.ir = 0x244E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $03C9D8: bra.w $3c9e0
pub fn i_03c9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3C9E0;
    10
}
/// $03C9DC: movea.l $c(a6), a2
pub fn i_03c9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9E0;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $03C9E0: cmpi.w #$0, $82(a2)
pub fn i_03c9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9E6;
    cpu.ir = 0xC6A;
    let ad2 = cpu.a[2].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03C9E6: beq.w $3ca20
pub fn i_03c9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3CA20; return 10; }
    8
}
/// $03C9EA: subi.w #$1, $82(a2)
pub fn i_03c9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9F0;
    cpu.ir = 0x46A;
    let ad1 = cpu.a[2].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03C9F0: move.w #$10f3, d0
pub fn i_03c9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03C9F4: jsr $2352.l
pub fn i_03c9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3C9FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3C9FA);
    cpu.pc = 0x2352;
    16
}
/// $03C9FA: lea.l $9c25e.l, a1
pub fn i_03c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x9C25E;
    20
}
/// $03CA00: jsr $5eaa4.l
pub fn i_03ca00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CA06);
    cpu.pc = 0x5EAA4;
    16
}
/// $03CA06: move.w $22(a2), $22(a0)
pub fn i_03ca06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA0C;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03CA0C: move.w $24(a2), $24(a0)
pub fn i_03ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA12;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03CA12: jsr $517fe.l
pub fn i_03ca12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CA18);
    cpu.pc = 0x517FE;
    16
}
/// $03CA18: addi.w #$1c, $24(a0)
pub fn i_03ca18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA1E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CA1E: rts
pub fn i_03ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03CA20: movem.l (a7)+, a0
pub fn i_03ca20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA24;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $03CA24: rts
pub fn i_03ca24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Shotgun_Fwd_03ca26  ($03CA26, 40 B, player/fire_shells_03c62a.s)
/// $03CA26: jsr $3c9c4(pc)
pub fn i_03ca26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CA2A);
    cpu.pc = 0x3C9C4;
    16
}
/// $03CA2A: btst.b #$0, $3a(a6)
pub fn i_03ca2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA30;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CA30: bne.w $3ca3c
pub fn i_03ca30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CA3C; return 10; }
    8
}
/// $03CA34: move.b #$0, d0
pub fn i_03ca34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA38;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CA38: bra.w $3ca40
pub fn i_03ca38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CA40;
    10
}
/// $03CA3C: move.b #$ff, d0
pub fn i_03ca3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA40;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CA40: move.b #$0, d1
pub fn i_03ca40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA44;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CA44: move.b d1, $98(a0)
pub fn i_03ca44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA48;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CA48: move.b d0, $99(a0)
pub fn i_03ca48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA4C;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CA4C: rts
pub fn i_03ca4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Shotgun_Up_03ca4e  ($03CA4E, 22 B, player/fire_shells_03c62a.s)
/// $03CA4E: jsr $3c9c4(pc)
pub fn i_03ca4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CA52);
    cpu.pc = 0x3C9C4;
    16
}
/// $03CA52: move.b #$1, d1
pub fn i_03ca52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA56;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CA56: move.b #$0, d0
pub fn i_03ca56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA5A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CA5A: move.b d1, $98(a0)
pub fn i_03ca5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA5E;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CA5E: move.b d0, $99(a0)
pub fn i_03ca5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA62;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CA62: rts
pub fn i_03ca62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Shotgun_Down_03ca64  ($03CA64, 40 B, player/fire_shells_03c62a.s)
/// $03CA64: jsr $3c9c4(pc)
pub fn i_03ca64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CA68);
    cpu.pc = 0x3C9C4;
    16
}
/// $03CA68: btst.b #$0, $3a(a6)
pub fn i_03ca68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA6E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CA6E: bne.w $3ca7a
pub fn i_03ca6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CA7A; return 10; }
    8
}
/// $03CA72: move.b #$0, d0
pub fn i_03ca72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA76;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CA76: bra.w $3ca7e
pub fn i_03ca76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CA7E;
    10
}
/// $03CA7A: move.b #$ff, d0
pub fn i_03ca7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA7E;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CA7E: move.b #$2, d1
pub fn i_03ca7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA82;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CA82: move.b d1, $98(a0)
pub fn i_03ca82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA86;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CA86: move.b d0, $99(a0)
pub fn i_03ca86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA8A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CA8A: rts
pub fn i_03ca8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Shotgun_FwdLow_03ca8c  ($03CA8C, 46 B, player/fire_shells_03c62a.s)
/// $03CA8C: jsr $3c9c4(pc)
pub fn i_03ca8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CA90);
    cpu.pc = 0x3C9C4;
    16
}
/// $03CA90: subi.w #$c, $24(a0)
pub fn i_03ca90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA96;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CA96: btst.b #$0, $3a(a6)
pub fn i_03ca96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CA9C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CA9C: bne.w $3caa8
pub fn i_03ca9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CAA8; return 10; }
    8
}
/// $03CAA0: move.b #$0, d0
pub fn i_03caa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAA4;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CAA4: bra.w $3caac
pub fn i_03caa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CAAC;
    10
}
/// $03CAA8: move.b #$ff, d0
pub fn i_03caa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAAC;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CAAC: move.b #$0, d1
pub fn i_03caac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAB0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CAB0: move.b d1, $98(a0)
pub fn i_03cab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAB4;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CAB4: move.b d0, $99(a0)
pub fn i_03cab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAB8;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CAB8: rts
pub fn i_03cab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CABA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Shotgun_Diag_03caba  ($03CABA, 46 B, player/fire_shells_03c62a.s)
/// $03CABA: jsr $3c9c4(pc)
pub fn i_03caba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CABE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CABE);
    cpu.pc = 0x3C9C4;
    16
}
/// $03CABE: subi.w #$c, $24(a0)
pub fn i_03cabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAC4;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CAC4: btst.b #$0, $3a(a6)
pub fn i_03cac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CACA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CACA: bne.w $3cad6
pub fn i_03caca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CACE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CAD6; return 10; }
    8
}
/// $03CACE: move.b #$0, d0
pub fn i_03cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAD2;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CAD2: bra.w $3cada
pub fn i_03cad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CADA;
    10
}
/// $03CAD6: move.b #$ff, d0
pub fn i_03cad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CADA;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CADA: move.b #$3, d1
pub fn i_03cada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CADE;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CADE: move.b d1, $98(a0)
pub fn i_03cade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAE2;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CAE2: move.b d0, $99(a0)
pub fn i_03cae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAE6;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CAE6: rts
pub fn i_03cae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Shotgun_DiagB_03cae8  ($03CAE8, 46 B, player/fire_shells_03c62a.s)
/// $03CAE8: jsr $3c9c4(pc)
pub fn i_03cae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CAEC);
    cpu.pc = 0x3C9C4;
    16
}
/// $03CAEC: subi.w #$c, $24(a0)
pub fn i_03caec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAF2;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CAF2: btst.b #$0, $3a(a6)
pub fn i_03caf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAF8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CAF8: bne.w $3cb04
pub fn i_03caf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CAFC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CB04; return 10; }
    8
}
/// $03CAFC: move.b #$0, d0
pub fn i_03cafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB00;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CB00: bra.w $3cb08
pub fn i_03cb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB04;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CB08;
    10
}
/// $03CB04: move.b #$ff, d0
pub fn i_03cb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB08;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CB08: move.b #$3, d1
pub fn i_03cb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB0C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CB0C: move.b d1, $98(a0)
pub fn i_03cb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB10;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CB10: move.b d0, $99(a0)
pub fn i_03cb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB14;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CB14: rts
pub fn i_03cb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Rocket_Spawn_03cb16  ($03CB16, 98 B, player/fire_shells_03c62a.s)
/// $03CB16: cmpa.l #$100440, a6
pub fn i_03cb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB1C;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03CB1C: beq.b $3cb28
pub fn i_03cb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB1E;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3CB28; return 10; }
    8
}
/// $03CB1E: cmpa.l #$1004e0, a6
pub fn i_03cb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB24;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03CB24: bne.w $3cb2e
pub fn i_03cb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB28;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CB2E; return 10; }
    8
}
/// $03CB28: movea.l a6, a2
pub fn i_03cb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB2A;
    cpu.ir = 0x244E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $03CB2A: bra.w $3cb32
pub fn i_03cb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CB32;
    10
}
/// $03CB2E: movea.l $c(a6), a2
pub fn i_03cb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB32;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $03CB32: cmpi.w #$0, $82(a2)
pub fn i_03cb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB38;
    cpu.ir = 0xC6A;
    let ad2 = cpu.a[2].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03CB38: beq.w $3cb72
pub fn i_03cb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3CB72; return 10; }
    8
}
/// $03CB3C: subi.w #$1, $82(a2)
pub fn i_03cb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB42;
    cpu.ir = 0x46A;
    let ad1 = cpu.a[2].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CB42: move.w #$10f5, d0
pub fn i_03cb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB46;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10F5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03CB46: jsr $2352.l
pub fn i_03cb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CB4C);
    cpu.pc = 0x2352;
    16
}
/// $03CB4C: lea.l $9bec2.l, a1
pub fn i_03cb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB52;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x9BEC2;
    20
}
/// $03CB52: jsr $5eab6.l
pub fn i_03cb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CB58);
    cpu.pc = 0x5EAB6;
    16
}
/// $03CB58: move.w $22(a2), $22(a0)
pub fn i_03cb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB5E;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03CB5E: move.w $24(a2), $24(a0)
pub fn i_03cb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB64;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03CB64: jsr $517fe.l
pub fn i_03cb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CB6A);
    cpu.pc = 0x517FE;
    16
}
/// $03CB6A: addi.w #$1c, $24(a0)
pub fn i_03cb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB70;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CB70: rts
pub fn i_03cb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03CB72: movem.l (a7)+, a0
pub fn i_03cb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB76;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $03CB76: rts
pub fn i_03cb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Rocket_Fwd_03cb78  ($03CB78, 68 B, player/fire_shells_03c62a.s)
/// $03CB78: lea.l $3d4ea(pc), a1
pub fn i_03cb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D4EA;
    16
}
/// $03CB7C: jsr $4ae.l
pub fn i_03cb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CB82);
    cpu.pc = 0x4AE;
    16
}
/// $03CB82: jsr $5dd02.l
pub fn i_03cb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CB88);
    cpu.pc = 0x5DD02;
    16
}
/// $03CB88: jsr $3cb16(pc)
pub fn i_03cb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CB8C);
    cpu.pc = 0x3CB16;
    16
}
/// $03CB8C: btst.b #$0, $3a(a6)
pub fn i_03cb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB92;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CB92: bne.w $3cba4
pub fn i_03cb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB96;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CBA4; return 10; }
    8
}
/// $03CB96: addi.w #$10, $22(a0)
pub fn i_03cb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CB9C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CB9C: move.b #$0, d0
pub fn i_03cb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBA0;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CBA0: bra.w $3cbae
pub fn i_03cba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CBAE;
    10
}
/// $03CBA4: subi.w #$10, $22(a0)
pub fn i_03cba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBAA;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CBAA: move.b #$ff, d0
pub fn i_03cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBAE;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CBAE: move.b #$0, d1
pub fn i_03cbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBB2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CBB2: move.b d1, $98(a0)
pub fn i_03cbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBB6;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CBB6: move.b d0, $99(a0)
pub fn i_03cbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBBA;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CBBA: rts
pub fn i_03cbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Rocket_Up_03cbbc  ($03CBBC, 38 B, player/fire_shells_03c62a.s)
/// $03CBBC: lea.l $3d4f8(pc), a1
pub fn i_03cbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D4F8;
    16
}
/// $03CBC0: jsr $4ae.l
pub fn i_03cbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CBC6);
    cpu.pc = 0x4AE;
    16
}
/// $03CBC6: jsr $5dd02.l
pub fn i_03cbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CBCC);
    cpu.pc = 0x5DD02;
    16
}
/// $03CBCC: jsr $3cb16(pc)
pub fn i_03cbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CBD0);
    cpu.pc = 0x3CB16;
    16
}
/// $03CBD0: move.b #$1, d1
pub fn i_03cbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBD4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CBD4: move.b #$0, d0
pub fn i_03cbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBD8;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CBD8: move.b d1, $98(a0)
pub fn i_03cbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBDC;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CBDC: move.b d0, $99(a0)
pub fn i_03cbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBE0;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CBE0: rts
pub fn i_03cbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Rocket_Back_03cbe2  ($03CBE2, 80 B, player/fire_shells_03c62a.s)
/// $03CBE2: lea.l $3d4ea(pc), a1
pub fn i_03cbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D4EA;
    16
}
/// $03CBE6: jsr $4ae.l
pub fn i_03cbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CBEC);
    cpu.pc = 0x4AE;
    16
}
/// $03CBEC: jsr $5dd02.l
pub fn i_03cbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CBF2);
    cpu.pc = 0x5DD02;
    16
}
/// $03CBF2: move.b $3a(a6), d0
pub fn i_03cbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBF6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03CBF6: eori.b #$1, d0
pub fn i_03cbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBFA;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03CBFA: move.b d0, $3a(a0)
pub fn i_03cbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CBFE;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CBFE: jsr $3cb16(pc)
pub fn i_03cbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CC02);
    cpu.pc = 0x3CB16;
    16
}
/// $03CC02: btst.b #$0, $3a(a6)
pub fn i_03cc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC08;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CC08: bne.w $3cc1a
pub fn i_03cc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CC1A; return 10; }
    8
}
/// $03CC0C: subi.w #$10, $22(a0)
pub fn i_03cc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC12;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CC12: move.b #$0, d0
pub fn i_03cc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC16;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CC16: bra.w $3cc24
pub fn i_03cc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CC24;
    10
}
/// $03CC1A: addi.w #$10, $22(a0)
pub fn i_03cc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC20;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CC20: move.b #$ff, d0
pub fn i_03cc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC24;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CC24: move.b #$2, d1
pub fn i_03cc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC28;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CC28: move.b d1, $98(a0)
pub fn i_03cc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC2C;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CC2C: move.b d0, $99(a0)
pub fn i_03cc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC30;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CC30: rts
pub fn i_03cc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Rocket_FwdLow_03cc32  ($03CC32, 68 B, player/fire_shells_03c62a.s)
/// $03CC32: lea.l $3d4ea(pc), a1
pub fn i_03cc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D4EA;
    16
}
/// $03CC36: jsr $4ae.l
pub fn i_03cc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CC3C);
    cpu.pc = 0x4AE;
    16
}
/// $03CC3C: jsr $5dd02.l
pub fn i_03cc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CC42);
    cpu.pc = 0x5DD02;
    16
}
/// $03CC42: subi.w #$c, $24(a0)
pub fn i_03cc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC48;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CC48: jsr $3cb16(pc)
pub fn i_03cc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CC4C);
    cpu.pc = 0x3CB16;
    16
}
/// $03CC4C: subi.w #$c, $24(a0)
pub fn i_03cc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC52;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CC52: btst.b #$0, $3a(a6)
pub fn i_03cc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC58;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CC58: bne.w $3cc64
pub fn i_03cc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC5C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CC64; return 10; }
    8
}
/// $03CC5C: move.b #$0, d0
pub fn i_03cc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC60;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CC60: bra.w $3cc68
pub fn i_03cc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC64;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CC68;
    10
}
/// $03CC64: move.b #$ff, d0
pub fn i_03cc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC68;
    cpu.ir = 0x103C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CC68: move.b #$0, d1
pub fn i_03cc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC6C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CC6C: move.b d1, $98(a0)
pub fn i_03cc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC70;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CC70: move.b d0, $99(a0)
pub fn i_03cc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CC74;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x039C98, i_039c98),
    (0x039C9C, i_039c9c),
    (0x039CA2, i_039ca2),
    (0x039CA8, i_039ca8),
    (0x039CAC, i_039cac),
    (0x039CB0, i_039cb0),
    (0x039CB4, i_039cb4),
    (0x039CBA, i_039cba),
    (0x039CE6, i_039ce6),
    (0x039CEA, i_039cea),
    (0x039CF0, i_039cf0),
    (0x039CF4, i_039cf4),
    (0x039CFA, i_039cfa),
    (0x039CFE, i_039cfe),
    (0x039D02, i_039d02),
    (0x039D06, i_039d06),
    (0x039D0C, i_039d0c),
    (0x039D38, i_039d38),
    (0x039D3C, i_039d3c),
    (0x039D42, i_039d42),
    (0x039D46, i_039d46),
    (0x039D4A, i_039d4a),
    (0x039D4E, i_039d4e),
    (0x039D54, i_039d54),
    (0x039D80, i_039d80),
    (0x039D82, i_039d82),
    (0x039D88, i_039d88),
    (0x039D8C, i_039d8c),
    (0x039D90, i_039d90),
    (0x039D94, i_039d94),
    (0x039D9A, i_039d9a),
    (0x039DC6, i_039dc6),
    (0x039DC8, i_039dc8),
    (0x039DCE, i_039dce),
    (0x039DD2, i_039dd2),
    (0x039DD6, i_039dd6),
    (0x039DDA, i_039dda),
    (0x039DE0, i_039de0),
    (0x039E0C, i_039e0c),
    (0x039E0E, i_039e0e),
    (0x039E14, i_039e14),
    (0x039E18, i_039e18),
    (0x039E1C, i_039e1c),
    (0x039E20, i_039e20),
    (0x039E26, i_039e26),
    (0x039E52, i_039e52),
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
    (0x03A318, i_03a318),
    (0x03A31E, i_03a31e),
    (0x03A322, i_03a322),
    (0x03A326, i_03a326),
    (0x03A32A, i_03a32a),
    (0x03A330, i_03a330),
    (0x03A35C, i_03a35c),
    (0x03A35E, i_03a35e),
    (0x03A364, i_03a364),
    (0x03A368, i_03a368),
    (0x03A36C, i_03a36c),
    (0x03A370, i_03a370),
    (0x03A376, i_03a376),
    (0x03A3A2, i_03a3a2),
    (0x03A3A4, i_03a3a4),
    (0x03A3AA, i_03a3aa),
    (0x03A3B0, i_03a3b0),
    (0x03A3B4, i_03a3b4),
    (0x03A3B8, i_03a3b8),
    (0x03A3BC, i_03a3bc),
    (0x03A3C2, i_03a3c2),
    (0x03A3EE, i_03a3ee),
    (0x03A3F0, i_03a3f0),
    (0x03A3F6, i_03a3f6),
    (0x03A3FC, i_03a3fc),
    (0x03A400, i_03a400),
    (0x03A404, i_03a404),
    (0x03A408, i_03a408),
    (0x03A40E, i_03a40e),
    (0x03A43A, i_03a43a),
    (0x03A43C, i_03a43c),
    (0x03A442, i_03a442),
    (0x03A446, i_03a446),
    (0x03A44A, i_03a44a),
    (0x03A44E, i_03a44e),
    (0x03A454, i_03a454),
    (0x03A480, i_03a480),
    (0x03A482, i_03a482),
    (0x03A488, i_03a488),
    (0x03A48C, i_03a48c),
    (0x03A490, i_03a490),
    (0x03A494, i_03a494),
    (0x03A49A, i_03a49a),
    (0x03A4C6, i_03a4c6),
    (0x03A4C8, i_03a4c8),
    (0x03A4CE, i_03a4ce),
    (0x03A4D4, i_03a4d4),
    (0x03A4D8, i_03a4d8),
    (0x03A4DC, i_03a4dc),
    (0x03A4E0, i_03a4e0),
    (0x03A4E6, i_03a4e6),
    (0x03A512, i_03a512),
    (0x03A514, i_03a514),
    (0x03A51A, i_03a51a),
    (0x03A520, i_03a520),
    (0x03A524, i_03a524),
    (0x03A528, i_03a528),
    (0x03A52C, i_03a52c),
    (0x03A532, i_03a532),
    (0x03A55E, i_03a55e),
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
    (0x03A6A2, i_03a6a2),
    (0x03A6A8, i_03a6a8),
    (0x03A6AE, i_03a6ae),
    (0x03A6B2, i_03a6b2),
    (0x03A6B6, i_03a6b6),
    (0x03A6BA, i_03a6ba),
    (0x03A6C0, i_03a6c0),
    (0x03A6EC, i_03a6ec),
    (0x03A6EE, i_03a6ee),
    (0x03A6F4, i_03a6f4),
    (0x03A6FA, i_03a6fa),
    (0x03A6FE, i_03a6fe),
    (0x03A702, i_03a702),
    (0x03A706, i_03a706),
    (0x03A70C, i_03a70c),
    (0x03A738, i_03a738),
    (0x03A73A, i_03a73a),
    (0x03A740, i_03a740),
    (0x03A744, i_03a744),
    (0x03A74A, i_03a74a),
    (0x03A74E, i_03a74e),
    (0x03A752, i_03a752),
    (0x03A756, i_03a756),
    (0x03A75C, i_03a75c),
    (0x03A788, i_03a788),
    (0x03A78C, i_03a78c),
    (0x03A792, i_03a792),
    (0x03A796, i_03a796),
    (0x03A79C, i_03a79c),
    (0x03A7A0, i_03a7a0),
    (0x03A7A4, i_03a7a4),
    (0x03A7A8, i_03a7a8),
    (0x03A7AE, i_03a7ae),
    (0x03A7DA, i_03a7da),
    (0x03A7DE, i_03a7de),
    (0x03A7E4, i_03a7e4),
    (0x03A7E8, i_03a7e8),
    (0x03A7EC, i_03a7ec),
    (0x03A7F0, i_03a7f0),
    (0x03A7F6, i_03a7f6),
    (0x03A822, i_03a822),
    (0x03A824, i_03a824),
    (0x03A82A, i_03a82a),
    (0x03A82E, i_03a82e),
    (0x03A834, i_03a834),
    (0x03A838, i_03a838),
    (0x03A83C, i_03a83c),
    (0x03A840, i_03a840),
    (0x03A846, i_03a846),
    (0x03A872, i_03a872),
    (0x03A874, i_03a874),
    (0x03A87A, i_03a87a),
    (0x03A87E, i_03a87e),
    (0x03A884, i_03a884),
    (0x03A888, i_03a888),
    (0x03A88C, i_03a88c),
    (0x03A890, i_03a890),
    (0x03A896, i_03a896),
    (0x03A8C2, i_03a8c2),
    (0x03A8C6, i_03a8c6),
    (0x03A8CC, i_03a8cc),
    (0x03A8D0, i_03a8d0),
    (0x03A8D4, i_03a8d4),
    (0x03A8D8, i_03a8d8),
    (0x03A8DE, i_03a8de),
    (0x03A90A, i_03a90a),
    (0x03A90C, i_03a90c),
    (0x03A912, i_03a912),
    (0x03A916, i_03a916),
    (0x03A91C, i_03a91c),
    (0x03A920, i_03a920),
    (0x03A924, i_03a924),
    (0x03A928, i_03a928),
    (0x03A92E, i_03a92e),
    (0x03A95A, i_03a95a),
    (0x03A95E, i_03a95e),
    (0x03A964, i_03a964),
    (0x03A968, i_03a968),
    (0x03A96E, i_03a96e),
    (0x03A972, i_03a972),
    (0x03A976, i_03a976),
    (0x03A97A, i_03a97a),
    (0x03A980, i_03a980),
    (0x03A9AC, i_03a9ac),
    (0x03A9B0, i_03a9b0),
    (0x03A9B6, i_03a9b6),
    (0x03A9BA, i_03a9ba),
    (0x03A9BE, i_03a9be),
    (0x03A9C2, i_03a9c2),
    (0x03A9C8, i_03a9c8),
    (0x03A9F4, i_03a9f4),
    (0x03A9F6, i_03a9f6),
    (0x03A9FC, i_03a9fc),
    (0x03AA00, i_03aa00),
    (0x03AA06, i_03aa06),
    (0x03AA0A, i_03aa0a),
    (0x03AA0E, i_03aa0e),
    (0x03AA12, i_03aa12),
    (0x03AA18, i_03aa18),
    (0x03AA44, i_03aa44),
    (0x03AA48, i_03aa48),
    (0x03AA4E, i_03aa4e),
    (0x03AA52, i_03aa52),
    (0x03AA58, i_03aa58),
    (0x03AA5C, i_03aa5c),
    (0x03AA60, i_03aa60),
    (0x03AA64, i_03aa64),
    (0x03AA6A, i_03aa6a),
    (0x03AA96, i_03aa96),
    (0x03AA9A, i_03aa9a),
    (0x03AAA0, i_03aaa0),
    (0x03AAA4, i_03aaa4),
    (0x03AAA8, i_03aaa8),
    (0x03AAAC, i_03aaac),
    (0x03AAB2, i_03aab2),
    (0x03AADE, i_03aade),
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
    (0x03AD58, i_03ad58),
    (0x03AD5E, i_03ad5e),
    (0x03AD62, i_03ad62),
    (0x03AD68, i_03ad68),
    (0x03AD6C, i_03ad6c),
    (0x03AD70, i_03ad70),
    (0x03AD74, i_03ad74),
    (0x03AD7A, i_03ad7a),
    (0x03ADA6, i_03ada6),
    (0x03ADAA, i_03adaa),
    (0x03ADB0, i_03adb0),
    (0x03ADB4, i_03adb4),
    (0x03ADBA, i_03adba),
    (0x03ADBE, i_03adbe),
    (0x03ADC2, i_03adc2),
    (0x03ADC6, i_03adc6),
    (0x03ADCC, i_03adcc),
    (0x03ADF8, i_03adf8),
    (0x03ADFC, i_03adfc),
    (0x03AE02, i_03ae02),
    (0x03AE06, i_03ae06),
    (0x03AE0C, i_03ae0c),
    (0x03AE10, i_03ae10),
    (0x03AE14, i_03ae14),
    (0x03AE18, i_03ae18),
    (0x03AE1E, i_03ae1e),
    (0x03AE4A, i_03ae4a),
    (0x03AE4E, i_03ae4e),
    (0x03AE54, i_03ae54),
    (0x03AE58, i_03ae58),
    (0x03AE5C, i_03ae5c),
    (0x03AE60, i_03ae60),
    (0x03AE66, i_03ae66),
    (0x03AE92, i_03ae92),
    (0x03AE94, i_03ae94),
    (0x03AE9A, i_03ae9a),
    (0x03AE9E, i_03ae9e),
    (0x03AEA4, i_03aea4),
    (0x03AEA8, i_03aea8),
    (0x03AEAC, i_03aeac),
    (0x03AEB0, i_03aeb0),
    (0x03AEB6, i_03aeb6),
    (0x03AEE2, i_03aee2),
    (0x03AEE6, i_03aee6),
    (0x03AEEC, i_03aeec),
    (0x03AEF0, i_03aef0),
    (0x03AEF6, i_03aef6),
    (0x03AEFA, i_03aefa),
    (0x03AEFE, i_03aefe),
    (0x03AF02, i_03af02),
    (0x03AF08, i_03af08),
    (0x03AF34, i_03af34),
    (0x03AF38, i_03af38),
    (0x03AF3E, i_03af3e),
    (0x03AF42, i_03af42),
    (0x03AF48, i_03af48),
    (0x03AF4C, i_03af4c),
    (0x03AF50, i_03af50),
    (0x03AF54, i_03af54),
    (0x03AF5A, i_03af5a),
    (0x03AF86, i_03af86),
    (0x03AF8A, i_03af8a),
    (0x03AF90, i_03af90),
    (0x03AF94, i_03af94),
    (0x03AF98, i_03af98),
    (0x03AF9C, i_03af9c),
    (0x03AFA2, i_03afa2),
    (0x03AFCE, i_03afce),
    (0x03AFD0, i_03afd0),
    (0x03AFD6, i_03afd6),
    (0x03AFDA, i_03afda),
    (0x03AFE0, i_03afe0),
    (0x03AFE4, i_03afe4),
    (0x03AFE8, i_03afe8),
    (0x03AFEC, i_03afec),
    (0x03AFF2, i_03aff2),
    (0x03B01E, i_03b01e),
    (0x03B022, i_03b022),
    (0x03B028, i_03b028),
    (0x03B02C, i_03b02c),
    (0x03B032, i_03b032),
    (0x03B036, i_03b036),
    (0x03B03A, i_03b03a),
    (0x03B03E, i_03b03e),
    (0x03B044, i_03b044),
    (0x03B070, i_03b070),
    (0x03B074, i_03b074),
    (0x03B07A, i_03b07a),
    (0x03B07E, i_03b07e),
    (0x03B084, i_03b084),
    (0x03B088, i_03b088),
    (0x03B08C, i_03b08c),
    (0x03B090, i_03b090),
    (0x03B096, i_03b096),
    (0x03B0C2, i_03b0c2),
    (0x03B0C6, i_03b0c6),
    (0x03B0CC, i_03b0cc),
    (0x03B0D0, i_03b0d0),
    (0x03B0D4, i_03b0d4),
    (0x03B0D8, i_03b0d8),
    (0x03B0DE, i_03b0de),
    (0x03B10A, i_03b10a),
    (0x03B10C, i_03b10c),
    (0x03B112, i_03b112),
    (0x03B116, i_03b116),
    (0x03B11C, i_03b11c),
    (0x03B120, i_03b120),
    (0x03B124, i_03b124),
    (0x03B128, i_03b128),
    (0x03B12E, i_03b12e),
    (0x03B15A, i_03b15a),
    (0x03B15E, i_03b15e),
    (0x03B164, i_03b164),
    (0x03B168, i_03b168),
    (0x03B16E, i_03b16e),
    (0x03B172, i_03b172),
    (0x03B176, i_03b176),
    (0x03B17A, i_03b17a),
    (0x03B180, i_03b180),
    (0x03B1AC, i_03b1ac),
    (0x03B1B0, i_03b1b0),
    (0x03B1B6, i_03b1b6),
    (0x03B1BA, i_03b1ba),
    (0x03B1C0, i_03b1c0),
    (0x03B1C4, i_03b1c4),
    (0x03B1C8, i_03b1c8),
    (0x03B1CC, i_03b1cc),
    (0x03B1D2, i_03b1d2),
    (0x03B1FE, i_03b1fe),
    (0x03B202, i_03b202),
    (0x03B208, i_03b208),
    (0x03B20C, i_03b20c),
    (0x03B210, i_03b210),
    (0x03B214, i_03b214),
    (0x03B21A, i_03b21a),
    (0x03B246, i_03b246),
    (0x03B248, i_03b248),
    (0x03B24E, i_03b24e),
    (0x03B252, i_03b252),
    (0x03B258, i_03b258),
    (0x03B25C, i_03b25c),
    (0x03B260, i_03b260),
    (0x03B264, i_03b264),
    (0x03B26A, i_03b26a),
    (0x03B296, i_03b296),
    (0x03B29A, i_03b29a),
    (0x03B2A0, i_03b2a0),
    (0x03B2A4, i_03b2a4),
    (0x03B2AA, i_03b2aa),
    (0x03B2AE, i_03b2ae),
    (0x03B2B2, i_03b2b2),
    (0x03B2B6, i_03b2b6),
    (0x03B2BC, i_03b2bc),
    (0x03B2E8, i_03b2e8),
    (0x03B2EC, i_03b2ec),
    (0x03B2F2, i_03b2f2),
    (0x03B2F6, i_03b2f6),
    (0x03B2FC, i_03b2fc),
    (0x03B300, i_03b300),
    (0x03B304, i_03b304),
    (0x03B308, i_03b308),
    (0x03B30E, i_03b30e),
    (0x03B33A, i_03b33a),
    (0x03B33E, i_03b33e),
    (0x03B344, i_03b344),
    (0x03B348, i_03b348),
    (0x03B34C, i_03b34c),
    (0x03B350, i_03b350),
    (0x03B356, i_03b356),
    (0x03B382, i_03b382),
    (0x03B384, i_03b384),
    (0x03B38A, i_03b38a),
    (0x03B38E, i_03b38e),
    (0x03B394, i_03b394),
    (0x03B398, i_03b398),
    (0x03B39C, i_03b39c),
    (0x03B3A0, i_03b3a0),
    (0x03B3A6, i_03b3a6),
    (0x03B3D2, i_03b3d2),
    (0x03B3D6, i_03b3d6),
    (0x03B3DC, i_03b3dc),
    (0x03B3E0, i_03b3e0),
    (0x03B3E6, i_03b3e6),
    (0x03B3EA, i_03b3ea),
    (0x03B3EE, i_03b3ee),
    (0x03B3F2, i_03b3f2),
    (0x03B3F8, i_03b3f8),
    (0x03B424, i_03b424),
    (0x03B428, i_03b428),
    (0x03B42E, i_03b42e),
    (0x03B432, i_03b432),
    (0x03B438, i_03b438),
    (0x03B43C, i_03b43c),
    (0x03B440, i_03b440),
    (0x03B444, i_03b444),
    (0x03B44A, i_03b44a),
    (0x03B476, i_03b476),
    (0x03B47A, i_03b47a),
    (0x03B480, i_03b480),
    (0x03B484, i_03b484),
    (0x03B488, i_03b488),
    (0x03B48C, i_03b48c),
    (0x03B492, i_03b492),
    (0x03B4BE, i_03b4be),
    (0x03B4C0, i_03b4c0),
    (0x03B4C6, i_03b4c6),
    (0x03B4CA, i_03b4ca),
    (0x03B4D0, i_03b4d0),
    (0x03B4D4, i_03b4d4),
    (0x03B4D8, i_03b4d8),
    (0x03B4DC, i_03b4dc),
    (0x03B4E2, i_03b4e2),
    (0x03B50E, i_03b50e),
    (0x03B512, i_03b512),
    (0x03B518, i_03b518),
    (0x03B51C, i_03b51c),
    (0x03B522, i_03b522),
    (0x03B526, i_03b526),
    (0x03B52A, i_03b52a),
    (0x03B52E, i_03b52e),
    (0x03B534, i_03b534),
    (0x03B560, i_03b560),
    (0x03B564, i_03b564),
    (0x03B56A, i_03b56a),
    (0x03B56E, i_03b56e),
    (0x03B572, i_03b572),
    (0x03B576, i_03b576),
    (0x03B57C, i_03b57c),
    (0x03B5A8, i_03b5a8),
    (0x03B5AA, i_03b5aa),
    (0x03B5B0, i_03b5b0),
    (0x03B5B4, i_03b5b4),
    (0x03B5BA, i_03b5ba),
    (0x03B5BE, i_03b5be),
    (0x03B5C2, i_03b5c2),
    (0x03B5C6, i_03b5c6),
    (0x03B5CC, i_03b5cc),
    (0x03B5F8, i_03b5f8),
    (0x03B5FC, i_03b5fc),
    (0x03B602, i_03b602),
    (0x03B606, i_03b606),
    (0x03B60C, i_03b60c),
    (0x03B610, i_03b610),
    (0x03B614, i_03b614),
    (0x03B618, i_03b618),
    (0x03B61E, i_03b61e),
    (0x03B64A, i_03b64a),
    (0x03B64E, i_03b64e),
    (0x03B654, i_03b654),
    (0x03B658, i_03b658),
    (0x03B65C, i_03b65c),
    (0x03B660, i_03b660),
    (0x03B666, i_03b666),
    (0x03B692, i_03b692),
    (0x03B694, i_03b694),
    (0x03B69A, i_03b69a),
    (0x03B69E, i_03b69e),
    (0x03B6A4, i_03b6a4),
    (0x03B6A8, i_03b6a8),
    (0x03B6AC, i_03b6ac),
    (0x03B6B0, i_03b6b0),
    (0x03B6B6, i_03b6b6),
    (0x03B6E2, i_03b6e2),
    (0x03B6E6, i_03b6e6),
    (0x03B6EC, i_03b6ec),
    (0x03B6F0, i_03b6f0),
    (0x03B6F6, i_03b6f6),
    (0x03B6FA, i_03b6fa),
    (0x03B6FE, i_03b6fe),
    (0x03B702, i_03b702),
    (0x03B708, i_03b708),
    (0x03B734, i_03b734),
    (0x03B738, i_03b738),
    (0x03B73E, i_03b73e),
    (0x03B742, i_03b742),
    (0x03B746, i_03b746),
    (0x03B74A, i_03b74a),
    (0x03B750, i_03b750),
    (0x03B77C, i_03b77c),
    (0x03B77E, i_03b77e),
    (0x03B784, i_03b784),
    (0x03B788, i_03b788),
    (0x03B78E, i_03b78e),
    (0x03B792, i_03b792),
    (0x03B796, i_03b796),
    (0x03B79A, i_03b79a),
    (0x03B7A0, i_03b7a0),
    (0x03B7CC, i_03b7cc),
    (0x03B7D0, i_03b7d0),
    (0x03B7D6, i_03b7d6),
    (0x03B7DA, i_03b7da),
    (0x03B7E0, i_03b7e0),
    (0x03B7E4, i_03b7e4),
    (0x03B7E8, i_03b7e8),
    (0x03B7EC, i_03b7ec),
    (0x03B7F2, i_03b7f2),
    (0x03B81E, i_03b81e),
    (0x03B822, i_03b822),
    (0x03B828, i_03b828),
    (0x03B82C, i_03b82c),
    (0x03B830, i_03b830),
    (0x03B834, i_03b834),
    (0x03B83A, i_03b83a),
    (0x03B866, i_03b866),
    (0x03B868, i_03b868),
    (0x03B86E, i_03b86e),
    (0x03B872, i_03b872),
    (0x03B878, i_03b878),
    (0x03B87C, i_03b87c),
    (0x03B880, i_03b880),
    (0x03B884, i_03b884),
    (0x03B88A, i_03b88a),
    (0x03B8B6, i_03b8b6),
    (0x03B8BA, i_03b8ba),
    (0x03B8C0, i_03b8c0),
    (0x03B8C4, i_03b8c4),
    (0x03B8CA, i_03b8ca),
    (0x03B8CE, i_03b8ce),
    (0x03B8D2, i_03b8d2),
    (0x03B8D6, i_03b8d6),
    (0x03B8DC, i_03b8dc),
    (0x03B908, i_03b908),
    (0x03B90C, i_03b90c),
    (0x03B912, i_03b912),
    (0x03B916, i_03b916),
    (0x03B91A, i_03b91a),
    (0x03B91E, i_03b91e),
    (0x03B924, i_03b924),
    (0x03B950, i_03b950),
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
    (0x03BA3C, i_03ba3c),
    (0x03BA42, i_03ba42),
    (0x03BA46, i_03ba46),
    (0x03BA4C, i_03ba4c),
    (0x03BA50, i_03ba50),
    (0x03BA54, i_03ba54),
    (0x03BA58, i_03ba58),
    (0x03BA5E, i_03ba5e),
    (0x03BA8A, i_03ba8a),
    (0x03BA8E, i_03ba8e),
    (0x03BA94, i_03ba94),
    (0x03BA98, i_03ba98),
    (0x03BA9E, i_03ba9e),
    (0x03BAA2, i_03baa2),
    (0x03BAA6, i_03baa6),
    (0x03BAAA, i_03baaa),
    (0x03BAB0, i_03bab0),
    (0x03BADC, i_03badc),
    (0x03BAE0, i_03bae0),
    (0x03BAE6, i_03bae6),
    (0x03BAEA, i_03baea),
    (0x03BAEE, i_03baee),
    (0x03BAF2, i_03baf2),
    (0x03BAF8, i_03baf8),
    (0x03BB24, i_03bb24),
    (0x03BB26, i_03bb26),
    (0x03BB2C, i_03bb2c),
    (0x03BB30, i_03bb30),
    (0x03BB36, i_03bb36),
    (0x03BB3A, i_03bb3a),
    (0x03BB3E, i_03bb3e),
    (0x03BB42, i_03bb42),
    (0x03BB48, i_03bb48),
    (0x03BB74, i_03bb74),
    (0x03BB78, i_03bb78),
    (0x03BB7E, i_03bb7e),
    (0x03BB82, i_03bb82),
    (0x03BB88, i_03bb88),
    (0x03BB8C, i_03bb8c),
    (0x03BB90, i_03bb90),
    (0x03BB94, i_03bb94),
    (0x03BB9A, i_03bb9a),
    (0x03BBC6, i_03bbc6),
    (0x03BBCA, i_03bbca),
    (0x03BBD0, i_03bbd0),
    (0x03BBD4, i_03bbd4),
    (0x03BBD8, i_03bbd8),
    (0x03BBDC, i_03bbdc),
    (0x03BBE2, i_03bbe2),
    (0x03BC0E, i_03bc0e),
    (0x03BC10, i_03bc10),
    (0x03BC16, i_03bc16),
    (0x03BC1C, i_03bc1c),
    (0x03BC22, i_03bc22),
    (0x03BC26, i_03bc26),
    (0x03BC2A, i_03bc2a),
    (0x03BC2E, i_03bc2e),
    (0x03BC34, i_03bc34),
    (0x03BC60, i_03bc60),
    (0x03BC62, i_03bc62),
    (0x03BC68, i_03bc68),
    (0x03BC6E, i_03bc6e),
    (0x03BC74, i_03bc74),
    (0x03BC78, i_03bc78),
    (0x03BC7C, i_03bc7c),
    (0x03BC80, i_03bc80),
    (0x03BC86, i_03bc86),
    (0x03BCB2, i_03bcb2),
    (0x03BCB4, i_03bcb4),
    (0x03BCBA, i_03bcba),
    (0x03BCC0, i_03bcc0),
    (0x03BCC6, i_03bcc6),
    (0x03BCCA, i_03bcca),
    (0x03BCCE, i_03bcce),
    (0x03BCD2, i_03bcd2),
    (0x03BCD8, i_03bcd8),
    (0x03BD04, i_03bd04),
    (0x03BD06, i_03bd06),
    (0x03BD0C, i_03bd0c),
    (0x03BD12, i_03bd12),
    (0x03BD18, i_03bd18),
    (0x03BD1C, i_03bd1c),
    (0x03BD20, i_03bd20),
    (0x03BD24, i_03bd24),
    (0x03BD2A, i_03bd2a),
    (0x03BD56, i_03bd56),
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
    (0x03BDFC, i_03bdfc),
    (0x03BE02, i_03be02),
    (0x03BE08, i_03be08),
    (0x03BE0E, i_03be0e),
    (0x03BE12, i_03be12),
    (0x03BE16, i_03be16),
    (0x03BE1A, i_03be1a),
    (0x03BE20, i_03be20),
    (0x03BE4C, i_03be4c),
    (0x03BE4E, i_03be4e),
    (0x03BE54, i_03be54),
    (0x03BE5A, i_03be5a),
    (0x03BE60, i_03be60),
    (0x03BE64, i_03be64),
    (0x03BE68, i_03be68),
    (0x03BE6C, i_03be6c),
    (0x03BE72, i_03be72),
    (0x03BE9E, i_03be9e),
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
    (0x03BF38, i_03bf38),
    (0x03BF3E, i_03bf3e),
    (0x03BF44, i_03bf44),
    (0x03BF48, i_03bf48),
    (0x03BF4C, i_03bf4c),
    (0x03BF50, i_03bf50),
    (0x03BF56, i_03bf56),
    (0x03BF82, i_03bf82),
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
    (0x03C06E, i_03c06e),
    (0x03C074, i_03c074),
    (0x03C07A, i_03c07a),
    (0x03C07E, i_03c07e),
    (0x03C084, i_03c084),
    (0x03C088, i_03c088),
    (0x03C08C, i_03c08c),
    (0x03C090, i_03c090),
    (0x03C096, i_03c096),
    (0x03C0C2, i_03c0c2),
    (0x03C0C6, i_03c0c6),
    (0x03C0CC, i_03c0cc),
    (0x03C0D0, i_03c0d0),
    (0x03C0D4, i_03c0d4),
    (0x03C0D8, i_03c0d8),
    (0x03C0DE, i_03c0de),
    (0x03C10A, i_03c10a),
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
    (0x03C1AA, i_03c1aa),
    (0x03C1B0, i_03c1b0),
    (0x03C1B6, i_03c1b6),
    (0x03C1BA, i_03c1ba),
    (0x03C1BE, i_03c1be),
    (0x03C1C2, i_03c1c2),
    (0x03C1C8, i_03c1c8),
    (0x03C1F4, i_03c1f4),
    (0x03C1F6, i_03c1f6),
    (0x03C1FC, i_03c1fc),
    (0x03C202, i_03c202),
    (0x03C206, i_03c206),
    (0x03C20A, i_03c20a),
    (0x03C20E, i_03c20e),
    (0x03C214, i_03c214),
    (0x03C240, i_03c240),
    (0x03C242, i_03c242),
    (0x03C248, i_03c248),
    (0x03C24E, i_03c24e),
    (0x03C252, i_03c252),
    (0x03C256, i_03c256),
    (0x03C25A, i_03c25a),
    (0x03C260, i_03c260),
    (0x03C28C, i_03c28c),
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
    (0x03C458, i_03c458),
    (0x03C45E, i_03c45e),
    (0x03C462, i_03c462),
    (0x03C468, i_03c468),
    (0x03C46C, i_03c46c),
    (0x03C470, i_03c470),
    (0x03C474, i_03c474),
    (0x03C47A, i_03c47a),
    (0x03C4A6, i_03c4a6),
    (0x03C4AA, i_03c4aa),
    (0x03C4B0, i_03c4b0),
    (0x03C4B4, i_03c4b4),
    (0x03C4BA, i_03c4ba),
    (0x03C4BE, i_03c4be),
    (0x03C4C2, i_03c4c2),
    (0x03C4C6, i_03c4c6),
    (0x03C4CC, i_03c4cc),
    (0x03C4F8, i_03c4f8),
    (0x03C4FC, i_03c4fc),
    (0x03C502, i_03c502),
    (0x03C506, i_03c506),
    (0x03C50A, i_03c50a),
    (0x03C50E, i_03c50e),
    (0x03C514, i_03c514),
    (0x03C540, i_03c540),
    (0x03C542, i_03c542),
    (0x03C548, i_03c548),
    (0x03C54C, i_03c54c),
    (0x03C550, i_03c550),
    (0x03C554, i_03c554),
    (0x03C55A, i_03c55a),
    (0x03C586, i_03c586),
    (0x03C588, i_03c588),
    (0x03C58C, i_03c58c),
    (0x03C592, i_03c592),
    (0x03C596, i_03c596),
    (0x03C59C, i_03c59c),
    (0x03C5A0, i_03c5a0),
    (0x03C5A4, i_03c5a4),
    (0x03C5A8, i_03c5a8),
    (0x03C5AE, i_03c5ae),
    (0x03C5DA, i_03c5da),
    (0x03C5DE, i_03c5de),
    (0x03C5E4, i_03c5e4),
    (0x03C5E8, i_03c5e8),
    (0x03C5EC, i_03c5ec),
    (0x03C5F0, i_03c5f0),
    (0x03C5F6, i_03c5f6),
    (0x03C622, i_03c622),
    (0x03C626, i_03c626),
    (0x03C62A, i_03c62a),
    (0x03C630, i_03c630),
    (0x03C632, i_03c632),
    (0x03C638, i_03c638),
    (0x03C63E, i_03c63e),
    (0x03C642, i_03c642),
    (0x03C646, i_03c646),
    (0x03C64A, i_03c64a),
    (0x03C650, i_03c650),
    (0x03C67C, i_03c67c),
    (0x03C67E, i_03c67e),
    (0x03C684, i_03c684),
    (0x03C688, i_03c688),
    (0x03C68C, i_03c68c),
    (0x03C690, i_03c690),
    (0x03C696, i_03c696),
    (0x03C6C2, i_03c6c2),
    (0x03C6C4, i_03c6c4),
    (0x03C6CA, i_03c6ca),
    (0x03C6D0, i_03c6d0),
    (0x03C6D4, i_03c6d4),
    (0x03C6D8, i_03c6d8),
    (0x03C6DC, i_03c6dc),
    (0x03C6E2, i_03c6e2),
    (0x03C70E, i_03c70e),
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
    (0x03C7C8, i_03c7c8),
    (0x03C7CC, i_03c7cc),
    (0x03C7D2, i_03c7d2),
    (0x03C7D6, i_03c7d6),
    (0x03C7DA, i_03c7da),
    (0x03C7DE, i_03c7de),
    (0x03C7E2, i_03c7e2),
    (0x03C7E6, i_03c7e6),
    (0x03C7EA, i_03c7ea),
    (0x03C7EC, i_03c7ec),
    (0x03C7F0, i_03c7f0),
    (0x03C7F6, i_03c7f6),
    (0x03C7FA, i_03c7fa),
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
    (0x03C854, i_03c854),
    (0x03C858, i_03c858),
    (0x03C85C, i_03c85c),
    (0x03C860, i_03c860),
    (0x03C862, i_03c862),
    (0x03C866, i_03c866),
    (0x03C86C, i_03c86c),
    (0x03C870, i_03c870),
    (0x03C872, i_03c872),
    (0x03C878, i_03c878),
    (0x03C87A, i_03c87a),
    (0x03C880, i_03c880),
    (0x03C884, i_03c884),
    (0x03C886, i_03c886),
    (0x03C88A, i_03c88a),
    (0x03C88E, i_03c88e),
    (0x03C894, i_03c894),
    (0x03C898, i_03c898),
    (0x03C89E, i_03c89e),
    (0x03C8A4, i_03c8a4),
    (0x03C8A8, i_03c8a8),
    (0x03C8AE, i_03c8ae),
    (0x03C8B2, i_03c8b2),
    (0x03C8B8, i_03c8b8),
    (0x03C8BE, i_03c8be),
    (0x03C8C4, i_03c8c4),
    (0x03C8CA, i_03c8ca),
    (0x03C8D0, i_03c8d0),
    (0x03C8D6, i_03c8d6),
    (0x03C8D8, i_03c8d8),
    (0x03C8DC, i_03c8dc),
    (0x03C8DE, i_03c8de),
    (0x03C8E2, i_03c8e2),
    (0x03C8E8, i_03c8e8),
    (0x03C8EC, i_03c8ec),
    (0x03C8F0, i_03c8f0),
    (0x03C8F4, i_03c8f4),
    (0x03C8F8, i_03c8f8),
    (0x03C8FA, i_03c8fa),
    (0x03C8FE, i_03c8fe),
    (0x03C902, i_03c902),
    (0x03C904, i_03c904),
    (0x03C908, i_03c908),
    (0x03C90C, i_03c90c),
    (0x03C910, i_03c910),
    (0x03C914, i_03c914),
    (0x03C918, i_03c918),
    (0x03C91A, i_03c91a),
    (0x03C91E, i_03c91e),
    (0x03C924, i_03c924),
    (0x03C928, i_03c928),
    (0x03C92C, i_03c92c),
    (0x03C930, i_03c930),
    (0x03C934, i_03c934),
    (0x03C938, i_03c938),
    (0x03C93C, i_03c93c),
    (0x03C940, i_03c940),
    (0x03C942, i_03c942),
    (0x03C946, i_03c946),
    (0x03C94C, i_03c94c),
    (0x03C950, i_03c950),
    (0x03C954, i_03c954),
    (0x03C958, i_03c958),
    (0x03C95C, i_03c95c),
    (0x03C95E, i_03c95e),
    (0x03C962, i_03c962),
    (0x03C966, i_03c966),
    (0x03C968, i_03c968),
    (0x03C96C, i_03c96c),
    (0x03C972, i_03c972),
    (0x03C978, i_03c978),
    (0x03C97C, i_03c97c),
    (0x03C980, i_03c980),
    (0x03C984, i_03c984),
    (0x03C988, i_03c988),
    (0x03C98C, i_03c98c),
    (0x03C990, i_03c990),
    (0x03C994, i_03c994),
    (0x03C996, i_03c996),
    (0x03C99A, i_03c99a),
    (0x03C9A0, i_03c9a0),
    (0x03C9A6, i_03c9a6),
    (0x03C9AA, i_03c9aa),
    (0x03C9AE, i_03c9ae),
    (0x03C9B2, i_03c9b2),
    (0x03C9B6, i_03c9b6),
    (0x03C9BA, i_03c9ba),
    (0x03C9BE, i_03c9be),
    (0x03C9C2, i_03c9c2),
    (0x03C9C4, i_03c9c4),
    (0x03C9CA, i_03c9ca),
    (0x03C9CC, i_03c9cc),
    (0x03C9D2, i_03c9d2),
    (0x03C9D6, i_03c9d6),
    (0x03C9D8, i_03c9d8),
    (0x03C9DC, i_03c9dc),
    (0x03C9E0, i_03c9e0),
    (0x03C9E6, i_03c9e6),
    (0x03C9EA, i_03c9ea),
    (0x03C9F0, i_03c9f0),
    (0x03C9F4, i_03c9f4),
    (0x03C9FA, i_03c9fa),
    (0x03CA00, i_03ca00),
    (0x03CA06, i_03ca06),
    (0x03CA0C, i_03ca0c),
    (0x03CA12, i_03ca12),
    (0x03CA18, i_03ca18),
    (0x03CA1E, i_03ca1e),
    (0x03CA20, i_03ca20),
    (0x03CA24, i_03ca24),
    (0x03CA26, i_03ca26),
    (0x03CA2A, i_03ca2a),
    (0x03CA30, i_03ca30),
    (0x03CA34, i_03ca34),
    (0x03CA38, i_03ca38),
    (0x03CA3C, i_03ca3c),
    (0x03CA40, i_03ca40),
    (0x03CA44, i_03ca44),
    (0x03CA48, i_03ca48),
    (0x03CA4C, i_03ca4c),
    (0x03CA4E, i_03ca4e),
    (0x03CA52, i_03ca52),
    (0x03CA56, i_03ca56),
    (0x03CA5A, i_03ca5a),
    (0x03CA5E, i_03ca5e),
    (0x03CA62, i_03ca62),
    (0x03CA64, i_03ca64),
    (0x03CA68, i_03ca68),
    (0x03CA6E, i_03ca6e),
    (0x03CA72, i_03ca72),
    (0x03CA76, i_03ca76),
    (0x03CA7A, i_03ca7a),
    (0x03CA7E, i_03ca7e),
    (0x03CA82, i_03ca82),
    (0x03CA86, i_03ca86),
    (0x03CA8A, i_03ca8a),
    (0x03CA8C, i_03ca8c),
    (0x03CA90, i_03ca90),
    (0x03CA96, i_03ca96),
    (0x03CA9C, i_03ca9c),
    (0x03CAA0, i_03caa0),
    (0x03CAA4, i_03caa4),
    (0x03CAA8, i_03caa8),
    (0x03CAAC, i_03caac),
    (0x03CAB0, i_03cab0),
    (0x03CAB4, i_03cab4),
    (0x03CAB8, i_03cab8),
    (0x03CABA, i_03caba),
    (0x03CABE, i_03cabe),
    (0x03CAC4, i_03cac4),
    (0x03CACA, i_03caca),
    (0x03CACE, i_03cace),
    (0x03CAD2, i_03cad2),
    (0x03CAD6, i_03cad6),
    (0x03CADA, i_03cada),
    (0x03CADE, i_03cade),
    (0x03CAE2, i_03cae2),
    (0x03CAE6, i_03cae6),
    (0x03CAE8, i_03cae8),
    (0x03CAEC, i_03caec),
    (0x03CAF2, i_03caf2),
    (0x03CAF8, i_03caf8),
    (0x03CAFC, i_03cafc),
    (0x03CB00, i_03cb00),
    (0x03CB04, i_03cb04),
    (0x03CB08, i_03cb08),
    (0x03CB0C, i_03cb0c),
    (0x03CB10, i_03cb10),
    (0x03CB14, i_03cb14),
    (0x03CB16, i_03cb16),
    (0x03CB1C, i_03cb1c),
    (0x03CB1E, i_03cb1e),
    (0x03CB24, i_03cb24),
    (0x03CB28, i_03cb28),
    (0x03CB2A, i_03cb2a),
    (0x03CB2E, i_03cb2e),
    (0x03CB32, i_03cb32),
    (0x03CB38, i_03cb38),
    (0x03CB3C, i_03cb3c),
    (0x03CB42, i_03cb42),
    (0x03CB46, i_03cb46),
    (0x03CB4C, i_03cb4c),
    (0x03CB52, i_03cb52),
    (0x03CB58, i_03cb58),
    (0x03CB5E, i_03cb5e),
    (0x03CB64, i_03cb64),
    (0x03CB6A, i_03cb6a),
    (0x03CB70, i_03cb70),
    (0x03CB72, i_03cb72),
    (0x03CB76, i_03cb76),
    (0x03CB78, i_03cb78),
    (0x03CB7C, i_03cb7c),
    (0x03CB82, i_03cb82),
    (0x03CB88, i_03cb88),
    (0x03CB8C, i_03cb8c),
    (0x03CB92, i_03cb92),
    (0x03CB96, i_03cb96),
    (0x03CB9C, i_03cb9c),
    (0x03CBA0, i_03cba0),
    (0x03CBA4, i_03cba4),
    (0x03CBAA, i_03cbaa),
    (0x03CBAE, i_03cbae),
    (0x03CBB2, i_03cbb2),
    (0x03CBB6, i_03cbb6),
    (0x03CBBA, i_03cbba),
    (0x03CBBC, i_03cbbc),
    (0x03CBC0, i_03cbc0),
    (0x03CBC6, i_03cbc6),
    (0x03CBCC, i_03cbcc),
    (0x03CBD0, i_03cbd0),
    (0x03CBD4, i_03cbd4),
    (0x03CBD8, i_03cbd8),
    (0x03CBDC, i_03cbdc),
    (0x03CBE0, i_03cbe0),
    (0x03CBE2, i_03cbe2),
    (0x03CBE6, i_03cbe6),
    (0x03CBEC, i_03cbec),
    (0x03CBF2, i_03cbf2),
    (0x03CBF6, i_03cbf6),
    (0x03CBFA, i_03cbfa),
    (0x03CBFE, i_03cbfe),
    (0x03CC02, i_03cc02),
    (0x03CC08, i_03cc08),
    (0x03CC0C, i_03cc0c),
    (0x03CC12, i_03cc12),
    (0x03CC16, i_03cc16),
    (0x03CC1A, i_03cc1a),
    (0x03CC20, i_03cc20),
    (0x03CC24, i_03cc24),
    (0x03CC28, i_03cc28),
    (0x03CC2C, i_03cc2c),
    (0x03CC30, i_03cc30),
    (0x03CC32, i_03cc32),
    (0x03CC36, i_03cc36),
    (0x03CC3C, i_03cc3c),
    (0x03CC42, i_03cc42),
    (0x03CC48, i_03cc48),
    (0x03CC4C, i_03cc4c),
    (0x03CC52, i_03cc52),
    (0x03CC58, i_03cc58),
    (0x03CC5C, i_03cc5c),
    (0x03CC60, i_03cc60),
    (0x03CC64, i_03cc64),
    (0x03CC68, i_03cc68),
    (0x03CC6C, i_03cc6c),
    (0x03CC70, i_03cc70),
];
