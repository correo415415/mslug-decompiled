// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $05A872: move.w d2, (a0)
pub fn i_05a872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A874;
    cpu.ir = 0x3082;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $05A874: dbra d0, $5a868
pub fn i_05a874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A878;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5A878; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5A878; return 14; }
    cpu.pc = 0x5A868;
    10
}
/// $05A878: clr.w $10e1fa.l
pub fn i_05a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A87E;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1FA) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1FA, r2 as u16);
    16
}
/// $05A87E: clr.w $10e1f6.l
pub fn i_05a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A884;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1F6) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1F6, r2 as u16);
    16
}
/// $05A884: clr.w $10e1f8.l
pub fn i_05a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A88A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E1F8) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E1F8, r2 as u16);
    16
}
/// $05A88A: lea.l $108080.l, a5
pub fn i_05a88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A890;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A890: clr.b $616c(a5)
pub fn i_05a890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A894;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0x616C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A894: clr.w $6168(a5)
pub fn i_05a894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A898;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x6168);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A898: clr.w $616a(a5)
pub fn i_05a898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A89C;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x616A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A89C: lea.l $614c(a5), a0
pub fn i_05a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8A0;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x614C);
    cpu.a[0] = ad1;
    16
}
/// $05A8A0: clr.b $8(a0)
pub fn i_05a8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8A4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A8A4: move.w #$ffff, $4(a0)
pub fn i_05a8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8AA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A8AA: lea.l $6158(a5), a0
pub fn i_05a8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8AE;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x6158);
    cpu.a[0] = ad1;
    16
}
/// $05A8AE: clr.b $8(a0)
pub fn i_05a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8B2;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05A8B2: move.w #$ffff, $4(a0)
pub fn i_05a8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8B8;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A8B8: rts
pub fn i_05a8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A8BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== VRAM_FixAutoclear_Reset_05a9ba  ($05A9BA, 28 B, mission/result_ending_gunner_walker_05934e.s)
/// $05A9BA: lea.l $108080.l, a5
pub fn i_05a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9C0;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A9C0: clr.b $10e1ec.l
pub fn i_05a9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9C6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E1EC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E1EC, r2 as u8);
    16
}
/// $05A9C6: clr.w $4254(a5)
pub fn i_05a9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9CA;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x4254);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A9CA: clr.w $6148(a5)
pub fn i_05a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9CE;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0x6148);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05A9CE: move.w #$348, $614a(a5)
pub fn i_05a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9D4;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x348;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05A9D4: rts
pub fn i_05a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_Dispatch_05A9D6  ($05A9D6, 192 B, video/sprite_dispatch_dual_05a9d6.s)
/// $05A9D6: andi.b #$3, d5
pub fn i_05a9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9DA;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9DA: ori.b #$4, d5
pub fn i_05a9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9DE;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9DE: bra.w $5a9e6
pub fn i_05a9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A9E6;
    10
}
/// $05A9E2: andi.b #$3, d5
pub fn i_05a9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9E6;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05A9E6: jsr $5b1b2(pc)
pub fn i_05a9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5A9EA);
    cpu.pc = 0x5B1B2;
    16
}
/// $05A9EA: move a1, usp
pub fn i_05a9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9EC;
    cpu.ir = 0x4E61;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.usp = cpu.a[1];
    4
}
/// $05A9EC: lea.l $108080.l, a5
pub fn i_05a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05A9F2: move.l a0, d7
pub fn i_05a9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F4;
    cpu.ir = 0x2E08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A9F4: bmi.w $5aa92
pub fn i_05a9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9F8;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05A9F8: swap d1
pub fn i_05a9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9FA;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05A9FA: move.w $4254(a5), d1
pub fn i_05a9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5A9FE;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05A9FE: cmpi.w #$11d0, d1
pub fn i_05a9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA02;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x11D0, Size::Word, false);
    8
}
/// $05AA02: bcc.w $5aa92
pub fn i_05aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05AA06: move.w $6148(a5), d6
pub fn i_05aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA0A;
    cpu.ir = 0x3C2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA0A: move.w $614a(a5), d7
pub fn i_05aa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA0E;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA0E: cmp.w d7, d6
pub fn i_05aa0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA10;
    cpu.ir = 0xBC47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05AA10: bcc.w $5aa92
pub fn i_05aa10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AA92; return 10; }
    8
}
/// $05AA14: btst.b #$2, d5
pub fn i_05aa14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA18;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05AA18: bne.w $5aa2e
pub fn i_05aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA1C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AA2E; return 10; }
    8
}
/// $05AA1C: lea.l $5428(a5), a4
pub fn i_05aa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA20;
    cpu.ir = 0x49ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[4] = ad1;
    16
}
/// $05AA20: move.l d1, (a4, d6.w)
pub fn i_05aa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA24;
    cpu.ir = 0x2981;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05AA24: addq.w #$4, d6
pub fn i_05aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA26;
    cpu.ir = 0x5846;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AA26: move.w d6, $6148(a5)
pub fn i_05aa26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA2A;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA2A: bra.w $5aa3c
pub fn i_05aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA3C;
    10
}
/// $05AA2E: subq.w #$4, d7
pub fn i_05aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA30;
    cpu.ir = 0x5947;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AA30: lea.l $5428(a5), a4
pub fn i_05aa30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA34;
    cpu.ir = 0x49ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[4] = ad1;
    16
}
/// $05AA34: move.l d1, (a4, d7.w)
pub fn i_05aa34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA38;
    cpu.ir = 0x2981;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05AA38: move.w d7, $614a(a5)
pub fn i_05aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA3C;
    cpu.ir = 0x3B47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA3C: move.w d5, -(a7)
pub fn i_05aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA3E;
    cpu.ir = 0x3F05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05AA3E: lsl.w #$8, d2
pub fn i_05aa3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA40;
    cpu.ir = 0xE14A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AA40: lea.l $4258(a5), a3
pub fn i_05aa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA44;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05AA44: lea.l $11d0(a3), a2
pub fn i_05aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA48;
    cpu.ir = 0x45EB;
    let ad1 = cpu.a[3].wrapping_add(0x11D0);
    cpu.a[2] = ad1;
    16
}
/// $05AA48: adda.w d1, a3
pub fn i_05aa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4A;
    cpu.ir = 0xD6C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AA4A: move.b d3, d7
pub fn i_05aa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4C;
    cpu.ir = 0x1E03;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AA4C: and.b d4, d7
pub fn i_05aa4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA4E;
    cpu.ir = 0xCE04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 & cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AA4E: addq.b #$1, d7
pub fn i_05aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA50;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AA50: sne.b d7
pub fn i_05aa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA52;
    cpu.ir = 0x56C7;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[7] & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AA52: andi.w #$4, d7
pub fn i_05aa52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA56;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x4) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AA56: andi.b #$3, d5
pub fn i_05aa56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5A;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AA5A: or.b d5, d7
pub fn i_05aa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5C;
    cpu.ir = 0x8E05;
    let v1 = cpu.d[5] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AA5C: add.w d7, d7
pub fn i_05aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA5E;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AA5E: add.w d7, d7
pub fn i_05aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA60;
    cpu.ir = 0xDE47;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AA60: jsr $5aa96(pc, d7.w)
pub fn i_05aa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA64;
    cpu.ir = 0x4EBB;
    let ad1 = 0x5AA96_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.push32(bus, 0x5AA64);
    cpu.jump_to(ad1);
    16
}
/// $05AA64: move.w (a7)+, d5
pub fn i_05aa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA66;
    cpu.ir = 0x3A1F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05AA66: lea.l $4258(a5), a2
pub fn i_05aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA6A;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AA6A: suba.l a2, a3
pub fn i_05aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA6C;
    cpu.ir = 0x97CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_sub(v1);
    8
}
/// $05AA6C: move.w $4254(a5), d0
pub fn i_05aa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA70;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05AA70: cmp.w a3, d0
pub fn i_05aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA72;
    cpu.ir = 0xB04B;
    let v1 = cpu.a[3] as i16 as i32 as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05AA72: beq.w $5aa7e
pub fn i_05aa72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AA7E; return 10; }
    8
}
/// $05AA76: move.w a3, $4254(a5)
pub fn i_05aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA7A;
    cpu.ir = 0x3B4B;
    let v1 = cpu.a[3] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x4254);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05AA7A: bra.w $5aa92
pub fn i_05aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA7E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA92;
    10
}
/// $05AA7E: btst.b #$2, d5
pub fn i_05aa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA82;
    cpu.ir = 0x805;
    let mk: u32 = 1u32 << ((2) % 32);
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05AA82: bne.w $5aa8e
pub fn i_05aa82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AA8E; return 10; }
    8
}
/// $05AA86: subq.w #$4, $6148(a5)
pub fn i_05aa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA8A;
    cpu.ir = 0x596D;
    let ad1 = cpu.a[5].wrapping_add(0x6148);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05AA8A: bra.w $5aa92
pub fn i_05aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AA92;
    10
}
/// $05AA8E: addq.w #$4, $614a(a5)
pub fn i_05aa8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA92;
    cpu.ir = 0x586D;
    let ad1 = cpu.a[5].wrapping_add(0x614A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05AA92: move usp, a1
pub fn i_05aa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA94;
    cpu.ir = 0x4E69;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[1] = cpu.usp;
    4
}
/// $05AA94: rts
pub fn i_05aa94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteDispatchJT_05AA96  ($05AA96, 1820 B, video/sprite_queue_render_05aa96.s)
/// $05AA96: bra.w $5aab6
pub fn i_05aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AAB6;
    10
}
/// $05AA9A: bra.w $5ab34
pub fn i_05aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AA9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AB34;
    10
}
/// $05AA9E: bra.w $5abb8
pub fn i_05aa9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5ABB8;
    10
}
/// $05AAA2: bra.w $5ac44
pub fn i_05aaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AC44;
    10
}
/// $05AAA6: bra.w $5acd6
pub fn i_05aaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5ACD6;
    10
}
/// $05AAAA: bra.w $5ae00
pub fn i_05aaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AE00;
    10
}
/// $05AAAE: bra.w $5af30
pub fn i_05aaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AF30;
    10
}
/// $05AAB2: bra.w $5b06e
pub fn i_05aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B06E;
    10
}
/// $05AAB6: cmpa.l a2, a3
pub fn i_05aab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAB8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AAB8: bcc.w $5ab32
pub fn i_05aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AABC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB32; return 10; }
    8
}
/// $05AABC: move.b (a0)+, -(a7)
pub fn i_05aabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AABE;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AABE: move.b (a0)+, d2
pub fn i_05aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC0;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AAC0: eor.b d5, d2
pub fn i_05aac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC2;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AAC2: move.w #$fff, d4
pub fn i_05aac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC6;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAC6: moveq #$0, d5
pub fn i_05aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAC8;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAC8: move.l (a0)+, d1
pub fn i_05aac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACA;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AACA: moveq #$0, d5
pub fn i_05aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACC;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AACC: move.b (a0)+, d5
pub fn i_05aacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AACE;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AACE: swap d5
pub fn i_05aace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD0;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAD0: move.b (a0)+, d5
pub fn i_05aad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD2;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AAD2: move.w d5, d6
pub fn i_05aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD4;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAD4: swap d6
pub fn i_05aad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD6;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAD6: move.w d5, d6
pub fn i_05aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAD8;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAD8: add.w d0, d1
pub fn i_05aad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AADA: swap d1
pub fn i_05aada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADC;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AADC: swap d0
pub fn i_05aadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AADE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AADE: add.w d0, d1
pub fn i_05aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE0;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AAE0: swap d0
pub fn i_05aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE2;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAE2: andi.w #$1ff, d1
pub fn i_05aae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE6;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AAE6: lsl.l #$7, d1
pub fn i_05aae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAE8;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AAE8: or.l d6, d1
pub fn i_05aae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEA;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAEA: clr.w d5
pub fn i_05aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEC;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AAEC: swap d5
pub fn i_05aaec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAEE;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AAEE: subq.b #$1, d5
pub fn i_05aaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF0;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AAF0: add.w d6, d6
pub fn i_05aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF2;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AAF2: move.w #$800, d3
pub fn i_05aaf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAF6;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AAF6: cmpi.w #$f800, d1
pub fn i_05aaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAFA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AAFA: bcc.w $5ab06
pub fn i_05aafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AAFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB06; return 10; }
    8
}
/// $05AAFE: cmpi.w #$a000, d1
pub fn i_05aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB02;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AB02: bcc.w $5ab14
pub fn i_05ab02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB14; return 10; }
    8
}
/// $05AB06: move.l a0, (a3)+
pub fn i_05ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB08;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB08: move.w d2, (a3)+
pub fn i_05ab08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0A;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB0A: move.w d4, (a3)+
pub fn i_05ab0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0C;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB0C: move.l d1, (a3)+
pub fn i_05ab0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB0E;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB0E: cmpa.l a2, a3
pub fn i_05ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB10;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB10: bcc.w $5ab20
pub fn i_05ab10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB20; return 10; }
    8
}
/// $05AB14: add.w d3, d1
pub fn i_05ab14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB16;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB16: adda.w d6, a0
pub fn i_05ab16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB18;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AB18: dbra d5, $5aaf6
pub fn i_05ab18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB1C;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5AB1C; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5AB1C; return 14; }
    cpu.pc = 0x5AAF6;
    10
}
/// $05AB1C: subq.b #$1, (a7)
pub fn i_05ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB1E;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05AB1E: bne.b $5aac6
pub fn i_05ab1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB20;
    cpu.ir = 0x66A6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AAC6; return 10; }
    8
}
/// $05AB20: addq.w #$2, a7
pub fn i_05ab20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB22;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05AB22: lea.l $4258(a5), a2
pub fn i_05ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB26;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AB26: cmpa.l a2, a3
pub fn i_05ab26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB28;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB28: beq.w $5ab32
pub fn i_05ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB2C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AB32; return 10; }
    8
}
/// $05AB2C: ori.b #$80, -$6(a3)
pub fn i_05ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB32;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05AB32: rts
pub fn i_05ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AB34: cmpa.l a2, a3
pub fn i_05ab34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB36;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB36: bcc.w $5abb6
pub fn i_05ab36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ABB6; return 10; }
    8
}
/// $05AB3A: move.b (a0)+, -(a7)
pub fn i_05ab3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3C;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AB3C: move.b (a0)+, d2
pub fn i_05ab3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB3E;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB3E: eor.b d5, d2
pub fn i_05ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB40;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AB40: move.w #$fff, d4
pub fn i_05ab40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB44;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB44: moveq #$0, d5
pub fn i_05ab44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB46;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB46: move.l (a0)+, d1
pub fn i_05ab46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB48;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB48: moveq #$0, d5
pub fn i_05ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB4A: move.b (a0)+, d5
pub fn i_05ab4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4C;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB4C: swap d5
pub fn i_05ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB4E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB4E: move.b (a0)+, d5
pub fn i_05ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB50;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AB50: swap d1
pub fn i_05ab50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB52;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB52: not.w d1
pub fn i_05ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB54;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AB54: swap d1
pub fn i_05ab54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB56;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB56: move.w d5, d6
pub fn i_05ab56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB58;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB58: swap d6
pub fn i_05ab58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5A;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB5A: move.w d5, d6
pub fn i_05ab5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5C;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB5C: add.w d0, d1
pub fn i_05ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB5E;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB5E: swap d1
pub fn i_05ab5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB60;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB60: swap d0
pub fn i_05ab60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB62;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB62: add.w d0, d1
pub fn i_05ab62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB64;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB64: swap d0
pub fn i_05ab64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB66;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB66: andi.w #$1ff, d1
pub fn i_05ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AB6A: lsl.l #$7, d1
pub fn i_05ab6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6C;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AB6C: or.l d6, d1
pub fn i_05ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB6E;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB6E: clr.w d5
pub fn i_05ab6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB70;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AB70: swap d5
pub fn i_05ab70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB72;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AB72: subq.b #$1, d5
pub fn i_05ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB74;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AB74: add.w d6, d6
pub fn i_05ab74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB76;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB76: move.w #$800, d3
pub fn i_05ab76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB7A;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AB7A: sub.w d3, d1
pub fn i_05ab7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB7C;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AB7C: cmpi.w #$f800, d1
pub fn i_05ab7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB80;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AB80: bcc.w $5ab8c
pub fn i_05ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB8C; return 10; }
    8
}
/// $05AB84: cmpi.w #$a000, d1
pub fn i_05ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB88;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AB88: bcc.w $5ab9a
pub fn i_05ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AB9A; return 10; }
    8
}
/// $05AB8C: move.l a0, (a3)+
pub fn i_05ab8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB8E;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB8E: move.w d2, (a3)+
pub fn i_05ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB90;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB90: move.w d4, (a3)+
pub fn i_05ab90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB92;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AB92: move.l d1, (a3)+
pub fn i_05ab92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB94;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AB94: cmpa.l a2, a3
pub fn i_05ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB96;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AB96: bcc.w $5aba4
pub fn i_05ab96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ABA4; return 10; }
    8
}
/// $05AB9A: adda.w d6, a0
pub fn i_05ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AB9C;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AB9C: dbra d5, $5ab7a
pub fn i_05ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA0;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ABA0; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ABA0; return 14; }
    cpu.pc = 0x5AB7A;
    10
}
/// $05ABA0: subq.b #$1, (a7)
pub fn i_05aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA2;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ABA2: bne.b $5ab44
pub fn i_05aba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA4;
    cpu.ir = 0x66A0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AB44; return 10; }
    8
}
/// $05ABA4: addq.w #$2, a7
pub fn i_05aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABA6;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ABA6: lea.l $4258(a5), a2
pub fn i_05aba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABAA;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05ABAA: cmpa.l a2, a3
pub fn i_05abaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABAC;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ABAC: beq.w $5abb6
pub fn i_05abac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ABB6; return 10; }
    8
}
/// $05ABB0: ori.b #$80, -$6(a3)
pub fn i_05abb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB6;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ABB6: rts
pub fn i_05abb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05ABB8: cmpa.l a2, a3
pub fn i_05abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABBA;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ABBA: bcc.w $5ac42
pub fn i_05abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC42; return 10; }
    8
}
/// $05ABBE: move.b (a0)+, -(a7)
pub fn i_05abbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC0;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05ABC0: move.b (a0)+, d2
pub fn i_05abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC2;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABC2: eor.b d5, d2
pub fn i_05abc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC4;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ABC4: move.w #$1fff, d4
pub fn i_05abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABC8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABC8: moveq #$0, d5
pub fn i_05abc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCA;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABCA: move.l (a0)+, d1
pub fn i_05abca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCC;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ABCC: moveq #$0, d5
pub fn i_05abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABCE;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABCE: move.b (a0)+, d5
pub fn i_05abce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD0;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABD0: swap d5
pub fn i_05abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD2;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABD2: move.b (a0)+, d5
pub fn i_05abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD4;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ABD4: neg.w d1
pub fn i_05abd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD6;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABD6: move.w d5, d6
pub fn i_05abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABD8;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABD8: lsl.w #$4, d6
pub fn i_05abd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDA;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05ABDA: subq.w #$1, d6
pub fn i_05abda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDC;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABDC: add.w d6, d1
pub fn i_05abdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABDE;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABDE: move.w d5, d6
pub fn i_05abde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE0;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABE0: swap d6
pub fn i_05abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE2;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABE2: move.w d5, d6
pub fn i_05abe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE4;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ABE4: add.w d0, d1
pub fn i_05abe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE6;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABE6: swap d1
pub fn i_05abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABE8;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABE8: swap d0
pub fn i_05abe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABEA: add.w d0, d1
pub fn i_05abea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEC;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABEC: swap d0
pub fn i_05abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABEE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABEE: andi.w #$1ff, d1
pub fn i_05abee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF2;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ABF2: lsl.l #$7, d1
pub fn i_05abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF4;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05ABF4: or.l d6, d1
pub fn i_05abf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF6;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABF6: clr.w d5
pub fn i_05abf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABF8;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ABF8: swap d5
pub fn i_05abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFA;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ABFA: subq.b #$1, d5
pub fn i_05abfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFC;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ABFC: add.w d6, d6
pub fn i_05abfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ABFE;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ABFE: adda.w d6, a0
pub fn i_05abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC00;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC00: move.w #$800, d3
pub fn i_05ac00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC04;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC04: cmpi.w #$f800, d1
pub fn i_05ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC08;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AC08: bcc.w $5ac14
pub fn i_05ac08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC14; return 10; }
    8
}
/// $05AC0C: cmpi.w #$a000, d1
pub fn i_05ac0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC10;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AC10: bcc.w $5ac22
pub fn i_05ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC22; return 10; }
    8
}
/// $05AC14: move.l a0, (a3)+
pub fn i_05ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC16;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC16: move.w d2, (a3)+
pub fn i_05ac16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC18;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AC18: move.w d4, (a3)+
pub fn i_05ac18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1A;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AC1A: move.l d1, (a3)+
pub fn i_05ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1C;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC1C: cmpa.l a2, a3
pub fn i_05ac1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC1E;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC1E: bcc.w $5ac30
pub fn i_05ac1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AC30; return 10; }
    8
}
/// $05AC22: add.w d3, d1
pub fn i_05ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC24;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC24: adda.w d6, a0
pub fn i_05ac24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC26;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC26: dbra d5, $5ac04
pub fn i_05ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2A;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5AC2A; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5AC2A; return 14; }
    cpu.pc = 0x5AC04;
    10
}
/// $05AC2A: suba.w d6, a0
pub fn i_05ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2C;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC2C: subq.b #$1, (a7)
pub fn i_05ac2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC2E;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05AC2E: bne.b $5abc8
pub fn i_05ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC30;
    cpu.ir = 0x6698;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5ABC8; return 10; }
    8
}
/// $05AC30: addq.w #$2, a7
pub fn i_05ac30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC32;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05AC32: lea.l $4258(a5), a2
pub fn i_05ac32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC36;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AC36: cmpa.l a2, a3
pub fn i_05ac36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC38;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC38: beq.w $5ac42
pub fn i_05ac38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AC42; return 10; }
    8
}
/// $05AC3C: ori.b #$80, -$6(a3)
pub fn i_05ac3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC42;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05AC42: rts
pub fn i_05ac42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AC44: cmpa.l a2, a3
pub fn i_05ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC46;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AC46: bcc.w $5acd4
pub fn i_05ac46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACD4; return 10; }
    8
}
/// $05AC4A: move.b (a0)+, -(a7)
pub fn i_05ac4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4C;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AC4C: move.b (a0)+, d2
pub fn i_05ac4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC4E;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC4E: eor.b d5, d2
pub fn i_05ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC50;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AC50: move.w #$1fff, d4
pub fn i_05ac50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC54;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC54: moveq #$0, d5
pub fn i_05ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC56;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC56: move.l (a0)+, d1
pub fn i_05ac56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC58;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AC58: moveq #$0, d5
pub fn i_05ac58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC5A: move.b (a0)+, d5
pub fn i_05ac5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5C;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC5C: swap d5
pub fn i_05ac5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC5E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC5E: move.b (a0)+, d5
pub fn i_05ac5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC60;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AC60: swap d1
pub fn i_05ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC62;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC62: not.w d1
pub fn i_05ac62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC64;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC64: swap d1
pub fn i_05ac64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC66;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC66: neg.w d1
pub fn i_05ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC68;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC68: move.w d5, d6
pub fn i_05ac68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6A;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC6A: lsl.w #$4, d6
pub fn i_05ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6C;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AC6C: subq.w #$1, d6
pub fn i_05ac6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC6E;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC6E: add.w d6, d1
pub fn i_05ac6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC70;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC70: move.w d5, d6
pub fn i_05ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC72;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC72: swap d6
pub fn i_05ac72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC74;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC74: move.w d5, d6
pub fn i_05ac74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC76;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC76: add.w d0, d1
pub fn i_05ac76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC78;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC78: swap d1
pub fn i_05ac78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7A;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC7A: swap d0
pub fn i_05ac7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC7C: add.w d0, d1
pub fn i_05ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC7E;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC7E: swap d0
pub fn i_05ac7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC80;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC80: andi.w #$1ff, d1
pub fn i_05ac80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC84;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AC84: lsl.l #$7, d1
pub fn i_05ac84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC86;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AC86: or.l d6, d1
pub fn i_05ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC88;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC88: clr.w d5
pub fn i_05ac88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8A;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AC8A: swap d5
pub fn i_05ac8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8C;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AC8C: subq.b #$1, d5
pub fn i_05ac8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC8E;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AC8E: add.w d6, d6
pub fn i_05ac8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC90;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC90: adda.w d6, a0
pub fn i_05ac90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC92;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AC92: move.w #$800, d3
pub fn i_05ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC96;
    cpu.ir = 0x363C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AC96: sub.w d3, d1
pub fn i_05ac96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC98;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AC98: cmpi.w #$f800, d1
pub fn i_05ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AC9C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AC9C: bcc.w $5aca8
pub fn i_05ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACA8; return 10; }
    8
}
/// $05ACA0: cmpi.w #$a000, d1
pub fn i_05aca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05ACA4: bcc.w $5acb6
pub fn i_05aca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACB6; return 10; }
    8
}
/// $05ACA8: move.l a0, (a3)+
pub fn i_05aca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAA;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ACAA: move.w d2, (a3)+
pub fn i_05acaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAC;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ACAC: move.w d4, (a3)+
pub fn i_05acac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACAE;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ACAE: move.l d1, (a3)+
pub fn i_05acae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB0;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ACB0: cmpa.l a2, a3
pub fn i_05acb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB2;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACB2: bcc.w $5acc2
pub fn i_05acb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ACC2; return 10; }
    8
}
/// $05ACB6: adda.w d6, a0
pub fn i_05acb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACB8;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ACB8: dbra d5, $5ac96
pub fn i_05acb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACBC;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ACBC; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ACBC; return 14; }
    cpu.pc = 0x5AC96;
    10
}
/// $05ACBC: suba.w d6, a0
pub fn i_05acbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACBE;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ACBE: subq.b #$1, (a7)
pub fn i_05acbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC0;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ACC0: bne.b $5ac54
pub fn i_05acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC2;
    cpu.ir = 0x6692;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AC54; return 10; }
    8
}
/// $05ACC2: addq.w #$2, a7
pub fn i_05acc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC4;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ACC4: lea.l $4258(a5), a2
pub fn i_05acc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACC8;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05ACC8: cmpa.l a2, a3
pub fn i_05acc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACCA;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACCA: beq.w $5acd4
pub fn i_05acca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACCE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ACD4; return 10; }
    8
}
/// $05ACCE: ori.b #$80, -$6(a3)
pub fn i_05acce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD4;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ACD4: rts
pub fn i_05acd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05ACD6: cmpa.l a2, a3
pub fn i_05acd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACD8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ACD8: bcc.w $5adfe
pub fn i_05acd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADFE; return 10; }
    8
}
/// $05ACDC: move.b (a0)+, -(a7)
pub fn i_05acdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACDE;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05ACDE: move.b (a0)+, d2
pub fn i_05acde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE0;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05ACE0: eor.b d5, d2
pub fn i_05ace0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE2;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05ACE2: andi.w #$ff, d4
pub fn i_05ace2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE6;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ACE6: move.w d4, d7
pub fn i_05ace6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACE8;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ACE8: addq.w #$1, d7
pub fn i_05ace8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEA;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ACEA: swap d7
pub fn i_05acea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEC;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ACEC: move.w d3, d7
pub fn i_05acec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACEE;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ACEE: andi.w #$ff, d7
pub fn i_05acee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF2;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ACF2: addq.w #$1, d7
pub fn i_05acf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF4;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05ACF4: lsl.w #$4, d4
pub fn i_05acf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACF6;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05ACF6: eori.b #$f0, d4
pub fn i_05acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACFA;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05ACFA: move.b d4, $616e(a5)
pub fn i_05acfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ACFE;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05ACFE: move.b d3, d4
pub fn i_05acfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD00;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD00: andi.b #$f0, d4
pub fn i_05ad00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD04;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AD04: move.w d4, d6
pub fn i_05ad04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD06;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD06: subi.w #$100, d4
pub fn i_05ad06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0A;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD0A: swap d4
pub fn i_05ad0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD0C: move.w d6, d4
pub fn i_05ad0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD0E;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD0E: andi.w #$f, d3
pub fn i_05ad0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD12;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD12: swap d2
pub fn i_05ad12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD14;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD14: move.w d3, d2
pub fn i_05ad14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD16;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD16: swap d2
pub fn i_05ad16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD18;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD18: moveq #$0, d5
pub fn i_05ad18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD1A: move.l (a0)+, d1
pub fn i_05ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1C;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AD1C: moveq #$0, d5
pub fn i_05ad1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD1E;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD1E: move.b (a0)+, d5
pub fn i_05ad1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD20;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AD20: swap d5
pub fn i_05ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD22;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD22: move.b (a0)+, d5
pub fn i_05ad22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD24;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AD24: moveq #$0, d3
pub fn i_05ad24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD26;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD26: move.l d1, d6
pub fn i_05ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD28;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD28: muls.w d7, d6
pub fn i_05ad28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2A;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD2A: spl.b d3
pub fn i_05ad2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2C;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AD2C: add.l d3, d6
pub fn i_05ad2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD2E;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05AD2E: asr.l #$8, d6
pub fn i_05ad2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD30;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05AD30: swap d7
pub fn i_05ad30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD32;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD32: swap d1
pub fn i_05ad32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD34;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD34: muls.w d7, d1
pub fn i_05ad34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD36;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD36: move.b d1, $616d(a5)
pub fn i_05ad36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3A;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AD3A: lsl.l #$8, d1
pub fn i_05ad3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3C;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05AD3C: move.w d6, d1
pub fn i_05ad3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD3E;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD3E: swap d7
pub fn i_05ad3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD40;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD40: move.w d5, d6
pub fn i_05ad40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD42;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD42: cmpi.b #$11, d6
pub fn i_05ad42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD46;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05AD46: bcc.w $5ad60
pub fn i_05ad46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AD60; return 10; }
    8
}
/// $05AD4A: subq.b #$1, d6
pub fn i_05ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4C;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AD4C: lsl.b #$4, d6
pub fn i_05ad4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD4E;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05AD4E: swap d2
pub fn i_05ad4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD50;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD50: or.b d2, d6
pub fn i_05ad50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD52;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AD52: lea.l $5a8ba(pc), a4
pub fn i_05ad52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD56;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05AD56: move.b (a4, d6.w), d6
pub fn i_05ad56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD5A;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05AD5A: swap d2
pub fn i_05ad5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD5C;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD5C: bra.w $5ad66
pub fn i_05ad5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD60;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AD66;
    10
}
/// $05AD60: swap d2
pub fn i_05ad60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD62;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD62: move.w d2, d6
pub fn i_05ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD64;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD64: swap d2
pub fn i_05ad64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD66;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD66: andi.b #$f0, d4
pub fn i_05ad66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6A;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AD6A: or.b d4, d6
pub fn i_05ad6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6C;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AD6C: move.b d6, d4
pub fn i_05ad6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD6E;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD6E: swap d4
pub fn i_05ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD70;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD70: move.b d6, d4
pub fn i_05ad70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD72;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AD72: swap d4
pub fn i_05ad72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD74;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD74: move.w d7, d6
pub fn i_05ad74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD76;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD76: mulu.w d5, d6
pub fn i_05ad76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD78;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AD78: addi.w #$ff, d6
pub fn i_05ad78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD7C;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD7C: lsr.w #$8, d6
pub fn i_05ad7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD7E;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AD7E: swap d6
pub fn i_05ad7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD80;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD80: move.w d5, d6
pub fn i_05ad80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD82;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AD82: add.w d0, d1
pub fn i_05ad82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD84;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD84: swap d1
pub fn i_05ad84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD86;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD86: swap d0
pub fn i_05ad86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD88;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD88: add.w d0, d1
pub fn i_05ad88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD8A;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD8A: swap d0
pub fn i_05ad8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD8C;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD8C: andi.w #$1ff, d1
pub fn i_05ad8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD90;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AD90: lsl.l #$7, d1
pub fn i_05ad90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD92;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AD92: or.l d6, d1
pub fn i_05ad92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD94;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD94: clr.w d5
pub fn i_05ad94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD96;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AD96: swap d5
pub fn i_05ad96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD98;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AD98: subq.b #$1, d5
pub fn i_05ad98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9A;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AD9A: add.w d6, d6
pub fn i_05ad9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9C;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AD9C: movea.l d4, a4
pub fn i_05ad9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AD9E;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05AD9E: move.b $616e(a5), d3
pub fn i_05ad9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADA2;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05ADA2: sub.b d3, $616d(a5)
pub fn i_05ada2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADA6;
    cpu.ir = 0x972D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05ADA6: bcc.w $5adb2
pub fn i_05ada6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADB2; return 10; }
    8
}
/// $05ADAA: swap d4
pub fn i_05adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAC;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ADAC: tst.w d4
pub fn i_05adac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADAE;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05ADAE: bmi.w $5adde
pub fn i_05adae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5ADDE; return 10; }
    8
}
/// $05ADB2: move.w d4, d3
pub fn i_05adb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB4;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05ADB4: andi.w #$f00, d3
pub fn i_05adb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADB8;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ADB8: addi.w #$100, d3
pub fn i_05adb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADBC;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05ADBC: lsr.w #$1, d3
pub fn i_05adbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADBE;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05ADBE: cmpi.w #$f800, d1
pub fn i_05adbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADC2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05ADC2: bcc.w $5adce
pub fn i_05adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADCE; return 10; }
    8
}
/// $05ADC6: cmpi.w #$a000, d1
pub fn i_05adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADCA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05ADCA: bcc.w $5addc
pub fn i_05adca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADDC; return 10; }
    8
}
/// $05ADCE: move.l a0, (a3)+
pub fn i_05adce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD0;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ADD0: move.w d2, (a3)+
pub fn i_05add0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD2;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ADD2: move.w d4, (a3)+
pub fn i_05add2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD4;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05ADD4: move.l d1, (a3)+
pub fn i_05add4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD6;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05ADD6: cmpa.l a2, a3
pub fn i_05add6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADD8;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ADD8: bcc.w $5adec
pub fn i_05add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5ADEC; return 10; }
    8
}
/// $05ADDC: add.w d3, d1
pub fn i_05addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADDE;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05ADDE: move.l a4, d4
pub fn i_05adde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE0;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05ADE0: adda.w d6, a0
pub fn i_05ade0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE2;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05ADE2: dbra d5, $5ad9e
pub fn i_05ade2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE6;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5ADE6; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5ADE6; return 14; }
    cpu.pc = 0x5AD9E;
    10
}
/// $05ADE6: subq.b #$1, (a7)
pub fn i_05ade6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADE8;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05ADE8: bne.w $5ad18
pub fn i_05ade8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AD18; return 10; }
    8
}
/// $05ADEC: addq.w #$2, a7
pub fn i_05adec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADEE;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05ADEE: lea.l $4258(a5), a2
pub fn i_05adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF2;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05ADF2: cmpa.l a2, a3
pub fn i_05adf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF4;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05ADF4: beq.w $5adfe
pub fn i_05adf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADF8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5ADFE; return 10; }
    8
}
/// $05ADF8: ori.b #$80, -$6(a3)
pub fn i_05adf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5ADFE;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05ADFE: rts
pub fn i_05adfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AE00: cmpa.l a2, a3
pub fn i_05ae00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE02;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AE02: bcc.w $5af2e
pub fn i_05ae02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF2E; return 10; }
    8
}
/// $05AE06: move.b (a0)+, -(a7)
pub fn i_05ae06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE08;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AE08: move.b (a0)+, d2
pub fn i_05ae08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE0A;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AE0A: eor.b d5, d2
pub fn i_05ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE0C;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AE0C: andi.w #$ff, d4
pub fn i_05ae0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE10;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AE10: move.w d4, d7
pub fn i_05ae10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE12;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE12: addq.w #$1, d7
pub fn i_05ae12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE14;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AE14: swap d7
pub fn i_05ae14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE16;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE16: move.w d3, d7
pub fn i_05ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE18;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE18: andi.w #$ff, d7
pub fn i_05ae18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE1C;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AE1C: addq.w #$1, d7
pub fn i_05ae1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE1E;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AE1E: lsl.w #$4, d4
pub fn i_05ae1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE20;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AE20: eori.b #$f0, d4
pub fn i_05ae20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE24;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AE24: move.b d4, $616e(a5)
pub fn i_05ae24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE28;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AE28: move.b d3, d4
pub fn i_05ae28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE2A;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AE2A: andi.b #$f0, d4
pub fn i_05ae2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE2E;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AE2E: move.w d4, d6
pub fn i_05ae2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE30;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE30: subi.w #$100, d4
pub fn i_05ae30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE34;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AE34: swap d4
pub fn i_05ae34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE36;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE36: move.w d6, d4
pub fn i_05ae36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE38;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE38: andi.w #$f, d3
pub fn i_05ae38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE3C;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AE3C: swap d2
pub fn i_05ae3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE3E;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE3E: move.w d3, d2
pub fn i_05ae3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE40;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE40: swap d2
pub fn i_05ae40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE42;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE42: moveq #$0, d5
pub fn i_05ae42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE44;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE44: move.l (a0)+, d1
pub fn i_05ae44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE46;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AE46: moveq #$0, d5
pub fn i_05ae46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE48;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE48: move.b (a0)+, d5
pub fn i_05ae48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE4A;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AE4A: swap d5
pub fn i_05ae4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE4C;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE4C: move.b (a0)+, d5
pub fn i_05ae4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE4E;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AE4E: swap d1
pub fn i_05ae4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE50;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE50: not.w d1
pub fn i_05ae50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE52;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AE52: swap d1
pub fn i_05ae52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE54;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE54: moveq #$0, d3
pub fn i_05ae54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE56;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE56: move.l d1, d6
pub fn i_05ae56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE58;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE58: muls.w d7, d6
pub fn i_05ae58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE5A;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AE5A: spl.b d3
pub fn i_05ae5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE5C;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AE5C: add.l d3, d6
pub fn i_05ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE5E;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05AE5E: asr.l #$8, d6
pub fn i_05ae5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE60;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05AE60: swap d7
pub fn i_05ae60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE62;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE62: swap d1
pub fn i_05ae62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE64;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE64: muls.w d7, d1
pub fn i_05ae64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE66;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AE66: move.b d1, $616d(a5)
pub fn i_05ae66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE6A;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AE6A: lsl.l #$8, d1
pub fn i_05ae6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE6C;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05AE6C: move.w d6, d1
pub fn i_05ae6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE6E;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE6E: swap d7
pub fn i_05ae6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE70;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE70: move.w d5, d6
pub fn i_05ae70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE72;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE72: cmpi.b #$11, d6
pub fn i_05ae72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE76;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05AE76: bcc.w $5ae90
pub fn i_05ae76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AE90; return 10; }
    8
}
/// $05AE7A: subq.b #$1, d6
pub fn i_05ae7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE7C;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AE7C: lsl.b #$4, d6
pub fn i_05ae7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE7E;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05AE7E: swap d2
pub fn i_05ae7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE80;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE80: or.b d2, d6
pub fn i_05ae80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE82;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AE82: lea.l $5a8ba(pc), a4
pub fn i_05ae82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE86;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05AE86: move.b (a4, d6.w), d6
pub fn i_05ae86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE8A;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05AE8A: swap d2
pub fn i_05ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE8C;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE8C: bra.w $5ae96
pub fn i_05ae8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE90;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AE96;
    10
}
/// $05AE90: swap d2
pub fn i_05ae90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE92;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE92: move.w d2, d6
pub fn i_05ae92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE94;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AE94: swap d2
pub fn i_05ae94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE96;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AE96: andi.b #$f0, d4
pub fn i_05ae96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE9A;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AE9A: or.b d4, d6
pub fn i_05ae9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE9C;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AE9C: move.b d6, d4
pub fn i_05ae9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AE9E;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AE9E: swap d4
pub fn i_05ae9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA0;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEA0: move.b d6, d4
pub fn i_05aea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA2;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AEA2: swap d4
pub fn i_05aea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA4;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEA4: move.w d7, d6
pub fn i_05aea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA6;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AEA6: mulu.w d5, d6
pub fn i_05aea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEA8;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AEA8: addi.w #$ff, d6
pub fn i_05aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEAC;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AEAC: lsr.w #$8, d6
pub fn i_05aeac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEAE;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AEAE: swap d6
pub fn i_05aeae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB0;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEB0: move.w d5, d6
pub fn i_05aeb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB2;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AEB2: add.w d0, d1
pub fn i_05aeb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB4;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AEB4: swap d1
pub fn i_05aeb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB6;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEB6: swap d0
pub fn i_05aeb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEB8;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEB8: add.w d0, d1
pub fn i_05aeb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEBA;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AEBA: swap d0
pub fn i_05aeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEBC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEBC: andi.w #$1ff, d1
pub fn i_05aebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC0;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AEC0: lsl.l #$7, d1
pub fn i_05aec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC2;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AEC2: or.l d6, d1
pub fn i_05aec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC4;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEC4: clr.w d5
pub fn i_05aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC6;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AEC6: swap d5
pub fn i_05aec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEC8;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEC8: subq.b #$1, d5
pub fn i_05aec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AECA;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AECA: add.w d6, d6
pub fn i_05aeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AECC;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AECC: movea.l d4, a4
pub fn i_05aecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AECE;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05AECE: move.b $616e(a5), d3
pub fn i_05aece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AED2;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05AED2: add.b d3, $616d(a5)
pub fn i_05aed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AED6;
    cpu.ir = 0xD72D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05AED6: bcc.w $5aee2
pub fn i_05aed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEDA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AEE2; return 10; }
    8
}
/// $05AEDA: swap d4
pub fn i_05aeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEDC;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AEDC: tst.w d4
pub fn i_05aedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEDE;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05AEDE: bmi.w $5af0e
pub fn i_05aede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEE2;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5AF0E; return 10; }
    8
}
/// $05AEE2: move.w d4, d3
pub fn i_05aee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEE4;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AEE4: andi.w #$f00, d3
pub fn i_05aee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEE8;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AEE8: addi.w #$100, d3
pub fn i_05aee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEEC;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AEEC: lsr.w #$1, d3
pub fn i_05aeec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEEE;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05AEEE: sub.w d3, d1
pub fn i_05aeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEF0;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AEF0: cmpi.w #$f800, d1
pub fn i_05aef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEF4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05AEF4: bcc.w $5af00
pub fn i_05aef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF00; return 10; }
    8
}
/// $05AEF8: cmpi.w #$a000, d1
pub fn i_05aef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AEFC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05AEFC: bcc.w $5af0e
pub fn i_05aefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF0E; return 10; }
    8
}
/// $05AF00: move.l a0, (a3)+
pub fn i_05af00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF02;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AF02: move.w d2, (a3)+
pub fn i_05af02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF04;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AF04: move.w d4, (a3)+
pub fn i_05af04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF06;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05AF06: move.l d1, (a3)+
pub fn i_05af06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF08;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AF08: cmpa.l a2, a3
pub fn i_05af08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF0A;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AF0A: bcc.w $5af1c
pub fn i_05af0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AF1C; return 10; }
    8
}
/// $05AF0E: move.l a4, d4
pub fn i_05af0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF10;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF10: adda.w d6, a0
pub fn i_05af10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF12;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05AF12: dbra d5, $5aece
pub fn i_05af12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF16;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5AF16; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5AF16; return 14; }
    cpu.pc = 0x5AECE;
    10
}
/// $05AF16: subq.b #$1, (a7)
pub fn i_05af16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF18;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05AF18: bne.w $5ae42
pub fn i_05af18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF1C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AE42; return 10; }
    8
}
/// $05AF1C: addq.w #$2, a7
pub fn i_05af1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF1E;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05AF1E: lea.l $4258(a5), a2
pub fn i_05af1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF22;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05AF22: cmpa.l a2, a3
pub fn i_05af22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF24;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AF24: beq.w $5af2e
pub fn i_05af24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5AF2E; return 10; }
    8
}
/// $05AF28: ori.b #$80, -$6(a3)
pub fn i_05af28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF2E;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05AF2E: rts
pub fn i_05af2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF30;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05AF30: cmpa.l a2, a3
pub fn i_05af30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF32;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05AF32: bcc.w $5b06c
pub fn i_05af32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B06C; return 10; }
    8
}
/// $05AF36: move.b (a0)+, -(a7)
pub fn i_05af36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF38;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05AF38: move.b (a0)+, d2
pub fn i_05af38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF3A;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AF3A: eor.b d5, d2
pub fn i_05af3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF3C;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AF3C: andi.w #$ff, d4
pub fn i_05af3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF40;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AF40: move.w d4, d7
pub fn i_05af40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF42;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF42: addq.w #$1, d7
pub fn i_05af42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF44;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AF44: swap d7
pub fn i_05af44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF46;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF46: move.w d3, d7
pub fn i_05af46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF48;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF48: andi.w #$ff, d7
pub fn i_05af48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF4C;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AF4C: addq.w #$1, d7
pub fn i_05af4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF4E;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AF4E: lsl.w #$4, d4
pub fn i_05af4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF50;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AF50: eori.b #$f0, d4
pub fn i_05af50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF54;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AF54: move.b d4, $616e(a5)
pub fn i_05af54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF58;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AF58: move.b d3, d4
pub fn i_05af58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF5A;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AF5A: andi.b #$f0, d4
pub fn i_05af5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF5E;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AF5E: move.w d4, d6
pub fn i_05af5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF60;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF60: subi.w #$100, d4
pub fn i_05af60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF64;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AF64: swap d4
pub fn i_05af64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF66;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF66: move.w d6, d4
pub fn i_05af66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF68;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF68: ori.l #$10001000, d4
pub fn i_05af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF6E;
    cpu.ir = 0x84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 | 0x10001000) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $05AF6E: andi.w #$f, d3
pub fn i_05af6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF72;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AF72: swap d2
pub fn i_05af72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF74;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF74: move.w d3, d2
pub fn i_05af74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF76;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF76: swap d2
pub fn i_05af76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF78;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF78: moveq #$0, d5
pub fn i_05af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF7A;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF7A: move.l (a0)+, d1
pub fn i_05af7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF7C;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05AF7C: moveq #$0, d5
pub fn i_05af7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF7E;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF7E: move.b (a0)+, d5
pub fn i_05af7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF80;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AF80: swap d5
pub fn i_05af80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF82;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF82: move.b (a0)+, d5
pub fn i_05af82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF84;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05AF84: neg.w d1
pub fn i_05af84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF86;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AF86: move.w d5, d6
pub fn i_05af86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF88;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AF88: lsl.w #$4, d6
pub fn i_05af88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF8A;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05AF8A: subq.w #$1, d6
pub fn i_05af8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF8C;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05AF8C: add.w d6, d1
pub fn i_05af8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF8E;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AF8E: moveq #$0, d3
pub fn i_05af8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF90;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF90: move.l d1, d6
pub fn i_05af90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF92;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF92: muls.w d7, d6
pub fn i_05af92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF94;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AF94: spl.b d3
pub fn i_05af94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF96;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05AF96: add.l d3, d6
pub fn i_05af96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF98;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05AF98: asr.l #$8, d6
pub fn i_05af98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF9A;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05AF9A: swap d7
pub fn i_05af9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF9C;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF9C: swap d1
pub fn i_05af9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AF9E;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AF9E: muls.w d7, d1
pub fn i_05af9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFA0;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AFA0: move.b d1, $616d(a5)
pub fn i_05afa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFA4;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05AFA4: lsl.l #$8, d1
pub fn i_05afa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFA6;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05AFA6: move.w d6, d1
pub fn i_05afa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFA8;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFA8: swap d7
pub fn i_05afa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFAA;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFAA: move.w d5, d6
pub fn i_05afaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFAC;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFAC: cmpi.b #$11, d6
pub fn i_05afac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFB0;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05AFB0: bcc.w $5afca
pub fn i_05afb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFB4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5AFCA; return 10; }
    8
}
/// $05AFB4: subq.b #$1, d6
pub fn i_05afb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFB6;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05AFB6: lsl.b #$4, d6
pub fn i_05afb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFB8;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05AFB8: swap d2
pub fn i_05afb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFBA;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFBA: or.b d2, d6
pub fn i_05afba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFBC;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AFBC: lea.l $5a8ba(pc), a4
pub fn i_05afbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFC0;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05AFC0: move.b (a4, d6.w), d6
pub fn i_05afc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFC4;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05AFC4: swap d2
pub fn i_05afc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFC6;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFC6: bra.w $5afd0
pub fn i_05afc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFCA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5AFD0;
    10
}
/// $05AFCA: swap d2
pub fn i_05afca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFCC;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFCC: move.w d2, d6
pub fn i_05afcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFCE;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFCE: swap d2
pub fn i_05afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFD0;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFD0: andi.b #$f0, d4
pub fn i_05afd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFD4;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05AFD4: or.b d4, d6
pub fn i_05afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFD6;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05AFD6: move.b d6, d4
pub fn i_05afd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFD8;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AFD8: swap d4
pub fn i_05afd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFDA;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFDA: move.b d6, d4
pub fn i_05afda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFDC;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05AFDC: swap d4
pub fn i_05afdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFDE;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFDE: move.w d7, d6
pub fn i_05afde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFE0;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFE0: mulu.w d5, d6
pub fn i_05afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFE2;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05AFE2: addi.w #$ff, d6
pub fn i_05afe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFE6;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AFE6: lsr.w #$8, d6
pub fn i_05afe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFE8;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05AFE8: swap d6
pub fn i_05afe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFEA;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFEA: move.w d5, d6
pub fn i_05afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFEC;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05AFEC: add.w d0, d1
pub fn i_05afec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFEE;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AFEE: swap d1
pub fn i_05afee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFF0;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFF0: swap d0
pub fn i_05aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFF2;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFF2: add.w d0, d1
pub fn i_05aff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFF4;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05AFF4: swap d0
pub fn i_05aff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFF6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFF6: andi.w #$1ff, d1
pub fn i_05aff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFFA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05AFFA: lsl.l #$7, d1
pub fn i_05affa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFFC;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05AFFC: or.l d6, d1
pub fn i_05affc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5AFFE;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05AFFE: clr.w d5
pub fn i_05affe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B000;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B000: swap d5
pub fn i_05b000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B002;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B002: subq.b #$1, d5
pub fn i_05b002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B004;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B004: add.w d6, d6
pub fn i_05b004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B006;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B006: adda.w d6, a0
pub fn i_05b006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B008;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B008: movea.l d4, a4
pub fn i_05b008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B00A;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05B00A: move.b $616e(a5), d3
pub fn i_05b00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B00E;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05B00E: sub.b d3, $616d(a5)
pub fn i_05b00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B012;
    cpu.ir = 0x972D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05B012: bcc.w $5b01e
pub fn i_05b012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B016;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B01E; return 10; }
    8
}
/// $05B016: swap d4
pub fn i_05b016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B018;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B018: tst.w d4
pub fn i_05b018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B01A;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05B01A: bmi.w $5b04a
pub fn i_05b01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B01E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5B04A; return 10; }
    8
}
/// $05B01E: move.w d4, d3
pub fn i_05b01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B020;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B020: andi.w #$f00, d3
pub fn i_05b020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B024;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B024: addi.w #$100, d3
pub fn i_05b024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B028;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B028: lsr.w #$1, d3
pub fn i_05b028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B02A;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05B02A: cmpi.w #$f800, d1
pub fn i_05b02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B02E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05B02E: bcc.w $5b03a
pub fn i_05b02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B032;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B03A; return 10; }
    8
}
/// $05B032: cmpi.w #$a000, d1
pub fn i_05b032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B036;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05B036: bcc.w $5b048
pub fn i_05b036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B03A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B048; return 10; }
    8
}
/// $05B03A: move.l a0, (a3)+
pub fn i_05b03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B03C;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B03C: move.w d2, (a3)+
pub fn i_05b03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B03E;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B03E: move.w d4, (a3)+
pub fn i_05b03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B040;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B040: move.l d1, (a3)+
pub fn i_05b040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B042;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B042: cmpa.l a2, a3
pub fn i_05b042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B044;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B044: bcc.w $5b05a
pub fn i_05b044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B048;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B05A; return 10; }
    8
}
/// $05B048: add.w d3, d1
pub fn i_05b048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B04A;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B04A: move.l a4, d4
pub fn i_05b04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B04C;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B04C: adda.w d6, a0
pub fn i_05b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B04E;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B04E: dbra d5, $5b00a
pub fn i_05b04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B052;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B052; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B052; return 14; }
    cpu.pc = 0x5B00A;
    10
}
/// $05B052: suba.w d6, a0
pub fn i_05b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B054;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B054: subq.b #$1, (a7)
pub fn i_05b054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B056;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05B056: bne.w $5af78
pub fn i_05b056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B05A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5AF78; return 10; }
    8
}
/// $05B05A: addq.w #$2, a7
pub fn i_05b05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B05C;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05B05C: lea.l $4258(a5), a2
pub fn i_05b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B060;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05B060: cmpa.l a2, a3
pub fn i_05b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B062;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B062: beq.w $5b06c
pub fn i_05b062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B066;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B06C; return 10; }
    8
}
/// $05B066: ori.b #$80, -$6(a3)
pub fn i_05b066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B06C;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05B06C: rts
pub fn i_05b06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B06E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B06E: cmpa.l a2, a3
pub fn i_05b06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B070;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B070: bcc.w $5b1b0
pub fn i_05b070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B074;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B1B0; return 10; }
    8
}
/// $05B074: move.b (a0)+, -(a7)
pub fn i_05b074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B076;
    cpu.ir = 0x1F18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write8(ad4, v3 as u8);
    14
}
/// $05B076: move.b (a0)+, d2
pub fn i_05b076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B078;
    cpu.ir = 0x1418;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05B078: eor.b d5, d2
pub fn i_05b078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B07A;
    cpu.ir = 0xBB02;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[5]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B07A: andi.w #$ff, d4
pub fn i_05b07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B07E;
    cpu.ir = 0x244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B07E: move.w d4, d7
pub fn i_05b07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B080;
    cpu.ir = 0x3E04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B080: addq.w #$1, d7
pub fn i_05b080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B082;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B082: swap d7
pub fn i_05b082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B084;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B084: move.w d3, d7
pub fn i_05b084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B086;
    cpu.ir = 0x3E03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B086: andi.w #$ff, d7
pub fn i_05b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B08A;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B08A: addq.w #$1, d7
pub fn i_05b08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B08C;
    cpu.ir = 0x5247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B08C: lsl.w #$4, d4
pub fn i_05b08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B08E;
    cpu.ir = 0xE94C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05B08E: eori.b #$f0, d4
pub fn i_05b08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B092;
    cpu.ir = 0xA04;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 ^ 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B092: move.b d4, $616e(a5)
pub fn i_05b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B096;
    cpu.ir = 0x1B44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05B096: move.b d3, d4
pub fn i_05b096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B098;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B098: andi.b #$f0, d4
pub fn i_05b098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B09C;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B09C: move.w d4, d6
pub fn i_05b09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B09E;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B09E: subi.w #$100, d4
pub fn i_05b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0A2;
    cpu.ir = 0x444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B0A2: swap d4
pub fn i_05b0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0A4;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0A4: move.w d6, d4
pub fn i_05b0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0A6;
    cpu.ir = 0x3806;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0A6: ori.l #$10001000, d4
pub fn i_05b0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0AC;
    cpu.ir = 0x84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = (v1 | 0x10001000) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[4] = r2;
    8
}
/// $05B0AC: andi.w #$f, d3
pub fn i_05b0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B0;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B0B0: swap d2
pub fn i_05b0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B2;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0B2: move.w d3, d2
pub fn i_05b0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B4;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0B4: swap d2
pub fn i_05b0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B6;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0B6: moveq #$0, d5
pub fn i_05b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0B8;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0B8: move.l (a0)+, d1
pub fn i_05b0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0BA;
    cpu.ir = 0x2218;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B0BA: moveq #$0, d5
pub fn i_05b0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0BC;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0BC: move.b (a0)+, d5
pub fn i_05b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0BE;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05B0BE: swap d5
pub fn i_05b0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C0;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0C0: move.b (a0)+, d5
pub fn i_05b0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C2;
    cpu.ir = 0x1A18;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05B0C2: swap d1
pub fn i_05b0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C4;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0C4: not.w d1
pub fn i_05b0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C6;
    cpu.ir = 0x4641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B0C6: swap d1
pub fn i_05b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0C8;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0C8: neg.w d1
pub fn i_05b0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0CA;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B0CA: move.w d5, d6
pub fn i_05b0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0CC;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0CC: lsl.w #$4, d6
pub fn i_05b0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0CE;
    cpu.ir = 0xE94E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05B0CE: subq.w #$1, d6
pub fn i_05b0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D0;
    cpu.ir = 0x5346;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B0D0: add.w d6, d1
pub fn i_05b0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D2;
    cpu.ir = 0xD246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B0D2: moveq #$0, d3
pub fn i_05b0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D4;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0D4: move.l d1, d6
pub fn i_05b0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D6;
    cpu.ir = 0x2C01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[6] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0D6: muls.w d7, d6
pub fn i_05b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0D8;
    cpu.ir = 0xCDC7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[6] as u16 as i16 as i32)) as u32;
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05B0D8: spl.b d3
pub fn i_05b0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0DA;
    cpu.ir = 0x5AC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Pl) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05B0DA: add.l d3, d6
pub fn i_05b0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0DC;
    cpu.ir = 0xDC83;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B0DC: asr.l #$8, d6
pub fn i_05b0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0DE;
    cpu.ir = 0xE086;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[6] = r;
    22
}
/// $05B0DE: swap d7
pub fn i_05b0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0E0;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0E0: swap d1
pub fn i_05b0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0E2;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0E2: muls.w d7, d1
pub fn i_05b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0E4;
    cpu.ir = 0xC3C7;
    let v1 = cpu.d[7] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05B0E4: move.b d1, $616d(a5)
pub fn i_05b0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0E8;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05B0E8: lsl.l #$8, d1
pub fn i_05b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0EA;
    cpu.ir = 0xE189;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 1, true);
    cpu.d[1] = r;
    22
}
/// $05B0EA: move.w d6, d1
pub fn i_05b0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0EC;
    cpu.ir = 0x3206;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0EC: swap d7
pub fn i_05b0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0EE;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0EE: move.w d5, d6
pub fn i_05b0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0F0;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B0F0: cmpi.b #$11, d6
pub fn i_05b0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0F4;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $05B0F4: bcc.w $5b10e
pub fn i_05b0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B10E; return 10; }
    8
}
/// $05B0F8: subq.b #$1, d6
pub fn i_05b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0FA;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B0FA: lsl.b #$4, d6
pub fn i_05b0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0FC;
    cpu.ir = 0xE90E;
    let h1 = cpu.d[6] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 1, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    14
}
/// $05B0FC: swap d2
pub fn i_05b0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B0FE;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B0FE: or.b d2, d6
pub fn i_05b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B100;
    cpu.ir = 0x8C02;
    let v1 = cpu.d[2] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05B100: lea.l $5a8ba(pc), a4
pub fn i_05b100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B104;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x5A8BA;
    16
}
/// $05B104: move.b (a4, d6.w), d6
pub fn i_05b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B108;
    cpu.ir = 0x1C34;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05B108: swap d2
pub fn i_05b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B10A;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B10A: bra.w $5b114
pub fn i_05b10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B10E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B114;
    10
}
/// $05B10E: swap d2
pub fn i_05b10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B110;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B110: move.w d2, d6
pub fn i_05b110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B112;
    cpu.ir = 0x3C02;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B112: swap d2
pub fn i_05b112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B114;
    cpu.ir = 0x4842;
    cpu.d[2] = cpu.d[2].rotate_left(16);
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B114: andi.b #$f0, d4
pub fn i_05b114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B118;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B118: or.b d4, d6
pub fn i_05b118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B11A;
    cpu.ir = 0x8C04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[6]) & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05B11A: move.b d6, d4
pub fn i_05b11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B11C;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B11C: swap d4
pub fn i_05b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B11E;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B11E: move.b d6, d4
pub fn i_05b11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B120;
    cpu.ir = 0x1806;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B120: swap d4
pub fn i_05b120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B122;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B122: move.w d7, d6
pub fn i_05b122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B124;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B124: mulu.w d5, d6
pub fn i_05b124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B126;
    cpu.ir = 0xCCC5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[6] & 0xFFFF);
    cpu.d[6] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05B126: addi.w #$ff, d6
pub fn i_05b126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B12A;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFF, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B12A: lsr.w #$8, d6
pub fn i_05b12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B12C;
    cpu.ir = 0xE04E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05B12C: swap d6
pub fn i_05b12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B12E;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B12E: move.w d5, d6
pub fn i_05b12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B130;
    cpu.ir = 0x3C05;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B130: add.w d0, d1
pub fn i_05b130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B132;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B132: swap d1
pub fn i_05b132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B134;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B134: swap d0
pub fn i_05b134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B136;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B136: add.w d0, d1
pub fn i_05b136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B138;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B138: swap d0
pub fn i_05b138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B13A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B13A: andi.w #$1ff, d1
pub fn i_05b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B13E;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B13E: lsl.l #$7, d1
pub fn i_05b13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B140;
    cpu.ir = 0xEF89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Long, 1, true);
    cpu.d[1] = r;
    20
}
/// $05B140: or.l d6, d1
pub fn i_05b140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B142;
    cpu.ir = 0x8286;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFFFFFF;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B142: clr.w d5
pub fn i_05b142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B144;
    cpu.ir = 0x4245;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B144: swap d5
pub fn i_05b144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B146;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B146: subq.b #$1, d5
pub fn i_05b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B148;
    cpu.ir = 0x5305;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B148: add.w d6, d6
pub fn i_05b148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B14A: adda.w d6, a0
pub fn i_05b14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14C;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B14C: movea.l d4, a4
pub fn i_05b14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B14E;
    cpu.ir = 0x2844;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $05B14E: move.b $616e(a5), d3
pub fn i_05b14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B152;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0x616E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05B152: add.b d3, $616d(a5)
pub fn i_05b152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B156;
    cpu.ir = 0xD72D;
    let ad1 = cpu.a[5].wrapping_add(0x616D);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[3] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $05B156: bcc.w $5b162
pub fn i_05b156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B162; return 10; }
    8
}
/// $05B15A: swap d4
pub fn i_05b15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B15C: tst.w d4
pub fn i_05b15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B15E;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05B15E: bmi.w $5b18e
pub fn i_05b15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B162;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x5B18E; return 10; }
    8
}
/// $05B162: move.w d4, d3
pub fn i_05b162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B164;
    cpu.ir = 0x3604;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B164: andi.w #$f00, d3
pub fn i_05b164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B168;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B168: addi.w #$100, d3
pub fn i_05b168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B16C;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B16C: lsr.w #$1, d3
pub fn i_05b16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B16E;
    cpu.ir = 0xE24B;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05B16E: sub.w d3, d1
pub fn i_05b16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B170;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B170: cmpi.w #$f800, d1
pub fn i_05b170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B174;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF800, Size::Word, false);
    8
}
/// $05B174: bcc.w $5b180
pub fn i_05b174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B178;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B180; return 10; }
    8
}
/// $05B178: cmpi.w #$a000, d1
pub fn i_05b178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B17C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA000, Size::Word, false);
    8
}
/// $05B17C: bcc.w $5b18e
pub fn i_05b17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B180;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B18E; return 10; }
    8
}
/// $05B180: move.l a0, (a3)+
pub fn i_05b180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B182;
    cpu.ir = 0x26C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B182: move.w d2, (a3)+
pub fn i_05b182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B184;
    cpu.ir = 0x36C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B184: move.w d4, (a3)+
pub fn i_05b184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B186;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05B186: move.l d1, (a3)+
pub fn i_05b186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B188;
    cpu.ir = 0x26C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B188: cmpa.l a2, a3
pub fn i_05b188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B18A;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B18A: bcc.w $5b19e
pub fn i_05b18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B18E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B19E; return 10; }
    8
}
/// $05B18E: move.l a4, d4
pub fn i_05b18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B190;
    cpu.ir = 0x280C;
    let v1 = cpu.a[4] & 0xFFFFFFFF;
    cpu.d[4] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B190: adda.w d6, a0
pub fn i_05b190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B192;
    cpu.ir = 0xD0C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B192: dbra d5, $5b14e
pub fn i_05b192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B196;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B196; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B196; return 14; }
    cpu.pc = 0x5B14E;
    10
}
/// $05B196: suba.w d6, a0
pub fn i_05b196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B198;
    cpu.ir = 0x90C6;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_sub((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B198: subq.b #$1, (a7)
pub fn i_05b198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B19A;
    cpu.ir = 0x5317;
    let ad1 = cpu.a[7];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $05B19A: bne.w $5b0b6
pub fn i_05b19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B19E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B0B6; return 10; }
    8
}
/// $05B19E: addq.w #$2, a7
pub fn i_05b19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A0;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $05B1A0: lea.l $4258(a5), a2
pub fn i_05b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A4;
    cpu.ir = 0x45ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[2] = ad1;
    16
}
/// $05B1A4: cmpa.l a2, a3
pub fn i_05b1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1A6;
    cpu.ir = 0xB7CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.a[3];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B1A6: beq.w $5b1b0
pub fn i_05b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B1B0; return 10; }
    8
}
/// $05B1AA: ori.b #$80, -$6(a3)
pub fn i_05b1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B0;
    cpu.ir = 0x2B;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFA);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x80) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05B1B0: rts
pub fn i_05b1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_DispatchSplashHook_05b1b2  ($05B1B2, 96 B, video/sprite_queue_render_05aa96.s)
/// $05B1B2: movem.l d0-d5/a0-a1, -(a7)
pub fn i_05b1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1B6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B1B6: cmpi.b #$0, $106ece.l
pub fn i_05b1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1BE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $05B1BE: bne.w $5b20c
pub fn i_05b1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1C2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1C2: move.l $106f50.l, d6
pub fn i_05b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1C8;
    cpu.ir = 0x2C39;
    let v2 = bus.read32(0x106F50);
    cpu.d[6] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05B1C8: swap d6
pub fn i_05b1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1CA;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B1CA: cmpi.w #$780, d6
pub fn i_05b1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1CE;
    cpu.ir = 0xC46;
    let v1 = cpu.d[6] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x780, Size::Word, false);
    8
}
/// $05B1CE: bgt.w $5b20c
pub fn i_05b1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1D2: move.w $22(a6), d0
pub fn i_05b1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1D6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B1D6: move.w $24(a6), d1
pub fn i_05b1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1DA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B1DA: addi.w #$8, d1
pub fn i_05b1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1DE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B1DE: jsr $43f02.l
pub fn i_05b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5B1E4);
    cpu.pc = 0x43F02;
    16
}
/// $05B1E4: move.b d0, d6
pub fn i_05b1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1E6;
    cpu.ir = 0x1C00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B1E6: cmpi.b #$3a, d6
pub fn i_05b1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1EA;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0x3A, Size::Byte, false);
    8
}
/// $05B1EA: bne.w $5b20c
pub fn i_05b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B20C; return 10; }
    8
}
/// $05B1EE: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1F2;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B1F2: movem.l d0-d5/a0-a1, -(a7)
pub fn i_05b1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1F6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B1F6: eori.b #$2, d5
pub fn i_05b1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1FA;
    cpu.ir = 0xA05;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B1FA: move.b #$7f, d3
pub fn i_05b1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B1FE;
    cpu.ir = 0x163C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05B1FE: jsr $5b212(pc)
pub fn i_05b1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B202;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5B202);
    cpu.pc = 0x5B212;
    16
}
/// $05B202: jsr $5a9ea(pc)
pub fn i_05b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B206;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5B206);
    cpu.pc = 0x5A9EA;
    16
}
/// $05B206: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B20A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B20A: rts
pub fn i_05b20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B20C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B20C: movem.l (a7)+, d0-d5/a0-a1
pub fn i_05b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B210;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B210: rts
pub fn i_05b210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B212;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_SplashScreenY_05b212  ($05B212, 32 B, video/sprite_queue_render_05aa96.s)
/// $05B212: movem.l d0-d4, -(a7)
pub fn i_05b212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B216;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05B216: move.w d0, d6
pub fn i_05b216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B218;
    cpu.ir = 0x3C00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B218: addq.w #$2, d6
pub fn i_05b218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B21A;
    cpu.ir = 0x5446;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B21A: move.w #$e8, d1
pub fn i_05b21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B21E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B21E: jsr $96a5a.l
pub fn i_05b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B224;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5B224);
    cpu.pc = 0x96A5A;
    16
}
/// $05B224: sub.w d1, d6
pub fn i_05b224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B226;
    cpu.ir = 0x9C41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B226: sub.w d6, d1
pub fn i_05b226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B228;
    cpu.ir = 0x9246;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B228: move.w d1, d6
pub fn i_05b228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B22A;
    cpu.ir = 0x3C01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B22A: movem.l (a7)+, d0-d4
pub fn i_05b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B22E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05B22E: move.w d6, d0
pub fn i_05b22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B230;
    cpu.ir = 0x3006;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B230: rts
pub fn i_05b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B232;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_SortAndRenderSCB1_05b232  ($05B232, 318 B, video/sprite_queue_render_05aa96.s)
/// $05B232: lea.l $108080.l, a5
pub fn i_05b232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B238;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05B238: lea.l $5424(a5), a0
pub fn i_05b238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B23C;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x5424);
    cpu.a[0] = ad1;
    16
}
/// $05B23C: move.w $6148(a5), d7
pub fn i_05b23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B240;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B240: beq.w $5b2de
pub fn i_05b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B244;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DE; return 10; }
    8
}
/// $05B244: move.w d7, d6
pub fn i_05b244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B246;
    cpu.ir = 0x3C07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B246: lsr.w #$1, d6
pub fn i_05b246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B248;
    cpu.ir = 0xE24E;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05B248: andi.w #$fffc, d6
pub fn i_05b248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B24C;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0xFFFC) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B24C: beq.w $5b2de
pub fn i_05b24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B250;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DE; return 10; }
    8
}
/// $05B250: move.l (a0, d6.w), d0
pub fn i_05b250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B254;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B254: move.w d6, d5
pub fn i_05b254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B256;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B256: move.w d5, d4
pub fn i_05b256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B258;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B258: add.w d4, d4
pub fn i_05b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B25A;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B25A: move.l (a0, d4.w), d1
pub fn i_05b25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B25E;
    cpu.ir = 0x2230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B25E: cmp.w d7, d4
pub fn i_05b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B260;
    cpu.ir = 0xB847;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B260: bcc.w $5b272
pub fn i_05b260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B264;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B272; return 10; }
    8
}
/// $05B264: move.l $4(a0, d4.w), d2
pub fn i_05b264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B268;
    cpu.ir = 0x2430;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B268: cmp.l d1, d2
pub fn i_05b268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B26A;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B26A: bcs.w $5b272
pub fn i_05b26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B26E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B272; return 10; }
    8
}
/// $05B26E: move.l d2, d1
pub fn i_05b26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B270;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B270: addq.w #$4, d4
pub fn i_05b270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B272;
    cpu.ir = 0x5844;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B272: cmp.l d0, d1
pub fn i_05b272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B274;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B274: bcs.w $5b284
pub fn i_05b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B278;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B284; return 10; }
    8
}
/// $05B278: move.l d1, (a0, d5.w)
pub fn i_05b278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B27C;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B27C: move.w d4, d5
pub fn i_05b27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B27E;
    cpu.ir = 0x3A04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B27E: add.w d4, d4
pub fn i_05b27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B280;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B280: cmp.w d4, d7
pub fn i_05b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B282;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B282: bcc.b $5b25a
pub fn i_05b282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B284;
    cpu.ir = 0x64D6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B25A; return 10; }
    8
}
/// $05B284: move.l d0, (a0, d5.w)
pub fn i_05b284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B288;
    cpu.ir = 0x2180;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B288: subq.w #$4, d6
pub fn i_05b288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B28A;
    cpu.ir = 0x5946;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B28A: bne.b $5b250
pub fn i_05b28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B28C;
    cpu.ir = 0x66C4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5B250; return 10; }
    8
}
/// $05B28C: move.w #$4, d6
pub fn i_05b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B290;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B290: move.l $4(a0), d1
pub fn i_05b290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B294;
    cpu.ir = 0x2228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B294: move.l (a0, d7.w), d0
pub fn i_05b294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B298;
    cpu.ir = 0x2030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B298: move.l d1, (a0, d7.w)
pub fn i_05b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B29C;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B29C: subq.w #$4, d7
pub fn i_05b29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B29E;
    cpu.ir = 0x5947;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B29E: cmp.w d6, d7
pub fn i_05b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A0;
    cpu.ir = 0xBE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2A0: beq.w $5b2da
pub fn i_05b2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5B2DA; return 10; }
    8
}
/// $05B2A4: move.w d6, d5
pub fn i_05b2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A6;
    cpu.ir = 0x3A06;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2A6: move.w d5, d4
pub fn i_05b2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2A8;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2A8: add.w d4, d4
pub fn i_05b2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2AA;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B2AA: move.l (a0, d4.w), d1
pub fn i_05b2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2AE;
    cpu.ir = 0x2230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2AE: cmp.w d7, d4
pub fn i_05b2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B0;
    cpu.ir = 0xB847;
    let v1 = cpu.d[7] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2B0: bcc.w $5b2c2
pub fn i_05b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B2C2; return 10; }
    8
}
/// $05B2B4: move.l $4(a0, d4.w), d2
pub fn i_05b2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2B8;
    cpu.ir = 0x2430;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2B8: cmp.l d1, d2
pub fn i_05b2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2BA;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B2BA: bcs.w $5b2c2
pub fn i_05b2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2C2; return 10; }
    8
}
/// $05B2BE: move.l d2, d1
pub fn i_05b2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C0;
    cpu.ir = 0x2202;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B2C0: addq.w #$4, d4
pub fn i_05b2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C2;
    cpu.ir = 0x5844;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B2C2: cmp.l d0, d1
pub fn i_05b2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C4;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B2C4: bcs.w $5b2d4
pub fn i_05b2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2C8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2D4; return 10; }
    8
}
/// $05B2C8: move.l d1, (a0, d5.w)
pub fn i_05b2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2CC;
    cpu.ir = 0x2181;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2CC: move.w d4, d5
pub fn i_05b2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2CE;
    cpu.ir = 0x3A04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B2CE: add.w d4, d4
pub fn i_05b2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D0;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B2D0: cmp.w d4, d7
pub fn i_05b2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D2;
    cpu.ir = 0xBE44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B2D2: bcc.b $5b2aa
pub fn i_05b2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D4;
    cpu.ir = 0x64D6;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B2AA; return 10; }
    8
}
/// $05B2D4: move.l d0, (a0, d5.w)
pub fn i_05b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2D8;
    cpu.ir = 0x2180;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[5] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05B2D8: bra.b $5b290
pub fn i_05b2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2DA;
    cpu.ir = 0x60B6;
    cpu.pc = 0x5B290;
    10
}
/// $05B2DA: move.l d0, $4(a0)
pub fn i_05b2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2DE;
    cpu.ir = 0x2140;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B2DE: lea.l $5428(a5), a0
pub fn i_05b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2E2;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x5428);
    cpu.a[0] = ad1;
    16
}
/// $05B2E2: lea.l $348(a0), a1
pub fn i_05b2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2E6;
    cpu.ir = 0x43E8;
    let ad1 = cpu.a[0].wrapping_add(0x348);
    cpu.a[1] = ad1;
    16
}
/// $05B2E6: move.w $614a(a5), d7
pub fn i_05b2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2EA;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B2EA: cmpi.w #$344, d7
pub fn i_05b2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2EE;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x344, Size::Word, false);
    8
}
/// $05B2EE: bcc.w $5b31a
pub fn i_05b2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B31A; return 10; }
    8
}
/// $05B2F2: adda.w d7, a0
pub fn i_05b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F4;
    cpu.ir = 0xD0C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B2F4: lea.l $4(a0), a2
pub fn i_05b2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2F8;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x4);
    cpu.a[2] = ad1;
    16
}
/// $05B2F8: movea.l a2, a3
pub fn i_05b2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FA;
    cpu.ir = 0x264A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $05B2FA: move.l (a2)+, d0
pub fn i_05b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FC;
    cpu.ir = 0x201A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B2FC: move.l -(a3), d1
pub fn i_05b2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B2FE;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B2FE: cmp.l d0, d1
pub fn i_05b2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B300;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B300: bls.w $5b316
pub fn i_05b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B304;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B316; return 10; }
    8
}
/// $05B304: move.l d1, $4(a3)
pub fn i_05b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B308;
    cpu.ir = 0x2741;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B308: move.l -(a3), d1
pub fn i_05b308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30A;
    cpu.ir = 0x2223;
    let ad2 = cpu.a[3].wrapping_sub(4); cpu.a[3] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B30A: cmpa.l a3, a0
pub fn i_05b30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30C;
    cpu.ir = 0xB1CB;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B30C: bhi.b $5b312
pub fn i_05b30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B30E;
    cpu.ir = 0x6204;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x5B312; return 10; }
    8
}
/// $05B30E: cmp.l d0, d1
pub fn i_05b30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B310;
    cpu.ir = 0xB280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05B310: bhi.b $5b304
pub fn i_05b310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B312;
    cpu.ir = 0x62F2;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x5B304; return 10; }
    8
}
/// $05B312: move.l d0, $4(a3)
pub fn i_05b312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B316;
    cpu.ir = 0x2740;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05B316: cmpa.l a1, a2
pub fn i_05b316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B318;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B318: bcs.b $5b2f8
pub fn i_05b318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B31A;
    cpu.ir = 0x65DE;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B2F8; return 10; }
    8
}
/// $05B31A: move.l a6, -(a7)
pub fn i_05b31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B31C;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05B31C: lea.l $3c0000.l, a4
pub fn i_05b31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B322;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x3C0000;
    20
}
/// $05B322: lea.l $614c(a5), a0
pub fn i_05b322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B326;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x614C);
    cpu.a[0] = ad1;
    16
}
/// $05B326: move.w $10e1f6.l, (a0)
pub fn i_05b326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B32C;
    cpu.ir = 0x30B9;
    let v2 = bus.read16(0x10E1F6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v2 as u16);
    20
}
/// $05B32C: move.w $10e1fa.l, $2(a0)
pub fn i_05b32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B334;
    cpu.ir = 0x3179;
    let v2 = bus.read16(0x10E1FA) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad3, v2 as u16);
    24
}
/// $05B334: lea.l $542a(a5), a1
pub fn i_05b334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B338;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05B338: move.w $6148(a5), d7
pub fn i_05b338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B33C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B33C: lea.l (a1, d7.w), a2
pub fn i_05b33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B340;
    cpu.ir = 0x45F1;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.a[2] = ad1;
    18
}
/// $05B340: bsr.w $5b370
pub fn i_05b340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B344;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B344);
    cpu.pc = 0x5B370;
    18
}
/// $05B344: lea.l $6158(a5), a0
pub fn i_05b344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B348;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x6158);
    cpu.a[0] = ad1;
    16
}
/// $05B348: move.w $10e1f8.l, (a0)
pub fn i_05b348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B34E;
    cpu.ir = 0x30B9;
    let v2 = bus.read16(0x10E1F8) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v2 as u16);
    20
}
/// $05B34E: move.w #$17c, $2(a0)
pub fn i_05b34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B354;
    cpu.ir = 0x317C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B354: lea.l $542a(a5), a1
pub fn i_05b354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B358;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05B358: lea.l $348(a1), a2
pub fn i_05b358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B35C;
    cpu.ir = 0x45E9;
    let ad1 = cpu.a[1].wrapping_add(0x348);
    cpu.a[2] = ad1;
    16
}
/// $05B35C: move.w $614a(a5), d7
pub fn i_05b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B360;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B360: adda.w d7, a1
pub fn i_05b360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B362;
    cpu.ir = 0xD2C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B362: bsr.w $5b370
pub fn i_05b362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B366;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B366);
    cpu.pc = 0x5B370;
    18
}
/// $05B366: movea.l (a7)+, a6
pub fn i_05b366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B368;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05B368: move.b #$ff, $616c(a5)
pub fn i_05b368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B36E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0x616C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05B36E: rts
pub fn i_05b36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B370;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_RenderRange_05b370  ($05B370, 144 B, video/sprite_queue_render_05aa96.s)
/// $05B370: move.w $4(a0), d2
pub fn i_05b370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B374;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B374: not.b $8(a0)
pub fn i_05b374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B378;
    cpu.ir = 0x4628;
    let ad1 = cpu.a[0].wrapping_add(0x8);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05B378: bpl.w $5b3be
pub fn i_05b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B37C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B3BE; return 10; }
    8
}
/// $05B37C: move.w (a0), d0
pub fn i_05b37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B37E;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B37E: move.w d0, d1
pub fn i_05b37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B380;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B380: lsl.w #$6, d1
pub fn i_05b380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B382;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B382: move.w $2(a0), d3
pub fn i_05b382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B386;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B386: cmpa.l a1, a2
pub fn i_05b386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B388;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B388: bls.w $5b3ba
pub fn i_05b388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B38C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B3BA; return 10; }
    8
}
/// $05B38C: move.w (a1), d5
pub fn i_05b38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B38E;
    cpu.ir = 0x3A11;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B38E: lea.l $4258(a5), a3
pub fn i_05b38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B392;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B392: adda.w d5, a3
pub fn i_05b392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B394;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B394: cmp.w d3, d0
pub fn i_05b394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B396;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B396: bcc.w $5b3fa
pub fn i_05b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B39A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B39A: cmp.w d2, d0
pub fn i_05b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B39C;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B39C: bcc.w $5b3fa
pub fn i_05b39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3A0: bsr.w $5b52e
pub fn i_05b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A4;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B3A4);
    cpu.pc = 0x5B52E;
    18
}
/// $05B3A4: addq.w #$1, d0
pub fn i_05b3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3A6;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B3A6: addi.w #$40, d1
pub fn i_05b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3AA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B3AA: tst.b $6(a3)
pub fn i_05b3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3AE;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B3AE: adda.w #$c, a3
pub fn i_05b3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B2;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3B2: bpl.b $5b394
pub fn i_05b3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B4;
    cpu.ir = 0x6AE0;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B394; return 10; }
    8
}
/// $05B3B4: adda.w #$4, a1
pub fn i_05b3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3B8;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3B8: bra.b $5b386
pub fn i_05b3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3BA;
    cpu.ir = 0x60CC;
    cpu.pc = 0x5B386;
    10
}
/// $05B3BA: bra.w $5b3fa
pub fn i_05b3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B3FA;
    10
}
/// $05B3BE: move.w $2(a0), d0
pub fn i_05b3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B3C2: move.w d0, d1
pub fn i_05b3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C4;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B3C4: lsl.w #$6, d1
pub fn i_05b3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C6;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B3C6: move.w (a0), d3
pub fn i_05b3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3C8;
    cpu.ir = 0x3610;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B3C8: cmpa.l a1, a2
pub fn i_05b3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3CA;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B3CA: bls.w $5b3fa
pub fn i_05b3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3CE;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3CE: subq.l #$4, a2
pub fn i_05b3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D0;
    cpu.ir = 0x598A;
    cpu.a[2] = cpu.a[2].wrapping_sub(4);
    8
}
/// $05B3D0: move.w (a2), d5
pub fn i_05b3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D2;
    cpu.ir = 0x3A12;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B3D2: lea.l $4258(a5), a3
pub fn i_05b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D6;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B3D6: adda.w d5, a3
pub fn i_05b3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3D8;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3D8: cmp.w d0, d3
pub fn i_05b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3DA;
    cpu.ir = 0xB640;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B3DA: bcc.w $5b3fa
pub fn i_05b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3DE: cmp.w d0, d2
pub fn i_05b3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E0;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B3E0: bcc.w $5b3fa
pub fn i_05b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B3FA; return 10; }
    8
}
/// $05B3E4: subq.w #$1, d0
pub fn i_05b3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3E6;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B3E6: subi.w #$40, d1
pub fn i_05b3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3EA;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B3EA: bsr.w $5b52e
pub fn i_05b3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3EE;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B3EE);
    cpu.pc = 0x5B52E;
    18
}
/// $05B3EE: tst.b $6(a3)
pub fn i_05b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F2;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B3F2: adda.w #$c, a3
pub fn i_05b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F6;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B3F6: bpl.b $5b3d8
pub fn i_05b3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3F8;
    cpu.ir = 0x6AE0;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B3D8; return 10; }
    8
}
/// $05B3F8: bra.b $5b3c8
pub fn i_05b3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3FA;
    cpu.ir = 0x60CE;
    cpu.pc = 0x5B3C8;
    10
}
/// $05B3FA: move.w d0, $6(a0)
pub fn i_05b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B3FE;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B3FE: rts
pub fn i_05b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B400;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteQueue_RenderSCB234_05b400  ($05B400, 302 B, video/sprite_queue_render_05aa96.s)
/// $05B400: tst.b $8(a0)
pub fn i_05b400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B404;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B404: bpl.w $5b496
pub fn i_05b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B408;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B496; return 10; }
    8
}
/// $05B408: move.w (a0), d0
pub fn i_05b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40A;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B40A: move.w d0, d1
pub fn i_05b40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B40C: lsl.w #$6, d1
pub fn i_05b40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B40E;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B40E: move.w $2(a0), d2
pub fn i_05b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B412;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B412: move.w #$40, d3
pub fn i_05b412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B416;
    cpu.ir = 0x363C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B416: cmpa.l a1, a2
pub fn i_05b416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B418;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B418: bls.w $5b45c
pub fn i_05b418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B41C;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B45C; return 10; }
    8
}
/// $05B41C: move.w (a1), d5
pub fn i_05b41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B41E;
    cpu.ir = 0x3A11;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B41E: lea.l $4258(a5), a3
pub fn i_05b41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B422;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B422: adda.w d5, a3
pub fn i_05b422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B424;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B424: cmp.w d2, d0
pub fn i_05b424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B426;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B426: bcc.w $5b524
pub fn i_05b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B42A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B524; return 10; }
    8
}
/// $05B42A: cmp.w $6(a0), d0
pub fn i_05b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B42E;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05B42E: bcc.w $5b444
pub fn i_05b42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B432;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B444; return 10; }
    8
}
/// $05B432: move.w #$8201, d7
pub fn i_05b432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B436;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x8201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B436: add.w d0, d7
pub fn i_05b436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B438;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B438: swap d7
pub fn i_05b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B43A;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B43A: move.w $8(a3), d7
pub fn i_05b43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B43E;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B43E: move.l d7, (a4)
pub fn i_05b43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B440;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B440: bra.w $5b448
pub fn i_05b440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B444;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B448;
    10
}
/// $05B444: bsr.w $5bf76
pub fn i_05b444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B448;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B448);
    cpu.pc = 0x5BF76;
    18
}
/// $05B448: addq.w #$1, d0
pub fn i_05b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B44A;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B44A: add.w d3, d1
pub fn i_05b44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B44C;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B44C: tst.b $6(a3)
pub fn i_05b44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B450;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B450: adda.w #$c, a3
pub fn i_05b450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B454;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B454: bpl.b $5b424
pub fn i_05b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B456;
    cpu.ir = 0x6ACE;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B424; return 10; }
    8
}
/// $05B456: adda.w #$4, a1
pub fn i_05b456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45A;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B45A: bra.b $5b416
pub fn i_05b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45C;
    cpu.ir = 0x60BA;
    cpu.pc = 0x5B416;
    10
}
/// $05B45C: move.w d2, d1
pub fn i_05b45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B45E;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B45E: sub.w d0, d1
pub fn i_05b45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B460;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B460: move.w d1, $a(a0)
pub fn i_05b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B464;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B464: move.w $4(a0), d1
pub fn i_05b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B468;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B468: bpl.w $5b46e
pub fn i_05b468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B46C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B46E; return 10; }
    8
}
/// $05B46C: move.w d2, d1
pub fn i_05b46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B46E;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B46E: cmp.w d0, d1
pub fn i_05b46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B470;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B470: bcc.w $5b476
pub fn i_05b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B474;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B476; return 10; }
    8
}
/// $05B474: move.w d0, d1
pub fn i_05b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B476;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B476: sub.w d1, d2
pub fn i_05b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B478;
    cpu.ir = 0x9441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B478: move.l #$8201, d7
pub fn i_05b478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B47E;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x8201;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B47E: add.w d1, d7
pub fn i_05b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B480;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B480: swap d7
pub fn i_05b480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B482;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B482: moveq #$1, d6
pub fn i_05b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B484;
    cpu.ir = 0x7C01;
    cpu.d[6] = 0x1;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B484: swap d6
pub fn i_05b484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B486;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B486: bra.w $5b48e
pub fn i_05b486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B48E;
    10
}
/// $05B48A: move.l d7, (a4)
pub fn i_05b48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B48C: add.l d6, d7
pub fn i_05b48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B48E;
    cpu.ir = 0xDE86;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B48E: dbra d2, $5b48a
pub fn i_05b48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B492;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B492; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B492; return 14; }
    cpu.pc = 0x5B48A;
    10
}
/// $05B492: bra.w $5b51e
pub fn i_05b492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B496;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B51E;
    10
}
/// $05B496: move.w $2(a0), d0
pub fn i_05b496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B49A: move.w d0, d1
pub fn i_05b49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B49C: lsl.w #$6, d1
pub fn i_05b49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B49E;
    cpu.ir = 0xED49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $05B49E: move.w (a0), d2
pub fn i_05b49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A0;
    cpu.ir = 0x3410;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B4A0: move.w #$40, d3
pub fn i_05b4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A4;
    cpu.ir = 0x363C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4A4: cmpa.l a1, a2
pub fn i_05b4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4A6;
    cpu.ir = 0xB5C9;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05B4A6: bls.w $5b4e8
pub fn i_05b4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x5B4E8; return 10; }
    8
}
/// $05B4AA: subq.l #$4, a2
pub fn i_05b4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AC;
    cpu.ir = 0x598A;
    cpu.a[2] = cpu.a[2].wrapping_sub(4);
    8
}
/// $05B4AC: move.w (a2), d5
pub fn i_05b4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4AE;
    cpu.ir = 0x3A12;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05B4AE: lea.l $4258(a5), a3
pub fn i_05b4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B2;
    cpu.ir = 0x47ED;
    let ad1 = cpu.a[5].wrapping_add(0x4258);
    cpu.a[3] = ad1;
    16
}
/// $05B4B2: adda.w d5, a3
pub fn i_05b4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B4;
    cpu.ir = 0xD6C5;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B4B4: cmp.w d0, d2
pub fn i_05b4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4B6;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B4B6: bcc.w $5b524
pub fn i_05b4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5B524; return 10; }
    8
}
/// $05B4BA: subq.w #$1, d0
pub fn i_05b4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BC;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B4BC: sub.w d3, d1
pub fn i_05b4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4BE;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4BE: cmp.w $6(a0), d0
pub fn i_05b4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4C2;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05B4C2: bcs.w $5b4d8
pub fn i_05b4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B4D8; return 10; }
    8
}
/// $05B4C6: move.w #$8201, d7
pub fn i_05b4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x8201;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4CA: add.w d0, d7
pub fn i_05b4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CC;
    cpu.ir = 0xDE40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4CC: swap d7
pub fn i_05b4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4CE;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B4CE: move.w $8(a3), d7
pub fn i_05b4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D2;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B4D2: move.l d7, (a4)
pub fn i_05b4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B4D4: bra.w $5b4dc
pub fn i_05b4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B4DC;
    10
}
/// $05B4D8: bsr.w $5bf76
pub fn i_05b4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4DC;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5B4DC);
    cpu.pc = 0x5BF76;
    18
}
/// $05B4DC: tst.b $6(a3)
pub fn i_05b4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E0;
    cpu.ir = 0x4A2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05B4E0: adda.w #$c, a3
pub fn i_05b4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E4;
    cpu.ir = 0xD6FC;
    let v1: u32 = 0xC;
    cpu.a[3] = cpu.a[3].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05B4E4: bpl.b $5b4b4
pub fn i_05b4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E6;
    cpu.ir = 0x6ACE;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B4B4; return 10; }
    8
}
/// $05B4E6: bra.b $5b4a4
pub fn i_05b4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4E8;
    cpu.ir = 0x60BC;
    cpu.pc = 0x5B4A4;
    10
}
/// $05B4E8: move.w d0, d1
pub fn i_05b4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4EA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4EA: sub.w d2, d1
pub fn i_05b4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4EC;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B4EC: move.w d1, $a(a0)
pub fn i_05b4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F0;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B4F0: move.w $4(a0), d1
pub fn i_05b4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F4;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B4F4: bpl.w $5b4fa
pub fn i_05b4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4F8;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x5B4FA; return 10; }
    8
}
/// $05B4F8: move.w d2, d1
pub fn i_05b4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4FA;
    cpu.ir = 0x3202;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B4FA: cmp.w d0, d1
pub fn i_05b4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B4FC;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05B4FC: bcs.w $5b502
pub fn i_05b4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B500;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5B502; return 10; }
    8
}
/// $05B500: move.w d0, d1
pub fn i_05b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B502;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B502: sub.w d2, d1
pub fn i_05b502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B504;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B504: move.l #$8201, d7
pub fn i_05b504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50A;
    cpu.ir = 0x2E3C;
    let v1: u32 = 0x8201;
    cpu.d[7] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B50A: add.w d2, d7
pub fn i_05b50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50C;
    cpu.ir = 0xDE42;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B50C: swap d7
pub fn i_05b50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B50E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B50E: moveq #$1, d6
pub fn i_05b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B510;
    cpu.ir = 0x7C01;
    cpu.d[6] = 0x1;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B510: swap d6
pub fn i_05b510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B512;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B512: bra.w $5b51a
pub fn i_05b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B516;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B51A;
    10
}
/// $05B516: move.l d7, (a4)
pub fn i_05b516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B518;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B518: add.l d6, d7
pub fn i_05b518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B51A;
    cpu.ir = 0xDE86;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B51A: dbra d1, $5b516
pub fn i_05b51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B51E;
    cpu.ir = 0x51C9;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5B51E; return 12; }
    let n = (cpu.d[1] as u16).wrapping_sub(1);
    cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5B51E; return 14; }
    cpu.pc = 0x5B516;
    10
}
/// $05B51E: move.w d0, $4(a0)
pub fn i_05b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B522;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B522: rts
pub fn i_05b522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B524;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B524: move.w d0, $4(a0)
pub fn i_05b524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B528;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05B528: clr.w $a(a0)
pub fn i_05b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B52C;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0xA);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05B52C: rts
pub fn i_05b52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B52E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SCB1_WriteTileColumn_05b52e  ($05B52E, 2632 B, video/sprite_queue_render_05aa96.s)
/// $05B52E: move.w $6(a3), d7
pub fn i_05b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B532;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B532: addq.b #$1, d7
pub fn i_05b532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B534;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B534: sne.b d6
pub fn i_05b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B536;
    cpu.ir = 0x56C6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05B536: andi.b #$4, d6
pub fn i_05b536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B53A;
    cpu.ir = 0x206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05B53A: rol.w #$7, d7
pub fn i_05b53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B53C;
    cpu.ir = 0xEF5F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 3, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $05B53C: andi.w #$8, d7
pub fn i_05b53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B540;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B540: or.b d6, d7
pub fn i_05b540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B542;
    cpu.ir = 0x8E06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05B542: jmp $5b546(pc, d7.w)
pub fn i_05b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B546;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B546_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B546: bra.w $5b556
pub fn i_05b546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B54A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B556;
    10
}
/// $05B54A: bra.w $5b962
pub fn i_05b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B54E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B962;
    10
}
/// $05B54E: bra.w $5b75c
pub fn i_05b54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B552;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B75C;
    10
}
/// $05B552: bra.w $5bc6c
pub fn i_05b552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B556;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BC6C;
    10
}
/// $05B556: move.w #$40, d7
pub fn i_05b556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55A;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B55A: add.w d1, d7
pub fn i_05b55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55C;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B55C: move.w d7, d5
pub fn i_05b55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B55E;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B55E: addq.w #$1, d5
pub fn i_05b55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B560;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B560: swap d5
pub fn i_05b560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B562;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B562: move.w $4(a3), d5
pub fn i_05b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B566;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B566: swap d7
pub fn i_05b566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B568;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B568: moveq #$2, d4
pub fn i_05b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56A;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B56A: swap d4
pub fn i_05b56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56C;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B56C: movea.l (a3), a6
pub fn i_05b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B56E;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B56E: move.w $a(a3), d6
pub fn i_05b56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B572;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B572: subq.b #$1, d6
pub fn i_05b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B574;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B574: andi.w #$1f, d6
pub fn i_05b574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B578;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B578: add.w d6, d6
pub fn i_05b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B57A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B57A: add.w d6, d6
pub fn i_05b57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B57C;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B57C: jmp $5b580(pc, d6.w)
pub fn i_05b57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B580;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B580_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B580: bra.w $5b732
pub fn i_05b580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B584;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B732;
    10
}
/// $05B584: bra.w $5b728
pub fn i_05b584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B588;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B728;
    10
}
/// $05B588: bra.w $5b71e
pub fn i_05b588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B58C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B71E;
    10
}
/// $05B58C: bra.w $5b714
pub fn i_05b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B590;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B714;
    10
}
/// $05B590: bra.w $5b70a
pub fn i_05b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B594;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B70A;
    10
}
/// $05B594: bra.w $5b700
pub fn i_05b594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B598;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B700;
    10
}
/// $05B598: bra.w $5b6f6
pub fn i_05b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B59C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6F6;
    10
}
/// $05B59C: bra.w $5b6ec
pub fn i_05b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6EC;
    10
}
/// $05B5A0: bra.w $5b6e2
pub fn i_05b5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6E2;
    10
}
/// $05B5A4: bra.w $5b6d8
pub fn i_05b5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6D8;
    10
}
/// $05B5A8: bra.w $5b6ce
pub fn i_05b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6CE;
    10
}
/// $05B5AC: bra.w $5b6c4
pub fn i_05b5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6C4;
    10
}
/// $05B5B0: bra.w $5b6ba
pub fn i_05b5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6BA;
    10
}
/// $05B5B4: bra.w $5b6b0
pub fn i_05b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6B0;
    10
}
/// $05B5B8: bra.w $5b6a6
pub fn i_05b5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B6A6;
    10
}
/// $05B5BC: bra.w $5b69c
pub fn i_05b5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B69C;
    10
}
/// $05B5C0: bra.w $5b692
pub fn i_05b5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B692;
    10
}
/// $05B5C4: bra.w $5b688
pub fn i_05b5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B688;
    10
}
/// $05B5C8: bra.w $5b67e
pub fn i_05b5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B67E;
    10
}
/// $05B5CC: bra.w $5b674
pub fn i_05b5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B674;
    10
}
/// $05B5D0: bra.w $5b66a
pub fn i_05b5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B66A;
    10
}
/// $05B5D4: bra.w $5b660
pub fn i_05b5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B660;
    10
}
/// $05B5D8: bra.w $5b656
pub fn i_05b5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B656;
    10
}
/// $05B5DC: bra.w $5b64c
pub fn i_05b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B64C;
    10
}
/// $05B5E0: bra.w $5b642
pub fn i_05b5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B642;
    10
}
/// $05B5E4: bra.w $5b638
pub fn i_05b5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B638;
    10
}
/// $05B5E8: bra.w $5b62e
pub fn i_05b5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B62E;
    10
}
/// $05B5EC: bra.w $5b624
pub fn i_05b5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B624;
    10
}
/// $05B5F0: bra.w $5b61a
pub fn i_05b5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B61A;
    10
}
/// $05B5F4: bra.w $5b610
pub fn i_05b5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B610;
    10
}
/// $05B5F8: bra.w $5b606
pub fn i_05b5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B606;
    10
}
/// $05B5FC: move.w (a6)+, d7
pub fn i_05b5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B5FE;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B5FE: move.l d7, (a4)
pub fn i_05b5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B600;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B600: add.l d4, d7
pub fn i_05b600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B602;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B602: move.l d5, (a4)
pub fn i_05b602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B604;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B604: add.l d4, d5
pub fn i_05b604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B606;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B606: move.w (a6)+, d7
pub fn i_05b606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B608;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B608: move.l d7, (a4)
pub fn i_05b608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B60A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B60A: add.l d4, d7
pub fn i_05b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B60C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B60C: move.l d5, (a4)
pub fn i_05b60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B60E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B60E: add.l d4, d5
pub fn i_05b60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B610;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B610: move.w (a6)+, d7
pub fn i_05b610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B612;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B612: move.l d7, (a4)
pub fn i_05b612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B614;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B614: add.l d4, d7
pub fn i_05b614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B616;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B616: move.l d5, (a4)
pub fn i_05b616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B618;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B618: add.l d4, d5
pub fn i_05b618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B61A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B61A: move.w (a6)+, d7
pub fn i_05b61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B61C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B61C: move.l d7, (a4)
pub fn i_05b61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B61E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B61E: add.l d4, d7
pub fn i_05b61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B620;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B620: move.l d5, (a4)
pub fn i_05b620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B622;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B622: add.l d4, d5
pub fn i_05b622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B624;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B624: move.w (a6)+, d7
pub fn i_05b624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B626;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B626: move.l d7, (a4)
pub fn i_05b626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B628;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B628: add.l d4, d7
pub fn i_05b628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B62A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B62A: move.l d5, (a4)
pub fn i_05b62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B62C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B62C: add.l d4, d5
pub fn i_05b62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B62E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B62E: move.w (a6)+, d7
pub fn i_05b62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B630;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B630: move.l d7, (a4)
pub fn i_05b630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B632;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B632: add.l d4, d7
pub fn i_05b632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B634;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B634: move.l d5, (a4)
pub fn i_05b634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B636;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B636: add.l d4, d5
pub fn i_05b636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B638;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B638: move.w (a6)+, d7
pub fn i_05b638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B63A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B63A: move.l d7, (a4)
pub fn i_05b63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B63C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B63C: add.l d4, d7
pub fn i_05b63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B63E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B63E: move.l d5, (a4)
pub fn i_05b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B640;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B640: add.l d4, d5
pub fn i_05b640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B642;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B642: move.w (a6)+, d7
pub fn i_05b642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B644;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B644: move.l d7, (a4)
pub fn i_05b644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B646;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B646: add.l d4, d7
pub fn i_05b646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B648;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B648: move.l d5, (a4)
pub fn i_05b648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B64A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B64A: add.l d4, d5
pub fn i_05b64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B64C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B64C: move.w (a6)+, d7
pub fn i_05b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B64E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B64E: move.l d7, (a4)
pub fn i_05b64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B650;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B650: add.l d4, d7
pub fn i_05b650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B652;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B652: move.l d5, (a4)
pub fn i_05b652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B654;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B654: add.l d4, d5
pub fn i_05b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B656;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B656: move.w (a6)+, d7
pub fn i_05b656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B658;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B658: move.l d7, (a4)
pub fn i_05b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B65A: add.l d4, d7
pub fn i_05b65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B65C: move.l d5, (a4)
pub fn i_05b65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B65E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B65E: add.l d4, d5
pub fn i_05b65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B660;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B660: move.w (a6)+, d7
pub fn i_05b660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B662;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B662: move.l d7, (a4)
pub fn i_05b662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B664;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B664: add.l d4, d7
pub fn i_05b664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B666;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B666: move.l d5, (a4)
pub fn i_05b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B668;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B668: add.l d4, d5
pub fn i_05b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B66A: move.w (a6)+, d7
pub fn i_05b66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B66C: move.l d7, (a4)
pub fn i_05b66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B66E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B66E: add.l d4, d7
pub fn i_05b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B670;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B670: move.l d5, (a4)
pub fn i_05b670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B672;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B672: add.l d4, d5
pub fn i_05b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B674;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B674: move.w (a6)+, d7
pub fn i_05b674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B676;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B676: move.l d7, (a4)
pub fn i_05b676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B678;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B678: add.l d4, d7
pub fn i_05b678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B67A: move.l d5, (a4)
pub fn i_05b67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B67C: add.l d4, d5
pub fn i_05b67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B67E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B67E: move.w (a6)+, d7
pub fn i_05b67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B680;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B680: move.l d7, (a4)
pub fn i_05b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B682;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B682: add.l d4, d7
pub fn i_05b682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B684;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B684: move.l d5, (a4)
pub fn i_05b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B686;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B686: add.l d4, d5
pub fn i_05b686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B688;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B688: move.w (a6)+, d7
pub fn i_05b688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B68A: move.l d7, (a4)
pub fn i_05b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B68C: add.l d4, d7
pub fn i_05b68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B68E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B68E: move.l d5, (a4)
pub fn i_05b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B690;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B690: add.l d4, d5
pub fn i_05b690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B692;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B692: move.w (a6)+, d7
pub fn i_05b692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B694;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B694: move.l d7, (a4)
pub fn i_05b694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B696;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B696: add.l d4, d7
pub fn i_05b696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B698;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B698: move.l d5, (a4)
pub fn i_05b698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B69A: add.l d4, d5
pub fn i_05b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B69C: move.w (a6)+, d7
pub fn i_05b69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B69E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B69E: move.l d7, (a4)
pub fn i_05b69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6A0: add.l d4, d7
pub fn i_05b6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6A2: move.l d5, (a4)
pub fn i_05b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6A4: add.l d4, d5
pub fn i_05b6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6A6: move.w (a6)+, d7
pub fn i_05b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6A8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6A8: move.l d7, (a4)
pub fn i_05b6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6AA: add.l d4, d7
pub fn i_05b6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6AC: move.l d5, (a4)
pub fn i_05b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6AE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6AE: add.l d4, d5
pub fn i_05b6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6B0: move.w (a6)+, d7
pub fn i_05b6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B2;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6B2: move.l d7, (a4)
pub fn i_05b6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6B4: add.l d4, d7
pub fn i_05b6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6B6: move.l d5, (a4)
pub fn i_05b6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6B8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6B8: add.l d4, d5
pub fn i_05b6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6BA: move.w (a6)+, d7
pub fn i_05b6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BC;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6BC: move.l d7, (a4)
pub fn i_05b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6BE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6BE: add.l d4, d7
pub fn i_05b6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6C0: move.l d5, (a4)
pub fn i_05b6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6C2: add.l d4, d5
pub fn i_05b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6C4: move.w (a6)+, d7
pub fn i_05b6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C6;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6C6: move.l d7, (a4)
pub fn i_05b6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6C8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6C8: add.l d4, d7
pub fn i_05b6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6CA: move.l d5, (a4)
pub fn i_05b6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6CC: add.l d4, d5
pub fn i_05b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6CE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6CE: move.w (a6)+, d7
pub fn i_05b6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6D0: move.l d7, (a4)
pub fn i_05b6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6D2: add.l d4, d7
pub fn i_05b6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6D4: move.l d5, (a4)
pub fn i_05b6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6D6: add.l d4, d5
pub fn i_05b6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6D8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6D8: move.w (a6)+, d7
pub fn i_05b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6DA: move.l d7, (a4)
pub fn i_05b6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6DC: add.l d4, d7
pub fn i_05b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6DE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6DE: move.l d5, (a4)
pub fn i_05b6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6E0: add.l d4, d5
pub fn i_05b6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6E2: move.w (a6)+, d7
pub fn i_05b6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E4;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6E4: move.l d7, (a4)
pub fn i_05b6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6E6: add.l d4, d7
pub fn i_05b6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6E8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6E8: move.l d5, (a4)
pub fn i_05b6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6EA: add.l d4, d5
pub fn i_05b6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6EC: move.w (a6)+, d7
pub fn i_05b6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6EE;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6EE: move.l d7, (a4)
pub fn i_05b6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6F0: add.l d4, d7
pub fn i_05b6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6F2: move.l d5, (a4)
pub fn i_05b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6F4: add.l d4, d5
pub fn i_05b6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B6F6: move.w (a6)+, d7
pub fn i_05b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6F8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B6F8: move.l d7, (a4)
pub fn i_05b6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6FA: add.l d4, d7
pub fn i_05b6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B6FC: move.l d5, (a4)
pub fn i_05b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B6FE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B6FE: add.l d4, d5
pub fn i_05b6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B700;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B700: move.w (a6)+, d7
pub fn i_05b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B702;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B702: move.l d7, (a4)
pub fn i_05b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B704;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B704: add.l d4, d7
pub fn i_05b704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B706;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B706: move.l d5, (a4)
pub fn i_05b706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B708;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B708: add.l d4, d5
pub fn i_05b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B70A: move.w (a6)+, d7
pub fn i_05b70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B70C: move.l d7, (a4)
pub fn i_05b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B70E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B70E: add.l d4, d7
pub fn i_05b70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B710;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B710: move.l d5, (a4)
pub fn i_05b710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B712;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B712: add.l d4, d5
pub fn i_05b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B714;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B714: move.w (a6)+, d7
pub fn i_05b714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B716;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B716: move.l d7, (a4)
pub fn i_05b716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B718;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B718: add.l d4, d7
pub fn i_05b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B71A: move.l d5, (a4)
pub fn i_05b71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B71C: add.l d4, d5
pub fn i_05b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B71E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B71E: move.w (a6)+, d7
pub fn i_05b71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B720;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B720: move.l d7, (a4)
pub fn i_05b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B722;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B722: add.l d4, d7
pub fn i_05b722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B724;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B724: move.l d5, (a4)
pub fn i_05b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B726;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B726: add.l d4, d5
pub fn i_05b726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B728;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B728: move.w (a6)+, d7
pub fn i_05b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B72A: move.l d7, (a4)
pub fn i_05b72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B72C: add.l d4, d7
pub fn i_05b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B72E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B72E: move.l d5, (a4)
pub fn i_05b72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B730;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B730: add.l d4, d5
pub fn i_05b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B732;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B732: move.w (a6)+, d7
pub fn i_05b732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B734;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05B734: move.l d7, (a4)
pub fn i_05b734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B736;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B736: add.l d4, d7
pub fn i_05b736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B738;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B738: move.l d5, (a4)
pub fn i_05b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B73A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B73A: add.l d4, d5
pub fn i_05b73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B73C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B73C: move.w #$8401, d6
pub fn i_05b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B740;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B740: add.w d0, d6
pub fn i_05b740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B742;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B742: swap d6
pub fn i_05b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B744;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B744: move.w $a(a3), d6
pub fn i_05b744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B748;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B748: move.l #$2000000, d5
pub fn i_05b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B74E;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B74E: move.l d6, (a4)
pub fn i_05b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B750;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B750: sub.l d5, d6
pub fn i_05b750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B752;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B752: sub.l d5, d6
pub fn i_05b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B754;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B754: move.w $6(a3), d6
pub fn i_05b754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B758;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B758: move.l d6, (a4)
pub fn i_05b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B75A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B75A: rts
pub fn i_05b75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B75C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B75C: move.w #$40, d7
pub fn i_05b75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B760;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B760: add.w d1, d7
pub fn i_05b760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B762;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B762: move.w d7, d5
pub fn i_05b762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B764;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B764: addq.w #$1, d5
pub fn i_05b764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B766;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B766: swap d5
pub fn i_05b766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B768;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B768: move.w $4(a3), d5
pub fn i_05b768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B76C;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B76C: swap d7
pub fn i_05b76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B76E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B76E: moveq #$2, d4
pub fn i_05b76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B770;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B770: swap d4
pub fn i_05b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B772;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B772: movea.l (a3), a6
pub fn i_05b772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B774;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B774: move.w $a(a3), d6
pub fn i_05b774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B778;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B778: subq.b #$1, d6
pub fn i_05b778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B77A;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B77A: andi.w #$1f, d6
pub fn i_05b77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B77E;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B77E: add.w d6, d6
pub fn i_05b77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B780;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B780: add.w d6, d6
pub fn i_05b780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B782;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B782: jmp $5b786(pc, d6.w)
pub fn i_05b782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B786;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B786_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B786: bra.w $5b938
pub fn i_05b786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B78A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B938;
    10
}
/// $05B78A: bra.w $5b92e
pub fn i_05b78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B78E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B92E;
    10
}
/// $05B78E: bra.w $5b924
pub fn i_05b78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B792;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B924;
    10
}
/// $05B792: bra.w $5b91a
pub fn i_05b792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B796;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B91A;
    10
}
/// $05B796: bra.w $5b910
pub fn i_05b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B79A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B910;
    10
}
/// $05B79A: bra.w $5b906
pub fn i_05b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B79E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B906;
    10
}
/// $05B79E: bra.w $5b8fc
pub fn i_05b79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8FC;
    10
}
/// $05B7A2: bra.w $5b8f2
pub fn i_05b7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8F2;
    10
}
/// $05B7A6: bra.w $5b8e8
pub fn i_05b7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8E8;
    10
}
/// $05B7AA: bra.w $5b8de
pub fn i_05b7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8DE;
    10
}
/// $05B7AE: bra.w $5b8d4
pub fn i_05b7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8D4;
    10
}
/// $05B7B2: bra.w $5b8ca
pub fn i_05b7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8CA;
    10
}
/// $05B7B6: bra.w $5b8c0
pub fn i_05b7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8C0;
    10
}
/// $05B7BA: bra.w $5b8b6
pub fn i_05b7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8B6;
    10
}
/// $05B7BE: bra.w $5b8ac
pub fn i_05b7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8AC;
    10
}
/// $05B7C2: bra.w $5b8a2
pub fn i_05b7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B8A2;
    10
}
/// $05B7C6: bra.w $5b898
pub fn i_05b7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B898;
    10
}
/// $05B7CA: bra.w $5b88e
pub fn i_05b7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B88E;
    10
}
/// $05B7CE: bra.w $5b884
pub fn i_05b7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B884;
    10
}
/// $05B7D2: bra.w $5b87a
pub fn i_05b7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B87A;
    10
}
/// $05B7D6: bra.w $5b870
pub fn i_05b7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7DA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B870;
    10
}
/// $05B7DA: bra.w $5b866
pub fn i_05b7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B866;
    10
}
/// $05B7DE: bra.w $5b85c
pub fn i_05b7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B85C;
    10
}
/// $05B7E2: bra.w $5b852
pub fn i_05b7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B852;
    10
}
/// $05B7E6: bra.w $5b848
pub fn i_05b7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B848;
    10
}
/// $05B7EA: bra.w $5b83e
pub fn i_05b7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B83E;
    10
}
/// $05B7EE: bra.w $5b834
pub fn i_05b7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B834;
    10
}
/// $05B7F2: bra.w $5b82a
pub fn i_05b7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B82A;
    10
}
/// $05B7F6: bra.w $5b820
pub fn i_05b7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B820;
    10
}
/// $05B7FA: bra.w $5b816
pub fn i_05b7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B7FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B816;
    10
}
/// $05B7FE: bra.w $5b80c
pub fn i_05b7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B802;
    cpu.ir = 0x6000;
    cpu.pc = 0x5B80C;
    10
}
/// $05B802: move.w -(a6), d7
pub fn i_05b802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B804;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B804: move.l d7, (a4)
pub fn i_05b804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B806;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B806: add.l d4, d7
pub fn i_05b806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B808;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B808: move.l d5, (a4)
pub fn i_05b808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B80A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B80A: add.l d4, d5
pub fn i_05b80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B80C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B80C: move.w -(a6), d7
pub fn i_05b80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B80E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B80E: move.l d7, (a4)
pub fn i_05b80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B810;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B810: add.l d4, d7
pub fn i_05b810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B812;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B812: move.l d5, (a4)
pub fn i_05b812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B814;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B814: add.l d4, d5
pub fn i_05b814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B816;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B816: move.w -(a6), d7
pub fn i_05b816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B818;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B818: move.l d7, (a4)
pub fn i_05b818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B81A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B81A: add.l d4, d7
pub fn i_05b81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B81C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B81C: move.l d5, (a4)
pub fn i_05b81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B81E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B81E: add.l d4, d5
pub fn i_05b81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B820;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B820: move.w -(a6), d7
pub fn i_05b820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B822;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B822: move.l d7, (a4)
pub fn i_05b822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B824;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B824: add.l d4, d7
pub fn i_05b824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B826;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B826: move.l d5, (a4)
pub fn i_05b826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B828;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B828: add.l d4, d5
pub fn i_05b828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B82A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B82A: move.w -(a6), d7
pub fn i_05b82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B82C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B82C: move.l d7, (a4)
pub fn i_05b82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B82E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B82E: add.l d4, d7
pub fn i_05b82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B830;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B830: move.l d5, (a4)
pub fn i_05b830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B832;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B832: add.l d4, d5
pub fn i_05b832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B834;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B834: move.w -(a6), d7
pub fn i_05b834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B836;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B836: move.l d7, (a4)
pub fn i_05b836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B838;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B838: add.l d4, d7
pub fn i_05b838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B83A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B83A: move.l d5, (a4)
pub fn i_05b83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B83C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B83C: add.l d4, d5
pub fn i_05b83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B83E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B83E: move.w -(a6), d7
pub fn i_05b83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B840;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B840: move.l d7, (a4)
pub fn i_05b840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B842;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B842: add.l d4, d7
pub fn i_05b842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B844;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B844: move.l d5, (a4)
pub fn i_05b844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B846;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B846: add.l d4, d5
pub fn i_05b846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B848;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B848: move.w -(a6), d7
pub fn i_05b848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B84A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B84A: move.l d7, (a4)
pub fn i_05b84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B84C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B84C: add.l d4, d7
pub fn i_05b84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B84E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B84E: move.l d5, (a4)
pub fn i_05b84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B850;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B850: add.l d4, d5
pub fn i_05b850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B852;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B852: move.w -(a6), d7
pub fn i_05b852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B854;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B854: move.l d7, (a4)
pub fn i_05b854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B856;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B856: add.l d4, d7
pub fn i_05b856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B858;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B858: move.l d5, (a4)
pub fn i_05b858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B85A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B85A: add.l d4, d5
pub fn i_05b85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B85C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B85C: move.w -(a6), d7
pub fn i_05b85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B85E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B85E: move.l d7, (a4)
pub fn i_05b85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B860;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B860: add.l d4, d7
pub fn i_05b860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B862;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B862: move.l d5, (a4)
pub fn i_05b862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B864;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B864: add.l d4, d5
pub fn i_05b864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B866;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B866: move.w -(a6), d7
pub fn i_05b866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B868;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B868: move.l d7, (a4)
pub fn i_05b868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B86A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B86A: add.l d4, d7
pub fn i_05b86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B86C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B86C: move.l d5, (a4)
pub fn i_05b86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B86E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B86E: add.l d4, d5
pub fn i_05b86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B870;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B870: move.w -(a6), d7
pub fn i_05b870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B872;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B872: move.l d7, (a4)
pub fn i_05b872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B874;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B874: add.l d4, d7
pub fn i_05b874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B876;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B876: move.l d5, (a4)
pub fn i_05b876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B878;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B878: add.l d4, d5
pub fn i_05b878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B87A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B87A: move.w -(a6), d7
pub fn i_05b87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B87C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B87C: move.l d7, (a4)
pub fn i_05b87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B87E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B87E: add.l d4, d7
pub fn i_05b87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B880;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B880: move.l d5, (a4)
pub fn i_05b880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B882;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B882: add.l d4, d5
pub fn i_05b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B884;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B884: move.w -(a6), d7
pub fn i_05b884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B886;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B886: move.l d7, (a4)
pub fn i_05b886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B888;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B888: add.l d4, d7
pub fn i_05b888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B88A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B88A: move.l d5, (a4)
pub fn i_05b88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B88C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B88C: add.l d4, d5
pub fn i_05b88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B88E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B88E: move.w -(a6), d7
pub fn i_05b88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B890;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B890: move.l d7, (a4)
pub fn i_05b890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B892;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B892: add.l d4, d7
pub fn i_05b892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B894;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B894: move.l d5, (a4)
pub fn i_05b894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B896;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B896: add.l d4, d5
pub fn i_05b896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B898;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B898: move.w -(a6), d7
pub fn i_05b898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B89A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B89A: move.l d7, (a4)
pub fn i_05b89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B89C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B89C: add.l d4, d7
pub fn i_05b89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B89E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B89E: move.l d5, (a4)
pub fn i_05b89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8A0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8A0: add.l d4, d5
pub fn i_05b8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8A2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8A2: move.w -(a6), d7
pub fn i_05b8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8A4;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8A4: move.l d7, (a4)
pub fn i_05b8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8A6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8A6: add.l d4, d7
pub fn i_05b8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8A8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8A8: move.l d5, (a4)
pub fn i_05b8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8AA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8AA: add.l d4, d5
pub fn i_05b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8AC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8AC: move.w -(a6), d7
pub fn i_05b8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8AE;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8AE: move.l d7, (a4)
pub fn i_05b8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8B0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8B0: add.l d4, d7
pub fn i_05b8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8B2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8B2: move.l d5, (a4)
pub fn i_05b8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8B4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8B4: add.l d4, d5
pub fn i_05b8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8B6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8B6: move.w -(a6), d7
pub fn i_05b8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8B8;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8B8: move.l d7, (a4)
pub fn i_05b8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8BA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8BA: add.l d4, d7
pub fn i_05b8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8BC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8BC: move.l d5, (a4)
pub fn i_05b8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8BE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8BE: add.l d4, d5
pub fn i_05b8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8C0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8C0: move.w -(a6), d7
pub fn i_05b8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8C2;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8C2: move.l d7, (a4)
pub fn i_05b8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8C4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8C4: add.l d4, d7
pub fn i_05b8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8C6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8C6: move.l d5, (a4)
pub fn i_05b8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8C8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8C8: add.l d4, d5
pub fn i_05b8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8CA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8CA: move.w -(a6), d7
pub fn i_05b8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8CC;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8CC: move.l d7, (a4)
pub fn i_05b8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8CE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8CE: add.l d4, d7
pub fn i_05b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8D0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8D0: move.l d5, (a4)
pub fn i_05b8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8D2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8D2: add.l d4, d5
pub fn i_05b8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8D4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8D4: move.w -(a6), d7
pub fn i_05b8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8D6;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8D6: move.l d7, (a4)
pub fn i_05b8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8D8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8D8: add.l d4, d7
pub fn i_05b8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8DA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8DA: move.l d5, (a4)
pub fn i_05b8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8DC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8DC: add.l d4, d5
pub fn i_05b8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8DE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8DE: move.w -(a6), d7
pub fn i_05b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8E0: move.l d7, (a4)
pub fn i_05b8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8E2: add.l d4, d7
pub fn i_05b8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8E4: move.l d5, (a4)
pub fn i_05b8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8E6: add.l d4, d5
pub fn i_05b8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8E8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8E8: move.w -(a6), d7
pub fn i_05b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8EA: move.l d7, (a4)
pub fn i_05b8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8EC: add.l d4, d7
pub fn i_05b8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8EE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8EE: move.l d5, (a4)
pub fn i_05b8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8F0: add.l d4, d5
pub fn i_05b8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8F2: move.w -(a6), d7
pub fn i_05b8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F4;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8F4: move.l d7, (a4)
pub fn i_05b8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8F6: add.l d4, d7
pub fn i_05b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8F8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B8F8: move.l d5, (a4)
pub fn i_05b8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B8FA: add.l d4, d5
pub fn i_05b8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B8FC: move.w -(a6), d7
pub fn i_05b8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B8FE;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B8FE: move.l d7, (a4)
pub fn i_05b8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B900;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B900: add.l d4, d7
pub fn i_05b900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B902;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B902: move.l d5, (a4)
pub fn i_05b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B904;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B904: add.l d4, d5
pub fn i_05b904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B906;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B906: move.w -(a6), d7
pub fn i_05b906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B908;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B908: move.l d7, (a4)
pub fn i_05b908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B90A: add.l d4, d7
pub fn i_05b90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B90C: move.l d5, (a4)
pub fn i_05b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B90E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B90E: add.l d4, d5
pub fn i_05b90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B910;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B910: move.w -(a6), d7
pub fn i_05b910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B912;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B912: move.l d7, (a4)
pub fn i_05b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B914;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B914: add.l d4, d7
pub fn i_05b914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B916;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B916: move.l d5, (a4)
pub fn i_05b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B918;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B918: add.l d4, d5
pub fn i_05b918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B91A: move.w -(a6), d7
pub fn i_05b91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B91C: move.l d7, (a4)
pub fn i_05b91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B91E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B91E: add.l d4, d7
pub fn i_05b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B920;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B920: move.l d5, (a4)
pub fn i_05b920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B922;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B922: add.l d4, d5
pub fn i_05b922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B924;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B924: move.w -(a6), d7
pub fn i_05b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B926;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B926: move.l d7, (a4)
pub fn i_05b926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B928;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B928: add.l d4, d7
pub fn i_05b928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B92A: move.l d5, (a4)
pub fn i_05b92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B92C: add.l d4, d5
pub fn i_05b92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B92E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B92E: move.w -(a6), d7
pub fn i_05b92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B930;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B930: move.l d7, (a4)
pub fn i_05b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B932;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B932: add.l d4, d7
pub fn i_05b932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B934;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x05A872, i_05a872),
    (0x05A874, i_05a874),
    (0x05A878, i_05a878),
    (0x05A87E, i_05a87e),
    (0x05A884, i_05a884),
    (0x05A88A, i_05a88a),
    (0x05A890, i_05a890),
    (0x05A894, i_05a894),
    (0x05A898, i_05a898),
    (0x05A89C, i_05a89c),
    (0x05A8A0, i_05a8a0),
    (0x05A8A4, i_05a8a4),
    (0x05A8AA, i_05a8aa),
    (0x05A8AE, i_05a8ae),
    (0x05A8B2, i_05a8b2),
    (0x05A8B8, i_05a8b8),
    (0x05A9BA, i_05a9ba),
    (0x05A9C0, i_05a9c0),
    (0x05A9C6, i_05a9c6),
    (0x05A9CA, i_05a9ca),
    (0x05A9CE, i_05a9ce),
    (0x05A9D4, i_05a9d4),
    (0x05A9D6, i_05a9d6),
    (0x05A9DA, i_05a9da),
    (0x05A9DE, i_05a9de),
    (0x05A9E2, i_05a9e2),
    (0x05A9E6, i_05a9e6),
    (0x05A9EA, i_05a9ea),
    (0x05A9EC, i_05a9ec),
    (0x05A9F2, i_05a9f2),
    (0x05A9F4, i_05a9f4),
    (0x05A9F8, i_05a9f8),
    (0x05A9FA, i_05a9fa),
    (0x05A9FE, i_05a9fe),
    (0x05AA02, i_05aa02),
    (0x05AA06, i_05aa06),
    (0x05AA0A, i_05aa0a),
    (0x05AA0E, i_05aa0e),
    (0x05AA10, i_05aa10),
    (0x05AA14, i_05aa14),
    (0x05AA18, i_05aa18),
    (0x05AA1C, i_05aa1c),
    (0x05AA20, i_05aa20),
    (0x05AA24, i_05aa24),
    (0x05AA26, i_05aa26),
    (0x05AA2A, i_05aa2a),
    (0x05AA2E, i_05aa2e),
    (0x05AA30, i_05aa30),
    (0x05AA34, i_05aa34),
    (0x05AA38, i_05aa38),
    (0x05AA3C, i_05aa3c),
    (0x05AA3E, i_05aa3e),
    (0x05AA40, i_05aa40),
    (0x05AA44, i_05aa44),
    (0x05AA48, i_05aa48),
    (0x05AA4A, i_05aa4a),
    (0x05AA4C, i_05aa4c),
    (0x05AA4E, i_05aa4e),
    (0x05AA50, i_05aa50),
    (0x05AA52, i_05aa52),
    (0x05AA56, i_05aa56),
    (0x05AA5A, i_05aa5a),
    (0x05AA5C, i_05aa5c),
    (0x05AA5E, i_05aa5e),
    (0x05AA60, i_05aa60),
    (0x05AA64, i_05aa64),
    (0x05AA66, i_05aa66),
    (0x05AA6A, i_05aa6a),
    (0x05AA6C, i_05aa6c),
    (0x05AA70, i_05aa70),
    (0x05AA72, i_05aa72),
    (0x05AA76, i_05aa76),
    (0x05AA7A, i_05aa7a),
    (0x05AA7E, i_05aa7e),
    (0x05AA82, i_05aa82),
    (0x05AA86, i_05aa86),
    (0x05AA8A, i_05aa8a),
    (0x05AA8E, i_05aa8e),
    (0x05AA92, i_05aa92),
    (0x05AA94, i_05aa94),
    (0x05AA96, i_05aa96),
    (0x05AA9A, i_05aa9a),
    (0x05AA9E, i_05aa9e),
    (0x05AAA2, i_05aaa2),
    (0x05AAA6, i_05aaa6),
    (0x05AAAA, i_05aaaa),
    (0x05AAAE, i_05aaae),
    (0x05AAB2, i_05aab2),
    (0x05AAB6, i_05aab6),
    (0x05AAB8, i_05aab8),
    (0x05AABC, i_05aabc),
    (0x05AABE, i_05aabe),
    (0x05AAC0, i_05aac0),
    (0x05AAC2, i_05aac2),
    (0x05AAC6, i_05aac6),
    (0x05AAC8, i_05aac8),
    (0x05AACA, i_05aaca),
    (0x05AACC, i_05aacc),
    (0x05AACE, i_05aace),
    (0x05AAD0, i_05aad0),
    (0x05AAD2, i_05aad2),
    (0x05AAD4, i_05aad4),
    (0x05AAD6, i_05aad6),
    (0x05AAD8, i_05aad8),
    (0x05AADA, i_05aada),
    (0x05AADC, i_05aadc),
    (0x05AADE, i_05aade),
    (0x05AAE0, i_05aae0),
    (0x05AAE2, i_05aae2),
    (0x05AAE6, i_05aae6),
    (0x05AAE8, i_05aae8),
    (0x05AAEA, i_05aaea),
    (0x05AAEC, i_05aaec),
    (0x05AAEE, i_05aaee),
    (0x05AAF0, i_05aaf0),
    (0x05AAF2, i_05aaf2),
    (0x05AAF6, i_05aaf6),
    (0x05AAFA, i_05aafa),
    (0x05AAFE, i_05aafe),
    (0x05AB02, i_05ab02),
    (0x05AB06, i_05ab06),
    (0x05AB08, i_05ab08),
    (0x05AB0A, i_05ab0a),
    (0x05AB0C, i_05ab0c),
    (0x05AB0E, i_05ab0e),
    (0x05AB10, i_05ab10),
    (0x05AB14, i_05ab14),
    (0x05AB16, i_05ab16),
    (0x05AB18, i_05ab18),
    (0x05AB1C, i_05ab1c),
    (0x05AB1E, i_05ab1e),
    (0x05AB20, i_05ab20),
    (0x05AB22, i_05ab22),
    (0x05AB26, i_05ab26),
    (0x05AB28, i_05ab28),
    (0x05AB2C, i_05ab2c),
    (0x05AB32, i_05ab32),
    (0x05AB34, i_05ab34),
    (0x05AB36, i_05ab36),
    (0x05AB3A, i_05ab3a),
    (0x05AB3C, i_05ab3c),
    (0x05AB3E, i_05ab3e),
    (0x05AB40, i_05ab40),
    (0x05AB44, i_05ab44),
    (0x05AB46, i_05ab46),
    (0x05AB48, i_05ab48),
    (0x05AB4A, i_05ab4a),
    (0x05AB4C, i_05ab4c),
    (0x05AB4E, i_05ab4e),
    (0x05AB50, i_05ab50),
    (0x05AB52, i_05ab52),
    (0x05AB54, i_05ab54),
    (0x05AB56, i_05ab56),
    (0x05AB58, i_05ab58),
    (0x05AB5A, i_05ab5a),
    (0x05AB5C, i_05ab5c),
    (0x05AB5E, i_05ab5e),
    (0x05AB60, i_05ab60),
    (0x05AB62, i_05ab62),
    (0x05AB64, i_05ab64),
    (0x05AB66, i_05ab66),
    (0x05AB6A, i_05ab6a),
    (0x05AB6C, i_05ab6c),
    (0x05AB6E, i_05ab6e),
    (0x05AB70, i_05ab70),
    (0x05AB72, i_05ab72),
    (0x05AB74, i_05ab74),
    (0x05AB76, i_05ab76),
    (0x05AB7A, i_05ab7a),
    (0x05AB7C, i_05ab7c),
    (0x05AB80, i_05ab80),
    (0x05AB84, i_05ab84),
    (0x05AB88, i_05ab88),
    (0x05AB8C, i_05ab8c),
    (0x05AB8E, i_05ab8e),
    (0x05AB90, i_05ab90),
    (0x05AB92, i_05ab92),
    (0x05AB94, i_05ab94),
    (0x05AB96, i_05ab96),
    (0x05AB9A, i_05ab9a),
    (0x05AB9C, i_05ab9c),
    (0x05ABA0, i_05aba0),
    (0x05ABA2, i_05aba2),
    (0x05ABA4, i_05aba4),
    (0x05ABA6, i_05aba6),
    (0x05ABAA, i_05abaa),
    (0x05ABAC, i_05abac),
    (0x05ABB0, i_05abb0),
    (0x05ABB6, i_05abb6),
    (0x05ABB8, i_05abb8),
    (0x05ABBA, i_05abba),
    (0x05ABBE, i_05abbe),
    (0x05ABC0, i_05abc0),
    (0x05ABC2, i_05abc2),
    (0x05ABC4, i_05abc4),
    (0x05ABC8, i_05abc8),
    (0x05ABCA, i_05abca),
    (0x05ABCC, i_05abcc),
    (0x05ABCE, i_05abce),
    (0x05ABD0, i_05abd0),
    (0x05ABD2, i_05abd2),
    (0x05ABD4, i_05abd4),
    (0x05ABD6, i_05abd6),
    (0x05ABD8, i_05abd8),
    (0x05ABDA, i_05abda),
    (0x05ABDC, i_05abdc),
    (0x05ABDE, i_05abde),
    (0x05ABE0, i_05abe0),
    (0x05ABE2, i_05abe2),
    (0x05ABE4, i_05abe4),
    (0x05ABE6, i_05abe6),
    (0x05ABE8, i_05abe8),
    (0x05ABEA, i_05abea),
    (0x05ABEC, i_05abec),
    (0x05ABEE, i_05abee),
    (0x05ABF2, i_05abf2),
    (0x05ABF4, i_05abf4),
    (0x05ABF6, i_05abf6),
    (0x05ABF8, i_05abf8),
    (0x05ABFA, i_05abfa),
    (0x05ABFC, i_05abfc),
    (0x05ABFE, i_05abfe),
    (0x05AC00, i_05ac00),
    (0x05AC04, i_05ac04),
    (0x05AC08, i_05ac08),
    (0x05AC0C, i_05ac0c),
    (0x05AC10, i_05ac10),
    (0x05AC14, i_05ac14),
    (0x05AC16, i_05ac16),
    (0x05AC18, i_05ac18),
    (0x05AC1A, i_05ac1a),
    (0x05AC1C, i_05ac1c),
    (0x05AC1E, i_05ac1e),
    (0x05AC22, i_05ac22),
    (0x05AC24, i_05ac24),
    (0x05AC26, i_05ac26),
    (0x05AC2A, i_05ac2a),
    (0x05AC2C, i_05ac2c),
    (0x05AC2E, i_05ac2e),
    (0x05AC30, i_05ac30),
    (0x05AC32, i_05ac32),
    (0x05AC36, i_05ac36),
    (0x05AC38, i_05ac38),
    (0x05AC3C, i_05ac3c),
    (0x05AC42, i_05ac42),
    (0x05AC44, i_05ac44),
    (0x05AC46, i_05ac46),
    (0x05AC4A, i_05ac4a),
    (0x05AC4C, i_05ac4c),
    (0x05AC4E, i_05ac4e),
    (0x05AC50, i_05ac50),
    (0x05AC54, i_05ac54),
    (0x05AC56, i_05ac56),
    (0x05AC58, i_05ac58),
    (0x05AC5A, i_05ac5a),
    (0x05AC5C, i_05ac5c),
    (0x05AC5E, i_05ac5e),
    (0x05AC60, i_05ac60),
    (0x05AC62, i_05ac62),
    (0x05AC64, i_05ac64),
    (0x05AC66, i_05ac66),
    (0x05AC68, i_05ac68),
    (0x05AC6A, i_05ac6a),
    (0x05AC6C, i_05ac6c),
    (0x05AC6E, i_05ac6e),
    (0x05AC70, i_05ac70),
    (0x05AC72, i_05ac72),
    (0x05AC74, i_05ac74),
    (0x05AC76, i_05ac76),
    (0x05AC78, i_05ac78),
    (0x05AC7A, i_05ac7a),
    (0x05AC7C, i_05ac7c),
    (0x05AC7E, i_05ac7e),
    (0x05AC80, i_05ac80),
    (0x05AC84, i_05ac84),
    (0x05AC86, i_05ac86),
    (0x05AC88, i_05ac88),
    (0x05AC8A, i_05ac8a),
    (0x05AC8C, i_05ac8c),
    (0x05AC8E, i_05ac8e),
    (0x05AC90, i_05ac90),
    (0x05AC92, i_05ac92),
    (0x05AC96, i_05ac96),
    (0x05AC98, i_05ac98),
    (0x05AC9C, i_05ac9c),
    (0x05ACA0, i_05aca0),
    (0x05ACA4, i_05aca4),
    (0x05ACA8, i_05aca8),
    (0x05ACAA, i_05acaa),
    (0x05ACAC, i_05acac),
    (0x05ACAE, i_05acae),
    (0x05ACB0, i_05acb0),
    (0x05ACB2, i_05acb2),
    (0x05ACB6, i_05acb6),
    (0x05ACB8, i_05acb8),
    (0x05ACBC, i_05acbc),
    (0x05ACBE, i_05acbe),
    (0x05ACC0, i_05acc0),
    (0x05ACC2, i_05acc2),
    (0x05ACC4, i_05acc4),
    (0x05ACC8, i_05acc8),
    (0x05ACCA, i_05acca),
    (0x05ACCE, i_05acce),
    (0x05ACD4, i_05acd4),
    (0x05ACD6, i_05acd6),
    (0x05ACD8, i_05acd8),
    (0x05ACDC, i_05acdc),
    (0x05ACDE, i_05acde),
    (0x05ACE0, i_05ace0),
    (0x05ACE2, i_05ace2),
    (0x05ACE6, i_05ace6),
    (0x05ACE8, i_05ace8),
    (0x05ACEA, i_05acea),
    (0x05ACEC, i_05acec),
    (0x05ACEE, i_05acee),
    (0x05ACF2, i_05acf2),
    (0x05ACF4, i_05acf4),
    (0x05ACF6, i_05acf6),
    (0x05ACFA, i_05acfa),
    (0x05ACFE, i_05acfe),
    (0x05AD00, i_05ad00),
    (0x05AD04, i_05ad04),
    (0x05AD06, i_05ad06),
    (0x05AD0A, i_05ad0a),
    (0x05AD0C, i_05ad0c),
    (0x05AD0E, i_05ad0e),
    (0x05AD12, i_05ad12),
    (0x05AD14, i_05ad14),
    (0x05AD16, i_05ad16),
    (0x05AD18, i_05ad18),
    (0x05AD1A, i_05ad1a),
    (0x05AD1C, i_05ad1c),
    (0x05AD1E, i_05ad1e),
    (0x05AD20, i_05ad20),
    (0x05AD22, i_05ad22),
    (0x05AD24, i_05ad24),
    (0x05AD26, i_05ad26),
    (0x05AD28, i_05ad28),
    (0x05AD2A, i_05ad2a),
    (0x05AD2C, i_05ad2c),
    (0x05AD2E, i_05ad2e),
    (0x05AD30, i_05ad30),
    (0x05AD32, i_05ad32),
    (0x05AD34, i_05ad34),
    (0x05AD36, i_05ad36),
    (0x05AD3A, i_05ad3a),
    (0x05AD3C, i_05ad3c),
    (0x05AD3E, i_05ad3e),
    (0x05AD40, i_05ad40),
    (0x05AD42, i_05ad42),
    (0x05AD46, i_05ad46),
    (0x05AD4A, i_05ad4a),
    (0x05AD4C, i_05ad4c),
    (0x05AD4E, i_05ad4e),
    (0x05AD50, i_05ad50),
    (0x05AD52, i_05ad52),
    (0x05AD56, i_05ad56),
    (0x05AD5A, i_05ad5a),
    (0x05AD5C, i_05ad5c),
    (0x05AD60, i_05ad60),
    (0x05AD62, i_05ad62),
    (0x05AD64, i_05ad64),
    (0x05AD66, i_05ad66),
    (0x05AD6A, i_05ad6a),
    (0x05AD6C, i_05ad6c),
    (0x05AD6E, i_05ad6e),
    (0x05AD70, i_05ad70),
    (0x05AD72, i_05ad72),
    (0x05AD74, i_05ad74),
    (0x05AD76, i_05ad76),
    (0x05AD78, i_05ad78),
    (0x05AD7C, i_05ad7c),
    (0x05AD7E, i_05ad7e),
    (0x05AD80, i_05ad80),
    (0x05AD82, i_05ad82),
    (0x05AD84, i_05ad84),
    (0x05AD86, i_05ad86),
    (0x05AD88, i_05ad88),
    (0x05AD8A, i_05ad8a),
    (0x05AD8C, i_05ad8c),
    (0x05AD90, i_05ad90),
    (0x05AD92, i_05ad92),
    (0x05AD94, i_05ad94),
    (0x05AD96, i_05ad96),
    (0x05AD98, i_05ad98),
    (0x05AD9A, i_05ad9a),
    (0x05AD9C, i_05ad9c),
    (0x05AD9E, i_05ad9e),
    (0x05ADA2, i_05ada2),
    (0x05ADA6, i_05ada6),
    (0x05ADAA, i_05adaa),
    (0x05ADAC, i_05adac),
    (0x05ADAE, i_05adae),
    (0x05ADB2, i_05adb2),
    (0x05ADB4, i_05adb4),
    (0x05ADB8, i_05adb8),
    (0x05ADBC, i_05adbc),
    (0x05ADBE, i_05adbe),
    (0x05ADC2, i_05adc2),
    (0x05ADC6, i_05adc6),
    (0x05ADCA, i_05adca),
    (0x05ADCE, i_05adce),
    (0x05ADD0, i_05add0),
    (0x05ADD2, i_05add2),
    (0x05ADD4, i_05add4),
    (0x05ADD6, i_05add6),
    (0x05ADD8, i_05add8),
    (0x05ADDC, i_05addc),
    (0x05ADDE, i_05adde),
    (0x05ADE0, i_05ade0),
    (0x05ADE2, i_05ade2),
    (0x05ADE6, i_05ade6),
    (0x05ADE8, i_05ade8),
    (0x05ADEC, i_05adec),
    (0x05ADEE, i_05adee),
    (0x05ADF2, i_05adf2),
    (0x05ADF4, i_05adf4),
    (0x05ADF8, i_05adf8),
    (0x05ADFE, i_05adfe),
    (0x05AE00, i_05ae00),
    (0x05AE02, i_05ae02),
    (0x05AE06, i_05ae06),
    (0x05AE08, i_05ae08),
    (0x05AE0A, i_05ae0a),
    (0x05AE0C, i_05ae0c),
    (0x05AE10, i_05ae10),
    (0x05AE12, i_05ae12),
    (0x05AE14, i_05ae14),
    (0x05AE16, i_05ae16),
    (0x05AE18, i_05ae18),
    (0x05AE1C, i_05ae1c),
    (0x05AE1E, i_05ae1e),
    (0x05AE20, i_05ae20),
    (0x05AE24, i_05ae24),
    (0x05AE28, i_05ae28),
    (0x05AE2A, i_05ae2a),
    (0x05AE2E, i_05ae2e),
    (0x05AE30, i_05ae30),
    (0x05AE34, i_05ae34),
    (0x05AE36, i_05ae36),
    (0x05AE38, i_05ae38),
    (0x05AE3C, i_05ae3c),
    (0x05AE3E, i_05ae3e),
    (0x05AE40, i_05ae40),
    (0x05AE42, i_05ae42),
    (0x05AE44, i_05ae44),
    (0x05AE46, i_05ae46),
    (0x05AE48, i_05ae48),
    (0x05AE4A, i_05ae4a),
    (0x05AE4C, i_05ae4c),
    (0x05AE4E, i_05ae4e),
    (0x05AE50, i_05ae50),
    (0x05AE52, i_05ae52),
    (0x05AE54, i_05ae54),
    (0x05AE56, i_05ae56),
    (0x05AE58, i_05ae58),
    (0x05AE5A, i_05ae5a),
    (0x05AE5C, i_05ae5c),
    (0x05AE5E, i_05ae5e),
    (0x05AE60, i_05ae60),
    (0x05AE62, i_05ae62),
    (0x05AE64, i_05ae64),
    (0x05AE66, i_05ae66),
    (0x05AE6A, i_05ae6a),
    (0x05AE6C, i_05ae6c),
    (0x05AE6E, i_05ae6e),
    (0x05AE70, i_05ae70),
    (0x05AE72, i_05ae72),
    (0x05AE76, i_05ae76),
    (0x05AE7A, i_05ae7a),
    (0x05AE7C, i_05ae7c),
    (0x05AE7E, i_05ae7e),
    (0x05AE80, i_05ae80),
    (0x05AE82, i_05ae82),
    (0x05AE86, i_05ae86),
    (0x05AE8A, i_05ae8a),
    (0x05AE8C, i_05ae8c),
    (0x05AE90, i_05ae90),
    (0x05AE92, i_05ae92),
    (0x05AE94, i_05ae94),
    (0x05AE96, i_05ae96),
    (0x05AE9A, i_05ae9a),
    (0x05AE9C, i_05ae9c),
    (0x05AE9E, i_05ae9e),
    (0x05AEA0, i_05aea0),
    (0x05AEA2, i_05aea2),
    (0x05AEA4, i_05aea4),
    (0x05AEA6, i_05aea6),
    (0x05AEA8, i_05aea8),
    (0x05AEAC, i_05aeac),
    (0x05AEAE, i_05aeae),
    (0x05AEB0, i_05aeb0),
    (0x05AEB2, i_05aeb2),
    (0x05AEB4, i_05aeb4),
    (0x05AEB6, i_05aeb6),
    (0x05AEB8, i_05aeb8),
    (0x05AEBA, i_05aeba),
    (0x05AEBC, i_05aebc),
    (0x05AEC0, i_05aec0),
    (0x05AEC2, i_05aec2),
    (0x05AEC4, i_05aec4),
    (0x05AEC6, i_05aec6),
    (0x05AEC8, i_05aec8),
    (0x05AECA, i_05aeca),
    (0x05AECC, i_05aecc),
    (0x05AECE, i_05aece),
    (0x05AED2, i_05aed2),
    (0x05AED6, i_05aed6),
    (0x05AEDA, i_05aeda),
    (0x05AEDC, i_05aedc),
    (0x05AEDE, i_05aede),
    (0x05AEE2, i_05aee2),
    (0x05AEE4, i_05aee4),
    (0x05AEE8, i_05aee8),
    (0x05AEEC, i_05aeec),
    (0x05AEEE, i_05aeee),
    (0x05AEF0, i_05aef0),
    (0x05AEF4, i_05aef4),
    (0x05AEF8, i_05aef8),
    (0x05AEFC, i_05aefc),
    (0x05AF00, i_05af00),
    (0x05AF02, i_05af02),
    (0x05AF04, i_05af04),
    (0x05AF06, i_05af06),
    (0x05AF08, i_05af08),
    (0x05AF0A, i_05af0a),
    (0x05AF0E, i_05af0e),
    (0x05AF10, i_05af10),
    (0x05AF12, i_05af12),
    (0x05AF16, i_05af16),
    (0x05AF18, i_05af18),
    (0x05AF1C, i_05af1c),
    (0x05AF1E, i_05af1e),
    (0x05AF22, i_05af22),
    (0x05AF24, i_05af24),
    (0x05AF28, i_05af28),
    (0x05AF2E, i_05af2e),
    (0x05AF30, i_05af30),
    (0x05AF32, i_05af32),
    (0x05AF36, i_05af36),
    (0x05AF38, i_05af38),
    (0x05AF3A, i_05af3a),
    (0x05AF3C, i_05af3c),
    (0x05AF40, i_05af40),
    (0x05AF42, i_05af42),
    (0x05AF44, i_05af44),
    (0x05AF46, i_05af46),
    (0x05AF48, i_05af48),
    (0x05AF4C, i_05af4c),
    (0x05AF4E, i_05af4e),
    (0x05AF50, i_05af50),
    (0x05AF54, i_05af54),
    (0x05AF58, i_05af58),
    (0x05AF5A, i_05af5a),
    (0x05AF5E, i_05af5e),
    (0x05AF60, i_05af60),
    (0x05AF64, i_05af64),
    (0x05AF66, i_05af66),
    (0x05AF68, i_05af68),
    (0x05AF6E, i_05af6e),
    (0x05AF72, i_05af72),
    (0x05AF74, i_05af74),
    (0x05AF76, i_05af76),
    (0x05AF78, i_05af78),
    (0x05AF7A, i_05af7a),
    (0x05AF7C, i_05af7c),
    (0x05AF7E, i_05af7e),
    (0x05AF80, i_05af80),
    (0x05AF82, i_05af82),
    (0x05AF84, i_05af84),
    (0x05AF86, i_05af86),
    (0x05AF88, i_05af88),
    (0x05AF8A, i_05af8a),
    (0x05AF8C, i_05af8c),
    (0x05AF8E, i_05af8e),
    (0x05AF90, i_05af90),
    (0x05AF92, i_05af92),
    (0x05AF94, i_05af94),
    (0x05AF96, i_05af96),
    (0x05AF98, i_05af98),
    (0x05AF9A, i_05af9a),
    (0x05AF9C, i_05af9c),
    (0x05AF9E, i_05af9e),
    (0x05AFA0, i_05afa0),
    (0x05AFA4, i_05afa4),
    (0x05AFA6, i_05afa6),
    (0x05AFA8, i_05afa8),
    (0x05AFAA, i_05afaa),
    (0x05AFAC, i_05afac),
    (0x05AFB0, i_05afb0),
    (0x05AFB4, i_05afb4),
    (0x05AFB6, i_05afb6),
    (0x05AFB8, i_05afb8),
    (0x05AFBA, i_05afba),
    (0x05AFBC, i_05afbc),
    (0x05AFC0, i_05afc0),
    (0x05AFC4, i_05afc4),
    (0x05AFC6, i_05afc6),
    (0x05AFCA, i_05afca),
    (0x05AFCC, i_05afcc),
    (0x05AFCE, i_05afce),
    (0x05AFD0, i_05afd0),
    (0x05AFD4, i_05afd4),
    (0x05AFD6, i_05afd6),
    (0x05AFD8, i_05afd8),
    (0x05AFDA, i_05afda),
    (0x05AFDC, i_05afdc),
    (0x05AFDE, i_05afde),
    (0x05AFE0, i_05afe0),
    (0x05AFE2, i_05afe2),
    (0x05AFE6, i_05afe6),
    (0x05AFE8, i_05afe8),
    (0x05AFEA, i_05afea),
    (0x05AFEC, i_05afec),
    (0x05AFEE, i_05afee),
    (0x05AFF0, i_05aff0),
    (0x05AFF2, i_05aff2),
    (0x05AFF4, i_05aff4),
    (0x05AFF6, i_05aff6),
    (0x05AFFA, i_05affa),
    (0x05AFFC, i_05affc),
    (0x05AFFE, i_05affe),
    (0x05B000, i_05b000),
    (0x05B002, i_05b002),
    (0x05B004, i_05b004),
    (0x05B006, i_05b006),
    (0x05B008, i_05b008),
    (0x05B00A, i_05b00a),
    (0x05B00E, i_05b00e),
    (0x05B012, i_05b012),
    (0x05B016, i_05b016),
    (0x05B018, i_05b018),
    (0x05B01A, i_05b01a),
    (0x05B01E, i_05b01e),
    (0x05B020, i_05b020),
    (0x05B024, i_05b024),
    (0x05B028, i_05b028),
    (0x05B02A, i_05b02a),
    (0x05B02E, i_05b02e),
    (0x05B032, i_05b032),
    (0x05B036, i_05b036),
    (0x05B03A, i_05b03a),
    (0x05B03C, i_05b03c),
    (0x05B03E, i_05b03e),
    (0x05B040, i_05b040),
    (0x05B042, i_05b042),
    (0x05B044, i_05b044),
    (0x05B048, i_05b048),
    (0x05B04A, i_05b04a),
    (0x05B04C, i_05b04c),
    (0x05B04E, i_05b04e),
    (0x05B052, i_05b052),
    (0x05B054, i_05b054),
    (0x05B056, i_05b056),
    (0x05B05A, i_05b05a),
    (0x05B05C, i_05b05c),
    (0x05B060, i_05b060),
    (0x05B062, i_05b062),
    (0x05B066, i_05b066),
    (0x05B06C, i_05b06c),
    (0x05B06E, i_05b06e),
    (0x05B070, i_05b070),
    (0x05B074, i_05b074),
    (0x05B076, i_05b076),
    (0x05B078, i_05b078),
    (0x05B07A, i_05b07a),
    (0x05B07E, i_05b07e),
    (0x05B080, i_05b080),
    (0x05B082, i_05b082),
    (0x05B084, i_05b084),
    (0x05B086, i_05b086),
    (0x05B08A, i_05b08a),
    (0x05B08C, i_05b08c),
    (0x05B08E, i_05b08e),
    (0x05B092, i_05b092),
    (0x05B096, i_05b096),
    (0x05B098, i_05b098),
    (0x05B09C, i_05b09c),
    (0x05B09E, i_05b09e),
    (0x05B0A2, i_05b0a2),
    (0x05B0A4, i_05b0a4),
    (0x05B0A6, i_05b0a6),
    (0x05B0AC, i_05b0ac),
    (0x05B0B0, i_05b0b0),
    (0x05B0B2, i_05b0b2),
    (0x05B0B4, i_05b0b4),
    (0x05B0B6, i_05b0b6),
    (0x05B0B8, i_05b0b8),
    (0x05B0BA, i_05b0ba),
    (0x05B0BC, i_05b0bc),
    (0x05B0BE, i_05b0be),
    (0x05B0C0, i_05b0c0),
    (0x05B0C2, i_05b0c2),
    (0x05B0C4, i_05b0c4),
    (0x05B0C6, i_05b0c6),
    (0x05B0C8, i_05b0c8),
    (0x05B0CA, i_05b0ca),
    (0x05B0CC, i_05b0cc),
    (0x05B0CE, i_05b0ce),
    (0x05B0D0, i_05b0d0),
    (0x05B0D2, i_05b0d2),
    (0x05B0D4, i_05b0d4),
    (0x05B0D6, i_05b0d6),
    (0x05B0D8, i_05b0d8),
    (0x05B0DA, i_05b0da),
    (0x05B0DC, i_05b0dc),
    (0x05B0DE, i_05b0de),
    (0x05B0E0, i_05b0e0),
    (0x05B0E2, i_05b0e2),
    (0x05B0E4, i_05b0e4),
    (0x05B0E8, i_05b0e8),
    (0x05B0EA, i_05b0ea),
    (0x05B0EC, i_05b0ec),
    (0x05B0EE, i_05b0ee),
    (0x05B0F0, i_05b0f0),
    (0x05B0F4, i_05b0f4),
    (0x05B0F8, i_05b0f8),
    (0x05B0FA, i_05b0fa),
    (0x05B0FC, i_05b0fc),
    (0x05B0FE, i_05b0fe),
    (0x05B100, i_05b100),
    (0x05B104, i_05b104),
    (0x05B108, i_05b108),
    (0x05B10A, i_05b10a),
    (0x05B10E, i_05b10e),
    (0x05B110, i_05b110),
    (0x05B112, i_05b112),
    (0x05B114, i_05b114),
    (0x05B118, i_05b118),
    (0x05B11A, i_05b11a),
    (0x05B11C, i_05b11c),
    (0x05B11E, i_05b11e),
    (0x05B120, i_05b120),
    (0x05B122, i_05b122),
    (0x05B124, i_05b124),
    (0x05B126, i_05b126),
    (0x05B12A, i_05b12a),
    (0x05B12C, i_05b12c),
    (0x05B12E, i_05b12e),
    (0x05B130, i_05b130),
    (0x05B132, i_05b132),
    (0x05B134, i_05b134),
    (0x05B136, i_05b136),
    (0x05B138, i_05b138),
    (0x05B13A, i_05b13a),
    (0x05B13E, i_05b13e),
    (0x05B140, i_05b140),
    (0x05B142, i_05b142),
    (0x05B144, i_05b144),
    (0x05B146, i_05b146),
    (0x05B148, i_05b148),
    (0x05B14A, i_05b14a),
    (0x05B14C, i_05b14c),
    (0x05B14E, i_05b14e),
    (0x05B152, i_05b152),
    (0x05B156, i_05b156),
    (0x05B15A, i_05b15a),
    (0x05B15C, i_05b15c),
    (0x05B15E, i_05b15e),
    (0x05B162, i_05b162),
    (0x05B164, i_05b164),
    (0x05B168, i_05b168),
    (0x05B16C, i_05b16c),
    (0x05B16E, i_05b16e),
    (0x05B170, i_05b170),
    (0x05B174, i_05b174),
    (0x05B178, i_05b178),
    (0x05B17C, i_05b17c),
    (0x05B180, i_05b180),
    (0x05B182, i_05b182),
    (0x05B184, i_05b184),
    (0x05B186, i_05b186),
    (0x05B188, i_05b188),
    (0x05B18A, i_05b18a),
    (0x05B18E, i_05b18e),
    (0x05B190, i_05b190),
    (0x05B192, i_05b192),
    (0x05B196, i_05b196),
    (0x05B198, i_05b198),
    (0x05B19A, i_05b19a),
    (0x05B19E, i_05b19e),
    (0x05B1A0, i_05b1a0),
    (0x05B1A4, i_05b1a4),
    (0x05B1A6, i_05b1a6),
    (0x05B1AA, i_05b1aa),
    (0x05B1B0, i_05b1b0),
    (0x05B1B2, i_05b1b2),
    (0x05B1B6, i_05b1b6),
    (0x05B1BE, i_05b1be),
    (0x05B1C2, i_05b1c2),
    (0x05B1C8, i_05b1c8),
    (0x05B1CA, i_05b1ca),
    (0x05B1CE, i_05b1ce),
    (0x05B1D2, i_05b1d2),
    (0x05B1D6, i_05b1d6),
    (0x05B1DA, i_05b1da),
    (0x05B1DE, i_05b1de),
    (0x05B1E4, i_05b1e4),
    (0x05B1E6, i_05b1e6),
    (0x05B1EA, i_05b1ea),
    (0x05B1EE, i_05b1ee),
    (0x05B1F2, i_05b1f2),
    (0x05B1F6, i_05b1f6),
    (0x05B1FA, i_05b1fa),
    (0x05B1FE, i_05b1fe),
    (0x05B202, i_05b202),
    (0x05B206, i_05b206),
    (0x05B20A, i_05b20a),
    (0x05B20C, i_05b20c),
    (0x05B210, i_05b210),
    (0x05B212, i_05b212),
    (0x05B216, i_05b216),
    (0x05B218, i_05b218),
    (0x05B21A, i_05b21a),
    (0x05B21E, i_05b21e),
    (0x05B224, i_05b224),
    (0x05B226, i_05b226),
    (0x05B228, i_05b228),
    (0x05B22A, i_05b22a),
    (0x05B22E, i_05b22e),
    (0x05B230, i_05b230),
    (0x05B232, i_05b232),
    (0x05B238, i_05b238),
    (0x05B23C, i_05b23c),
    (0x05B240, i_05b240),
    (0x05B244, i_05b244),
    (0x05B246, i_05b246),
    (0x05B248, i_05b248),
    (0x05B24C, i_05b24c),
    (0x05B250, i_05b250),
    (0x05B254, i_05b254),
    (0x05B256, i_05b256),
    (0x05B258, i_05b258),
    (0x05B25A, i_05b25a),
    (0x05B25E, i_05b25e),
    (0x05B260, i_05b260),
    (0x05B264, i_05b264),
    (0x05B268, i_05b268),
    (0x05B26A, i_05b26a),
    (0x05B26E, i_05b26e),
    (0x05B270, i_05b270),
    (0x05B272, i_05b272),
    (0x05B274, i_05b274),
    (0x05B278, i_05b278),
    (0x05B27C, i_05b27c),
    (0x05B27E, i_05b27e),
    (0x05B280, i_05b280),
    (0x05B282, i_05b282),
    (0x05B284, i_05b284),
    (0x05B288, i_05b288),
    (0x05B28A, i_05b28a),
    (0x05B28C, i_05b28c),
    (0x05B290, i_05b290),
    (0x05B294, i_05b294),
    (0x05B298, i_05b298),
    (0x05B29C, i_05b29c),
    (0x05B29E, i_05b29e),
    (0x05B2A0, i_05b2a0),
    (0x05B2A4, i_05b2a4),
    (0x05B2A6, i_05b2a6),
    (0x05B2A8, i_05b2a8),
    (0x05B2AA, i_05b2aa),
    (0x05B2AE, i_05b2ae),
    (0x05B2B0, i_05b2b0),
    (0x05B2B4, i_05b2b4),
    (0x05B2B8, i_05b2b8),
    (0x05B2BA, i_05b2ba),
    (0x05B2BE, i_05b2be),
    (0x05B2C0, i_05b2c0),
    (0x05B2C2, i_05b2c2),
    (0x05B2C4, i_05b2c4),
    (0x05B2C8, i_05b2c8),
    (0x05B2CC, i_05b2cc),
    (0x05B2CE, i_05b2ce),
    (0x05B2D0, i_05b2d0),
    (0x05B2D2, i_05b2d2),
    (0x05B2D4, i_05b2d4),
    (0x05B2D8, i_05b2d8),
    (0x05B2DA, i_05b2da),
    (0x05B2DE, i_05b2de),
    (0x05B2E2, i_05b2e2),
    (0x05B2E6, i_05b2e6),
    (0x05B2EA, i_05b2ea),
    (0x05B2EE, i_05b2ee),
    (0x05B2F2, i_05b2f2),
    (0x05B2F4, i_05b2f4),
    (0x05B2F8, i_05b2f8),
    (0x05B2FA, i_05b2fa),
    (0x05B2FC, i_05b2fc),
    (0x05B2FE, i_05b2fe),
    (0x05B300, i_05b300),
    (0x05B304, i_05b304),
    (0x05B308, i_05b308),
    (0x05B30A, i_05b30a),
    (0x05B30C, i_05b30c),
    (0x05B30E, i_05b30e),
    (0x05B310, i_05b310),
    (0x05B312, i_05b312),
    (0x05B316, i_05b316),
    (0x05B318, i_05b318),
    (0x05B31A, i_05b31a),
    (0x05B31C, i_05b31c),
    (0x05B322, i_05b322),
    (0x05B326, i_05b326),
    (0x05B32C, i_05b32c),
    (0x05B334, i_05b334),
    (0x05B338, i_05b338),
    (0x05B33C, i_05b33c),
    (0x05B340, i_05b340),
    (0x05B344, i_05b344),
    (0x05B348, i_05b348),
    (0x05B34E, i_05b34e),
    (0x05B354, i_05b354),
    (0x05B358, i_05b358),
    (0x05B35C, i_05b35c),
    (0x05B360, i_05b360),
    (0x05B362, i_05b362),
    (0x05B366, i_05b366),
    (0x05B368, i_05b368),
    (0x05B36E, i_05b36e),
    (0x05B370, i_05b370),
    (0x05B374, i_05b374),
    (0x05B378, i_05b378),
    (0x05B37C, i_05b37c),
    (0x05B37E, i_05b37e),
    (0x05B380, i_05b380),
    (0x05B382, i_05b382),
    (0x05B386, i_05b386),
    (0x05B388, i_05b388),
    (0x05B38C, i_05b38c),
    (0x05B38E, i_05b38e),
    (0x05B392, i_05b392),
    (0x05B394, i_05b394),
    (0x05B396, i_05b396),
    (0x05B39A, i_05b39a),
    (0x05B39C, i_05b39c),
    (0x05B3A0, i_05b3a0),
    (0x05B3A4, i_05b3a4),
    (0x05B3A6, i_05b3a6),
    (0x05B3AA, i_05b3aa),
    (0x05B3AE, i_05b3ae),
    (0x05B3B2, i_05b3b2),
    (0x05B3B4, i_05b3b4),
    (0x05B3B8, i_05b3b8),
    (0x05B3BA, i_05b3ba),
    (0x05B3BE, i_05b3be),
    (0x05B3C2, i_05b3c2),
    (0x05B3C4, i_05b3c4),
    (0x05B3C6, i_05b3c6),
    (0x05B3C8, i_05b3c8),
    (0x05B3CA, i_05b3ca),
    (0x05B3CE, i_05b3ce),
    (0x05B3D0, i_05b3d0),
    (0x05B3D2, i_05b3d2),
    (0x05B3D6, i_05b3d6),
    (0x05B3D8, i_05b3d8),
    (0x05B3DA, i_05b3da),
    (0x05B3DE, i_05b3de),
    (0x05B3E0, i_05b3e0),
    (0x05B3E4, i_05b3e4),
    (0x05B3E6, i_05b3e6),
    (0x05B3EA, i_05b3ea),
    (0x05B3EE, i_05b3ee),
    (0x05B3F2, i_05b3f2),
    (0x05B3F6, i_05b3f6),
    (0x05B3F8, i_05b3f8),
    (0x05B3FA, i_05b3fa),
    (0x05B3FE, i_05b3fe),
    (0x05B400, i_05b400),
    (0x05B404, i_05b404),
    (0x05B408, i_05b408),
    (0x05B40A, i_05b40a),
    (0x05B40C, i_05b40c),
    (0x05B40E, i_05b40e),
    (0x05B412, i_05b412),
    (0x05B416, i_05b416),
    (0x05B418, i_05b418),
    (0x05B41C, i_05b41c),
    (0x05B41E, i_05b41e),
    (0x05B422, i_05b422),
    (0x05B424, i_05b424),
    (0x05B426, i_05b426),
    (0x05B42A, i_05b42a),
    (0x05B42E, i_05b42e),
    (0x05B432, i_05b432),
    (0x05B436, i_05b436),
    (0x05B438, i_05b438),
    (0x05B43A, i_05b43a),
    (0x05B43E, i_05b43e),
    (0x05B440, i_05b440),
    (0x05B444, i_05b444),
    (0x05B448, i_05b448),
    (0x05B44A, i_05b44a),
    (0x05B44C, i_05b44c),
    (0x05B450, i_05b450),
    (0x05B454, i_05b454),
    (0x05B456, i_05b456),
    (0x05B45A, i_05b45a),
    (0x05B45C, i_05b45c),
    (0x05B45E, i_05b45e),
    (0x05B460, i_05b460),
    (0x05B464, i_05b464),
    (0x05B468, i_05b468),
    (0x05B46C, i_05b46c),
    (0x05B46E, i_05b46e),
    (0x05B470, i_05b470),
    (0x05B474, i_05b474),
    (0x05B476, i_05b476),
    (0x05B478, i_05b478),
    (0x05B47E, i_05b47e),
    (0x05B480, i_05b480),
    (0x05B482, i_05b482),
    (0x05B484, i_05b484),
    (0x05B486, i_05b486),
    (0x05B48A, i_05b48a),
    (0x05B48C, i_05b48c),
    (0x05B48E, i_05b48e),
    (0x05B492, i_05b492),
    (0x05B496, i_05b496),
    (0x05B49A, i_05b49a),
    (0x05B49C, i_05b49c),
    (0x05B49E, i_05b49e),
    (0x05B4A0, i_05b4a0),
    (0x05B4A4, i_05b4a4),
    (0x05B4A6, i_05b4a6),
    (0x05B4AA, i_05b4aa),
    (0x05B4AC, i_05b4ac),
    (0x05B4AE, i_05b4ae),
    (0x05B4B2, i_05b4b2),
    (0x05B4B4, i_05b4b4),
    (0x05B4B6, i_05b4b6),
    (0x05B4BA, i_05b4ba),
    (0x05B4BC, i_05b4bc),
    (0x05B4BE, i_05b4be),
    (0x05B4C2, i_05b4c2),
    (0x05B4C6, i_05b4c6),
    (0x05B4CA, i_05b4ca),
    (0x05B4CC, i_05b4cc),
    (0x05B4CE, i_05b4ce),
    (0x05B4D2, i_05b4d2),
    (0x05B4D4, i_05b4d4),
    (0x05B4D8, i_05b4d8),
    (0x05B4DC, i_05b4dc),
    (0x05B4E0, i_05b4e0),
    (0x05B4E4, i_05b4e4),
    (0x05B4E6, i_05b4e6),
    (0x05B4E8, i_05b4e8),
    (0x05B4EA, i_05b4ea),
    (0x05B4EC, i_05b4ec),
    (0x05B4F0, i_05b4f0),
    (0x05B4F4, i_05b4f4),
    (0x05B4F8, i_05b4f8),
    (0x05B4FA, i_05b4fa),
    (0x05B4FC, i_05b4fc),
    (0x05B500, i_05b500),
    (0x05B502, i_05b502),
    (0x05B504, i_05b504),
    (0x05B50A, i_05b50a),
    (0x05B50C, i_05b50c),
    (0x05B50E, i_05b50e),
    (0x05B510, i_05b510),
    (0x05B512, i_05b512),
    (0x05B516, i_05b516),
    (0x05B518, i_05b518),
    (0x05B51A, i_05b51a),
    (0x05B51E, i_05b51e),
    (0x05B522, i_05b522),
    (0x05B524, i_05b524),
    (0x05B528, i_05b528),
    (0x05B52C, i_05b52c),
    (0x05B52E, i_05b52e),
    (0x05B532, i_05b532),
    (0x05B534, i_05b534),
    (0x05B536, i_05b536),
    (0x05B53A, i_05b53a),
    (0x05B53C, i_05b53c),
    (0x05B540, i_05b540),
    (0x05B542, i_05b542),
    (0x05B546, i_05b546),
    (0x05B54A, i_05b54a),
    (0x05B54E, i_05b54e),
    (0x05B552, i_05b552),
    (0x05B556, i_05b556),
    (0x05B55A, i_05b55a),
    (0x05B55C, i_05b55c),
    (0x05B55E, i_05b55e),
    (0x05B560, i_05b560),
    (0x05B562, i_05b562),
    (0x05B566, i_05b566),
    (0x05B568, i_05b568),
    (0x05B56A, i_05b56a),
    (0x05B56C, i_05b56c),
    (0x05B56E, i_05b56e),
    (0x05B572, i_05b572),
    (0x05B574, i_05b574),
    (0x05B578, i_05b578),
    (0x05B57A, i_05b57a),
    (0x05B57C, i_05b57c),
    (0x05B580, i_05b580),
    (0x05B584, i_05b584),
    (0x05B588, i_05b588),
    (0x05B58C, i_05b58c),
    (0x05B590, i_05b590),
    (0x05B594, i_05b594),
    (0x05B598, i_05b598),
    (0x05B59C, i_05b59c),
    (0x05B5A0, i_05b5a0),
    (0x05B5A4, i_05b5a4),
    (0x05B5A8, i_05b5a8),
    (0x05B5AC, i_05b5ac),
    (0x05B5B0, i_05b5b0),
    (0x05B5B4, i_05b5b4),
    (0x05B5B8, i_05b5b8),
    (0x05B5BC, i_05b5bc),
    (0x05B5C0, i_05b5c0),
    (0x05B5C4, i_05b5c4),
    (0x05B5C8, i_05b5c8),
    (0x05B5CC, i_05b5cc),
    (0x05B5D0, i_05b5d0),
    (0x05B5D4, i_05b5d4),
    (0x05B5D8, i_05b5d8),
    (0x05B5DC, i_05b5dc),
    (0x05B5E0, i_05b5e0),
    (0x05B5E4, i_05b5e4),
    (0x05B5E8, i_05b5e8),
    (0x05B5EC, i_05b5ec),
    (0x05B5F0, i_05b5f0),
    (0x05B5F4, i_05b5f4),
    (0x05B5F8, i_05b5f8),
    (0x05B5FC, i_05b5fc),
    (0x05B5FE, i_05b5fe),
    (0x05B600, i_05b600),
    (0x05B602, i_05b602),
    (0x05B604, i_05b604),
    (0x05B606, i_05b606),
    (0x05B608, i_05b608),
    (0x05B60A, i_05b60a),
    (0x05B60C, i_05b60c),
    (0x05B60E, i_05b60e),
    (0x05B610, i_05b610),
    (0x05B612, i_05b612),
    (0x05B614, i_05b614),
    (0x05B616, i_05b616),
    (0x05B618, i_05b618),
    (0x05B61A, i_05b61a),
    (0x05B61C, i_05b61c),
    (0x05B61E, i_05b61e),
    (0x05B620, i_05b620),
    (0x05B622, i_05b622),
    (0x05B624, i_05b624),
    (0x05B626, i_05b626),
    (0x05B628, i_05b628),
    (0x05B62A, i_05b62a),
    (0x05B62C, i_05b62c),
    (0x05B62E, i_05b62e),
    (0x05B630, i_05b630),
    (0x05B632, i_05b632),
    (0x05B634, i_05b634),
    (0x05B636, i_05b636),
    (0x05B638, i_05b638),
    (0x05B63A, i_05b63a),
    (0x05B63C, i_05b63c),
    (0x05B63E, i_05b63e),
    (0x05B640, i_05b640),
    (0x05B642, i_05b642),
    (0x05B644, i_05b644),
    (0x05B646, i_05b646),
    (0x05B648, i_05b648),
    (0x05B64A, i_05b64a),
    (0x05B64C, i_05b64c),
    (0x05B64E, i_05b64e),
    (0x05B650, i_05b650),
    (0x05B652, i_05b652),
    (0x05B654, i_05b654),
    (0x05B656, i_05b656),
    (0x05B658, i_05b658),
    (0x05B65A, i_05b65a),
    (0x05B65C, i_05b65c),
    (0x05B65E, i_05b65e),
    (0x05B660, i_05b660),
    (0x05B662, i_05b662),
    (0x05B664, i_05b664),
    (0x05B666, i_05b666),
    (0x05B668, i_05b668),
    (0x05B66A, i_05b66a),
    (0x05B66C, i_05b66c),
    (0x05B66E, i_05b66e),
    (0x05B670, i_05b670),
    (0x05B672, i_05b672),
    (0x05B674, i_05b674),
    (0x05B676, i_05b676),
    (0x05B678, i_05b678),
    (0x05B67A, i_05b67a),
    (0x05B67C, i_05b67c),
    (0x05B67E, i_05b67e),
    (0x05B680, i_05b680),
    (0x05B682, i_05b682),
    (0x05B684, i_05b684),
    (0x05B686, i_05b686),
    (0x05B688, i_05b688),
    (0x05B68A, i_05b68a),
    (0x05B68C, i_05b68c),
    (0x05B68E, i_05b68e),
    (0x05B690, i_05b690),
    (0x05B692, i_05b692),
    (0x05B694, i_05b694),
    (0x05B696, i_05b696),
    (0x05B698, i_05b698),
    (0x05B69A, i_05b69a),
    (0x05B69C, i_05b69c),
    (0x05B69E, i_05b69e),
    (0x05B6A0, i_05b6a0),
    (0x05B6A2, i_05b6a2),
    (0x05B6A4, i_05b6a4),
    (0x05B6A6, i_05b6a6),
    (0x05B6A8, i_05b6a8),
    (0x05B6AA, i_05b6aa),
    (0x05B6AC, i_05b6ac),
    (0x05B6AE, i_05b6ae),
    (0x05B6B0, i_05b6b0),
    (0x05B6B2, i_05b6b2),
    (0x05B6B4, i_05b6b4),
    (0x05B6B6, i_05b6b6),
    (0x05B6B8, i_05b6b8),
    (0x05B6BA, i_05b6ba),
    (0x05B6BC, i_05b6bc),
    (0x05B6BE, i_05b6be),
    (0x05B6C0, i_05b6c0),
    (0x05B6C2, i_05b6c2),
    (0x05B6C4, i_05b6c4),
    (0x05B6C6, i_05b6c6),
    (0x05B6C8, i_05b6c8),
    (0x05B6CA, i_05b6ca),
    (0x05B6CC, i_05b6cc),
    (0x05B6CE, i_05b6ce),
    (0x05B6D0, i_05b6d0),
    (0x05B6D2, i_05b6d2),
    (0x05B6D4, i_05b6d4),
    (0x05B6D6, i_05b6d6),
    (0x05B6D8, i_05b6d8),
    (0x05B6DA, i_05b6da),
    (0x05B6DC, i_05b6dc),
    (0x05B6DE, i_05b6de),
    (0x05B6E0, i_05b6e0),
    (0x05B6E2, i_05b6e2),
    (0x05B6E4, i_05b6e4),
    (0x05B6E6, i_05b6e6),
    (0x05B6E8, i_05b6e8),
    (0x05B6EA, i_05b6ea),
    (0x05B6EC, i_05b6ec),
    (0x05B6EE, i_05b6ee),
    (0x05B6F0, i_05b6f0),
    (0x05B6F2, i_05b6f2),
    (0x05B6F4, i_05b6f4),
    (0x05B6F6, i_05b6f6),
    (0x05B6F8, i_05b6f8),
    (0x05B6FA, i_05b6fa),
    (0x05B6FC, i_05b6fc),
    (0x05B6FE, i_05b6fe),
    (0x05B700, i_05b700),
    (0x05B702, i_05b702),
    (0x05B704, i_05b704),
    (0x05B706, i_05b706),
    (0x05B708, i_05b708),
    (0x05B70A, i_05b70a),
    (0x05B70C, i_05b70c),
    (0x05B70E, i_05b70e),
    (0x05B710, i_05b710),
    (0x05B712, i_05b712),
    (0x05B714, i_05b714),
    (0x05B716, i_05b716),
    (0x05B718, i_05b718),
    (0x05B71A, i_05b71a),
    (0x05B71C, i_05b71c),
    (0x05B71E, i_05b71e),
    (0x05B720, i_05b720),
    (0x05B722, i_05b722),
    (0x05B724, i_05b724),
    (0x05B726, i_05b726),
    (0x05B728, i_05b728),
    (0x05B72A, i_05b72a),
    (0x05B72C, i_05b72c),
    (0x05B72E, i_05b72e),
    (0x05B730, i_05b730),
    (0x05B732, i_05b732),
    (0x05B734, i_05b734),
    (0x05B736, i_05b736),
    (0x05B738, i_05b738),
    (0x05B73A, i_05b73a),
    (0x05B73C, i_05b73c),
    (0x05B740, i_05b740),
    (0x05B742, i_05b742),
    (0x05B744, i_05b744),
    (0x05B748, i_05b748),
    (0x05B74E, i_05b74e),
    (0x05B750, i_05b750),
    (0x05B752, i_05b752),
    (0x05B754, i_05b754),
    (0x05B758, i_05b758),
    (0x05B75A, i_05b75a),
    (0x05B75C, i_05b75c),
    (0x05B760, i_05b760),
    (0x05B762, i_05b762),
    (0x05B764, i_05b764),
    (0x05B766, i_05b766),
    (0x05B768, i_05b768),
    (0x05B76C, i_05b76c),
    (0x05B76E, i_05b76e),
    (0x05B770, i_05b770),
    (0x05B772, i_05b772),
    (0x05B774, i_05b774),
    (0x05B778, i_05b778),
    (0x05B77A, i_05b77a),
    (0x05B77E, i_05b77e),
    (0x05B780, i_05b780),
    (0x05B782, i_05b782),
    (0x05B786, i_05b786),
    (0x05B78A, i_05b78a),
    (0x05B78E, i_05b78e),
    (0x05B792, i_05b792),
    (0x05B796, i_05b796),
    (0x05B79A, i_05b79a),
    (0x05B79E, i_05b79e),
    (0x05B7A2, i_05b7a2),
    (0x05B7A6, i_05b7a6),
    (0x05B7AA, i_05b7aa),
    (0x05B7AE, i_05b7ae),
    (0x05B7B2, i_05b7b2),
    (0x05B7B6, i_05b7b6),
    (0x05B7BA, i_05b7ba),
    (0x05B7BE, i_05b7be),
    (0x05B7C2, i_05b7c2),
    (0x05B7C6, i_05b7c6),
    (0x05B7CA, i_05b7ca),
    (0x05B7CE, i_05b7ce),
    (0x05B7D2, i_05b7d2),
    (0x05B7D6, i_05b7d6),
    (0x05B7DA, i_05b7da),
    (0x05B7DE, i_05b7de),
    (0x05B7E2, i_05b7e2),
    (0x05B7E6, i_05b7e6),
    (0x05B7EA, i_05b7ea),
    (0x05B7EE, i_05b7ee),
    (0x05B7F2, i_05b7f2),
    (0x05B7F6, i_05b7f6),
    (0x05B7FA, i_05b7fa),
    (0x05B7FE, i_05b7fe),
    (0x05B802, i_05b802),
    (0x05B804, i_05b804),
    (0x05B806, i_05b806),
    (0x05B808, i_05b808),
    (0x05B80A, i_05b80a),
    (0x05B80C, i_05b80c),
    (0x05B80E, i_05b80e),
    (0x05B810, i_05b810),
    (0x05B812, i_05b812),
    (0x05B814, i_05b814),
    (0x05B816, i_05b816),
    (0x05B818, i_05b818),
    (0x05B81A, i_05b81a),
    (0x05B81C, i_05b81c),
    (0x05B81E, i_05b81e),
    (0x05B820, i_05b820),
    (0x05B822, i_05b822),
    (0x05B824, i_05b824),
    (0x05B826, i_05b826),
    (0x05B828, i_05b828),
    (0x05B82A, i_05b82a),
    (0x05B82C, i_05b82c),
    (0x05B82E, i_05b82e),
    (0x05B830, i_05b830),
    (0x05B832, i_05b832),
    (0x05B834, i_05b834),
    (0x05B836, i_05b836),
    (0x05B838, i_05b838),
    (0x05B83A, i_05b83a),
    (0x05B83C, i_05b83c),
    (0x05B83E, i_05b83e),
    (0x05B840, i_05b840),
    (0x05B842, i_05b842),
    (0x05B844, i_05b844),
    (0x05B846, i_05b846),
    (0x05B848, i_05b848),
    (0x05B84A, i_05b84a),
    (0x05B84C, i_05b84c),
    (0x05B84E, i_05b84e),
    (0x05B850, i_05b850),
    (0x05B852, i_05b852),
    (0x05B854, i_05b854),
    (0x05B856, i_05b856),
    (0x05B858, i_05b858),
    (0x05B85A, i_05b85a),
    (0x05B85C, i_05b85c),
    (0x05B85E, i_05b85e),
    (0x05B860, i_05b860),
    (0x05B862, i_05b862),
    (0x05B864, i_05b864),
    (0x05B866, i_05b866),
    (0x05B868, i_05b868),
    (0x05B86A, i_05b86a),
    (0x05B86C, i_05b86c),
    (0x05B86E, i_05b86e),
    (0x05B870, i_05b870),
    (0x05B872, i_05b872),
    (0x05B874, i_05b874),
    (0x05B876, i_05b876),
    (0x05B878, i_05b878),
    (0x05B87A, i_05b87a),
    (0x05B87C, i_05b87c),
    (0x05B87E, i_05b87e),
    (0x05B880, i_05b880),
    (0x05B882, i_05b882),
    (0x05B884, i_05b884),
    (0x05B886, i_05b886),
    (0x05B888, i_05b888),
    (0x05B88A, i_05b88a),
    (0x05B88C, i_05b88c),
    (0x05B88E, i_05b88e),
    (0x05B890, i_05b890),
    (0x05B892, i_05b892),
    (0x05B894, i_05b894),
    (0x05B896, i_05b896),
    (0x05B898, i_05b898),
    (0x05B89A, i_05b89a),
    (0x05B89C, i_05b89c),
    (0x05B89E, i_05b89e),
    (0x05B8A0, i_05b8a0),
    (0x05B8A2, i_05b8a2),
    (0x05B8A4, i_05b8a4),
    (0x05B8A6, i_05b8a6),
    (0x05B8A8, i_05b8a8),
    (0x05B8AA, i_05b8aa),
    (0x05B8AC, i_05b8ac),
    (0x05B8AE, i_05b8ae),
    (0x05B8B0, i_05b8b0),
    (0x05B8B2, i_05b8b2),
    (0x05B8B4, i_05b8b4),
    (0x05B8B6, i_05b8b6),
    (0x05B8B8, i_05b8b8),
    (0x05B8BA, i_05b8ba),
    (0x05B8BC, i_05b8bc),
    (0x05B8BE, i_05b8be),
    (0x05B8C0, i_05b8c0),
    (0x05B8C2, i_05b8c2),
    (0x05B8C4, i_05b8c4),
    (0x05B8C6, i_05b8c6),
    (0x05B8C8, i_05b8c8),
    (0x05B8CA, i_05b8ca),
    (0x05B8CC, i_05b8cc),
    (0x05B8CE, i_05b8ce),
    (0x05B8D0, i_05b8d0),
    (0x05B8D2, i_05b8d2),
    (0x05B8D4, i_05b8d4),
    (0x05B8D6, i_05b8d6),
    (0x05B8D8, i_05b8d8),
    (0x05B8DA, i_05b8da),
    (0x05B8DC, i_05b8dc),
    (0x05B8DE, i_05b8de),
    (0x05B8E0, i_05b8e0),
    (0x05B8E2, i_05b8e2),
    (0x05B8E4, i_05b8e4),
    (0x05B8E6, i_05b8e6),
    (0x05B8E8, i_05b8e8),
    (0x05B8EA, i_05b8ea),
    (0x05B8EC, i_05b8ec),
    (0x05B8EE, i_05b8ee),
    (0x05B8F0, i_05b8f0),
    (0x05B8F2, i_05b8f2),
    (0x05B8F4, i_05b8f4),
    (0x05B8F6, i_05b8f6),
    (0x05B8F8, i_05b8f8),
    (0x05B8FA, i_05b8fa),
    (0x05B8FC, i_05b8fc),
    (0x05B8FE, i_05b8fe),
    (0x05B900, i_05b900),
    (0x05B902, i_05b902),
    (0x05B904, i_05b904),
    (0x05B906, i_05b906),
    (0x05B908, i_05b908),
    (0x05B90A, i_05b90a),
    (0x05B90C, i_05b90c),
    (0x05B90E, i_05b90e),
    (0x05B910, i_05b910),
    (0x05B912, i_05b912),
    (0x05B914, i_05b914),
    (0x05B916, i_05b916),
    (0x05B918, i_05b918),
    (0x05B91A, i_05b91a),
    (0x05B91C, i_05b91c),
    (0x05B91E, i_05b91e),
    (0x05B920, i_05b920),
    (0x05B922, i_05b922),
    (0x05B924, i_05b924),
    (0x05B926, i_05b926),
    (0x05B928, i_05b928),
    (0x05B92A, i_05b92a),
    (0x05B92C, i_05b92c),
    (0x05B92E, i_05b92e),
    (0x05B930, i_05b930),
    (0x05B932, i_05b932),
];
