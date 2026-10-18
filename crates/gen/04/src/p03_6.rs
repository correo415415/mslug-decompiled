// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $03CD1E: move.w $24(a2), $24(a0)
pub fn i_03cd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD24;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03CD24: move.w $38(a2), $38(a0)
pub fn i_03cd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD2A;
    cpu.ir = 0x316A;
    let ad2 = cpu.a[2].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03CD2A: jsr $517fe.l
pub fn i_03cd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3CD30);
    cpu.pc = 0x517FE;
    16
}
/// $03CD30: addi.w #$14, $24(a0)
pub fn i_03cd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD36;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CD36: rts
pub fn i_03cd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03CD38: movem.l (a7)+, a0
pub fn i_03cd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD3C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $03CD3C: rts
pub fn i_03cd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_FwdNeg_03cd3e  ($03CD3E, 50 B, player/fire_shells_03c62a.s)
/// $03CD3E: jsr $3ccd2(pc)
pub fn i_03cd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CD42);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CD42: btst.b #$0, $3a(a6)
pub fn i_03cd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD48;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CD48: bne.w $3cd54
pub fn i_03cd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD4C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CD54; return 10; }
    8
}
/// $03CD4C: move.b #$0, d1
pub fn i_03cd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD50;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CD50: bra.w $3cd58
pub fn i_03cd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD54;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CD58;
    10
}
/// $03CD54: move.b #$2, d1
pub fn i_03cd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD58;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CD58: move.b #$0, d0
pub fn i_03cd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD5C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CD5C: lsl.b #$3, d0
pub fn i_03cd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD5E;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CD5E: move.b d0, $98(a0)
pub fn i_03cd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD62;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CD62: move.b $7a(a6), $9a(a0)
pub fn i_03cd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD68;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CD68: move.b #$fe, $9b(a0)
pub fn i_03cd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD6E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CD6E: rts
pub fn i_03cd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Fwd0_03cd70  ($03CD70, 50 B, player/fire_shells_03c62a.s)
/// $03CD70: jsr $3ccd2(pc)
pub fn i_03cd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CD74);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CD74: btst.b #$0, $3a(a6)
pub fn i_03cd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD7A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CD7A: bne.w $3cd86
pub fn i_03cd7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD7E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CD86; return 10; }
    8
}
/// $03CD7E: move.b #$0, d1
pub fn i_03cd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD82;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CD82: bra.w $3cd8a
pub fn i_03cd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD86;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CD8A;
    10
}
/// $03CD86: move.b #$2, d1
pub fn i_03cd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD8A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CD8A: move.b #$0, d0
pub fn i_03cd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD8E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CD8E: lsl.b #$3, d0
pub fn i_03cd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD90;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CD90: move.b d0, $98(a0)
pub fn i_03cd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD94;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CD94: move.b $7a(a6), $9a(a0)
pub fn i_03cd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CD9A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CD9A: move.b #$0, $9b(a0)
pub fn i_03cd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDA0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CDA0: rts
pub fn i_03cda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_FwdPos_03cda2  ($03CDA2, 50 B, player/fire_shells_03c62a.s)
/// $03CDA2: jsr $3ccd2(pc)
pub fn i_03cda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CDA6);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CDA6: btst.b #$0, $3a(a6)
pub fn i_03cda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDAC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CDAC: bne.w $3cdb8
pub fn i_03cdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDB0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CDB8; return 10; }
    8
}
/// $03CDB0: move.b #$0, d1
pub fn i_03cdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDB4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CDB4: bra.w $3cdbc
pub fn i_03cdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CDBC;
    10
}
/// $03CDB8: move.b #$2, d1
pub fn i_03cdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDBC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CDBC: move.b #$0, d0
pub fn i_03cdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDC0;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CDC0: lsl.b #$3, d0
pub fn i_03cdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDC2;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CDC2: move.b d0, $98(a0)
pub fn i_03cdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDC6;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CDC6: move.b $7a(a6), $9a(a0)
pub fn i_03cdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDCC;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CDCC: move.b #$2, $9b(a0)
pub fn i_03cdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDD2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CDD2: rts
pub fn i_03cdd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_FwdNegLow_03cdd4  ($03CDD4, 26 B, player/fire_shells_03c62a.s)
/// $03CDD4: jsr $3cd3e(pc)
pub fn i_03cdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CDD8);
    cpu.pc = 0x3CD3E;
    16
}
/// $03CDD8: btst.b #$0, $3a(a6)
pub fn i_03cdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDDE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CDDE: bne.w $3cde6
pub fn i_03cdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDE2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CDE6; return 10; }
    8
}
/// $03CDE2: bra.w $3cde6
pub fn i_03cde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CDE6;
    10
}
/// $03CDE6: subi.w #$c, $24(a0)
pub fn i_03cde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDEC;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CDEC: rts
pub fn i_03cdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Fwd0Low_03cdee  ($03CDEE, 26 B, player/fire_shells_03c62a.s)
/// $03CDEE: jsr $3cd70(pc)
pub fn i_03cdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CDF2);
    cpu.pc = 0x3CD70;
    16
}
/// $03CDF2: btst.b #$0, $3a(a6)
pub fn i_03cdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDF8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CDF8: bne.w $3ce00
pub fn i_03cdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CDFC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CE00; return 10; }
    8
}
/// $03CDFC: bra.w $3ce00
pub fn i_03cdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE00;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CE00;
    10
}
/// $03CE00: subi.w #$c, $24(a0)
pub fn i_03ce00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE06;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CE06: rts
pub fn i_03ce06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_FwdPosLow_03ce08  ($03CE08, 26 B, player/fire_shells_03c62a.s)
/// $03CE08: jsr $3cda2(pc)
pub fn i_03ce08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CE0C);
    cpu.pc = 0x3CDA2;
    16
}
/// $03CE0C: btst.b #$0, $3a(a6)
pub fn i_03ce0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE12;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CE12: bne.w $3ce1a
pub fn i_03ce12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CE1A; return 10; }
    8
}
/// $03CE16: bra.w $3ce1a
pub fn i_03ce16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CE1A;
    10
}
/// $03CE1A: subi.w #$c, $24(a0)
pub fn i_03ce1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE20;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CE20: rts
pub fn i_03ce20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_UpNeg_03ce22  ($03CE22, 50 B, player/fire_shells_03c62a.s)
/// $03CE22: jsr $3ccd2(pc)
pub fn i_03ce22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CE26);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CE26: btst.b #$0, $3a(a6)
pub fn i_03ce26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE2C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CE2C: bne.w $3ce38
pub fn i_03ce2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CE38; return 10; }
    8
}
/// $03CE30: move.b #$0, d1
pub fn i_03ce30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE34;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CE34: bra.w $3ce3c
pub fn i_03ce34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE38;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CE3C;
    10
}
/// $03CE38: move.b #$2, d1
pub fn i_03ce38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE3C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CE3C: move.b #$8, d0
pub fn i_03ce3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE40;
    cpu.ir = 0x103C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CE40: lsl.b #$3, d0
pub fn i_03ce40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE42;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CE42: move.b d0, $98(a0)
pub fn i_03ce42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE46;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CE46: move.b $7a(a6), $9a(a0)
pub fn i_03ce46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE4C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CE4C: move.b #$fe, $9b(a0)
pub fn i_03ce4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE52;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CE52: rts
pub fn i_03ce52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Up0_03ce54  ($03CE54, 50 B, player/fire_shells_03c62a.s)
/// $03CE54: jsr $3ccd2(pc)
pub fn i_03ce54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CE58);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CE58: btst.b #$0, $3a(a6)
pub fn i_03ce58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE5E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CE5E: bne.w $3ce6a
pub fn i_03ce5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CE6A; return 10; }
    8
}
/// $03CE62: move.b #$0, d1
pub fn i_03ce62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE66;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CE66: bra.w $3ce6e
pub fn i_03ce66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CE6E;
    10
}
/// $03CE6A: move.b #$2, d1
pub fn i_03ce6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE6E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CE6E: move.b #$8, d0
pub fn i_03ce6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE72;
    cpu.ir = 0x103C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CE72: lsl.b #$3, d0
pub fn i_03ce72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE74;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CE74: move.b d0, $98(a0)
pub fn i_03ce74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE78;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CE78: move.b $7a(a6), $9a(a0)
pub fn i_03ce78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE7E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CE7E: move.b #$0, $9b(a0)
pub fn i_03ce7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE84;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CE84: rts
pub fn i_03ce84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE86;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_UpPos_03ce86  ($03CE86, 50 B, player/fire_shells_03c62a.s)
/// $03CE86: jsr $3ccd2(pc)
pub fn i_03ce86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CE8A);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CE8A: btst.b #$0, $3a(a6)
pub fn i_03ce8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE90;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CE90: bne.w $3ce9c
pub fn i_03ce90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CE9C; return 10; }
    8
}
/// $03CE94: move.b #$0, d1
pub fn i_03ce94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE98;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CE98: bra.w $3cea0
pub fn i_03ce98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CE9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CEA0;
    10
}
/// $03CE9C: move.b #$2, d1
pub fn i_03ce9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEA0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CEA0: move.b #$8, d0
pub fn i_03cea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEA4;
    cpu.ir = 0x103C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CEA4: lsl.b #$3, d0
pub fn i_03cea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEA6;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CEA6: move.b d0, $98(a0)
pub fn i_03cea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEAA;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CEAA: move.b $7a(a6), $9a(a0)
pub fn i_03ceaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEB0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CEB0: move.b #$2, $9b(a0)
pub fn i_03ceb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEB6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CEB6: rts
pub fn i_03ceb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_BackNeg_03ceb8  ($03CEB8, 50 B, player/fire_shells_03c62a.s)
/// $03CEB8: jsr $3ccd2(pc)
pub fn i_03ceb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CEBC);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CEBC: btst.b #$0, $3a(a6)
pub fn i_03cebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEC2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CEC2: bne.w $3cece
pub fn i_03cec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CECE; return 10; }
    8
}
/// $03CEC6: move.b #$0, d1
pub fn i_03cec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CECA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CECA: bra.w $3ced2
pub fn i_03ceca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CECE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CED2;
    10
}
/// $03CECE: move.b #$2, d1
pub fn i_03cece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CED2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CED2: move.b #$10, d0
pub fn i_03ced2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CED6;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CED6: lsl.b #$3, d0
pub fn i_03ced6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CED8;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CED8: move.b d0, $98(a0)
pub fn i_03ced8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEDC;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CEDC: move.b $7a(a6), $9a(a0)
pub fn i_03cedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEE2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CEE2: move.b #$fe, $9b(a0)
pub fn i_03cee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEE8;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CEE8: rts
pub fn i_03cee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Back0_03ceea  ($03CEEA, 58 B, player/fire_shells_03c62a.s)
/// $03CEEA: jsr $3ccd2(pc)
pub fn i_03ceea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CEEE);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CEEE: btst.b #$0, $3a(a6)
pub fn i_03ceee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEF4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CEF4: bne.w $3cf04
pub fn i_03cef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CF04; return 10; }
    8
}
/// $03CEF8: move.b #$10, d0
pub fn i_03cef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CEFC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CEFC: move.b #$0, d1
pub fn i_03cefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF00;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF00: bra.w $3cf0c
pub fn i_03cf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF04;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CF0C;
    10
}
/// $03CF04: move.b #$10, d0
pub fn i_03cf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF08;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF08: move.b #$2, d1
pub fn i_03cf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF0C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF0C: move.b #$10, d0
pub fn i_03cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF10;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF10: lsl.b #$3, d0
pub fn i_03cf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF12;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CF12: move.b d0, $98(a0)
pub fn i_03cf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF16;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CF16: move.b $7a(a6), $9a(a0)
pub fn i_03cf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF1C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CF1C: move.b #$0, $9b(a0)
pub fn i_03cf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF22;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CF22: rts
pub fn i_03cf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_BackPos_03cf24  ($03CF24, 50 B, player/fire_shells_03c62a.s)
/// $03CF24: jsr $3ccd2(pc)
pub fn i_03cf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CF28);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CF28: btst.b #$0, $3a(a6)
pub fn i_03cf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF2E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CF2E: bne.w $3cf3a
pub fn i_03cf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF32;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CF3A; return 10; }
    8
}
/// $03CF32: move.b #$0, d1
pub fn i_03cf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF36;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF36: bra.w $3cf3e
pub fn i_03cf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CF3E;
    10
}
/// $03CF3A: move.b #$2, d1
pub fn i_03cf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF3E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF3E: move.b #$10, d0
pub fn i_03cf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF42;
    cpu.ir = 0x103C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF42: lsl.b #$3, d0
pub fn i_03cf42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF44;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CF44: move.b d0, $98(a0)
pub fn i_03cf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF48;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CF48: move.b $7a(a6), $9a(a0)
pub fn i_03cf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF4E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CF4E: move.b #$2, $9b(a0)
pub fn i_03cf4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF54;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CF54: rts
pub fn i_03cf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Ang17Neg_03cf56  ($03CF56, 62 B, player/fire_shells_03c62a.s)
/// $03CF56: jsr $3ccd2(pc)
pub fn i_03cf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CF5A);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CF5A: btst.b #$0, $3a(a6)
pub fn i_03cf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF60;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CF60: bne.w $3cf72
pub fn i_03cf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF64;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CF72; return 10; }
    8
}
/// $03CF64: subi.w #$10, $22(a0)
pub fn i_03cf64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF6A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CF6A: move.b #$0, d1
pub fn i_03cf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF6E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF6E: bra.w $3cf7c
pub fn i_03cf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF72;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CF7C;
    10
}
/// $03CF72: addi.w #$10, $22(a0)
pub fn i_03cf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF78;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CF78: move.b #$2, d1
pub fn i_03cf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF7C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF7C: move.b #$17, d0
pub fn i_03cf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF80;
    cpu.ir = 0x103C;
    let v1: u32 = 0x17;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CF80: lsl.b #$3, d0
pub fn i_03cf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF82;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CF82: move.b d0, $98(a0)
pub fn i_03cf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF86;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CF86: move.b $7a(a6), $9a(a0)
pub fn i_03cf86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF8C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CF8C: move.b #$fe, $9b(a0)
pub fn i_03cf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF92;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CF92: rts
pub fn i_03cf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Ang17_0_03cf94  ($03CF94, 62 B, player/fire_shells_03c62a.s)
/// $03CF94: jsr $3ccd2(pc)
pub fn i_03cf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CF98);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CF98: btst.b #$0, $3a(a6)
pub fn i_03cf98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CF9E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CF9E: bne.w $3cfb0
pub fn i_03cf9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFA2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CFB0; return 10; }
    8
}
/// $03CFA2: subi.w #$10, $22(a0)
pub fn i_03cfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFA8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CFA8: move.b #$0, d1
pub fn i_03cfa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFAC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CFAC: bra.w $3cfba
pub fn i_03cfac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFB0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CFBA;
    10
}
/// $03CFB0: addi.w #$10, $22(a0)
pub fn i_03cfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFB6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CFB6: move.b #$2, d1
pub fn i_03cfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFBA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CFBA: move.b #$17, d0
pub fn i_03cfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFBE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x17;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CFBE: lsl.b #$3, d0
pub fn i_03cfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFC0;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CFC0: move.b d0, $98(a0)
pub fn i_03cfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFC4;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CFC4: move.b $7a(a6), $9a(a0)
pub fn i_03cfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFCA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03CFCA: move.b #$0, $9b(a0)
pub fn i_03cfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFD0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03CFD0: rts
pub fn i_03cfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Ang17Pos_03cfd2  ($03CFD2, 62 B, player/fire_shells_03c62a.s)
/// $03CFD2: jsr $3ccd2(pc)
pub fn i_03cfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3CFD6);
    cpu.pc = 0x3CCD2;
    16
}
/// $03CFD6: btst.b #$0, $3a(a6)
pub fn i_03cfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFDC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03CFDC: bne.w $3cfee
pub fn i_03cfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3CFEE; return 10; }
    8
}
/// $03CFE0: subi.w #$10, $22(a0)
pub fn i_03cfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFE6;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CFE6: move.b #$0, d1
pub fn i_03cfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFEA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CFEA: bra.w $3cff8
pub fn i_03cfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3CFF8;
    10
}
/// $03CFEE: addi.w #$10, $22(a0)
pub fn i_03cfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFF4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03CFF4: move.b #$2, d1
pub fn i_03cff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFF8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CFF8: move.b #$17, d0
pub fn i_03cff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFFC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x17;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03CFFC: lsl.b #$3, d0
pub fn i_03cffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3CFFE;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03CFFE: move.b d0, $98(a0)
pub fn i_03cffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D002;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D002: move.b $7a(a6), $9a(a0)
pub fn i_03d002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D008;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D008: move.b #$2, $9b(a0)
pub fn i_03d008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D00E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D00E: rts
pub fn i_03d00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D010;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_DownNeg_03d010  ($03D010, 50 B, player/fire_shells_03c62a.s)
/// $03D010: jsr $3ccd2(pc)
pub fn i_03d010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D014;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D014);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D014: btst.b #$0, $3a(a6)
pub fn i_03d014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D01A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D01A: bne.w $3d026
pub fn i_03d01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D01E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D026; return 10; }
    8
}
/// $03D01E: move.b #$0, d1
pub fn i_03d01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D022;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D022: bra.w $3d02a
pub fn i_03d022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D026;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D02A;
    10
}
/// $03D026: move.b #$2, d1
pub fn i_03d026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D02A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D02A: move.b #$18, d0
pub fn i_03d02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D02E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D02E: lsl.b #$3, d0
pub fn i_03d02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D030;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D030: move.b d0, $98(a0)
pub fn i_03d030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D034;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D034: move.b $7a(a6), $9a(a0)
pub fn i_03d034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D03A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D03A: move.b #$fe, $9b(a0)
pub fn i_03d03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D040;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D040: rts
pub fn i_03d040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D042;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Down0_03d042  ($03D042, 50 B, player/fire_shells_03c62a.s)
/// $03D042: jsr $3ccd2(pc)
pub fn i_03d042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D046;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D046);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D046: btst.b #$0, $3a(a6)
pub fn i_03d046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D04C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D04C: bne.w $3d058
pub fn i_03d04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D050;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D058; return 10; }
    8
}
/// $03D050: move.b #$0, d1
pub fn i_03d050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D054;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D054: bra.w $3d05c
pub fn i_03d054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D058;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D05C;
    10
}
/// $03D058: move.b #$2, d1
pub fn i_03d058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D05C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D05C: move.b #$18, d0
pub fn i_03d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D060;
    cpu.ir = 0x103C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D060: lsl.b #$3, d0
pub fn i_03d060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D062;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D062: move.b d0, $98(a0)
pub fn i_03d062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D066;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D066: move.b $7a(a6), $9a(a0)
pub fn i_03d066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D06C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D06C: move.b #$0, $9b(a0)
pub fn i_03d06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D072;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D072: rts
pub fn i_03d072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D074;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_DownPos_03d074  ($03D074, 50 B, player/fire_shells_03c62a.s)
/// $03D074: jsr $3ccd2(pc)
pub fn i_03d074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D078;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D078);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D078: btst.b #$0, $3a(a6)
pub fn i_03d078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D07E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D07E: bne.w $3d08a
pub fn i_03d07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D082;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D08A; return 10; }
    8
}
/// $03D082: move.b #$0, d1
pub fn i_03d082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D086;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D086: bra.w $3d08e
pub fn i_03d086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D08A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D08E;
    10
}
/// $03D08A: move.b #$2, d1
pub fn i_03d08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D08E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D08E: move.b #$18, d0
pub fn i_03d08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D092;
    cpu.ir = 0x103C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D092: lsl.b #$3, d0
pub fn i_03d092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D094;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D094: move.b d0, $98(a0)
pub fn i_03d094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D098;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D098: move.b $7a(a6), $9a(a0)
pub fn i_03d098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D09E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D09E: move.b #$2, $9b(a0)
pub fn i_03d09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0A4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D0A4: rts
pub fn i_03d0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Spread1_3_03d0a6  ($03D0A6, 94 B, player/fire_shells_03c62a.s)
/// $03D0A6: jsr $3ccd2(pc)
pub fn i_03d0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D0AA);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D0AA: btst.b #$0, $3a(a6)
pub fn i_03d0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0B0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D0B0: bne.w $3d0bc
pub fn i_03d0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D0BC; return 10; }
    8
}
/// $03D0B4: move.b #$0, d1
pub fn i_03d0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0B8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D0B8: bra.w $3d0c0
pub fn i_03d0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D0C0;
    10
}
/// $03D0BC: move.b #$2, d1
pub fn i_03d0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0C0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D0C0: move.b #$1, d0
pub fn i_03d0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0C4;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D0C4: lsl.b #$3, d0
pub fn i_03d0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0C6;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D0C6: move.b d0, $98(a0)
pub fn i_03d0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0CA;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D0CA: move.b $7a(a6), $9a(a0)
pub fn i_03d0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0D0;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D0D0: clr.b $9b(a0)
pub fn i_03d0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0D4;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D0D4: jsr $3ccd2(pc)
pub fn i_03d0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D0D8);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D0D8: btst.b #$0, $3a(a6)
pub fn i_03d0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0DE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D0DE: bne.w $3d0ea
pub fn i_03d0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0E2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D0EA; return 10; }
    8
}
/// $03D0E2: move.b #$0, d1
pub fn i_03d0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0E6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D0E6: bra.w $3d0ee
pub fn i_03d0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D0EE;
    10
}
/// $03D0EA: move.b #$2, d1
pub fn i_03d0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0EE;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D0EE: move.b #$3, d0
pub fn i_03d0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0F2;
    cpu.ir = 0x103C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D0F2: lsl.b #$3, d0
pub fn i_03d0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0F4;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D0F4: move.b d0, $98(a0)
pub fn i_03d0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0F8;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D0F8: move.b $7a(a6), $9a(a0)
pub fn i_03d0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D0FE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D0FE: clr.b $9b(a0)
pub fn i_03d0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D102;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D102: rts
pub fn i_03d102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D104;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Spread5_7_03d104  ($03D104, 94 B, player/fire_shells_03c62a.s)
/// $03D104: jsr $3ccd2(pc)
pub fn i_03d104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D108;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D108);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D108: btst.b #$0, $3a(a6)
pub fn i_03d108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D10E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D10E: bne.w $3d11a
pub fn i_03d10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D112;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D11A; return 10; }
    8
}
/// $03D112: move.b #$0, d1
pub fn i_03d112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D116;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D116: bra.w $3d11e
pub fn i_03d116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D11A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D11E;
    10
}
/// $03D11A: move.b #$2, d1
pub fn i_03d11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D11E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D11E: move.b #$5, d0
pub fn i_03d11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D122;
    cpu.ir = 0x103C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D122: lsl.b #$3, d0
pub fn i_03d122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D124;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D124: move.b d0, $98(a0)
pub fn i_03d124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D128;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D128: move.b $7a(a6), $9a(a0)
pub fn i_03d128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D12E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D12E: clr.b $9b(a0)
pub fn i_03d12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D132;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D132: jsr $3ccd2(pc)
pub fn i_03d132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D136;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D136);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D136: btst.b #$0, $3a(a6)
pub fn i_03d136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D13C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D13C: bne.w $3d148
pub fn i_03d13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D140;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D148; return 10; }
    8
}
/// $03D140: move.b #$0, d1
pub fn i_03d140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D144;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D144: bra.w $3d14c
pub fn i_03d144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D148;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D14C;
    10
}
/// $03D148: move.b #$2, d1
pub fn i_03d148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D14C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D14C: move.b #$7, d0
pub fn i_03d14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D150;
    cpu.ir = 0x103C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D150: lsl.b #$3, d0
pub fn i_03d150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D152;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D152: move.b d0, $98(a0)
pub fn i_03d152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D156;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D156: move.b $7a(a6), $9a(a0)
pub fn i_03d156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D15C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D15C: clr.b $9b(a0)
pub fn i_03d15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D160;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D160: rts
pub fn i_03d160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D162;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Spread9_B_03d162  ($03D162, 94 B, player/fire_shells_03c62a.s)
/// $03D162: jsr $3ccd2(pc)
pub fn i_03d162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D166;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D166);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D166: btst.b #$0, $3a(a6)
pub fn i_03d166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D16C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D16C: bne.w $3d178
pub fn i_03d16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D170;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D178; return 10; }
    8
}
/// $03D170: move.b #$0, d1
pub fn i_03d170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D174;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D174: bra.w $3d17c
pub fn i_03d174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D178;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D17C;
    10
}
/// $03D178: move.b #$2, d1
pub fn i_03d178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D17C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D17C: move.b #$9, d0
pub fn i_03d17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D180;
    cpu.ir = 0x103C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D180: lsl.b #$3, d0
pub fn i_03d180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D182;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D182: move.b d0, $98(a0)
pub fn i_03d182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D186;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D186: move.b $7a(a6), $9a(a0)
pub fn i_03d186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D18C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D18C: clr.b $9b(a0)
pub fn i_03d18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D190;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D190: jsr $3ccd2(pc)
pub fn i_03d190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D194;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D194);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D194: btst.b #$0, $3a(a6)
pub fn i_03d194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D19A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D19A: bne.w $3d1a6
pub fn i_03d19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D19E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D1A6; return 10; }
    8
}
/// $03D19E: move.b #$0, d1
pub fn i_03d19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1A2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D1A2: bra.w $3d1aa
pub fn i_03d1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D1AA;
    10
}
/// $03D1A6: move.b #$2, d1
pub fn i_03d1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1AA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D1AA: move.b #$b, d0
pub fn i_03d1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1AE;
    cpu.ir = 0x103C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D1AE: lsl.b #$3, d0
pub fn i_03d1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1B0;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D1B0: move.b d0, $98(a0)
pub fn i_03d1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1B4;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D1B4: move.b $7a(a6), $9a(a0)
pub fn i_03d1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1BA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D1BA: clr.b $9b(a0)
pub fn i_03d1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1BE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D1BE: rts
pub fn i_03d1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_SpreadD_F_03d1c0  ($03D1C0, 94 B, player/fire_shells_03c62a.s)
/// $03D1C0: jsr $3ccd2(pc)
pub fn i_03d1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D1C4);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D1C4: btst.b #$0, $3a(a6)
pub fn i_03d1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D1CA: bne.w $3d1d6
pub fn i_03d1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D1D6; return 10; }
    8
}
/// $03D1CE: move.b #$0, d1
pub fn i_03d1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1D2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D1D2: bra.w $3d1da
pub fn i_03d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D1DA;
    10
}
/// $03D1D6: move.b #$2, d1
pub fn i_03d1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1DA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D1DA: move.b #$d, d0
pub fn i_03d1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1DE;
    cpu.ir = 0x103C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D1DE: lsl.b #$3, d0
pub fn i_03d1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1E0;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D1E0: move.b d0, $98(a0)
pub fn i_03d1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1E4;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D1E4: move.b $7a(a6), $9a(a0)
pub fn i_03d1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1EA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D1EA: clr.b $9b(a0)
pub fn i_03d1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1EE;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D1EE: jsr $3ccd2(pc)
pub fn i_03d1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D1F2);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D1F2: btst.b #$0, $3a(a6)
pub fn i_03d1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1F8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D1F8: bne.w $3d204
pub fn i_03d1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D1FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D204; return 10; }
    8
}
/// $03D1FC: move.b #$0, d1
pub fn i_03d1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D200;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D200: bra.w $3d208
pub fn i_03d200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D204;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D208;
    10
}
/// $03D204: move.b #$2, d1
pub fn i_03d204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D208;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D208: move.b #$f, d0
pub fn i_03d208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D20C;
    cpu.ir = 0x103C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D20C: lsl.b #$3, d0
pub fn i_03d20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D20E;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D20E: move.b d0, $98(a0)
pub fn i_03d20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D212;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D212: move.b $7a(a6), $9a(a0)
pub fn i_03d212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D218;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D218: clr.b $9b(a0)
pub fn i_03d218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D21C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D21C: rts
pub fn i_03d21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D21E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Spread11_13_03d21e  ($03D21E, 94 B, player/fire_shells_03c62a.s)
/// $03D21E: jsr $3ccd2(pc)
pub fn i_03d21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D222;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D222);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D222: btst.b #$0, $3a(a6)
pub fn i_03d222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D228;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D228: bne.w $3d234
pub fn i_03d228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D22C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D234; return 10; }
    8
}
/// $03D22C: move.b #$0, d1
pub fn i_03d22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D230;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D230: bra.w $3d238
pub fn i_03d230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D234;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D238;
    10
}
/// $03D234: move.b #$2, d1
pub fn i_03d234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D238;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D238: move.b #$11, d0
pub fn i_03d238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D23C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D23C: lsl.b #$3, d0
pub fn i_03d23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D23E;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D23E: move.b d0, $98(a0)
pub fn i_03d23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D242;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D242: move.b $7a(a6), $9a(a0)
pub fn i_03d242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D248;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D248: clr.b $9b(a0)
pub fn i_03d248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D24C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D24C: jsr $3ccd2(pc)
pub fn i_03d24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D250;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D250);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D250: btst.b #$0, $3a(a6)
pub fn i_03d250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D256;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D256: bne.w $3d262
pub fn i_03d256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D25A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D262; return 10; }
    8
}
/// $03D25A: move.b #$0, d1
pub fn i_03d25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D25E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D25E: bra.w $3d266
pub fn i_03d25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D262;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D266;
    10
}
/// $03D262: move.b #$2, d1
pub fn i_03d262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D266;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D266: move.b #$13, d0
pub fn i_03d266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D26A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x13;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D26A: lsl.b #$3, d0
pub fn i_03d26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D26C;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D26C: move.b d0, $98(a0)
pub fn i_03d26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D270;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D270: move.b $7a(a6), $9a(a0)
pub fn i_03d270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D276;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D276: clr.b $9b(a0)
pub fn i_03d276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D27A;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D27A: rts
pub fn i_03d27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D27C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Spread15_17_03d27c  ($03D27C, 94 B, player/fire_shells_03c62a.s)
/// $03D27C: jsr $3ccd2(pc)
pub fn i_03d27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D280;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D280);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D280: btst.b #$0, $3a(a6)
pub fn i_03d280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D286;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D286: bne.w $3d292
pub fn i_03d286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D28A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D292; return 10; }
    8
}
/// $03D28A: move.b #$0, d1
pub fn i_03d28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D28E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D28E: bra.w $3d296
pub fn i_03d28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D292;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D296;
    10
}
/// $03D292: move.b #$2, d1
pub fn i_03d292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D296;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D296: move.b #$15, d0
pub fn i_03d296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D29A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x15;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D29A: lsl.b #$3, d0
pub fn i_03d29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D29C;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D29C: move.b d0, $98(a0)
pub fn i_03d29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2A0;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D2A0: move.b $7a(a6), $9a(a0)
pub fn i_03d2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2A6;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D2A6: clr.b $9b(a0)
pub fn i_03d2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2AA;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D2AA: jsr $3ccd2(pc)
pub fn i_03d2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D2AE);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D2AE: btst.b #$0, $3a(a6)
pub fn i_03d2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2B4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D2B4: bne.w $3d2c0
pub fn i_03d2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D2C0; return 10; }
    8
}
/// $03D2B8: move.b #$0, d1
pub fn i_03d2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2BC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D2BC: bra.w $3d2c4
pub fn i_03d2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D2C4;
    10
}
/// $03D2C0: move.b #$2, d1
pub fn i_03d2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2C4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D2C4: move.b #$17, d0
pub fn i_03d2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2C8;
    cpu.ir = 0x103C;
    let v1: u32 = 0x17;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D2C8: lsl.b #$3, d0
pub fn i_03d2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2CA;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D2CA: move.b d0, $98(a0)
pub fn i_03d2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2CE;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D2CE: move.b $7a(a6), $9a(a0)
pub fn i_03d2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2D4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D2D4: clr.b $9b(a0)
pub fn i_03d2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2D8;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D2D8: rts
pub fn i_03d2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Spread19_1B_03d2da  ($03D2DA, 94 B, player/fire_shells_03c62a.s)
/// $03D2DA: jsr $3ccd2(pc)
pub fn i_03d2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D2DE);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D2DE: btst.b #$0, $3a(a6)
pub fn i_03d2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2E4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D2E4: bne.w $3d2f0
pub fn i_03d2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D2F0; return 10; }
    8
}
/// $03D2E8: move.b #$0, d1
pub fn i_03d2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2EC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D2EC: bra.w $3d2f4
pub fn i_03d2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D2F4;
    10
}
/// $03D2F0: move.b #$2, d1
pub fn i_03d2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2F4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D2F4: move.b #$19, d0
pub fn i_03d2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2F8;
    cpu.ir = 0x103C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D2F8: lsl.b #$3, d0
pub fn i_03d2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2FA;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D2FA: move.b d0, $98(a0)
pub fn i_03d2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D2FE;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D2FE: move.b $7a(a6), $9a(a0)
pub fn i_03d2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D304;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D304: clr.b $9b(a0)
pub fn i_03d304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D308;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D308: jsr $3ccd2(pc)
pub fn i_03d308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D30C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D30C);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D30C: btst.b #$0, $3a(a6)
pub fn i_03d30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D312;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D312: bne.w $3d31e
pub fn i_03d312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D316;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D31E; return 10; }
    8
}
/// $03D316: move.b #$0, d1
pub fn i_03d316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D31A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D31A: bra.w $3d322
pub fn i_03d31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D31E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D322;
    10
}
/// $03D31E: move.b #$2, d1
pub fn i_03d31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D322;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D322: move.b #$1b, d0
pub fn i_03d322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D326;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D326: lsl.b #$3, d0
pub fn i_03d326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D328;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D328: move.b d0, $98(a0)
pub fn i_03d328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D32C;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D32C: move.b $7a(a6), $9a(a0)
pub fn i_03d32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D332;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D332: clr.b $9b(a0)
pub fn i_03d332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D336;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D336: rts
pub fn i_03d336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerFire_Flame_Spread1D_1F_03d338  ($03D338, 94 B, player/fire_shells_03c62a.s)
/// $03D338: jsr $3ccd2(pc)
pub fn i_03d338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D33C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D33C);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D33C: btst.b #$0, $3a(a6)
pub fn i_03d33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D342;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D342: bne.w $3d34e
pub fn i_03d342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D346;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D34E; return 10; }
    8
}
/// $03D346: move.b #$0, d1
pub fn i_03d346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D34A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D34A: bra.w $3d352
pub fn i_03d34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D34E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D352;
    10
}
/// $03D34E: move.b #$2, d1
pub fn i_03d34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D352;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D352: move.b #$1d, d0
pub fn i_03d352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D356;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D356: lsl.b #$3, d0
pub fn i_03d356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D358;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D358: move.b d0, $98(a0)
pub fn i_03d358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D35C;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D35C: move.b $7a(a6), $9a(a0)
pub fn i_03d35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D362;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D362: clr.b $9b(a0)
pub fn i_03d362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D366;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D366: jsr $3ccd2(pc)
pub fn i_03d366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D36A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3D36A);
    cpu.pc = 0x3CCD2;
    16
}
/// $03D36A: btst.b #$0, $3a(a6)
pub fn i_03d36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D370;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D370: bne.w $3d37c
pub fn i_03d370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D374;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D37C; return 10; }
    8
}
/// $03D374: move.b #$0, d1
pub fn i_03d374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D378;
    cpu.ir = 0x123C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D378: bra.w $3d380
pub fn i_03d378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D37C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D380;
    10
}
/// $03D37C: move.b #$2, d1
pub fn i_03d37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D380;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D380: move.b #$1f, d0
pub fn i_03d380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D384;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03D384: lsl.b #$3, d0
pub fn i_03d384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D386;
    cpu.ir = 0xE708;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $03D386: move.b d0, $98(a0)
pub fn i_03d386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D38A;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03D38A: move.b $7a(a6), $9a(a0)
pub fn i_03d38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D390;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03D390: clr.b $9b(a0)
pub fn i_03d390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D394;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03D394: rts
pub fn i_03d394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D396;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShellCasing_Pistol_03d396  ($03D396, 96 B, player/fire_shells_03c62a.s)
/// $03D396: lea.l $3d3f6(pc), a0
pub fn i_03d396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D39A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3D3F6;
    16
}
/// $03D39A: jsr $28cd4.l
pub fn i_03d39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D3A0);
    cpu.pc = 0x28CD4;
    16
}
/// $03D3A0: bra.w $3d3b2
pub fn i_03d3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D3B2;
    10
}
/// $03D3B2: move.w #$184, d1
pub fn i_03d3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x184;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D3B6: jsr $236e.l
pub fn i_03d3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D3BC);
    cpu.pc = 0x236E;
    16
}
/// $03D3BC: move.w #$d000, d0
pub fn i_03d3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D3C0: jsr $28134.l
pub fn i_03d3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D3C6);
    cpu.pc = 0x28134;
    16
}
/// $03D3C6: andi.w #$ffe3, $38(a6)
pub fn i_03d3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3CC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D3CC: ori.w #$10, $38(a6)
pub fn i_03d3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3D2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D3D2: addi.w #$200, $38(a6)
pub fn i_03d3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3D8;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x200, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D3D8: lea.l $3d3de(pc), a1
pub fn i_03d3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D3DE;
    16
}
/// $03D3DC: move.l a1, (a6)
pub fn i_03d3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03D3DE: jsr $2783a.l
pub fn i_03d3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D3E4);
    cpu.pc = 0x2783A;
    16
}
/// $03D3E4: jsr $28d70.l
pub fn i_03d3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D3EA);
    cpu.pc = 0x28D70;
    16
}
/// $03D3EA: bcc.w $3d3f4
pub fn i_03d3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3D3F4; return 10; }
    8
}
/// $03D3EE: jmp $518.l
pub fn i_03d3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3F4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03D3F4: rts
pub fn i_03d3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D3F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ShellCasing_Rocket_03d4ea  ($03D4EA, 14 B, player/fire_shells_03c62a.s)
/// $03D4EA: lea.l $3d54a(pc), a0
pub fn i_03d4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D4EE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3D54A;
    16
}
/// $03D4EE: jsr $28cd4.l
pub fn i_03d4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D4F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D4F4);
    cpu.pc = 0x28CD4;
    16
}
/// $03D4F4: bra.w $3d506
pub fn i_03d4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D4F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D506;
    10
}
// ===== ShellCasing_RocketB_03d4f8  ($03D4F8, 82 B, player/fire_shells_03c62a.s)
/// $03D4F8: lea.l $3d5b0(pc), a0
pub fn i_03d4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D4FC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3D5B0;
    16
}
/// $03D4FC: jsr $28cd4.l
pub fn i_03d4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D502);
    cpu.pc = 0x28CD4;
    16
}
/// $03D502: bra.w $3d506
pub fn i_03d502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D506;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D506;
    10
}
/// $03D506: move.w #$17c, d1
pub fn i_03d506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D50A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D50A: jsr $236e.l
pub fn i_03d50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D510);
    cpu.pc = 0x236E;
    16
}
/// $03D510: move.w #$d000, d0
pub fn i_03d510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D514;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D514: jsr $28134.l
pub fn i_03d514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D51A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D51A);
    cpu.pc = 0x28134;
    16
}
/// $03D51A: andi.w #$ffe3, $38(a6)
pub fn i_03d51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D520;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D520: ori.w #$10, $38(a6)
pub fn i_03d520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D526;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D526: addi.w #$200, $38(a6)
pub fn i_03d526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D52C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x200, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D52C: lea.l $3d532(pc), a1
pub fn i_03d52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D530;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D532;
    16
}
/// $03D530: move.l a1, (a6)
pub fn i_03d530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D532;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03D532: jsr $2783a.l
pub fn i_03d532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D538;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D538);
    cpu.pc = 0x2783A;
    16
}
/// $03D538: jsr $28d70.l
pub fn i_03d538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D53E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D53E);
    cpu.pc = 0x28D70;
    16
}
/// $03D53E: bcc.w $3d548
pub fn i_03d53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D542;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3D548; return 10; }
    8
}
/// $03D542: jmp $518.l
pub fn i_03d542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D548;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03D548: rts
pub fn i_03d548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D54A;
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
// ===== Scene3Debris_SpawnLow8_03d640  ($03D640, 46 B, player/fire_shells_03c62a.s)
/// $03D640: cmpi.b #$3, $106ece.l
pub fn i_03d640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D648;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $03D648: bne.w $3d66c
pub fn i_03d648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D64C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D66C; return 10; }
    8
}
/// $03D64C: lea.l $3d76e(pc), a1
pub fn i_03d64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D650;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D76E;
    16
}
/// $03D650: jsr $4ae.l
pub fn i_03d650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D656;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D656);
    cpu.pc = 0x4AE;
    16
}
/// $03D656: jsr $5dd02.l
pub fn i_03d656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D65C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D65C);
    cpu.pc = 0x5DD02;
    16
}
/// $03D65C: andi.w #$ffe3, $38(a0)
pub fn i_03d65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D662;
    cpu.ir = 0x268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D662: ori.w #$10, $38(a0)
pub fn i_03d662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D668;
    cpu.ir = 0x68;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D668: addq.w #$8, $24(a0)
pub fn i_03d668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D66C;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03D66C: rts
pub fn i_03d66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D66E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Debris_SpawnUpD_03d66e  ($03D66E, 70 B, player/fire_shells_03c62a.s)
/// $03D66E: cmpi.b #$3, $106ece.l
pub fn i_03d66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D676;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $03D676: bne.w $3d6b2
pub fn i_03d676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D67A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D6B2; return 10; }
    8
}
/// $03D67A: lea.l $3d76e(pc), a1
pub fn i_03d67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D67E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D76E;
    16
}
/// $03D67E: jsr $4ae.l
pub fn i_03d67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D684;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D684);
    cpu.pc = 0x4AE;
    16
}
/// $03D684: jsr $5dd02.l
pub fn i_03d684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D68A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D68A);
    cpu.pc = 0x5DD02;
    16
}
/// $03D68A: andi.w #$ffe3, $38(a0)
pub fn i_03d68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D690;
    cpu.ir = 0x268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D690: ori.w #$10, $38(a0)
pub fn i_03d690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D696;
    cpu.ir = 0x68;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D696: subi.w #$d, $24(a0)
pub fn i_03d696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D69C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xD, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D69C: btst.b #$0, $3a(a6)
pub fn i_03d69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6A2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D6A2: bne.w $3d6ae
pub fn i_03d6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D6AE; return 10; }
    8
}
/// $03D6A6: addq.w #$3, $22(a0)
pub fn i_03d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6AA;
    cpu.ir = 0x5668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03D6AA: bra.w $3d6b2
pub fn i_03d6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6AE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D6B2;
    10
}
/// $03D6AE: subq.w #$3, $22(a0)
pub fn i_03d6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6B2;
    cpu.ir = 0x5768;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03D6B2: rts
pub fn i_03d6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Debris_SpawnUpC_03d6b4  ($03D6B4, 48 B, player/fire_shells_03c62a.s)
/// $03D6B4: cmpi.b #$3, $106ece.l
pub fn i_03d6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6BC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $03D6BC: bne.w $3d6e2
pub fn i_03d6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D6E2; return 10; }
    8
}
/// $03D6C0: lea.l $3d76e(pc), a1
pub fn i_03d6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D76E;
    16
}
/// $03D6C4: jsr $4ae.l
pub fn i_03d6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D6CA);
    cpu.pc = 0x4AE;
    16
}
/// $03D6CA: jsr $5dd02.l
pub fn i_03d6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D6D0);
    cpu.pc = 0x5DD02;
    16
}
/// $03D6D0: andi.w #$ffe3, $38(a0)
pub fn i_03d6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6D6;
    cpu.ir = 0x268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D6D6: ori.w #$10, $38(a0)
pub fn i_03d6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6DC;
    cpu.ir = 0x68;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D6DC: subi.w #$c, $24(a0)
pub fn i_03d6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6E2;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D6E2: rts
pub fn i_03d6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Debris_SpawnUpB_03d6e4  ($03D6E4, 70 B, player/fire_shells_03c62a.s)
/// $03D6E4: cmpi.b #$3, $106ece.l
pub fn i_03d6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6EC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $03D6EC: bne.w $3d728
pub fn i_03d6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D728; return 10; }
    8
}
/// $03D6F0: lea.l $3d76e(pc), a1
pub fn i_03d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D76E;
    16
}
/// $03D6F4: jsr $4ae.l
pub fn i_03d6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D6FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D6FA);
    cpu.pc = 0x4AE;
    16
}
/// $03D6FA: jsr $5dd02.l
pub fn i_03d6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D700;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D700);
    cpu.pc = 0x5DD02;
    16
}
/// $03D700: andi.w #$ffe3, $38(a0)
pub fn i_03d700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D706;
    cpu.ir = 0x268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D706: ori.w #$10, $38(a0)
pub fn i_03d706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D70C;
    cpu.ir = 0x68;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D70C: subi.w #$b, $24(a0)
pub fn i_03d70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D712;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xB, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D712: btst.b #$0, $3a(a6)
pub fn i_03d712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D718;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D718: bne.w $3d724
pub fn i_03d718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D71C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D724; return 10; }
    8
}
/// $03D71C: subq.w #$4, $22(a0)
pub fn i_03d71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D720;
    cpu.ir = 0x5968;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03D720: bra.w $3d728
pub fn i_03d720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D724;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D728;
    10
}
/// $03D724: addq.w #$4, $22(a0)
pub fn i_03d724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D728;
    cpu.ir = 0x5868;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03D728: rts
pub fn i_03d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D72A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Debris_SpawnPrio4_03d72a  ($03D72A, 68 B, player/fire_shells_03c62a.s)
/// $03D72A: cmpi.b #$3, $106ece.l
pub fn i_03d72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D732;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $03D732: bne.w $3d76c
pub fn i_03d732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D736;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D76C; return 10; }
    8
}
/// $03D736: lea.l $3d76e(pc), a1
pub fn i_03d736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D73A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D76E;
    16
}
/// $03D73A: jsr $4ae.l
pub fn i_03d73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D740;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D740);
    cpu.pc = 0x4AE;
    16
}
/// $03D740: jsr $5dd02.l
pub fn i_03d740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D746;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D746);
    cpu.pc = 0x5DD02;
    16
}
/// $03D746: andi.w #$ffe3, $38(a0)
pub fn i_03d746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D74C;
    cpu.ir = 0x268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D74C: ori.w #$4, $38(a0)
pub fn i_03d74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D752;
    cpu.ir = 0x68;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D752: subq.w #$6, $24(a0)
pub fn i_03d752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D756;
    cpu.ir = 0x5D68;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 6, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03D756: btst.b #$0, $3a(a6)
pub fn i_03d756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D75C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03D75C: bne.w $3d768
pub fn i_03d75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D760;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3D768; return 10; }
    8
}
/// $03D760: subq.w #$3, $22(a0)
pub fn i_03d760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D764;
    cpu.ir = 0x5768;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03D764: bra.w $3d76c
pub fn i_03d764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D768;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D76C;
    10
}
/// $03D768: addq.w #$3, $22(a0)
pub fn i_03d768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D76C;
    cpu.ir = 0x5668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03D76C: rts
pub fn i_03d76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D76E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scene3Debris_Task_03d76e  ($03D76E, 50 B, player/fire_shells_03c62a.s)
/// $03D76E: move.w #$1bd, d1
pub fn i_03d76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D772;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D772: jsr $236e.l
pub fn i_03d772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D778);
    cpu.pc = 0x236E;
    16
}
/// $03D778: lea.l $3d7a0(pc), a0
pub fn i_03d778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D77C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3D7A0;
    16
}
/// $03D77C: jsr $28cd4.l
pub fn i_03d77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D782);
    cpu.pc = 0x28CD4;
    16
}
/// $03D782: lea.l $3d788(pc), a1
pub fn i_03d782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D786;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D788;
    16
}
/// $03D786: move.l a1, (a6)
pub fn i_03d786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D788;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03D788: jsr $2783a.l
pub fn i_03d788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D78E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D78E);
    cpu.pc = 0x2783A;
    16
}
/// $03D78E: jsr $28d70.l
pub fn i_03d78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D794;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D794);
    cpu.pc = 0x28D70;
    16
}
/// $03D794: bcc.w $3d79e
pub fn i_03d794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D798;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3D79E; return 10; }
    8
}
/// $03D798: jmp $518.l
pub fn i_03d798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D79E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03D79E: rts
pub fn i_03d79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D7A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_DeathBlood_03d842  ($03D842, 82 B, player/fire_shells_03c62a.s)
/// $03D842: lea.l $3d894(pc), a0
pub fn i_03d842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D846;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3D894;
    16
}
/// $03D846: jsr $28cd4.l
pub fn i_03d846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D84C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D84C);
    cpu.pc = 0x28CD4;
    16
}
/// $03D84C: bra.w $3d850
pub fn i_03d84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D850;
    cpu.ir = 0x6000;
    cpu.pc = 0x3D850;
    10
}
/// $03D850: move.w #$17a, d1
pub fn i_03d850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D854;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D854: jsr $236e.l
pub fn i_03d854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D85A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D85A);
    cpu.pc = 0x236E;
    16
}
/// $03D85A: move.w #$d000, d0
pub fn i_03d85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D85E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D85E: jsr $28134.l
pub fn i_03d85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D864;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D864);
    cpu.pc = 0x28134;
    16
}
/// $03D864: andi.w #$ffe3, $38(a6)
pub fn i_03d864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D86A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D86A: ori.w #$10, $38(a6)
pub fn i_03d86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D870;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D870: addi.w #$200, $38(a6)
pub fn i_03d870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D876;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x200, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D876: lea.l $3d87c(pc), a1
pub fn i_03d876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D87A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D87C;
    16
}
/// $03D87A: move.l a1, (a6)
pub fn i_03d87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D87C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03D87C: jsr $2783a.l
pub fn i_03d87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D882);
    cpu.pc = 0x2783A;
    16
}
/// $03D882: jsr $28d70.l
pub fn i_03d882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D888);
    cpu.pc = 0x28D70;
    16
}
/// $03D888: bcc.w $3d892
pub fn i_03d888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D88C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3D892; return 10; }
    8
}
/// $03D88C: jmp $518.l
pub fn i_03d88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D892;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03D892: rts
pub fn i_03d892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D894;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_SpawnFx3D8FA_03d8fa  ($03D8FA, 74 B, player/fire_shells_03c62a.s)
/// $03D8FA: move.w #$17c, d1
pub fn i_03d8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D8FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D8FE: jsr $236e.l
pub fn i_03d8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D904);
    cpu.pc = 0x236E;
    16
}
/// $03D904: move.w #$2000, d0
pub fn i_03d904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D908;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03D908: jsr $28134.l
pub fn i_03d908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D90E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D90E);
    cpu.pc = 0x28134;
    16
}
/// $03D90E: andi.w #$ffe3, $38(a6)
pub fn i_03d90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D914;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D914: ori.w #$10, $38(a6)
pub fn i_03d914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D91A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $03D91A: lea.l $2f7d3a.l, a0
pub fn i_03d91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D920;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7D3A;
    20
}
/// $03D920: jsr $28cd4.l
pub fn i_03d920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D926;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D926);
    cpu.pc = 0x28CD4;
    16
}
/// $03D926: lea.l $3d92c(pc), a1
pub fn i_03d926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D92A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3D92C;
    16
}
/// $03D92A: move.l a1, (a6)
pub fn i_03d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D92C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03D92C: jsr $2783a.l
pub fn i_03d92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D932);
    cpu.pc = 0x2783A;
    16
}
/// $03D932: jsr $28d70.l
pub fn i_03d932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D938;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3D938);
    cpu.pc = 0x28D70;
    16
}
/// $03D938: bcc.w $3d942
pub fn i_03d938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D93C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3D942; return 10; }
    8
}
/// $03D93C: jmp $518.l
pub fn i_03d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D942;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $03D942: rts
pub fn i_03d942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3D944;
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
// ===== SlugCannon_ArmOffsetCurve_03daa8  ($03DAA8, 276 B, pow/results_squadleader_03daa8.s)
/// $03DAA8: dc.w $ffea
pub fn i_03daa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DAAA;
    cpu.ir = 0xFFEA;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== ClearXN_03dbbc  ($03DBBC, 6 B, thunks/ccr_helpers.c)
/// $03DBBC: andi.b #$ee, ccr
pub fn i_03dbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBC0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $03DBC0: rts
pub fn i_03dbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_03dbc2  ($03DBC2, 6 B, thunks/ccr_helpers.c)
/// $03DBC2: ori.b #$11, ccr
pub fn i_03dbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBC6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $03DBC6: rts
pub fn i_03dbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_Entry_03dbc8  ($03DBC8, 30 B, pow/results_squadleader_03daa8.s)
/// $03DBC8: cmpi.b #$1, $10fdb6.l
pub fn i_03dbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBD0;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $03DBD0: beq.w $3dbf4
pub fn i_03dbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3DBF4; return 10; }
    8
}
/// $03DBD4: cmpi.b #$1, $10fdb7.l
pub fn i_03dbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBDC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $03DBDC: beq.w $3dbf4
pub fn i_03dbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBE0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3DBF4; return 10; }
    8
}
/// $03DBE0: clr.b $106ed2.l
pub fn i_03dbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBE6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
// ===== Jsr5B6ThenJmpScheduler_03dbe6  ($03DBE6, 14 B, kernel/jsr5b6_jmp518.c)
/// $03DBE6: jsr $5b6.l
pub fn i_03dbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DBEC);
    cpu.pc = 0x5B6;
    16
}
/// $03DBEC: jmp $518.l
pub fn i_03dbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBF2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Results_WaitByLevel_03dbf4  ($03DBF4, 26 B, pow/results_squadleader_03daa8.s)
/// $03DBF4: lea.l $28293a.l, a1
pub fn i_03dbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBFA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28293A;
    20
}
/// $03DBFA: clr.w d0
pub fn i_03dbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DBFC;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DBFC: move.b $106ecf.l, d0
pub fn i_03dbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC02;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $03DC02: andi.w #$7, d0
pub fn i_03dc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC06;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DC06: lsl.w #$1, d0
pub fn i_03dc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC08;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03DC08: move.w (a1, d0.w), $70(a6)
pub fn i_03dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC0E;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    22
}
// ===== SetTaskHandler_03dc0e  ($03DC0E, 8 B, kernel/task_handlers.c)
/// $03DC0E: lea.l $3dc16(pc), a1
pub fn i_03dc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DC16;
    16
}
/// $03DC12: move.l a1, (a6)
pub fn i_03dc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DC14: rts
pub fn i_03dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_Countdown_03dc16  ($03DC16, 14 B, pow/results_squadleader_03daa8.s)
/// $03DC16: subq.w #$1, $70(a6)
pub fn i_03dc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC1A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03DC1A: cmpi.w #$0, $70(a6)
pub fn i_03dc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC20;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03DC20: bgt.w $3dc2a
pub fn i_03dc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC24;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3DC2A; return 10; }
    8
}
// ===== SetTaskHandler_03dc24  ($03DC24, 8 B, kernel/task_handlers.c)
/// $03DC24: lea.l $3dc2c(pc), a1
pub fn i_03dc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DC2C;
    16
}
/// $03DC28: move.l a1, (a6)
pub fn i_03dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC2A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DC2A: rts
pub fn i_03dc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_FadeInit_03dc2c  ($03DC2C, 64 B, pow/results_squadleader_03daa8.s)
/// $03DC2C: jsr $22c8.l
pub fn i_03dc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DC32);
    cpu.pc = 0x22C8;
    16
}
/// $03DC32: move.b #$60, d0
pub fn i_03dc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC36;
    cpu.ir = 0x103C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03DC36: jsr $2308.l
pub fn i_03dc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DC3C);
    cpu.pc = 0x2308;
    16
}
/// $03DC3C: move.w #$18, d0
pub fn i_03dc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC40;
    cpu.ir = 0x303C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DC40: move.w #$18, d1
pub fn i_03dc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC44;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DC44: move.w #$18, d2
pub fn i_03dc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC48;
    cpu.ir = 0x343C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DC48: move.w #$5, d3
pub fn i_03dc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC4C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DC4C: jsr $52580.l
pub fn i_03dc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DC52);
    cpu.pc = 0x52580;
    16
}
/// $03DC52: move.w #$f, $70(a6)
pub fn i_03dc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC58;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DC58: lea.l $3dc5e(pc), a1
pub fn i_03dc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DC5E;
    16
}
/// $03DC5C: move.l a1, (a6)
pub fn i_03dc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DC5E: subq.w #$1, $70(a6)
pub fn i_03dc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC62;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03DC62: cmpi.w #$0, $70(a6)
pub fn i_03dc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC68;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03DC68: bgt.w $3dc72
pub fn i_03dc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC6C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3DC72; return 10; }
    8
}
// ===== SetTaskHandler_03dc6c  ($03DC6C, 8 B, kernel/task_handlers.c)
/// $03DC6C: lea.l $3dc74(pc), a1
pub fn i_03dc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DC74;
    16
}
/// $03DC70: move.l a1, (a6)
pub fn i_03dc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DC72: rts
pub fn i_03dc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_SetupPlayers_03dc74  ($03DC74, 172 B, pow/results_squadleader_03daa8.s)
/// $03DC74: clr.l $80(a6)
pub fn i_03dc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC78;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03DC78: clr.l $84(a6)
pub fn i_03dc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC7C;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03DC7C: clr.b $88(a6)
pub fn i_03dc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC80;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DC80: clr.b $8a(a6)
pub fn i_03dc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC84;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DC84: clr.b $8e(a6)
pub fn i_03dc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC88;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DC88: clr.b $90(a6)
pub fn i_03dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC8C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DC8C: cmpi.b #$1, $10fdb6.l
pub fn i_03dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC94;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB6) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $03DC94: bne.w $3dcb8
pub fn i_03dc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3DCB8; return 10; }
    8
}
/// $03DC98: lea.l $3df56(pc), a1
pub fn i_03dc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DC9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DF56;
    16
}
/// $03DC9C: jsr $4ae.l
pub fn i_03dc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DCA2);
    cpu.pc = 0x4AE;
    16
}
/// $03DCA2: move.l a0, $80(a6)
pub fn i_03dca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCA6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03DCA6: addq.b #$1, $88(a6)
pub fn i_03dca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCAA;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DCAA: addq.b #$1, $8e(a6)
pub fn i_03dcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCAE;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DCAE: move.w $106f4c.l, d1
pub fn i_03dcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCB4;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F4C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $03DCB4: bra.w $3dcba
pub fn i_03dcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCB8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3DCBA;
    10
}
/// $03DCB8: clr.w d1
pub fn i_03dcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCBA;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DCBA: cmpi.b #$1, $10fdb7.l
pub fn i_03dcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCC2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FDB7) as u32;
    sub_with_flags(cpu, v2, 0x1, Size::Byte, false);
    20
}
/// $03DCC2: bne.w $3dce6
pub fn i_03dcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3DCE6; return 10; }
    8
}
/// $03DCC6: lea.l $3dfdc(pc), a1
pub fn i_03dcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DFDC;
    16
}
/// $03DCCA: jsr $4ae.l
pub fn i_03dcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DCD0);
    cpu.pc = 0x4AE;
    16
}
/// $03DCD0: move.l a0, $84(a6)
pub fn i_03dcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCD4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03DCD4: addq.b #$1, $88(a6)
pub fn i_03dcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCD8;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DCD8: addq.b #$1, $8e(a6)
pub fn i_03dcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCDC;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x8E);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DCDC: move.w $106f4e.l, d2
pub fn i_03dcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCE2;
    cpu.ir = 0x3439;
    let v2 = bus.read16(0x106F4E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $03DCE2: bra.w $3dce8
pub fn i_03dce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3DCE8;
    10
}
/// $03DCE6: clr.w d2
pub fn i_03dce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCE8;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DCE8: jsr $516da.l
pub fn i_03dce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DCEE);
    cpu.pc = 0x516DA;
    16
}
/// $03DCEE: cmp.w d1, d2
pub fn i_03dcee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCF0;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $03DCF0: bcc.w $3dcfe
pub fn i_03dcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3DCFE; return 10; }
    8
}
/// $03DCF4: move.w #$0, $8c(a6)
pub fn i_03dcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DCFA: bra.w $3dd04
pub fn i_03dcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DCFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3DD04;
    10
}
/// $03DCFE: move.w #$1, $8c(a6)
pub fn i_03dcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD04;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DD04: cmp.w d1, d2
pub fn i_03dd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD06;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $03DD06: bne.w $3dd10
pub fn i_03dd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3DD10; return 10; }
    8
}
/// $03DD0A: move.w #$2, $8c(a6)
pub fn i_03dd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD10;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DD10: move.b $88(a6), $89(a6)
pub fn i_03dd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD16;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03DD16: cmpi.b #$0, $88(a6)
pub fn i_03dd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD1C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03DD1C: bne.w $3dd28
pub fn i_03dd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD20;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3DD28; return 10; }
    8
}
// ===== SetTaskHandler_03dd20  ($03DD20, 8 B, kernel/task_handlers.c)
/// $03DD20: lea.l $3debe(pc), a1
pub fn i_03dd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DEBE;
    16
}
/// $03DD24: move.l a1, (a6)
pub fn i_03dd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DD26: rts
pub fn i_03dd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_DrawFrame_03dd28  ($03DD28, 376 B, pow/results_squadleader_03daa8.s)
/// $03DD28: cmpi.b #$0, $10fd83.l
pub fn i_03dd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD30;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $03DD30: bne.w $3de8c
pub fn i_03dd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD34;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3DE8C; return 10; }
    8
}
/// $03DD34: move.w #$71e6, d0
pub fn i_03dd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD38;
    cpu.ir = 0x303C;
    let v1: u32 = 0x71E6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DD38: move.w #$d22, d1
pub fn i_03dd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD3C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DD3C: ori.w #$9000, d1
pub fn i_03dd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD40;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x9000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD40: movem.w d0-d1, $3c0000.l
pub fn i_03dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD48;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DD48: addi.w #$20, d0
pub fn i_03dd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD4C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD4C: addq.w #$1, d1
pub fn i_03dd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD4E;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DD4E: movem.w d0-d1, $3c0000.l
pub fn i_03dd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD56;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DD56: subi.w #$1f, d0
pub fn i_03dd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD5A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD5A: addi.w #$f, d1
pub fn i_03dd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD5E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD5E: movem.w d0-d1, $3c0000.l
pub fn i_03dd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD66;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DD66: addi.w #$20, d0
pub fn i_03dd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD6A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD6A: addq.w #$1, d1
pub fn i_03dd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD6C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DD6C: movem.w d0-d1, $3c0000.l
pub fn i_03dd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD74;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DD74: subi.w #$21, d0
pub fn i_03dd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD78;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD78: move.w #$7226, d0
pub fn i_03dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD7C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7226;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DD7C: move.w #$d24, d1
pub fn i_03dd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD80;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DD80: ori.w #$9000, d1
pub fn i_03dd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD84;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x9000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD84: movem.w d0-d1, $3c0000.l
pub fn i_03dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD8C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DD8C: addi.w #$20, d0
pub fn i_03dd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD90;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD90: addq.w #$1, d1
pub fn i_03dd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD92;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DD92: movem.w d0-d1, $3c0000.l
pub fn i_03dd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD9A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DD9A: subi.w #$1f, d0
pub fn i_03dd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DD9E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DD9E: addi.w #$f, d1
pub fn i_03dd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDA2;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DDA2: movem.w d0-d1, $3c0000.l
pub fn i_03dda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDAA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DDAA: addi.w #$20, d0
pub fn i_03ddaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDAE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DDAE: addq.w #$1, d1
pub fn i_03ddae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDB0;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DDB0: movem.w d0-d1, $3c0000.l
pub fn i_03ddb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDB8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DDB8: subi.w #$21, d0
pub fn i_03ddb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDBC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DDBC: move.w #$7266, d0
pub fn i_03ddbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDC0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7266;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DDC0: move.w #$de6, d1
pub fn i_03ddc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDC4;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDE6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DDC4: ori.w #$9000, d1
pub fn i_03ddc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDC8;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x9000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DDC8: movem.w d0-d1, $3c0000.l
pub fn i_03ddc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDD0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DDD0: addi.w #$20, d0
pub fn i_03ddd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDD4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DDD4: addq.w #$1, d1
pub fn i_03ddd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDD6;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DDD6: movem.w d0-d1, $3c0000.l
pub fn i_03ddd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDDE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DDDE: subi.w #$1f, d0
pub fn i_03ddde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDE2;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DDE2: addi.w #$f, d1
pub fn i_03dde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDE6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DDE6: movem.w d0-d1, $3c0000.l
pub fn i_03dde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDEE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DDEE: addi.w #$20, d0
pub fn i_03ddee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDF2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DDF2: addq.w #$1, d1
pub fn i_03ddf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDF4;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DDF4: movem.w d0-d1, $3c0000.l
pub fn i_03ddf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DDFC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DDFC: subi.w #$21, d0
pub fn i_03ddfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE00;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE00: move.w #$72a6, d0
pub fn i_03de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72A6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DE04: move.w #$de8, d1
pub fn i_03de04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE08;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DE08: ori.w #$9000, d1
pub fn i_03de08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE0C;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x9000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE0C: movem.w d0-d1, $3c0000.l
pub fn i_03de0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE14;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DE14: addi.w #$20, d0
pub fn i_03de14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE18;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE18: addq.w #$1, d1
pub fn i_03de18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE1A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DE1A: movem.w d0-d1, $3c0000.l
pub fn i_03de1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE22;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DE22: subi.w #$1f, d0
pub fn i_03de22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE26;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE26: addi.w #$f, d1
pub fn i_03de26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE2A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE2A: movem.w d0-d1, $3c0000.l
pub fn i_03de2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE32;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DE32: addi.w #$20, d0
pub fn i_03de32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE36;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE36: addq.w #$1, d1
pub fn i_03de36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE38;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DE38: movem.w d0-d1, $3c0000.l
pub fn i_03de38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE40;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DE40: subi.w #$21, d0
pub fn i_03de40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE44;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE44: move.w #$72e6, d0
pub fn i_03de44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x72E6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DE48: move.w #$dea, d1
pub fn i_03de48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xDEA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DE4C: ori.w #$9000, d1
pub fn i_03de4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE50;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x9000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE50: movem.w d0-d1, $3c0000.l
pub fn i_03de50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE58;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DE58: addi.w #$20, d0
pub fn i_03de58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE5C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE5C: addq.w #$1, d1
pub fn i_03de5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE5E;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DE5E: movem.w d0-d1, $3c0000.l
pub fn i_03de5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE66;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DE66: subi.w #$1f, d0
pub fn i_03de66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE6A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE6A: addi.w #$f, d1
pub fn i_03de6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE6E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE6E: movem.w d0-d1, $3c0000.l
pub fn i_03de6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE76;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DE76: addi.w #$20, d0
pub fn i_03de76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE7A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE7A: addq.w #$1, d1
pub fn i_03de7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE7C;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DE7C: movem.w d0-d1, $3c0000.l
pub fn i_03de7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE84;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03DE84: subi.w #$21, d0
pub fn i_03de84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE88;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DE88: bra.w $3dea0
pub fn i_03de88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x3DEA0;
    10
}
/// $03DE8C: movea.w #$7146, a1
pub fn i_03de8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE90;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7146;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03DE90: move.b #$3, d1
pub fn i_03de90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE94;
    cpu.ir = 0x123C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03DE94: lea.l $282a68.l, a2
pub fn i_03de94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DE9A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A68;
    20
}
/// $03DE9A: jsr $477fc.l
pub fn i_03de9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DEA0);
    cpu.pc = 0x477FC;
    16
}
// ===== SetTaskHandler_03dea0  ($03DEA0, 8 B, kernel/task_handlers.c)
/// $03DEA0: lea.l $3dea8(pc), a1
pub fn i_03dea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DEA8;
    16
}
/// $03DEA4: move.l a1, (a6)
pub fn i_03dea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DEA6: rts
pub fn i_03dea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_WaitDone_03dea8  ($03DEA8, 14 B, pow/results_squadleader_03daa8.s)
/// $03DEA8: jsr $3edd2(pc)
pub fn i_03dea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEAC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3DEAC);
    cpu.pc = 0x3EDD2;
    16
}
/// $03DEAC: cmpi.b #$0, $90(a6)
pub fn i_03deac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEB2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03DEB2: beq.w $3debc
pub fn i_03deb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3DEBC; return 10; }
    8
}
// ===== SetTaskHandler_03deb6  ($03DEB6, 8 B, kernel/task_handlers.c)
/// $03DEB6: lea.l $3debe(pc), a1
pub fn i_03deb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DEBE;
    16
}
/// $03DEBA: move.l a1, (a6)
pub fn i_03deba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DEBC: rts
pub fn i_03debc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_FadeOut_03debe  ($03DEBE, 28 B, pow/results_squadleader_03daa8.s)
/// $03DEBE: move.w #$0, d0
pub fn i_03debe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEC2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DEC2: move.w #$0, d1
pub fn i_03dec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEC6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DEC6: move.w #$0, d2
pub fn i_03dec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DECA;
    cpu.ir = 0x343C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DECA: move.w #$3, d3
pub fn i_03deca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DECE;
    cpu.ir = 0x363C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DECE: jsr $52580.l
pub fn i_03dece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DED4);
    cpu.pc = 0x52580;
    16
}
/// $03DED4: move.w #$a, $70(a6)
pub fn i_03ded4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEDA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_03deda  ($03DEDA, 8 B, kernel/task_handlers.c)
/// $03DEDA: lea.l $3dee2(pc), a1
pub fn i_03deda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DEE2;
    16
}
/// $03DEDE: move.l a1, (a6)
pub fn i_03dede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DEE0: rts
pub fn i_03dee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_Teardown_03dee2  ($03DEE2, 72 B, pow/results_squadleader_03daa8.s)
/// $03DEE2: subq.w #$1, $70(a6)
pub fn i_03dee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEE6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03DEE6: cmpi.w #$0, $70(a6)
pub fn i_03dee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEEC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03DEEC: bgt.w $3df30
pub fn i_03deec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEF0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3DF30; return 10; }
    8
}
/// $03DEF0: adda.w $24(a6), a1
pub fn i_03def0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEF4;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03DEF4: movea.w #$7046, a1
pub fn i_03def4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEF8;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7046;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03DEF8: move.w #$2320, d0
pub fn i_03def8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DEFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DEFC: move.w #$24, d1
pub fn i_03defc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF00;
    cpu.ir = 0x323C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DF00: move.w #$16, d2
pub fn i_03df00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF04;
    cpu.ir = 0x343C;
    let v1: u32 = 0x16;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DF04: jsr $5da9c.l
pub fn i_03df04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DF0A);
    cpu.pc = 0x5DA9C;
    16
}
/// $03DF0A: jsr $5b6.l
pub fn i_03df0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DF10);
    cpu.pc = 0x5B6;
    16
}
/// $03DF10: lea.l $28294a.l, a1
pub fn i_03df10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF16;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x28294A;
    20
}
/// $03DF16: clr.w d0
pub fn i_03df16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF18;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03DF18: move.b $106ecf.l, d0
pub fn i_03df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF1E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECF) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $03DF1E: andi.w #$7, d0
pub fn i_03df1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF22;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03DF22: lsl.w #$1, d0
pub fn i_03df22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF24;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03DF24: move.w (a1, d0.w), $70(a6)
pub fn i_03df24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF2A;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    22
}
// ===== SetTaskHandler_03df2a  ($03DF2A, 8 B, kernel/task_handlers.c)
/// $03DF2A: lea.l $3df32(pc), a1
pub fn i_03df2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DF32;
    16
}
/// $03DF2E: move.l a1, (a6)
pub fn i_03df2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DF30: rts
pub fn i_03df30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ToBanner_03df32  ($03DF32, 26 B, pow/results_squadleader_03daa8.s)
/// $03DF32: subq.w #$1, $70(a6)
pub fn i_03df32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF36;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03DF36: cmpi.w #$0, $70(a6)
pub fn i_03df36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF3C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03DF3C: bgt.w $3df52
pub fn i_03df3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF40;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3DF52; return 10; }
    8
}
/// $03DF40: lea.l $7a9f0.l, a1
pub fn i_03df40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF46;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7A9F0;
    20
}
/// $03DF46: jsr $4ae.l
pub fn i_03df46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DF4C);
    cpu.pc = 0x4AE;
    16
}
// ===== SetTaskHandler_03df4c  ($03DF4C, 8 B, kernel/task_handlers.c)
/// $03DF4C: lea.l $3df54(pc), a1
pub fn i_03df4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3DF54;
    16
}
/// $03DF50: move.l a1, (a6)
pub fn i_03df50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03DF52: rts
pub fn i_03df52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF54;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Stub_0003DF54  ($03DF54, 2 B, thunks/trivial_rts.c)
/// $03DF54: rts
pub fn i_03df54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_PlayerColumn_03df56  ($03DF56, 294 B, pow/results_squadleader_03daa8.s)
/// $03DF56: clr.b $20(a6)
pub fn i_03df56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF5A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03DF5A: lea.l $3e514(pc), a1
pub fn i_03df5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E514;
    16
}
/// $03DF5E: jsr $4ae.l
pub fn i_03df5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DF64);
    cpu.pc = 0x4AE;
    16
}
/// $03DF64: move.b #$0, $68(a0)
pub fn i_03df64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF6A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03DF6A: move.b #$0, $6e(a0)
pub fn i_03df6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF70;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x6E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03DF70: move.w #$182, d1
pub fn i_03df70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF74;
    cpu.ir = 0x323C;
    let v1: u32 = 0x182;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03DF74: jsr $29f2.l
pub fn i_03df74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DF7A);
    cpu.pc = 0x29F2;
    16
}
/// $03DF7A: move.w $14(a6), d1
pub fn i_03df7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF7E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03DF7E: jsr $2c66.l
pub fn i_03df7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DF84);
    cpu.pc = 0x2C66;
    16
}
/// $03DF84: move.w #$0, $76(a6)
pub fn i_03df84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF8A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DF8A: move.b #$0, $68(a6)
pub fn i_03df8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF90;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03DF90: move.b #$0, $6e(a6)
pub fn i_03df90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03DF96: move.w #$30, $22(a6)
pub fn i_03df96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DF9C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DF9C: move.w #$190, $24(a6)
pub fn i_03df9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DFA2: clr.w $26(a6)
pub fn i_03dfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFA6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03DFA6: move.w #$0, $72(a6)
pub fn i_03dfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFAC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03DFAC: move.w $106f4c.l, $74(a6)
pub fn i_03dfac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFB4;
    cpu.ir = 0x3D79;
    let v2 = bus.read16(0x106F4C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad3, v2 as u16);
    24
}
/// $03DFB4: lea.l $3ecac(pc), a1
pub fn i_03dfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3ECAC;
    16
}
/// $03DFB8: jsr $4ae.l
pub fn i_03dfb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DFBE);
    cpu.pc = 0x4AE;
    16
}
/// $03DFBE: move.w $72(a6), $72(a0)
pub fn i_03dfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFC4;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03DFC4: movea.w #$7149, a1
pub fn i_03dfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFC8;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7149;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03DFC8: move.b #$4, d1
pub fn i_03dfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFCC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03DFCC: lea.l $282a3e.l, a2
pub fn i_03dfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFD2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A3E;
    20
}
/// $03DFD2: jsr $4784c.l
pub fn i_03dfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3DFD8);
    cpu.pc = 0x4784C;
    16
}
/// $03DFD8: bra.w $3e05e
pub fn i_03dfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3DFDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x3E05E;
    10
}
/// $03E05E: move.w $74(a6), $5c(a6)
pub fn i_03e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E064;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E064: move.w #$e000, $38(a6)
pub fn i_03e064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E06A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E06A: lea.l $282966.l, a0
pub fn i_03e06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E070;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x282966;
    20
}
/// $03E070: jsr $28cd4.l
pub fn i_03e070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E076;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E076);
    cpu.pc = 0x28CD4;
    16
}
/// $03E076: move.w #$5, $70(a6)
pub fn i_03e076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E07C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_03e07c  ($03E07C, 8 B, kernel/task_handlers.c)
/// $03E07C: lea.l $3e084(pc), a1
pub fn i_03e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E080;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E084;
    16
}
/// $03E080: move.l a1, (a6)
pub fn i_03e080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E082;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E082: rts
pub fn i_03e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E084;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColPhaseScore_03e084  ($03E084, 76 B, pow/results_squadleader_03daa8.s)
/// $03E084: movea.l $c(a6), a1
pub fn i_03e084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E088;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E088: cmpi.b #$1, $8a(a1)
pub fn i_03e088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E08E;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03E08E: bne.w $3e0a2
pub fn i_03e08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E092;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E0A2; return 10; }
    8
}
/// $03E092: cmpi.w #$1, $70(a6)
pub fn i_03e092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E098;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03E098: ble.w $3e0a2
pub fn i_03e098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E09C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x3E0A2; return 10; }
    8
}
/// $03E09C: move.w #$1, $70(a6)
pub fn i_03e09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E0A2: subq.w #$1, $70(a6)
pub fn i_03e0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0A6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E0A6: cmpi.w #$0, $70(a6)
pub fn i_03e0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0AC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E0AC: bge.w $3e0ce
pub fn i_03e0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0B0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x3E0CE; return 10; }
    8
}
/// $03E0B0: move.w #$0, d0
pub fn i_03e0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0B4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E0B4: move.w #$714c, d4
pub fn i_03e0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0B8;
    cpu.ir = 0x383C;
    let v1: u32 = 0x714C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E0B8: add.w $72(a6), d4
pub fn i_03e0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0BC;
    cpu.ir = 0xD86E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E0BC: jsr $4772a.l
pub fn i_03e0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E0C2);
    cpu.pc = 0x4772A;
    16
}
/// $03E0C2: lea.l $3e0d0(pc), a1
pub fn i_03e0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E0D0;
    16
}
/// $03E0C6: move.l a1, (a6)
pub fn i_03e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E0C8: move.w #$1e, $70(a6)
pub fn i_03e0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0CE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E0CE: rts
pub fn i_03e0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColPhaseWait_03e0d0  ($03E0D0, 78 B, pow/results_squadleader_03daa8.s)
/// $03E0D0: movea.l $c(a6), a1
pub fn i_03e0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0D4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E0D4: cmpi.b #$1, $8a(a1)
pub fn i_03e0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0DA;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03E0DA: bne.w $3e0ee
pub fn i_03e0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E0EE; return 10; }
    8
}
/// $03E0DE: cmpi.w #$1, $70(a6)
pub fn i_03e0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0E4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03E0E4: ble.w $3e0ee
pub fn i_03e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0E8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x3E0EE; return 10; }
    8
}
/// $03E0E8: move.w #$1, $70(a6)
pub fn i_03e0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E0EE: subq.w #$1, $70(a6)
pub fn i_03e0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0F2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E0F2: cmpi.w #$0, $70(a6)
pub fn i_03e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0F8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E0F8: bge.w $3e176
pub fn i_03e0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E0FC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x3E176; return 10; }
    8
}
/// $03E0FC: cmpi.w #$0, $5c(a6)
pub fn i_03e0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E102;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E102: bne.w $3e126
pub fn i_03e102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E106;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E126; return 10; }
    8
}
/// $03E106: clr.w $5c(a6)
pub fn i_03e106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E10A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E10A: movea.l $c(a6), a1
pub fn i_03e10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E10E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E10E: subq.b #$1, $88(a1)
pub fn i_03e10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E112;
    cpu.ir = 0x5329;
    let ad1 = cpu.a[1].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03E112: move.w #$1e, $70(a6)
pub fn i_03e112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E118;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E118: lea.l $3e17e(pc), a1
pub fn i_03e118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E11C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E17E;
    16
}
/// $03E11C: move.l a1, (a6)
pub fn i_03e11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E11E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03e11e  ($03E11E, 8 B, thunks/jsr_abs_thunks.c)
/// $03E11E: jsr $28d70.l
pub fn i_03e11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E124);
    cpu.pc = 0x28D70;
    16
}
/// $03E124: rts
pub fn i_03e124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E126;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColCommitScore_03e126  ($03E126, 80 B, pow/results_squadleader_03daa8.s)
/// $03E126: subq.w #$1, $5c(a6)
pub fn i_03e126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E12A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E12A: cmpi.w #$0, $76(a6)
pub fn i_03e12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E130;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E130: bne.w $3e140
pub fn i_03e130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E134;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E140; return 10; }
    8
}
/// $03E134: move.w $5c(a6), $106f4c.l
pub fn i_03e134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E13C;
    cpu.ir = 0x33EE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F4C, v3 as u16);
    24
}
/// $03E13C: bra.w $3e148
pub fn i_03e13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E140;
    cpu.ir = 0x6000;
    cpu.pc = 0x3E148;
    10
}
/// $03E140: move.w $5c(a6), $106f4e.l
pub fn i_03e140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E148;
    cpu.ir = 0x33EE;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F4E, v3 as u16);
    24
}
/// $03E148: lea.l $3e91c(pc), a1
pub fn i_03e148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E14C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E91C;
    16
}
/// $03E14C: jsr $4ae.l
pub fn i_03e14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E152);
    cpu.pc = 0x4AE;
    16
}
/// $03E152: move.w $76(a6), $76(a0)
pub fn i_03e152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E158;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E158: move.w $72(a6), $72(a0)
pub fn i_03e158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E15E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E15E: move.w $74(a6), d0
pub fn i_03e15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E162;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E162: sub.w $5c(a6), d0
pub fn i_03e162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E166;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E166: move.w d0, $74(a0)
pub fn i_03e166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E16A;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E16A: move.w $5c(a6), $5c(a0)
pub fn i_03e16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E170;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E170: move.w #$a, $70(a6)
pub fn i_03e170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E176;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_03e176  ($03E176, 8 B, thunks/jsr_abs_thunks.c)
/// $03E176: jsr $28d70.l
pub fn i_03e176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E17C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E17C);
    cpu.pc = 0x28D70;
    16
}
/// $03E17C: rts
pub fn i_03e17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E17E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColWaitOther_03e17e  ($03E17E, 26 B, pow/results_squadleader_03daa8.s)
/// $03E17E: movea.l $c(a6), a1
pub fn i_03e17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E182;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E182: cmpi.b #$0, $88(a1)
pub fn i_03e182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E188;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03E188: bne.w $3e198
pub fn i_03e188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E18C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E198; return 10; }
    8
}
/// $03E18C: move.w #$14, $70(a6)
pub fn i_03e18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E192;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E192: lea.l $3e1a0(pc), a1
pub fn i_03e192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E196;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E1A0;
    16
}
/// $03E196: move.l a1, (a6)
pub fn i_03e196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E198;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03e198  ($03E198, 8 B, thunks/jsr_abs_thunks.c)
/// $03E198: jsr $28d70.l
pub fn i_03e198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E19E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E19E);
    cpu.pc = 0x28D70;
    16
}
/// $03E19E: rts
pub fn i_03e19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColPhaseBonus_03e1a0  ($03E1A0, 96 B, pow/results_squadleader_03daa8.s)
/// $03E1A0: movea.l $c(a6), a1
pub fn i_03e1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1A4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E1A4: cmpi.b #$1, $8a(a1)
pub fn i_03e1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1AA;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03E1AA: bne.w $3e1be
pub fn i_03e1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E1BE; return 10; }
    8
}
/// $03E1AE: cmpi.w #$1, $70(a6)
pub fn i_03e1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03E1B4: ble.w $3e1be
pub fn i_03e1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1B8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x3E1BE; return 10; }
    8
}
/// $03E1B8: move.w #$1, $70(a6)
pub fn i_03e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E1BE: subq.w #$1, $70(a6)
pub fn i_03e1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1C2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E1C2: cmpi.w #$0, $70(a6)
pub fn i_03e1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1C8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E1C8: bge.w $3e200
pub fn i_03e1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1CC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x3E200; return 10; }
    8
}
/// $03E1CC: movea.w #$71cd, a1
pub fn i_03e1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1D0;
    cpu.ir = 0x327C;
    let v1: u32 = 0x71CD;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03E1D0: adda.w $72(a6), a1
pub fn i_03e1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1D4;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03E1D4: move.w #$2300, d0
pub fn i_03e1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E1D8: lea.l $282a96.l, a2
pub fn i_03e1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A96;
    20
}
/// $03E1DE: jsr $5dad8.l
pub fn i_03e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E1E4);
    cpu.pc = 0x5DAD8;
    16
}
/// $03E1E4: clr.w $5c(a6)
pub fn i_03e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1E8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E1E8: move.w #$0, d0
pub fn i_03e1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E1EC: move.w #$71cf, d4
pub fn i_03e1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1F0;
    cpu.ir = 0x383C;
    let v1: u32 = 0x71CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E1F0: add.w $72(a6), d4
pub fn i_03e1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1F4;
    cpu.ir = 0xD86E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E1F4: jsr $4772a.l
pub fn i_03e1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E1FA);
    cpu.pc = 0x4772A;
    16
}
/// $03E1FA: lea.l $3e208(pc), a1
pub fn i_03e1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E1FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E208;
    16
}
/// $03E1FE: move.l a1, (a6)
pub fn i_03e1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E200;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03e200  ($03E200, 8 B, thunks/jsr_abs_thunks.c)
/// $03E200: jsr $28d70.l
pub fn i_03e200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E206;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E206);
    cpu.pc = 0x28D70;
    16
}
/// $03E206: rts
pub fn i_03e206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E208;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColPhaseTotal_03e208  ($03E208, 164 B, pow/results_squadleader_03daa8.s)
/// $03E208: movea.l $c(a6), a1
pub fn i_03e208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E20C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E20C: cmpi.b #$1, $8a(a1)
pub fn i_03e20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E212;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03E212: bne.w $3e226
pub fn i_03e212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E216;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E226; return 10; }
    8
}
/// $03E216: cmpi.w #$1, $70(a6)
pub fn i_03e216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E21C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $03E21C: ble.w $3e226
pub fn i_03e21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E220;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x3E226; return 10; }
    8
}
/// $03E220: move.w #$1, $70(a6)
pub fn i_03e220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E226;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E226: subq.w #$1, $70(a6)
pub fn i_03e226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E22A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E22A: cmpi.w #$0, $70(a6)
pub fn i_03e22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E230;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E230: bge.w $3e2ac
pub fn i_03e230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E234;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x3E2AC; return 10; }
    8
}
/// $03E234: move.w #$3, $70(a6)
pub fn i_03e234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E23A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E23A: tst.w $5c(a6)
pub fn i_03e23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E23E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $03E23E: beq.w $3e28e
pub fn i_03e23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E242;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3E28E; return 10; }
    8
}
/// $03E242: move.w #$10d8, d0
pub fn i_03e242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E246;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E246: jsr $2352.l
pub fn i_03e246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E24C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E24C);
    cpu.pc = 0x2352;
    16
}
/// $03E24C: move.l #$1000, d0
pub fn i_03e24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E252;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03E252: jsr $51a44.l
pub fn i_03e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E258;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E258);
    cpu.pc = 0x51A44;
    16
}
/// $03E258: move.w #$0, d0
pub fn i_03e258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E25C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E25C: move.w #$714f, d4
pub fn i_03e25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E260;
    cpu.ir = 0x383C;
    let v1: u32 = 0x714F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E260: add.w $72(a6), d4
pub fn i_03e260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E264;
    cpu.ir = 0xD86E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E264: jsr $4772a.l
pub fn i_03e264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E26A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E26A);
    cpu.pc = 0x4772A;
    16
}
/// $03E26A: move.w #$0, d0
pub fn i_03e26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E26E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E26E: move.w #$71cf, d4
pub fn i_03e26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E272;
    cpu.ir = 0x383C;
    let v1: u32 = 0x71CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E272: add.w $72(a6), d4
pub fn i_03e272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E276;
    cpu.ir = 0xD86E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E276: jsr $4768a.l
pub fn i_03e276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E27C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E27C);
    cpu.pc = 0x4768A;
    16
}
/// $03E27C: move.w $5c(a6), d0
pub fn i_03e27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E280;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E280: move.w #$710f, d4
pub fn i_03e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E284;
    cpu.ir = 0x383C;
    let v1: u32 = 0x710F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E284: add.w $72(a6), d4
pub fn i_03e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E288;
    cpu.ir = 0xD86E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E288: jsr $4772a.l
pub fn i_03e288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E28E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E28E);
    cpu.pc = 0x4772A;
    16
}
/// $03E28E: addq.w #$1, $5c(a6)
pub fn i_03e28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E292;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E292: move.w $74(a6), d0
pub fn i_03e292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E296;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E296: cmp.w $5c(a6), d0
pub fn i_03e296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E29A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03E29A: bcc.w $3e2ac
pub fn i_03e29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E29E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3E2AC; return 10; }
    8
}
/// $03E29E: movea.l $c(a6), a1
pub fn i_03e29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2A2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E2A2: subq.b #$1, $8e(a1)
pub fn i_03e2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2A6;
    cpu.ir = 0x5329;
    let ad1 = cpu.a[1].wrapping_add(0x8E);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $03E2A6: lea.l $3e2b4(pc), a1
pub fn i_03e2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E2B4;
    16
}
/// $03E2AA: move.l a1, (a6)
pub fn i_03e2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03e2ac  ($03E2AC, 8 B, thunks/jsr_abs_thunks.c)
/// $03E2AC: jsr $28d70.l
pub fn i_03e2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E2B2);
    cpu.pc = 0x28D70;
    16
}
/// $03E2B2: rts
pub fn i_03e2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColPause_03e2b4  ($03E2B4, 32 B, pow/results_squadleader_03daa8.s)
/// $03E2B4: move.w #$a, $70(a6)
pub fn i_03e2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E2BA: lea.l $3e2c0(pc), a1
pub fn i_03e2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E2C0;
    16
}
/// $03E2BE: move.l a1, (a6)
pub fn i_03e2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E2C0: subq.w #$1, $70(a6)
pub fn i_03e2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2C4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E2C4: cmpi.w #$0, $70(a6)
pub fn i_03e2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2CA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E2CA: bge.w $3e2d4
pub fn i_03e2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2CE;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x3E2D4; return 10; }
    8
}
/// $03E2CE: lea.l $3e2dc(pc), a1
pub fn i_03e2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E2DC;
    16
}
/// $03E2D2: move.l a1, (a6)
pub fn i_03e2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03e2d4  ($03E2D4, 8 B, thunks/jsr_abs_thunks.c)
/// $03E2D4: jsr $28d70.l
pub fn i_03e2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E2DA);
    cpu.pc = 0x28D70;
    16
}
/// $03E2DA: rts
pub fn i_03e2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColWaitAll_03e2dc  ($03E2DC, 26 B, pow/results_squadleader_03daa8.s)
/// $03E2DC: movea.l $c(a6), a1
pub fn i_03e2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2E0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E2E0: cmpi.b #$0, $8e(a1)
pub fn i_03e2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2E6;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03E2E6: bne.w $3e2f6
pub fn i_03e2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E2F6; return 10; }
    8
}
/// $03E2EA: move.w #$1e, $70(a6)
pub fn i_03e2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E2F0: lea.l $3e2fe(pc), a1
pub fn i_03e2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E2FE;
    16
}
/// $03E2F4: move.l a1, (a6)
pub fn i_03e2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03e2f6  ($03E2F6, 8 B, thunks/jsr_abs_thunks.c)
/// $03E2F6: jsr $28d70.l
pub fn i_03e2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E2FC);
    cpu.pc = 0x28D70;
    16
}
/// $03E2FC: rts
pub fn i_03e2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E2FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColWinner_03e2fe  ($03E2FE, 260 B, pow/results_squadleader_03daa8.s)
/// $03E2FE: move.w $76(a6), d0
pub fn i_03e2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E302;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E302: jsr $2ad36.l
pub fn i_03e302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E308);
    cpu.pc = 0x2AD36;
    16
}
/// $03E308: bcc.w $3e3dc
pub fn i_03e308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E30C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3E3DC; return 10; }
    8
}
/// $03E30C: move.l #$10000, d0
pub fn i_03e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E312;
    cpu.ir = 0x203C;
    let v1: u32 = 0x10000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03E312: jsr $51a44.l
pub fn i_03e312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E318);
    cpu.pc = 0x51A44;
    16
}
/// $03E318: move.w #$10d8, d0
pub fn i_03e318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E31C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E31C: jsr $2352.l
pub fn i_03e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E322);
    cpu.pc = 0x2352;
    16
}
/// $03E322: jsr $2ad70.l
pub fn i_03e322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E328);
    cpu.pc = 0x2AD70;
    16
}
/// $03E328: move.w $74(a6), d0
pub fn i_03e328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E32C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E32C: addi.w #$a, d0
pub fn i_03e32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E330;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E330: move.w #$710f, d4
pub fn i_03e330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E334;
    cpu.ir = 0x383C;
    let v1: u32 = 0x710F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E334: add.w $72(a6), d4
pub fn i_03e334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E338;
    cpu.ir = 0xD86E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E338: jsr $4772a.l
pub fn i_03e338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E33E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E33E);
    cpu.pc = 0x4772A;
    16
}
/// $03E33E: cmpi.w #$0, $74(a6)
pub fn i_03e33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E344;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E344: bne.w $3e3dc
pub fn i_03e344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E348;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E3DC; return 10; }
    8
}
/// $03E348: move.w #$718f, d0
pub fn i_03e348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E34C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x718F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E34C: add.w $72(a6), d0
pub fn i_03e34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E350;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E350: move.w #$0, d1
pub fn i_03e350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E354;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E354: lsl.w #$1, d1
pub fn i_03e354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E356;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03E356: addi.w #$4b60, d1
pub fn i_03e356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E35A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4B60, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E35A: movem.w d0-d1, $3c0000.l
pub fn i_03e35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E362;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E362: addi.w #$20, d0
pub fn i_03e362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E366;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E366: addq.w #$1, d1
pub fn i_03e366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E368;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E368: movem.w d0-d1, $3c0000.l
pub fn i_03e368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E370;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E370: subi.w #$1f, d0
pub fn i_03e370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E374;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E374: addi.w #$f, d1
pub fn i_03e374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E378;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E378: movem.w d0-d1, $3c0000.l
pub fn i_03e378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E380;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E380: addi.w #$20, d0
pub fn i_03e380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E384;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E384: addq.w #$1, d1
pub fn i_03e384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E386;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E386: movem.w d0-d1, $3c0000.l
pub fn i_03e386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E38E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E38E: subi.w #$21, d0
pub fn i_03e38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E392;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E392: move.w #$71cf, d0
pub fn i_03e392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E396;
    cpu.ir = 0x303C;
    let v1: u32 = 0x71CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E396: add.w $72(a6), d0
pub fn i_03e396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E39A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E39A: move.w #$0, d1
pub fn i_03e39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E39E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E39E: lsl.w #$1, d1
pub fn i_03e39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3A0;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03E3A0: addi.w #$4b60, d1
pub fn i_03e3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3A4;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4B60, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E3A4: movem.w d0-d1, $3c0000.l
pub fn i_03e3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3AC;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E3AC: addi.w #$20, d0
pub fn i_03e3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3B0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E3B0: addq.w #$1, d1
pub fn i_03e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3B2;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E3B2: movem.w d0-d1, $3c0000.l
pub fn i_03e3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3BA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E3BA: subi.w #$1f, d0
pub fn i_03e3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3BE;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E3BE: addi.w #$f, d1
pub fn i_03e3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3C2;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E3C2: movem.w d0-d1, $3c0000.l
pub fn i_03e3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3CA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E3CA: addi.w #$20, d0
pub fn i_03e3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3CE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E3CE: addq.w #$1, d1
pub fn i_03e3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3D0;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E3D0: movem.w d0-d1, $3c0000.l
pub fn i_03e3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3D8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E3D8: subi.w #$21, d0
pub fn i_03e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3DC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E3DC: lea.l $3e3e2(pc), a1
pub fn i_03e3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E3E2;
    16
}
/// $03E3E0: move.l a1, (a6)
pub fn i_03e3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E3E2: subq.w #$1, $70(a6)
pub fn i_03e3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3E6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E3E6: cmpi.w #$0, $70(a6)
pub fn i_03e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E3EC: bge.w $3e402
pub fn i_03e3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3F0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x3E402; return 10; }
    8
}
/// $03E3F0: lea.l $3e40a(pc), a1
pub fn i_03e3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E40A;
    16
}
/// $03E3F4: move.l a1, (a6)
pub fn i_03e3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E3F6: move.w #$1e, $70(a6)
pub fn i_03e3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E3FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E3FC: jsr $2a252.l
pub fn i_03e3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E402;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E402);
    cpu.pc = 0x2A252;
    16
}
// ===== JsrAbsThunk_03e402  ($03E402, 8 B, thunks/jsr_abs_thunks.c)
/// $03E402: jsr $28d70.l
pub fn i_03e402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E408);
    cpu.pc = 0x28D70;
    16
}
/// $03E408: rts
pub fn i_03e408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E40A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColSpawnPrize_03e40a  ($03E40A, 38 B, pow/results_squadleader_03daa8.s)
/// $03E40A: cmpi.w #$0, $74(a6)
pub fn i_03e40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E410;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E410: bne.w $3e438
pub fn i_03e410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E414;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E438; return 10; }
    8
}
/// $03E414: lea.l $3eb84(pc), a1
pub fn i_03e414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E418;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EB84;
    16
}
/// $03E418: jsr $4ae.l
pub fn i_03e418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E41E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E41E);
    cpu.pc = 0x4AE;
    16
}
/// $03E41E: move.w $72(a6), $72(a0)
pub fn i_03e41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E424;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E424: move.w #$3c, $70(a6)
pub fn i_03e424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E42A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E42A: jsr $28d70.l
pub fn i_03e42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E430);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_03e430  ($03E430, 8 B, kernel/task_handlers.c)
/// $03E430: lea.l $3e4e6(pc), a1
pub fn i_03e430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E434;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E4E6;
    16
}
/// $03E434: move.l a1, (a6)
pub fn i_03e434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E436;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E436: rts
pub fn i_03e436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E438;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColSpawnPrizeTie_03e438  ($03E438, 58 B, pow/results_squadleader_03daa8.s)
/// $03E438: movea.l $c(a6), a1
pub fn i_03e438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E43C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E43C: cmpi.b #$2, $89(a1)
pub fn i_03e43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E442;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $03E442: bne.w $3e494
pub fn i_03e442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E446;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E494; return 10; }
    8
}
/// $03E446: movea.l $c(a6), a1
pub fn i_03e446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E44A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E44A: move.w $8c(a1), d0
pub fn i_03e44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E44E;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x8C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E44E: cmpi.w #$2, d0
pub fn i_03e44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E452;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $03E452: bne.w $3e47a
pub fn i_03e452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E456;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E47A; return 10; }
    8
}
/// $03E456: lea.l $3ec18(pc), a1
pub fn i_03e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E45A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EC18;
    16
}
/// $03E45A: jsr $4ae.l
pub fn i_03e45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E460);
    cpu.pc = 0x4AE;
    16
}
/// $03E460: move.w $72(a6), $72(a0)
pub fn i_03e460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E466;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E466: move.w #$3c, $70(a6)
pub fn i_03e466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E46C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E46C: jsr $28d70.l
pub fn i_03e46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E472);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_03e472  ($03E472, 8 B, kernel/task_handlers.c)
/// $03E472: lea.l $3e4e6(pc), a1
pub fn i_03e472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E476;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E4E6;
    16
}
/// $03E476: move.l a1, (a6)
pub fn i_03e476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E478;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E478: rts
pub fn i_03e478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E47A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColAnim_03e47a  ($03E47A, 14 B, pow/results_squadleader_03daa8.s)
/// $03E47A: cmp.w $76(a6), d0
pub fn i_03e47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E47E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $03E47E: beq.w $3e490
pub fn i_03e47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E482;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3E490; return 10; }
    8
}
/// $03E482: jsr $28d70.l
pub fn i_03e482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E488);
    cpu.pc = 0x28D70;
    16
}
// ===== SetTaskHandler_03e488  ($03E488, 8 B, kernel/task_handlers.c)
/// $03E488: lea.l $3e50c(pc), a1
pub fn i_03e488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E48C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E50C;
    16
}
/// $03E48C: move.l a1, (a6)
pub fn i_03e48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E48E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E48E: rts
pub fn i_03e48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E490;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColRts_03e490  ($03E490, 4 B, pow/results_squadleader_03daa8.s)
/// $03E490: bra.w $3e4b2
pub fn i_03e490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E494;
    cpu.ir = 0x6000;
    cpu.pc = 0x3E4B2;
    10
}
// ===== Results_ColAnimWait_03e494  ($03E494, 22 B, pow/results_squadleader_03daa8.s)
/// $03E494: cmpi.w #$a, $74(a6)
pub fn i_03e494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E49A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Word, false);
    16
}
/// $03E49A: bcc.w $3e4b2
pub fn i_03e49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E49E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3E4B2; return 10; }
    8
}
/// $03E49E: jsr $28d70.l
pub fn i_03e49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E4A4);
    cpu.pc = 0x28D70;
    16
}
/// $03E4A4: move.w #$3c, $70(a6)
pub fn i_03e4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetTaskHandler_03e4aa  ($03E4AA, 8 B, kernel/task_handlers.c)
/// $03E4AA: lea.l $3e4e6(pc), a1
pub fn i_03e4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E4E6;
    16
}
/// $03E4AE: move.l a1, (a6)
pub fn i_03e4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E4B0: rts
pub fn i_03e4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ColSpawnFinal_03e4b2  ($03E4B2, 52 B, pow/results_squadleader_03daa8.s)
/// $03E4B2: lea.l $3ea2e(pc), a1
pub fn i_03e4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EA2E;
    16
}
/// $03E4B6: jsr $4ae.l
pub fn i_03e4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E4BC);
    cpu.pc = 0x4AE;
    16
}
/// $03E4BC: move.w $72(a6), $72(a0)
pub fn i_03e4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4C2;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E4C2: move.w $68(a6), $68(a0)
pub fn i_03e4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4C8;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E4C8: move.w $6e(a6), $6e(a0)
pub fn i_03e4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4CE;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x6E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E4CE: move.w $74(a6), $74(a0)
pub fn i_03e4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4D4;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E4D4: move.w $76(a6), $76(a0)
pub fn i_03e4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4DA;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $03E4DA: move.w #$64, $70(a6)
pub fn i_03e4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x64;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E4E0: lea.l $3e4e6(pc), a1
pub fn i_03e4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E4E6;
    16
}
/// $03E4E4: move.l a1, (a6)
pub fn i_03e4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== Results_ColFinish_03e4e6  ($03E4E6, 30 B, pow/results_squadleader_03daa8.s)
/// $03E4E6: subq.w #$1, $70(a6)
pub fn i_03e4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4EA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E4EA: cmpi.w #$0, $70(a6)
pub fn i_03e4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4F0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03E4F0: bgt.w $3e504
pub fn i_03e4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4F4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3E504; return 10; }
    8
}
/// $03E4F4: movea.l $c(a6), a1
pub fn i_03e4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4F8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E4F8: move.b #$1, $90(a1)
pub fn i_03e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E4FE;
    cpu.ir = 0x137C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03E4FE: lea.l $3e50c(pc), a1
pub fn i_03e4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E502;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E50C;
    16
}
/// $03E502: move.l a1, (a6)
pub fn i_03e502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E504;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03e504  ($03E504, 8 B, thunks/jsr_abs_thunks.c)
/// $03E504: jsr $28d70.l
pub fn i_03e504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E50A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E50A);
    cpu.pc = 0x28D70;
    16
}
/// $03E50A: rts
pub fn i_03e50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E50C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_03e50c  ($03E50C, 8 B, thunks/jsr_abs_thunks.c)
/// $03E50C: jsr $28d70.l
pub fn i_03e50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E512);
    cpu.pc = 0x28D70;
    16
}
/// $03E512: rts
pub fn i_03e512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_PowRoster_03e514  ($03E514, 448 B, pow/results_squadleader_03daa8.s)
/// $03E514: clr.l $70(a6)
pub fn i_03e514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E518;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E518: clr.l $74(a6)
pub fn i_03e518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E51C;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E51C: clr.l $78(a6)
pub fn i_03e51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E520;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E520: clr.l $7c(a6)
pub fn i_03e520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E524;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E524: clr.l $80(a6)
pub fn i_03e524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E528;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E528: clr.l $84(a6)
pub fn i_03e528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E52C;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E52C: clr.l $88(a6)
pub fn i_03e52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E530;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E530: clr.l $8c(a6)
pub fn i_03e530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E534;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E534: clr.l $90(a6)
pub fn i_03e534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E538;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E538: clr.l $94(a6)
pub fn i_03e538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E53C;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E53C: clr.l $98(a6)
pub fn i_03e53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E540;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E540: clr.l $9c(a6)
pub fn i_03e540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E544;
    cpu.ir = 0x42AE;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $03E544: clr.w $5c(a6)
pub fn i_03e544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E548;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E548: clr.b $5e(a6)
pub fn i_03e548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E54C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x5E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03E54C: lea.l $3e552(pc), a1
pub fn i_03e54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E550;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E552;
    16
}
/// $03E550: move.l a1, (a6)
pub fn i_03e550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E552;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03E552: movea.l $c(a6), a1
pub fn i_03e552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E556;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E556: cmpi.b #$0, $20(a1)
pub fn i_03e556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E55C;
    cpu.ir = 0xC29;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03E55C: beq.w $3e6d8
pub fn i_03e55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E560;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3E6D8; return 10; }
    8
}
/// $03E560: clr.b $20(a1)
pub fn i_03e560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E564;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03E564: cmpi.b #$0, $5e(a6)
pub fn i_03e564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E56A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03E56A: beq.w $3e5ec
pub fn i_03e56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E56E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3E5EC; return 10; }
    8
}
/// $03E56E: move.l $74(a6), $70(a6)
pub fn i_03e56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E574;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03E574: move.l $78(a6), $74(a6)
pub fn i_03e574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E57A;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03E57A: move.l $7c(a6), $78(a6)
pub fn i_03e57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E580;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03E580: move.l $80(a6), $7c(a6)
pub fn i_03e580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E586;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03E586: move.l $84(a6), $80(a6)
pub fn i_03e586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E58C;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03E58C: move.l $88(a6), $84(a6)
pub fn i_03e58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E592;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03E592: move.l $8c(a6), $88(a6)
pub fn i_03e592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E598;
    cpu.ir = 0x2D6E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x88);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03E598: move.b $91(a6), $90(a6)
pub fn i_03e598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E59E;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E59E: move.b $92(a6), $91(a6)
pub fn i_03e59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5A4;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x91);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5A4: move.b $93(a6), $92(a6)
pub fn i_03e5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5AA;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5AA: move.b $94(a6), $93(a6)
pub fn i_03e5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5B0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5B0: move.b $95(a6), $94(a6)
pub fn i_03e5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5B6;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x95);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x94);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5B6: move.b $96(a6), $95(a6)
pub fn i_03e5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5BC;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x95);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5BC: move.b $97(a6), $96(a6)
pub fn i_03e5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5C2;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5C2: move.b $99(a6), $98(a6)
pub fn i_03e5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5C8;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5C8: move.b $9a(a6), $99(a6)
pub fn i_03e5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5CE;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5CE: move.b $9b(a6), $9a(a6)
pub fn i_03e5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5D4;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5D4: move.b $9c(a6), $9b(a6)
pub fn i_03e5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5DA;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5DA: move.b $9d(a6), $9c(a6)
pub fn i_03e5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5E0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5E0: move.b $9e(a6), $9d(a6)
pub fn i_03e5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5E6;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9D);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5E6: move.b $9f(a6), $9e(a6)
pub fn i_03e5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5EC;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $03E5EC: cmpi.b #$0, $10fd83.l
pub fn i_03e5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5F4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $03E5F4: bne.w $3e600
pub fn i_03e5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E600; return 10; }
    8
}
/// $03E5F8: jsr $3e6da(pc)
pub fn i_03e5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E5FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E5FC);
    cpu.pc = 0x3E6DA;
    16
}
/// $03E5FC: bra.w $3e604
pub fn i_03e5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E600;
    cpu.ir = 0x6000;
    cpu.pc = 0x3E604;
    10
}
/// $03E600: jsr $3e740(pc)
pub fn i_03e600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E604;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E604);
    cpu.pc = 0x3E740;
    16
}
/// $03E604: addq.w #$1, $5c(a6)
pub fn i_03e604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E608;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03E608: cmpi.w #$6, $5c(a6)
pub fn i_03e608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E60E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Word, false);
    16
}
/// $03E60E: bls.w $3e61e
pub fn i_03e60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E612;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x3E61E; return 10; }
    8
}
/// $03E612: move.b #$ff, $5e(a6)
pub fn i_03e612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E618;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03E618: move.w #$6, $5c(a6)
pub fn i_03e618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E61E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03E61E: cmpi.b #$0, $68(a6)
pub fn i_03e61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E624;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03E624: bne.w $3e62c
pub fn i_03e624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E628;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E62C; return 10; }
    8
}
/// $03E628: movea.w #$7094, a1
pub fn i_03e628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E62C;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7094;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03E62C: cmpi.b #$1, $68(a6)
pub fn i_03e62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E632;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03E632: bne.w $3e63a
pub fn i_03e632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E636;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E63A; return 10; }
    8
}
/// $03E636: movea.w #$72b4, a1
pub fn i_03e636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E63A;
    cpu.ir = 0x327C;
    let v1: u32 = 0x72B4;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03E63A: move.w #$2320, d0
pub fn i_03e63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E63E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E63E: move.w #$f, d1
pub fn i_03e63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E642;
    cpu.ir = 0x323C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E642: move.w #$7, d2
pub fn i_03e642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E646;
    cpu.ir = 0x343C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E646: jsr $5da9c.l
pub fn i_03e646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E64C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E64C);
    cpu.pc = 0x5DA9C;
    16
}
/// $03E64C: movea.l $70(a6), a1
pub fn i_03e64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E650;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E650: move.b $90(a6), d2
pub fn i_03e650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E654;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E654: move.b $98(a6), d3
pub fn i_03e654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E658;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E658: move.w #$7094, d0
pub fn i_03e658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E65C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7094;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E65C: jsr $3e7a6(pc)
pub fn i_03e65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E660;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E660);
    cpu.pc = 0x3E7A6;
    16
}
/// $03E660: movea.l $74(a6), a1
pub fn i_03e660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E664;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E664: move.b $91(a6), d2
pub fn i_03e664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E668;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E668: move.b $99(a6), d3
pub fn i_03e668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E66C;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E66C: move.w #$7095, d0
pub fn i_03e66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E670;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7095;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E670: jsr $3e7a6(pc)
pub fn i_03e670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E674;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E674);
    cpu.pc = 0x3E7A6;
    16
}
/// $03E674: movea.l $78(a6), a1
pub fn i_03e674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E678;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E678: move.b $92(a6), d2
pub fn i_03e678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E67C;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E67C: move.b $9a(a6), d3
pub fn i_03e67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E680;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E680: move.w #$7096, d0
pub fn i_03e680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E684;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7096;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E684: jsr $3e7a6(pc)
pub fn i_03e684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E688;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E688);
    cpu.pc = 0x3E7A6;
    16
}
/// $03E688: movea.l $7c(a6), a1
pub fn i_03e688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E68C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E68C: move.b $93(a6), d2
pub fn i_03e68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E690;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E690: move.b $9b(a6), d3
pub fn i_03e690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E694;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E694: move.w #$7097, d0
pub fn i_03e694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E698;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7097;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E698: jsr $3e7a6(pc)
pub fn i_03e698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E69C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E69C);
    cpu.pc = 0x3E7A6;
    16
}
/// $03E69C: movea.l $80(a6), a1
pub fn i_03e69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6A0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E6A0: move.b $94(a6), d2
pub fn i_03e6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6A4;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E6A4: move.b $9c(a6), d3
pub fn i_03e6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6A8;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E6A8: move.w #$7098, d0
pub fn i_03e6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6AC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7098;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E6AC: jsr $3e7a6(pc)
pub fn i_03e6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E6B0);
    cpu.pc = 0x3E7A6;
    16
}
/// $03E6B0: movea.l $84(a6), a1
pub fn i_03e6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6B4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E6B4: move.b $95(a6), d2
pub fn i_03e6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6B8;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x95);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E6B8: move.b $9d(a6), d3
pub fn i_03e6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6BC;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E6BC: move.w #$7099, d0
pub fn i_03e6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7099;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E6C0: jsr $3e7a6(pc)
pub fn i_03e6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E6C4);
    cpu.pc = 0x3E7A6;
    16
}
/// $03E6C4: movea.l $88(a6), a1
pub fn i_03e6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6C8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03E6C8: move.b $96(a6), d2
pub fn i_03e6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6CC;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x96);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E6CC: move.b $9e(a6), d3
pub fn i_03e6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6D0;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03E6D0: move.w #$709a, d0
pub fn i_03e6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x709A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrPcThunk_03e6d4  ($03E6D4, 6 B, thunks/jsr_pc_thunks.c)
/// $03E6D4: jsr $3e7a6(pc)
pub fn i_03e6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E6D8);
    cpu.pc = 0x3E7A6;
    16
}
/// $03E6D8: rts
pub fn i_03e6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_RosterPickA_03e6da  ($03E6DA, 102 B, pow/results_squadleader_03daa8.s)
/// $03E6DA: move.w #$40, d0
pub fn i_03e6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E6DE: jsr $5e9e4.l
pub fn i_03e6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E6E4);
    cpu.pc = 0x5E9E4;
    16
}
/// $03E6E4: move.w $5c(a6), d1
pub fn i_03e6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6E8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E6E8: addi.w #$98, d1
pub fn i_03e6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6EC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x98, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E6EC: move.b d0, (a6, d1.w)
pub fn i_03e6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6F0;
    cpu.ir = 0x1D80;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $03E6F0: move.w #$3b2, d0
pub fn i_03e6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6F4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3B2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E6F4: jsr $5e9e4.l
pub fn i_03e6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E6FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E6FA);
    cpu.pc = 0x5E9E4;
    16
}
/// $03E6FA: lea.l $282de0.l, a1
pub fn i_03e6fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E700;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x282DE0;
    20
}
/// $03E700: clr.l d2
pub fn i_03e700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E702;
    cpu.ir = 0x4282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = r2;
    4
}
/// $03E702: move.b (a1, d0.w), d2
pub fn i_03e702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E706;
    cpu.ir = 0x1431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $03E706: move.w $5c(a6), d1
pub fn i_03e706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E70A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E70A: addi.w #$90, d1
pub fn i_03e70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E70E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x90, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E70E: move.b d2, (a6, d1.w)
pub fn i_03e70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E712;
    cpu.ir = 0x1D82;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $03E712: move.l d2, d3
pub fn i_03e712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E714;
    cpu.ir = 0x2602;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03E714: subq.w #$1, d2
pub fn i_03e714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E716;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E716: lsl.w #$3, d2
pub fn i_03e716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E718;
    cpu.ir = 0xE74A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E718: lea.l $2831b2.l, a1
pub fn i_03e718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E71E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2831B2;
    20
}
/// $03E71E: movea.l (a1, d2.w), a2
pub fn i_03e71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E722;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $03E722: move.l $4(a1, d2.w), d0
pub fn i_03e722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E726;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $03E726: jsr $5e9e4.l
pub fn i_03e726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E72C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E72C);
    cpu.pc = 0x5E9E4;
    16
}
/// $03E72C: mulu.w d3, d0
pub fn i_03e72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E72E;
    cpu.ir = 0xC0C3;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $03E72E: adda.l d0, a2
pub fn i_03e72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E730;
    cpu.ir = 0xD5C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $03E730: move.w $5c(a6), d1
pub fn i_03e730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E734;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E734: lsl.w #$2, d1
pub fn i_03e734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E736;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $03E736: addi.w #$70, d1
pub fn i_03e736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E73A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x70, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E73A: move.l a2, (a6, d1.w)
pub fn i_03e73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E73E;
    cpu.ir = 0x2D8A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $03E73E: rts
pub fn i_03e73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E740;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_RosterPickB_03e740  ($03E740, 102 B, pow/results_squadleader_03daa8.s)
/// $03E740: move.w #$40, d0
pub fn i_03e740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E744;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E744: jsr $5e9e4.l
pub fn i_03e744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E74A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E74A);
    cpu.pc = 0x5E9E4;
    16
}
/// $03E74A: move.w $5c(a6), d1
pub fn i_03e74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E74E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E74E: addi.w #$98, d1
pub fn i_03e74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E752;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x98, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E752: move.b d0, (a6, d1.w)
pub fn i_03e752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E756;
    cpu.ir = 0x1D80;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $03E756: move.w #$14c, d0
pub fn i_03e756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E75A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x14C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E75A: jsr $5e9e4.l
pub fn i_03e75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E760);
    cpu.pc = 0x5E9E4;
    16
}
/// $03E760: lea.l $282c3c.l, a1
pub fn i_03e760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E766;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x282C3C;
    20
}
/// $03E766: clr.l d2
pub fn i_03e766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E768;
    cpu.ir = 0x4282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = r2;
    4
}
/// $03E768: move.b (a1, d0.w), d2
pub fn i_03e768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E76C;
    cpu.ir = 0x1431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $03E76C: move.w $5c(a6), d1
pub fn i_03e76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E770;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E770: addi.w #$90, d1
pub fn i_03e770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E774;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x90, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E774: move.b d2, (a6, d1.w)
pub fn i_03e774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E778;
    cpu.ir = 0x1D82;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $03E778: move.l d2, d3
pub fn i_03e778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E77A;
    cpu.ir = 0x2602;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03E77A: subq.w #$2, d2
pub fn i_03e77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E77C;
    cpu.ir = 0x5542;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E77C: lsl.w #$3, d2
pub fn i_03e77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E77E;
    cpu.ir = 0xE74A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03E77E: lea.l $282d88.l, a1
pub fn i_03e77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E784;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x282D88;
    20
}
/// $03E784: movea.l (a1, d2.w), a2
pub fn i_03e784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E788;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $03E788: move.l $4(a1, d2.w), d0
pub fn i_03e788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E78C;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $03E78C: jsr $5e9e4.l
pub fn i_03e78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E792;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E792);
    cpu.pc = 0x5E9E4;
    16
}
/// $03E792: mulu.w d3, d0
pub fn i_03e792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E794;
    cpu.ir = 0xC0C3;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $03E794: adda.l d0, a2
pub fn i_03e794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E796;
    cpu.ir = 0xD5C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $03E796: move.w $5c(a6), d1
pub fn i_03e796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E79A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03E79A: lsl.w #$2, d1
pub fn i_03e79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E79C;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $03E79C: addi.w #$70, d1
pub fn i_03e79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7A0;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x70, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E7A0: move.l a2, (a6, d1.w)
pub fn i_03e7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7A4;
    cpu.ir = 0x2D8A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $03E7A4: rts
pub fn i_03e7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_RosterDrawDispatch_03e7a6  ($03E7A6, 20 B, pow/results_squadleader_03daa8.s)
/// $03E7A6: cmpi.b #$0, $10fd83.l
pub fn i_03e7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7AE;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10FD83) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $03E7AE: bne.w $3e7ba
pub fn i_03e7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E7BA; return 10; }
    8
}
/// $03E7B2: jsr $3e7c0(pc)
pub fn i_03e7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E7B6);
    cpu.pc = 0x3E7C0;
    16
}
/// $03E7B6: bra.w $3e7be
pub fn i_03e7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x3E7BE;
    10
}
// ===== JsrPcThunk_03e7ba  ($03E7BA, 6 B, thunks/jsr_pc_thunks.c)
/// $03E7BA: jsr $3e84c(pc)
pub fn i_03e7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3E7BE);
    cpu.pc = 0x3E84C;
    16
}
/// $03E7BE: rts
pub fn i_03e7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_RosterDrawA_03e7c0  ($03E7C0, 132 B, pow/results_squadleader_03daa8.s)
/// $03E7C0: cmpa.l #$0, a1
pub fn i_03e7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7C6;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03E7C6: bne.w $3e7cc
pub fn i_03e7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E7CC; return 10; }
    8
}
/// $03E7CA: rts
pub fn i_03e7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03E7CC: lea.l $282a9c.l, a2
pub fn i_03e7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7D2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A9C;
    20
}
/// $03E7D2: clr.l d1
pub fn i_03e7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7D4;
    cpu.ir = 0x4281;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = r2;
    4
}
/// $03E7D4: move.b d2, d1
pub fn i_03e7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7D6;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03E7D6: lsl.w #$1, d1
pub fn i_03e7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7D8;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03E7D8: add.w (a2, d1.w), d0
pub fn i_03e7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7DC;
    cpu.ir = 0xD072;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $03E7DC: cmpi.b #$1, $68(a6)
pub fn i_03e7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7E2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03E7E2: bne.w $3e7ea
pub fn i_03e7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E7EA; return 10; }
    8
}
/// $03E7E6: addi.w #$220, d0
pub fn i_03e7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7EA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x220, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E7EA: clr.w d1
pub fn i_03e7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7EC;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E7EC: move.b (a1)+, d1
pub fn i_03e7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7EE;
    cpu.ir = 0x1219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    8
}
/// $03E7EE: cmpi.b #$80, d1
pub fn i_03e7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7F2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $03E7F2: bcc.w $3e7fe
pub fn i_03e7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3E7FE; return 10; }
    8
}
/// $03E7F6: ori.w #$2300, d1
pub fn i_03e7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7FA;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E7FA: bra.w $3e806
pub fn i_03e7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E7FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x3E806;
    10
}
/// $03E7FE: subi.w #$80, d1
pub fn i_03e7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E802;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E802: ori.w #$2a00, d1
pub fn i_03e802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E806;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2A00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E806: movem.w d0-d1, $3c0000.l
pub fn i_03e806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E80E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E80E: addi.w #$20, d0
pub fn i_03e80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E812;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E812: subq.b #$1, d2
pub fn i_03e812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E814;
    cpu.ir = 0x5302;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $03E814: cmpi.b #$0, d2
pub fn i_03e814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E818;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $03E818: bgt.b $3e7ea
pub fn i_03e818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E81A;
    cpu.ir = 0x6ED0;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3E7EA; return 10; }
    8
}
/// $03E81A: move.w #$2320, d1
pub fn i_03e81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E81E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E81E: movem.w d0-d1, $3c0000.l
pub fn i_03e81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E826;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E826: addi.w #$20, d0
pub fn i_03e826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E82A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E82A: movea.w d0, a1
pub fn i_03e82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E82C;
    cpu.ir = 0x3240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03E82C: lea.l $282bbc.l, a2
pub fn i_03e82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E832;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282BBC;
    20
}
/// $03E832: clr.w d4
pub fn i_03e832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E834;
    cpu.ir = 0x4244;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E834: move.b d3, d4
pub fn i_03e834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E836;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03E836: lsl.w #$1, d4
pub fn i_03e836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E838;
    cpu.ir = 0xE34C;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03E838: move.w (a2, d4.w), d0
pub fn i_03e838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E83C;
    cpu.ir = 0x3032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $03E83C: move.w #$4, d1
pub fn i_03e83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E840;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E840: move.w #$1, d2
pub fn i_03e840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E844;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_03e844  ($03E844, 8 B, thunks/jsr_abs_thunks.c)
/// $03E844: jsr $5da56.l
pub fn i_03e844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E84A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E84A);
    cpu.pc = 0x5DA56;
    16
}
/// $03E84A: rts
pub fn i_03e84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E84C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_RosterDrawB_03e84c  ($03E84C, 140 B, pow/results_squadleader_03daa8.s)
/// $03E84C: cmpa.l #$0, a1
pub fn i_03e84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E852;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03E852: bne.w $3e858
pub fn i_03e852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E856;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E858; return 10; }
    8
}
/// $03E856: rts
pub fn i_03e856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E858;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03E858: cmpi.b #$1, $68(a6)
pub fn i_03e858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E85E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03E85E: bne.w $3e866
pub fn i_03e85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E862;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3E866; return 10; }
    8
}
/// $03E862: addi.w #$220, d0
pub fn i_03e862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E866;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x220, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E866: lea.l $282abc.l, a2
pub fn i_03e866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E86C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282ABC;
    20
}
/// $03E86C: clr.l d4
pub fn i_03e86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E86E;
    cpu.ir = 0x4284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = r2;
    4
}
/// $03E86E: move.b d3, d4
pub fn i_03e86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E870;
    cpu.ir = 0x1803;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03E870: lsl.l #$2, d4
pub fn i_03e870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E872;
    cpu.ir = 0xE58C;
    let h1 = cpu.d[4] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 1, true);
    cpu.d[4] = r;
    10
}
/// $03E872: adda.l d4, a2
pub fn i_03e872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E874;
    cpu.ir = 0xD5C4;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $03E874: move.w #$2300, d1
pub fn i_03e874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E878;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E878: add.b (a2)+, d1
pub fn i_03e878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E87A;
    cpu.ir = 0xD21A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $03E87A: movem.w d0-d1, $3c0000.l
pub fn i_03e87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E882;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E882: addi.w #$20, d0
pub fn i_03e882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E886;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E886: move.w #$2300, d1
pub fn i_03e886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E88A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E88A: add.b (a2)+, d1
pub fn i_03e88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E88C;
    cpu.ir = 0xD21A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $03E88C: movem.w d0-d1, $3c0000.l
pub fn i_03e88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E894;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E894: addi.w #$20, d0
pub fn i_03e894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E898;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E898: move.w #$2300, d1
pub fn i_03e898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E89C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E89C: add.b (a2)+, d1
pub fn i_03e89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E89E;
    cpu.ir = 0xD21A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $03E89E: movem.w d0-d1, $3c0000.l
pub fn i_03e89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8A6;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E8A6: addi.w #$20, d0
pub fn i_03e8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8AA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E8AA: move.w #$2300, d1
pub fn i_03e8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E8AE: add.b (a2), d1
pub fn i_03e8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8B0;
    cpu.ir = 0xD212;
    let v2 = bus.read8(cpu.a[2]) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v2, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $03E8B0: movem.w d0-d1, $3c0000.l
pub fn i_03e8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8B8;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E8B8: addi.w #$20, d0
pub fn i_03e8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8BC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E8BC: move.w #$2300, d1
pub fn i_03e8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E8C0: add.b (a1)+, d1
pub fn i_03e8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8C2;
    cpu.ir = 0xD219;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    8
}
/// $03E8C2: movem.w d0-d1, $3c0000.l
pub fn i_03e8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8CA;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03E8CA: addi.w #$20, d0
pub fn i_03e8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8CE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03E8CE: subq.b #$1, d2
pub fn i_03e8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8D0;
    cpu.ir = 0x5302;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $03E8D0: cmpi.b #$0, d2
pub fn i_03e8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8D4;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $03E8D4: bgt.b $3e8bc
pub fn i_03e8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8D6;
    cpu.ir = 0x6EE6;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3E8BC; return 10; }
    8
}
/// $03E8D6: rts
pub fn i_03e8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ClampScoreP1_03e8d8  ($03E8D8, 34 B, pow/results_squadleader_03daa8.s)
/// $03E8D8: move.w $106f4c.l, d1
pub fn i_03e8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8DE;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F4C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $03E8DE: addq.w #$1, d1
pub fn i_03e8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8E0;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E8E0: cmpi.w #$e, d1
pub fn i_03e8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8E4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $03E8E4: bls.w $3e8ec
pub fn i_03e8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8E8;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x3E8EC; return 10; }
    8
}
/// $03E8E8: move.w #$e, d1
pub fn i_03e8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8EC;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E8EC: lea.l $2829fe.l, a1
pub fn i_03e8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8F2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2829FE;
    20
}
/// $03E8F2: lsl.w #$1, d1
pub fn i_03e8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8F4;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03E8F4: move.w (a1, d1.w), d0
pub fn i_03e8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8F8;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $03E8F8: rts
pub fn i_03e8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E8FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_ClampScoreP2_03e8fa  ($03E8FA, 34 B, pow/results_squadleader_03daa8.s)
/// $03E8FA: move.w $106f4e.l, d1
pub fn i_03e8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E900;
    cpu.ir = 0x3239;
    let v2 = bus.read16(0x106F4E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $03E900: addq.w #$1, d1
pub fn i_03e900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E902;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03E902: cmpi.w #$e, d1
pub fn i_03e902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E906;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xE, Size::Word, false);
    8
}
/// $03E906: bls.w $3e90e
pub fn i_03e906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E90A;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x3E90E; return 10; }
    8
}
/// $03E90A: move.w #$e, d1
pub fn i_03e90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E90E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E90E: lea.l $282a1e.l, a1
pub fn i_03e90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E914;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x282A1E;
    20
}
/// $03E914: lsl.w #$1, d1
pub fn i_03e914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E916;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03E916: move.w (a1, d1.w), d0
pub fn i_03e916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E91A;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $03E91A: rts
pub fn i_03e91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E91C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_03e9d4  ($03E9D4, 8 B, thunks/jsr_abs_thunks.c)
/// $03E9D4: jsr $28d70.l
pub fn i_03e9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E9DA);
    cpu.pc = 0x28D70;
    16
}
/// $03E9DA: rts
pub fn i_03e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_PrizeFall_03e9dc  ($03E9DC, 22 B, pow/results_squadleader_03daa8.s)
/// $03E9DC: jsr $8d2f8.l
pub fn i_03e9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E9E2);
    cpu.pc = 0x8D2F8;
    16
}
/// $03E9E2: cmpi.w #$198, $24(a6)
pub fn i_03e9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9E8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x198, Size::Word, false);
    16
}
/// $03E9E8: ble.w $3e9f2
pub fn i_03e9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9EC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x3E9F2; return 10; }
    8
}
/// $03E9EC: lea.l $3e9fa(pc), a1
pub fn i_03e9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3E9FA;
    16
}
/// $03E9F0: move.l a1, (a6)
pub fn i_03e9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_03e9f2  ($03E9F2, 8 B, thunks/jsr_abs_thunks.c)
/// $03E9F2: jsr $28d70.l
pub fn i_03e9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3E9F8);
    cpu.pc = 0x28D70;
    16
}
/// $03E9F8: rts
pub fn i_03e9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_PrizeLand_03e9fa  ($03E9FA, 38 B, pow/results_squadleader_03daa8.s)
/// $03E9FA: move.w #$10d6, d0
pub fn i_03e9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3E9FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10D6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03E9FE: jsr $2352.l
pub fn i_03e9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EA04);
    cpu.pc = 0x2352;
    16
}
/// $03EA04: movea.l $c(a6), a1
pub fn i_03ea04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA08;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $03EA08: move.b #$ff, $20(a1)
pub fn i_03ea08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA0E;
    cpu.ir = 0x137C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03EA0E: move.w $74(a6), d0
pub fn i_03ea0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA12;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03EA12: move.w #$714c, d4
pub fn i_03ea12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA16;
    cpu.ir = 0x383C;
    let v1: u32 = 0x714C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EA16: add.w $72(a6), d4
pub fn i_03ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA1A;
    cpu.ir = 0xD86E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EA1A: jsr $4772a.l
pub fn i_03ea1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EA20);
    cpu.pc = 0x4772A;
    16
}
// ===== Jsr5B6ThenJmpScheduler_03ea20  ($03EA20, 14 B, kernel/jsr5b6_jmp518.c)
/// $03EA20: jsr $5b6.l
pub fn i_03ea20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EA26);
    cpu.pc = 0x5B6;
    16
}
/// $03EA26: jmp $518.l
pub fn i_03ea26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA2C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Results_BannerA_03ea2e  ($03EA2E, 48 B, pow/results_squadleader_03daa8.s)
/// $03EA2E: move.w #$6, $30(a6)
pub fn i_03ea2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA34;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EA34: move.w #$0, $70(a6)
pub fn i_03ea34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA3A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EA3A: move.b #$0, $5c(a6)
pub fn i_03ea3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA40;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03EA40: lea.l $3ea46(pc), a1
pub fn i_03ea40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EA46;
    16
}
/// $03EA44: move.l a1, (a6)
pub fn i_03ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EA46: subq.w #$1, $70(a6)
pub fn i_03ea46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA4A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03EA4A: cmpi.w #$0, $70(a6)
pub fn i_03ea4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA50;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EA50: bgt.w $3eaa0
pub fn i_03ea50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA54;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EAA0; return 10; }
    8
}
/// $03EA54: cmpi.w #$0, $30(a6)
pub fn i_03ea54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA5A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EA5A: bgt.w $3ea66
pub fn i_03ea5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA5E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EA66; return 10; }
    8
}
// ===== SetTaskHandler_03ea5e  ($03EA5E, 8 B, kernel/task_handlers.c)
/// $03EA5E: lea.l $3eaa2(pc), a1
pub fn i_03ea5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EAA2;
    16
}
/// $03EA62: move.l a1, (a6)
pub fn i_03ea62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EA64: rts
pub fn i_03ea64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerABlink_03ea66  ($03EA66, 60 B, pow/results_squadleader_03daa8.s)
/// $03EA66: cmpi.b #$0, $5c(a6)
pub fn i_03ea66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA6C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03EA6C: bne.w $3ea7a
pub fn i_03ea6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EA7A; return 10; }
    8
}
/// $03EA70: lea.l $282a4c.l, a2
pub fn i_03ea70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA76;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A4C;
    20
}
/// $03EA76: bra.w $3ea80
pub fn i_03ea76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EA80;
    10
}
/// $03EA7A: lea.l $282a54.l, a2
pub fn i_03ea7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA80;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A54;
    20
}
/// $03EA80: movea.w #$70d1, a1
pub fn i_03ea80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA84;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70D1;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EA84: adda.w $72(a6), a1
pub fn i_03ea84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA88;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EA88: move.b #$4, d1
pub fn i_03ea88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA8C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EA8C: jsr $4784c.l
pub fn i_03ea8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EA92);
    cpu.pc = 0x4784C;
    16
}
/// $03EA92: move.w #$8, $70(a6)
pub fn i_03ea92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA98;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EA98: not.b $5c(a6)
pub fn i_03ea98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EA9C;
    cpu.ir = 0x462E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03EA9C: subq.w #$1, $30(a6)
pub fn i_03ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAA0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03EAA0: rts
pub fn i_03eaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerAFinal_03eaa2  ($03EAA2, 218 B, pow/results_squadleader_03daa8.s)
/// $03EAA2: move.w #$1083, d0
pub fn i_03eaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAA6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1083;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EAA6: jsr $2352.l
pub fn i_03eaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EAAC);
    cpu.pc = 0x2352;
    16
}
/// $03EAAC: move.l #$100000, d0
pub fn i_03eaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAB2;
    cpu.ir = 0x203C;
    let v1: u32 = 0x100000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $03EAB2: jsr $51a44.l
pub fn i_03eab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EAB8);
    cpu.pc = 0x51A44;
    16
}
/// $03EAB8: move.w #$70cf, d0
pub fn i_03eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EABC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x70CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EABC: add.w $72(a6), d0
pub fn i_03eabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAC0;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EAC0: move.w #$1, d1
pub fn i_03eac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAC4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EAC4: lsl.w #$1, d1
pub fn i_03eac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAC6;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03EAC6: addi.w #$4b60, d1
pub fn i_03eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EACA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4B60, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EACA: movem.w d0-d1, $3c0000.l
pub fn i_03eaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAD2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03EAD2: addi.w #$20, d0
pub fn i_03ead2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAD6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EAD6: addq.w #$1, d1
pub fn i_03ead6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAD8;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03EAD8: movem.w d0-d1, $3c0000.l
pub fn i_03ead8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAE0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03EAE0: subi.w #$1f, d0
pub fn i_03eae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAE4;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EAE4: addi.w #$f, d1
pub fn i_03eae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAE8;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EAE8: movem.w d0-d1, $3c0000.l
pub fn i_03eae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAF0;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03EAF0: addi.w #$20, d0
pub fn i_03eaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAF4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EAF4: addq.w #$1, d1
pub fn i_03eaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAF6;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03EAF6: movem.w d0-d1, $3c0000.l
pub fn i_03eaf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EAFE;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03EAFE: subi.w #$21, d0
pub fn i_03eafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB02;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EB02: cmpi.w #$a, $74(a6)
pub fn i_03eb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB08;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA, Size::Word, false);
    16
}
/// $03EB08: bcc.w $3eb64
pub fn i_03eb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB0C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3EB64; return 10; }
    8
}
/// $03EB0C: move.w $76(a6), d0
pub fn i_03eb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB10;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03EB10: jsr $2ad36.l
pub fn i_03eb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EB16);
    cpu.pc = 0x2AD36;
    16
}
/// $03EB16: bcs.w $3eb64
pub fn i_03eb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB1A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3EB64; return 10; }
    8
}
/// $03EB1A: move.w #$710f, d0
pub fn i_03eb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB1E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x710F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EB1E: add.w $72(a6), d0
pub fn i_03eb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB22;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $03EB22: move.w #$0, d1
pub fn i_03eb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB26;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03EB26: lsl.w #$1, d1
pub fn i_03eb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB28;
    cpu.ir = 0xE349;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $03EB28: addi.w #$4b60, d1
pub fn i_03eb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB2C;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x4B60, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EB2C: movem.w d0-d1, $3c0000.l
pub fn i_03eb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB34;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03EB34: addi.w #$20, d0
pub fn i_03eb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB38;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EB38: addq.w #$1, d1
pub fn i_03eb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB3A;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03EB3A: movem.w d0-d1, $3c0000.l
pub fn i_03eb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB42;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03EB42: subi.w #$1f, d0
pub fn i_03eb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB46;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EB46: addi.w #$f, d1
pub fn i_03eb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB4A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xF, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EB4A: movem.w d0-d1, $3c0000.l
pub fn i_03eb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB52;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03EB52: addi.w #$20, d0
pub fn i_03eb52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB56;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EB56: addq.w #$1, d1
pub fn i_03eb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB58;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $03EB58: movem.w d0-d1, $3c0000.l
pub fn i_03eb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB60;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $03EB60: subi.w #$21, d0
pub fn i_03eb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB64;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x21, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03EB64: movea.w #$70d1, a1
pub fn i_03eb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB68;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70D1;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EB68: adda.w $72(a6), a1
pub fn i_03eb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB6C;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EB6C: move.b #$4, d1
pub fn i_03eb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB70;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EB70: lea.l $282a4c.l, a2
pub fn i_03eb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB76;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A4C;
    20
}
/// $03EB76: jsr $4784c.l
pub fn i_03eb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EB7C);
    cpu.pc = 0x4784C;
    16
}
// ===== SetTaskHandler_03eb7c  ($03EB7C, 8 B, kernel/task_handlers.c)
/// $03EB7C: lea.l $3eb82(pc), a1
pub fn i_03eb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EB82;
    16
}
/// $03EB80: move.l a1, (a6)
pub fn i_03eb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EB82: rts
pub fn i_03eb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerB_03eb84  ($03EB84, 48 B, pow/results_squadleader_03daa8.s)
/// $03EB84: move.w #$6, $30(a6)
pub fn i_03eb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB8A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EB8A: move.w #$0, $70(a6)
pub fn i_03eb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB90;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EB90: move.b #$0, $5c(a6)
pub fn i_03eb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03EB96: lea.l $3eb9c(pc), a1
pub fn i_03eb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EB9C;
    16
}
/// $03EB9A: move.l a1, (a6)
pub fn i_03eb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EB9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EB9C: subq.w #$1, $70(a6)
pub fn i_03eb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBA0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03EBA0: cmpi.w #$0, $70(a6)
pub fn i_03eba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBA6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EBA6: bgt.w $3ebf6
pub fn i_03eba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBAA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EBF6; return 10; }
    8
}
/// $03EBAA: cmpi.w #$0, $30(a6)
pub fn i_03ebaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBB0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EBB0: bgt.w $3ebbc
pub fn i_03ebb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBB4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EBBC; return 10; }
    8
}
// ===== SetTaskHandler_03ebb4  ($03EBB4, 8 B, kernel/task_handlers.c)
/// $03EBB4: lea.l $3ebf8(pc), a1
pub fn i_03ebb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EBF8;
    16
}
/// $03EBB8: move.l a1, (a6)
pub fn i_03ebb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EBBA: rts
pub fn i_03ebba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerBBlink_03ebbc  ($03EBBC, 60 B, pow/results_squadleader_03daa8.s)
/// $03EBBC: cmpi.b #$0, $5c(a6)
pub fn i_03ebbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBC2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03EBC2: bne.w $3ebd0
pub fn i_03ebc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3EBD0; return 10; }
    8
}
/// $03EBC6: lea.l $282a7c.l, a2
pub fn i_03ebc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBCC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A7C;
    20
}
/// $03EBCC: bra.w $3ebd6
pub fn i_03ebcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x3EBD6;
    10
}
/// $03EBD0: lea.l $282a88.l, a2
pub fn i_03ebd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBD6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A88;
    20
}
/// $03EBD6: movea.w #$70d1, a1
pub fn i_03ebd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBDA;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70D1;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EBDA: adda.w $72(a6), a1
pub fn i_03ebda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBDE;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EBDE: move.b #$4, d1
pub fn i_03ebde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBE2;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EBE2: jsr $477fc.l
pub fn i_03ebe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EBE8);
    cpu.pc = 0x477FC;
    16
}
/// $03EBE8: move.w #$8, $70(a6)
pub fn i_03ebe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBEE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EBEE: not.b $5c(a6)
pub fn i_03ebee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBF2;
    cpu.ir = 0x462E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03EBF2: subq.w #$1, $30(a6)
pub fn i_03ebf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBF6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03EBF6: rts
pub fn i_03ebf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerBDraw_03ebf8  ($03EBF8, 24 B, pow/results_squadleader_03daa8.s)
/// $03EBF8: movea.w #$70d1, a1
pub fn i_03ebf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EBFC;
    cpu.ir = 0x327C;
    let v1: u32 = 0x70D1;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $03EBFC: adda.w $72(a6), a1
pub fn i_03ebfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC00;
    cpu.ir = 0xD2EE;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = cpu.a[1].wrapping_add((v3 as u16 as i16 as i32 as u32));
    16
}
/// $03EC00: move.b #$4, d1
pub fn i_03ec00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC04;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $03EC04: lea.l $282a7c.l, a2
pub fn i_03ec04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC0A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x282A7C;
    20
}
/// $03EC0A: jsr $477fc.l
pub fn i_03ec0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3EC10);
    cpu.pc = 0x477FC;
    16
}
// ===== SetTaskHandler_03ec10  ($03EC10, 8 B, kernel/task_handlers.c)
/// $03EC10: lea.l $3ec16(pc), a1
pub fn i_03ec10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EC16;
    16
}
/// $03EC14: move.l a1, (a6)
pub fn i_03ec14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EC16: rts
pub fn i_03ec16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Results_BannerC_03ec18  ($03EC18, 48 B, pow/results_squadleader_03daa8.s)
/// $03EC18: move.w #$6, $30(a6)
pub fn i_03ec18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EC1E: move.w #$0, $70(a6)
pub fn i_03ec1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03EC24: move.b #$0, $5c(a6)
pub fn i_03ec24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC2A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03EC2A: lea.l $3ec30(pc), a1
pub fn i_03ec2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EC30;
    16
}
/// $03EC2E: move.l a1, (a6)
pub fn i_03ec2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03EC30: subq.w #$1, $70(a6)
pub fn i_03ec30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC34;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03EC34: cmpi.w #$0, $70(a6)
pub fn i_03ec34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC3A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EC3A: bgt.w $3ec8a
pub fn i_03ec3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC3E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EC8A; return 10; }
    8
}
/// $03EC3E: cmpi.w #$0, $30(a6)
pub fn i_03ec3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC44;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03EC44: bgt.w $3ec50
pub fn i_03ec44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC48;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x3EC50; return 10; }
    8
}
// ===== SetTaskHandler_03ec48  ($03EC48, 8 B, kernel/task_handlers.c)
/// $03EC48: lea.l $3ec8c(pc), a1
pub fn i_03ec48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3EC4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3EC8C;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x03CD1E, i_03cd1e),
    (0x03CD24, i_03cd24),
    (0x03CD2A, i_03cd2a),
    (0x03CD30, i_03cd30),
    (0x03CD36, i_03cd36),
    (0x03CD38, i_03cd38),
    (0x03CD3C, i_03cd3c),
    (0x03CD3E, i_03cd3e),
    (0x03CD42, i_03cd42),
    (0x03CD48, i_03cd48),
    (0x03CD4C, i_03cd4c),
    (0x03CD50, i_03cd50),
    (0x03CD54, i_03cd54),
    (0x03CD58, i_03cd58),
    (0x03CD5C, i_03cd5c),
    (0x03CD5E, i_03cd5e),
    (0x03CD62, i_03cd62),
    (0x03CD68, i_03cd68),
    (0x03CD6E, i_03cd6e),
    (0x03CD70, i_03cd70),
    (0x03CD74, i_03cd74),
    (0x03CD7A, i_03cd7a),
    (0x03CD7E, i_03cd7e),
    (0x03CD82, i_03cd82),
    (0x03CD86, i_03cd86),
    (0x03CD8A, i_03cd8a),
    (0x03CD8E, i_03cd8e),
    (0x03CD90, i_03cd90),
    (0x03CD94, i_03cd94),
    (0x03CD9A, i_03cd9a),
    (0x03CDA0, i_03cda0),
    (0x03CDA2, i_03cda2),
    (0x03CDA6, i_03cda6),
    (0x03CDAC, i_03cdac),
    (0x03CDB0, i_03cdb0),
    (0x03CDB4, i_03cdb4),
    (0x03CDB8, i_03cdb8),
    (0x03CDBC, i_03cdbc),
    (0x03CDC0, i_03cdc0),
    (0x03CDC2, i_03cdc2),
    (0x03CDC6, i_03cdc6),
    (0x03CDCC, i_03cdcc),
    (0x03CDD2, i_03cdd2),
    (0x03CDD4, i_03cdd4),
    (0x03CDD8, i_03cdd8),
    (0x03CDDE, i_03cdde),
    (0x03CDE2, i_03cde2),
    (0x03CDE6, i_03cde6),
    (0x03CDEC, i_03cdec),
    (0x03CDEE, i_03cdee),
    (0x03CDF2, i_03cdf2),
    (0x03CDF8, i_03cdf8),
    (0x03CDFC, i_03cdfc),
    (0x03CE00, i_03ce00),
    (0x03CE06, i_03ce06),
    (0x03CE08, i_03ce08),
    (0x03CE0C, i_03ce0c),
    (0x03CE12, i_03ce12),
    (0x03CE16, i_03ce16),
    (0x03CE1A, i_03ce1a),
    (0x03CE20, i_03ce20),
    (0x03CE22, i_03ce22),
    (0x03CE26, i_03ce26),
    (0x03CE2C, i_03ce2c),
    (0x03CE30, i_03ce30),
    (0x03CE34, i_03ce34),
    (0x03CE38, i_03ce38),
    (0x03CE3C, i_03ce3c),
    (0x03CE40, i_03ce40),
    (0x03CE42, i_03ce42),
    (0x03CE46, i_03ce46),
    (0x03CE4C, i_03ce4c),
    (0x03CE52, i_03ce52),
    (0x03CE54, i_03ce54),
    (0x03CE58, i_03ce58),
    (0x03CE5E, i_03ce5e),
    (0x03CE62, i_03ce62),
    (0x03CE66, i_03ce66),
    (0x03CE6A, i_03ce6a),
    (0x03CE6E, i_03ce6e),
    (0x03CE72, i_03ce72),
    (0x03CE74, i_03ce74),
    (0x03CE78, i_03ce78),
    (0x03CE7E, i_03ce7e),
    (0x03CE84, i_03ce84),
    (0x03CE86, i_03ce86),
    (0x03CE8A, i_03ce8a),
    (0x03CE90, i_03ce90),
    (0x03CE94, i_03ce94),
    (0x03CE98, i_03ce98),
    (0x03CE9C, i_03ce9c),
    (0x03CEA0, i_03cea0),
    (0x03CEA4, i_03cea4),
    (0x03CEA6, i_03cea6),
    (0x03CEAA, i_03ceaa),
    (0x03CEB0, i_03ceb0),
    (0x03CEB6, i_03ceb6),
    (0x03CEB8, i_03ceb8),
    (0x03CEBC, i_03cebc),
    (0x03CEC2, i_03cec2),
    (0x03CEC6, i_03cec6),
    (0x03CECA, i_03ceca),
    (0x03CECE, i_03cece),
    (0x03CED2, i_03ced2),
    (0x03CED6, i_03ced6),
    (0x03CED8, i_03ced8),
    (0x03CEDC, i_03cedc),
    (0x03CEE2, i_03cee2),
    (0x03CEE8, i_03cee8),
    (0x03CEEA, i_03ceea),
    (0x03CEEE, i_03ceee),
    (0x03CEF4, i_03cef4),
    (0x03CEF8, i_03cef8),
    (0x03CEFC, i_03cefc),
    (0x03CF00, i_03cf00),
    (0x03CF04, i_03cf04),
    (0x03CF08, i_03cf08),
    (0x03CF0C, i_03cf0c),
    (0x03CF10, i_03cf10),
    (0x03CF12, i_03cf12),
    (0x03CF16, i_03cf16),
    (0x03CF1C, i_03cf1c),
    (0x03CF22, i_03cf22),
    (0x03CF24, i_03cf24),
    (0x03CF28, i_03cf28),
    (0x03CF2E, i_03cf2e),
    (0x03CF32, i_03cf32),
    (0x03CF36, i_03cf36),
    (0x03CF3A, i_03cf3a),
    (0x03CF3E, i_03cf3e),
    (0x03CF42, i_03cf42),
    (0x03CF44, i_03cf44),
    (0x03CF48, i_03cf48),
    (0x03CF4E, i_03cf4e),
    (0x03CF54, i_03cf54),
    (0x03CF56, i_03cf56),
    (0x03CF5A, i_03cf5a),
    (0x03CF60, i_03cf60),
    (0x03CF64, i_03cf64),
    (0x03CF6A, i_03cf6a),
    (0x03CF6E, i_03cf6e),
    (0x03CF72, i_03cf72),
    (0x03CF78, i_03cf78),
    (0x03CF7C, i_03cf7c),
    (0x03CF80, i_03cf80),
    (0x03CF82, i_03cf82),
    (0x03CF86, i_03cf86),
    (0x03CF8C, i_03cf8c),
    (0x03CF92, i_03cf92),
    (0x03CF94, i_03cf94),
    (0x03CF98, i_03cf98),
    (0x03CF9E, i_03cf9e),
    (0x03CFA2, i_03cfa2),
    (0x03CFA8, i_03cfa8),
    (0x03CFAC, i_03cfac),
    (0x03CFB0, i_03cfb0),
    (0x03CFB6, i_03cfb6),
    (0x03CFBA, i_03cfba),
    (0x03CFBE, i_03cfbe),
    (0x03CFC0, i_03cfc0),
    (0x03CFC4, i_03cfc4),
    (0x03CFCA, i_03cfca),
    (0x03CFD0, i_03cfd0),
    (0x03CFD2, i_03cfd2),
    (0x03CFD6, i_03cfd6),
    (0x03CFDC, i_03cfdc),
    (0x03CFE0, i_03cfe0),
    (0x03CFE6, i_03cfe6),
    (0x03CFEA, i_03cfea),
    (0x03CFEE, i_03cfee),
    (0x03CFF4, i_03cff4),
    (0x03CFF8, i_03cff8),
    (0x03CFFC, i_03cffc),
    (0x03CFFE, i_03cffe),
    (0x03D002, i_03d002),
    (0x03D008, i_03d008),
    (0x03D00E, i_03d00e),
    (0x03D010, i_03d010),
    (0x03D014, i_03d014),
    (0x03D01A, i_03d01a),
    (0x03D01E, i_03d01e),
    (0x03D022, i_03d022),
    (0x03D026, i_03d026),
    (0x03D02A, i_03d02a),
    (0x03D02E, i_03d02e),
    (0x03D030, i_03d030),
    (0x03D034, i_03d034),
    (0x03D03A, i_03d03a),
    (0x03D040, i_03d040),
    (0x03D042, i_03d042),
    (0x03D046, i_03d046),
    (0x03D04C, i_03d04c),
    (0x03D050, i_03d050),
    (0x03D054, i_03d054),
    (0x03D058, i_03d058),
    (0x03D05C, i_03d05c),
    (0x03D060, i_03d060),
    (0x03D062, i_03d062),
    (0x03D066, i_03d066),
    (0x03D06C, i_03d06c),
    (0x03D072, i_03d072),
    (0x03D074, i_03d074),
    (0x03D078, i_03d078),
    (0x03D07E, i_03d07e),
    (0x03D082, i_03d082),
    (0x03D086, i_03d086),
    (0x03D08A, i_03d08a),
    (0x03D08E, i_03d08e),
    (0x03D092, i_03d092),
    (0x03D094, i_03d094),
    (0x03D098, i_03d098),
    (0x03D09E, i_03d09e),
    (0x03D0A4, i_03d0a4),
    (0x03D0A6, i_03d0a6),
    (0x03D0AA, i_03d0aa),
    (0x03D0B0, i_03d0b0),
    (0x03D0B4, i_03d0b4),
    (0x03D0B8, i_03d0b8),
    (0x03D0BC, i_03d0bc),
    (0x03D0C0, i_03d0c0),
    (0x03D0C4, i_03d0c4),
    (0x03D0C6, i_03d0c6),
    (0x03D0CA, i_03d0ca),
    (0x03D0D0, i_03d0d0),
    (0x03D0D4, i_03d0d4),
    (0x03D0D8, i_03d0d8),
    (0x03D0DE, i_03d0de),
    (0x03D0E2, i_03d0e2),
    (0x03D0E6, i_03d0e6),
    (0x03D0EA, i_03d0ea),
    (0x03D0EE, i_03d0ee),
    (0x03D0F2, i_03d0f2),
    (0x03D0F4, i_03d0f4),
    (0x03D0F8, i_03d0f8),
    (0x03D0FE, i_03d0fe),
    (0x03D102, i_03d102),
    (0x03D104, i_03d104),
    (0x03D108, i_03d108),
    (0x03D10E, i_03d10e),
    (0x03D112, i_03d112),
    (0x03D116, i_03d116),
    (0x03D11A, i_03d11a),
    (0x03D11E, i_03d11e),
    (0x03D122, i_03d122),
    (0x03D124, i_03d124),
    (0x03D128, i_03d128),
    (0x03D12E, i_03d12e),
    (0x03D132, i_03d132),
    (0x03D136, i_03d136),
    (0x03D13C, i_03d13c),
    (0x03D140, i_03d140),
    (0x03D144, i_03d144),
    (0x03D148, i_03d148),
    (0x03D14C, i_03d14c),
    (0x03D150, i_03d150),
    (0x03D152, i_03d152),
    (0x03D156, i_03d156),
    (0x03D15C, i_03d15c),
    (0x03D160, i_03d160),
    (0x03D162, i_03d162),
    (0x03D166, i_03d166),
    (0x03D16C, i_03d16c),
    (0x03D170, i_03d170),
    (0x03D174, i_03d174),
    (0x03D178, i_03d178),
    (0x03D17C, i_03d17c),
    (0x03D180, i_03d180),
    (0x03D182, i_03d182),
    (0x03D186, i_03d186),
    (0x03D18C, i_03d18c),
    (0x03D190, i_03d190),
    (0x03D194, i_03d194),
    (0x03D19A, i_03d19a),
    (0x03D19E, i_03d19e),
    (0x03D1A2, i_03d1a2),
    (0x03D1A6, i_03d1a6),
    (0x03D1AA, i_03d1aa),
    (0x03D1AE, i_03d1ae),
    (0x03D1B0, i_03d1b0),
    (0x03D1B4, i_03d1b4),
    (0x03D1BA, i_03d1ba),
    (0x03D1BE, i_03d1be),
    (0x03D1C0, i_03d1c0),
    (0x03D1C4, i_03d1c4),
    (0x03D1CA, i_03d1ca),
    (0x03D1CE, i_03d1ce),
    (0x03D1D2, i_03d1d2),
    (0x03D1D6, i_03d1d6),
    (0x03D1DA, i_03d1da),
    (0x03D1DE, i_03d1de),
    (0x03D1E0, i_03d1e0),
    (0x03D1E4, i_03d1e4),
    (0x03D1EA, i_03d1ea),
    (0x03D1EE, i_03d1ee),
    (0x03D1F2, i_03d1f2),
    (0x03D1F8, i_03d1f8),
    (0x03D1FC, i_03d1fc),
    (0x03D200, i_03d200),
    (0x03D204, i_03d204),
    (0x03D208, i_03d208),
    (0x03D20C, i_03d20c),
    (0x03D20E, i_03d20e),
    (0x03D212, i_03d212),
    (0x03D218, i_03d218),
    (0x03D21C, i_03d21c),
    (0x03D21E, i_03d21e),
    (0x03D222, i_03d222),
    (0x03D228, i_03d228),
    (0x03D22C, i_03d22c),
    (0x03D230, i_03d230),
    (0x03D234, i_03d234),
    (0x03D238, i_03d238),
    (0x03D23C, i_03d23c),
    (0x03D23E, i_03d23e),
    (0x03D242, i_03d242),
    (0x03D248, i_03d248),
    (0x03D24C, i_03d24c),
    (0x03D250, i_03d250),
    (0x03D256, i_03d256),
    (0x03D25A, i_03d25a),
    (0x03D25E, i_03d25e),
    (0x03D262, i_03d262),
    (0x03D266, i_03d266),
    (0x03D26A, i_03d26a),
    (0x03D26C, i_03d26c),
    (0x03D270, i_03d270),
    (0x03D276, i_03d276),
    (0x03D27A, i_03d27a),
    (0x03D27C, i_03d27c),
    (0x03D280, i_03d280),
    (0x03D286, i_03d286),
    (0x03D28A, i_03d28a),
    (0x03D28E, i_03d28e),
    (0x03D292, i_03d292),
    (0x03D296, i_03d296),
    (0x03D29A, i_03d29a),
    (0x03D29C, i_03d29c),
    (0x03D2A0, i_03d2a0),
    (0x03D2A6, i_03d2a6),
    (0x03D2AA, i_03d2aa),
    (0x03D2AE, i_03d2ae),
    (0x03D2B4, i_03d2b4),
    (0x03D2B8, i_03d2b8),
    (0x03D2BC, i_03d2bc),
    (0x03D2C0, i_03d2c0),
    (0x03D2C4, i_03d2c4),
    (0x03D2C8, i_03d2c8),
    (0x03D2CA, i_03d2ca),
    (0x03D2CE, i_03d2ce),
    (0x03D2D4, i_03d2d4),
    (0x03D2D8, i_03d2d8),
    (0x03D2DA, i_03d2da),
    (0x03D2DE, i_03d2de),
    (0x03D2E4, i_03d2e4),
    (0x03D2E8, i_03d2e8),
    (0x03D2EC, i_03d2ec),
    (0x03D2F0, i_03d2f0),
    (0x03D2F4, i_03d2f4),
    (0x03D2F8, i_03d2f8),
    (0x03D2FA, i_03d2fa),
    (0x03D2FE, i_03d2fe),
    (0x03D304, i_03d304),
    (0x03D308, i_03d308),
    (0x03D30C, i_03d30c),
    (0x03D312, i_03d312),
    (0x03D316, i_03d316),
    (0x03D31A, i_03d31a),
    (0x03D31E, i_03d31e),
    (0x03D322, i_03d322),
    (0x03D326, i_03d326),
    (0x03D328, i_03d328),
    (0x03D32C, i_03d32c),
    (0x03D332, i_03d332),
    (0x03D336, i_03d336),
    (0x03D338, i_03d338),
    (0x03D33C, i_03d33c),
    (0x03D342, i_03d342),
    (0x03D346, i_03d346),
    (0x03D34A, i_03d34a),
    (0x03D34E, i_03d34e),
    (0x03D352, i_03d352),
    (0x03D356, i_03d356),
    (0x03D358, i_03d358),
    (0x03D35C, i_03d35c),
    (0x03D362, i_03d362),
    (0x03D366, i_03d366),
    (0x03D36A, i_03d36a),
    (0x03D370, i_03d370),
    (0x03D374, i_03d374),
    (0x03D378, i_03d378),
    (0x03D37C, i_03d37c),
    (0x03D380, i_03d380),
    (0x03D384, i_03d384),
    (0x03D386, i_03d386),
    (0x03D38A, i_03d38a),
    (0x03D390, i_03d390),
    (0x03D394, i_03d394),
    (0x03D396, i_03d396),
    (0x03D39A, i_03d39a),
    (0x03D3A0, i_03d3a0),
    (0x03D3B2, i_03d3b2),
    (0x03D3B6, i_03d3b6),
    (0x03D3BC, i_03d3bc),
    (0x03D3C0, i_03d3c0),
    (0x03D3C6, i_03d3c6),
    (0x03D3CC, i_03d3cc),
    (0x03D3D2, i_03d3d2),
    (0x03D3D8, i_03d3d8),
    (0x03D3DC, i_03d3dc),
    (0x03D3DE, i_03d3de),
    (0x03D3E4, i_03d3e4),
    (0x03D3EA, i_03d3ea),
    (0x03D3EE, i_03d3ee),
    (0x03D3F4, i_03d3f4),
    (0x03D4EA, i_03d4ea),
    (0x03D4EE, i_03d4ee),
    (0x03D4F4, i_03d4f4),
    (0x03D4F8, i_03d4f8),
    (0x03D4FC, i_03d4fc),
    (0x03D502, i_03d502),
    (0x03D506, i_03d506),
    (0x03D50A, i_03d50a),
    (0x03D510, i_03d510),
    (0x03D514, i_03d514),
    (0x03D51A, i_03d51a),
    (0x03D520, i_03d520),
    (0x03D526, i_03d526),
    (0x03D52C, i_03d52c),
    (0x03D530, i_03d530),
    (0x03D532, i_03d532),
    (0x03D538, i_03d538),
    (0x03D53E, i_03d53e),
    (0x03D542, i_03d542),
    (0x03D548, i_03d548),
    (0x03D616, i_03d616),
    (0x03D61E, i_03d61e),
    (0x03D622, i_03d622),
    (0x03D626, i_03d626),
    (0x03D62C, i_03d62c),
    (0x03D632, i_03d632),
    (0x03D638, i_03d638),
    (0x03D63E, i_03d63e),
    (0x03D640, i_03d640),
    (0x03D648, i_03d648),
    (0x03D64C, i_03d64c),
    (0x03D650, i_03d650),
    (0x03D656, i_03d656),
    (0x03D65C, i_03d65c),
    (0x03D662, i_03d662),
    (0x03D668, i_03d668),
    (0x03D66C, i_03d66c),
    (0x03D66E, i_03d66e),
    (0x03D676, i_03d676),
    (0x03D67A, i_03d67a),
    (0x03D67E, i_03d67e),
    (0x03D684, i_03d684),
    (0x03D68A, i_03d68a),
    (0x03D690, i_03d690),
    (0x03D696, i_03d696),
    (0x03D69C, i_03d69c),
    (0x03D6A2, i_03d6a2),
    (0x03D6A6, i_03d6a6),
    (0x03D6AA, i_03d6aa),
    (0x03D6AE, i_03d6ae),
    (0x03D6B2, i_03d6b2),
    (0x03D6B4, i_03d6b4),
    (0x03D6BC, i_03d6bc),
    (0x03D6C0, i_03d6c0),
    (0x03D6C4, i_03d6c4),
    (0x03D6CA, i_03d6ca),
    (0x03D6D0, i_03d6d0),
    (0x03D6D6, i_03d6d6),
    (0x03D6DC, i_03d6dc),
    (0x03D6E2, i_03d6e2),
    (0x03D6E4, i_03d6e4),
    (0x03D6EC, i_03d6ec),
    (0x03D6F0, i_03d6f0),
    (0x03D6F4, i_03d6f4),
    (0x03D6FA, i_03d6fa),
    (0x03D700, i_03d700),
    (0x03D706, i_03d706),
    (0x03D70C, i_03d70c),
    (0x03D712, i_03d712),
    (0x03D718, i_03d718),
    (0x03D71C, i_03d71c),
    (0x03D720, i_03d720),
    (0x03D724, i_03d724),
    (0x03D728, i_03d728),
    (0x03D72A, i_03d72a),
    (0x03D732, i_03d732),
    (0x03D736, i_03d736),
    (0x03D73A, i_03d73a),
    (0x03D740, i_03d740),
    (0x03D746, i_03d746),
    (0x03D74C, i_03d74c),
    (0x03D752, i_03d752),
    (0x03D756, i_03d756),
    (0x03D75C, i_03d75c),
    (0x03D760, i_03d760),
    (0x03D764, i_03d764),
    (0x03D768, i_03d768),
    (0x03D76C, i_03d76c),
    (0x03D76E, i_03d76e),
    (0x03D772, i_03d772),
    (0x03D778, i_03d778),
    (0x03D77C, i_03d77c),
    (0x03D782, i_03d782),
    (0x03D786, i_03d786),
    (0x03D788, i_03d788),
    (0x03D78E, i_03d78e),
    (0x03D794, i_03d794),
    (0x03D798, i_03d798),
    (0x03D79E, i_03d79e),
    (0x03D842, i_03d842),
    (0x03D846, i_03d846),
    (0x03D84C, i_03d84c),
    (0x03D850, i_03d850),
    (0x03D854, i_03d854),
    (0x03D85A, i_03d85a),
    (0x03D85E, i_03d85e),
    (0x03D864, i_03d864),
    (0x03D86A, i_03d86a),
    (0x03D870, i_03d870),
    (0x03D876, i_03d876),
    (0x03D87A, i_03d87a),
    (0x03D87C, i_03d87c),
    (0x03D882, i_03d882),
    (0x03D888, i_03d888),
    (0x03D88C, i_03d88c),
    (0x03D892, i_03d892),
    (0x03D8FA, i_03d8fa),
    (0x03D8FE, i_03d8fe),
    (0x03D904, i_03d904),
    (0x03D908, i_03d908),
    (0x03D90E, i_03d90e),
    (0x03D914, i_03d914),
    (0x03D91A, i_03d91a),
    (0x03D920, i_03d920),
    (0x03D926, i_03d926),
    (0x03D92A, i_03d92a),
    (0x03D92C, i_03d92c),
    (0x03D932, i_03d932),
    (0x03D938, i_03d938),
    (0x03D93C, i_03d93c),
    (0x03D942, i_03d942),
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
    (0x03DAA8, i_03daa8),
    (0x03DBBC, i_03dbbc),
    (0x03DBC0, i_03dbc0),
    (0x03DBC2, i_03dbc2),
    (0x03DBC6, i_03dbc6),
    (0x03DBC8, i_03dbc8),
    (0x03DBD0, i_03dbd0),
    (0x03DBD4, i_03dbd4),
    (0x03DBDC, i_03dbdc),
    (0x03DBE0, i_03dbe0),
    (0x03DBE6, i_03dbe6),
    (0x03DBEC, i_03dbec),
    (0x03DBF4, i_03dbf4),
    (0x03DBFA, i_03dbfa),
    (0x03DBFC, i_03dbfc),
    (0x03DC02, i_03dc02),
    (0x03DC06, i_03dc06),
    (0x03DC08, i_03dc08),
    (0x03DC0E, i_03dc0e),
    (0x03DC12, i_03dc12),
    (0x03DC14, i_03dc14),
    (0x03DC16, i_03dc16),
    (0x03DC1A, i_03dc1a),
    (0x03DC20, i_03dc20),
    (0x03DC24, i_03dc24),
    (0x03DC28, i_03dc28),
    (0x03DC2A, i_03dc2a),
    (0x03DC2C, i_03dc2c),
    (0x03DC32, i_03dc32),
    (0x03DC36, i_03dc36),
    (0x03DC3C, i_03dc3c),
    (0x03DC40, i_03dc40),
    (0x03DC44, i_03dc44),
    (0x03DC48, i_03dc48),
    (0x03DC4C, i_03dc4c),
    (0x03DC52, i_03dc52),
    (0x03DC58, i_03dc58),
    (0x03DC5C, i_03dc5c),
    (0x03DC5E, i_03dc5e),
    (0x03DC62, i_03dc62),
    (0x03DC68, i_03dc68),
    (0x03DC6C, i_03dc6c),
    (0x03DC70, i_03dc70),
    (0x03DC72, i_03dc72),
    (0x03DC74, i_03dc74),
    (0x03DC78, i_03dc78),
    (0x03DC7C, i_03dc7c),
    (0x03DC80, i_03dc80),
    (0x03DC84, i_03dc84),
    (0x03DC88, i_03dc88),
    (0x03DC8C, i_03dc8c),
    (0x03DC94, i_03dc94),
    (0x03DC98, i_03dc98),
    (0x03DC9C, i_03dc9c),
    (0x03DCA2, i_03dca2),
    (0x03DCA6, i_03dca6),
    (0x03DCAA, i_03dcaa),
    (0x03DCAE, i_03dcae),
    (0x03DCB4, i_03dcb4),
    (0x03DCB8, i_03dcb8),
    (0x03DCBA, i_03dcba),
    (0x03DCC2, i_03dcc2),
    (0x03DCC6, i_03dcc6),
    (0x03DCCA, i_03dcca),
    (0x03DCD0, i_03dcd0),
    (0x03DCD4, i_03dcd4),
    (0x03DCD8, i_03dcd8),
    (0x03DCDC, i_03dcdc),
    (0x03DCE2, i_03dce2),
    (0x03DCE6, i_03dce6),
    (0x03DCE8, i_03dce8),
    (0x03DCEE, i_03dcee),
    (0x03DCF0, i_03dcf0),
    (0x03DCF4, i_03dcf4),
    (0x03DCFA, i_03dcfa),
    (0x03DCFE, i_03dcfe),
    (0x03DD04, i_03dd04),
    (0x03DD06, i_03dd06),
    (0x03DD0A, i_03dd0a),
    (0x03DD10, i_03dd10),
    (0x03DD16, i_03dd16),
    (0x03DD1C, i_03dd1c),
    (0x03DD20, i_03dd20),
    (0x03DD24, i_03dd24),
    (0x03DD26, i_03dd26),
    (0x03DD28, i_03dd28),
    (0x03DD30, i_03dd30),
    (0x03DD34, i_03dd34),
    (0x03DD38, i_03dd38),
    (0x03DD3C, i_03dd3c),
    (0x03DD40, i_03dd40),
    (0x03DD48, i_03dd48),
    (0x03DD4C, i_03dd4c),
    (0x03DD4E, i_03dd4e),
    (0x03DD56, i_03dd56),
    (0x03DD5A, i_03dd5a),
    (0x03DD5E, i_03dd5e),
    (0x03DD66, i_03dd66),
    (0x03DD6A, i_03dd6a),
    (0x03DD6C, i_03dd6c),
    (0x03DD74, i_03dd74),
    (0x03DD78, i_03dd78),
    (0x03DD7C, i_03dd7c),
    (0x03DD80, i_03dd80),
    (0x03DD84, i_03dd84),
    (0x03DD8C, i_03dd8c),
    (0x03DD90, i_03dd90),
    (0x03DD92, i_03dd92),
    (0x03DD9A, i_03dd9a),
    (0x03DD9E, i_03dd9e),
    (0x03DDA2, i_03dda2),
    (0x03DDAA, i_03ddaa),
    (0x03DDAE, i_03ddae),
    (0x03DDB0, i_03ddb0),
    (0x03DDB8, i_03ddb8),
    (0x03DDBC, i_03ddbc),
    (0x03DDC0, i_03ddc0),
    (0x03DDC4, i_03ddc4),
    (0x03DDC8, i_03ddc8),
    (0x03DDD0, i_03ddd0),
    (0x03DDD4, i_03ddd4),
    (0x03DDD6, i_03ddd6),
    (0x03DDDE, i_03ddde),
    (0x03DDE2, i_03dde2),
    (0x03DDE6, i_03dde6),
    (0x03DDEE, i_03ddee),
    (0x03DDF2, i_03ddf2),
    (0x03DDF4, i_03ddf4),
    (0x03DDFC, i_03ddfc),
    (0x03DE00, i_03de00),
    (0x03DE04, i_03de04),
    (0x03DE08, i_03de08),
    (0x03DE0C, i_03de0c),
    (0x03DE14, i_03de14),
    (0x03DE18, i_03de18),
    (0x03DE1A, i_03de1a),
    (0x03DE22, i_03de22),
    (0x03DE26, i_03de26),
    (0x03DE2A, i_03de2a),
    (0x03DE32, i_03de32),
    (0x03DE36, i_03de36),
    (0x03DE38, i_03de38),
    (0x03DE40, i_03de40),
    (0x03DE44, i_03de44),
    (0x03DE48, i_03de48),
    (0x03DE4C, i_03de4c),
    (0x03DE50, i_03de50),
    (0x03DE58, i_03de58),
    (0x03DE5C, i_03de5c),
    (0x03DE5E, i_03de5e),
    (0x03DE66, i_03de66),
    (0x03DE6A, i_03de6a),
    (0x03DE6E, i_03de6e),
    (0x03DE76, i_03de76),
    (0x03DE7A, i_03de7a),
    (0x03DE7C, i_03de7c),
    (0x03DE84, i_03de84),
    (0x03DE88, i_03de88),
    (0x03DE8C, i_03de8c),
    (0x03DE90, i_03de90),
    (0x03DE94, i_03de94),
    (0x03DE9A, i_03de9a),
    (0x03DEA0, i_03dea0),
    (0x03DEA4, i_03dea4),
    (0x03DEA6, i_03dea6),
    (0x03DEA8, i_03dea8),
    (0x03DEAC, i_03deac),
    (0x03DEB2, i_03deb2),
    (0x03DEB6, i_03deb6),
    (0x03DEBA, i_03deba),
    (0x03DEBC, i_03debc),
    (0x03DEBE, i_03debe),
    (0x03DEC2, i_03dec2),
    (0x03DEC6, i_03dec6),
    (0x03DECA, i_03deca),
    (0x03DECE, i_03dece),
    (0x03DED4, i_03ded4),
    (0x03DEDA, i_03deda),
    (0x03DEDE, i_03dede),
    (0x03DEE0, i_03dee0),
    (0x03DEE2, i_03dee2),
    (0x03DEE6, i_03dee6),
    (0x03DEEC, i_03deec),
    (0x03DEF0, i_03def0),
    (0x03DEF4, i_03def4),
    (0x03DEF8, i_03def8),
    (0x03DEFC, i_03defc),
    (0x03DF00, i_03df00),
    (0x03DF04, i_03df04),
    (0x03DF0A, i_03df0a),
    (0x03DF10, i_03df10),
    (0x03DF16, i_03df16),
    (0x03DF18, i_03df18),
    (0x03DF1E, i_03df1e),
    (0x03DF22, i_03df22),
    (0x03DF24, i_03df24),
    (0x03DF2A, i_03df2a),
    (0x03DF2E, i_03df2e),
    (0x03DF30, i_03df30),
    (0x03DF32, i_03df32),
    (0x03DF36, i_03df36),
    (0x03DF3C, i_03df3c),
    (0x03DF40, i_03df40),
    (0x03DF46, i_03df46),
    (0x03DF4C, i_03df4c),
    (0x03DF50, i_03df50),
    (0x03DF52, i_03df52),
    (0x03DF54, i_03df54),
    (0x03DF56, i_03df56),
    (0x03DF5A, i_03df5a),
    (0x03DF5E, i_03df5e),
    (0x03DF64, i_03df64),
    (0x03DF6A, i_03df6a),
    (0x03DF70, i_03df70),
    (0x03DF74, i_03df74),
    (0x03DF7A, i_03df7a),
    (0x03DF7E, i_03df7e),
    (0x03DF84, i_03df84),
    (0x03DF8A, i_03df8a),
    (0x03DF90, i_03df90),
    (0x03DF96, i_03df96),
    (0x03DF9C, i_03df9c),
    (0x03DFA2, i_03dfa2),
    (0x03DFA6, i_03dfa6),
    (0x03DFAC, i_03dfac),
    (0x03DFB4, i_03dfb4),
    (0x03DFB8, i_03dfb8),
    (0x03DFBE, i_03dfbe),
    (0x03DFC4, i_03dfc4),
    (0x03DFC8, i_03dfc8),
    (0x03DFCC, i_03dfcc),
    (0x03DFD2, i_03dfd2),
    (0x03DFD8, i_03dfd8),
    (0x03E05E, i_03e05e),
    (0x03E064, i_03e064),
    (0x03E06A, i_03e06a),
    (0x03E070, i_03e070),
    (0x03E076, i_03e076),
    (0x03E07C, i_03e07c),
    (0x03E080, i_03e080),
    (0x03E082, i_03e082),
    (0x03E084, i_03e084),
    (0x03E088, i_03e088),
    (0x03E08E, i_03e08e),
    (0x03E092, i_03e092),
    (0x03E098, i_03e098),
    (0x03E09C, i_03e09c),
    (0x03E0A2, i_03e0a2),
    (0x03E0A6, i_03e0a6),
    (0x03E0AC, i_03e0ac),
    (0x03E0B0, i_03e0b0),
    (0x03E0B4, i_03e0b4),
    (0x03E0B8, i_03e0b8),
    (0x03E0BC, i_03e0bc),
    (0x03E0C2, i_03e0c2),
    (0x03E0C6, i_03e0c6),
    (0x03E0C8, i_03e0c8),
    (0x03E0CE, i_03e0ce),
    (0x03E0D0, i_03e0d0),
    (0x03E0D4, i_03e0d4),
    (0x03E0DA, i_03e0da),
    (0x03E0DE, i_03e0de),
    (0x03E0E4, i_03e0e4),
    (0x03E0E8, i_03e0e8),
    (0x03E0EE, i_03e0ee),
    (0x03E0F2, i_03e0f2),
    (0x03E0F8, i_03e0f8),
    (0x03E0FC, i_03e0fc),
    (0x03E102, i_03e102),
    (0x03E106, i_03e106),
    (0x03E10A, i_03e10a),
    (0x03E10E, i_03e10e),
    (0x03E112, i_03e112),
    (0x03E118, i_03e118),
    (0x03E11C, i_03e11c),
    (0x03E11E, i_03e11e),
    (0x03E124, i_03e124),
    (0x03E126, i_03e126),
    (0x03E12A, i_03e12a),
    (0x03E130, i_03e130),
    (0x03E134, i_03e134),
    (0x03E13C, i_03e13c),
    (0x03E140, i_03e140),
    (0x03E148, i_03e148),
    (0x03E14C, i_03e14c),
    (0x03E152, i_03e152),
    (0x03E158, i_03e158),
    (0x03E15E, i_03e15e),
    (0x03E162, i_03e162),
    (0x03E166, i_03e166),
    (0x03E16A, i_03e16a),
    (0x03E170, i_03e170),
    (0x03E176, i_03e176),
    (0x03E17C, i_03e17c),
    (0x03E17E, i_03e17e),
    (0x03E182, i_03e182),
    (0x03E188, i_03e188),
    (0x03E18C, i_03e18c),
    (0x03E192, i_03e192),
    (0x03E196, i_03e196),
    (0x03E198, i_03e198),
    (0x03E19E, i_03e19e),
    (0x03E1A0, i_03e1a0),
    (0x03E1A4, i_03e1a4),
    (0x03E1AA, i_03e1aa),
    (0x03E1AE, i_03e1ae),
    (0x03E1B4, i_03e1b4),
    (0x03E1B8, i_03e1b8),
    (0x03E1BE, i_03e1be),
    (0x03E1C2, i_03e1c2),
    (0x03E1C8, i_03e1c8),
    (0x03E1CC, i_03e1cc),
    (0x03E1D0, i_03e1d0),
    (0x03E1D4, i_03e1d4),
    (0x03E1D8, i_03e1d8),
    (0x03E1DE, i_03e1de),
    (0x03E1E4, i_03e1e4),
    (0x03E1E8, i_03e1e8),
    (0x03E1EC, i_03e1ec),
    (0x03E1F0, i_03e1f0),
    (0x03E1F4, i_03e1f4),
    (0x03E1FA, i_03e1fa),
    (0x03E1FE, i_03e1fe),
    (0x03E200, i_03e200),
    (0x03E206, i_03e206),
    (0x03E208, i_03e208),
    (0x03E20C, i_03e20c),
    (0x03E212, i_03e212),
    (0x03E216, i_03e216),
    (0x03E21C, i_03e21c),
    (0x03E220, i_03e220),
    (0x03E226, i_03e226),
    (0x03E22A, i_03e22a),
    (0x03E230, i_03e230),
    (0x03E234, i_03e234),
    (0x03E23A, i_03e23a),
    (0x03E23E, i_03e23e),
    (0x03E242, i_03e242),
    (0x03E246, i_03e246),
    (0x03E24C, i_03e24c),
    (0x03E252, i_03e252),
    (0x03E258, i_03e258),
    (0x03E25C, i_03e25c),
    (0x03E260, i_03e260),
    (0x03E264, i_03e264),
    (0x03E26A, i_03e26a),
    (0x03E26E, i_03e26e),
    (0x03E272, i_03e272),
    (0x03E276, i_03e276),
    (0x03E27C, i_03e27c),
    (0x03E280, i_03e280),
    (0x03E284, i_03e284),
    (0x03E288, i_03e288),
    (0x03E28E, i_03e28e),
    (0x03E292, i_03e292),
    (0x03E296, i_03e296),
    (0x03E29A, i_03e29a),
    (0x03E29E, i_03e29e),
    (0x03E2A2, i_03e2a2),
    (0x03E2A6, i_03e2a6),
    (0x03E2AA, i_03e2aa),
    (0x03E2AC, i_03e2ac),
    (0x03E2B2, i_03e2b2),
    (0x03E2B4, i_03e2b4),
    (0x03E2BA, i_03e2ba),
    (0x03E2BE, i_03e2be),
    (0x03E2C0, i_03e2c0),
    (0x03E2C4, i_03e2c4),
    (0x03E2CA, i_03e2ca),
    (0x03E2CE, i_03e2ce),
    (0x03E2D2, i_03e2d2),
    (0x03E2D4, i_03e2d4),
    (0x03E2DA, i_03e2da),
    (0x03E2DC, i_03e2dc),
    (0x03E2E0, i_03e2e0),
    (0x03E2E6, i_03e2e6),
    (0x03E2EA, i_03e2ea),
    (0x03E2F0, i_03e2f0),
    (0x03E2F4, i_03e2f4),
    (0x03E2F6, i_03e2f6),
    (0x03E2FC, i_03e2fc),
    (0x03E2FE, i_03e2fe),
    (0x03E302, i_03e302),
    (0x03E308, i_03e308),
    (0x03E30C, i_03e30c),
    (0x03E312, i_03e312),
    (0x03E318, i_03e318),
    (0x03E31C, i_03e31c),
    (0x03E322, i_03e322),
    (0x03E328, i_03e328),
    (0x03E32C, i_03e32c),
    (0x03E330, i_03e330),
    (0x03E334, i_03e334),
    (0x03E338, i_03e338),
    (0x03E33E, i_03e33e),
    (0x03E344, i_03e344),
    (0x03E348, i_03e348),
    (0x03E34C, i_03e34c),
    (0x03E350, i_03e350),
    (0x03E354, i_03e354),
    (0x03E356, i_03e356),
    (0x03E35A, i_03e35a),
    (0x03E362, i_03e362),
    (0x03E366, i_03e366),
    (0x03E368, i_03e368),
    (0x03E370, i_03e370),
    (0x03E374, i_03e374),
    (0x03E378, i_03e378),
    (0x03E380, i_03e380),
    (0x03E384, i_03e384),
    (0x03E386, i_03e386),
    (0x03E38E, i_03e38e),
    (0x03E392, i_03e392),
    (0x03E396, i_03e396),
    (0x03E39A, i_03e39a),
    (0x03E39E, i_03e39e),
    (0x03E3A0, i_03e3a0),
    (0x03E3A4, i_03e3a4),
    (0x03E3AC, i_03e3ac),
    (0x03E3B0, i_03e3b0),
    (0x03E3B2, i_03e3b2),
    (0x03E3BA, i_03e3ba),
    (0x03E3BE, i_03e3be),
    (0x03E3C2, i_03e3c2),
    (0x03E3CA, i_03e3ca),
    (0x03E3CE, i_03e3ce),
    (0x03E3D0, i_03e3d0),
    (0x03E3D8, i_03e3d8),
    (0x03E3DC, i_03e3dc),
    (0x03E3E0, i_03e3e0),
    (0x03E3E2, i_03e3e2),
    (0x03E3E6, i_03e3e6),
    (0x03E3EC, i_03e3ec),
    (0x03E3F0, i_03e3f0),
    (0x03E3F4, i_03e3f4),
    (0x03E3F6, i_03e3f6),
    (0x03E3FC, i_03e3fc),
    (0x03E402, i_03e402),
    (0x03E408, i_03e408),
    (0x03E40A, i_03e40a),
    (0x03E410, i_03e410),
    (0x03E414, i_03e414),
    (0x03E418, i_03e418),
    (0x03E41E, i_03e41e),
    (0x03E424, i_03e424),
    (0x03E42A, i_03e42a),
    (0x03E430, i_03e430),
    (0x03E434, i_03e434),
    (0x03E436, i_03e436),
    (0x03E438, i_03e438),
    (0x03E43C, i_03e43c),
    (0x03E442, i_03e442),
    (0x03E446, i_03e446),
    (0x03E44A, i_03e44a),
    (0x03E44E, i_03e44e),
    (0x03E452, i_03e452),
    (0x03E456, i_03e456),
    (0x03E45A, i_03e45a),
    (0x03E460, i_03e460),
    (0x03E466, i_03e466),
    (0x03E46C, i_03e46c),
    (0x03E472, i_03e472),
    (0x03E476, i_03e476),
    (0x03E478, i_03e478),
    (0x03E47A, i_03e47a),
    (0x03E47E, i_03e47e),
    (0x03E482, i_03e482),
    (0x03E488, i_03e488),
    (0x03E48C, i_03e48c),
    (0x03E48E, i_03e48e),
    (0x03E490, i_03e490),
    (0x03E494, i_03e494),
    (0x03E49A, i_03e49a),
    (0x03E49E, i_03e49e),
    (0x03E4A4, i_03e4a4),
    (0x03E4AA, i_03e4aa),
    (0x03E4AE, i_03e4ae),
    (0x03E4B0, i_03e4b0),
    (0x03E4B2, i_03e4b2),
    (0x03E4B6, i_03e4b6),
    (0x03E4BC, i_03e4bc),
    (0x03E4C2, i_03e4c2),
    (0x03E4C8, i_03e4c8),
    (0x03E4CE, i_03e4ce),
    (0x03E4D4, i_03e4d4),
    (0x03E4DA, i_03e4da),
    (0x03E4E0, i_03e4e0),
    (0x03E4E4, i_03e4e4),
    (0x03E4E6, i_03e4e6),
    (0x03E4EA, i_03e4ea),
    (0x03E4F0, i_03e4f0),
    (0x03E4F4, i_03e4f4),
    (0x03E4F8, i_03e4f8),
    (0x03E4FE, i_03e4fe),
    (0x03E502, i_03e502),
    (0x03E504, i_03e504),
    (0x03E50A, i_03e50a),
    (0x03E50C, i_03e50c),
    (0x03E512, i_03e512),
    (0x03E514, i_03e514),
    (0x03E518, i_03e518),
    (0x03E51C, i_03e51c),
    (0x03E520, i_03e520),
    (0x03E524, i_03e524),
    (0x03E528, i_03e528),
    (0x03E52C, i_03e52c),
    (0x03E530, i_03e530),
    (0x03E534, i_03e534),
    (0x03E538, i_03e538),
    (0x03E53C, i_03e53c),
    (0x03E540, i_03e540),
    (0x03E544, i_03e544),
    (0x03E548, i_03e548),
    (0x03E54C, i_03e54c),
    (0x03E550, i_03e550),
    (0x03E552, i_03e552),
    (0x03E556, i_03e556),
    (0x03E55C, i_03e55c),
    (0x03E560, i_03e560),
    (0x03E564, i_03e564),
    (0x03E56A, i_03e56a),
    (0x03E56E, i_03e56e),
    (0x03E574, i_03e574),
    (0x03E57A, i_03e57a),
    (0x03E580, i_03e580),
    (0x03E586, i_03e586),
    (0x03E58C, i_03e58c),
    (0x03E592, i_03e592),
    (0x03E598, i_03e598),
    (0x03E59E, i_03e59e),
    (0x03E5A4, i_03e5a4),
    (0x03E5AA, i_03e5aa),
    (0x03E5B0, i_03e5b0),
    (0x03E5B6, i_03e5b6),
    (0x03E5BC, i_03e5bc),
    (0x03E5C2, i_03e5c2),
    (0x03E5C8, i_03e5c8),
    (0x03E5CE, i_03e5ce),
    (0x03E5D4, i_03e5d4),
    (0x03E5DA, i_03e5da),
    (0x03E5E0, i_03e5e0),
    (0x03E5E6, i_03e5e6),
    (0x03E5EC, i_03e5ec),
    (0x03E5F4, i_03e5f4),
    (0x03E5F8, i_03e5f8),
    (0x03E5FC, i_03e5fc),
    (0x03E600, i_03e600),
    (0x03E604, i_03e604),
    (0x03E608, i_03e608),
    (0x03E60E, i_03e60e),
    (0x03E612, i_03e612),
    (0x03E618, i_03e618),
    (0x03E61E, i_03e61e),
    (0x03E624, i_03e624),
    (0x03E628, i_03e628),
    (0x03E62C, i_03e62c),
    (0x03E632, i_03e632),
    (0x03E636, i_03e636),
    (0x03E63A, i_03e63a),
    (0x03E63E, i_03e63e),
    (0x03E642, i_03e642),
    (0x03E646, i_03e646),
    (0x03E64C, i_03e64c),
    (0x03E650, i_03e650),
    (0x03E654, i_03e654),
    (0x03E658, i_03e658),
    (0x03E65C, i_03e65c),
    (0x03E660, i_03e660),
    (0x03E664, i_03e664),
    (0x03E668, i_03e668),
    (0x03E66C, i_03e66c),
    (0x03E670, i_03e670),
    (0x03E674, i_03e674),
    (0x03E678, i_03e678),
    (0x03E67C, i_03e67c),
    (0x03E680, i_03e680),
    (0x03E684, i_03e684),
    (0x03E688, i_03e688),
    (0x03E68C, i_03e68c),
    (0x03E690, i_03e690),
    (0x03E694, i_03e694),
    (0x03E698, i_03e698),
    (0x03E69C, i_03e69c),
    (0x03E6A0, i_03e6a0),
    (0x03E6A4, i_03e6a4),
    (0x03E6A8, i_03e6a8),
    (0x03E6AC, i_03e6ac),
    (0x03E6B0, i_03e6b0),
    (0x03E6B4, i_03e6b4),
    (0x03E6B8, i_03e6b8),
    (0x03E6BC, i_03e6bc),
    (0x03E6C0, i_03e6c0),
    (0x03E6C4, i_03e6c4),
    (0x03E6C8, i_03e6c8),
    (0x03E6CC, i_03e6cc),
    (0x03E6D0, i_03e6d0),
    (0x03E6D4, i_03e6d4),
    (0x03E6D8, i_03e6d8),
    (0x03E6DA, i_03e6da),
    (0x03E6DE, i_03e6de),
    (0x03E6E4, i_03e6e4),
    (0x03E6E8, i_03e6e8),
    (0x03E6EC, i_03e6ec),
    (0x03E6F0, i_03e6f0),
    (0x03E6F4, i_03e6f4),
    (0x03E6FA, i_03e6fa),
    (0x03E700, i_03e700),
    (0x03E702, i_03e702),
    (0x03E706, i_03e706),
    (0x03E70A, i_03e70a),
    (0x03E70E, i_03e70e),
    (0x03E712, i_03e712),
    (0x03E714, i_03e714),
    (0x03E716, i_03e716),
    (0x03E718, i_03e718),
    (0x03E71E, i_03e71e),
    (0x03E722, i_03e722),
    (0x03E726, i_03e726),
    (0x03E72C, i_03e72c),
    (0x03E72E, i_03e72e),
    (0x03E730, i_03e730),
    (0x03E734, i_03e734),
    (0x03E736, i_03e736),
    (0x03E73A, i_03e73a),
    (0x03E73E, i_03e73e),
    (0x03E740, i_03e740),
    (0x03E744, i_03e744),
    (0x03E74A, i_03e74a),
    (0x03E74E, i_03e74e),
    (0x03E752, i_03e752),
    (0x03E756, i_03e756),
    (0x03E75A, i_03e75a),
    (0x03E760, i_03e760),
    (0x03E766, i_03e766),
    (0x03E768, i_03e768),
    (0x03E76C, i_03e76c),
    (0x03E770, i_03e770),
    (0x03E774, i_03e774),
    (0x03E778, i_03e778),
    (0x03E77A, i_03e77a),
    (0x03E77C, i_03e77c),
    (0x03E77E, i_03e77e),
    (0x03E784, i_03e784),
    (0x03E788, i_03e788),
    (0x03E78C, i_03e78c),
    (0x03E792, i_03e792),
    (0x03E794, i_03e794),
    (0x03E796, i_03e796),
    (0x03E79A, i_03e79a),
    (0x03E79C, i_03e79c),
    (0x03E7A0, i_03e7a0),
    (0x03E7A4, i_03e7a4),
    (0x03E7A6, i_03e7a6),
    (0x03E7AE, i_03e7ae),
    (0x03E7B2, i_03e7b2),
    (0x03E7B6, i_03e7b6),
    (0x03E7BA, i_03e7ba),
    (0x03E7BE, i_03e7be),
    (0x03E7C0, i_03e7c0),
    (0x03E7C6, i_03e7c6),
    (0x03E7CA, i_03e7ca),
    (0x03E7CC, i_03e7cc),
    (0x03E7D2, i_03e7d2),
    (0x03E7D4, i_03e7d4),
    (0x03E7D6, i_03e7d6),
    (0x03E7D8, i_03e7d8),
    (0x03E7DC, i_03e7dc),
    (0x03E7E2, i_03e7e2),
    (0x03E7E6, i_03e7e6),
    (0x03E7EA, i_03e7ea),
    (0x03E7EC, i_03e7ec),
    (0x03E7EE, i_03e7ee),
    (0x03E7F2, i_03e7f2),
    (0x03E7F6, i_03e7f6),
    (0x03E7FA, i_03e7fa),
    (0x03E7FE, i_03e7fe),
    (0x03E802, i_03e802),
    (0x03E806, i_03e806),
    (0x03E80E, i_03e80e),
    (0x03E812, i_03e812),
    (0x03E814, i_03e814),
    (0x03E818, i_03e818),
    (0x03E81A, i_03e81a),
    (0x03E81E, i_03e81e),
    (0x03E826, i_03e826),
    (0x03E82A, i_03e82a),
    (0x03E82C, i_03e82c),
    (0x03E832, i_03e832),
    (0x03E834, i_03e834),
    (0x03E836, i_03e836),
    (0x03E838, i_03e838),
    (0x03E83C, i_03e83c),
    (0x03E840, i_03e840),
    (0x03E844, i_03e844),
    (0x03E84A, i_03e84a),
    (0x03E84C, i_03e84c),
    (0x03E852, i_03e852),
    (0x03E856, i_03e856),
    (0x03E858, i_03e858),
    (0x03E85E, i_03e85e),
    (0x03E862, i_03e862),
    (0x03E866, i_03e866),
    (0x03E86C, i_03e86c),
    (0x03E86E, i_03e86e),
    (0x03E870, i_03e870),
    (0x03E872, i_03e872),
    (0x03E874, i_03e874),
    (0x03E878, i_03e878),
    (0x03E87A, i_03e87a),
    (0x03E882, i_03e882),
    (0x03E886, i_03e886),
    (0x03E88A, i_03e88a),
    (0x03E88C, i_03e88c),
    (0x03E894, i_03e894),
    (0x03E898, i_03e898),
    (0x03E89C, i_03e89c),
    (0x03E89E, i_03e89e),
    (0x03E8A6, i_03e8a6),
    (0x03E8AA, i_03e8aa),
    (0x03E8AE, i_03e8ae),
    (0x03E8B0, i_03e8b0),
    (0x03E8B8, i_03e8b8),
    (0x03E8BC, i_03e8bc),
    (0x03E8C0, i_03e8c0),
    (0x03E8C2, i_03e8c2),
    (0x03E8CA, i_03e8ca),
    (0x03E8CE, i_03e8ce),
    (0x03E8D0, i_03e8d0),
    (0x03E8D4, i_03e8d4),
    (0x03E8D6, i_03e8d6),
    (0x03E8D8, i_03e8d8),
    (0x03E8DE, i_03e8de),
    (0x03E8E0, i_03e8e0),
    (0x03E8E4, i_03e8e4),
    (0x03E8E8, i_03e8e8),
    (0x03E8EC, i_03e8ec),
    (0x03E8F2, i_03e8f2),
    (0x03E8F4, i_03e8f4),
    (0x03E8F8, i_03e8f8),
    (0x03E8FA, i_03e8fa),
    (0x03E900, i_03e900),
    (0x03E902, i_03e902),
    (0x03E906, i_03e906),
    (0x03E90A, i_03e90a),
    (0x03E90E, i_03e90e),
    (0x03E914, i_03e914),
    (0x03E916, i_03e916),
    (0x03E91A, i_03e91a),
    (0x03E9D4, i_03e9d4),
    (0x03E9DA, i_03e9da),
    (0x03E9DC, i_03e9dc),
    (0x03E9E2, i_03e9e2),
    (0x03E9E8, i_03e9e8),
    (0x03E9EC, i_03e9ec),
    (0x03E9F0, i_03e9f0),
    (0x03E9F2, i_03e9f2),
    (0x03E9F8, i_03e9f8),
    (0x03E9FA, i_03e9fa),
    (0x03E9FE, i_03e9fe),
    (0x03EA04, i_03ea04),
    (0x03EA08, i_03ea08),
    (0x03EA0E, i_03ea0e),
    (0x03EA12, i_03ea12),
    (0x03EA16, i_03ea16),
    (0x03EA1A, i_03ea1a),
    (0x03EA20, i_03ea20),
    (0x03EA26, i_03ea26),
    (0x03EA2E, i_03ea2e),
    (0x03EA34, i_03ea34),
    (0x03EA3A, i_03ea3a),
    (0x03EA40, i_03ea40),
    (0x03EA44, i_03ea44),
    (0x03EA46, i_03ea46),
    (0x03EA4A, i_03ea4a),
    (0x03EA50, i_03ea50),
    (0x03EA54, i_03ea54),
    (0x03EA5A, i_03ea5a),
    (0x03EA5E, i_03ea5e),
    (0x03EA62, i_03ea62),
    (0x03EA64, i_03ea64),
    (0x03EA66, i_03ea66),
    (0x03EA6C, i_03ea6c),
    (0x03EA70, i_03ea70),
    (0x03EA76, i_03ea76),
    (0x03EA7A, i_03ea7a),
    (0x03EA80, i_03ea80),
    (0x03EA84, i_03ea84),
    (0x03EA88, i_03ea88),
    (0x03EA8C, i_03ea8c),
    (0x03EA92, i_03ea92),
    (0x03EA98, i_03ea98),
    (0x03EA9C, i_03ea9c),
    (0x03EAA0, i_03eaa0),
    (0x03EAA2, i_03eaa2),
    (0x03EAA6, i_03eaa6),
    (0x03EAAC, i_03eaac),
    (0x03EAB2, i_03eab2),
    (0x03EAB8, i_03eab8),
    (0x03EABC, i_03eabc),
    (0x03EAC0, i_03eac0),
    (0x03EAC4, i_03eac4),
    (0x03EAC6, i_03eac6),
    (0x03EACA, i_03eaca),
    (0x03EAD2, i_03ead2),
    (0x03EAD6, i_03ead6),
    (0x03EAD8, i_03ead8),
    (0x03EAE0, i_03eae0),
    (0x03EAE4, i_03eae4),
    (0x03EAE8, i_03eae8),
    (0x03EAF0, i_03eaf0),
    (0x03EAF4, i_03eaf4),
    (0x03EAF6, i_03eaf6),
    (0x03EAFE, i_03eafe),
    (0x03EB02, i_03eb02),
    (0x03EB08, i_03eb08),
    (0x03EB0C, i_03eb0c),
    (0x03EB10, i_03eb10),
    (0x03EB16, i_03eb16),
    (0x03EB1A, i_03eb1a),
    (0x03EB1E, i_03eb1e),
    (0x03EB22, i_03eb22),
    (0x03EB26, i_03eb26),
    (0x03EB28, i_03eb28),
    (0x03EB2C, i_03eb2c),
    (0x03EB34, i_03eb34),
    (0x03EB38, i_03eb38),
    (0x03EB3A, i_03eb3a),
    (0x03EB42, i_03eb42),
    (0x03EB46, i_03eb46),
    (0x03EB4A, i_03eb4a),
    (0x03EB52, i_03eb52),
    (0x03EB56, i_03eb56),
    (0x03EB58, i_03eb58),
    (0x03EB60, i_03eb60),
    (0x03EB64, i_03eb64),
    (0x03EB68, i_03eb68),
    (0x03EB6C, i_03eb6c),
    (0x03EB70, i_03eb70),
    (0x03EB76, i_03eb76),
    (0x03EB7C, i_03eb7c),
    (0x03EB80, i_03eb80),
    (0x03EB82, i_03eb82),
    (0x03EB84, i_03eb84),
    (0x03EB8A, i_03eb8a),
    (0x03EB90, i_03eb90),
    (0x03EB96, i_03eb96),
    (0x03EB9A, i_03eb9a),
    (0x03EB9C, i_03eb9c),
    (0x03EBA0, i_03eba0),
    (0x03EBA6, i_03eba6),
    (0x03EBAA, i_03ebaa),
    (0x03EBB0, i_03ebb0),
    (0x03EBB4, i_03ebb4),
    (0x03EBB8, i_03ebb8),
    (0x03EBBA, i_03ebba),
    (0x03EBBC, i_03ebbc),
    (0x03EBC2, i_03ebc2),
    (0x03EBC6, i_03ebc6),
    (0x03EBCC, i_03ebcc),
    (0x03EBD0, i_03ebd0),
    (0x03EBD6, i_03ebd6),
    (0x03EBDA, i_03ebda),
    (0x03EBDE, i_03ebde),
    (0x03EBE2, i_03ebe2),
    (0x03EBE8, i_03ebe8),
    (0x03EBEE, i_03ebee),
    (0x03EBF2, i_03ebf2),
    (0x03EBF6, i_03ebf6),
    (0x03EBF8, i_03ebf8),
    (0x03EBFC, i_03ebfc),
    (0x03EC00, i_03ec00),
    (0x03EC04, i_03ec04),
    (0x03EC0A, i_03ec0a),
    (0x03EC10, i_03ec10),
    (0x03EC14, i_03ec14),
    (0x03EC16, i_03ec16),
    (0x03EC18, i_03ec18),
    (0x03EC1E, i_03ec1e),
    (0x03EC24, i_03ec24),
    (0x03EC2A, i_03ec2a),
    (0x03EC2E, i_03ec2e),
    (0x03EC30, i_03ec30),
    (0x03EC34, i_03ec34),
    (0x03EC3A, i_03ec3a),
    (0x03EC3E, i_03ec3e),
    (0x03EC44, i_03ec44),
    (0x03EC48, i_03ec48),
];
