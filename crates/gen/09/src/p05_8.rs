// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $05E960: move.b d1, $33(a6)
pub fn i_05e960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E964;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05E964: movea.l #$70a7, a1
pub fn i_05e964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E96A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x70A7;
    cpu.a[1] = v1;
    4
}
/// $05E96A: move.w $10e1e4.l, d0
pub fn i_05e96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E970;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10E1E4) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05E970: jsr $5d6c2.l
pub fn i_05e970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E976);
    cpu.pc = 0x5D6C2;
    16
}
/// $05E976: movea.l #$7147, a1
pub fn i_05e976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E97C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7147;
    cpu.a[1] = v1;
    4
}
/// $05E97C: move.w $10e1e6.l, d0
pub fn i_05e97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E982;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10E1E6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
// ===== JsrAbsThunk_05e982  ($05E982, 8 B, thunks/jsr_abs_thunks.c)
/// $05E982: jsr $5d6c2.l
pub fn i_05e982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E988;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5E988);
    cpu.pc = 0x5D6C2;
    16
}
/// $05E988: rts
pub fn i_05e988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E98A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MarkFlag2TimerMax_05e98a  ($05E98A, 14 B, props/late_props_turrets_05e000.s)
/// $05E98A: ori.b #$4, $12(a6)
pub fn i_05e98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E990;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05E990: move.w #$ffff, $30(a6)
pub fn i_05e990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E996;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E996: rts
pub fn i_05e996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E998;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rng_Seed_05e998  ($05E998, 30 B, props/late_props_turrets_05e000.s)
/// $05E998: lea.l $10e230.l, a0
pub fn i_05e998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E99E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E230;
    20
}
/// $05E99E: moveq #$1f, d7
pub fn i_05e99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A0;
    cpu.ir = 0x7E1F;
    cpu.d[7] = 0x1F;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E9A0: move.w d0, (a0)+
pub fn i_05e9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A2;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05E9A2: mulu.w #$b7c7, d0
pub fn i_05e9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A6;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0xB7C7;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05E9A6: addi.w #$81f5, d0
pub fn i_05e9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9AA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x81F5, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9AA: dbra d7, $5e9a0
pub fn i_05e9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9AE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5E9AE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5E9AE; return 14; }
    cpu.pc = 0x5E9A0;
    10
}
/// $05E9AE: clr.w $10e270.l
pub fn i_05e9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9B4;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E270) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E270, r2 as u16);
    16
}
/// $05E9B4: rts
pub fn i_05e9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RNG_LFSRStep_SelfSeed_05E9B6  ($05E9B6, 46 B, kernel/rng_lfsr_self_seed_05e9b6.s)
/// $05E9B6: lea.l $10e230.l, a4
pub fn i_05e9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9BC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E230;
    20
}
/// $05E9BC: move.w $10e270.l, d7
pub fn i_05e9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9C2;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E270) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05E9C2: addi.w #$2, d7
pub fn i_05e9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9C6;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9C6: move.w #$3e, d6
pub fn i_05e9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9CA;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E9CA: and.w d6, d7
pub fn i_05e9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9CC;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E9CC: move.w d7, $10e270.l
pub fn i_05e9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9D2;
    cpu.ir = 0x33C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E270, v1 as u16);
    16
}
/// $05E9D2: move.w (a4, d7.w), d0
pub fn i_05e9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9D6;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05E9D6: subi.w #$15, d7
pub fn i_05e9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9DA;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x15, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9DA: and.w d6, d7
pub fn i_05e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9DC;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E9DC: adda.w d7, a4
pub fn i_05e9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9DE;
    cpu.ir = 0xD8C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[4] = cpu.a[4].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05E9DE: eor.w d0, (a4)
pub fn i_05e9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9E0;
    cpu.ir = 0xB154;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    8
}
/// $05E9E0: move.w (a4), d0
pub fn i_05e9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9E2;
    cpu.ir = 0x3014;
    let v2 = bus.read16(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05E9E2: rts
pub fn i_05e9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RNG_LFSRStep_05E9E4  ($05E9E4, 56 B, kernel/rng_lfsr_step_05e9e4.s)
/// $05E9E4: move.w d0, d5
pub fn i_05e9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9E6;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E9E6: lea.l $10e230.l, a4
pub fn i_05e9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9EC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E230;
    20
}
/// $05E9EC: move.w $10e270.l, d7
pub fn i_05e9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9F2;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E270) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05E9F2: addi.w #$2, d7
pub fn i_05e9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9F6;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9F6: move.w #$3e, d6
pub fn i_05e9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9FA;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E9FA: and.w d6, d7
pub fn i_05e9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9FC;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E9FC: move.w d7, $10e270.l
pub fn i_05e9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA02;
    cpu.ir = 0x33C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E270, v1 as u16);
    16
}
/// $05EA02: move.w (a4, d7.w), d0
pub fn i_05ea02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA06;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05EA06: rol.w #$1, d0
pub fn i_05ea06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA08;
    cpu.ir = 0xE358;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05EA08: subi.w #$15, d7
pub fn i_05ea08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA0C;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x15, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05EA0C: and.w d6, d7
pub fn i_05ea0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA0E;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05EA0E: eor.w d0, (a4, d7.w)
pub fn i_05ea0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA12;
    cpu.ir = 0xB174;
    let ad1 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    14
}
/// $05EA12: move.w (a4, d7.w), d0
pub fn i_05ea12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA16;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05EA16: mulu.w d5, d0
pub fn i_05ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA18;
    cpu.ir = 0xC0C5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05EA18: swap d0
pub fn i_05ea18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA1A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05EA1A: rts
pub fn i_05ea1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rng_Mask_05ea1c  ($05EA1C, 54 B, props/late_props_turrets_05e000.s)
/// $05EA1C: move.w d0, d5
pub fn i_05ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA1E;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EA1E: lea.l $10e230.l, a4
pub fn i_05ea1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA24;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E230;
    20
}
/// $05EA24: move.w $10e270.l, d7
pub fn i_05ea24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA2A;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E270) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05EA2A: addi.w #$2, d7
pub fn i_05ea2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA2E;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05EA2E: move.w #$3e, d6
pub fn i_05ea2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA32;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EA32: and.w d6, d7
pub fn i_05ea32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA34;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05EA34: move.w d7, $10e270.l
pub fn i_05ea34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA3A;
    cpu.ir = 0x33C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E270, v1 as u16);
    16
}
/// $05EA3A: move.w (a4, d7.w), d0
pub fn i_05ea3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA3E;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05EA3E: rol.w #$1, d0
pub fn i_05ea3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA40;
    cpu.ir = 0xE358;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05EA40: subi.w #$15, d7
pub fn i_05ea40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA44;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x15, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05EA44: and.w d6, d7
pub fn i_05ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA46;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05EA46: eor.w d0, (a4, d7.w)
pub fn i_05ea46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA4A;
    cpu.ir = 0xB174;
    let ad1 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    14
}
/// $05EA4A: move.w (a4, d7.w), d0
pub fn i_05ea4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA4E;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05EA4E: and.w d5, d0
pub fn i_05ea4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA50;
    cpu.ir = 0xC045;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05EA50: rts
pub fn i_05ea50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Owner_GenCheck_05ea52  ($05EA52, 16 B, props/late_props_turrets_05e000.s)
/// $05EA52: movea.l $8(a6), a1
pub fn i_05ea52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA56;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05EA56: move.b $10(a6), d0
pub fn i_05ea56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA5A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05EA5A: cmp.b $10(a1), d0
pub fn i_05ea5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA5E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05EA5E: bcs.w $5ea68
pub fn i_05ea5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA62;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5EA68; return 10; }
    8
}
// ===== ClearXN_05ea62  ($05EA62, 6 B, thunks/ccr_helpers.c)
/// $05EA62: andi.b #$ee, ccr
pub fn i_05ea62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA66;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05EA66: rts
pub fn i_05ea66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05ea68  ($05EA68, 6 B, thunks/ccr_helpers.c)
/// $05EA68: ori.b #$11, ccr
pub fn i_05ea68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA6C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05EA6C: rts
pub fn i_05ea6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slot1008A0_SetHandlerIfA5_05ea6e  ($05EA6E, 24 B, props/late_props_turrets_05e000.s)
/// $05EA6E: cmpa.l #$ffffffff, a5
pub fn i_05ea6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA74;
    cpu.ir = 0xBBFC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[5];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05EA74: beq.w $5ea84
pub fn i_05ea74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA78;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5EA84; return 10; }
    8
}
/// $05EA78: lea.l $1008a0.l, a0
pub fn i_05ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1008A0;
    20
}
/// $05EA7E: move.l #$5ea96, (a0)
pub fn i_05ea7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA84;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x5EA96;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05EA84: rts
pub fn i_05ea84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slot1008A0_SetHandler400_05ea86  ($05EA86, 14 B, props/late_props_turrets_05e000.s)
/// $05EA86: lea.l $1008a0.l, a0
pub fn i_05ea86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1008A0;
    20
}
/// $05EA8C: move.l #$400, (a0)
pub fn i_05ea8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA92;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x400;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05EA92: rts
pub fn i_05ea92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0005EA94  ($05EA94, 2 B, thunks/trivial_rts.c)
/// $05EA94: rts
pub fn i_05ea94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_Clear106F42_05ea96  ($05EA96, 14 B, props/late_props_turrets_05e000.s)
/// $05EA96: lea.l $5ea9c(pc), a1
pub fn i_05ea96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5EA9C;
    16
}
/// $05EA9A: move.l a1, (a6)
pub fn i_05ea9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05EA9C: clr.w $106f42.l
pub fn i_05ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAA2;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F42) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F42, r2 as u16);
    16
}
/// $05EAA2: rts
pub fn i_05eaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slot1008A0_Alloc4AE_05eaa4  ($05EAA4, 18 B, props/late_props_turrets_05e000.s)
/// $05EAA4: move.l a6, -(a7)
pub fn i_05eaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAA6;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05EAA6: lea.l $1008a0.l, a6
pub fn i_05eaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAAC;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $05EAAC: jsr $4ae.l
pub fn i_05eaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EAB2);
    cpu.pc = 0x4AE;
    16
}
/// $05EAB2: movea.l (a7)+, a6
pub fn i_05eab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAB4;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05EAB4: rts
pub fn i_05eab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slot1008A0_Alloc6FE_05eab6  ($05EAB6, 18 B, props/late_props_turrets_05e000.s)
/// $05EAB6: move.l a6, -(a7)
pub fn i_05eab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAB8;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05EAB8: lea.l $1008a0.l, a6
pub fn i_05eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EABE;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $05EABE: jsr $6fe.l
pub fn i_05eabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EAC4);
    cpu.pc = 0x6FE;
    16
}
/// $05EAC4: movea.l (a7)+, a6
pub fn i_05eac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAC6;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05EAC6: rts
pub fn i_05eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Owner_GenCheck_05eac8  ($05EAC8, 16 B, props/late_props_turrets_05e000.s)
/// $05EAC8: movea.l $8(a6), a1
pub fn i_05eac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EACC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05EACC: move.b $10(a6), d0
pub fn i_05eacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAD0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05EAD0: cmp.b $10(a1), d0
pub fn i_05ead0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAD4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05EAD4: bcs.w $5eade
pub fn i_05ead4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAD8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5EADE; return 10; }
    8
}
// ===== ClearXN_05ead8  ($05EAD8, 6 B, thunks/ccr_helpers.c)
/// $05EAD8: andi.b #$ee, ccr
pub fn i_05ead8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EADC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05EADC: rts
pub fn i_05eadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EADE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05eade  ($05EADE, 6 B, thunks/ccr_helpers.c)
/// $05EADE: ori.b #$11, ccr
pub fn i_05eade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAE2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05EAE2: rts
pub fn i_05eae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_DrawMessageRow_05eae4  ($05EAE4, 88 B, props/late_props_turrets_05e000.s)
/// $05EAE4: lea.l $5eb98(pc), a0
pub fn i_05eae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAE8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5EB98;
    16
}
/// $05EAE8: jsr $2b58.l
pub fn i_05eae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EAEE);
    cpu.pc = 0x2B58;
    16
}
/// $05EAEE: movea.w #$704d, a1
pub fn i_05eaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAF2;
    cpu.ir = 0x327C;
    let v1: u32 = 0x704D;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $05EAF2: lea.l $5eb9e(pc), a0
pub fn i_05eaf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAF6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5EB9E;
    16
}
/// $05EAF6: asl.l #$2, d3
pub fn i_05eaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAF8;
    cpu.ir = 0xE583;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[3] = r;
    10
}
/// $05EAF8: adda.l d3, a0
pub fn i_05eaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAFA;
    cpu.ir = 0xD1C3;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $05EAFA: movea.l (a0), a0
pub fn i_05eafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAFC;
    cpu.ir = 0x2050;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[0] = v2;
    12
}
/// $05EAFC: move.w #$2, d1
pub fn i_05eafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB00;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB00: move.w #$2, d2
pub fn i_05eb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB04;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB04: move.w (a0), d0
pub fn i_05eb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB06;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05EB06: cmpi.w #$ffff, d0
pub fn i_05eb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB0A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $05EB0A: beq.w $5eb2c
pub fn i_05eb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5EB2C; return 10; }
    8
}
/// $05EB0E: ori.w #$1000, d0
pub fn i_05eb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB12;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x1000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05EB12: andi.w #$1fff, d0
pub fn i_05eb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB16;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1FFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05EB16: movem.l d0-d2/a0-a1, -(a7)
pub fn i_05eb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB1A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05EB1A: jsr $5da56.l
pub fn i_05eb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EB20);
    cpu.pc = 0x5DA56;
    16
}
/// $05EB20: movem.l (a7)+, d0-d2/a0-a1
pub fn i_05eb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB24;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05EB24: addq.l #$2, a0
pub fn i_05eb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB26;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $05EB26: adda.w #$40, a1
pub fn i_05eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB2A;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05EB2A: bra.b $5eb04
pub fn i_05eb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB2C;
    cpu.ir = 0x60D8;
    cpu.pc = 0x5EB04;
    10
}
/// $05EB2C: move.w #$1, d1
pub fn i_05eb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB30;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB30: move.w #$192, d2
pub fn i_05eb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB34;
    cpu.ir = 0x343C;
    let v1: u32 = 0x192;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB34: move.w #$1, d3
pub fn i_05eb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB38;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB38: move.w #$1, d4
pub fn i_05eb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB3C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_05eb3c  ($05EB3C, 8 B, thunks/jsr_abs_thunks.c)
/// $05EB3C: jsr $2c26.l
pub fn i_05eb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EB42);
    cpu.pc = 0x2C26;
    16
}
/// $05EB42: rts
pub fn i_05eb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_DrawTileRow_Step40_05eb44  ($05EB44, 42 B, props/late_props_turrets_05e000.s)
/// $05EB44: move.w #$2, d1
pub fn i_05eb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB48;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB48: move.w #$2, d2
pub fn i_05eb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB4C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB4C: move.w (a0), d0
pub fn i_05eb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB4E;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05EB4E: cmpi.w #$ffff, d0
pub fn i_05eb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB52;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $05EB52: beq.w $5eb6c
pub fn i_05eb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5EB6C; return 10; }
    8
}
/// $05EB56: movem.l d0-d2/a0-a1, -(a7)
pub fn i_05eb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB5A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05EB5A: jsr $5da56.l
pub fn i_05eb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EB60);
    cpu.pc = 0x5DA56;
    16
}
/// $05EB60: movem.l (a7)+, d0-d2/a0-a1
pub fn i_05eb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB64;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05EB64: addq.l #$2, a0
pub fn i_05eb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB66;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $05EB66: adda.w #$40, a1
pub fn i_05eb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB6A;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05EB6A: bra.b $5eb4c
pub fn i_05eb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB6C;
    cpu.ir = 0x60E0;
    cpu.pc = 0x5EB4C;
    10
}
/// $05EB6C: rts
pub fn i_05eb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_DrawTileRow_Step20_05eb6e  ($05EB6E, 42 B, props/late_props_turrets_05e000.s)
/// $05EB6E: move.w #$1, d1
pub fn i_05eb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB72;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB72: move.w #$2, d2
pub fn i_05eb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB76;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EB76: move.w (a0), d0
pub fn i_05eb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB78;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05EB78: cmpi.w #$ffff, d0
pub fn i_05eb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB7C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $05EB7C: beq.w $5eb96
pub fn i_05eb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5EB96; return 10; }
    8
}
/// $05EB80: movem.l d0-d2/a0-a1, -(a7)
pub fn i_05eb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB84;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05EB84: jsr $5da56.l
pub fn i_05eb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EB8A);
    cpu.pc = 0x5DA56;
    16
}
/// $05EB8A: movem.l (a7)+, d0-d2/a0-a1
pub fn i_05eb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB8E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05EB8E: addq.l #$2, a0
pub fn i_05eb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB90;
    cpu.ir = 0x5488;
    cpu.a[0] = cpu.a[0].wrapping_add(2);
    8
}
/// $05EB90: adda.w #$20, a1
pub fn i_05eb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB94;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x20;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05EB94: bra.b $5eb76
pub fn i_05eb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB96;
    cpu.ir = 0x60E0;
    cpu.pc = 0x5EB76;
    10
}
/// $05EB96: rts
pub fn i_05eb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== MessageTileRows_05eb98  ($05EB98, 1038 B, props/late_props_turrets_05e000.s)
/// $05EB98: btst.b #$0, d1
pub fn i_05eb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB9C;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05EB9C: dc.w $ffff
pub fn i_05eb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EB9E;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== JsrAbsThunk_05efa6  ($05EFA6, 8 B, thunks/jsr_abs_thunks.c)
/// $05EFA6: jsr $5d6c2.l
pub fn i_05efa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EFAC);
    cpu.pc = 0x5D6C2;
    16
}
/// $05EFAC: rts
pub fn i_05efac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Owner_GenCheck_05efae  ($05EFAE, 16 B, props/late_props_turrets_05e000.s)
/// $05EFAE: movea.l $8(a6), a1
pub fn i_05efae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFB2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05EFB2: move.b $10(a6), d0
pub fn i_05efb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFB6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05EFB6: cmp.b $10(a1), d0
pub fn i_05efb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFBA;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05EFBA: bcs.w $5efc4
pub fn i_05efba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFBE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5EFC4; return 10; }
    8
}
// ===== ClearXN_05efbe  ($05EFBE, 6 B, thunks/ccr_helpers.c)
/// $05EFBE: andi.b #$ee, ccr
pub fn i_05efbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFC2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05EFC2: rts
pub fn i_05efc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05efc4  ($05EFC4, 6 B, thunks/ccr_helpers.c)
/// $05EFC4: ori.b #$11, ccr
pub fn i_05efc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFC8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05EFC8: rts
pub fn i_05efc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugColl_Init_05efca  ($05EFCA, 64 B, props/late_props_turrets_05e000.s)
/// $05EFCA: btst.b #$1, $100001.l
pub fn i_05efca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFD2;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((1) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05EFD2: beq.w $5efe0
pub fn i_05efd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFD6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5EFE0; return 10; }
    8
}
/// $05EFD6: lea.l $5f212(pc), a1
pub fn i_05efd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F212;
    16
}
/// $05EFDA: jsr $4ae.l
pub fn i_05efda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EFE0);
    cpu.pc = 0x4AE;
    16
}
/// $05EFE0: move.w #$10, $22(a6)
pub fn i_05efe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFE6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05EFE6: move.w #$1e8, $24(a6)
pub fn i_05efe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFEC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05EFEC: move.w #$7043, $70(a6)
pub fn i_05efec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFF2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7043;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05EFF2: move.b #$ff, $32(a6)
pub fn i_05eff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFF8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05EFF8: move.b #$ff, $33(a6)
pub fn i_05eff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EFFE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05EFFE: move.b #$0, $3a(a6)
pub fn i_05effe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F004;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F004: move.w #$1, $38(a6)
pub fn i_05f004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F00A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== DebugColl_Idle_05f00a  ($05F00A, 52 B, props/late_props_turrets_05e000.s)
/// $05F00A: move.w #$ffff, $74(a6)
pub fn i_05f00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F010;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F010: lea.l $5f016(pc), a1
pub fn i_05f010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F014;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F016;
    16
}
/// $05F014: move.l a1, (a6)
pub fn i_05f014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F016;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F016: jsr $5cd06.l
pub fn i_05f016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F01C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F01C);
    cpu.pc = 0x5CD06;
    16
}
/// $05F01C: bcc.w $5f044
pub fn i_05f01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F020;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F044; return 10; }
    8
}
/// $05F020: jsr $5ce14.l
pub fn i_05f020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F026);
    cpu.pc = 0x5CE14;
    16
}
/// $05F026: bcc.w $5f030
pub fn i_05f026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F02A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F030; return 10; }
    8
}
/// $05F02A: lea.l $5f046(pc), a1
pub fn i_05f02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F02E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F046;
    16
}
/// $05F02E: move.l a1, (a6)
pub fn i_05f02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F030;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F030: bcs.w $5f044
pub fn i_05f030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F034;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5F044; return 10; }
    8
}
/// $05F034: jsr $5ce26.l
pub fn i_05f034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F03A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F03A);
    cpu.pc = 0x5CE26;
    16
}
/// $05F03A: bcc.w $5f044
pub fn i_05f03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F03E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F044; return 10; }
    8
}
// ===== SetTaskHandler_05f03e  ($05F03E, 8 B, kernel/task_handlers.c)
/// $05F03E: lea.l $5f0b0(pc), a1
pub fn i_05f03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F042;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F0B0;
    16
}
/// $05F042: move.l a1, (a6)
pub fn i_05f042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F044;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F044: rts
pub fn i_05f044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F046;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugColl_ShowLowNibble_05f046  ($05F046, 98 B, props/late_props_turrets_05e000.s)
/// $05F046: move.w #$ffff, $74(a6)
pub fn i_05f046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F04C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F04C: lea.l $5f052(pc), a1
pub fn i_05f04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F050;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F052;
    16
}
/// $05F050: move.l a1, (a6)
pub fn i_05f050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F052;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F052: clr.l d0
pub fn i_05f052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F054;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $05F054: clr.l d1
pub fn i_05f054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F056;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $05F056: move.w $74(a6), d0
pub fn i_05f056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F05A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F05A: move.l $106f50.l, d1
pub fn i_05f05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F060;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F50);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05F060: swap d1
pub fn i_05f060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F062;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05F062: cmp.w d1, d0
pub fn i_05f062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F064;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05F064: beq.w $5f07e
pub fn i_05f064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F068;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F07E; return 10; }
    8
}
/// $05F068: jsr $c004c2.l
pub fn i_05f068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F06E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F06E);
    cpu.pc = 0xC004C2;
    16
}
/// $05F06E: jsr $5f11a(pc)
pub fn i_05f06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F072;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F072);
    cpu.pc = 0x5F11A;
    16
}
/// $05F072: move.l $106f50.l, d0
pub fn i_05f072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F078;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05F078: swap d0
pub fn i_05f078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F07A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05F07A: move.w d0, $74(a6)
pub fn i_05f07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F07E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F07E: jsr $5cd06.l
pub fn i_05f07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F084;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F084);
    cpu.pc = 0x5CD06;
    16
}
/// $05F084: bcc.w $5f0ae
pub fn i_05f084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F088;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F0AE; return 10; }
    8
}
/// $05F088: jsr $5cce2.l
pub fn i_05f088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F08E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F08E);
    cpu.pc = 0x5CCE2;
    16
}
/// $05F08E: bcc.w $5f098
pub fn i_05f08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F092;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F098; return 10; }
    8
}
/// $05F092: lea.l $5f0b0(pc), a1
pub fn i_05f092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F096;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F0B0;
    16
}
/// $05F096: move.l a1, (a6)
pub fn i_05f096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F098;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F098: jsr $5ce38.l
pub fn i_05f098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F09E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F09E);
    cpu.pc = 0x5CE38;
    16
}
/// $05F09E: bcc.w $5f0ae
pub fn i_05f09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F0AE; return 10; }
    8
}
/// $05F0A2: jsr $c004c2.l
pub fn i_05f0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F0A8);
    cpu.pc = 0xC004C2;
    16
}
// ===== SetTaskHandler_05f0a8  ($05F0A8, 8 B, kernel/task_handlers.c)
/// $05F0A8: lea.l $5f00a(pc), a1
pub fn i_05f0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F00A;
    16
}
/// $05F0AC: move.l a1, (a6)
pub fn i_05f0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F0AE: rts
pub fn i_05f0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugColl_ShowHighNibble_05f0b0  ($05F0B0, 98 B, props/late_props_turrets_05e000.s)
/// $05F0B0: move.w #$ffff, $74(a6)
pub fn i_05f0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F0B6: lea.l $5f0bc(pc), a1
pub fn i_05f0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F0BC;
    16
}
/// $05F0BA: move.l a1, (a6)
pub fn i_05f0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F0BC: clr.l d0
pub fn i_05f0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0BE;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $05F0BE: clr.l d1
pub fn i_05f0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0C0;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $05F0C0: move.w $74(a6), d0
pub fn i_05f0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0C4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F0C4: move.l $106f50.l, d1
pub fn i_05f0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0CA;
    cpu.ir = 0x2239;
    let v2 = bus.read32(0x106F50);
    cpu.d[1] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05F0CA: swap d1
pub fn i_05f0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0CC;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05F0CC: cmp.w d1, d0
pub fn i_05f0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0CE;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05F0CE: beq.w $5f0e8
pub fn i_05f0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0D2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F0E8; return 10; }
    8
}
/// $05F0D2: jsr $c004c2.l
pub fn i_05f0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F0D8);
    cpu.pc = 0xC004C2;
    16
}
/// $05F0D8: jsr $5f18e(pc)
pub fn i_05f0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F0DC);
    cpu.pc = 0x5F18E;
    16
}
/// $05F0DC: move.l $106f50.l, d0
pub fn i_05f0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0E2;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05F0E2: swap d0
pub fn i_05f0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0E4;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05F0E4: move.w d0, $74(a6)
pub fn i_05f0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0E8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F0E8: jsr $5cd06.l
pub fn i_05f0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F0EE);
    cpu.pc = 0x5CD06;
    16
}
/// $05F0EE: bcc.w $5f118
pub fn i_05f0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F118; return 10; }
    8
}
/// $05F0F2: jsr $5ccd0.l
pub fn i_05f0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F0F8);
    cpu.pc = 0x5CCD0;
    16
}
/// $05F0F8: bcc.w $5f102
pub fn i_05f0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F0FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F102; return 10; }
    8
}
/// $05F0FC: lea.l $5f046(pc), a1
pub fn i_05f0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F100;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F046;
    16
}
/// $05F100: move.l a1, (a6)
pub fn i_05f100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F102;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F102: jsr $5ce38.l
pub fn i_05f102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F108);
    cpu.pc = 0x5CE38;
    16
}
/// $05F108: bcc.w $5f118
pub fn i_05f108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F10C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F118; return 10; }
    8
}
/// $05F10C: jsr $c004c2.l
pub fn i_05f10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F112;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F112);
    cpu.pc = 0xC004C2;
    16
}
// ===== SetTaskHandler_05f112  ($05F112, 8 B, kernel/task_handlers.c)
/// $05F112: lea.l $5f00a(pc), a1
pub fn i_05f112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F116;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F00A;
    16
}
/// $05F116: move.l a1, (a6)
pub fn i_05f116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F118;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F118: rts
pub fn i_05f118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F11A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugColl_DrawLowNibble_05f11a  ($05F11A, 116 B, props/late_props_turrets_05e000.s)
/// $05F11A: move.w $22(a6), d0
pub fn i_05f11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F11E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F11E: move.w $24(a6), d1
pub fn i_05f11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F122;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F122: jsr $43f02.l
pub fn i_05f122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F128;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F128);
    cpu.pc = 0x43F02;
    16
}
/// $05F128: lea.l $5f30a(pc), a0
pub fn i_05f128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F12C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5F30A;
    16
}
/// $05F12C: andi.l #$f, d0
pub fn i_05f12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F132;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05F132: cmpi.l #$0, d0
pub fn i_05f132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F138;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Long, false);
    8
}
/// $05F138: beq.w $5f154
pub fn i_05f138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F13C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F154; return 10; }
    8
}
/// $05F13C: asl.l #$1, d0
pub fn i_05f13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F13E;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $05F13E: adda.l d0, a0
pub fn i_05f13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F140;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $05F140: move.w (a0), d0
pub fn i_05f140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F142;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05F142: movea.w $70(a6), a1
pub fn i_05f142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F146;
    cpu.ir = 0x326E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = v3 as i16 as i32 as u32;
    12
}
/// $05F146: move.w #$1, d1
pub fn i_05f146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F14A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F14A: move.w #$1, d2
pub fn i_05f14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F14E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F14E: jsr $5da56.l
pub fn i_05f14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F154);
    cpu.pc = 0x5DA56;
    16
}
/// $05F154: addq.w #$1, $70(a6)
pub fn i_05f154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F158;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F158: subq.w #$8, $24(a6)
pub fn i_05f158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F15C;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F15C: cmpi.w #$118, $24(a6)
pub fn i_05f15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F162;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x118, Size::Word, false);
    16
}
/// $05F162: bcc.b $5f11a
pub fn i_05f162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F164;
    cpu.ir = 0x64B6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F11A; return 10; }
    8
}
/// $05F164: move.w #$1e8, $24(a6)
pub fn i_05f164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F16A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F16A: addq.w #$5, $70(a6)
pub fn i_05f16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F16E;
    cpu.ir = 0x5A6E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 5, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F16E: addq.w #$8, $22(a6)
pub fn i_05f16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F172;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F172: cmpi.w #$128, $22(a6)
pub fn i_05f172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F178;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x128, Size::Word, false);
    16
}
/// $05F178: bls.b $5f11a
pub fn i_05f178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F17A;
    cpu.ir = 0x63A0;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5F11A; return 10; }
    8
}
/// $05F17A: move.w #$10, $22(a6)
pub fn i_05f17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F180;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F180: move.w #$1e8, $24(a6)
pub fn i_05f180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F186;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F186: move.w #$7043, $70(a6)
pub fn i_05f186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F18C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7043;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F18C: rts
pub fn i_05f18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F18E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugColl_DrawHighNibble_05f18e  ($05F18E, 132 B, props/late_props_turrets_05e000.s)
/// $05F18E: move.w $22(a6), d0
pub fn i_05f18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F192;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F192: move.w $24(a6), d1
pub fn i_05f192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F196;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F196: jsr $43f02.l
pub fn i_05f196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F19C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F19C);
    cpu.pc = 0x43F02;
    16
}
/// $05F19C: lea.l $5f30a(pc), a0
pub fn i_05f19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1A0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5F30A;
    16
}
/// $05F1A0: andi.l #$ff, d0
pub fn i_05f1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1A6;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05F1A6: cmpi.l #$0, d0
pub fn i_05f1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1AC;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Long, false);
    8
}
/// $05F1AC: beq.w $5f1d8
pub fn i_05f1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F1D8; return 10; }
    8
}
/// $05F1B0: andi.l #$f0, d0
pub fn i_05f1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1B6;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF0) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05F1B6: asr.l #$4, d0
pub fn i_05f1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1B8;
    cpu.ir = 0xE880;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 0, false);
    cpu.d[0] = r;
    14
}
/// $05F1B8: asl.l #$1, d0
pub fn i_05f1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1BA;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $05F1BA: adda.l d0, a0
pub fn i_05f1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1BC;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $05F1BC: move.w (a0), d0
pub fn i_05f1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1BE;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05F1BE: andi.w #$fff, d0
pub fn i_05f1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1C2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05F1C2: addi.w #$5000, d0
pub fn i_05f1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1C6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x5000, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05F1C6: movea.w $70(a6), a1
pub fn i_05f1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1CA;
    cpu.ir = 0x326E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = v3 as i16 as i32 as u32;
    12
}
/// $05F1CA: move.w #$1, d1
pub fn i_05f1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F1CE: move.w #$1, d2
pub fn i_05f1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1D2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F1D2: jsr $5da56.l
pub fn i_05f1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F1D8);
    cpu.pc = 0x5DA56;
    16
}
/// $05F1D8: addq.w #$1, $70(a6)
pub fn i_05f1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1DC;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F1DC: subq.w #$8, $24(a6)
pub fn i_05f1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1E0;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F1E0: cmpi.w #$118, $24(a6)
pub fn i_05f1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x118, Size::Word, false);
    16
}
/// $05F1E6: bcc.b $5f18e
pub fn i_05f1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1E8;
    cpu.ir = 0x64A6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F18E; return 10; }
    8
}
/// $05F1E8: move.w #$1e8, $24(a6)
pub fn i_05f1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F1EE: addq.w #$5, $70(a6)
pub fn i_05f1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1F2;
    cpu.ir = 0x5A6E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 5, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F1F2: addq.w #$8, $22(a6)
pub fn i_05f1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1F6;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F1F6: cmpi.w #$128, $22(a6)
pub fn i_05f1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1FC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x128, Size::Word, false);
    16
}
/// $05F1FC: bls.b $5f18e
pub fn i_05f1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F1FE;
    cpu.ir = 0x6390;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5F18E; return 10; }
    8
}
/// $05F1FE: move.w #$10, $22(a6)
pub fn i_05f1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F204;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F204: move.w #$1e8, $24(a6)
pub fn i_05f204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F20A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F20A: move.w #$7043, $70(a6)
pub fn i_05f20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F210;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7043;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F210: rts
pub fn i_05f210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F212;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugColl_Cursor_05f212  ($05F212, 240 B, props/late_props_turrets_05e000.s)
/// $05F212: move.w #$e, d1
pub fn i_05f212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F216;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F216: jsr $236e.l
pub fn i_05f216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F21C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F21C);
    cpu.pc = 0x236E;
    16
}
/// $05F21C: move.w #$10, $22(a6)
pub fn i_05f21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F222;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F222: move.w #$1e8, $24(a6)
pub fn i_05f222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F228;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F228: move.b #$ff, $32(a6)
pub fn i_05f228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F22E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F22E: move.b #$ff, $33(a6)
pub fn i_05f22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F234;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F234: move.b #$0, $3a(a6)
pub fn i_05f234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F23A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F23A: move.w #$ffff, $38(a6)
pub fn i_05f23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F240;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F240: bset.b #$6, $12(a6)
pub fn i_05f240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F246;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05F246: lea.l $5f352(pc), a0
pub fn i_05f246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F24A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5F352;
    16
}
/// $05F24A: jsr $28cd4.l
pub fn i_05f24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F250);
    cpu.pc = 0x28CD4;
    16
}
/// $05F250: lea.l $5f256(pc), a1
pub fn i_05f250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F254;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F256;
    16
}
/// $05F254: move.l a1, (a6)
pub fn i_05f254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F256;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F256: jsr $5cd06.l
pub fn i_05f256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F25C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F25C);
    cpu.pc = 0x5CD06;
    16
}
/// $05F25C: bcc.w $5f298
pub fn i_05f25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F260;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F298; return 10; }
    8
}
/// $05F260: jsr $5cea4.l
pub fn i_05f260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F266);
    cpu.pc = 0x5CEA4;
    16
}
/// $05F266: bcc.w $5f26e
pub fn i_05f266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F26A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F26E; return 10; }
    8
}
/// $05F26A: addq.w #$1, $24(a6)
pub fn i_05f26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F26E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F26E: jsr $5ceb6.l
pub fn i_05f26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F274);
    cpu.pc = 0x5CEB6;
    16
}
/// $05F274: bcc.w $5f27c
pub fn i_05f274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F278;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F27C; return 10; }
    8
}
/// $05F278: subq.w #$1, $24(a6)
pub fn i_05f278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F27C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F27C: jsr $5cec8.l
pub fn i_05f27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F282;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F282);
    cpu.pc = 0x5CEC8;
    16
}
/// $05F282: bcc.w $5f28a
pub fn i_05f282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F286;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F28A; return 10; }
    8
}
/// $05F286: addq.w #$1, $22(a6)
pub fn i_05f286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F28A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F28A: jsr $5ceda.l
pub fn i_05f28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F290;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F290);
    cpu.pc = 0x5CEDA;
    16
}
/// $05F290: bcc.w $5f298
pub fn i_05f290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F294;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F298; return 10; }
    8
}
/// $05F294: subq.w #$1, $22(a6)
pub fn i_05f294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F298;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F298: jsr $28d70.l
pub fn i_05f298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F29E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F29E);
    cpu.pc = 0x28D70;
    16
}
/// $05F29E: move.w $22(a6), d0
pub fn i_05f29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2A2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F2A2: move.w $24(a6), d1
pub fn i_05f2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2A6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F2A6: movem.l d1, -(a7)
pub fn i_05f2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2AA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05F2AA: jsr $440bc.l
pub fn i_05f2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F2B0);
    cpu.pc = 0x440BC;
    16
}
/// $05F2B0: movea.w #$7425, a1
pub fn i_05f2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2B4;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7425;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $05F2B4: jsr $5d6c2.l
pub fn i_05f2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F2BA);
    cpu.pc = 0x5D6C2;
    16
}
/// $05F2BA: movem.l (a7)+, d0
pub fn i_05f2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2BE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05F2BE: movea.w #$7426, a1
pub fn i_05f2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2C2;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7426;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $05F2C2: jsr $5d6c2.l
pub fn i_05f2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F2C8);
    cpu.pc = 0x5D6C2;
    16
}
/// $05F2C8: move.w $22(a6), d0
pub fn i_05f2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2CC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F2CC: move.w $24(a6), d1
pub fn i_05f2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2D0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F2D0: jsr $43f02.l
pub fn i_05f2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F2D6);
    cpu.pc = 0x43F02;
    16
}
/// $05F2D6: andi.w #$ff, d0
pub fn i_05f2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2DA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05F2DA: movea.w #$7427, a1
pub fn i_05f2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2DE;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7427;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $05F2DE: jsr $5d6c2.l
pub fn i_05f2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F2E4);
    cpu.pc = 0x5D6C2;
    16
}
/// $05F2E4: lea.l $5f35e(pc), a0
pub fn i_05f2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2E8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5F35E;
    16
}
/// $05F2E8: jsr $5dd56.l
pub fn i_05f2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F2EE);
    cpu.pc = 0x5DD56;
    16
}
/// $05F2EE: bcc.w $5f2fa
pub fn i_05f2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F2FA; return 10; }
    8
}
/// $05F2F2: move.w #$ffff, d0
pub fn i_05f2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2F6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F2F6: bra.w $5f2fe
pub fn i_05f2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5F2FE;
    10
}
/// $05F2FA: move.w #$0, d0
pub fn i_05f2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F2FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F2FE: movea.w #$7428, a1
pub fn i_05f2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F302;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7428;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
// ===== JsrAbsThunk_05f302  ($05F302, 8 B, thunks/jsr_abs_thunks.c)
/// $05F302: jsr $5d6c2.l
pub fn i_05f302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F308);
    cpu.pc = 0x5D6C2;
    16
}
/// $05F308: rts
pub fn i_05f308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F30A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugColl_Tiles_05f30a  ($05F30A, 110 B, props/late_props_turrets_05e000.s)
/// $05F30A: move.l $31(a0, d2.w), d0
pub fn i_05f30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F30E;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x31).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05F30E: move.l $33(a2, d2.w), d0
pub fn i_05f30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F312;
    cpu.ir = 0x2032;
    let ad2 = cpu.a[2].wrapping_add(0x33).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05F312: move.l $35(a4, d2.w), d0
pub fn i_05f312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F316;
    cpu.ir = 0x2034;
    let ad2 = cpu.a[4].wrapping_add(0x35).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05F316: move.l $37(a6, d2.w), d0
pub fn i_05f316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F31A;
    cpu.ir = 0x2036;
    let ad2 = cpu.a[6].wrapping_add(0x37).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05F31A: move.l $2039.w, d0
pub fn i_05f31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F31E;
    cpu.ir = 0x2038;
    let v2 = bus.read32(0x2039);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05F31E: movea.l d1, a0
pub fn i_05f31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F320;
    cpu.ir = 0x2041;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F320: movea.l d2, a0
pub fn i_05f320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F322;
    cpu.ir = 0x2042;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F322: movea.l d3, a0
pub fn i_05f322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F324;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F324: movea.l d4, a0
pub fn i_05f324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F326;
    cpu.ir = 0x2044;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F326: movea.l d5, a0
pub fn i_05f326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F328;
    cpu.ir = 0x2045;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F328: movea.l d6, a0
pub fn i_05f328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F32A;
    cpu.ir = 0x2046;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F32A: movea.l d7, a0
pub fn i_05f32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F32C;
    cpu.ir = 0x2047;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F32C: movea.l a0, a0
pub fn i_05f32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F32E;
    cpu.ir = 0x2048;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F32E: movea.l a1, a0
pub fn i_05f32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F330;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F330: movea.l a2, a0
pub fn i_05f330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F332;
    cpu.ir = 0x204A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F332: movea.l a3, a0
pub fn i_05f332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F334;
    cpu.ir = 0x204B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F334: movea.l a4, a0
pub fn i_05f334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F336;
    cpu.ir = 0x204C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F336: movea.l a5, a0
pub fn i_05f336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F338;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F338: movea.l a6, a0
pub fn i_05f338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F33A;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F33A: movea.l a7, a0
pub fn i_05f33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F33C;
    cpu.ir = 0x204F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05F33C: movea.l (a0), a0
pub fn i_05f33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F33E;
    cpu.ir = 0x2050;
    let v2 = bus.read32(cpu.a[0]);
    cpu.a[0] = v2;
    12
}
/// $05F33E: movea.l (a1), a0
pub fn i_05f33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F340;
    cpu.ir = 0x2051;
    let v2 = bus.read32(cpu.a[1]);
    cpu.a[0] = v2;
    12
}
/// $05F340: movea.l (a2), a0
pub fn i_05f340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F342;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $05F342: movea.l (a3), a0
pub fn i_05f342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F344;
    cpu.ir = 0x2053;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[0] = v2;
    12
}
/// $05F344: movea.l (a4), a0
pub fn i_05f344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F346;
    cpu.ir = 0x2054;
    let v2 = bus.read32(cpu.a[4]);
    cpu.a[0] = v2;
    12
}
/// $05F346: movea.l (a5), a0
pub fn i_05f346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F348;
    cpu.ir = 0x2055;
    let v2 = bus.read32(cpu.a[5]);
    cpu.a[0] = v2;
    12
}
/// $05F348: movea.l (a6), a0
pub fn i_05f348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F34A;
    cpu.ir = 0x2056;
    let v2 = bus.read32(cpu.a[6]);
    cpu.a[0] = v2;
    12
}
/// $05F34A: movea.l (a7), a0
pub fn i_05f34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F34C;
    cpu.ir = 0x2057;
    let v2 = bus.read32(cpu.a[7]);
    cpu.a[0] = v2;
    12
}
/// $05F34C: movea.l (a0)+, a0
pub fn i_05f34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F34E;
    cpu.ir = 0x2058;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $05F34E: movea.l (a1)+, a0
pub fn i_05f34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F350;
    cpu.ir = 0x2059;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $05F350: movea.l (a2)+, a0
pub fn i_05f350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F352;
    cpu.ir = 0x205A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    12
}
/// $05F352: ori.b #$8, d1
pub fn i_05f352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F356;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05F356: ori.b #$8, -(a4)
pub fn i_05f356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F35A;
    cpu.ir = 0x24;
    let ad1 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $05F35A: dc.w $ffff
pub fn i_05f35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F35C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== ClearXN_05f378  ($05F378, 6 B, thunks/ccr_helpers.c)
/// $05F378: andi.b #$ee, ccr
pub fn i_05f378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F37C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05F37C: rts
pub fn i_05f37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F37E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05f37e  ($05F37E, 6 B, thunks/ccr_helpers.c)
/// $05F37E: ori.b #$11, ccr
pub fn i_05f37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F382;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05F382: rts
pub fn i_05f382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F384;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Init_05f384  ($05F384, 102 B, props/late_props_turrets_05e000.s)
/// $05F384: move.b #$88, $20(a6)
pub fn i_05f384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F38A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x88;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F38A: move.w #$27, d1
pub fn i_05f38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F38E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F38E: jsr $236e.l
pub fn i_05f38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F394);
    cpu.pc = 0x236E;
    16
}
/// $05F394: move.b #$ff, $32(a6)
pub fn i_05f394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F39A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F39A: move.b #$ff, $33(a6)
pub fn i_05f39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F3A0: move.w #$4000, $38(a6)
pub fn i_05f3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3A6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F3A6: addi.w #$1, $38(a6)
pub fn i_05f3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3AC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05F3AC: jsr $267e2.l
pub fn i_05f3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F3B2);
    cpu.pc = 0x267E2;
    16
}
/// $05F3B2: lea.l $2c0be2.l, a0
pub fn i_05f3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0BE2;
    20
}
/// $05F3B8: jsr $28cd4.l
pub fn i_05f3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F3BE);
    cpu.pc = 0x28CD4;
    16
}
/// $05F3BE: move.b #$0, $71(a6)
pub fn i_05f3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F3C4: lea.l $5f3ca(pc), a1
pub fn i_05f3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F3CA;
    16
}
/// $05F3C8: move.l a1, (a6)
pub fn i_05f3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F3CA: jsr $2783a.l
pub fn i_05f3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F3D0);
    cpu.pc = 0x2783A;
    16
}
/// $05F3D0: cmpi.w #$160, $22(a6)
pub fn i_05f3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3D6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $05F3D6: bgt.w $5f3f0
pub fn i_05f3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3DA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5F3F0; return 10; }
    8
}
/// $05F3DA: lea.l $5f3f2(pc), a1
pub fn i_05f3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F3F2;
    16
}
/// $05F3DE: move.l a1, (a6)
pub fn i_05f3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F3E0: cmpi.b #$88, $20(a6)
pub fn i_05f3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3E6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $05F3E6: bne.w $5f3f0
pub fn i_05f3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5F3F0; return 10; }
    8
}
// ===== SetTaskHandler_05f3ea  ($05F3EA, 8 B, kernel/task_handlers.c)
/// $05F3EA: lea.l $5f482(pc), a1
pub fn i_05f3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F482;
    16
}
/// $05F3EE: move.l a1, (a6)
pub fn i_05f3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F3F0: rts
pub fn i_05f3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Watch_05f3f2  ($05F3F2, 144 B, props/late_props_turrets_05e000.s)
/// $05F3F2: move.b #$0, $70(a6)
pub fn i_05f3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F3F8: lea.l $5f3fe(pc), a1
pub fn i_05f3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F3FE;
    16
}
/// $05F3FC: move.l a1, (a6)
pub fn i_05f3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F3FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F3FE: jsr $2783a.l
pub fn i_05f3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F404;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F404);
    cpu.pc = 0x2783A;
    16
}
/// $05F404: jsr $28d70.l
pub fn i_05f404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F40A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F40A);
    cpu.pc = 0x28D70;
    16
}
/// $05F40A: jsr $2870a.l
pub fn i_05f40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F410);
    cpu.pc = 0x2870A;
    16
}
/// $05F410: bclr.b #$3, $13(a6)
pub fn i_05f410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F416;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05F416: lea.l $2c0ef2.l, a0
pub fn i_05f416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F41C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0EF2;
    20
}
/// $05F41C: btst.b #$0, $3a(a6)
pub fn i_05f41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F422;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05F422: beq.w $5f42c
pub fn i_05f422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F426;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F42C; return 10; }
    8
}
/// $05F426: lea.l $2c0efc.l, a0
pub fn i_05f426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F42C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0EFC;
    20
}
/// $05F42C: jsr $5e086.l
pub fn i_05f42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F432);
    cpu.pc = 0x5E086;
    16
}
/// $05F432: bcs.w $5f45e
pub fn i_05f432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F436;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5F45E; return 10; }
    8
}
/// $05F436: cmpi.b #$0, $71(a6)
pub fn i_05f436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F43C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $05F43C: bgt.w $5f45a
pub fn i_05f43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F440;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5F45A; return 10; }
    8
}
/// $05F440: lea.l $5f6c4(pc), a1
pub fn i_05f440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F444;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F6C4;
    16
}
/// $05F444: jsr $4ae.l
pub fn i_05f444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F44A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F44A);
    cpu.pc = 0x4AE;
    16
}
/// $05F44A: jsr $5dd02.l
pub fn i_05f44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F450);
    cpu.pc = 0x5DD02;
    16
}
/// $05F450: lea.l $5f482(pc), a1
pub fn i_05f450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F454;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F482;
    16
}
/// $05F454: move.l a1, (a6)
pub fn i_05f454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F456;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F456: bra.w $5f45e
pub fn i_05f456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F45A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5F45E;
    10
}
/// $05F45A: subq.b #$1, $71(a6)
pub fn i_05f45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F45E;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x71);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05F45E: movea.l $c(a6), a0
pub fn i_05f45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F462;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F462: cmpi.b #$ff, $21(a0)
pub fn i_05f462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F468;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05F468: bne.w $5f472
pub fn i_05f468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F46C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5F472; return 10; }
    8
}
/// $05F46C: lea.l $5f508(pc), a1
pub fn i_05f46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F470;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F508;
    16
}
/// $05F470: move.l a1, (a6)
pub fn i_05f470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F472;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F472: jsr $5f9ac(pc)
pub fn i_05f472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F476;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F476);
    cpu.pc = 0x5F9AC;
    16
}
/// $05F476: bcc.w $5f480
pub fn i_05f476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F47A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F480; return 10; }
    8
}
/// $05F47A: jmp $518.l
pub fn i_05f47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F480;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F480: rts
pub fn i_05f480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Fire_05f482  ($05F482, 134 B, props/late_props_turrets_05e000.s)
/// $05F482: move.b #$0, $21(a6)
pub fn i_05f482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F488;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F488: lea.l $5f48e(pc), a1
pub fn i_05f488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F48C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F48E;
    16
}
/// $05F48C: move.l a1, (a6)
pub fn i_05f48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F48E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F48E: jsr $2783a.l
pub fn i_05f48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F494);
    cpu.pc = 0x2783A;
    16
}
/// $05F494: jsr $28d70.l
pub fn i_05f494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F49A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F49A);
    cpu.pc = 0x28D70;
    16
}
/// $05F49A: jsr $2870a.l
pub fn i_05f49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F4A0);
    cpu.pc = 0x2870A;
    16
}
/// $05F4A0: bclr.b #$3, $13(a6)
pub fn i_05f4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4A6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05F4A6: cmpi.b #$88, $20(a6)
pub fn i_05f4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4AC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $05F4AC: bne.w $5f4be
pub fn i_05f4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4B0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5F4BE; return 10; }
    8
}
/// $05F4B0: movea.l $c(a6), a0
pub fn i_05f4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4B4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F4B4: cmpi.b #$ff, $21(a0)
pub fn i_05f4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4BA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05F4BA: beq.w $5f4c8
pub fn i_05f4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F4C8; return 10; }
    8
}
/// $05F4BE: cmpi.b #$ff, $21(a6)
pub fn i_05f4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05F4C4: bne.w $5f4ce
pub fn i_05f4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5F4CE; return 10; }
    8
}
/// $05F4C8: lea.l $5f508(pc), a1
pub fn i_05f4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F508;
    16
}
/// $05F4CC: move.l a1, (a6)
pub fn i_05f4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F4CE: cmpi.b #$ee, $21(a6)
pub fn i_05f4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xEE, Size::Byte, false);
    16
}
/// $05F4D4: bne.w $5f4e4
pub fn i_05f4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5F4E4; return 10; }
    8
}
/// $05F4D8: move.b #$1e, $71(a6)
pub fn i_05f4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F4DE: lea.l $5f3f2(pc), a1
pub fn i_05f4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F3F2;
    16
}
/// $05F4E2: move.l a1, (a6)
pub fn i_05f4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F4E4: movea.l $c(a6), a0
pub fn i_05f4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F4E8: cmpi.b #$ff, $21(a0)
pub fn i_05f4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4EE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05F4EE: bne.w $5f4f8
pub fn i_05f4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5F4F8; return 10; }
    8
}
/// $05F4F2: lea.l $5f508(pc), a1
pub fn i_05f4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F508;
    16
}
/// $05F4F6: move.l a1, (a6)
pub fn i_05f4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F4F8: jsr $5f9ac(pc)
pub fn i_05f4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F4FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F4FC);
    cpu.pc = 0x5F9AC;
    16
}
/// $05F4FC: bcc.w $5f506
pub fn i_05f4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F500;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F506; return 10; }
    8
}
/// $05F500: jmp $518.l
pub fn i_05f500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F506;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F506: rts
pub fn i_05f506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F508;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Die_05f508  ($05F508, 162 B, props/late_props_turrets_05e000.s)
/// $05F508: lea.l $5f668(pc), a1
pub fn i_05f508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F50C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F668;
    16
}
/// $05F50C: jsr $4ae.l
pub fn i_05f50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F512);
    cpu.pc = 0x4AE;
    16
}
/// $05F512: jsr $5dd02.l
pub fn i_05f512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F518;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F518);
    cpu.pc = 0x5DD02;
    16
}
/// $05F518: jsr $267e2.l
pub fn i_05f518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F51E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F51E);
    cpu.pc = 0x267E2;
    16
}
/// $05F51E: move.w #$0, $28(a6)
pub fn i_05f51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F524;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F524: move.w #$0, $2a(a6)
pub fn i_05f524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F52A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F52A: move.w #$0, $2c(a6)
pub fn i_05f52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F530;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F530: move.w #$ff40, $2e(a6)
pub fn i_05f530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F536;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F536: move.b #$ff, $70(a6)
pub fn i_05f536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F53C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F53C: lea.l $2c0c5e.l, a0
pub fn i_05f53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F542;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0C5E;
    20
}
/// $05F542: jsr $28cd4.l
pub fn i_05f542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F548;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F548);
    cpu.pc = 0x28CD4;
    16
}
/// $05F548: lea.l $2c0dba.l, a0
pub fn i_05f548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F54E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0DBA;
    20
}
/// $05F54E: move.l a0, $4c(a6)
pub fn i_05f54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F552;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05F552: jsr $283ca.l
pub fn i_05f552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F558;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F558);
    cpu.pc = 0x283CA;
    16
}
/// $05F558: lea.l $5f55e(pc), a1
pub fn i_05f558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F55C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F55E;
    16
}
/// $05F55C: move.l a1, (a6)
pub fn i_05f55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F55E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F55E: jsr $27d50.l
pub fn i_05f55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F564);
    cpu.pc = 0x27D50;
    16
}
/// $05F564: bcc.w $5f56e
pub fn i_05f564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F568;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F56E; return 10; }
    8
}
/// $05F568: lea.l $5f5fa(pc), a1
pub fn i_05f568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F56C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F5FA;
    16
}
/// $05F56C: move.l a1, (a6)
pub fn i_05f56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F56E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F56E: jsr $28d70.l
pub fn i_05f56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F574;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F574);
    cpu.pc = 0x28D70;
    16
}
/// $05F574: jsr $5f91e(pc)
pub fn i_05f574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F578;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F578);
    cpu.pc = 0x5F91E;
    16
}
/// $05F578: jsr $2870a.l
pub fn i_05f578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F57E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F57E);
    cpu.pc = 0x2870A;
    16
}
/// $05F57E: bclr.b #$3, $13(a6)
pub fn i_05f57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F584;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05F584: jsr $283d8.l
pub fn i_05f584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F58A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F58A);
    cpu.pc = 0x283D8;
    16
}
/// $05F58A: btst.b #$1, $13(a6)
pub fn i_05f58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F590;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05F590: beq.w $5f59a
pub fn i_05f590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F594;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F59A; return 10; }
    8
}
/// $05F594: lea.l $5f5aa(pc), a1
pub fn i_05f594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F598;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F5AA;
    16
}
/// $05F598: move.l a1, (a6)
pub fn i_05f598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F59A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F59A: jsr $5f9ac(pc)
pub fn i_05f59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F59E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F59E);
    cpu.pc = 0x5F9AC;
    16
}
/// $05F59E: bcc.w $5f5a8
pub fn i_05f59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F5A8; return 10; }
    8
}
/// $05F5A2: jmp $518.l
pub fn i_05f5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F5A8: rts
pub fn i_05f5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Fall_05f5aa  ($05F5AA, 80 B, props/late_props_turrets_05e000.s)
/// $05F5AA: move.w #$d000, $38(a6)
pub fn i_05f5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F5B0: move.w $2e(a6), d0
pub fn i_05f5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F5B4: asr.w #$1, d0
pub fn i_05f5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5B6;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05F5B6: move.w d0, $2e(a6)
pub fn i_05f5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5BA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F5BA: move.w #$0, $2a(a6)
pub fn i_05f5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F5C0: move.l #$ffffffff, $4c(a6)
pub fn i_05f5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5C8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05F5C8: lea.l $2c0cc4.l, a0
pub fn i_05f5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0CC4;
    20
}
/// $05F5CE: jsr $28cd4.l
pub fn i_05f5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F5D4);
    cpu.pc = 0x28CD4;
    16
}
/// $05F5D4: lea.l $5f5da(pc), a1
pub fn i_05f5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F5DA;
    16
}
/// $05F5D8: move.l a1, (a6)
pub fn i_05f5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F5DA: jsr $27d50.l
pub fn i_05f5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F5E0);
    cpu.pc = 0x27D50;
    16
}
/// $05F5E0: jsr $28d70.l
pub fn i_05f5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F5E6);
    cpu.pc = 0x28D70;
    16
}
/// $05F5E6: bcs.w $5f5f2
pub fn i_05f5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5EA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5F5F2; return 10; }
    8
}
/// $05F5EA: jsr $5f9ac(pc)
pub fn i_05f5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F5EE);
    cpu.pc = 0x5F9AC;
    16
}
/// $05F5EE: bcc.w $5f5f8
pub fn i_05f5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F5F8; return 10; }
    8
}
/// $05F5F2: jmp $518.l
pub fn i_05f5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5F8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F5F8: rts
pub fn i_05f5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F5FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Flee_05f5fa  ($05F5FA, 110 B, props/late_props_turrets_05e000.s)
/// $05F5FA: jsr $2783a.l
pub fn i_05f5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F600);
    cpu.pc = 0x2783A;
    16
}
/// $05F600: move.w #$1053, d0
pub fn i_05f600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F604;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1053;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F604: jsr $2352.l
pub fn i_05f604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F60A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F60A);
    cpu.pc = 0x2352;
    16
}
/// $05F60A: move.w #$8000, $38(a6)
pub fn i_05f60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F610;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F610: lea.l $789f0.l, a1
pub fn i_05f610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F616;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x789F0;
    20
}
/// $05F616: jsr $4ae.l
pub fn i_05f616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F61C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F61C);
    cpu.pc = 0x4AE;
    16
}
/// $05F61C: jsr $5dd02.l
pub fn i_05f61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F622;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F622);
    cpu.pc = 0x5DD02;
    16
}
/// $05F622: addi.w #$1, $38(a0)
pub fn i_05f622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F628;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05F628: lea.l $2c0f06.l, a1
pub fn i_05f628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F62E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C0F06;
    20
}
/// $05F62E: jsr $43fac.l
pub fn i_05f62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F634);
    cpu.pc = 0x43FAC;
    16
}
/// $05F634: jsr $267e2.l
pub fn i_05f634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F63A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F63A);
    cpu.pc = 0x267E2;
    16
}
/// $05F63A: lea.l $2c0c98.l, a0
pub fn i_05f63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F640;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0C98;
    20
}
/// $05F640: jsr $28cd4.l
pub fn i_05f640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F646);
    cpu.pc = 0x28CD4;
    16
}
/// $05F646: lea.l $5f64c(pc), a1
pub fn i_05f646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F64A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F64C;
    16
}
/// $05F64A: move.l a1, (a6)
pub fn i_05f64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F64C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F64C: jsr $27afc.l
pub fn i_05f64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F652);
    cpu.pc = 0x27AFC;
    16
}
/// $05F652: jsr $28d70.l
pub fn i_05f652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F658);
    cpu.pc = 0x28D70;
    16
}
/// $05F658: jsr $5f9ac(pc)
pub fn i_05f658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F65C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F65C);
    cpu.pc = 0x5F9AC;
    16
}
/// $05F65C: bcc.w $5f666
pub fn i_05f65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F660;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F666; return 10; }
    8
}
/// $05F660: jmp $518.l
pub fn i_05f660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F666;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F666: rts
pub fn i_05f666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F668;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_InitB_05f668  ($05F668, 92 B, props/late_props_turrets_05e000.s)
/// $05F668: move.w #$27, d1
pub fn i_05f668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F66C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F66C: jsr $236e.l
pub fn i_05f66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F672;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F672);
    cpu.pc = 0x236E;
    16
}
/// $05F672: move.b #$ff, $32(a6)
pub fn i_05f672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F678;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F678: move.b #$ff, $33(a6)
pub fn i_05f678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F67E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F67E: jsr $267e2.l
pub fn i_05f67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F684;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F684);
    cpu.pc = 0x267E2;
    16
}
/// $05F684: lea.l $2c0daa.l, a0
pub fn i_05f684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F68A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0DAA;
    20
}
/// $05F68A: jsr $28cd4.l
pub fn i_05f68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F690;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F690);
    cpu.pc = 0x28CD4;
    16
}
/// $05F690: lea.l $5f696(pc), a1
pub fn i_05f690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F694;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F696;
    16
}
/// $05F694: move.l a1, (a6)
pub fn i_05f694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F696;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F696: jsr $2783a.l
pub fn i_05f696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F69C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F69C);
    cpu.pc = 0x2783A;
    16
}
/// $05F69C: jsr $28d70.l
pub fn i_05f69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F6A2);
    cpu.pc = 0x28D70;
    16
}
/// $05F6A2: movea.l $c(a6), a0
pub fn i_05f6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6A6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F6A6: movea.l $c(a0), a0
pub fn i_05f6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6AA;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F6AA: cmpi.b #$ff, $21(a0)
pub fn i_05f6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6B0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05F6B0: beq.w $5f6bc
pub fn i_05f6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F6BC; return 10; }
    8
}
/// $05F6B4: jsr $5f9c8(pc)
pub fn i_05f6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F6B8);
    cpu.pc = 0x5F9C8;
    16
}
/// $05F6B8: bcc.w $5f6c2
pub fn i_05f6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F6C2; return 10; }
    8
}
/// $05F6BC: jmp $518.l
pub fn i_05f6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6C2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F6C2: rts
pub fn i_05f6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Grenade_05f6c4  ($05F6C4, 142 B, props/late_props_turrets_05e000.s)
/// $05F6C4: move.w #$1d, d1
pub fn i_05f6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F6C8: jsr $236e.l
pub fn i_05f6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F6CE);
    cpu.pc = 0x236E;
    16
}
/// $05F6CE: move.w #$4a, d0
pub fn i_05f6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6D2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F6D2: btst.b #$0, $3a(a6)
pub fn i_05f6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6D8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05F6D8: beq.w $5f6e0
pub fn i_05f6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F6E0; return 10; }
    8
}
/// $05F6DC: move.w #$ffc4, d0
pub fn i_05f6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFC4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F6E0: add.w d0, $22(a6)
pub fn i_05f6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6E4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F6E4: addi.w #$30, $24(a6)
pub fn i_05f6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6EA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05F6EA: jsr $267e2.l
pub fn i_05f6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F6F0);
    cpu.pc = 0x267E2;
    16
}
/// $05F6F0: lea.l $2c09ac.l, a0
pub fn i_05f6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C09AC;
    20
}
/// $05F6F6: jsr $28cd4.l
pub fn i_05f6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F6FC);
    cpu.pc = 0x28CD4;
    16
}
/// $05F6FC: move.w #$ff80, $2a(a6)
pub fn i_05f6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F702;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F702: subq.w #$1, $24(a6)
pub fn i_05f702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F706;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F706: move.w $22(a6), d1
pub fn i_05f706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F70A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F70A: move.w $24(a6), d2
pub fn i_05f70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F70E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F70E: jsr $280c6.l
pub fn i_05f70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F714);
    cpu.pc = 0x280C6;
    16
}
/// $05F714: bcc.b $5f702
pub fn i_05f714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F716;
    cpu.ir = 0x64EC;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F702; return 10; }
    8
}
/// $05F716: move.b #$0, $70(a6)
pub fn i_05f716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F71C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F71C: lea.l $5f722(pc), a1
pub fn i_05f71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F720;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F722;
    16
}
/// $05F720: move.l a1, (a6)
pub fn i_05f720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F722;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F722: jsr $2783a.l
pub fn i_05f722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F728);
    cpu.pc = 0x2783A;
    16
}
/// $05F728: jsr $28d70.l
pub fn i_05f728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F72E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F72E);
    cpu.pc = 0x28D70;
    16
}
/// $05F72E: bcc.w $5f738
pub fn i_05f72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F732;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F738; return 10; }
    8
}
/// $05F732: lea.l $5f752(pc), a1
pub fn i_05f732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F736;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F752;
    16
}
/// $05F736: move.l a1, (a6)
pub fn i_05f736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F738;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F738: jsr $5f990(pc)
pub fn i_05f738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F73C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F73C);
    cpu.pc = 0x5F990;
    16
}
/// $05F73C: bcc.w $5f750
pub fn i_05f73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F740;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F750; return 10; }
    8
}
/// $05F740: movea.l $c(a6), a0
pub fn i_05f740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F744;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F744: move.b #$ee, $21(a0)
pub fn i_05f744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F74A;
    cpu.ir = 0x117C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F74A: jmp $518.l
pub fn i_05f74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F750;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F750: rts
pub fn i_05f750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F752;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_GrenadeBurst_05f752  ($05F752, 162 B, props/late_props_turrets_05e000.s)
/// $05F752: jsr $267e2.l
pub fn i_05f752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F758);
    cpu.pc = 0x267E2;
    16
}
/// $05F758: lea.l $29b744.l, a0
pub fn i_05f758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F75E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $05F75E: jsr $28cd4.l
pub fn i_05f75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F764);
    cpu.pc = 0x28CD4;
    16
}
/// $05F764: move.w #$fe00, d0
pub fn i_05f764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F768;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F768: btst.b #$0, $3a(a6)
pub fn i_05f768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F76E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05F76E: beq.w $5f774
pub fn i_05f76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F772;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F774; return 10; }
    8
}
/// $05F772: neg.w d0
pub fn i_05f772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F774;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05F774: move.w d0, $28(a6)
pub fn i_05f774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F778;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05F778: lea.l $5f77e(pc), a1
pub fn i_05f778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F77C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F77E;
    16
}
/// $05F77C: move.l a1, (a6)
pub fn i_05f77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F77E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F77E: jsr $27cee.l
pub fn i_05f77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F784;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F784);
    cpu.pc = 0x27CEE;
    16
}
/// $05F784: jsr $28d70.l
pub fn i_05f784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F78A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F78A);
    cpu.pc = 0x28D70;
    16
}
/// $05F78A: jsr $2870a.l
pub fn i_05f78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F790);
    cpu.pc = 0x2870A;
    16
}
/// $05F790: bcc.w $5f79e
pub fn i_05f790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F794;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F79E; return 10; }
    8
}
/// $05F794: movea.l $c(a6), a0
pub fn i_05f794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F798;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F798: move.b #$ee, $21(a0)
pub fn i_05f798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F79E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F79E: movea.l $c(a6), a0
pub fn i_05f79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7A2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F7A2: cmpi.b #$ff, $70(a0)
pub fn i_05f7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7A8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05F7A8: bne.w $5f7c6
pub fn i_05f7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7AC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5F7C6; return 10; }
    8
}
/// $05F7AC: lea.l $58f82.l, a1
pub fn i_05f7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7B2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $05F7B2: move.l a1, (a6)
pub fn i_05f7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F7B4: lea.l $776e2.l, a1
pub fn i_05f7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $05F7BA: jsr $4ae.l
pub fn i_05f7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F7C0);
    cpu.pc = 0x4AE;
    16
}
/// $05F7C0: jsr $5dd02.l
pub fn i_05f7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F7C6);
    cpu.pc = 0x5DD02;
    16
}
/// $05F7C6: jsr $5f94c(pc)
pub fn i_05f7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F7CA);
    cpu.pc = 0x5F94C;
    16
}
/// $05F7CA: bcc.w $5f7d4
pub fn i_05f7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F7D4; return 10; }
    8
}
/// $05F7CE: lea.l $5f7f4(pc), a1
pub fn i_05f7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F7F4;
    16
}
/// $05F7D2: move.l a1, (a6)
pub fn i_05f7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F7D4: jsr $49fd0.l
pub fn i_05f7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F7DA);
    cpu.pc = 0x49FD0;
    16
}
/// $05F7DA: jsr $5f990(pc)
pub fn i_05f7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F7DE);
    cpu.pc = 0x5F990;
    16
}
/// $05F7DE: bcc.w $5f7f2
pub fn i_05f7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F7F2; return 10; }
    8
}
/// $05F7E2: movea.l $c(a6), a0
pub fn i_05f7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7E6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F7E6: move.b #$ee, $21(a0)
pub fn i_05f7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7EC;
    cpu.ir = 0x117C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F7EC: jmp $518.l
pub fn i_05f7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7F2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F7F2: rts
pub fn i_05f7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Muzzle_05f7f4  ($05F7F4, 180 B, props/late_props_turrets_05e000.s)
/// $05F7F4: jsr $5e4dc.l
pub fn i_05f7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F7FA);
    cpu.pc = 0x5E4DC;
    16
}
/// $05F7FA: move.w #$18, d0
pub fn i_05f7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F7FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05F7FE: btst.b #$0, $3a(a6)
pub fn i_05f7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F804;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05F804: beq.w $5f80a
pub fn i_05f804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F808;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F80A; return 10; }
    8
}
/// $05F808: neg.w d0
pub fn i_05f808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F80A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05F80A: add.w d0, $22(a6)
pub fn i_05f80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F80E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05F80E: addi.w #$10, $24(a6)
pub fn i_05f80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F814;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05F814: jsr $267e2.l
pub fn i_05f814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F81A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F81A);
    cpu.pc = 0x267E2;
    16
}
/// $05F81A: lea.l $2c0a3a.l, a0
pub fn i_05f81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F820;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0A3A;
    20
}
/// $05F820: jsr $28cd4.l
pub fn i_05f820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F826);
    cpu.pc = 0x28CD4;
    16
}
/// $05F826: lea.l $5f82c(pc), a1
pub fn i_05f826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F82A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F82C;
    16
}
/// $05F82A: move.l a1, (a6)
pub fn i_05f82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F82C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F82C: jsr $2783a.l
pub fn i_05f82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F832;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F832);
    cpu.pc = 0x2783A;
    16
}
/// $05F832: jsr $28d70.l
pub fn i_05f832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F838);
    cpu.pc = 0x28D70;
    16
}
/// $05F838: bcc.w $5f842
pub fn i_05f838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F83C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F842; return 10; }
    8
}
/// $05F83C: lea.l $5f8a8(pc), a1
pub fn i_05f83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F840;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F8A8;
    16
}
/// $05F840: move.l a1, (a6)
pub fn i_05f840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F842;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F842: jsr $2870a.l
pub fn i_05f842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F848;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F848);
    cpu.pc = 0x2870A;
    16
}
/// $05F848: bcc.w $5f856
pub fn i_05f848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F84C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F856; return 10; }
    8
}
/// $05F84C: movea.l $c(a6), a0
pub fn i_05f84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F850;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F850: move.b #$ee, $21(a0)
pub fn i_05f850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F856;
    cpu.ir = 0x117C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F856: movea.l $c(a6), a0
pub fn i_05f856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F85A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F85A: cmpi.b #$ff, $70(a0)
pub fn i_05f85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F860;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05F860: bne.w $5f888
pub fn i_05f860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F864;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5F888; return 10; }
    8
}
/// $05F864: cmpi.b #$ff, $70(a6)
pub fn i_05f864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F86A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05F86A: beq.w $5f888
pub fn i_05f86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F86E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F888; return 10; }
    8
}
/// $05F86E: lea.l $58f82.l, a1
pub fn i_05f86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F874;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $05F874: move.l a1, (a6)
pub fn i_05f874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F876;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F876: lea.l $776e2.l, a1
pub fn i_05f876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F87C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $05F87C: jsr $4ae.l
pub fn i_05f87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F882);
    cpu.pc = 0x4AE;
    16
}
/// $05F882: jsr $5dd02.l
pub fn i_05f882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F888);
    cpu.pc = 0x5DD02;
    16
}
/// $05F888: jsr $49fd0.l
pub fn i_05f888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F88E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F88E);
    cpu.pc = 0x49FD0;
    16
}
/// $05F88E: jsr $5f990(pc)
pub fn i_05f88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F892;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F892);
    cpu.pc = 0x5F990;
    16
}
/// $05F892: bcc.w $5f8a6
pub fn i_05f892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F896;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F8A6; return 10; }
    8
}
/// $05F896: movea.l $c(a6), a0
pub fn i_05f896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F89A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F89A: move.b #$ee, $21(a0)
pub fn i_05f89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8A0;
    cpu.ir = 0x117C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F8A0: jmp $518.l
pub fn i_05f8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8A6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F8A6: rts
pub fn i_05f8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Shell_05f8a8  ($05F8A8, 118 B, props/late_props_turrets_05e000.s)
/// $05F8A8: jsr $267e2.l
pub fn i_05f8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F8AE);
    cpu.pc = 0x267E2;
    16
}
/// $05F8AE: lea.l $2c0bb6.l, a0
pub fn i_05f8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0BB6;
    20
}
/// $05F8B4: jsr $28cd4.l
pub fn i_05f8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F8BA);
    cpu.pc = 0x28CD4;
    16
}
/// $05F8BA: lea.l $5f8c0(pc), a1
pub fn i_05f8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5F8C0;
    16
}
/// $05F8BE: move.l a1, (a6)
pub fn i_05f8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F8C0: jsr $2783a.l
pub fn i_05f8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F8C6);
    cpu.pc = 0x2783A;
    16
}
/// $05F8C6: jsr $28d70.l
pub fn i_05f8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F8CC);
    cpu.pc = 0x28D70;
    16
}
/// $05F8CC: bcc.w $5f8ea
pub fn i_05f8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8D0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F8EA; return 10; }
    8
}
/// $05F8D0: lea.l $58f82.l, a1
pub fn i_05f8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8D6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $05F8D6: move.l a1, (a6)
pub fn i_05f8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05F8D8: lea.l $776e2.l, a1
pub fn i_05f8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8DE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $05F8DE: jsr $4ae.l
pub fn i_05f8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F8E4);
    cpu.pc = 0x4AE;
    16
}
/// $05F8E4: jsr $5dd02.l
pub fn i_05f8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F8EA);
    cpu.pc = 0x5DD02;
    16
}
/// $05F8EA: jsr $2870a.l
pub fn i_05f8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F8F0);
    cpu.pc = 0x2870A;
    16
}
/// $05F8F0: bcc.w $5f8fe
pub fn i_05f8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F8FE; return 10; }
    8
}
/// $05F8F4: movea.l $c(a6), a0
pub fn i_05f8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8F8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F8F8: move.b #$ee, $21(a0)
pub fn i_05f8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F8FE;
    cpu.ir = 0x117C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F8FE: jsr $49fd0.l
pub fn i_05f8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F904);
    cpu.pc = 0x49FD0;
    16
}
/// $05F904: jsr $5f990(pc)
pub fn i_05f904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F908;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5F908);
    cpu.pc = 0x5F990;
    16
}
/// $05F908: bcc.w $5f91c
pub fn i_05f908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F90C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F91C; return 10; }
    8
}
/// $05F90C: movea.l $c(a6), a0
pub fn i_05f90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F910;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F910: move.b #$ee, $21(a0)
pub fn i_05f910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F916;
    cpu.ir = 0x117C;
    let v1: u32 = 0xEE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F916: jmp $518.l
pub fn i_05f916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F91C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05F91C: rts
pub fn i_05f91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F91E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_OffsetPlus10_05f91e  ($05F91E, 20 B, props/late_props_turrets_05e000.s)
/// $05F91E: move.w $22(a6), d0
pub fn i_05f91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F922;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F922: subi.w #$20, d0
pub fn i_05f922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F926;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05F926: move.w $24(a6), d1
pub fn i_05f926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F92A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F92A: addi.w #$10, d1
pub fn i_05f92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F92E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05F92E: move.w #$40, d2
pub fn i_05f92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F932;
    cpu.ir = 0x343C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_05f932  ($05F932, 8 B, thunks/jsr_abs_thunks.c)
/// $05F932: jsr $99812.l
pub fn i_05f932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F938;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F938);
    cpu.pc = 0x99812;
    16
}
/// $05F938: rts
pub fn i_05f938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F93A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_Helper_05f93a  ($05F93A, 18 B, props/late_props_turrets_05e000.s)
/// $05F93A: movea.l $c(a6), a0
pub fn i_05f93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F93E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F93E: move.b #$ff, $21(a0)
pub fn i_05f93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F944;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F944: move.b #$ff, $70(a6)
pub fn i_05f944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F94A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05F94A: rts
pub fn i_05f94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F94C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerSoldier_SpawnDebris_05f94c  ($05F94C, 56 B, props/late_props_turrets_05e000.s)
/// $05F94C: movea.l $c(a6), a0
pub fn i_05f94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F950;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05F950: move.w $22(a0), d0
pub fn i_05f950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F954;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F954: sub.w $22(a6), d0
pub fn i_05f954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F958;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05F958: btst.b #$f, d0
pub fn i_05f958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F95C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05F95C: beq.w $5f962
pub fn i_05f95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F960;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F962; return 10; }
    8
}
/// $05F960: neg.w d0
pub fn i_05f960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F962;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05F962: cmpi.w #$a, d0
pub fn i_05f962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F966;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $05F966: bgt.w $5f98a
pub fn i_05f966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F96A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5F98A; return 10; }
    8
}
/// $05F96A: move.w $24(a0), d0
pub fn i_05f96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F96E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05F96E: sub.w $24(a6), d0
pub fn i_05f96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F972;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05F972: btst.b #$f, d0
pub fn i_05f972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F976;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05F976: beq.w $5f97c
pub fn i_05f976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F97A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5F97C; return 10; }
    8
}
/// $05F97A: neg.w d0
pub fn i_05f97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F97C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05F97C: cmpi.w #$30, d0
pub fn i_05f97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F980;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $05F980: bgt.w $5f98a
pub fn i_05f980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F984;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5F98A; return 10; }
    8
}
// ===== SetC_05f984  ($05F984, 6 B, thunks/ccr_helpers.c)
/// $05F984: ori.b #$1, ccr
pub fn i_05f984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F988;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05F988: rts
pub fn i_05f988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F98A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_05f98a  ($05F98A, 6 B, thunks/ccr_helpers.c)
/// $05F98A: andi.b #$fe, ccr
pub fn i_05f98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F98E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05F98E: rts
pub fn i_05f98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F990;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C0ED4_05f990  ($05F990, 16 B, props/late_props_turrets_05e000.s)
/// $05F990: lea.l $2c0ed4.l, a0
pub fn i_05f990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F996;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0ED4;
    20
}
/// $05F996: jsr $5dd56.l
pub fn i_05f996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F99C);
    cpu.pc = 0x5DD56;
    16
}
/// $05F99C: bcc.w $5f9a6
pub fn i_05f99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F9A6; return 10; }
    8
}
// ===== SetC_05f9a0  ($05F9A0, 6 B, thunks/ccr_helpers.c)
/// $05F9A0: ori.b #$1, ccr
pub fn i_05f9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9A4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05F9A4: rts
pub fn i_05f9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_05f9a6  ($05F9A6, 6 B, thunks/ccr_helpers.c)
/// $05F9A6: andi.b #$fe, ccr
pub fn i_05f9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9AA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05F9AA: rts
pub fn i_05f9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C0EDE_05f9ac  ($05F9AC, 16 B, props/late_props_turrets_05e000.s)
/// $05F9AC: lea.l $2c0ede.l, a0
pub fn i_05f9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0EDE;
    20
}
/// $05F9B2: jsr $5dd5c.l
pub fn i_05f9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F9B8);
    cpu.pc = 0x5DD5C;
    16
}
/// $05F9B8: bcc.w $5f9c2
pub fn i_05f9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F9C2; return 10; }
    8
}
// ===== SetC_05f9bc  ($05F9BC, 6 B, thunks/ccr_helpers.c)
/// $05F9BC: ori.b #$1, ccr
pub fn i_05f9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9C0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05F9C0: rts
pub fn i_05f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_05f9c2  ($05F9C2, 6 B, thunks/ccr_helpers.c)
/// $05F9C2: andi.b #$fe, ccr
pub fn i_05f9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9C6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05F9C6: rts
pub fn i_05f9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C0EE8_05f9c8  ($05F9C8, 16 B, props/late_props_turrets_05e000.s)
/// $05F9C8: lea.l $2c0ee8.l, a0
pub fn i_05f9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0EE8;
    20
}
/// $05F9CE: jsr $5dd56.l
pub fn i_05f9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5F9D4);
    cpu.pc = 0x5DD56;
    16
}
/// $05F9D4: bcc.w $5f9de
pub fn i_05f9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5F9DE; return 10; }
    8
}
// ===== SetC_05f9d8  ($05F9D8, 6 B, thunks/ccr_helpers.c)
/// $05F9D8: ori.b #$1, ccr
pub fn i_05f9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9DC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05F9DC: rts
pub fn i_05f9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_05f9de  ($05F9DE, 6 B, thunks/ccr_helpers.c)
/// $05F9DE: andi.b #$fe, ccr
pub fn i_05f9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9E2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05F9E2: rts
pub fn i_05f9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C0EF2_05f9e4  ($05F9E4, 16 B, props/late_props_turrets_05e000.s)
/// $05F9E4: movea.l $8(a6), a1
pub fn i_05f9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9E8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05F9E8: move.b $10(a6), d0
pub fn i_05f9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9EC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05F9EC: cmp.b $10(a1), d0
pub fn i_05f9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9F0;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05F9F0: bcs.w $5f9fa
pub fn i_05f9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9F4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5F9FA; return 10; }
    8
}
// ===== ClearXN_05f9f4  ($05F9F4, 6 B, thunks/ccr_helpers.c)
/// $05F9F4: andi.b #$ee, ccr
pub fn i_05f9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9F8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05F9F8: rts
pub fn i_05f9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05f9fa  ($05F9FA, 6 B, thunks/ccr_helpers.c)
/// $05F9FA: ori.b #$11, ccr
pub fn i_05f9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5F9FE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05F9FE: rts
pub fn i_05f9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutOccupant_Init_05fa00  ($05FA00, 78 B, props/late_props_turrets_05e000.s)
/// $05FA00: move.w #$e, d1
pub fn i_05fa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA04;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FA04: jsr $236e.l
pub fn i_05fa04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA0A);
    cpu.pc = 0x236E;
    16
}
/// $05FA0A: move.b #$ff, $32(a6)
pub fn i_05fa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA10;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FA10: move.b #$ff, $33(a6)
pub fn i_05fa10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA16;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FA16: addq.w #$2, $38(a6)
pub fn i_05fa16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA1A;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05FA1A: move.b #$0, $20(a6)
pub fn i_05fa1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA20;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FA20: move.b #$0, $21(a6)
pub fn i_05fa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA26;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FA26: move.w #$10, $66(a6)
pub fn i_05fa26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FA2C: jsr $267e2.l
pub fn i_05fa2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA32);
    cpu.pc = 0x267E2;
    16
}
/// $05FA32: lea.l $2c0f20.l, a0
pub fn i_05fa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA38;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0F20;
    20
}
/// $05FA38: jsr $28cd4.l
pub fn i_05fa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA3E);
    cpu.pc = 0x28CD4;
    16
}
/// $05FA3E: lea.l $5fba0(pc), a1
pub fn i_05fa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FBA0;
    16
}
/// $05FA42: jsr $4ae.l
pub fn i_05fa42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA48);
    cpu.pc = 0x4AE;
    16
}
/// $05FA48: jsr $5dd02.l
pub fn i_05fa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA4E);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_05fa4e  ($05FA4E, 8 B, kernel/task_handlers.c)
/// $05FA4E: lea.l $5fa56(pc), a1
pub fn i_05fa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FA56;
    16
}
/// $05FA52: move.l a1, (a6)
pub fn i_05fa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FA54: rts
pub fn i_05fa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutOccupant_Idle_05fa56  ($05FA56, 72 B, props/late_props_turrets_05e000.s)
/// $05FA56: jsr $2783a.l
pub fn i_05fa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA5C);
    cpu.pc = 0x2783A;
    16
}
/// $05FA5C: jsr $28d70.l
pub fn i_05fa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA62);
    cpu.pc = 0x28D70;
    16
}
/// $05FA62: cmpi.b #$ff, $21(a6)
pub fn i_05fa62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA68;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05FA68: bne.w $5fa72
pub fn i_05fa68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FA72; return 10; }
    8
}
/// $05FA6C: lea.l $5faa6(pc), a1
pub fn i_05fa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FAA6;
    16
}
/// $05FA70: move.l a1, (a6)
pub fn i_05fa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FA72: jsr $5fd08(pc)
pub fn i_05fa72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FA76);
    cpu.pc = 0x5FD08;
    16
}
/// $05FA76: bcc.w $5fa80
pub fn i_05fa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FA80; return 10; }
    8
}
/// $05FA7A: lea.l $5faa6(pc), a1
pub fn i_05fa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FAA6;
    16
}
/// $05FA7E: move.l a1, (a6)
pub fn i_05fa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FA80: jsr $2870a.l
pub fn i_05fa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA86);
    cpu.pc = 0x2870A;
    16
}
/// $05FA86: bcc.w $5fa90
pub fn i_05fa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FA90; return 10; }
    8
}
/// $05FA8A: move.b #$ff, $20(a6)
pub fn i_05fa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FA90: jsr $49fd0.l
pub fn i_05fa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FA96);
    cpu.pc = 0x49FD0;
    16
}
/// $05FA96: jsr $5fd40(pc)
pub fn i_05fa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FA9A);
    cpu.pc = 0x5FD40;
    16
}
/// $05FA9A: bcc.w $5faa4
pub fn i_05fa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FA9E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FAA4; return 10; }
    8
}
// ===== SetTaskHandler_05fa9e  ($05FA9E, 8 B, kernel/task_handlers.c)
/// $05FA9E: lea.l $5fb88(pc), a1
pub fn i_05fa9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAA2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FB88;
    16
}
/// $05FAA2: move.l a1, (a6)
pub fn i_05faa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAA4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FAA4: rts
pub fn i_05faa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutOccupant_Emerge_05faa6  ($05FAA6, 118 B, props/late_props_turrets_05e000.s)
/// $05FAA6: jsr $267e2.l
pub fn i_05faa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FAAC);
    cpu.pc = 0x267E2;
    16
}
/// $05FAAC: lea.l $2c0f9c.l, a0
pub fn i_05faac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0F9C;
    20
}
/// $05FAB2: jsr $28cd4.l
pub fn i_05fab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FAB8);
    cpu.pc = 0x28CD4;
    16
}
/// $05FAB8: lea.l $5fabe(pc), a1
pub fn i_05fab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FABC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FABE;
    16
}
/// $05FABC: move.l a1, (a6)
pub fn i_05fabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FABE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FABE: jsr $2783a.l
pub fn i_05fabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FAC4);
    cpu.pc = 0x2783A;
    16
}
/// $05FAC4: jsr $28d70.l
pub fn i_05fac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FACA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FACA);
    cpu.pc = 0x28D70;
    16
}
/// $05FACA: bcc.w $5fafe
pub fn i_05faca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FACE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FAFE; return 10; }
    8
}
/// $05FACE: move.b #$80, $20(a6)
pub fn i_05face(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAD4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FAD4: lea.l $5fb24(pc), a1
pub fn i_05fad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FB24;
    16
}
/// $05FAD8: move.l a1, (a6)
pub fn i_05fad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FADA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FADA: cmpi.b #$ff, $21(a6)
pub fn i_05fada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAE0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05FAE0: bne.w $5fafe
pub fn i_05fae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FAFE; return 10; }
    8
}
/// $05FAE4: lea.l $58f82.l, a1
pub fn i_05fae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAEA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $05FAEA: move.l a1, (a6)
pub fn i_05faea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FAEC: lea.l $776e2.l, a1
pub fn i_05faec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAF2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $05FAF2: jsr $4ae.l
pub fn i_05faf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FAF8);
    cpu.pc = 0x4AE;
    16
}
/// $05FAF8: jsr $5dd02.l
pub fn i_05faf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FAFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FAFE);
    cpu.pc = 0x5DD02;
    16
}
/// $05FAFE: jsr $2870a.l
pub fn i_05fafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB04);
    cpu.pc = 0x2870A;
    16
}
/// $05FB04: bcc.w $5fb0e
pub fn i_05fb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB08;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FB0E; return 10; }
    8
}
/// $05FB08: move.b #$ff, $20(a6)
pub fn i_05fb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB0E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FB0E: jsr $49fd0.l
pub fn i_05fb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB14);
    cpu.pc = 0x49FD0;
    16
}
/// $05FB14: jsr $5fd40(pc)
pub fn i_05fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FB18);
    cpu.pc = 0x5FD40;
    16
}
/// $05FB18: bcc.w $5fb22
pub fn i_05fb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FB22; return 10; }
    8
}
// ===== SetTaskHandler_05fb1c  ($05FB1C, 8 B, kernel/task_handlers.c)
/// $05FB1C: lea.l $5fb88(pc), a1
pub fn i_05fb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FB88;
    16
}
/// $05FB20: move.l a1, (a6)
pub fn i_05fb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FB22: rts
pub fn i_05fb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutOccupant_Out_05fb24  ($05FB24, 92 B, props/late_props_turrets_05e000.s)
/// $05FB24: jsr $267e2.l
pub fn i_05fb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB2A);
    cpu.pc = 0x267E2;
    16
}
/// $05FB2A: lea.l $2c10d8.l, a0
pub fn i_05fb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C10D8;
    20
}
/// $05FB30: jsr $28cd4.l
pub fn i_05fb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB36);
    cpu.pc = 0x28CD4;
    16
}
/// $05FB36: lea.l $5fb3c(pc), a1
pub fn i_05fb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FB3C;
    16
}
/// $05FB3A: move.l a1, (a6)
pub fn i_05fb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB3C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FB3C: jsr $2783a.l
pub fn i_05fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB42);
    cpu.pc = 0x2783A;
    16
}
/// $05FB42: jsr $28d70.l
pub fn i_05fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB48);
    cpu.pc = 0x28D70;
    16
}
/// $05FB48: cmpi.b #$ff, $21(a6)
pub fn i_05fb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB4E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05FB4E: bne.w $5fb58
pub fn i_05fb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FB58; return 10; }
    8
}
/// $05FB52: lea.l $5faa6(pc), a1
pub fn i_05fb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB56;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FAA6;
    16
}
/// $05FB56: move.l a1, (a6)
pub fn i_05fb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB58;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FB58: jsr $2870a.l
pub fn i_05fb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB5E);
    cpu.pc = 0x2870A;
    16
}
/// $05FB5E: bcc.w $5fb72
pub fn i_05fb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FB72; return 10; }
    8
}
/// $05FB62: move.b #$ff, $20(a6)
pub fn i_05fb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FB68: move.w #$10ae, d0
pub fn i_05fb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB6C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10AE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FB6C: jsr $2222.l
pub fn i_05fb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB72);
    cpu.pc = 0x2222;
    16
}
/// $05FB72: jsr $49fd0.l
pub fn i_05fb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FB78);
    cpu.pc = 0x49FD0;
    16
}
/// $05FB78: jsr $5fd40(pc)
pub fn i_05fb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FB7C);
    cpu.pc = 0x5FD40;
    16
}
/// $05FB7C: bcc.w $5fb86
pub fn i_05fb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FB86; return 10; }
    8
}
// ===== SetTaskHandler_05fb80  ($05FB80, 8 B, kernel/task_handlers.c)
/// $05FB80: lea.l $5fb88(pc), a1
pub fn i_05fb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FB88;
    16
}
/// $05FB84: move.l a1, (a6)
pub fn i_05fb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FB86: rts
pub fn i_05fb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutOccupant_WaitParent_05fb88  ($05FB88, 24 B, props/late_props_turrets_05e000.s)
/// $05FB88: lea.l $5fb8e(pc), a1
pub fn i_05fb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FB8E;
    16
}
/// $05FB8C: move.l a1, (a6)
pub fn i_05fb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FB8E: cmpi.b #$ff, $21(a6)
pub fn i_05fb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB94;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05FB94: bne.w $5fb9e
pub fn i_05fb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FB9E; return 10; }
    8
}
/// $05FB98: jmp $518.l
pub fn i_05fb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FB9E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FB9E: rts
pub fn i_05fb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutDoor_Init_05fba0  ($05FBA0, 62 B, props/late_props_turrets_05e000.s)
/// $05FBA0: move.w #$19, d1
pub fn i_05fba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBA4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FBA4: jsr $236e.l
pub fn i_05fba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FBAA);
    cpu.pc = 0x236E;
    16
}
/// $05FBAA: move.b #$0, $46(a6)
pub fn i_05fbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBB0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FBB0: move.b #$ff, $32(a6)
pub fn i_05fbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FBB6: move.b #$ff, $33(a6)
pub fn i_05fbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FBBC: move.b #$0, $44(a6)
pub fn i_05fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FBC2: move.w #$10, $66(a6)
pub fn i_05fbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FBC8: subq.w #$1, $38(a6)
pub fn i_05fbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBCC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05FBCC: jsr $267e2.l
pub fn i_05fbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FBD2);
    cpu.pc = 0x267E2;
    16
}
/// $05FBD2: lea.l $2c14cc.l, a0
pub fn i_05fbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBD8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C14CC;
    20
}
/// $05FBD8: jsr $28cd4.l
pub fn i_05fbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FBDE);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_05fbde  ($05FBDE, 8 B, kernel/task_handlers.c)
/// $05FBDE: lea.l $5fbe6(pc), a1
pub fn i_05fbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FBE6;
    16
}
/// $05FBE2: move.l a1, (a6)
pub fn i_05fbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FBE4: rts
pub fn i_05fbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutDoor_Idle_05fbe6  ($05FBE6, 56 B, props/late_props_turrets_05e000.s)
/// $05FBE6: jsr $2783a.l
pub fn i_05fbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FBEC);
    cpu.pc = 0x2783A;
    16
}
/// $05FBEC: jsr $28d70.l
pub fn i_05fbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FBF2);
    cpu.pc = 0x28D70;
    16
}
/// $05FBF2: movea.l $c(a6), a0
pub fn i_05fbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBF6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05FBF6: cmpi.b #$80, $20(a0)
pub fn i_05fbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FBFC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $05FBFC: bne.w $5fc06
pub fn i_05fbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FC06; return 10; }
    8
}
/// $05FC00: lea.l $5fc26(pc), a1
pub fn i_05fc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FC26;
    16
}
/// $05FC04: move.l a1, (a6)
pub fn i_05fc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FC06: jsr $2870a.l
pub fn i_05fc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FC0C);
    cpu.pc = 0x2870A;
    16
}
/// $05FC0C: bcc.w $5fc16
pub fn i_05fc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FC16; return 10; }
    8
}
/// $05FC10: lea.l $5fc78(pc), a1
pub fn i_05fc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FC78;
    16
}
/// $05FC14: move.l a1, (a6)
pub fn i_05fc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FC16: jsr $5fd40(pc)
pub fn i_05fc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FC1A);
    cpu.pc = 0x5FD40;
    16
}
/// $05FC1A: bcc.w $5fc24
pub fn i_05fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FC24; return 10; }
    8
}
// ===== SetTaskHandler_05fc1e  ($05FC1E, 8 B, kernel/task_handlers.c)
/// $05FC1E: lea.l $5fce6(pc), a1
pub fn i_05fc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FCE6;
    16
}
/// $05FC22: move.l a1, (a6)
pub fn i_05fc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FC24: rts
pub fn i_05fc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutDoor_Open_05fc26  ($05FC26, 74 B, props/late_props_turrets_05e000.s)
/// $05FC26: lea.l $2c14e2.l, a0
pub fn i_05fc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C14E2;
    20
}
/// $05FC2C: jsr $28cd4.l
pub fn i_05fc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FC32);
    cpu.pc = 0x28CD4;
    16
}
/// $05FC32: lea.l $5fc38(pc), a1
pub fn i_05fc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FC38;
    16
}
/// $05FC36: move.l a1, (a6)
pub fn i_05fc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FC38: jsr $2783a.l
pub fn i_05fc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FC3E);
    cpu.pc = 0x2783A;
    16
}
/// $05FC3E: movea.l $c(a6), a0
pub fn i_05fc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC42;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05FC42: cmpi.b #$ff, $20(a0)
pub fn i_05fc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC48;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $05FC48: bne.w $5fc52
pub fn i_05fc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC4C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FC52; return 10; }
    8
}
/// $05FC4C: move.b #$2, $46(a6)
pub fn i_05fc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FC52: jsr $28d70.l
pub fn i_05fc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FC58);
    cpu.pc = 0x28D70;
    16
}
/// $05FC58: jsr $2870a.l
pub fn i_05fc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FC5E);
    cpu.pc = 0x2870A;
    16
}
/// $05FC5E: bcc.w $5fc68
pub fn i_05fc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FC68; return 10; }
    8
}
/// $05FC62: lea.l $5fc78(pc), a1
pub fn i_05fc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FC78;
    16
}
/// $05FC66: move.l a1, (a6)
pub fn i_05fc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FC68: jsr $5fd40(pc)
pub fn i_05fc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FC6C);
    cpu.pc = 0x5FD40;
    16
}
/// $05FC6C: bcc.w $5fc76
pub fn i_05fc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FC76; return 10; }
    8
}
// ===== SetTaskHandler_05fc70  ($05FC70, 8 B, kernel/task_handlers.c)
/// $05FC70: lea.l $5fce6(pc), a1
pub fn i_05fc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FCE6;
    16
}
/// $05FC74: move.l a1, (a6)
pub fn i_05fc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FC76: rts
pub fn i_05fc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutDoor_Break_05fc78  ($05FC78, 102 B, props/late_props_turrets_05e000.s)
/// $05FC78: move.w #$10ae, d0
pub fn i_05fc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC7C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10AE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FC7C: jsr $2222.l
pub fn i_05fc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FC82);
    cpu.pc = 0x2222;
    16
}
/// $05FC82: movea.l $c(a6), a0
pub fn i_05fc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC86;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05FC86: move.b #$ff, $21(a0)
pub fn i_05fc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC8C;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FC8C: move.w #$108e, d0
pub fn i_05fc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC90;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FC90: jsr $2352.l
pub fn i_05fc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FC96);
    cpu.pc = 0x2352;
    16
}
/// $05FC96: lea.l $2c1546.l, a0
pub fn i_05fc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FC9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1546;
    20
}
/// $05FC9C: jsr $28cd4.l
pub fn i_05fc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FCA2);
    cpu.pc = 0x28CD4;
    16
}
/// $05FCA2: lea.l $5fca8(pc), a1
pub fn i_05fca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FCA8;
    16
}
/// $05FCA6: move.l a1, (a6)
pub fn i_05fca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FCA8: jsr $2783a.l
pub fn i_05fca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FCAE);
    cpu.pc = 0x2783A;
    16
}
/// $05FCAE: jsr $28d70.l
pub fn i_05fcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FCB4);
    cpu.pc = 0x28D70;
    16
}
/// $05FCB4: bcc.w $5fcd6
pub fn i_05fcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FCD6; return 10; }
    8
}
/// $05FCB8: lea.l $77efe.l, a1
pub fn i_05fcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCBE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77EFE;
    20
}
/// $05FCBE: jsr $6fe.l
pub fn i_05fcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FCC4);
    cpu.pc = 0x6FE;
    16
}
/// $05FCC4: jsr $5dd02.l
pub fn i_05fcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FCCA);
    cpu.pc = 0x5DD02;
    16
}
/// $05FCCA: subi.w #$10, $22(a0)
pub fn i_05fcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCD0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05FCD0: lea.l $5fce6(pc), a1
pub fn i_05fcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FCE6;
    16
}
/// $05FCD4: move.l a1, (a6)
pub fn i_05fcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FCD6: jsr $5fd40(pc)
pub fn i_05fcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FCDA);
    cpu.pc = 0x5FD40;
    16
}
/// $05FCDA: bcc.w $5fce4
pub fn i_05fcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCDE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FCE4; return 10; }
    8
}
// ===== SetTaskHandler_05fcde  ($05FCDE, 8 B, kernel/task_handlers.c)
/// $05FCDE: lea.l $5fce6(pc), a1
pub fn i_05fcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FCE6;
    16
}
/// $05FCE2: move.l a1, (a6)
pub fn i_05fce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FCE4: rts
pub fn i_05fce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutDoor_Free_05fce6  ($05FCE6, 32 B, props/late_props_turrets_05e000.s)
/// $05FCE6: move.w #$10ae, d0
pub fn i_05fce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10AE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FCEA: jsr $2222.l
pub fn i_05fcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FCF0);
    cpu.pc = 0x2222;
    16
}
/// $05FCF0: movea.l $c(a6), a0
pub fn i_05fcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCF4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05FCF4: move.b #$ff, $21(a0)
pub fn i_05fcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCFA;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FCFA: lea.l $5fd00(pc), a1
pub fn i_05fcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FCFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FD00;
    16
}
/// $05FCFE: move.l a1, (a6)
pub fn i_05fcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FD00: jmp $518.l
pub fn i_05fd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD06;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== HutDoor_Rts_05fd06  ($05FD06, 2 B, props/late_props_turrets_05e000.s)
/// $05FD06: rts
pub fn i_05fd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HutOccupant_PlayersNear_05fd08  ($05FD08, 44 B, props/late_props_turrets_05e000.s)
/// $05FD08: lea.l $100440.l, a0
pub fn i_05fd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD0E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $05FD0E: move.w $22(a6), d0
pub fn i_05fd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD12;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05FD12: sub.w $22(a0), d0
pub fn i_05fd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD16;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05FD16: cmpi.w #$a0, d0
pub fn i_05fd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD1A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $05FD1A: blt.w $5fd3a
pub fn i_05fd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD1E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5FD3A; return 10; }
    8
}
/// $05FD1E: lea.l $1004e0.l, a0
pub fn i_05fd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $05FD24: move.w $22(a6), d0
pub fn i_05fd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD28;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05FD28: sub.w $22(a0), d0
pub fn i_05fd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD2C;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05FD2C: cmpi.w #$a0, d0
pub fn i_05fd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD30;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $05FD30: blt.w $5fd3a
pub fn i_05fd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD34;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5FD3A; return 10; }
    8
}
// ===== ClearC_05fd34  ($05FD34, 6 B, thunks/ccr_helpers.c)
/// $05FD34: andi.b #$fe, ccr
pub fn i_05fd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD38;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05FD38: rts
pub fn i_05fd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD3A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_05fd3a  ($05FD3A, 6 B, thunks/ccr_helpers.c)
/// $05FD3A: ori.b #$1, ccr
pub fn i_05fd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD3E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05FD3E: rts
pub fn i_05fd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C168E_05fd40  ($05FD40, 16 B, props/late_props_turrets_05e000.s)
/// $05FD40: lea.l $2c168e.l, a0
pub fn i_05fd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C168E;
    20
}
/// $05FD46: jsr $5dd5c.l
pub fn i_05fd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FD4C);
    cpu.pc = 0x5DD5C;
    16
}
/// $05FD4C: bcc.w $5fd56
pub fn i_05fd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FD56; return 10; }
    8
}
// ===== SetC_05fd50  ($05FD50, 6 B, thunks/ccr_helpers.c)
/// $05FD50: ori.b #$1, ccr
pub fn i_05fd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD54;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05FD54: rts
pub fn i_05fd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_05fd56  ($05FD56, 6 B, thunks/ccr_helpers.c)
/// $05FD56: andi.b #$fe, ccr
pub fn i_05fd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD5A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05FD5A: rts
pub fn i_05fd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Offworld_Box2C168E_B_05fd5c  ($05FD5C, 16 B, props/late_props_turrets_05e000.s)
/// $05FD5C: movea.l $8(a6), a1
pub fn i_05fd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD60;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05FD60: move.b $10(a6), d0
pub fn i_05fd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD64;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05FD64: cmp.b $10(a1), d0
pub fn i_05fd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD68;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05FD68: bcs.w $5fd72
pub fn i_05fd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD6C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5FD72; return 10; }
    8
}
// ===== ClearXN_05fd6c  ($05FD6C, 6 B, thunks/ccr_helpers.c)
/// $05FD6C: andi.b #$ee, ccr
pub fn i_05fd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD70;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05FD70: rts
pub fn i_05fd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05fd72  ($05FD72, 6 B, thunks/ccr_helpers.c)
/// $05FD72: ori.b #$11, ccr
pub fn i_05fd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD76;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05FD76: rts
pub fn i_05fd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Tmpl1F_05fd78  ($05FD78, 54 B, props/late_props_turrets_05e000.s)
/// $05FD78: lea.l $2c175e.l, a0
pub fn i_05fd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C175E;
    20
}
/// $05FD7E: jsr $28cd4.l
pub fn i_05fd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FD84);
    cpu.pc = 0x28CD4;
    16
}
/// $05FD84: bra.w $5fda4
pub fn i_05fd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD88;
    cpu.ir = 0x6000;
    cpu.pc = 0x5FDA4;
    10
}
/// $05FD88: lea.l $2c17ac.l, a0
pub fn i_05fd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD8E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C17AC;
    20
}
/// $05FD8E: jsr $28cd4.l
pub fn i_05fd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FD94);
    cpu.pc = 0x28CD4;
    16
}
/// $05FD94: bra.w $5fda4
pub fn i_05fd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD98;
    cpu.ir = 0x6000;
    cpu.pc = 0x5FDA4;
    10
}
/// $05FD98: lea.l $2c1886.l, a0
pub fn i_05fd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FD9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1886;
    20
}
/// $05FD9E: jsr $28cd4.l
pub fn i_05fd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FDA4);
    cpu.pc = 0x28CD4;
    16
}
/// $05FDA4: cmpi.w #$140, $22(a6)
pub fn i_05fda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDAA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $05FDAA: blt.w $5fdb6
pub fn i_05fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDAE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5FDB6; return 10; }
    8
}
// ===== JsrAbsThunk_05fdae  ($05FDAE, 8 B, thunks/jsr_abs_thunks.c)
/// $05FDAE: jsr $2783a.l
pub fn i_05fdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FDB4);
    cpu.pc = 0x2783A;
    16
}
/// $05FDB4: rts
pub fn i_05fdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Init_05fdb6  ($05FDB6, 112 B, props/late_props_turrets_05e000.s)
/// $05FDB6: move.w #$e, d1
pub fn i_05fdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDBA;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FDBA: jsr $236e.l
pub fn i_05fdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FDC0);
    cpu.pc = 0x236E;
    16
}
/// $05FDC0: move.b #$ff, $32(a6)
pub fn i_05fdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FDC6: move.b #$ff, $33(a6)
pub fn i_05fdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDCC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FDCC: move.b $98(a6), d0
pub fn i_05fdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDD0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05FDD0: andi.b #$1, d0
pub fn i_05fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDD4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05FDD4: move.b d0, $3a(a6)
pub fn i_05fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDD8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FDD8: move.w #$8000, $38(a6)
pub fn i_05fdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FDDE: move.w #$1, $66(a6)
pub fn i_05fdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FDE4: jsr $267e2.l
pub fn i_05fde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FDEA);
    cpu.pc = 0x267E2;
    16
}
/// $05FDEA: lea.l $5fdf0(pc), a1
pub fn i_05fdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FDF0;
    16
}
/// $05FDEE: move.l a1, (a6)
pub fn i_05fdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FDF0: jsr $2783a.l
pub fn i_05fdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FDF6);
    cpu.pc = 0x2783A;
    16
}
/// $05FDF6: jsr $28d70.l
pub fn i_05fdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FDFC);
    cpu.pc = 0x28D70;
    16
}
/// $05FDFC: bcc.w $5fe06
pub fn i_05fdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FE06; return 10; }
    8
}
/// $05FE00: lea.l $5fe26(pc), a1
pub fn i_05fe00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $05FE04: move.l a1, (a6)
pub fn i_05fe04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FE06: jsr $2870a.l
pub fn i_05fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE0C);
    cpu.pc = 0x2870A;
    16
}
/// $05FE0C: bcc.w $5fe16
pub fn i_05fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FE16; return 10; }
    8
}
/// $05FE10: lea.l $60210(pc), a1
pub fn i_05fe10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FE14: move.l a1, (a6)
pub fn i_05fe14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FE16: jsr $6037c(pc)
pub fn i_05fe16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FE1A);
    cpu.pc = 0x6037C;
    16
}
/// $05FE1A: bcc.w $5fe24
pub fn i_05fe1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FE24; return 10; }
    8
}
/// $05FE1E: jmp $518.l
pub fn i_05fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE24;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FE24: rts
pub fn i_05fe24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Idle_05fe26  ($05FE26, 154 B, props/late_props_turrets_05e000.s)
/// $05FE26: lea.l $2b79a0.l, a0
pub fn i_05fe26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B79A0;
    20
}
/// $05FE2C: jsr $799de.l
pub fn i_05fe2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE32);
    cpu.pc = 0x799DE;
    16
}
/// $05FE32: move.w d0, $70(a6)
pub fn i_05fe32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE36;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FE36: lea.l $2c196e.l, a0
pub fn i_05fe36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE3C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C196E;
    20
}
/// $05FE3C: jsr $28cd4.l
pub fn i_05fe3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE42);
    cpu.pc = 0x28CD4;
    16
}
/// $05FE42: lea.l $5fe48(pc), a1
pub fn i_05fe42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE48;
    16
}
/// $05FE46: move.l a1, (a6)
pub fn i_05fe46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FE48: jsr $2783a.l
pub fn i_05fe48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE4E);
    cpu.pc = 0x2783A;
    16
}
/// $05FE4E: jsr $28d70.l
pub fn i_05fe4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE54);
    cpu.pc = 0x28D70;
    16
}
/// $05FE54: cmpi.w #$ff, $70(a6)
pub fn i_05fe54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE5A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    16
}
/// $05FE5A: beq.w $5fe78
pub fn i_05fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5FE78; return 10; }
    8
}
/// $05FE5E: subi.w #$1, $70(a6)
pub fn i_05fe5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE64;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05FE64: beq.w $5fe72
pub fn i_05fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5FE72; return 10; }
    8
}
/// $05FE68: subi.w #$1, $70(a6)
pub fn i_05fe68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE6E;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05FE6E: bne.w $5fe78
pub fn i_05fe6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FE78; return 10; }
    8
}
/// $05FE72: lea.l $5fec0(pc), a1
pub fn i_05fe72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FEC0;
    16
}
/// $05FE76: move.l a1, (a6)
pub fn i_05fe76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FE78: jsr $5e0d4.l
pub fn i_05fe78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE7E);
    cpu.pc = 0x5E0D4;
    16
}
/// $05FE7E: move.w $22(a0), d0
pub fn i_05fe7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE82;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05FE82: sub.w $22(a6), d0
pub fn i_05fe82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE86;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05FE86: rol.w #$1, d0
pub fn i_05fe86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE88;
    cpu.ir = 0xE358;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05FE88: andi.w #$1, d0
pub fn i_05fe88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE8C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05FE8C: move.b $3a(a6), d1
pub fn i_05fe8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE90;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05FE90: andi.b #$1, d1
pub fn i_05fe90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE94;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05FE94: cmp.b d0, d1
pub fn i_05fe94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE96;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05FE96: bne.w $5fea0
pub fn i_05fe96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE9A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FEA0; return 10; }
    8
}
/// $05FE9A: lea.l $5ff1c(pc), a1
pub fn i_05fe9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FF1C;
    16
}
/// $05FE9E: move.l a1, (a6)
pub fn i_05fe9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FEA0: jsr $2870a.l
pub fn i_05fea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEA6);
    cpu.pc = 0x2870A;
    16
}
/// $05FEA6: bcc.w $5feb0
pub fn i_05fea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FEB0; return 10; }
    8
}
/// $05FEAA: lea.l $60210(pc), a1
pub fn i_05feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FEAE: move.l a1, (a6)
pub fn i_05feae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FEB0: jsr $6037c(pc)
pub fn i_05feb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FEB4);
    cpu.pc = 0x6037C;
    16
}
/// $05FEB4: bcc.w $5febe
pub fn i_05feb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FEBE; return 10; }
    8
}
/// $05FEB8: jmp $518.l
pub fn i_05feb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEBE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FEBE: rts
pub fn i_05febe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Turn_05fec0  ($05FEC0, 92 B, props/late_props_turrets_05e000.s)
/// $05FEC0: jsr $5e0d4.l
pub fn i_05fec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEC6);
    cpu.pc = 0x5E0D4;
    16
}
/// $05FEC6: move.w $22(a0), d0
pub fn i_05fec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FECA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05FECA: sub.w $22(a6), d0
pub fn i_05feca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FECE;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05FECE: asl.w #$1, d0
pub fn i_05fece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FED0;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05FED0: move.w d0, $28(a6)
pub fn i_05fed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FED4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FED4: lea.l $2c19ce.l, a0
pub fn i_05fed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C19CE;
    20
}
/// $05FEDA: jsr $28cd4.l
pub fn i_05feda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEE0);
    cpu.pc = 0x28CD4;
    16
}
/// $05FEE0: lea.l $5fee6(pc), a1
pub fn i_05fee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FEE6;
    16
}
/// $05FEE4: move.l a1, (a6)
pub fn i_05fee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FEE6: jsr $2783a.l
pub fn i_05fee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEEC);
    cpu.pc = 0x2783A;
    16
}
/// $05FEEC: jsr $28d70.l
pub fn i_05feec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEF2);
    cpu.pc = 0x28D70;
    16
}
/// $05FEF2: bcc.w $5fefc
pub fn i_05fef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FEFC; return 10; }
    8
}
/// $05FEF6: lea.l $5fe26(pc), a1
pub fn i_05fef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $05FEFA: move.l a1, (a6)
pub fn i_05fefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FEFC: jsr $2870a.l
pub fn i_05fefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF02);
    cpu.pc = 0x2870A;
    16
}
/// $05FF02: bcc.w $5ff0c
pub fn i_05ff02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF0C; return 10; }
    8
}
/// $05FF06: lea.l $60210(pc), a1
pub fn i_05ff06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FF0A: move.l a1, (a6)
pub fn i_05ff0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FF0C: jsr $6037c(pc)
pub fn i_05ff0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FF10);
    cpu.pc = 0x6037C;
    16
}
/// $05FF10: bcc.w $5ff1a
pub fn i_05ff10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF1A; return 10; }
    8
}
/// $05FF14: jmp $518.l
pub fn i_05ff14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FF1A: rts
pub fn i_05ff1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Destroyed_05ff1c  ($05FF1C, 72 B, props/late_props_turrets_05e000.s)
/// $05FF1C: lea.l $2c1b86.l, a0
pub fn i_05ff1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1B86;
    20
}
/// $05FF22: jsr $28cd4.l
pub fn i_05ff22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF28);
    cpu.pc = 0x28CD4;
    16
}
/// $05FF28: lea.l $5ff2e(pc), a1
pub fn i_05ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FF2E;
    16
}
/// $05FF2C: move.l a1, (a6)
pub fn i_05ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FF2E: jsr $2783a.l
pub fn i_05ff2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF34);
    cpu.pc = 0x2783A;
    16
}
/// $05FF34: jsr $28d70.l
pub fn i_05ff34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF3A);
    cpu.pc = 0x28D70;
    16
}
/// $05FF3A: bcc.w $5ff44
pub fn i_05ff3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF44; return 10; }
    8
}
/// $05FF3E: lea.l $5fe26(pc), a1
pub fn i_05ff3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $05FF42: move.l a1, (a6)
pub fn i_05ff42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FF44: jsr $2870a.l
pub fn i_05ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF4A);
    cpu.pc = 0x2870A;
    16
}
/// $05FF4A: bcc.w $5ff54
pub fn i_05ff4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF54; return 10; }
    8
}
/// $05FF4E: lea.l $60210(pc), a1
pub fn i_05ff4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FF52: move.l a1, (a6)
pub fn i_05ff52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FF54: jsr $6037c(pc)
pub fn i_05ff54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FF58);
    cpu.pc = 0x6037C;
    16
}
/// $05FF58: bcc.w $5ff62
pub fn i_05ff58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF62; return 10; }
    8
}
/// $05FF5C: jmp $518.l
pub fn i_05ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF62;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FF62: rts
pub fn i_05ff62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Tmpl22_05ff64  ($05FF64, 10 B, props/late_props_turrets_05e000.s)
/// $05FF64: cmpi.w #$140, $22(a6)
pub fn i_05ff64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF6A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $05FF6A: blt.w $5ff76
pub fn i_05ff6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF6E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5FF76; return 10; }
    8
}
// ===== JsrAbsThunk_05ff6e  ($05FF6E, 8 B, thunks/jsr_abs_thunks.c)
/// $05FF6E: jsr $2783a.l
pub fn i_05ff6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF74);
    cpu.pc = 0x2783A;
    16
}
/// $05FF74: rts
pub fn i_05ff74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable22_Init_05ff76  ($05FF76, 146 B, props/late_props_turrets_05e000.s)
/// $05FF76: move.w #$e, d1
pub fn i_05ff76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF7A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FF7A: jsr $236e.l
pub fn i_05ff7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF80);
    cpu.pc = 0x236E;
    16
}
/// $05FF80: move.b #$ff, $32(a6)
pub fn i_05ff80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF86;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FF86: move.b #$ff, $33(a6)
pub fn i_05ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF8C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FF8C: move.b $98(a6), d0
pub fn i_05ff8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF90;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05FF90: andi.b #$1, d0
pub fn i_05ff90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF94;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05FF94: move.b d0, $3a(a6)
pub fn i_05ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF98;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FF98: move.w #$8000, $38(a6)
pub fn i_05ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF9E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FF9E: move.w #$1, $66(a6)
pub fn i_05ff9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FFA4: jsr $267e2.l
pub fn i_05ffa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFAA);
    cpu.pc = 0x267E2;
    16
}
/// $05FFAA: lea.l $2c1b02.l, a0
pub fn i_05ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1B02;
    20
}
/// $05FFB0: jsr $28cd4.l
pub fn i_05ffb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFB6);
    cpu.pc = 0x28CD4;
    16
}
/// $05FFB6: lea.l $5ffbc(pc), a1
pub fn i_05ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FFBC;
    16
}
/// $05FFBA: move.l a1, (a6)
pub fn i_05ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FFBC: jsr $2783a.l
pub fn i_05ffbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFC2);
    cpu.pc = 0x2783A;
    16
}
/// $05FFC2: jsr $28d70.l
pub fn i_05ffc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFC8);
    cpu.pc = 0x28D70;
    16
}
/// $05FFC8: jsr $5e0d4.l
pub fn i_05ffc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFCE);
    cpu.pc = 0x5E0D4;
    16
}
/// $05FFCE: move.w $22(a0), d0
pub fn i_05ffce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFD2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05FFD2: sub.w $22(a6), d0
pub fn i_05ffd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFD6;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05FFD6: jsr $603b4(pc)
pub fn i_05ffd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FFDA);
    cpu.pc = 0x603B4;
    16
}
/// $05FFDA: cmpi.w #$20, d0
pub fn i_05ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFDE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $05FFDE: bgt.w $5ffe8
pub fn i_05ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFE2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5FFE8; return 10; }
    8
}
/// $05FFE2: lea.l $60008(pc), a1
pub fn i_05ffe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60008;
    16
}
/// $05FFE6: move.l a1, (a6)
pub fn i_05ffe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFE8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FFE8: jsr $2870a.l
pub fn i_05ffe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFEE);
    cpu.pc = 0x2870A;
    16
}
/// $05FFEE: bcc.w $5fff8
pub fn i_05ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FFF8; return 10; }
    8
}
/// $05FFF2: lea.l $60210(pc), a1
pub fn i_05fff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FFF6: move.l a1, (a6)
pub fn i_05fff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FFF8: jsr $6037c(pc)
pub fn i_05fff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FFFC);
    cpu.pc = 0x6037C;
    16
}
/// $05FFFC: bcc.w $60006
pub fn i_05fffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60000;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60006; return 10; }
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x05E960, i_05e960),
    (0x05E964, i_05e964),
    (0x05E96A, i_05e96a),
    (0x05E970, i_05e970),
    (0x05E976, i_05e976),
    (0x05E97C, i_05e97c),
    (0x05E982, i_05e982),
    (0x05E988, i_05e988),
    (0x05E98A, i_05e98a),
    (0x05E990, i_05e990),
    (0x05E996, i_05e996),
    (0x05E998, i_05e998),
    (0x05E99E, i_05e99e),
    (0x05E9A0, i_05e9a0),
    (0x05E9A2, i_05e9a2),
    (0x05E9A6, i_05e9a6),
    (0x05E9AA, i_05e9aa),
    (0x05E9AE, i_05e9ae),
    (0x05E9B4, i_05e9b4),
    (0x05E9B6, i_05e9b6),
    (0x05E9BC, i_05e9bc),
    (0x05E9C2, i_05e9c2),
    (0x05E9C6, i_05e9c6),
    (0x05E9CA, i_05e9ca),
    (0x05E9CC, i_05e9cc),
    (0x05E9D2, i_05e9d2),
    (0x05E9D6, i_05e9d6),
    (0x05E9DA, i_05e9da),
    (0x05E9DC, i_05e9dc),
    (0x05E9DE, i_05e9de),
    (0x05E9E0, i_05e9e0),
    (0x05E9E2, i_05e9e2),
    (0x05E9E4, i_05e9e4),
    (0x05E9E6, i_05e9e6),
    (0x05E9EC, i_05e9ec),
    (0x05E9F2, i_05e9f2),
    (0x05E9F6, i_05e9f6),
    (0x05E9FA, i_05e9fa),
    (0x05E9FC, i_05e9fc),
    (0x05EA02, i_05ea02),
    (0x05EA06, i_05ea06),
    (0x05EA08, i_05ea08),
    (0x05EA0C, i_05ea0c),
    (0x05EA0E, i_05ea0e),
    (0x05EA12, i_05ea12),
    (0x05EA16, i_05ea16),
    (0x05EA18, i_05ea18),
    (0x05EA1A, i_05ea1a),
    (0x05EA1C, i_05ea1c),
    (0x05EA1E, i_05ea1e),
    (0x05EA24, i_05ea24),
    (0x05EA2A, i_05ea2a),
    (0x05EA2E, i_05ea2e),
    (0x05EA32, i_05ea32),
    (0x05EA34, i_05ea34),
    (0x05EA3A, i_05ea3a),
    (0x05EA3E, i_05ea3e),
    (0x05EA40, i_05ea40),
    (0x05EA44, i_05ea44),
    (0x05EA46, i_05ea46),
    (0x05EA4A, i_05ea4a),
    (0x05EA4E, i_05ea4e),
    (0x05EA50, i_05ea50),
    (0x05EA52, i_05ea52),
    (0x05EA56, i_05ea56),
    (0x05EA5A, i_05ea5a),
    (0x05EA5E, i_05ea5e),
    (0x05EA62, i_05ea62),
    (0x05EA66, i_05ea66),
    (0x05EA68, i_05ea68),
    (0x05EA6C, i_05ea6c),
    (0x05EA6E, i_05ea6e),
    (0x05EA74, i_05ea74),
    (0x05EA78, i_05ea78),
    (0x05EA7E, i_05ea7e),
    (0x05EA84, i_05ea84),
    (0x05EA86, i_05ea86),
    (0x05EA8C, i_05ea8c),
    (0x05EA92, i_05ea92),
    (0x05EA94, i_05ea94),
    (0x05EA96, i_05ea96),
    (0x05EA9A, i_05ea9a),
    (0x05EA9C, i_05ea9c),
    (0x05EAA2, i_05eaa2),
    (0x05EAA4, i_05eaa4),
    (0x05EAA6, i_05eaa6),
    (0x05EAAC, i_05eaac),
    (0x05EAB2, i_05eab2),
    (0x05EAB4, i_05eab4),
    (0x05EAB6, i_05eab6),
    (0x05EAB8, i_05eab8),
    (0x05EABE, i_05eabe),
    (0x05EAC4, i_05eac4),
    (0x05EAC6, i_05eac6),
    (0x05EAC8, i_05eac8),
    (0x05EACC, i_05eacc),
    (0x05EAD0, i_05ead0),
    (0x05EAD4, i_05ead4),
    (0x05EAD8, i_05ead8),
    (0x05EADC, i_05eadc),
    (0x05EADE, i_05eade),
    (0x05EAE2, i_05eae2),
    (0x05EAE4, i_05eae4),
    (0x05EAE8, i_05eae8),
    (0x05EAEE, i_05eaee),
    (0x05EAF2, i_05eaf2),
    (0x05EAF6, i_05eaf6),
    (0x05EAF8, i_05eaf8),
    (0x05EAFA, i_05eafa),
    (0x05EAFC, i_05eafc),
    (0x05EB00, i_05eb00),
    (0x05EB04, i_05eb04),
    (0x05EB06, i_05eb06),
    (0x05EB0A, i_05eb0a),
    (0x05EB0E, i_05eb0e),
    (0x05EB12, i_05eb12),
    (0x05EB16, i_05eb16),
    (0x05EB1A, i_05eb1a),
    (0x05EB20, i_05eb20),
    (0x05EB24, i_05eb24),
    (0x05EB26, i_05eb26),
    (0x05EB2A, i_05eb2a),
    (0x05EB2C, i_05eb2c),
    (0x05EB30, i_05eb30),
    (0x05EB34, i_05eb34),
    (0x05EB38, i_05eb38),
    (0x05EB3C, i_05eb3c),
    (0x05EB42, i_05eb42),
    (0x05EB44, i_05eb44),
    (0x05EB48, i_05eb48),
    (0x05EB4C, i_05eb4c),
    (0x05EB4E, i_05eb4e),
    (0x05EB52, i_05eb52),
    (0x05EB56, i_05eb56),
    (0x05EB5A, i_05eb5a),
    (0x05EB60, i_05eb60),
    (0x05EB64, i_05eb64),
    (0x05EB66, i_05eb66),
    (0x05EB6A, i_05eb6a),
    (0x05EB6C, i_05eb6c),
    (0x05EB6E, i_05eb6e),
    (0x05EB72, i_05eb72),
    (0x05EB76, i_05eb76),
    (0x05EB78, i_05eb78),
    (0x05EB7C, i_05eb7c),
    (0x05EB80, i_05eb80),
    (0x05EB84, i_05eb84),
    (0x05EB8A, i_05eb8a),
    (0x05EB8E, i_05eb8e),
    (0x05EB90, i_05eb90),
    (0x05EB94, i_05eb94),
    (0x05EB96, i_05eb96),
    (0x05EB98, i_05eb98),
    (0x05EB9C, i_05eb9c),
    (0x05EFA6, i_05efa6),
    (0x05EFAC, i_05efac),
    (0x05EFAE, i_05efae),
    (0x05EFB2, i_05efb2),
    (0x05EFB6, i_05efb6),
    (0x05EFBA, i_05efba),
    (0x05EFBE, i_05efbe),
    (0x05EFC2, i_05efc2),
    (0x05EFC4, i_05efc4),
    (0x05EFC8, i_05efc8),
    (0x05EFCA, i_05efca),
    (0x05EFD2, i_05efd2),
    (0x05EFD6, i_05efd6),
    (0x05EFDA, i_05efda),
    (0x05EFE0, i_05efe0),
    (0x05EFE6, i_05efe6),
    (0x05EFEC, i_05efec),
    (0x05EFF2, i_05eff2),
    (0x05EFF8, i_05eff8),
    (0x05EFFE, i_05effe),
    (0x05F004, i_05f004),
    (0x05F00A, i_05f00a),
    (0x05F010, i_05f010),
    (0x05F014, i_05f014),
    (0x05F016, i_05f016),
    (0x05F01C, i_05f01c),
    (0x05F020, i_05f020),
    (0x05F026, i_05f026),
    (0x05F02A, i_05f02a),
    (0x05F02E, i_05f02e),
    (0x05F030, i_05f030),
    (0x05F034, i_05f034),
    (0x05F03A, i_05f03a),
    (0x05F03E, i_05f03e),
    (0x05F042, i_05f042),
    (0x05F044, i_05f044),
    (0x05F046, i_05f046),
    (0x05F04C, i_05f04c),
    (0x05F050, i_05f050),
    (0x05F052, i_05f052),
    (0x05F054, i_05f054),
    (0x05F056, i_05f056),
    (0x05F05A, i_05f05a),
    (0x05F060, i_05f060),
    (0x05F062, i_05f062),
    (0x05F064, i_05f064),
    (0x05F068, i_05f068),
    (0x05F06E, i_05f06e),
    (0x05F072, i_05f072),
    (0x05F078, i_05f078),
    (0x05F07A, i_05f07a),
    (0x05F07E, i_05f07e),
    (0x05F084, i_05f084),
    (0x05F088, i_05f088),
    (0x05F08E, i_05f08e),
    (0x05F092, i_05f092),
    (0x05F096, i_05f096),
    (0x05F098, i_05f098),
    (0x05F09E, i_05f09e),
    (0x05F0A2, i_05f0a2),
    (0x05F0A8, i_05f0a8),
    (0x05F0AC, i_05f0ac),
    (0x05F0AE, i_05f0ae),
    (0x05F0B0, i_05f0b0),
    (0x05F0B6, i_05f0b6),
    (0x05F0BA, i_05f0ba),
    (0x05F0BC, i_05f0bc),
    (0x05F0BE, i_05f0be),
    (0x05F0C0, i_05f0c0),
    (0x05F0C4, i_05f0c4),
    (0x05F0CA, i_05f0ca),
    (0x05F0CC, i_05f0cc),
    (0x05F0CE, i_05f0ce),
    (0x05F0D2, i_05f0d2),
    (0x05F0D8, i_05f0d8),
    (0x05F0DC, i_05f0dc),
    (0x05F0E2, i_05f0e2),
    (0x05F0E4, i_05f0e4),
    (0x05F0E8, i_05f0e8),
    (0x05F0EE, i_05f0ee),
    (0x05F0F2, i_05f0f2),
    (0x05F0F8, i_05f0f8),
    (0x05F0FC, i_05f0fc),
    (0x05F100, i_05f100),
    (0x05F102, i_05f102),
    (0x05F108, i_05f108),
    (0x05F10C, i_05f10c),
    (0x05F112, i_05f112),
    (0x05F116, i_05f116),
    (0x05F118, i_05f118),
    (0x05F11A, i_05f11a),
    (0x05F11E, i_05f11e),
    (0x05F122, i_05f122),
    (0x05F128, i_05f128),
    (0x05F12C, i_05f12c),
    (0x05F132, i_05f132),
    (0x05F138, i_05f138),
    (0x05F13C, i_05f13c),
    (0x05F13E, i_05f13e),
    (0x05F140, i_05f140),
    (0x05F142, i_05f142),
    (0x05F146, i_05f146),
    (0x05F14A, i_05f14a),
    (0x05F14E, i_05f14e),
    (0x05F154, i_05f154),
    (0x05F158, i_05f158),
    (0x05F15C, i_05f15c),
    (0x05F162, i_05f162),
    (0x05F164, i_05f164),
    (0x05F16A, i_05f16a),
    (0x05F16E, i_05f16e),
    (0x05F172, i_05f172),
    (0x05F178, i_05f178),
    (0x05F17A, i_05f17a),
    (0x05F180, i_05f180),
    (0x05F186, i_05f186),
    (0x05F18C, i_05f18c),
    (0x05F18E, i_05f18e),
    (0x05F192, i_05f192),
    (0x05F196, i_05f196),
    (0x05F19C, i_05f19c),
    (0x05F1A0, i_05f1a0),
    (0x05F1A6, i_05f1a6),
    (0x05F1AC, i_05f1ac),
    (0x05F1B0, i_05f1b0),
    (0x05F1B6, i_05f1b6),
    (0x05F1B8, i_05f1b8),
    (0x05F1BA, i_05f1ba),
    (0x05F1BC, i_05f1bc),
    (0x05F1BE, i_05f1be),
    (0x05F1C2, i_05f1c2),
    (0x05F1C6, i_05f1c6),
    (0x05F1CA, i_05f1ca),
    (0x05F1CE, i_05f1ce),
    (0x05F1D2, i_05f1d2),
    (0x05F1D8, i_05f1d8),
    (0x05F1DC, i_05f1dc),
    (0x05F1E0, i_05f1e0),
    (0x05F1E6, i_05f1e6),
    (0x05F1E8, i_05f1e8),
    (0x05F1EE, i_05f1ee),
    (0x05F1F2, i_05f1f2),
    (0x05F1F6, i_05f1f6),
    (0x05F1FC, i_05f1fc),
    (0x05F1FE, i_05f1fe),
    (0x05F204, i_05f204),
    (0x05F20A, i_05f20a),
    (0x05F210, i_05f210),
    (0x05F212, i_05f212),
    (0x05F216, i_05f216),
    (0x05F21C, i_05f21c),
    (0x05F222, i_05f222),
    (0x05F228, i_05f228),
    (0x05F22E, i_05f22e),
    (0x05F234, i_05f234),
    (0x05F23A, i_05f23a),
    (0x05F240, i_05f240),
    (0x05F246, i_05f246),
    (0x05F24A, i_05f24a),
    (0x05F250, i_05f250),
    (0x05F254, i_05f254),
    (0x05F256, i_05f256),
    (0x05F25C, i_05f25c),
    (0x05F260, i_05f260),
    (0x05F266, i_05f266),
    (0x05F26A, i_05f26a),
    (0x05F26E, i_05f26e),
    (0x05F274, i_05f274),
    (0x05F278, i_05f278),
    (0x05F27C, i_05f27c),
    (0x05F282, i_05f282),
    (0x05F286, i_05f286),
    (0x05F28A, i_05f28a),
    (0x05F290, i_05f290),
    (0x05F294, i_05f294),
    (0x05F298, i_05f298),
    (0x05F29E, i_05f29e),
    (0x05F2A2, i_05f2a2),
    (0x05F2A6, i_05f2a6),
    (0x05F2AA, i_05f2aa),
    (0x05F2B0, i_05f2b0),
    (0x05F2B4, i_05f2b4),
    (0x05F2BA, i_05f2ba),
    (0x05F2BE, i_05f2be),
    (0x05F2C2, i_05f2c2),
    (0x05F2C8, i_05f2c8),
    (0x05F2CC, i_05f2cc),
    (0x05F2D0, i_05f2d0),
    (0x05F2D6, i_05f2d6),
    (0x05F2DA, i_05f2da),
    (0x05F2DE, i_05f2de),
    (0x05F2E4, i_05f2e4),
    (0x05F2E8, i_05f2e8),
    (0x05F2EE, i_05f2ee),
    (0x05F2F2, i_05f2f2),
    (0x05F2F6, i_05f2f6),
    (0x05F2FA, i_05f2fa),
    (0x05F2FE, i_05f2fe),
    (0x05F302, i_05f302),
    (0x05F308, i_05f308),
    (0x05F30A, i_05f30a),
    (0x05F30E, i_05f30e),
    (0x05F312, i_05f312),
    (0x05F316, i_05f316),
    (0x05F31A, i_05f31a),
    (0x05F31E, i_05f31e),
    (0x05F320, i_05f320),
    (0x05F322, i_05f322),
    (0x05F324, i_05f324),
    (0x05F326, i_05f326),
    (0x05F328, i_05f328),
    (0x05F32A, i_05f32a),
    (0x05F32C, i_05f32c),
    (0x05F32E, i_05f32e),
    (0x05F330, i_05f330),
    (0x05F332, i_05f332),
    (0x05F334, i_05f334),
    (0x05F336, i_05f336),
    (0x05F338, i_05f338),
    (0x05F33A, i_05f33a),
    (0x05F33C, i_05f33c),
    (0x05F33E, i_05f33e),
    (0x05F340, i_05f340),
    (0x05F342, i_05f342),
    (0x05F344, i_05f344),
    (0x05F346, i_05f346),
    (0x05F348, i_05f348),
    (0x05F34A, i_05f34a),
    (0x05F34C, i_05f34c),
    (0x05F34E, i_05f34e),
    (0x05F350, i_05f350),
    (0x05F352, i_05f352),
    (0x05F356, i_05f356),
    (0x05F35A, i_05f35a),
    (0x05F378, i_05f378),
    (0x05F37C, i_05f37c),
    (0x05F37E, i_05f37e),
    (0x05F382, i_05f382),
    (0x05F384, i_05f384),
    (0x05F38A, i_05f38a),
    (0x05F38E, i_05f38e),
    (0x05F394, i_05f394),
    (0x05F39A, i_05f39a),
    (0x05F3A0, i_05f3a0),
    (0x05F3A6, i_05f3a6),
    (0x05F3AC, i_05f3ac),
    (0x05F3B2, i_05f3b2),
    (0x05F3B8, i_05f3b8),
    (0x05F3BE, i_05f3be),
    (0x05F3C4, i_05f3c4),
    (0x05F3C8, i_05f3c8),
    (0x05F3CA, i_05f3ca),
    (0x05F3D0, i_05f3d0),
    (0x05F3D6, i_05f3d6),
    (0x05F3DA, i_05f3da),
    (0x05F3DE, i_05f3de),
    (0x05F3E0, i_05f3e0),
    (0x05F3E6, i_05f3e6),
    (0x05F3EA, i_05f3ea),
    (0x05F3EE, i_05f3ee),
    (0x05F3F0, i_05f3f0),
    (0x05F3F2, i_05f3f2),
    (0x05F3F8, i_05f3f8),
    (0x05F3FC, i_05f3fc),
    (0x05F3FE, i_05f3fe),
    (0x05F404, i_05f404),
    (0x05F40A, i_05f40a),
    (0x05F410, i_05f410),
    (0x05F416, i_05f416),
    (0x05F41C, i_05f41c),
    (0x05F422, i_05f422),
    (0x05F426, i_05f426),
    (0x05F42C, i_05f42c),
    (0x05F432, i_05f432),
    (0x05F436, i_05f436),
    (0x05F43C, i_05f43c),
    (0x05F440, i_05f440),
    (0x05F444, i_05f444),
    (0x05F44A, i_05f44a),
    (0x05F450, i_05f450),
    (0x05F454, i_05f454),
    (0x05F456, i_05f456),
    (0x05F45A, i_05f45a),
    (0x05F45E, i_05f45e),
    (0x05F462, i_05f462),
    (0x05F468, i_05f468),
    (0x05F46C, i_05f46c),
    (0x05F470, i_05f470),
    (0x05F472, i_05f472),
    (0x05F476, i_05f476),
    (0x05F47A, i_05f47a),
    (0x05F480, i_05f480),
    (0x05F482, i_05f482),
    (0x05F488, i_05f488),
    (0x05F48C, i_05f48c),
    (0x05F48E, i_05f48e),
    (0x05F494, i_05f494),
    (0x05F49A, i_05f49a),
    (0x05F4A0, i_05f4a0),
    (0x05F4A6, i_05f4a6),
    (0x05F4AC, i_05f4ac),
    (0x05F4B0, i_05f4b0),
    (0x05F4B4, i_05f4b4),
    (0x05F4BA, i_05f4ba),
    (0x05F4BE, i_05f4be),
    (0x05F4C4, i_05f4c4),
    (0x05F4C8, i_05f4c8),
    (0x05F4CC, i_05f4cc),
    (0x05F4CE, i_05f4ce),
    (0x05F4D4, i_05f4d4),
    (0x05F4D8, i_05f4d8),
    (0x05F4DE, i_05f4de),
    (0x05F4E2, i_05f4e2),
    (0x05F4E4, i_05f4e4),
    (0x05F4E8, i_05f4e8),
    (0x05F4EE, i_05f4ee),
    (0x05F4F2, i_05f4f2),
    (0x05F4F6, i_05f4f6),
    (0x05F4F8, i_05f4f8),
    (0x05F4FC, i_05f4fc),
    (0x05F500, i_05f500),
    (0x05F506, i_05f506),
    (0x05F508, i_05f508),
    (0x05F50C, i_05f50c),
    (0x05F512, i_05f512),
    (0x05F518, i_05f518),
    (0x05F51E, i_05f51e),
    (0x05F524, i_05f524),
    (0x05F52A, i_05f52a),
    (0x05F530, i_05f530),
    (0x05F536, i_05f536),
    (0x05F53C, i_05f53c),
    (0x05F542, i_05f542),
    (0x05F548, i_05f548),
    (0x05F54E, i_05f54e),
    (0x05F552, i_05f552),
    (0x05F558, i_05f558),
    (0x05F55C, i_05f55c),
    (0x05F55E, i_05f55e),
    (0x05F564, i_05f564),
    (0x05F568, i_05f568),
    (0x05F56C, i_05f56c),
    (0x05F56E, i_05f56e),
    (0x05F574, i_05f574),
    (0x05F578, i_05f578),
    (0x05F57E, i_05f57e),
    (0x05F584, i_05f584),
    (0x05F58A, i_05f58a),
    (0x05F590, i_05f590),
    (0x05F594, i_05f594),
    (0x05F598, i_05f598),
    (0x05F59A, i_05f59a),
    (0x05F59E, i_05f59e),
    (0x05F5A2, i_05f5a2),
    (0x05F5A8, i_05f5a8),
    (0x05F5AA, i_05f5aa),
    (0x05F5B0, i_05f5b0),
    (0x05F5B4, i_05f5b4),
    (0x05F5B6, i_05f5b6),
    (0x05F5BA, i_05f5ba),
    (0x05F5C0, i_05f5c0),
    (0x05F5C8, i_05f5c8),
    (0x05F5CE, i_05f5ce),
    (0x05F5D4, i_05f5d4),
    (0x05F5D8, i_05f5d8),
    (0x05F5DA, i_05f5da),
    (0x05F5E0, i_05f5e0),
    (0x05F5E6, i_05f5e6),
    (0x05F5EA, i_05f5ea),
    (0x05F5EE, i_05f5ee),
    (0x05F5F2, i_05f5f2),
    (0x05F5F8, i_05f5f8),
    (0x05F5FA, i_05f5fa),
    (0x05F600, i_05f600),
    (0x05F604, i_05f604),
    (0x05F60A, i_05f60a),
    (0x05F610, i_05f610),
    (0x05F616, i_05f616),
    (0x05F61C, i_05f61c),
    (0x05F622, i_05f622),
    (0x05F628, i_05f628),
    (0x05F62E, i_05f62e),
    (0x05F634, i_05f634),
    (0x05F63A, i_05f63a),
    (0x05F640, i_05f640),
    (0x05F646, i_05f646),
    (0x05F64A, i_05f64a),
    (0x05F64C, i_05f64c),
    (0x05F652, i_05f652),
    (0x05F658, i_05f658),
    (0x05F65C, i_05f65c),
    (0x05F660, i_05f660),
    (0x05F666, i_05f666),
    (0x05F668, i_05f668),
    (0x05F66C, i_05f66c),
    (0x05F672, i_05f672),
    (0x05F678, i_05f678),
    (0x05F67E, i_05f67e),
    (0x05F684, i_05f684),
    (0x05F68A, i_05f68a),
    (0x05F690, i_05f690),
    (0x05F694, i_05f694),
    (0x05F696, i_05f696),
    (0x05F69C, i_05f69c),
    (0x05F6A2, i_05f6a2),
    (0x05F6A6, i_05f6a6),
    (0x05F6AA, i_05f6aa),
    (0x05F6B0, i_05f6b0),
    (0x05F6B4, i_05f6b4),
    (0x05F6B8, i_05f6b8),
    (0x05F6BC, i_05f6bc),
    (0x05F6C2, i_05f6c2),
    (0x05F6C4, i_05f6c4),
    (0x05F6C8, i_05f6c8),
    (0x05F6CE, i_05f6ce),
    (0x05F6D2, i_05f6d2),
    (0x05F6D8, i_05f6d8),
    (0x05F6DC, i_05f6dc),
    (0x05F6E0, i_05f6e0),
    (0x05F6E4, i_05f6e4),
    (0x05F6EA, i_05f6ea),
    (0x05F6F0, i_05f6f0),
    (0x05F6F6, i_05f6f6),
    (0x05F6FC, i_05f6fc),
    (0x05F702, i_05f702),
    (0x05F706, i_05f706),
    (0x05F70A, i_05f70a),
    (0x05F70E, i_05f70e),
    (0x05F714, i_05f714),
    (0x05F716, i_05f716),
    (0x05F71C, i_05f71c),
    (0x05F720, i_05f720),
    (0x05F722, i_05f722),
    (0x05F728, i_05f728),
    (0x05F72E, i_05f72e),
    (0x05F732, i_05f732),
    (0x05F736, i_05f736),
    (0x05F738, i_05f738),
    (0x05F73C, i_05f73c),
    (0x05F740, i_05f740),
    (0x05F744, i_05f744),
    (0x05F74A, i_05f74a),
    (0x05F750, i_05f750),
    (0x05F752, i_05f752),
    (0x05F758, i_05f758),
    (0x05F75E, i_05f75e),
    (0x05F764, i_05f764),
    (0x05F768, i_05f768),
    (0x05F76E, i_05f76e),
    (0x05F772, i_05f772),
    (0x05F774, i_05f774),
    (0x05F778, i_05f778),
    (0x05F77C, i_05f77c),
    (0x05F77E, i_05f77e),
    (0x05F784, i_05f784),
    (0x05F78A, i_05f78a),
    (0x05F790, i_05f790),
    (0x05F794, i_05f794),
    (0x05F798, i_05f798),
    (0x05F79E, i_05f79e),
    (0x05F7A2, i_05f7a2),
    (0x05F7A8, i_05f7a8),
    (0x05F7AC, i_05f7ac),
    (0x05F7B2, i_05f7b2),
    (0x05F7B4, i_05f7b4),
    (0x05F7BA, i_05f7ba),
    (0x05F7C0, i_05f7c0),
    (0x05F7C6, i_05f7c6),
    (0x05F7CA, i_05f7ca),
    (0x05F7CE, i_05f7ce),
    (0x05F7D2, i_05f7d2),
    (0x05F7D4, i_05f7d4),
    (0x05F7DA, i_05f7da),
    (0x05F7DE, i_05f7de),
    (0x05F7E2, i_05f7e2),
    (0x05F7E6, i_05f7e6),
    (0x05F7EC, i_05f7ec),
    (0x05F7F2, i_05f7f2),
    (0x05F7F4, i_05f7f4),
    (0x05F7FA, i_05f7fa),
    (0x05F7FE, i_05f7fe),
    (0x05F804, i_05f804),
    (0x05F808, i_05f808),
    (0x05F80A, i_05f80a),
    (0x05F80E, i_05f80e),
    (0x05F814, i_05f814),
    (0x05F81A, i_05f81a),
    (0x05F820, i_05f820),
    (0x05F826, i_05f826),
    (0x05F82A, i_05f82a),
    (0x05F82C, i_05f82c),
    (0x05F832, i_05f832),
    (0x05F838, i_05f838),
    (0x05F83C, i_05f83c),
    (0x05F840, i_05f840),
    (0x05F842, i_05f842),
    (0x05F848, i_05f848),
    (0x05F84C, i_05f84c),
    (0x05F850, i_05f850),
    (0x05F856, i_05f856),
    (0x05F85A, i_05f85a),
    (0x05F860, i_05f860),
    (0x05F864, i_05f864),
    (0x05F86A, i_05f86a),
    (0x05F86E, i_05f86e),
    (0x05F874, i_05f874),
    (0x05F876, i_05f876),
    (0x05F87C, i_05f87c),
    (0x05F882, i_05f882),
    (0x05F888, i_05f888),
    (0x05F88E, i_05f88e),
    (0x05F892, i_05f892),
    (0x05F896, i_05f896),
    (0x05F89A, i_05f89a),
    (0x05F8A0, i_05f8a0),
    (0x05F8A6, i_05f8a6),
    (0x05F8A8, i_05f8a8),
    (0x05F8AE, i_05f8ae),
    (0x05F8B4, i_05f8b4),
    (0x05F8BA, i_05f8ba),
    (0x05F8BE, i_05f8be),
    (0x05F8C0, i_05f8c0),
    (0x05F8C6, i_05f8c6),
    (0x05F8CC, i_05f8cc),
    (0x05F8D0, i_05f8d0),
    (0x05F8D6, i_05f8d6),
    (0x05F8D8, i_05f8d8),
    (0x05F8DE, i_05f8de),
    (0x05F8E4, i_05f8e4),
    (0x05F8EA, i_05f8ea),
    (0x05F8F0, i_05f8f0),
    (0x05F8F4, i_05f8f4),
    (0x05F8F8, i_05f8f8),
    (0x05F8FE, i_05f8fe),
    (0x05F904, i_05f904),
    (0x05F908, i_05f908),
    (0x05F90C, i_05f90c),
    (0x05F910, i_05f910),
    (0x05F916, i_05f916),
    (0x05F91C, i_05f91c),
    (0x05F91E, i_05f91e),
    (0x05F922, i_05f922),
    (0x05F926, i_05f926),
    (0x05F92A, i_05f92a),
    (0x05F92E, i_05f92e),
    (0x05F932, i_05f932),
    (0x05F938, i_05f938),
    (0x05F93A, i_05f93a),
    (0x05F93E, i_05f93e),
    (0x05F944, i_05f944),
    (0x05F94A, i_05f94a),
    (0x05F94C, i_05f94c),
    (0x05F950, i_05f950),
    (0x05F954, i_05f954),
    (0x05F958, i_05f958),
    (0x05F95C, i_05f95c),
    (0x05F960, i_05f960),
    (0x05F962, i_05f962),
    (0x05F966, i_05f966),
    (0x05F96A, i_05f96a),
    (0x05F96E, i_05f96e),
    (0x05F972, i_05f972),
    (0x05F976, i_05f976),
    (0x05F97A, i_05f97a),
    (0x05F97C, i_05f97c),
    (0x05F980, i_05f980),
    (0x05F984, i_05f984),
    (0x05F988, i_05f988),
    (0x05F98A, i_05f98a),
    (0x05F98E, i_05f98e),
    (0x05F990, i_05f990),
    (0x05F996, i_05f996),
    (0x05F99C, i_05f99c),
    (0x05F9A0, i_05f9a0),
    (0x05F9A4, i_05f9a4),
    (0x05F9A6, i_05f9a6),
    (0x05F9AA, i_05f9aa),
    (0x05F9AC, i_05f9ac),
    (0x05F9B2, i_05f9b2),
    (0x05F9B8, i_05f9b8),
    (0x05F9BC, i_05f9bc),
    (0x05F9C0, i_05f9c0),
    (0x05F9C2, i_05f9c2),
    (0x05F9C6, i_05f9c6),
    (0x05F9C8, i_05f9c8),
    (0x05F9CE, i_05f9ce),
    (0x05F9D4, i_05f9d4),
    (0x05F9D8, i_05f9d8),
    (0x05F9DC, i_05f9dc),
    (0x05F9DE, i_05f9de),
    (0x05F9E2, i_05f9e2),
    (0x05F9E4, i_05f9e4),
    (0x05F9E8, i_05f9e8),
    (0x05F9EC, i_05f9ec),
    (0x05F9F0, i_05f9f0),
    (0x05F9F4, i_05f9f4),
    (0x05F9F8, i_05f9f8),
    (0x05F9FA, i_05f9fa),
    (0x05F9FE, i_05f9fe),
    (0x05FA00, i_05fa00),
    (0x05FA04, i_05fa04),
    (0x05FA0A, i_05fa0a),
    (0x05FA10, i_05fa10),
    (0x05FA16, i_05fa16),
    (0x05FA1A, i_05fa1a),
    (0x05FA20, i_05fa20),
    (0x05FA26, i_05fa26),
    (0x05FA2C, i_05fa2c),
    (0x05FA32, i_05fa32),
    (0x05FA38, i_05fa38),
    (0x05FA3E, i_05fa3e),
    (0x05FA42, i_05fa42),
    (0x05FA48, i_05fa48),
    (0x05FA4E, i_05fa4e),
    (0x05FA52, i_05fa52),
    (0x05FA54, i_05fa54),
    (0x05FA56, i_05fa56),
    (0x05FA5C, i_05fa5c),
    (0x05FA62, i_05fa62),
    (0x05FA68, i_05fa68),
    (0x05FA6C, i_05fa6c),
    (0x05FA70, i_05fa70),
    (0x05FA72, i_05fa72),
    (0x05FA76, i_05fa76),
    (0x05FA7A, i_05fa7a),
    (0x05FA7E, i_05fa7e),
    (0x05FA80, i_05fa80),
    (0x05FA86, i_05fa86),
    (0x05FA8A, i_05fa8a),
    (0x05FA90, i_05fa90),
    (0x05FA96, i_05fa96),
    (0x05FA9A, i_05fa9a),
    (0x05FA9E, i_05fa9e),
    (0x05FAA2, i_05faa2),
    (0x05FAA4, i_05faa4),
    (0x05FAA6, i_05faa6),
    (0x05FAAC, i_05faac),
    (0x05FAB2, i_05fab2),
    (0x05FAB8, i_05fab8),
    (0x05FABC, i_05fabc),
    (0x05FABE, i_05fabe),
    (0x05FAC4, i_05fac4),
    (0x05FACA, i_05faca),
    (0x05FACE, i_05face),
    (0x05FAD4, i_05fad4),
    (0x05FAD8, i_05fad8),
    (0x05FADA, i_05fada),
    (0x05FAE0, i_05fae0),
    (0x05FAE4, i_05fae4),
    (0x05FAEA, i_05faea),
    (0x05FAEC, i_05faec),
    (0x05FAF2, i_05faf2),
    (0x05FAF8, i_05faf8),
    (0x05FAFE, i_05fafe),
    (0x05FB04, i_05fb04),
    (0x05FB08, i_05fb08),
    (0x05FB0E, i_05fb0e),
    (0x05FB14, i_05fb14),
    (0x05FB18, i_05fb18),
    (0x05FB1C, i_05fb1c),
    (0x05FB20, i_05fb20),
    (0x05FB22, i_05fb22),
    (0x05FB24, i_05fb24),
    (0x05FB2A, i_05fb2a),
    (0x05FB30, i_05fb30),
    (0x05FB36, i_05fb36),
    (0x05FB3A, i_05fb3a),
    (0x05FB3C, i_05fb3c),
    (0x05FB42, i_05fb42),
    (0x05FB48, i_05fb48),
    (0x05FB4E, i_05fb4e),
    (0x05FB52, i_05fb52),
    (0x05FB56, i_05fb56),
    (0x05FB58, i_05fb58),
    (0x05FB5E, i_05fb5e),
    (0x05FB62, i_05fb62),
    (0x05FB68, i_05fb68),
    (0x05FB6C, i_05fb6c),
    (0x05FB72, i_05fb72),
    (0x05FB78, i_05fb78),
    (0x05FB7C, i_05fb7c),
    (0x05FB80, i_05fb80),
    (0x05FB84, i_05fb84),
    (0x05FB86, i_05fb86),
    (0x05FB88, i_05fb88),
    (0x05FB8C, i_05fb8c),
    (0x05FB8E, i_05fb8e),
    (0x05FB94, i_05fb94),
    (0x05FB98, i_05fb98),
    (0x05FB9E, i_05fb9e),
    (0x05FBA0, i_05fba0),
    (0x05FBA4, i_05fba4),
    (0x05FBAA, i_05fbaa),
    (0x05FBB0, i_05fbb0),
    (0x05FBB6, i_05fbb6),
    (0x05FBBC, i_05fbbc),
    (0x05FBC2, i_05fbc2),
    (0x05FBC8, i_05fbc8),
    (0x05FBCC, i_05fbcc),
    (0x05FBD2, i_05fbd2),
    (0x05FBD8, i_05fbd8),
    (0x05FBDE, i_05fbde),
    (0x05FBE2, i_05fbe2),
    (0x05FBE4, i_05fbe4),
    (0x05FBE6, i_05fbe6),
    (0x05FBEC, i_05fbec),
    (0x05FBF2, i_05fbf2),
    (0x05FBF6, i_05fbf6),
    (0x05FBFC, i_05fbfc),
    (0x05FC00, i_05fc00),
    (0x05FC04, i_05fc04),
    (0x05FC06, i_05fc06),
    (0x05FC0C, i_05fc0c),
    (0x05FC10, i_05fc10),
    (0x05FC14, i_05fc14),
    (0x05FC16, i_05fc16),
    (0x05FC1A, i_05fc1a),
    (0x05FC1E, i_05fc1e),
    (0x05FC22, i_05fc22),
    (0x05FC24, i_05fc24),
    (0x05FC26, i_05fc26),
    (0x05FC2C, i_05fc2c),
    (0x05FC32, i_05fc32),
    (0x05FC36, i_05fc36),
    (0x05FC38, i_05fc38),
    (0x05FC3E, i_05fc3e),
    (0x05FC42, i_05fc42),
    (0x05FC48, i_05fc48),
    (0x05FC4C, i_05fc4c),
    (0x05FC52, i_05fc52),
    (0x05FC58, i_05fc58),
    (0x05FC5E, i_05fc5e),
    (0x05FC62, i_05fc62),
    (0x05FC66, i_05fc66),
    (0x05FC68, i_05fc68),
    (0x05FC6C, i_05fc6c),
    (0x05FC70, i_05fc70),
    (0x05FC74, i_05fc74),
    (0x05FC76, i_05fc76),
    (0x05FC78, i_05fc78),
    (0x05FC7C, i_05fc7c),
    (0x05FC82, i_05fc82),
    (0x05FC86, i_05fc86),
    (0x05FC8C, i_05fc8c),
    (0x05FC90, i_05fc90),
    (0x05FC96, i_05fc96),
    (0x05FC9C, i_05fc9c),
    (0x05FCA2, i_05fca2),
    (0x05FCA6, i_05fca6),
    (0x05FCA8, i_05fca8),
    (0x05FCAE, i_05fcae),
    (0x05FCB4, i_05fcb4),
    (0x05FCB8, i_05fcb8),
    (0x05FCBE, i_05fcbe),
    (0x05FCC4, i_05fcc4),
    (0x05FCCA, i_05fcca),
    (0x05FCD0, i_05fcd0),
    (0x05FCD4, i_05fcd4),
    (0x05FCD6, i_05fcd6),
    (0x05FCDA, i_05fcda),
    (0x05FCDE, i_05fcde),
    (0x05FCE2, i_05fce2),
    (0x05FCE4, i_05fce4),
    (0x05FCE6, i_05fce6),
    (0x05FCEA, i_05fcea),
    (0x05FCF0, i_05fcf0),
    (0x05FCF4, i_05fcf4),
    (0x05FCFA, i_05fcfa),
    (0x05FCFE, i_05fcfe),
    (0x05FD00, i_05fd00),
    (0x05FD06, i_05fd06),
    (0x05FD08, i_05fd08),
    (0x05FD0E, i_05fd0e),
    (0x05FD12, i_05fd12),
    (0x05FD16, i_05fd16),
    (0x05FD1A, i_05fd1a),
    (0x05FD1E, i_05fd1e),
    (0x05FD24, i_05fd24),
    (0x05FD28, i_05fd28),
    (0x05FD2C, i_05fd2c),
    (0x05FD30, i_05fd30),
    (0x05FD34, i_05fd34),
    (0x05FD38, i_05fd38),
    (0x05FD3A, i_05fd3a),
    (0x05FD3E, i_05fd3e),
    (0x05FD40, i_05fd40),
    (0x05FD46, i_05fd46),
    (0x05FD4C, i_05fd4c),
    (0x05FD50, i_05fd50),
    (0x05FD54, i_05fd54),
    (0x05FD56, i_05fd56),
    (0x05FD5A, i_05fd5a),
    (0x05FD5C, i_05fd5c),
    (0x05FD60, i_05fd60),
    (0x05FD64, i_05fd64),
    (0x05FD68, i_05fd68),
    (0x05FD6C, i_05fd6c),
    (0x05FD70, i_05fd70),
    (0x05FD72, i_05fd72),
    (0x05FD76, i_05fd76),
    (0x05FD78, i_05fd78),
    (0x05FD7E, i_05fd7e),
    (0x05FD84, i_05fd84),
    (0x05FD88, i_05fd88),
    (0x05FD8E, i_05fd8e),
    (0x05FD94, i_05fd94),
    (0x05FD98, i_05fd98),
    (0x05FD9E, i_05fd9e),
    (0x05FDA4, i_05fda4),
    (0x05FDAA, i_05fdaa),
    (0x05FDAE, i_05fdae),
    (0x05FDB4, i_05fdb4),
    (0x05FDB6, i_05fdb6),
    (0x05FDBA, i_05fdba),
    (0x05FDC0, i_05fdc0),
    (0x05FDC6, i_05fdc6),
    (0x05FDCC, i_05fdcc),
    (0x05FDD0, i_05fdd0),
    (0x05FDD4, i_05fdd4),
    (0x05FDD8, i_05fdd8),
    (0x05FDDE, i_05fdde),
    (0x05FDE4, i_05fde4),
    (0x05FDEA, i_05fdea),
    (0x05FDEE, i_05fdee),
    (0x05FDF0, i_05fdf0),
    (0x05FDF6, i_05fdf6),
    (0x05FDFC, i_05fdfc),
    (0x05FE00, i_05fe00),
    (0x05FE04, i_05fe04),
    (0x05FE06, i_05fe06),
    (0x05FE0C, i_05fe0c),
    (0x05FE10, i_05fe10),
    (0x05FE14, i_05fe14),
    (0x05FE16, i_05fe16),
    (0x05FE1A, i_05fe1a),
    (0x05FE1E, i_05fe1e),
    (0x05FE24, i_05fe24),
    (0x05FE26, i_05fe26),
    (0x05FE2C, i_05fe2c),
    (0x05FE32, i_05fe32),
    (0x05FE36, i_05fe36),
    (0x05FE3C, i_05fe3c),
    (0x05FE42, i_05fe42),
    (0x05FE46, i_05fe46),
    (0x05FE48, i_05fe48),
    (0x05FE4E, i_05fe4e),
    (0x05FE54, i_05fe54),
    (0x05FE5A, i_05fe5a),
    (0x05FE5E, i_05fe5e),
    (0x05FE64, i_05fe64),
    (0x05FE68, i_05fe68),
    (0x05FE6E, i_05fe6e),
    (0x05FE72, i_05fe72),
    (0x05FE76, i_05fe76),
    (0x05FE78, i_05fe78),
    (0x05FE7E, i_05fe7e),
    (0x05FE82, i_05fe82),
    (0x05FE86, i_05fe86),
    (0x05FE88, i_05fe88),
    (0x05FE8C, i_05fe8c),
    (0x05FE90, i_05fe90),
    (0x05FE94, i_05fe94),
    (0x05FE96, i_05fe96),
    (0x05FE9A, i_05fe9a),
    (0x05FE9E, i_05fe9e),
    (0x05FEA0, i_05fea0),
    (0x05FEA6, i_05fea6),
    (0x05FEAA, i_05feaa),
    (0x05FEAE, i_05feae),
    (0x05FEB0, i_05feb0),
    (0x05FEB4, i_05feb4),
    (0x05FEB8, i_05feb8),
    (0x05FEBE, i_05febe),
    (0x05FEC0, i_05fec0),
    (0x05FEC6, i_05fec6),
    (0x05FECA, i_05feca),
    (0x05FECE, i_05fece),
    (0x05FED0, i_05fed0),
    (0x05FED4, i_05fed4),
    (0x05FEDA, i_05feda),
    (0x05FEE0, i_05fee0),
    (0x05FEE4, i_05fee4),
    (0x05FEE6, i_05fee6),
    (0x05FEEC, i_05feec),
    (0x05FEF2, i_05fef2),
    (0x05FEF6, i_05fef6),
    (0x05FEFA, i_05fefa),
    (0x05FEFC, i_05fefc),
    (0x05FF02, i_05ff02),
    (0x05FF06, i_05ff06),
    (0x05FF0A, i_05ff0a),
    (0x05FF0C, i_05ff0c),
    (0x05FF10, i_05ff10),
    (0x05FF14, i_05ff14),
    (0x05FF1A, i_05ff1a),
    (0x05FF1C, i_05ff1c),
    (0x05FF22, i_05ff22),
    (0x05FF28, i_05ff28),
    (0x05FF2C, i_05ff2c),
    (0x05FF2E, i_05ff2e),
    (0x05FF34, i_05ff34),
    (0x05FF3A, i_05ff3a),
    (0x05FF3E, i_05ff3e),
    (0x05FF42, i_05ff42),
    (0x05FF44, i_05ff44),
    (0x05FF4A, i_05ff4a),
    (0x05FF4E, i_05ff4e),
    (0x05FF52, i_05ff52),
    (0x05FF54, i_05ff54),
    (0x05FF58, i_05ff58),
    (0x05FF5C, i_05ff5c),
    (0x05FF62, i_05ff62),
    (0x05FF64, i_05ff64),
    (0x05FF6A, i_05ff6a),
    (0x05FF6E, i_05ff6e),
    (0x05FF74, i_05ff74),
    (0x05FF76, i_05ff76),
    (0x05FF7A, i_05ff7a),
    (0x05FF80, i_05ff80),
    (0x05FF86, i_05ff86),
    (0x05FF8C, i_05ff8c),
    (0x05FF90, i_05ff90),
    (0x05FF94, i_05ff94),
    (0x05FF98, i_05ff98),
    (0x05FF9E, i_05ff9e),
    (0x05FFA4, i_05ffa4),
    (0x05FFAA, i_05ffaa),
    (0x05FFB0, i_05ffb0),
    (0x05FFB6, i_05ffb6),
    (0x05FFBA, i_05ffba),
    (0x05FFBC, i_05ffbc),
    (0x05FFC2, i_05ffc2),
    (0x05FFC8, i_05ffc8),
    (0x05FFCE, i_05ffce),
    (0x05FFD2, i_05ffd2),
    (0x05FFD6, i_05ffd6),
    (0x05FFDA, i_05ffda),
    (0x05FFDE, i_05ffde),
    (0x05FFE2, i_05ffe2),
    (0x05FFE6, i_05ffe6),
    (0x05FFE8, i_05ffe8),
    (0x05FFEE, i_05ffee),
    (0x05FFF2, i_05fff2),
    (0x05FFF6, i_05fff6),
    (0x05FFF8, i_05fff8),
    (0x05FFFC, i_05fffc),
];
