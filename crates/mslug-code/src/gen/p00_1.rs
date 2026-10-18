// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p00_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $002964: moveq #$0, d2
pub fn i_002964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2966;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002966: move.b (a2, d3.w), d2
pub fn i_002966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x296A;
    cpu.ir = 0x1432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $00296A: move.l d4, d1
pub fn i_00296a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x296C;
    cpu.ir = 0x2204;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $00296C: addi.w #$94, d2
pub fn i_00296c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2970;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x94, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002970: move.w d2, d4
pub fn i_002970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2972;
    cpu.ir = 0x3802;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002972: lsl.w #$5, d2
pub fn i_002972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2974;
    cpu.ir = 0xEB4A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002974: lea.l $1082c8.l, a1
pub fn i_002974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x297A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $00297A: move.l d1, $2(a1, d2.w)
pub fn i_00297a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x297E;
    cpu.ir = 0x2381;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $00297E: addq.w #$1, $6(a1, d2.w)
pub fn i_00297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2982;
    cpu.ir = 0x5271;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $002982: cmpi.w #$1, $6(a1, d2.w)
pub fn i_002982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2988;
    cpu.ir = 0xC71;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    18
}
/// $002988: bne.w $2992
pub fn i_002988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x298C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2992; return 10; }
    8
}
/// $00298C: move.b #$1, (a1, d2.w)
pub fn i_00298c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2992;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002992: move.w d4, d2
pub fn i_002992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2994;
    cpu.ir = 0x3404;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002994: move.w d2, $14(a6)
pub fn i_002994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2998;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $002998: move.w $1e(a6), d1
pub fn i_002998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $00299C: add.w d1, d1
pub fn i_00299c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299E;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $00299E: move.w d2, $16(a6, d1.w)
pub fn i_00299e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A2;
    cpu.ir = 0x3D82;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $0029A2: addq.w #$1, $1e(a6)
pub fn i_0029a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A6;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0029A6: rts
pub fn i_0029a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeSpriteSlot_29A8  ($0029A8, 74 B, video/sprite_slot_probe_0029a8.s)
/// $0029A8: move.l d1, d5
pub fn i_0029a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29AA;
    cpu.ir = 0x2A01;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0029AA: clr.b d2
pub fn i_0029aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29AC;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0029AC: andi.l #$ffff, d1
pub fn i_0029ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29B2;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $0029B2: lsl.l #$6, d1
pub fn i_0029b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29B4;
    cpu.ir = 0xED89;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[1] = r;
    18
}
/// $0029B4: lea.l $14e00.l, a1
pub fn i_0029b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x14E00;
    20
}
/// $0029BA: add.l a1, d1
pub fn i_0029ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29BC;
    cpu.ir = 0xD289;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[1] = r;
    4
}
/// $0029BC: move.w $1e(a6), d3
pub fn i_0029bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C0;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x1E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0029C0: tst.w d3
pub fn i_0029c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C2;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0029C2: beq.w $29ee
pub fn i_0029c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29EE; return 10; }
    8
}
/// $0029C6: subq.w #$1, d3
pub fn i_0029c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29C8;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0029C8: move.w d3, d4
pub fn i_0029c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CA;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0029CA: add.w d4, d4
pub fn i_0029ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29CC;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0029CC: move.w $16(a6, d4.w), d0
pub fn i_0029cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29D0;
    cpu.ir = 0x3036;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0029D0: lsl.w #$5, d0
pub fn i_0029d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29D2;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0029D2: lea.l $1082c8.l, a1
pub fn i_0029d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29D8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $0029D8: cmp.l $2(a1, d0.w), d1
pub fn i_0029d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29DC;
    cpu.ir = 0xB2B1;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    18
}
/// $0029DC: bne.b $29c0
pub fn i_0029dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29DE;
    cpu.ir = 0x66E2;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29C0; return 10; }
    8
}
/// $0029DE: btst.b #$2, (a6)
pub fn i_0029de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29E2;
    cpu.ir = 0x816;
    let mk: u32 = 1u32 << ((2) % 8);
    let v2 = bus.read8(cpu.a[6]) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    12
}
/// $0029E2: bne.b $29c0
pub fn i_0029e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29E4;
    cpu.ir = 0x66DC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29C0; return 10; }
    8
}
/// $0029E4: move.w $16(a6, d4.w), $14(a6)
pub fn i_0029e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29EA;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x16).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0029EA: move.b #$ff, d2
pub fn i_0029ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29EE;
    cpu.ir = 0x143C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0029EE: move.l d5, d1
pub fn i_0029ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29F0;
    cpu.ir = 0x2205;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0029F0: rts
pub fn i_0029f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002AE8: trap #$f
pub fn i_002ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $002AEC: move.w (a0), d1
pub fn i_002aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AEE;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $002AEE: cmpi.w #$ffff, d1
pub fn i_002aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $002AF2: beq.w $2b48
pub fn i_002af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B48; return 10; }
    8
}
/// $002AF6: andi.l #$ff, d1
pub fn i_002af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2AFC;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $002AFC: cmpi.w #$20, d1
pub fn i_002afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B00;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $002B00: bgt.b $2ae8
pub fn i_002b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B02;
    cpu.ir = 0x6EE6;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2AE8; return 10; }
    8
}
/// $002B02: cmpi.w #$0, d1
pub fn i_002b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B06;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $002B06: blt.b $2ae8
pub fn i_002b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B08;
    cpu.ir = 0x6DE0;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2AE8; return 10; }
    8
}
/// $002B08: addi.w #$74, d1
pub fn i_002b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x74, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002B0C: lsl.w #$5, d1
pub fn i_002b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B0E;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002B0E: lea.l $1082c8.l, a1
pub fn i_002b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B14;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002B14: move.w $2(a0), d2
pub fn i_002b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B18;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $002B18: cmpi.w #$ffff, d2
pub fn i_002b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B1C;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $002B1C: beq.w $2b4a
pub fn i_002b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B20;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B4A; return 10; }
    8
}
/// $002B20: andi.l #$ffff, d2
pub fn i_002b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B26;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $002B26: lsl.l #$6, d2
pub fn i_002b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B28;
    cpu.ir = 0xED8A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[2] = r;
    18
}
/// $002B28: lea.l $14e00.l, a2
pub fn i_002b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B2E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x14E00;
    20
}
/// $002B2E: add.l a2, d2
pub fn i_002b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B30;
    cpu.ir = 0xD48A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $002B30: move.l d2, $2(a1, d1.w)
pub fn i_002b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B34;
    cpu.ir = 0x2382;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $002B34: move.w #$1, $6(a1, d1.w)
pub fn i_002b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3A;
    cpu.ir = 0x33BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002B3A: move.b (a0), (a1, d1.w)
pub fn i_002b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B3E;
    cpu.ir = 0x1390;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad3, v2 as u8);
    18
}
/// $002B3E: ori.b #$1, (a1, d1.w)
pub fn i_002b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B44;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002B44: addq.l #$4, a0
pub fn i_002b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B46;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $002B46: bra.b $2aec
pub fn i_002b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B48;
    cpu.ir = 0x60A4;
    cpu.pc = 0x2AEC;
    10
}
/// $002B48: rts
pub fn i_002b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002B4A: clr.w $6(a1, d1.w)
pub fn i_002b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B4E;
    cpu.ir = 0x4271;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $002B4E: move.b #$40, (a1, d1.w)
pub fn i_002b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B54;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002B54: addq.l #$4, a0
pub fn i_002b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B56;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $002B56: bra.b $2aec
pub fn i_002b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B58;
    cpu.ir = 0x6094;
    cpu.pc = 0x2AEC;
    10
}
// ===== PalSlot_LoadListHi_002b58  ($002B58, 108 B, boot/boot_script_sound_palslots_000400.s)
/// $002B58: move.w (a0), d1
pub fn i_002b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5A;
    cpu.ir = 0x3210;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $002B5A: cmpi.w #$ffff, d1
pub fn i_002b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B5E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $002B5E: beq.w $2bb4
pub fn i_002b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B62;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BB4; return 10; }
    8
}
/// $002B62: andi.l #$ff, d1
pub fn i_002b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B68;
    cpu.ir = 0x281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[1] = r2;
    8
}
/// $002B68: cmpi.w #$ff, d1
pub fn i_002b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B6C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Word, false);
    8
}
/// $002B6C: beq.w $2b78
pub fn i_002b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B70;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2B78; return 10; }
    8
}
/// $002B70: cmpi.w #$74, d1
pub fn i_002b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B74;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74, Size::Word, false);
    8
}
/// $002B74: bgt.w $2ae8
pub fn i_002b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B78;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2AE8; return 10; }
    8
}
/// $002B78: lsl.w #$5, d1
pub fn i_002b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B7A;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002B7A: lea.l $1082c8.l, a1
pub fn i_002b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B80;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002B80: move.w $2(a0), d2
pub fn i_002b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B84;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $002B84: cmpi.w #$ffff, d2
pub fn i_002b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B88;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $002B88: beq.w $2bb6
pub fn i_002b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2BB6; return 10; }
    8
}
/// $002B8C: andi.l #$ffff, d2
pub fn i_002b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B92;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $002B92: lsl.l #$6, d2
pub fn i_002b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B94;
    cpu.ir = 0xED8A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[2] = r;
    18
}
/// $002B94: lea.l $1ce00.l, a2
pub fn i_002b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x1CE00;
    20
}
/// $002B9A: add.l a2, d2
pub fn i_002b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2B9C;
    cpu.ir = 0xD48A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $002B9C: move.l d2, $2(a1, d1.w)
pub fn i_002b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA0;
    cpu.ir = 0x2382;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $002BA0: move.w #$1, $6(a1, d1.w)
pub fn i_002ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BA6;
    cpu.ir = 0x33BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002BA6: move.b (a0), (a1, d1.w)
pub fn i_002ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BAA;
    cpu.ir = 0x1390;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad3, v2 as u8);
    18
}
/// $002BAA: ori.b #$1, (a1, d1.w)
pub fn i_002baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB0;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002BB0: addq.l #$4, a0
pub fn i_002bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB2;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $002BB2: bra.b $2b58
pub fn i_002bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB4;
    cpu.ir = 0x60A4;
    cpu.pc = 0x2B58;
    10
}
/// $002BB4: rts
pub fn i_002bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002BB6: clr.w $6(a1, d1.w)
pub fn i_002bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BBA;
    cpu.ir = 0x4271;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $002BBA: move.b #$40, (a1, d1.w)
pub fn i_002bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC0;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002BC0: addq.l #$4, a0
pub fn i_002bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC2;
    cpu.ir = 0x5888;
    cpu.a[0] = cpu.a[0].wrapping_add(4);
    8
}
/// $002BC2: bra.b $2b58
pub fn i_002bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC4;
    cpu.ir = 0x6094;
    cpu.pc = 0x2B58;
    10
}
// ===== PalSlot_Release_002bc4  ($002BC4, 98 B, boot/boot_script_sound_palslots_000400.s)
/// $002BC4: movem.l d1-d3/a1-a2, -(a7)
pub fn i_002bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BC8;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $002BC8: move.w d1, d2
pub fn i_002bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCA;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002BCA: lsl.w #$5, d1
pub fn i_002bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BCC;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002BCC: lea.l $1082c8.l, a1
pub fn i_002bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002BD2: subq.w #$1, $6(a1, d1.w)
pub fn i_002bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BD6;
    cpu.ir = 0x5371;
    let ad1 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $002BD6: bne.w $2c20
pub fn i_002bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BDA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2C20; return 10; }
    8
}
/// $002BDA: move.b #$80, (a1, d1.w)
pub fn i_002bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE0;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002BE0: ori.b #$8, (a1, d1.w)
pub fn i_002be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BE6;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002BE6: clr.l $2(a1, d1.w)
pub fn i_002be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEA;
    cpu.ir = 0x42B1;
    let ad1 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    18
}
/// $002BEA: clr.w $8(a1, d1.w)
pub fn i_002bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BEE;
    cpu.ir = 0x4271;
    let ad1 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    14
}
/// $002BEE: move.w $1082c6.l, d1
pub fn i_002bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BF4;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x1082C6) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $002BF4: lea.l $1081c2.l, a1
pub fn i_002bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1081C2;
    20
}
/// $002BFA: subi.w #$94, d2
pub fn i_002bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2BFE;
    cpu.ir = 0x442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x94, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002BFE: move.b d2, (a1, d1.w)
pub fn i_002bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C02;
    cpu.ir = 0x1382;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002C02: addq.w #$1, $1082c6.l
pub fn i_002c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C08;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x1082C6) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x1082C6, r2 as u16);
    16
}
/// $002C08: cmpi.w #$30, $1082c6.l
pub fn i_002c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C10;
    cpu.ir = 0xC79;
    let v2 = bus.read16(0x1082C6) as u32;
    sub_with_flags(cpu, v2, 0x30, Size::Word, false);
    20
}
/// $002C10: blt.w $2c1a
pub fn i_002c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C14;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2C1A; return 10; }
    8
}
/// $002C14: clr.w $1082c6.l
pub fn i_002c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C1A;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x1082C6) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x1082C6, r2 as u16);
    16
}
/// $002C1A: addq.w #$1, $1082c2.l
pub fn i_002c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C20;
    cpu.ir = 0x5279;
    let v1 = bus.read16(0x1082C2) as u32;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    bus.write16(0x1082C2, r2 as u16);
    16
}
/// $002C20: movem.l (a7)+, d1-d3/a1-a2
pub fn i_002c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C24;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $002C24: rts
pub fn i_002c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_SetupSlotFromTableA  ($002C26, 64 B, video/sprite_setup_slot_from_table_002c26.s)
/// $002C26: lea.l $1ce00.l, a2
pub fn i_002c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C2C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x1CE00;
    20
}
/// $002C2C: bra.w $2c36
pub fn i_002c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C30;
    cpu.ir = 0x6000;
    cpu.pc = 0x2C36;
    10
}
/// $002C30: lea.l $14e00.l, a2
pub fn i_002c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C36;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x14E00;
    20
}
/// $002C36: andi.l #$ffff, d2
pub fn i_002c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3C;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $002C3C: lsl.l #$6, d2
pub fn i_002c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C3E;
    cpu.ir = 0xED8A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Long, 1, true);
    cpu.d[2] = r;
    18
}
/// $002C3E: add.l a2, d2
pub fn i_002c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C40;
    cpu.ir = 0xD48A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $002C40: lsl.w #$5, d1
pub fn i_002c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C42;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002C42: lea.l $1082c8.l, a1
pub fn i_002c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C48;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002C48: move.l d2, $a(a1, d1.w)
pub fn i_002c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C4C;
    cpu.ir = 0x2382;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0xA).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $002C4C: move.w d3, $8(a1, d1.w)
pub fn i_002c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C50;
    cpu.ir = 0x3383;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x8).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002C50: move.w d3, $10(a1, d1.w)
pub fn i_002c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C54;
    cpu.ir = 0x3383;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x10).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad2, v1 as u16);
    14
}
/// $002C54: move.b #$0, $e(a1, d1.w)
pub fn i_002c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5A;
    cpu.ir = 0x13BC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xE).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002C5A: move.b d4, $f(a1, d1.w)
pub fn i_002c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C5E;
    cpu.ir = 0x1384;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xF).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $002C5E: ori.b #$2, (a1, d1.w)
pub fn i_002c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C64;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002C64: rts
pub fn i_002c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalSlot_SetBit3_002c66  ($002C66, 16 B, boot/boot_script_sound_palslots_000400.s)
/// $002C66: lea.l $1082c8.l, a1
pub fn i_002c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002C6C: lsl.w #$5, d1
pub fn i_002c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C6E;
    cpu.ir = 0xEB49;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $002C6E: ori.b #$8, (a1, d1.w)
pub fn i_002c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C74;
    cpu.ir = 0x31;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002C74: rts
pub fn i_002c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PalSlot_UpdateAll_002c86  ($002C86, 682 B, boot/boot_script_sound_palslots_000400.s)
/// $002C86: cmpi.b #$ff, $10a2d2.l
pub fn i_002c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C8E;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10A2D2) as u32;
    sub_with_flags(cpu, v2, 0xFF, Size::Byte, false);
    20
}
/// $002C8E: beq.w $2c9e
pub fn i_002c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2C9E; return 10; }
    8
}
/// $002C92: lea.l $1387e.l, a0
pub fn i_002c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1387E;
    20
}
/// $002C98: jsr $1f4a.l
pub fn i_002c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2C9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2C9E);
    cpu.pc = 0x1F4A;
    16
}
/// $002C9E: moveq #$0, d4
pub fn i_002c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA0;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002CA0: clr.b $10a2cd.l
pub fn i_002ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CA6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10A2CD) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10A2CD, r2 as u8);
    16
}
/// $002CA6: clr.b $10a2ce.l
pub fn i_002ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CAC;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10A2CE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10A2CE, r2 as u8);
    16
}
/// $002CAC: move.b $10a2c8.l, d0
pub fn i_002cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB2;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2C8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $002CB2: bne.w $2d88
pub fn i_002cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CB6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D88; return 10; }
    8
}
/// $002CB6: move.b $10a2c9.l, d0
pub fn i_002cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CBC;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x10A2C9) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $002CBC: beq.w $2ce0
pub fn i_002cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CE0; return 10; }
    8
}
/// $002CC0: cmpi.b #$80, d0
pub fn i_002cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $002CC4: beq.w $2ce0
pub fn i_002cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CC8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2CE0; return 10; }
    8
}
/// $002CC8: moveq #$0, d1
pub fn i_002cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CCA;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002CCA: move.b d1, $10a2ca.l
pub fn i_002cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD0;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CA, v1 as u8);
    16
}
/// $002CD0: move.b d1, $10a2cb.l
pub fn i_002cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CD6;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CB, v1 as u8);
    16
}
/// $002CD6: move.b d1, $10a2cc.l
pub fn i_002cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CDC;
    cpu.ir = 0x13C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CC, v1 as u8);
    16
}
/// $002CDC: bra.w $2d88
pub fn i_002cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D88;
    10
}
/// $002CE0: lea.l $1082c8.l, a1
pub fn i_002ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002CE6: moveq #$0, d1
pub fn i_002ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CE8;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002CE8: move.w #$c3, d0
pub fn i_002ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002CEC: btst.b #$7, (a1, d1.w)
pub fn i_002cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF2;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002CF2: bne.w $2d52
pub fn i_002cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D52; return 10; }
    8
}
/// $002CF6: btst.b #$1, (a1, d1.w)
pub fn i_002cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2CFC;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002CFC: beq.w $2d0e
pub fn i_002cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D0E; return 10; }
    8
}
/// $002D00: jsr $13408.l
pub fn i_002d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D06);
    cpu.pc = 0x13408;
    16
}
/// $002D06: move.b #$ff, $10a2ce.l
pub fn i_002d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D0E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002D0E: btst.b #$0, (a1, d1.w)
pub fn i_002d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D14;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002D14: beq.w $2d32
pub fn i_002d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D18;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D32; return 10; }
    8
}
/// $002D18: movea.l $2(a1, d1.w), a2
pub fn i_002d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D1C;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $002D1C: lea.l $10a2d4.l, a3
pub fn i_002d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D22;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10A2D4;
    20
}
/// $002D22: adda.l d1, a3
pub fn i_002d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D24;
    cpu.ir = 0xD7C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $002D24: jsr $133b0.l
pub fn i_002d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D2A);
    cpu.pc = 0x133B0;
    16
}
/// $002D2A: move.b #$ff, $10a2ce.l
pub fn i_002d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D32;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002D32: btst.b #$6, (a1, d1.w)
pub fn i_002d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D38;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002D38: beq.w $2d52
pub fn i_002d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2D52; return 10; }
    8
}
/// $002D3C: lea.l $10a2d4.l, a3
pub fn i_002d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D42;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10A2D4;
    20
}
/// $002D42: adda.l d1, a3
pub fn i_002d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D44;
    cpu.ir = 0xD7C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $002D44: jsr $133e6.l
pub fn i_002d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2D4A);
    cpu.pc = 0x133E6;
    16
}
/// $002D4A: move.b #$ff, $10a2ce.l
pub fn i_002d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D52;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002D52: addi.w #$20, d1
pub fn i_002d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D56;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002D56: dbra d0, $2cec
pub fn i_002d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D5A;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2D5A; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2D5A; return 14; }
    cpu.pc = 0x2CEC;
    10
}
/// $002D5A: tst.b $10a2cd.l
pub fn i_002d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D60;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002D60: bne.w $2d7c
pub fn i_002d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D7C; return 10; }
    8
}
/// $002D64: move.b #$ff, $10a2cd.l
pub fn i_002d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D6C;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CD, v1 as u8);
    16
}
/// $002D6C: lea.l $1082c8.l, a1
pub fn i_002d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D72;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002D72: move.w #$1fe0, d1
pub fn i_002d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D76;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002D76: clr.w d0
pub fn i_002d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D78;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002D78: bra.w $2cec
pub fn i_002d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D7C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2CEC;
    10
}
/// $002D7C: move.b $10a2c8.l, $10a2c9.l
pub fn i_002d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D86;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10A2C8) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2C9, v2 as u8);
    28
}
/// $002D86: rts
pub fn i_002d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $002D88: cmpi.b #$ff, d0
pub fn i_002d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D8C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $002D8C: beq.w $2e62
pub fn i_002d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E62; return 10; }
    8
}
/// $002D90: lea.l $1082c8.l, a1
pub fn i_002d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002D96: moveq #$0, d1
pub fn i_002d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D98;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002D98: move.w #$c3, d0
pub fn i_002d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2D9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002D9C: btst.b #$7, (a1, d1.w)
pub fn i_002d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA2;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DA2: bne.w $2e30
pub fn i_002da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2E30; return 10; }
    8
}
/// $002DA6: btst.b #$1, (a1, d1.w)
pub fn i_002da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DAC;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DAC: beq.w $2db6
pub fn i_002dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DB6; return 10; }
    8
}
/// $002DB0: jsr $13408.l
pub fn i_002db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DB6);
    cpu.pc = 0x13408;
    16
}
/// $002DB6: btst.b #$1, (a1, d1.w)
pub fn i_002db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DBC;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DBC: bne.w $2e30
pub fn i_002dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2E30; return 10; }
    8
}
/// $002DC0: movea.l $2(a1, d1.w), a2
pub fn i_002dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DC4;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $002DC4: lea.l $10a2d4.l, a3
pub fn i_002dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCA;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10A2D4;
    20
}
/// $002DCA: adda.l d1, a3
pub fn i_002dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DCC;
    cpu.ir = 0xD7C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $002DCC: btst.b #$6, (a1, d1.w)
pub fn i_002dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD2;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DD2: beq.w $2de0
pub fn i_002dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DE0; return 10; }
    8
}
/// $002DD6: jsr $133e6.l
pub fn i_002dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DDC);
    cpu.pc = 0x133E6;
    16
}
/// $002DDC: bra.w $2e30
pub fn i_002ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E30;
    10
}
/// $002DE0: btst.b #$3, (a1, d1.w)
pub fn i_002de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE6;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DE6: beq.w $2dfe
pub fn i_002de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DFE; return 10; }
    8
}
/// $002DEA: btst.b #$0, (a1, d1.w)
pub fn i_002dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF0;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002DF0: beq.w $2e30
pub fn i_002df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E30; return 10; }
    8
}
/// $002DF4: jsr $133b0.l
pub fn i_002df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DFA);
    cpu.pc = 0x133B0;
    16
}
/// $002DFA: bra.w $2e30
pub fn i_002dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E30;
    10
}
/// $002DFE: movea.l a3, a4
pub fn i_002dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E00;
    cpu.ir = 0x284B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $002E00: addq.w #$2, a3
pub fn i_002e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E02;
    cpu.ir = 0x544B;
    cpu.a[3] = cpu.a[3].wrapping_add(2);
    8
}
/// $002E02: addq.l #$4, a2
pub fn i_002e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E04;
    cpu.ir = 0x588A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $002E04: move.w #$e, d5
pub fn i_002e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E08;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002E08: move.b (a2)+, d2
pub fn i_002e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0A;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002E0A: move.b (a2)+, d3
pub fn i_002e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0C;
    cpu.ir = 0x161A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002E0C: move.b (a2)+, d4
pub fn i_002e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E0E;
    cpu.ir = 0x181A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002E0E: addq.l #$1, a2
pub fn i_002e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E10;
    cpu.ir = 0x528A;
    cpu.a[2] = cpu.a[2].wrapping_add(1);
    8
}
/// $002E10: jsr $13624.l
pub fn i_002e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E16);
    cpu.pc = 0x13624;
    16
}
/// $002E16: lea.l $2f30(pc), a0
pub fn i_002e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2F30;
    16
}
/// $002E1A: add.w d4, d4
pub fn i_002e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1C;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $002E1C: adda.l d4, a0
pub fn i_002e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E1E;
    cpu.ir = 0xD1C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $002E1E: move.w (a0), d4
pub fn i_002e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E20;
    cpu.ir = 0x3810;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $002E20: move.w d4, (a3)+
pub fn i_002e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E22;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $002E22: dbra d5, $2e08
pub fn i_002e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E26;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2E26; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2E26; return 14; }
    cpu.pc = 0x2E08;
    10
}
/// $002E26: andi.b #$fe, (a1, d1.w)
pub fn i_002e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E2C;
    cpu.ir = 0x231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002E2C: move.w #$1, (a4)
pub fn i_002e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E30;
    cpu.ir = 0x38BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $002E30: addi.w #$20, d1
pub fn i_002e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E34;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002E34: dbra d0, $2d9c
pub fn i_002e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E38;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2E38; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2E38; return 14; }
    cpu.pc = 0x2D9C;
    10
}
/// $002E38: move.b #$ff, $10a2ce.l
pub fn i_002e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E40;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002E40: tst.b $10a2cd.l
pub fn i_002e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E46;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002E46: bne.w $2d7c
pub fn i_002e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E4A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D7C; return 10; }
    8
}
/// $002E4A: move.b #$ff, $10a2cd.l
pub fn i_002e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E52;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CD, v1 as u8);
    16
}
/// $002E52: lea.l $1082c8.l, a1
pub fn i_002e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E58;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002E58: move.w #$1fe0, d1
pub fn i_002e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E5C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002E5C: clr.w d0
pub fn i_002e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E5E;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002E5E: bra.w $2d9c
pub fn i_002e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E62;
    cpu.ir = 0x6000;
    cpu.pc = 0x2D9C;
    10
}
/// $002E62: lea.l $1082c8.l, a1
pub fn i_002e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002E68: moveq #$0, d1
pub fn i_002e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6A;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $002E6A: move.w #$c3, d0
pub fn i_002e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E6E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002E6E: btst.b #$7, (a1, d1.w)
pub fn i_002e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E74;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002E74: bne.w $2efe
pub fn i_002e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2EFE; return 10; }
    8
}
/// $002E78: btst.b #$1, (a1, d1.w)
pub fn i_002e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E7E;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002E7E: beq.w $2e88
pub fn i_002e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E82;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E88; return 10; }
    8
}
/// $002E82: jsr $13408.l
pub fn i_002e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E88);
    cpu.pc = 0x13408;
    16
}
/// $002E88: btst.b #$1, (a1, d1.w)
pub fn i_002e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E8E;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002E8E: bne.w $2efe
pub fn i_002e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2EFE; return 10; }
    8
}
/// $002E92: movea.l $2(a1, d1.w), a2
pub fn i_002e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E96;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $002E96: lea.l $10a2d4.l, a3
pub fn i_002e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9C;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10A2D4;
    20
}
/// $002E9C: adda.l d1, a3
pub fn i_002e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9E;
    cpu.ir = 0xD7C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $002E9E: btst.b #$6, (a1, d1.w)
pub fn i_002e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA4;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002EA4: beq.w $2eb2
pub fn i_002ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EB2; return 10; }
    8
}
/// $002EA8: jsr $133e6.l
pub fn i_002ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EAE);
    cpu.pc = 0x133E6;
    16
}
/// $002EAE: bra.w $2efe
pub fn i_002eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2EFE;
    10
}
/// $002EB2: btst.b #$3, (a1, d1.w)
pub fn i_002eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB8;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002EB8: beq.w $2ed0
pub fn i_002eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EBC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2ED0; return 10; }
    8
}
/// $002EBC: btst.b #$0, (a1, d1.w)
pub fn i_002ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC2;
    cpu.ir = 0x831;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    18
}
/// $002EC2: beq.w $2efe
pub fn i_002ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EC6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2EFE; return 10; }
    8
}
/// $002EC6: jsr $133b0.l
pub fn i_002ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2ECC);
    cpu.pc = 0x133B0;
    16
}
/// $002ECC: bra.w $2efe
pub fn i_002ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED0;
    cpu.ir = 0x6000;
    cpu.pc = 0x2EFE;
    10
}
/// $002ED0: movea.l a3, a4
pub fn i_002ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED2;
    cpu.ir = 0x284B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $002ED2: addq.w #$2, a3
pub fn i_002ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED4;
    cpu.ir = 0x544B;
    cpu.a[3] = cpu.a[3].wrapping_add(2);
    8
}
/// $002ED4: addq.l #$4, a2
pub fn i_002ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2ED6;
    cpu.ir = 0x588A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $002ED6: move.w #$e, d5
pub fn i_002ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDA;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002EDA: move.b (a2)+, d2
pub fn i_002eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDC;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002EDC: move.b (a2)+, d3
pub fn i_002edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EDE;
    cpu.ir = 0x161A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002EDE: move.b (a2)+, d4
pub fn i_002ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE0;
    cpu.ir = 0x181A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    8
}
/// $002EE0: addq.l #$1, a2
pub fn i_002ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE2;
    cpu.ir = 0x528A;
    cpu.a[2] = cpu.a[2].wrapping_add(1);
    8
}
/// $002EE2: jsr $13694.l
pub fn i_002ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EE8);
    cpu.pc = 0x13694;
    16
}
/// $002EE8: add.w d4, d4
pub fn i_002ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEA;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $002EEA: move.w $2f30(pc, d4.l), d4
pub fn i_002eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EEE;
    cpu.ir = 0x383B;
    let ad2 = 0x2F30_u32.wrapping_add(cpu.d[4]);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $002EEE: move.w d4, (a3)+
pub fn i_002eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF0;
    cpu.ir = 0x36C4;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $002EF0: dbra d5, $2eda
pub fn i_002ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EF4;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2EF4; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2EF4; return 14; }
    cpu.pc = 0x2EDA;
    10
}
/// $002EF4: andi.b #$fe, (a1, d1.w)
pub fn i_002ef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFA;
    cpu.ir = 0x231;
    let ad1 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $002EFA: move.w #$1, (a4)
pub fn i_002efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EFE;
    cpu.ir = 0x38BC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[4], v1 as u16);
    8
}
/// $002EFE: addi.w #$20, d1
pub fn i_002efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F02;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $002F02: dbra d0, $2e6e
pub fn i_002f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F06;
    cpu.ir = 0x51C8;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x2F06; return 12; }
    let n = (cpu.d[0] as u16).wrapping_sub(1);
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x2F06; return 14; }
    cpu.pc = 0x2E6E;
    10
}
/// $002F06: move.b #$ff, $10a2ce.l
pub fn i_002f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F0E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CE, v1 as u8);
    16
}
/// $002F0E: tst.b $10a2cd.l
pub fn i_002f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F14;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10A2CD) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $002F14: bne.w $2d7c
pub fn i_002f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F18;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2D7C; return 10; }
    8
}
/// $002F18: move.b #$ff, $10a2cd.l
pub fn i_002f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F20;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2CD, v1 as u8);
    16
}
/// $002F20: lea.l $1082c8.l, a1
pub fn i_002f20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F26;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1082C8;
    20
}
/// $002F26: move.w #$1fe0, d1
pub fn i_002f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1FE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $002F2A: clr.w d0
pub fn i_002f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F2C;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $002F2C: bra.w $2e6e
pub fn i_002f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F30;
    cpu.ir = 0x6000;
    cpu.pc = 0x2E6E;
    10
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x002964, i_002964),
    (0x002966, i_002966),
    (0x00296A, i_00296a),
    (0x00296C, i_00296c),
    (0x002970, i_002970),
    (0x002972, i_002972),
    (0x002974, i_002974),
    (0x00297A, i_00297a),
    (0x00297E, i_00297e),
    (0x002982, i_002982),
    (0x002988, i_002988),
    (0x00298C, i_00298c),
    (0x002992, i_002992),
    (0x002994, i_002994),
    (0x002998, i_002998),
    (0x00299C, i_00299c),
    (0x00299E, i_00299e),
    (0x0029A2, i_0029a2),
    (0x0029A6, i_0029a6),
    (0x0029A8, i_0029a8),
    (0x0029AA, i_0029aa),
    (0x0029AC, i_0029ac),
    (0x0029B2, i_0029b2),
    (0x0029B4, i_0029b4),
    (0x0029BA, i_0029ba),
    (0x0029BC, i_0029bc),
    (0x0029C0, i_0029c0),
    (0x0029C2, i_0029c2),
    (0x0029C6, i_0029c6),
    (0x0029C8, i_0029c8),
    (0x0029CA, i_0029ca),
    (0x0029CC, i_0029cc),
    (0x0029D0, i_0029d0),
    (0x0029D2, i_0029d2),
    (0x0029D8, i_0029d8),
    (0x0029DC, i_0029dc),
    (0x0029DE, i_0029de),
    (0x0029E2, i_0029e2),
    (0x0029E4, i_0029e4),
    (0x0029EA, i_0029ea),
    (0x0029EE, i_0029ee),
    (0x0029F0, i_0029f0),
    (0x002AE8, i_002ae8),
    (0x002AEC, i_002aec),
    (0x002AEE, i_002aee),
    (0x002AF2, i_002af2),
    (0x002AF6, i_002af6),
    (0x002AFC, i_002afc),
    (0x002B00, i_002b00),
    (0x002B02, i_002b02),
    (0x002B06, i_002b06),
    (0x002B08, i_002b08),
    (0x002B0C, i_002b0c),
    (0x002B0E, i_002b0e),
    (0x002B14, i_002b14),
    (0x002B18, i_002b18),
    (0x002B1C, i_002b1c),
    (0x002B20, i_002b20),
    (0x002B26, i_002b26),
    (0x002B28, i_002b28),
    (0x002B2E, i_002b2e),
    (0x002B30, i_002b30),
    (0x002B34, i_002b34),
    (0x002B3A, i_002b3a),
    (0x002B3E, i_002b3e),
    (0x002B44, i_002b44),
    (0x002B46, i_002b46),
    (0x002B48, i_002b48),
    (0x002B4A, i_002b4a),
    (0x002B4E, i_002b4e),
    (0x002B54, i_002b54),
    (0x002B56, i_002b56),
    (0x002B58, i_002b58),
    (0x002B5A, i_002b5a),
    (0x002B5E, i_002b5e),
    (0x002B62, i_002b62),
    (0x002B68, i_002b68),
    (0x002B6C, i_002b6c),
    (0x002B70, i_002b70),
    (0x002B74, i_002b74),
    (0x002B78, i_002b78),
    (0x002B7A, i_002b7a),
    (0x002B80, i_002b80),
    (0x002B84, i_002b84),
    (0x002B88, i_002b88),
    (0x002B8C, i_002b8c),
    (0x002B92, i_002b92),
    (0x002B94, i_002b94),
    (0x002B9A, i_002b9a),
    (0x002B9C, i_002b9c),
    (0x002BA0, i_002ba0),
    (0x002BA6, i_002ba6),
    (0x002BAA, i_002baa),
    (0x002BB0, i_002bb0),
    (0x002BB2, i_002bb2),
    (0x002BB4, i_002bb4),
    (0x002BB6, i_002bb6),
    (0x002BBA, i_002bba),
    (0x002BC0, i_002bc0),
    (0x002BC2, i_002bc2),
    (0x002BC4, i_002bc4),
    (0x002BC8, i_002bc8),
    (0x002BCA, i_002bca),
    (0x002BCC, i_002bcc),
    (0x002BD2, i_002bd2),
    (0x002BD6, i_002bd6),
    (0x002BDA, i_002bda),
    (0x002BE0, i_002be0),
    (0x002BE6, i_002be6),
    (0x002BEA, i_002bea),
    (0x002BEE, i_002bee),
    (0x002BF4, i_002bf4),
    (0x002BFA, i_002bfa),
    (0x002BFE, i_002bfe),
    (0x002C02, i_002c02),
    (0x002C08, i_002c08),
    (0x002C10, i_002c10),
    (0x002C14, i_002c14),
    (0x002C1A, i_002c1a),
    (0x002C20, i_002c20),
    (0x002C24, i_002c24),
    (0x002C26, i_002c26),
    (0x002C2C, i_002c2c),
    (0x002C30, i_002c30),
    (0x002C36, i_002c36),
    (0x002C3C, i_002c3c),
    (0x002C3E, i_002c3e),
    (0x002C40, i_002c40),
    (0x002C42, i_002c42),
    (0x002C48, i_002c48),
    (0x002C4C, i_002c4c),
    (0x002C50, i_002c50),
    (0x002C54, i_002c54),
    (0x002C5A, i_002c5a),
    (0x002C5E, i_002c5e),
    (0x002C64, i_002c64),
    (0x002C66, i_002c66),
    (0x002C6C, i_002c6c),
    (0x002C6E, i_002c6e),
    (0x002C74, i_002c74),
    (0x002C86, i_002c86),
    (0x002C8E, i_002c8e),
    (0x002C92, i_002c92),
    (0x002C98, i_002c98),
    (0x002C9E, i_002c9e),
    (0x002CA0, i_002ca0),
    (0x002CA6, i_002ca6),
    (0x002CAC, i_002cac),
    (0x002CB2, i_002cb2),
    (0x002CB6, i_002cb6),
    (0x002CBC, i_002cbc),
    (0x002CC0, i_002cc0),
    (0x002CC4, i_002cc4),
    (0x002CC8, i_002cc8),
    (0x002CCA, i_002cca),
    (0x002CD0, i_002cd0),
    (0x002CD6, i_002cd6),
    (0x002CDC, i_002cdc),
    (0x002CE0, i_002ce0),
    (0x002CE6, i_002ce6),
    (0x002CE8, i_002ce8),
    (0x002CEC, i_002cec),
    (0x002CF2, i_002cf2),
    (0x002CF6, i_002cf6),
    (0x002CFC, i_002cfc),
    (0x002D00, i_002d00),
    (0x002D06, i_002d06),
    (0x002D0E, i_002d0e),
    (0x002D14, i_002d14),
    (0x002D18, i_002d18),
    (0x002D1C, i_002d1c),
    (0x002D22, i_002d22),
    (0x002D24, i_002d24),
    (0x002D2A, i_002d2a),
    (0x002D32, i_002d32),
    (0x002D38, i_002d38),
    (0x002D3C, i_002d3c),
    (0x002D42, i_002d42),
    (0x002D44, i_002d44),
    (0x002D4A, i_002d4a),
    (0x002D52, i_002d52),
    (0x002D56, i_002d56),
    (0x002D5A, i_002d5a),
    (0x002D60, i_002d60),
    (0x002D64, i_002d64),
    (0x002D6C, i_002d6c),
    (0x002D72, i_002d72),
    (0x002D76, i_002d76),
    (0x002D78, i_002d78),
    (0x002D7C, i_002d7c),
    (0x002D86, i_002d86),
    (0x002D88, i_002d88),
    (0x002D8C, i_002d8c),
    (0x002D90, i_002d90),
    (0x002D96, i_002d96),
    (0x002D98, i_002d98),
    (0x002D9C, i_002d9c),
    (0x002DA2, i_002da2),
    (0x002DA6, i_002da6),
    (0x002DAC, i_002dac),
    (0x002DB0, i_002db0),
    (0x002DB6, i_002db6),
    (0x002DBC, i_002dbc),
    (0x002DC0, i_002dc0),
    (0x002DC4, i_002dc4),
    (0x002DCA, i_002dca),
    (0x002DCC, i_002dcc),
    (0x002DD2, i_002dd2),
    (0x002DD6, i_002dd6),
    (0x002DDC, i_002ddc),
    (0x002DE0, i_002de0),
    (0x002DE6, i_002de6),
    (0x002DEA, i_002dea),
    (0x002DF0, i_002df0),
    (0x002DF4, i_002df4),
    (0x002DFA, i_002dfa),
    (0x002DFE, i_002dfe),
    (0x002E00, i_002e00),
    (0x002E02, i_002e02),
    (0x002E04, i_002e04),
    (0x002E08, i_002e08),
    (0x002E0A, i_002e0a),
    (0x002E0C, i_002e0c),
    (0x002E0E, i_002e0e),
    (0x002E10, i_002e10),
    (0x002E16, i_002e16),
    (0x002E1A, i_002e1a),
    (0x002E1C, i_002e1c),
    (0x002E1E, i_002e1e),
    (0x002E20, i_002e20),
    (0x002E22, i_002e22),
    (0x002E26, i_002e26),
    (0x002E2C, i_002e2c),
    (0x002E30, i_002e30),
    (0x002E34, i_002e34),
    (0x002E38, i_002e38),
    (0x002E40, i_002e40),
    (0x002E46, i_002e46),
    (0x002E4A, i_002e4a),
    (0x002E52, i_002e52),
    (0x002E58, i_002e58),
    (0x002E5C, i_002e5c),
    (0x002E5E, i_002e5e),
    (0x002E62, i_002e62),
    (0x002E68, i_002e68),
    (0x002E6A, i_002e6a),
    (0x002E6E, i_002e6e),
    (0x002E74, i_002e74),
    (0x002E78, i_002e78),
    (0x002E7E, i_002e7e),
    (0x002E82, i_002e82),
    (0x002E88, i_002e88),
    (0x002E8E, i_002e8e),
    (0x002E92, i_002e92),
    (0x002E96, i_002e96),
    (0x002E9C, i_002e9c),
    (0x002E9E, i_002e9e),
    (0x002EA4, i_002ea4),
    (0x002EA8, i_002ea8),
    (0x002EAE, i_002eae),
    (0x002EB2, i_002eb2),
    (0x002EB8, i_002eb8),
    (0x002EBC, i_002ebc),
    (0x002EC2, i_002ec2),
    (0x002EC6, i_002ec6),
    (0x002ECC, i_002ecc),
    (0x002ED0, i_002ed0),
    (0x002ED2, i_002ed2),
    (0x002ED4, i_002ed4),
    (0x002ED6, i_002ed6),
    (0x002EDA, i_002eda),
    (0x002EDC, i_002edc),
    (0x002EDE, i_002ede),
    (0x002EE0, i_002ee0),
    (0x002EE2, i_002ee2),
    (0x002EE8, i_002ee8),
    (0x002EEA, i_002eea),
    (0x002EEE, i_002eee),
    (0x002EF0, i_002ef0),
    (0x002EF4, i_002ef4),
    (0x002EFA, i_002efa),
    (0x002EFE, i_002efe),
    (0x002F02, i_002f02),
    (0x002F06, i_002f06),
    (0x002F0E, i_002f0e),
    (0x002F14, i_002f14),
    (0x002F18, i_002f18),
    (0x002F20, i_002f20),
    (0x002F26, i_002f26),
    (0x002F2A, i_002f2a),
    (0x002F2C, i_002f2c),
];
