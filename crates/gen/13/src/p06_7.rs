// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $06B9BA: move.w #$4000, $38(a0)
pub fn i_06b9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9C0;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B9C0: lea.l $77fd6.l, a1
pub fn i_06b9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9C6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06B9C6: jsr $4ae.l
pub fn i_06b9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9CC);
    cpu.pc = 0x4AE;
    16
}
/// $06B9CC: jsr $5dd02.l
pub fn i_06b9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9D2);
    cpu.pc = 0x5DD02;
    16
}
/// $06B9D2: move.w #$4000, $38(a0)
pub fn i_06b9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9D8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B9D8: rts
pub fn i_06b9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_DeathExplodeB_06b9da  ($06B9DA, 54 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06B9DA: move.w #$1021, d0
pub fn i_06b9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06B9DE: jsr $2352.l
pub fn i_06b9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9E4);
    cpu.pc = 0x2352;
    16
}
/// $06B9E4: lea.l $2ca3f2.l, a1
pub fn i_06b9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CA3F2;
    20
}
/// $06B9EA: jsr $77c7e.l
pub fn i_06b9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6B9F0);
    cpu.pc = 0x77C7E;
    16
}
/// $06B9F0: move.w #$4000, $38(a0)
pub fn i_06b9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9F6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06B9F6: lea.l $77fd6.l, a1
pub fn i_06b9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6B9FC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06B9FC: jsr $4ae.l
pub fn i_06b9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA02);
    cpu.pc = 0x4AE;
    16
}
/// $06BA02: jsr $5dd02.l
pub fn i_06ba02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA08);
    cpu.pc = 0x5DD02;
    16
}
/// $06BA08: move.w #$4000, $38(a0)
pub fn i_06ba08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA0E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BA0E: rts
pub fn i_06ba0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bazooka_CmpSlotWithParent_06ba10  ($06BA10, 16 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BA10: movea.l $8(a6), a1
pub fn i_06ba10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA14;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06BA14: move.b $10(a6), d0
pub fn i_06ba14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA18;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06BA18: cmp.b $10(a1), d0
pub fn i_06ba18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA1C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BA1C: bcs.w $6ba26
pub fn i_06ba1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA20;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BA26; return 10; }
    8
}
// ===== ClearXN_06ba20  ($06BA20, 6 B, thunks/ccr_helpers.c)
/// $06BA20: andi.b #$ee, ccr
pub fn i_06ba20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA24;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06BA24: rts
pub fn i_06ba24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06ba26  ($06BA26, 6 B, thunks/ccr_helpers.c)
/// $06BA26: ori.b #$11, ccr
pub fn i_06ba26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA2A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06BA2A: rts
pub fn i_06ba2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_Tmpl59_06ba2c  ($06BA2C, 214 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BA2C: clr.b $9e(a6)
pub fn i_06ba2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA30;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BA30: bra.w $6ba3a
pub fn i_06ba30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA34;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BA3A;
    10
}
/// $06BA34: move.b #$1, $9e(a6)
pub fn i_06ba34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BA3A: jsr $283ca.l
pub fn i_06ba3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA40);
    cpu.pc = 0x283CA;
    16
}
/// $06BA40: jsr $5e7c0.l
pub fn i_06ba40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA46);
    cpu.pc = 0x5E7C0;
    16
}
/// $06BA46: move.b $99(a6), d0
pub fn i_06ba46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA4A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06BA4A: andi.b #$1, d0
pub fn i_06ba4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA4E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06BA4E: move.b d0, $3a(a6)
pub fn i_06ba4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA52;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BA52: move.b d0, $8d(a6)
pub fn i_06ba52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA56;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BA56: jsr $6d446(pc)
pub fn i_06ba56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BA5A);
    cpu.pc = 0x6D446;
    16
}
/// $06BA5A: move.b $9a(a6), $96(a6)
pub fn i_06ba5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA60;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x96);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06BA60: jsr $6d574(pc)
pub fn i_06ba60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BA64);
    cpu.pc = 0x6D574;
    16
}
/// $06BA64: lea.l $2bd468.l, a0
pub fn i_06ba64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD468;
    20
}
/// $06BA6A: jsr $799de.l
pub fn i_06ba6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA70);
    cpu.pc = 0x799DE;
    16
}
/// $06BA70: move.w d0, $36(a6)
pub fn i_06ba70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA74;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BA74: lea.l $2bd4ea.l, a0
pub fn i_06ba74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD4EA;
    20
}
/// $06BA7A: jsr $799de.l
pub fn i_06ba7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA80);
    cpu.pc = 0x799DE;
    16
}
/// $06BA80: move.w d0, $70(a6)
pub fn i_06ba80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA84;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BA84: lea.l $2bd3e6.l, a0
pub fn i_06ba84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD3E6;
    20
}
/// $06BA8A: jsr $799de.l
pub fn i_06ba8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA90);
    cpu.pc = 0x799DE;
    16
}
/// $06BA90: move.w d0, $66(a6)
pub fn i_06ba90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA94;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BA94: move.w #$8000, d0
pub fn i_06ba94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06BA98: jsr $28134.l
pub fn i_06ba98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BA9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BA9E);
    cpu.pc = 0x28134;
    16
}
/// $06BA9E: andi.w #$ffe3, $38(a6)
pub fn i_06ba9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAA4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06BAA4: ori.w #$14, $38(a6)
pub fn i_06baa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAAA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06BAAA: move.l #$2cc7a6, $60(a6)
pub fn i_06baaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAB2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CC7A6;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06BAB2: move.l #$2cc8d6, $48(a6)
pub fn i_06bab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BABA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CC8D6;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06BABA: clr.b $89(a6)
pub fn i_06baba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BABE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x89);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BABE: move.w #$8, $8a(a6)
pub fn i_06babe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAC4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BAC4: move.b #$ff, $97(a6)
pub fn i_06bac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BACA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BACA: move.w $24(a6), $82(a6)
pub fn i_06baca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAD0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06BAD0: jsr $30696.l
pub fn i_06bad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BAD6);
    cpu.pc = 0x30696;
    16
}
/// $06BAD6: lea.l $6c562(pc), a1
pub fn i_06bad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BADA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C562;
    16
}
/// $06BADA: jsr $4ae.l
pub fn i_06bada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BAE0);
    cpu.pc = 0x4AE;
    16
}
/// $06BAE0: jsr $5dd02.l
pub fn i_06bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BAE6);
    cpu.pc = 0x5DD02;
    16
}
/// $06BAE6: move.b $9f(a6), $9f(a0)
pub fn i_06bae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAEC;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x9F);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06BAEC: move.b $98(a6), $98(a0)
pub fn i_06baec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAF2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06BAF2: move.b $9a(a6), $96(a0)
pub fn i_06baf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAF8;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x96);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06BAF8: bset.b #$2, $6b(a6)
pub fn i_06baf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BAFE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BAFE: bra.w $6bee0
pub fn i_06bafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB02;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BEE0;
    10
}
// ===== RocketVehicle_Drive_06bb02  ($06BB02, 86 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BB02: move.b #$0, $72(a6)
pub fn i_06bb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB08;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BB08: move.w $36(a6), d0
pub fn i_06bb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB0C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06BB0C: btst.b #$0, $8d(a6)
pub fn i_06bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB12;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06BB12: bne.w $6bb18
pub fn i_06bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB16;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6BB18; return 10; }
    8
}
/// $06BB16: neg.w d0
pub fn i_06bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB18;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06BB18: move.w d0, $28(a6)
pub fn i_06bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB1C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06BB1C: jsr $6d206(pc)
pub fn i_06bb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB20);
    cpu.pc = 0x6D206;
    16
}
/// $06BB20: bcs.w $6bb6c
pub fn i_06bb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB24;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BB6C; return 10; }
    8
}
/// $06BB24: jsr $6d1dc(pc)
pub fn i_06bb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB28;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB28);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BB28: bcs.w $6c272
pub fn i_06bb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB2C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BB2C: move.b d0, $97(a6)
pub fn i_06bb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB30;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BB30: andi.w #$3, d0
pub fn i_06bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB34;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BB34: movea.l #$6bb58, a0
pub fn i_06bb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB3A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BB58;
    cpu.a[0] = v1;
    4
}
/// $06BB3A: lsl.w #$2, d0
pub fn i_06bb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB3C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BB3C: movea.l (a0, d0.w), a0
pub fn i_06bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB40;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BB40: cmpa.l #$ffffffff, a0
pub fn i_06bb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB46;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BB46: beq.w $6bb50
pub fn i_06bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BB50; return 10; }
    8
}
/// $06BB4A: jsr $28cd4.l
pub fn i_06bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BB50);
    cpu.pc = 0x28CD4;
    16
}
/// $06BB50: jsr $6d484(pc)
pub fn i_06bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB54);
    cpu.pc = 0x6D484;
    16
}
/// $06BB54: bra.w $6bb68
pub fn i_06bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB58;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BB68;
    10
}
/// $06BB68: bra.w $6bbb0
pub fn i_06bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BBB0;
    10
}
/// $06BB6C: jsr $6d1dc(pc)
pub fn i_06bb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB70);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BB70: bcs.w $6c272
pub fn i_06bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB74;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BB74: move.b d0, $97(a6)
pub fn i_06bb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB78;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BB78: andi.w #$3, d0
pub fn i_06bb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB7C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BB7C: movea.l #$6bba0, a0
pub fn i_06bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB82;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BBA0;
    cpu.a[0] = v1;
    4
}
/// $06BB82: lsl.w #$2, d0
pub fn i_06bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB84;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BB84: movea.l (a0, d0.w), a0
pub fn i_06bb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB88;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BB88: cmpa.l #$ffffffff, a0
pub fn i_06bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB8E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BB8E: beq.w $6bb98
pub fn i_06bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BB98; return 10; }
    8
}
/// $06BB92: jsr $28cd4.l
pub fn i_06bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BB98);
    cpu.pc = 0x28CD4;
    16
}
/// $06BB98: jsr $6d484(pc)
pub fn i_06bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BB9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BB9C);
    cpu.pc = 0x6D484;
    16
}
/// $06BB9C: bra.w $6bbb0
pub fn i_06bb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBA0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BBB0;
    10
}
/// $06BBB0: lea.l $6bbb6(pc), a1
pub fn i_06bbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BBB6;
    16
}
/// $06BBB4: move.l a1, (a6)
pub fn i_06bbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BBB6: jsr $28998.l
pub fn i_06bbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BBBC);
    cpu.pc = 0x28998;
    16
}
/// $06BBBC: jsr $6d0f6(pc)
pub fn i_06bbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BBC0);
    cpu.pc = 0x6D0F6;
    16
}
/// $06BBC0: bcc.w $6bbd0
pub fn i_06bbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BBD0; return 10; }
    8
}
/// $06BBC4: eori.b #$1, $8d(a6)
pub fn i_06bbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBCA;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BBCA: lea.l $6bcfc(pc), a1
pub fn i_06bbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BBCE: move.l a1, (a6)
pub fn i_06bbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BBD0: jsr $6d1dc(pc)
pub fn i_06bbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BBD4);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BBD4: bcs.w $6c272
pub fn i_06bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBD8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BBD8: cmp.b $97(a6), d0
pub fn i_06bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBDC;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BBDC: beq.w $6bc74
pub fn i_06bbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBE0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BC74; return 10; }
    8
}
/// $06BBE0: jsr $6d206(pc)
pub fn i_06bbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BBE4);
    cpu.pc = 0x6D206;
    16
}
/// $06BBE4: bcs.w $6bc30
pub fn i_06bbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBE8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BC30; return 10; }
    8
}
/// $06BBE8: jsr $6d1dc(pc)
pub fn i_06bbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBEC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BBEC);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BBEC: bcs.w $6c272
pub fn i_06bbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBF0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BBF0: move.b d0, $97(a6)
pub fn i_06bbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBF4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BBF4: andi.w #$3, d0
pub fn i_06bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BBF8: movea.l #$6bc1c, a0
pub fn i_06bbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BBFE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BC1C;
    cpu.a[0] = v1;
    4
}
/// $06BBFE: lsl.w #$2, d0
pub fn i_06bbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC00;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BC00: movea.l (a0, d0.w), a0
pub fn i_06bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC04;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BC04: cmpa.l #$ffffffff, a0
pub fn i_06bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC0A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BC0A: beq.w $6bc14
pub fn i_06bc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC0E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BC14; return 10; }
    8
}
/// $06BC0E: jsr $28cd4.l
pub fn i_06bc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BC14);
    cpu.pc = 0x28CD4;
    16
}
/// $06BC14: jsr $6d484(pc)
pub fn i_06bc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BC18);
    cpu.pc = 0x6D484;
    16
}
/// $06BC18: bra.w $6bc2c
pub fn i_06bc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BC2C;
    10
}
/// $06BC2C: bra.w $6bc74
pub fn i_06bc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC30;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BC74;
    10
}
/// $06BC30: jsr $6d1dc(pc)
pub fn i_06bc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BC34);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BC34: bcs.w $6c272
pub fn i_06bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC38;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BC38: move.b d0, $97(a6)
pub fn i_06bc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC3C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BC3C: andi.w #$3, d0
pub fn i_06bc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC40;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BC40: movea.l #$6bc64, a0
pub fn i_06bc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC46;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BC64;
    cpu.a[0] = v1;
    4
}
/// $06BC46: lsl.w #$2, d0
pub fn i_06bc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC48;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BC48: movea.l (a0, d0.w), a0
pub fn i_06bc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC4C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BC4C: cmpa.l #$ffffffff, a0
pub fn i_06bc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC52;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BC52: beq.w $6bc5c
pub fn i_06bc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BC5C; return 10; }
    8
}
/// $06BC56: jsr $28cd4.l
pub fn i_06bc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BC5C);
    cpu.pc = 0x28CD4;
    16
}
/// $06BC5C: jsr $6d484(pc)
pub fn i_06bc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BC60);
    cpu.pc = 0x6D484;
    16
}
/// $06BC60: bra.w $6bc74
pub fn i_06bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC64;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BC74;
    10
}
/// $06BC74: jsr $28d70.l
pub fn i_06bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BC7A);
    cpu.pc = 0x28D70;
    16
}
/// $06BC7A: jsr $6d172(pc)
pub fn i_06bc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC7E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BC7E);
    cpu.pc = 0x6D172;
    16
}
/// $06BC7E: bcc.w $6bc92
pub fn i_06bc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BC92; return 10; }
    8
}
/// $06BC82: eori.b #$1, $8d(a6)
pub fn i_06bc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC88;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BC88: lea.l $6bcfc(pc), a1
pub fn i_06bc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BC8C: move.l a1, (a6)
pub fn i_06bc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BC8E: bra.w $6bcda
pub fn i_06bc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC92;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BCDA;
    10
}
/// $06BC92: cmpi.w #$30, $22(a6)
pub fn i_06bc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC98;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $06BC98: bgt.w $6bcb6
pub fn i_06bc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BC9C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6BCB6; return 10; }
    8
}
/// $06BC9C: btst.b #$0, $8d(a6)
pub fn i_06bc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCA2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06BCA2: bne.w $6bcb6
pub fn i_06bca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCA6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6BCB6; return 10; }
    8
}
/// $06BCA6: ori.b #$1, $8d(a6)
pub fn i_06bca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCAC;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BCAC: clr.w $28(a6)
pub fn i_06bcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCB0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BCB0: lea.l $6bcfc(pc), a1
pub fn i_06bcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BCB4: move.l a1, (a6)
pub fn i_06bcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BCB6: cmpi.w #$110, $22(a6)
pub fn i_06bcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCBC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x110, Size::Word, false);
    16
}
/// $06BCBC: blt.w $6bcda
pub fn i_06bcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCC0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6BCDA; return 10; }
    8
}
/// $06BCC0: btst.b #$0, $8d(a6)
pub fn i_06bcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCC6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06BCC6: beq.w $6bcda
pub fn i_06bcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCCA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BCDA; return 10; }
    8
}
/// $06BCCA: andi.b #$fe, $8d(a6)
pub fn i_06bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCD0;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BCD0: clr.w $28(a6)
pub fn i_06bcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCD4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BCD4: lea.l $6bcfc(pc), a1
pub fn i_06bcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BCD8: move.l a1, (a6)
pub fn i_06bcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BCDA: jsr $6d228(pc)
pub fn i_06bcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BCDE);
    cpu.pc = 0x6D228;
    16
}
/// $06BCDE: bcc.w $6bcec
pub fn i_06bcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BCEC; return 10; }
    8
}
/// $06BCE2: clr.w $28(a6)
pub fn i_06bce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCE6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BCE6: lea.l $6bcfc(pc), a1
pub fn i_06bce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BCEA: move.l a1, (a6)
pub fn i_06bcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BCEC: jsr $283ca.l
pub fn i_06bcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BCF2);
    cpu.pc = 0x283CA;
    16
}
/// $06BCF2: jsr $283d8.l
pub fn i_06bcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BCF8);
    cpu.pc = 0x283D8;
    16
}
/// $06BCF8: bra.w $6c418
pub fn i_06bcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BCFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Stop_06bcfc  ($06BCFC, 74 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BCFC: move.b #$0, $72(a6)
pub fn i_06bcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD02;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BD02: jsr $6d1dc(pc)
pub fn i_06bd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD06);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BD06: bcs.w $6c272
pub fn i_06bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD0A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BD0A: jsr $6d206(pc)
pub fn i_06bd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD0E);
    cpu.pc = 0x6D206;
    16
}
/// $06BD0E: bcc.w $6bd5a
pub fn i_06bd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BD5A; return 10; }
    8
}
/// $06BD12: jsr $6d1dc(pc)
pub fn i_06bd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD16);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BD16: bcs.w $6c272
pub fn i_06bd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD1A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BD1A: move.b d0, $97(a6)
pub fn i_06bd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD1E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BD1E: andi.w #$3, d0
pub fn i_06bd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD22;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BD22: movea.l #$6bd46, a0
pub fn i_06bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD28;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BD46;
    cpu.a[0] = v1;
    4
}
/// $06BD28: lsl.w #$2, d0
pub fn i_06bd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD2A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BD2A: movea.l (a0, d0.w), a0
pub fn i_06bd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD2E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BD2E: cmpa.l #$ffffffff, a0
pub fn i_06bd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD34;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BD34: beq.w $6bd3e
pub fn i_06bd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BD3E; return 10; }
    8
}
/// $06BD38: jsr $28cd4.l
pub fn i_06bd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BD3E);
    cpu.pc = 0x28CD4;
    16
}
/// $06BD3E: jsr $6d484(pc)
pub fn i_06bd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD42);
    cpu.pc = 0x6D484;
    16
}
/// $06BD42: bra.w $6bd56
pub fn i_06bd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD46;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BD56;
    10
}
/// $06BD56: bra.w $6bd9e
pub fn i_06bd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BD9E;
    10
}
/// $06BD5A: jsr $6d1dc(pc)
pub fn i_06bd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD5E);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BD5E: bcs.w $6c272
pub fn i_06bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD62;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BD62: move.b d0, $97(a6)
pub fn i_06bd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD66;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BD66: andi.w #$3, d0
pub fn i_06bd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD6A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BD6A: movea.l #$6bd8e, a0
pub fn i_06bd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD70;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BD8E;
    cpu.a[0] = v1;
    4
}
/// $06BD70: lsl.w #$2, d0
pub fn i_06bd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD72;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BD72: movea.l (a0, d0.w), a0
pub fn i_06bd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD76;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BD76: cmpa.l #$ffffffff, a0
pub fn i_06bd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD7C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BD7C: beq.w $6bd86
pub fn i_06bd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BD86; return 10; }
    8
}
/// $06BD80: jsr $28cd4.l
pub fn i_06bd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BD86);
    cpu.pc = 0x28CD4;
    16
}
/// $06BD86: jsr $6d484(pc)
pub fn i_06bd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD8A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BD8A);
    cpu.pc = 0x6D484;
    16
}
/// $06BD8A: bra.w $6bd9e
pub fn i_06bd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BD8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BD9E;
    10
}
/// $06BD9E: clr.w $28(a6)
pub fn i_06bd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDA2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BDA2: lea.l $6bda8(pc), a1
pub fn i_06bda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDA6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BDA8;
    16
}
/// $06BDA6: move.l a1, (a6)
pub fn i_06bda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDA8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BDA8: jsr $28998.l
pub fn i_06bda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BDAE);
    cpu.pc = 0x28998;
    16
}
/// $06BDAE: jsr $6d0f6(pc)
pub fn i_06bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BDB2);
    cpu.pc = 0x6D0F6;
    16
}
/// $06BDB2: jsr $6d1dc(pc)
pub fn i_06bdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDB6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BDB6);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BDB6: bcs.w $6bdc2
pub fn i_06bdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDBA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BDC2; return 10; }
    8
}
/// $06BDBA: cmp.b $97(a6), d0
pub fn i_06bdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDBE;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BDBE: beq.w $6bdc8
pub fn i_06bdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BDC8; return 10; }
    8
}
/// $06BDC2: lea.l $6bee0(pc), a1
pub fn i_06bdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BEE0;
    16
}
/// $06BDC6: move.l a1, (a6)
pub fn i_06bdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BDC8: jsr $28d70.l
pub fn i_06bdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BDCE);
    cpu.pc = 0x28D70;
    16
}
/// $06BDCE: bcc.w $6bdd8
pub fn i_06bdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDD2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BDD8; return 10; }
    8
}
/// $06BDD2: lea.l $6bee0(pc), a1
pub fn i_06bdd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BEE0;
    16
}
/// $06BDD6: move.l a1, (a6)
pub fn i_06bdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BDD8: jsr $283ca.l
pub fn i_06bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BDDE);
    cpu.pc = 0x283CA;
    16
}
/// $06BDDE: jsr $283d8.l
pub fn i_06bdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BDE4);
    cpu.pc = 0x283D8;
    16
}
/// $06BDE4: bra.w $6c418
pub fn i_06bde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Turn_06bde8  ($06BDE8, 74 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BDE8: move.b #$0, $72(a6)
pub fn i_06bde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDEE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BDEE: jsr $6d1dc(pc)
pub fn i_06bdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BDF2);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BDF2: bcs.w $6c272
pub fn i_06bdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BDF6: jsr $6d206(pc)
pub fn i_06bdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BDFA);
    cpu.pc = 0x6D206;
    16
}
/// $06BDFA: bcs.w $6be46
pub fn i_06bdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BDFE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BE46; return 10; }
    8
}
/// $06BDFE: jsr $6d1dc(pc)
pub fn i_06bdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE02);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BE02: bcs.w $6c272
pub fn i_06be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE06;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BE06: move.b d0, $97(a6)
pub fn i_06be06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE0A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BE0A: andi.w #$3, d0
pub fn i_06be0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE0E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BE0E: movea.l #$6be32, a0
pub fn i_06be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE14;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BE32;
    cpu.a[0] = v1;
    4
}
/// $06BE14: lsl.w #$2, d0
pub fn i_06be14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE16;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BE16: movea.l (a0, d0.w), a0
pub fn i_06be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE1A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BE1A: cmpa.l #$ffffffff, a0
pub fn i_06be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE20;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BE20: beq.w $6be2a
pub fn i_06be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BE2A; return 10; }
    8
}
/// $06BE24: jsr $28cd4.l
pub fn i_06be24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BE2A);
    cpu.pc = 0x28CD4;
    16
}
/// $06BE2A: jsr $6d484(pc)
pub fn i_06be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE2E);
    cpu.pc = 0x6D484;
    16
}
/// $06BE2E: bra.w $6be42
pub fn i_06be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE32;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BE42;
    10
}
/// $06BE42: bra.w $6be8a
pub fn i_06be42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE46;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BE8A;
    10
}
/// $06BE46: jsr $6d1dc(pc)
pub fn i_06be46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE4A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BE4A: bcs.w $6c272
pub fn i_06be4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE4E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BE4E: move.b d0, $97(a6)
pub fn i_06be4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE52;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BE52: andi.w #$3, d0
pub fn i_06be52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE56;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BE56: movea.l #$6be7a, a0
pub fn i_06be56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE5C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BE7A;
    cpu.a[0] = v1;
    4
}
/// $06BE5C: lsl.w #$2, d0
pub fn i_06be5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE5E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BE5E: movea.l (a0, d0.w), a0
pub fn i_06be5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE62;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BE62: cmpa.l #$ffffffff, a0
pub fn i_06be62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE68;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BE68: beq.w $6be72
pub fn i_06be68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BE72; return 10; }
    8
}
/// $06BE6C: jsr $28cd4.l
pub fn i_06be6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BE72);
    cpu.pc = 0x28CD4;
    16
}
/// $06BE72: jsr $6d484(pc)
pub fn i_06be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE76;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE76);
    cpu.pc = 0x6D484;
    16
}
/// $06BE76: bra.w $6be8a
pub fn i_06be76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BE8A;
    10
}
/// $06BE8A: lea.l $6be90(pc), a1
pub fn i_06be8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BE90;
    16
}
/// $06BE8E: move.l a1, (a6)
pub fn i_06be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BE90: jsr $28998.l
pub fn i_06be90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BE96);
    cpu.pc = 0x28998;
    16
}
/// $06BE96: jsr $6d0f6(pc)
pub fn i_06be96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BE9A);
    cpu.pc = 0x6D0F6;
    16
}
/// $06BE9A: bcc.w $6beaa
pub fn i_06be9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BE9E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BEAA; return 10; }
    8
}
/// $06BE9E: eori.b #$1, $8d(a6)
pub fn i_06be9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEA4;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06BEA4: lea.l $6bcfc(pc), a1
pub fn i_06bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BCFC;
    16
}
/// $06BEA8: move.l a1, (a6)
pub fn i_06bea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BEAA: jsr $6d1dc(pc)
pub fn i_06beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BEAE);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BEAE: bcs.w $6beba
pub fn i_06beae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEB2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BEBA; return 10; }
    8
}
/// $06BEB2: cmp.b $97(a6), d0
pub fn i_06beb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEB6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BEB6: beq.w $6bec0
pub fn i_06beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BEC0; return 10; }
    8
}
/// $06BEBA: lea.l $6bee0(pc), a1
pub fn i_06beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BEE0;
    16
}
/// $06BEBE: move.l a1, (a6)
pub fn i_06bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BEC0: jsr $28d70.l
pub fn i_06bec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BEC6);
    cpu.pc = 0x28D70;
    16
}
/// $06BEC6: bcc.w $6bed0
pub fn i_06bec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BECA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BED0; return 10; }
    8
}
/// $06BECA: lea.l $6bb02(pc), a1
pub fn i_06beca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BECE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BB02;
    16
}
/// $06BECE: move.l a1, (a6)
pub fn i_06bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BED0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BED0: jsr $283ca.l
pub fn i_06bed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BED6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BED6);
    cpu.pc = 0x283CA;
    16
}
/// $06BED6: jsr $283d8.l
pub fn i_06bed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BEDC);
    cpu.pc = 0x283D8;
    16
}
/// $06BEDC: bra.w $6c418
pub fn i_06bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Idle_06bee0  ($06BEE0, 86 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BEE0: move.b #$0, $72(a6)
pub fn i_06bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BEE6: lea.l $2bd670.l, a0
pub fn i_06bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD670;
    20
}
/// $06BEEC: jsr $799de.l
pub fn i_06beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BEF2);
    cpu.pc = 0x799DE;
    16
}
/// $06BEF2: move.b d0, $88(a6)
pub fn i_06bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEF6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BEF6: jsr $6d1dc(pc)
pub fn i_06bef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BEFA);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BEFA: bcs.w $6c272
pub fn i_06befa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BEFE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BEFE: clr.w $28(a6)
pub fn i_06befe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06BF02: jsr $6d1dc(pc)
pub fn i_06bf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF06);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BF06: bcs.w $6c272
pub fn i_06bf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF0A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BF0A: move.b d0, $97(a6)
pub fn i_06bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF0E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BF0E: andi.w #$3, d0
pub fn i_06bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF12;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BF12: movea.l #$6bf36, a0
pub fn i_06bf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF18;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BF36;
    cpu.a[0] = v1;
    4
}
/// $06BF18: lsl.w #$2, d0
pub fn i_06bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF1A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BF1A: movea.l (a0, d0.w), a0
pub fn i_06bf1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF1E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BF1E: cmpa.l #$ffffffff, a0
pub fn i_06bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF24;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BF24: beq.w $6bf2e
pub fn i_06bf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF28;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BF2E; return 10; }
    8
}
/// $06BF28: jsr $28cd4.l
pub fn i_06bf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BF2E);
    cpu.pc = 0x28CD4;
    16
}
/// $06BF2E: jsr $6d484(pc)
pub fn i_06bf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF32);
    cpu.pc = 0x6D484;
    16
}
/// $06BF32: bra.w $6bf46
pub fn i_06bf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF36;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BF46;
    10
}
/// $06BF46: lea.l $6bf4c(pc), a1
pub fn i_06bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BF4C;
    16
}
/// $06BF4A: move.l a1, (a6)
pub fn i_06bf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BF4C: jsr $28998.l
pub fn i_06bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BF52);
    cpu.pc = 0x28998;
    16
}
/// $06BF52: jsr $6d0f6(pc)
pub fn i_06bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF56);
    cpu.pc = 0x6D0F6;
    16
}
/// $06BF56: jsr $6d1dc(pc)
pub fn i_06bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF5A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BF5A: bcs.w $6bf66
pub fn i_06bf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF5E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6BF66; return 10; }
    8
}
/// $06BF5E: cmp.b $97(a6), d0
pub fn i_06bf5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF62;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06BF62: beq.w $6bfaa
pub fn i_06bf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BFAA; return 10; }
    8
}
/// $06BF66: jsr $6d1dc(pc)
pub fn i_06bf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF6A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BF6A: bcs.w $6c272
pub fn i_06bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF6E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BF6E: move.b d0, $97(a6)
pub fn i_06bf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF72;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BF72: andi.w #$3, d0
pub fn i_06bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF76;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06BF76: movea.l #$6bf9a, a0
pub fn i_06bf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF7C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6BF9A;
    cpu.a[0] = v1;
    4
}
/// $06BF7C: lsl.w #$2, d0
pub fn i_06bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF7E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06BF7E: movea.l (a0, d0.w), a0
pub fn i_06bf7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF82;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06BF82: cmpa.l #$ffffffff, a0
pub fn i_06bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF88;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06BF88: beq.w $6bf92
pub fn i_06bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6BF92; return 10; }
    8
}
/// $06BF8C: jsr $28cd4.l
pub fn i_06bf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BF92);
    cpu.pc = 0x28CD4;
    16
}
/// $06BF92: jsr $6d484(pc)
pub fn i_06bf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF96;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BF96);
    cpu.pc = 0x6D484;
    16
}
/// $06BF96: bra.w $6bfaa
pub fn i_06bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BF9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6BFAA;
    10
}
/// $06BFAA: jsr $28d70.l
pub fn i_06bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BFB0);
    cpu.pc = 0x28D70;
    16
}
/// $06BFB0: subq.b #$1, $88(a6)
pub fn i_06bfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFB4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BFB4: bgt.w $6bfbe
pub fn i_06bfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFB8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6BFBE; return 10; }
    8
}
/// $06BFB8: lea.l $6bde8(pc), a1
pub fn i_06bfb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BDE8;
    16
}
/// $06BFBC: move.l a1, (a6)
pub fn i_06bfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BFBE: jsr $6d228(pc)
pub fn i_06bfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BFC2);
    cpu.pc = 0x6D228;
    16
}
/// $06BFC2: bcc.w $6bfcc
pub fn i_06bfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6BFCC; return 10; }
    8
}
/// $06BFC6: lea.l $6bfd0(pc), a1
pub fn i_06bfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BFD0;
    16
}
/// $06BFCA: move.l a1, (a6)
pub fn i_06bfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06BFCC: bra.w $6c418
pub fn i_06bfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Alert_06bfd0  ($06BFD0, 90 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06BFD0: jsr $267e2.l
pub fn i_06bfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BFD6);
    cpu.pc = 0x267E2;
    16
}
/// $06BFD6: jsr $5e0d4.l
pub fn i_06bfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6BFDC);
    cpu.pc = 0x5E0D4;
    16
}
/// $06BFDC: move.l a0, $90(a6)
pub fn i_06bfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFE0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06BFE0: move.b #$2, $72(a6)
pub fn i_06bfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06BFE6: clr.b $88(a6)
pub fn i_06bfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFEA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BFEA: clr.b $73(a6)
pub fn i_06bfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFEE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06BFEE: jsr $6d1dc(pc)
pub fn i_06bfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BFF2);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BFF2: bcs.w $6c272
pub fn i_06bff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BFF6: jsr $6d1dc(pc)
pub fn i_06bff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFFA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6BFFA);
    cpu.pc = 0x6D1DC;
    16
}
/// $06BFFA: bcs.w $6c272
pub fn i_06bffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6BFFE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06BFFE: move.b d0, $97(a6)
pub fn i_06bffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C002;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C002: andi.w #$3, d0
pub fn i_06c002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C006;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C006: movea.l #$6c02a, a0
pub fn i_06c006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C00C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C02A;
    cpu.a[0] = v1;
    4
}
/// $06C00C: lsl.w #$2, d0
pub fn i_06c00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C00E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C00E: movea.l (a0, d0.w), a0
pub fn i_06c00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C012;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C012: cmpa.l #$ffffffff, a0
pub fn i_06c012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C018;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C018: beq.w $6c022
pub fn i_06c018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C01C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C022; return 10; }
    8
}
/// $06C01C: jsr $28cd4.l
pub fn i_06c01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C022);
    cpu.pc = 0x28CD4;
    16
}
/// $06C022: jsr $6d484(pc)
pub fn i_06c022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C026;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C026);
    cpu.pc = 0x6D484;
    16
}
/// $06C026: bra.w $6c03a
pub fn i_06c026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C02A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C03A;
    10
}
/// $06C03A: lea.l $6c040(pc), a1
pub fn i_06c03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C03E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C040;
    16
}
/// $06C03E: move.l a1, (a6)
pub fn i_06c03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C040;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C040: jsr $28998.l
pub fn i_06c040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C046;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C046);
    cpu.pc = 0x28998;
    16
}
/// $06C046: jsr $6d0f6(pc)
pub fn i_06c046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C04A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C04A);
    cpu.pc = 0x6D0F6;
    16
}
/// $06C04A: jsr $28d70.l
pub fn i_06c04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C050;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C050);
    cpu.pc = 0x28D70;
    16
}
/// $06C050: tst.b $73(a6)
pub fn i_06c050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C054;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06C054: beq.w $6c05e
pub fn i_06c054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C058;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C05E; return 10; }
    8
}
/// $06C058: lea.l $6c062(pc), a1
pub fn i_06c058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C05C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C062;
    16
}
/// $06C05C: move.l a1, (a6)
pub fn i_06c05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C05E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C05E: bra.w $6c418
pub fn i_06c05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C062;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_BurstCount_06c062  ($06C062, 20 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C062: lea.l $2bd5ee.l, a0
pub fn i_06c062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C068;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD5EE;
    20
}
/// $06C068: jsr $799de.l
pub fn i_06c068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C06E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C06E);
    cpu.pc = 0x799DE;
    16
}
/// $06C06E: move.b d0, $88(a6)
pub fn i_06c06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C072;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C072: bra.w $6c084
pub fn i_06c072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C076;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C084;
    10
}
// ===== RocketVehicle_Attack_06c076  ($06C076, 92 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C076: subq.b #$1, $88(a6)
pub fn i_06c076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C07A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C07A: cmpi.b #$0, $88(a6)
pub fn i_06c07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C080;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06C080: ble.w $6c122
pub fn i_06c080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C084;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6C122; return 10; }
    8
}
/// $06C084: clr.b $73(a6)
pub fn i_06c084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C088;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C088: move.b #$3, $72(a6)
pub fn i_06c088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C08E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C08E: lea.l $2bd56c.l, a0
pub fn i_06c08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C094;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD56C;
    20
}
/// $06C094: jsr $799de.l
pub fn i_06c094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C09A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C09A);
    cpu.pc = 0x799DE;
    16
}
/// $06C09A: move.w d0, $70(a6)
pub fn i_06c09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C09E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06C09E: jsr $6d1dc(pc)
pub fn i_06c09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C0A2);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C0A2: bcs.w $6c272
pub fn i_06c0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0A6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06C0A6: move.b d0, $97(a6)
pub fn i_06c0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0AA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C0AA: andi.w #$3, d0
pub fn i_06c0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0AE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C0AE: movea.l #$6c0d2, a0
pub fn i_06c0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C0D2;
    cpu.a[0] = v1;
    4
}
/// $06C0B4: lsl.w #$2, d0
pub fn i_06c0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0B6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C0B6: movea.l (a0, d0.w), a0
pub fn i_06c0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0BA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C0BA: cmpa.l #$ffffffff, a0
pub fn i_06c0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0C0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C0C0: beq.w $6c0ca
pub fn i_06c0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C0CA; return 10; }
    8
}
/// $06C0C4: jsr $28cd4.l
pub fn i_06c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C0CA);
    cpu.pc = 0x28CD4;
    16
}
/// $06C0CA: jsr $6d484(pc)
pub fn i_06c0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C0CE);
    cpu.pc = 0x6D484;
    16
}
/// $06C0CE: bra.w $6c0e2
pub fn i_06c0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C0E2;
    10
}
/// $06C0E2: lea.l $6c0e8(pc), a1
pub fn i_06c0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C0E8;
    16
}
/// $06C0E6: move.l a1, (a6)
pub fn i_06c0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C0E8: jsr $28998.l
pub fn i_06c0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C0EE);
    cpu.pc = 0x28998;
    16
}
/// $06C0EE: jsr $6d0f6(pc)
pub fn i_06c0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C0F2);
    cpu.pc = 0x6D0F6;
    16
}
/// $06C0F2: jsr $28d70.l
pub fn i_06c0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C0F8);
    cpu.pc = 0x28D70;
    16
}
/// $06C0F8: tst.b $73(a6)
pub fn i_06c0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C0FC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06C0FC: beq.w $6c10a
pub fn i_06c0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C100;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C10A; return 10; }
    8
}
/// $06C100: clr.b $73(a6)
pub fn i_06c100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C104;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C104: move.b #$5, $72(a6)
pub fn i_06c104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C10A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C10A: subq.w #$1, $70(a6)
pub fn i_06c10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C10E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06C10E: cmpi.w #$0, $70(a6)
pub fn i_06c10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C114;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06C114: bgt.w $6c11e
pub fn i_06c114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C118;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6C11E; return 10; }
    8
}
/// $06C118: lea.l $6c076(pc), a1
pub fn i_06c118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C11C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C076;
    16
}
/// $06C11C: move.l a1, (a6)
pub fn i_06c11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C11E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C11E: bra.w $6c418
pub fn i_06c11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C122;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
/// $06C122: lea.l $2bd4ea.l, a0
pub fn i_06c122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C128;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD4EA;
    20
}
/// $06C128: jsr $799de.l
pub fn i_06c128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C12E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C12E);
    cpu.pc = 0x799DE;
    16
}
/// $06C12E: move.w d0, $70(a6)
pub fn i_06c12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C132;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06C132: addi.w #$1e, $70(a6)
pub fn i_06c132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C138;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06C138: clr.b $88(a6)
pub fn i_06c138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C13C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C13C: clr.b $73(a6)
pub fn i_06c13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C140;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C140: jsr $6d1dc(pc)
pub fn i_06c140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C144;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C144);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C144: bcs.w $6c272
pub fn i_06c144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C148;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06C148: move.b #$4, $72(a6)
pub fn i_06c148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C14E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C14E: jsr $6d1dc(pc)
pub fn i_06c14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C152;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C152);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C152: bcs.w $6c272
pub fn i_06c152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C156;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06C156: move.b d0, $97(a6)
pub fn i_06c156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C15A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C15A: andi.w #$3, d0
pub fn i_06c15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C15E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C15E: movea.l #$6c182, a0
pub fn i_06c15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C164;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C182;
    cpu.a[0] = v1;
    4
}
/// $06C164: lsl.w #$2, d0
pub fn i_06c164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C166;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C166: movea.l (a0, d0.w), a0
pub fn i_06c166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C16A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C16A: cmpa.l #$ffffffff, a0
pub fn i_06c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C170;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C170: beq.w $6c17a
pub fn i_06c170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C174;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C17A; return 10; }
    8
}
/// $06C174: jsr $28cd4.l
pub fn i_06c174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C17A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C17A);
    cpu.pc = 0x28CD4;
    16
}
/// $06C17A: jsr $6d484(pc)
pub fn i_06c17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C17E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C17E);
    cpu.pc = 0x6D484;
    16
}
/// $06C17E: bra.w $6c192
pub fn i_06c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C182;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C192;
    10
}
/// $06C192: lea.l $6c198(pc), a1
pub fn i_06c192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C196;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C198;
    16
}
/// $06C196: move.l a1, (a6)
pub fn i_06c196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C198;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C198: jsr $28998.l
pub fn i_06c198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C19E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C19E);
    cpu.pc = 0x28998;
    16
}
/// $06C19E: jsr $6d0f6(pc)
pub fn i_06c19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C1A2);
    cpu.pc = 0x6D0F6;
    16
}
/// $06C1A2: jsr $28d70.l
pub fn i_06c1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C1A8);
    cpu.pc = 0x28D70;
    16
}
/// $06C1A8: tst.b $73(a6)
pub fn i_06c1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1AC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06C1AC: beq.w $6c1b6
pub fn i_06c1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C1B6; return 10; }
    8
}
/// $06C1B0: lea.l $6bde8(pc), a1
pub fn i_06c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BDE8;
    16
}
/// $06C1B4: move.l a1, (a6)
pub fn i_06c1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C1B6: bra.w $6c418
pub fn i_06c1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C418;
    10
}
// ===== RocketVehicle_Hurt_06c1ba  ($06C1BA, 76 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C1BA: bclr.b #$3, $13(a6)
pub fn i_06c1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1C0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C1C0: clr.w $28(a6)
pub fn i_06c1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1C4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06C1C4: jsr $6d1dc(pc)
pub fn i_06c1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C1C8);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C1C8: bcs.w $6c272
pub fn i_06c1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06C1CC: move.b #$6, $72(a6)
pub fn i_06c1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C1D2: jsr $6d1dc(pc)
pub fn i_06c1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C1D6);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C1D6: bcs.w $6c272
pub fn i_06c1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1DA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C272; return 10; }
    8
}
/// $06C1DA: move.b d0, $97(a6)
pub fn i_06c1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1DE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C1DE: andi.w #$3, d0
pub fn i_06c1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1E2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C1E2: movea.l #$6c206, a0
pub fn i_06c1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1E8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C206;
    cpu.a[0] = v1;
    4
}
/// $06C1E8: lsl.w #$2, d0
pub fn i_06c1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1EA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C1EA: movea.l (a0, d0.w), a0
pub fn i_06c1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1EE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C1EE: cmpa.l #$ffffffff, a0
pub fn i_06c1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1F4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C1F4: beq.w $6c1fe
pub fn i_06c1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C1FE; return 10; }
    8
}
/// $06C1F8: jsr $28cd4.l
pub fn i_06c1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C1FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C1FE);
    cpu.pc = 0x28CD4;
    16
}
/// $06C1FE: jsr $6d484(pc)
pub fn i_06c1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C202;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C202);
    cpu.pc = 0x6D484;
    16
}
/// $06C202: bra.w $6c216
pub fn i_06c202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C206;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C216;
    10
}
/// $06C216: lea.l $6c21c(pc), a1
pub fn i_06c216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C21A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C21C;
    16
}
/// $06C21A: move.l a1, (a6)
pub fn i_06c21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C21C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C21C: jsr $28998.l
pub fn i_06c21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C222);
    cpu.pc = 0x28998;
    16
}
/// $06C222: jsr $6d0f6(pc)
pub fn i_06c222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C226;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C226);
    cpu.pc = 0x6D0F6;
    16
}
/// $06C226: jsr $6d1dc(pc)
pub fn i_06c226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C22A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C22A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C22A: bcs.w $6c236
pub fn i_06c22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C22E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C236; return 10; }
    8
}
/// $06C22E: cmp.b $97(a6), d0
pub fn i_06c22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C232;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06C232: beq.w $6c23c
pub fn i_06c232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C236;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C23C; return 10; }
    8
}
/// $06C236: lea.l $6bee0(pc), a1
pub fn i_06c236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C23A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BEE0;
    16
}
/// $06C23A: move.l a1, (a6)
pub fn i_06c23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C23C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C23C: jsr $28d70.l
pub fn i_06c23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C242);
    cpu.pc = 0x28D70;
    16
}
/// $06C242: bcc.w $6c252
pub fn i_06c242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C246;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C252; return 10; }
    8
}
/// $06C246: bclr.b #$3, $13(a6)
pub fn i_06c246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C24C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C24C: lea.l $6bde8(pc), a1
pub fn i_06c24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C250;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BDE8;
    16
}
/// $06C250: move.l a1, (a6)
pub fn i_06c250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C252;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C252: jsr $2870a.l
pub fn i_06c252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C258;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C258);
    cpu.pc = 0x2870A;
    16
}
/// $06C258: bcc.w $6c26e
pub fn i_06c258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C25C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C26E; return 10; }
    8
}
/// $06C25C: lea.l $5e766.l, a0
pub fn i_06c25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C262;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06C262: jsr $5e770.l
pub fn i_06c262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C268;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C268);
    cpu.pc = 0x5E770;
    16
}
/// $06C268: bclr.b #$3, $13(a6)
pub fn i_06c268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C26E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C26E: bra.w $6c448
pub fn i_06c26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C272;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C448;
    10
}
// ===== RocketVehicle_Die_06c272  ($06C272, 266 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C272: move.w $36(a6), d0
pub fn i_06c272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C276;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06C276: btst.b #$0, $8d(a6)
pub fn i_06c276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C27C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06C27C: bne.w $6c282
pub fn i_06c27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C280;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6C282; return 10; }
    8
}
/// $06C280: neg.w d0
pub fn i_06c280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C282;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06C282: move.w d0, $28(a6)
pub fn i_06c282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C286;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06C286: move.b #$1, $72(a6)
pub fn i_06c286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C28C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C28C: move.l #$2cc9d2, $48(a6)
pub fn i_06c28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C294;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CC9D2;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C294: move.l #$2cc88a, $60(a6)
pub fn i_06c294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C29C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CC88A;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C29C: move.l #$2ccda2, d0
pub fn i_06c29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2A2;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2CCDA2;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06C2A2: tst.b $9e(a6)
pub fn i_06c2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2A6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06C2A6: beq.w $6c2b0
pub fn i_06c2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C2B0; return 10; }
    8
}
/// $06C2AA: move.l #$2cce42, d0
pub fn i_06c2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2B0;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2CCE42;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06C2B0: move.l d0, $4c(a6)
pub fn i_06c2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2B4;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C2B4: lea.l $2cd636.l, a0
pub fn i_06c2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD636;
    20
}
/// $06C2BA: jsr $28cd4.l
pub fn i_06c2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C2C0);
    cpu.pc = 0x28CD4;
    16
}
/// $06C2C0: lea.l $6c2c6(pc), a1
pub fn i_06c2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C2C6;
    16
}
/// $06C2C4: move.l a1, (a6)
pub fn i_06c2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C2C6: jsr $28998.l
pub fn i_06c2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C2CC);
    cpu.pc = 0x28998;
    16
}
/// $06C2CC: jsr $6d0f6(pc)
pub fn i_06c2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C2D0);
    cpu.pc = 0x6D0F6;
    16
}
/// $06C2D0: jsr $6d1dc(pc)
pub fn i_06c2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C2D4);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C2D4: bcs.w $6c2de
pub fn i_06c2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2D8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C2DE; return 10; }
    8
}
/// $06C2D8: lea.l $6bb02(pc), a1
pub fn i_06c2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BB02;
    16
}
/// $06C2DC: move.l a1, (a6)
pub fn i_06c2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C2DE: jsr $28d70.l
pub fn i_06c2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C2E4);
    cpu.pc = 0x28D70;
    16
}
/// $06C2E4: jsr $2870a.l
pub fn i_06c2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C2EA);
    cpu.pc = 0x2870A;
    16
}
/// $06C2EA: bcc.w $6c2f4
pub fn i_06c2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C2F4; return 10; }
    8
}
/// $06C2EE: bclr.b #$3, $13(a6)
pub fn i_06c2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C2F4: subq.w #$1, $70(a6)
pub fn i_06c2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2F8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06C2F8: jsr $6d13c(pc)
pub fn i_06c2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C2FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C2FC);
    cpu.pc = 0x6D13C;
    16
}
/// $06C2FC: bcc.w $6c310
pub fn i_06c2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C300;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C310; return 10; }
    8
}
/// $06C300: eori.b #$1, $8d(a6)
pub fn i_06c300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C306;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C306: lea.l $6c37c(pc), a1
pub fn i_06c306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C30A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C37C;
    16
}
/// $06C30A: move.l a1, (a6)
pub fn i_06c30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C30C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C30C: bra.w $6c36c
pub fn i_06c30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C310;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C36C;
    10
}
/// $06C310: jsr $6d172(pc)
pub fn i_06c310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C314;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C314);
    cpu.pc = 0x6D172;
    16
}
/// $06C314: bcc.w $6c328
pub fn i_06c314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C318;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C328; return 10; }
    8
}
/// $06C318: eori.b #$1, $8d(a6)
pub fn i_06c318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C31E;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C31E: lea.l $6c37c(pc), a1
pub fn i_06c31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C322;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C37C;
    16
}
/// $06C322: move.l a1, (a6)
pub fn i_06c322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C324;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C324: bra.w $6c36c
pub fn i_06c324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C328;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C36C;
    10
}
/// $06C328: cmpi.w #$10, $22(a6)
pub fn i_06c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C32E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $06C32E: bgt.w $6c34c
pub fn i_06c32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C332;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6C34C; return 10; }
    8
}
/// $06C332: btst.b #$0, $8d(a6)
pub fn i_06c332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C338;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06C338: bne.w $6c34c
pub fn i_06c338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C33C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6C34C; return 10; }
    8
}
/// $06C33C: ori.b #$1, $8d(a6)
pub fn i_06c33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C342;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C342: lea.l $6c37c(pc), a1
pub fn i_06c342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C346;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C37C;
    16
}
/// $06C346: move.l a1, (a6)
pub fn i_06c346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C348;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C348: bra.w $6c36c
pub fn i_06c348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C34C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C36C;
    10
}
/// $06C34C: cmpi.w #$130, $22(a6)
pub fn i_06c34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C352;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x130, Size::Word, false);
    16
}
/// $06C352: blt.w $6c36c
pub fn i_06c352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C356;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6C36C; return 10; }
    8
}
/// $06C356: btst.b #$0, $8d(a6)
pub fn i_06c356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C35C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06C35C: beq.w $6c36c
pub fn i_06c35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C360;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C36C; return 10; }
    8
}
/// $06C360: andi.b #$fe, $8d(a6)
pub fn i_06c360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C366;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C366: lea.l $6c37c(pc), a1
pub fn i_06c366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C36A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C37C;
    16
}
/// $06C36A: move.l a1, (a6)
pub fn i_06c36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C36C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C36C: jsr $283ca.l
pub fn i_06c36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C372;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C372);
    cpu.pc = 0x283CA;
    16
}
/// $06C372: jsr $283d8.l
pub fn i_06c372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C378);
    cpu.pc = 0x283D8;
    16
}
/// $06C378: bra.w $6c448
pub fn i_06c378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C37C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C448;
    10
}
// ===== RocketVehicle_Wreck_06c37c  ($06C37C, 228 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C37C: jsr $267e2.l
pub fn i_06c37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C382);
    cpu.pc = 0x267E2;
    16
}
/// $06C382: move.b #$1, $72(a6)
pub fn i_06c382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C388;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C388: move.l #$2cc9d2, $48(a6)
pub fn i_06c388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C390;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CC9D2;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C390: move.l #$2cc88a, $60(a6)
pub fn i_06c390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C398;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2CC88A;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C398: move.l #$2ccda2, d0
pub fn i_06c398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C39E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2CCDA2;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06C39E: tst.b $9e(a6)
pub fn i_06c39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3A2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06C3A2: beq.w $6c3ac
pub fn i_06c3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C3AC; return 10; }
    8
}
/// $06C3A6: move.l #$2cce42, d0
pub fn i_06c3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3AC;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2CCE42;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06C3AC: move.l d0, $4c(a6)
pub fn i_06c3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3B0;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C3B0: lea.l $2bd670.l, a0
pub fn i_06c3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD670;
    20
}
/// $06C3B6: jsr $799de.l
pub fn i_06c3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C3BC);
    cpu.pc = 0x799DE;
    16
}
/// $06C3BC: lsr.b #$1, d0
pub fn i_06c3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3BE;
    cpu.ir = 0xE208;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $06C3BE: move.b d0, $88(a6)
pub fn i_06c3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3C2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C3C2: lea.l $2cd636.l, a0
pub fn i_06c3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD636;
    20
}
/// $06C3C8: jsr $28cd4.l
pub fn i_06c3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C3CE);
    cpu.pc = 0x28CD4;
    16
}
/// $06C3CE: lea.l $6c3d4(pc), a1
pub fn i_06c3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C3D4;
    16
}
/// $06C3D2: move.l a1, (a6)
pub fn i_06c3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C3D4: jsr $28998.l
pub fn i_06c3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C3DA);
    cpu.pc = 0x28998;
    16
}
/// $06C3DA: jsr $6d0f6(pc)
pub fn i_06c3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C3DE);
    cpu.pc = 0x6D0F6;
    16
}
/// $06C3DE: jsr $6d1dc(pc)
pub fn i_06c3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C3E2);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C3E2: bcs.w $6c3ec
pub fn i_06c3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3E6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C3EC; return 10; }
    8
}
/// $06C3E6: lea.l $6bee0(pc), a1
pub fn i_06c3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6BEE0;
    16
}
/// $06C3EA: move.l a1, (a6)
pub fn i_06c3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C3EC: jsr $28d70.l
pub fn i_06c3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C3F2);
    cpu.pc = 0x28D70;
    16
}
/// $06C3F2: jsr $2870a.l
pub fn i_06c3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C3F8);
    cpu.pc = 0x2870A;
    16
}
/// $06C3F8: bcc.w $6c402
pub fn i_06c3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C3FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C402; return 10; }
    8
}
/// $06C3FC: bclr.b #$3, $13(a6)
pub fn i_06c3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C402;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C402: subq.w #$1, $70(a6)
pub fn i_06c402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C406;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06C406: subq.b #$1, $88(a6)
pub fn i_06c406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C40A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C40A: bgt.w $6c414
pub fn i_06c40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C40E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6C414; return 10; }
    8
}
/// $06C40E: lea.l $6c272(pc), a1
pub fn i_06c40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C412;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C272;
    16
}
/// $06C412: move.l a1, (a6)
pub fn i_06c412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C414;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C414: bra.w $6c448
pub fn i_06c414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C418;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C448;
    10
}
/// $06C418: clr.b $94(a6)
pub fn i_06c418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C41C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C41C: jsr $2870a.l
pub fn i_06c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C422;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C422);
    cpu.pc = 0x2870A;
    16
}
/// $06C422: bcc.w $6c448
pub fn i_06c422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C426;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C448; return 10; }
    8
}
/// $06C426: lea.l $5e766.l, a0
pub fn i_06c426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C42C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $06C42C: jsr $5e770.l
pub fn i_06c42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C432);
    cpu.pc = 0x5E770;
    16
}
/// $06C432: bclr.b #$3, $13(a6)
pub fn i_06c432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C438;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C438: move.b #$1, $94(a6)
pub fn i_06c438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C43E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C43E: bra.w $6c448
pub fn i_06c43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C442;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C448;
    10
}
/// $06C442: lea.l $6c1ba(pc), a1
pub fn i_06c442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C446;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C1BA;
    16
}
/// $06C446: move.l a1, (a6)
pub fn i_06c446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C448;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C448: jsr $28758.l
pub fn i_06c448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C44E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C44E);
    cpu.pc = 0x28758;
    16
}
/// $06C44E: bcc.w $6c458
pub fn i_06c44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C452;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C458; return 10; }
    8
}
/// $06C452: lea.l $6c468(pc), a1
pub fn i_06c452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C456;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C468;
    16
}
/// $06C456: move.l a1, (a6)
pub fn i_06c456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C458;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C458: jsr $6d05c(pc)
pub fn i_06c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C45C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C45C);
    cpu.pc = 0x6D05C;
    16
}
/// $06C45C: bcc.w $6c466
pub fn i_06c45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C460;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C466; return 10; }
    8
}
// ===== SetTaskHandler_06c460  ($06C460, 8 B, kernel/task_handlers.c)
/// $06C460: lea.l $6c4c6(pc), a1
pub fn i_06c460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C464;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C4C6;
    16
}
/// $06C464: move.l a1, (a6)
pub fn i_06c464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C466;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C466: rts
pub fn i_06c466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C468;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_Corpse_06c468  ($06C468, 86 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C468: bclr.b #$1, $12(a6)
pub fn i_06c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C46E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C46E: clr.w $28(a6)
pub fn i_06c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C472;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06C472: move.b #$7, $72(a6)
pub fn i_06c472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C478;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C478: jsr $6d1dc(pc)
pub fn i_06c478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C47C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C47C);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C47C: andi.w #$3, d0
pub fn i_06c47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C480;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C480: movea.l #$2ccfb6, a0
pub fn i_06c480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C486;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2CCFB6;
    cpu.a[0] = v1;
    4
}
/// $06C486: lsl.w #$2, d0
pub fn i_06c486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C488;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C488: movea.l (a0, d0.w), a0
pub fn i_06c488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C48C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C48C: cmpa.l #$ffffffff, a0
pub fn i_06c48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C492;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C492: beq.w $6c49c
pub fn i_06c492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C496;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C49C; return 10; }
    8
}
/// $06C496: jsr $28cd4.l
pub fn i_06c496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C49C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C49C);
    cpu.pc = 0x28CD4;
    16
}
/// $06C49C: lea.l $6c4a2(pc), a1
pub fn i_06c49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C4A2;
    16
}
/// $06C4A0: move.l a1, (a6)
pub fn i_06c4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4A2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C4A2: jsr $6d0f6(pc)
pub fn i_06c4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C4A6);
    cpu.pc = 0x6D0F6;
    16
}
/// $06C4A6: jsr $28d70.l
pub fn i_06c4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C4AC);
    cpu.pc = 0x28D70;
    16
}
/// $06C4AC: bcc.w $6c4b6
pub fn i_06c4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C4B6; return 10; }
    8
}
/// $06C4B0: lea.l $6c4c6(pc), a1
pub fn i_06c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C4C6;
    16
}
/// $06C4B4: move.l a1, (a6)
pub fn i_06c4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C4B6: jsr $6d05c(pc)
pub fn i_06c4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C4BA);
    cpu.pc = 0x6D05C;
    16
}
/// $06C4BA: bcc.w $6c4c4
pub fn i_06c4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C4C4; return 10; }
    8
}
// ===== SetTaskHandler_06c4be  ($06C4BE, 8 B, kernel/task_handlers.c)
/// $06C4BE: lea.l $6c4c6(pc), a1
pub fn i_06c4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C4C6;
    16
}
/// $06C4C2: move.l a1, (a6)
pub fn i_06c4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C4C4: rts
pub fn i_06c4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_DetachLinks_06c4c6  ($06C4C6, 112 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C4C6: move.l $74(a6), d0
pub fn i_06c4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4CA;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C4CA: cmpi.l #$ffffffff, d0
pub fn i_06c4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4D0;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $06C4D0: beq.w $6c4dc
pub fn i_06c4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C4DC; return 10; }
    8
}
/// $06C4D4: movea.l d0, a0
pub fn i_06c4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4D6;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06C4D6: move.l #$6c554, (a0)
pub fn i_06c4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4DC;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x6C554;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C4DC: move.l $78(a6), d0
pub fn i_06c4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4E0;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C4E0: cmpi.l #$ffffffff, d0
pub fn i_06c4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4E6;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $06C4E6: beq.w $6c4f2
pub fn i_06c4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C4F2; return 10; }
    8
}
/// $06C4EA: movea.l d0, a0
pub fn i_06c4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4EC;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06C4EC: move.l #$6c554, (a0)
pub fn i_06c4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4F2;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x6C554;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C4F2: move.l $7c(a6), d0
pub fn i_06c4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4F6;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C4F6: cmpi.l #$ffffffff, d0
pub fn i_06c4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C4FC;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $06C4FC: beq.w $6c508
pub fn i_06c4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C500;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C508; return 10; }
    8
}
/// $06C500: movea.l d0, a0
pub fn i_06c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C502;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06C502: move.l #$6c554, (a0)
pub fn i_06c502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C508;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x6C554;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C508: move.w #$3c, $70(a6)
pub fn i_06c508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C50E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06C50E: move.l #$ffffffff, $48(a6)
pub fn i_06c50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C516;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06C516: bclr.b #$1, $12(a6)
pub fn i_06c516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C51C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C51C: move.b #$7, $72(a6)
pub fn i_06c51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C522;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C522: lea.l $6c528(pc), a1
pub fn i_06c522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C526;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C528;
    16
}
/// $06C526: move.l a1, (a6)
pub fn i_06c526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C528;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C528: subq.w #$1, $70(a6)
pub fn i_06c528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C52C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06C52C: cmpi.w #$0, $70(a6)
pub fn i_06c52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C532;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06C532: bgt.w $6c53c
pub fn i_06c532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C536;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6C53C; return 10; }
    8
}
// ===== SetTaskHandler_06c536  ($06C536, 8 B, kernel/task_handlers.c)
/// $06C536: lea.l $6c53e(pc), a1
pub fn i_06c536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C53A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C53E;
    16
}
/// $06C53A: move.l a1, (a6)
pub fn i_06c53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C53C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C53C: rts
pub fn i_06c53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C53E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_06c53e  ($06C53E, 14 B, kernel/jsr5b6_jmp518.c)
/// $06C53E: jsr $5b6.l
pub fn i_06c53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C544;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C544);
    cpu.pc = 0x5B6;
    16
}
/// $06C544: jmp $518.l
pub fn i_06c544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C54A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06C54A: rts
pub fn i_06c54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C54C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06c54c  ($06C54C, 8 B, kernel/jmp_to_scheduler.c)
/// $06C54C: jmp $518.l
pub fn i_06c54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C552;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06C552: rts
pub fn i_06c552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C554;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_06c554  ($06C554, 14 B, kernel/jsr5b6_jmp518.c)
/// $06C554: jsr $5b6.l
pub fn i_06c554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C55A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C55A);
    cpu.pc = 0x5B6;
    16
}
/// $06C55A: jmp $518.l
pub fn i_06c55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C560;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06C560: rts
pub fn i_06c560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C562;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_Rider_06c562  ($06C562, 80 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C562: jsr $6d574(pc)
pub fn i_06c562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C566;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C566);
    cpu.pc = 0x6D574;
    16
}
/// $06C566: andi.b #$3, $98(a6)
pub fn i_06c566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C56C;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C56C: move.b #$0, $72(a6)
pub fn i_06c56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C572;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C572: move.w #$8, $34(a6)
pub fn i_06c572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C578;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06C578: movea.l $c(a6), a0
pub fn i_06c578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C57C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C57C: move.w $8a(a0), $8a(a6)
pub fn i_06c57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C582;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06C582: jsr $6d1dc(pc)
pub fn i_06c582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C586;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C586);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C586: bcs.w $6c6ca
pub fn i_06c586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C58A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C58A: move.b d0, $97(a6)
pub fn i_06c58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C58E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C58E: andi.w #$3, d0
pub fn i_06c58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C592;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C592: movea.l #$6c5b2, a0
pub fn i_06c592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C598;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C5B2;
    cpu.a[0] = v1;
    4
}
/// $06C598: lsl.w #$2, d0
pub fn i_06c598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C59A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C59A: movea.l (a0, d0.w), a0
pub fn i_06c59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C59E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C59E: cmpa.l #$ffffffff, a0
pub fn i_06c59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5A4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C5A4: beq.w $6c5ae
pub fn i_06c5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C5AE; return 10; }
    8
}
/// $06C5A8: jsr $28cd4.l
pub fn i_06c5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C5AE);
    cpu.pc = 0x28CD4;
    16
}
/// $06C5AE: bra.w $6c5c2
pub fn i_06c5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C5C2;
    10
}
/// $06C5C2: lea.l $6c5c8(pc), a1
pub fn i_06c5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C5C8;
    16
}
/// $06C5C6: move.l a1, (a6)
pub fn i_06c5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5C8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C5C8: movea.l $c(a6), a0
pub fn i_06c5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5CC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C5CC: move.b $72(a0), d0
pub fn i_06c5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5D0;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06C5D0: cmp.b $72(a6), d0
pub fn i_06c5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5D4;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06C5D4: beq.w $6c60c
pub fn i_06c5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C60C; return 10; }
    8
}
/// $06C5D8: cmpi.b #$7, $72(a6)
pub fn i_06c5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Byte, false);
    16
}
/// $06C5DE: beq.w $6cdba
pub fn i_06c5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDBA; return 10; }
    8
}
/// $06C5E2: move.b d0, $72(a6)
pub fn i_06c5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5E6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C5E6: cmpi.b #$9, d0
pub fn i_06c5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5EA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06C5EA: bcs.w $6c5fa
pub fn i_06c5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5EE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C5FA; return 10; }
    8
}
/// $06C5EE: nop
pub fn i_06c5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5F0;
    cpu.ir = 0x4E71;
    4
}
/// $06C5F0: nop
pub fn i_06c5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5F2;
    cpu.ir = 0x4E71;
    4
}
/// $06C5F2: cmpi.b #$9, d0
pub fn i_06c5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5F6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06C5F6: nop
pub fn i_06c5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5F8;
    cpu.ir = 0x4E71;
    4
}
/// $06C5F8: trap #$f
pub fn i_06c5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5FA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06C5FA: andi.w #$f, d0
pub fn i_06c5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C5FE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C5FE: lea.l $2cd606.l, a0
pub fn i_06c5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C604;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD606;
    20
}
/// $06C604: lsl.w #$2, d0
pub fn i_06c604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C606;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C606: movea.l (a0, d0.w), a1
pub fn i_06c606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C60A;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06C60A: jmp (a1)
pub fn i_06c60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C60C;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06C60C: move.b d0, $72(a6)
pub fn i_06c60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C610;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C610: jsr $6d29c(pc)
pub fn i_06c610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C614;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C614);
    cpu.pc = 0x6D29C;
    16
}
/// $06C614: jsr $6d1dc(pc)
pub fn i_06c614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C618;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C618);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C618: bcs.w $6c6ca
pub fn i_06c618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C61C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C61C: cmp.b $97(a6), d0
pub fn i_06c61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C620;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06C620: beq.w $6c672
pub fn i_06c620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C624;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C672; return 10; }
    8
}
/// $06C624: move.b d0, $97(a6)
pub fn i_06c624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C628;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C628: movea.l $c(a6), a0
pub fn i_06c628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C62C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C62C: move.w $8a(a0), $8a(a6)
pub fn i_06c62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C632;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06C632: jsr $6d1dc(pc)
pub fn i_06c632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C636;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C636);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C636: bcs.w $6c6ca
pub fn i_06c636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C63A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C63A: move.b d0, $97(a6)
pub fn i_06c63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C63E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C63E: andi.w #$3, d0
pub fn i_06c63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C642;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C642: movea.l #$6c662, a0
pub fn i_06c642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C648;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C662;
    cpu.a[0] = v1;
    4
}
/// $06C648: lsl.w #$2, d0
pub fn i_06c648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C64A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C64A: movea.l (a0, d0.w), a0
pub fn i_06c64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C64E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C64E: cmpa.l #$ffffffff, a0
pub fn i_06c64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C654;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C654: beq.w $6c65e
pub fn i_06c654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C658;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C65E; return 10; }
    8
}
/// $06C658: jsr $28cd4.l
pub fn i_06c658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C65E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C65E);
    cpu.pc = 0x28CD4;
    16
}
/// $06C65E: bra.w $6c672
pub fn i_06c65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C662;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C672;
    10
}
/// $06C672: jsr $28d70.l
pub fn i_06c672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C678;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C678);
    cpu.pc = 0x28D70;
    16
}
/// $06C678: bcc.w $6c6c6
pub fn i_06c678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C67C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C6C6; return 10; }
    8
}
/// $06C67C: movea.l $c(a6), a0
pub fn i_06c67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C680;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C680: move.w $8a(a0), $8a(a6)
pub fn i_06c680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C686;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06C686: jsr $6d1dc(pc)
pub fn i_06c686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C68A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C68A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C68A: bcs.w $6c6ca
pub fn i_06c68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C68E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C68E: move.b d0, $97(a6)
pub fn i_06c68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C692;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C692: andi.w #$3, d0
pub fn i_06c692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C696;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C696: movea.l #$6c6b6, a0
pub fn i_06c696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C69C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C6B6;
    cpu.a[0] = v1;
    4
}
/// $06C69C: lsl.w #$2, d0
pub fn i_06c69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C69E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C69E: movea.l (a0, d0.w), a0
pub fn i_06c69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6A2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C6A2: cmpa.l #$ffffffff, a0
pub fn i_06c6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6A8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C6A8: beq.w $6c6b2
pub fn i_06c6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C6B2; return 10; }
    8
}
/// $06C6AC: jsr $28cd4.l
pub fn i_06c6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C6B2);
    cpu.pc = 0x28CD4;
    16
}
/// $06C6B2: bra.w $6c6c6
pub fn i_06c6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C6C6;
    10
}
/// $06C6C6: bra.w $6cd62
pub fn i_06c6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CD62;
    10
}
/// $06C6CA: move.w #$8, $34(a6)
pub fn i_06c6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06C6D0: lea.l $2cf614.l, a0
pub fn i_06c6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6D6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CF614;
    20
}
/// $06C6D6: jsr $28cd4.l
pub fn i_06c6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C6DC);
    cpu.pc = 0x28CD4;
    16
}
/// $06C6DC: lea.l $6c6e2(pc), a1
pub fn i_06c6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C6E2;
    16
}
/// $06C6E0: move.l a1, (a6)
pub fn i_06c6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C6E2: movea.l $c(a6), a0
pub fn i_06c6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6E6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C6E6: move.b $72(a0), d0
pub fn i_06c6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6EA;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06C6EA: cmp.b $72(a6), d0
pub fn i_06c6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6EE;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06C6EE: beq.w $6c726
pub fn i_06c6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C726; return 10; }
    8
}
/// $06C6F2: cmpi.b #$7, $72(a6)
pub fn i_06c6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6F8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Byte, false);
    16
}
/// $06C6F8: beq.w $6cdba
pub fn i_06c6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C6FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDBA; return 10; }
    8
}
/// $06C6FC: move.b d0, $72(a6)
pub fn i_06c6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C700;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C700: cmpi.b #$9, d0
pub fn i_06c700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C704;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06C704: bcs.w $6c714
pub fn i_06c704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C708;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C714; return 10; }
    8
}
/// $06C708: nop
pub fn i_06c708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C70A;
    cpu.ir = 0x4E71;
    4
}
/// $06C70A: nop
pub fn i_06c70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C70C;
    cpu.ir = 0x4E71;
    4
}
/// $06C70C: cmpi.b #$9, d0
pub fn i_06c70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C710;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06C710: nop
pub fn i_06c710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C712;
    cpu.ir = 0x4E71;
    4
}
/// $06C712: trap #$f
pub fn i_06c712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C714;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06C714: andi.w #$f, d0
pub fn i_06c714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C718;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C718: lea.l $2cd606.l, a0
pub fn i_06c718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C71E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD606;
    20
}
/// $06C71E: lsl.w #$2, d0
pub fn i_06c71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C720;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C720: movea.l (a0, d0.w), a1
pub fn i_06c720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C724;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06C724: jmp (a1)
pub fn i_06c724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C726;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06C726: move.b d0, $72(a6)
pub fn i_06c726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C72A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C72A: jsr $6d29c(pc)
pub fn i_06c72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C72E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C72E);
    cpu.pc = 0x6D29C;
    16
}
/// $06C72E: jsr $6d1dc(pc)
pub fn i_06c72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C732;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C732);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C732: bcc.w $6c572
pub fn i_06c732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C736;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C572; return 10; }
    8
}
// ===== JsrAbsThunk_06c736  ($06C736, 8 B, thunks/jsr_abs_thunks.c)
/// $06C736: jsr $28d70.l
pub fn i_06c736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C73C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C73C);
    cpu.pc = 0x28D70;
    16
}
/// $06C73C: rts
pub fn i_06c73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C73E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_RiderIdle_06c73e  ($06C73E, 72 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C73E: clr.b $88(a6)
pub fn i_06c73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C742;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C742: movea.l $c(a6), a0
pub fn i_06c742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C746;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C746: move.l $90(a0), $90(a6)
pub fn i_06c746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C74C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $06C74C: movea.l $c(a6), a0
pub fn i_06c74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C750;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C750: move.w $8a(a0), $8a(a6)
pub fn i_06c750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C756;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06C756: jsr $6d1dc(pc)
pub fn i_06c756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C75A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C75A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C75A: bcs.w $6c6ca
pub fn i_06c75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C75E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C75E: move.b d0, $97(a6)
pub fn i_06c75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C762;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C762: andi.w #$3, d0
pub fn i_06c762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C766;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C766: movea.l #$6c786, a0
pub fn i_06c766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C76C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C786;
    cpu.a[0] = v1;
    4
}
/// $06C76C: lsl.w #$2, d0
pub fn i_06c76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C76E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C76E: movea.l (a0, d0.w), a0
pub fn i_06c76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C772;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C772: cmpa.l #$ffffffff, a0
pub fn i_06c772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C778;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C778: beq.w $6c782
pub fn i_06c778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C77C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C782; return 10; }
    8
}
/// $06C77C: jsr $28cd4.l
pub fn i_06c77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C782);
    cpu.pc = 0x28CD4;
    16
}
/// $06C782: bra.w $6c796
pub fn i_06c782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C786;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C796;
    10
}
/// $06C796: lea.l $6c79c(pc), a1
pub fn i_06c796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C79A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C79C;
    16
}
/// $06C79A: move.l a1, (a6)
pub fn i_06c79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C79C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C79C: addq.b #$1, $88(a6)
pub fn i_06c79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7A0;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C7A0: andi.b #$3, $88(a6)
pub fn i_06c7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7A6;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C7A6: bne.w $6c7c8
pub fn i_06c7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7AA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6C7C8; return 10; }
    8
}
/// $06C7AA: jsr $6d2fa(pc)
pub fn i_06c7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C7AE);
    cpu.pc = 0x6D2FA;
    16
}
/// $06C7AE: movea.l $c(a6), a0
pub fn i_06c7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7B2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C7B2: clr.b $73(a0)
pub fn i_06c7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7B6;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C7B6: jsr $6d340(pc)
pub fn i_06c7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C7BA);
    cpu.pc = 0x6D340;
    16
}
/// $06C7BA: bcc.w $6c7c8
pub fn i_06c7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C7C8; return 10; }
    8
}
/// $06C7BE: movea.l $c(a6), a0
pub fn i_06c7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7C2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C7C2: move.b #$1, $73(a0)
pub fn i_06c7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7C8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C7C8: movea.l $c(a6), a0
pub fn i_06c7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7CC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C7CC: move.b $72(a0), d0
pub fn i_06c7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7D0;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06C7D0: cmp.b $72(a6), d0
pub fn i_06c7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7D4;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06C7D4: beq.w $6c80c
pub fn i_06c7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C80C; return 10; }
    8
}
/// $06C7D8: cmpi.b #$7, $72(a6)
pub fn i_06c7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7DE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Byte, false);
    16
}
/// $06C7DE: beq.w $6cdba
pub fn i_06c7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDBA; return 10; }
    8
}
/// $06C7E2: move.b d0, $72(a6)
pub fn i_06c7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7E6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C7E6: cmpi.b #$9, d0
pub fn i_06c7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7EA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06C7EA: bcs.w $6c7fa
pub fn i_06c7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7EE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C7FA; return 10; }
    8
}
/// $06C7EE: nop
pub fn i_06c7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7F0;
    cpu.ir = 0x4E71;
    4
}
/// $06C7F0: nop
pub fn i_06c7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7F2;
    cpu.ir = 0x4E71;
    4
}
/// $06C7F2: cmpi.b #$9, d0
pub fn i_06c7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7F6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06C7F6: nop
pub fn i_06c7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7F8;
    cpu.ir = 0x4E71;
    4
}
/// $06C7F8: trap #$f
pub fn i_06c7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7FA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06C7FA: andi.w #$f, d0
pub fn i_06c7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C7FE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C7FE: lea.l $2cd606.l, a0
pub fn i_06c7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C804;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD606;
    20
}
/// $06C804: lsl.w #$2, d0
pub fn i_06c804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C806;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C806: movea.l (a0, d0.w), a1
pub fn i_06c806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C80A;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06C80A: jmp (a1)
pub fn i_06c80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C80C;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06C80C: move.b d0, $72(a6)
pub fn i_06c80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C810;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C810: jsr $6d29c(pc)
pub fn i_06c810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C814;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C814);
    cpu.pc = 0x6D29C;
    16
}
/// $06C814: jsr $6d1dc(pc)
pub fn i_06c814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C818;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C818);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C818: bcs.w $6c6ca
pub fn i_06c818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C81C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C81C: cmp.b $97(a6), d0
pub fn i_06c81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C820;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06C820: beq.w $6c872
pub fn i_06c820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C824;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C872; return 10; }
    8
}
/// $06C824: move.b d0, $97(a6)
pub fn i_06c824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C828;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C828: movea.l $c(a6), a0
pub fn i_06c828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C82C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C82C: move.w $8a(a0), $8a(a6)
pub fn i_06c82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C832;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06C832: jsr $6d1dc(pc)
pub fn i_06c832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C836;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C836);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C836: bcs.w $6c6ca
pub fn i_06c836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C83A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C83A: move.b d0, $97(a6)
pub fn i_06c83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C83E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C83E: andi.w #$3, d0
pub fn i_06c83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C842;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C842: movea.l #$6c862, a0
pub fn i_06c842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C848;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C862;
    cpu.a[0] = v1;
    4
}
/// $06C848: lsl.w #$2, d0
pub fn i_06c848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C84A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C84A: movea.l (a0, d0.w), a0
pub fn i_06c84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C84E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C84E: cmpa.l #$ffffffff, a0
pub fn i_06c84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C854;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C854: beq.w $6c85e
pub fn i_06c854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C858;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C85E; return 10; }
    8
}
/// $06C858: jsr $28cd4.l
pub fn i_06c858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C85E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C85E);
    cpu.pc = 0x28CD4;
    16
}
/// $06C85E: bra.w $6c872
pub fn i_06c85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C862;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C872;
    10
}
/// $06C872: jsr $28d70.l
pub fn i_06c872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C878);
    cpu.pc = 0x28D70;
    16
}
/// $06C878: bcc.w $6c8c6
pub fn i_06c878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C87C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C8C6; return 10; }
    8
}
/// $06C87C: movea.l $c(a6), a0
pub fn i_06c87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C880;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C880: move.w $8a(a0), $8a(a6)
pub fn i_06c880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C886;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06C886: jsr $6d1dc(pc)
pub fn i_06c886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C88A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C88A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C88A: bcs.w $6c6ca
pub fn i_06c88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C88E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C88E: move.b d0, $97(a6)
pub fn i_06c88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C892;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C892: andi.w #$3, d0
pub fn i_06c892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C896;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C896: movea.l #$6c8b6, a0
pub fn i_06c896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C89C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C8B6;
    cpu.a[0] = v1;
    4
}
/// $06C89C: lsl.w #$2, d0
pub fn i_06c89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C89E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C89E: movea.l (a0, d0.w), a0
pub fn i_06c89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8A2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C8A2: cmpa.l #$ffffffff, a0
pub fn i_06c8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8A8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C8A8: beq.w $6c8b2
pub fn i_06c8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8AC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C8B2; return 10; }
    8
}
/// $06C8AC: jsr $28cd4.l
pub fn i_06c8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C8B2);
    cpu.pc = 0x28CD4;
    16
}
/// $06C8B2: bra.w $6c8c6
pub fn i_06c8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C8C6;
    10
}
/// $06C8C6: bra.w $6cd62
pub fn i_06c8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CD62;
    10
}
// ===== RocketVehicle_RiderAim_06c8ca  ($06C8CA, 176 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C8CA: movea.l $c(a6), a0
pub fn i_06c8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8CE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C8CE: move.w $8a(a0), $8a(a6)
pub fn i_06c8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8D4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06C8D4: jsr $6d1dc(pc)
pub fn i_06c8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C8D8);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C8D8: bcs.w $6c6ca
pub fn i_06c8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8DC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C8DC: move.b d0, $97(a6)
pub fn i_06c8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8E0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C8E0: andi.w #$3, d0
pub fn i_06c8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8E4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C8E4: lea.l $2cd02a.l, a0
pub fn i_06c8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD02A;
    20
}
/// $06C8EA: lsl.w #$2, d0
pub fn i_06c8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8EC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C8EC: movea.l (a0, d0.w), a0
pub fn i_06c8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8F0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C8F0: move.w $34(a6), d0
pub fn i_06c8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8F4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06C8F4: andi.w #$f, d0
pub fn i_06c8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C8F8: lsl.w #$2, d0
pub fn i_06c8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8FA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C8FA: movea.l (a0, d0.w), a0
pub fn i_06c8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C8FE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C8FE: cmpa.l #$ffffffff, a0
pub fn i_06c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C904;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C904: beq.w $6c90e
pub fn i_06c904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C908;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C90E; return 10; }
    8
}
/// $06C908: jsr $28cd4.l
pub fn i_06c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C90E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C90E);
    cpu.pc = 0x28CD4;
    16
}
/// $06C90E: jsr $6d4c2(pc)
pub fn i_06c90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C912;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C912);
    cpu.pc = 0x6D4C2;
    16
}
/// $06C912: lea.l $6c918(pc), a1
pub fn i_06c912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C916;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C918;
    16
}
/// $06C916: move.l a1, (a6)
pub fn i_06c916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C918;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C918: movea.l $c(a6), a0
pub fn i_06c918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C91C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C91C: move.b $72(a0), d0
pub fn i_06c91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C920;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06C920: cmp.b $72(a6), d0
pub fn i_06c920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C924;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06C924: beq.w $6c95c
pub fn i_06c924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C928;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C95C; return 10; }
    8
}
/// $06C928: cmpi.b #$7, $72(a6)
pub fn i_06c928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C92E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Byte, false);
    16
}
/// $06C92E: beq.w $6cdba
pub fn i_06c92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C932;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDBA; return 10; }
    8
}
/// $06C932: move.b d0, $72(a6)
pub fn i_06c932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C936;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C936: cmpi.b #$9, d0
pub fn i_06c936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C93A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06C93A: bcs.w $6c94a
pub fn i_06c93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C93E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C94A; return 10; }
    8
}
/// $06C93E: nop
pub fn i_06c93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C940;
    cpu.ir = 0x4E71;
    4
}
/// $06C940: nop
pub fn i_06c940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C942;
    cpu.ir = 0x4E71;
    4
}
/// $06C942: cmpi.b #$9, d0
pub fn i_06c942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C946;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06C946: nop
pub fn i_06c946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C948;
    cpu.ir = 0x4E71;
    4
}
/// $06C948: trap #$f
pub fn i_06c948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C94A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06C94A: andi.w #$f, d0
pub fn i_06c94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C94E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C94E: lea.l $2cd606.l, a0
pub fn i_06c94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C954;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD606;
    20
}
/// $06C954: lsl.w #$2, d0
pub fn i_06c954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C956;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C956: movea.l (a0, d0.w), a1
pub fn i_06c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C95A;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06C95A: jmp (a1)
pub fn i_06c95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C95C;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06C95C: move.b d0, $72(a6)
pub fn i_06c95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C960;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C960: jsr $6d29c(pc)
pub fn i_06c960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C964;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C964);
    cpu.pc = 0x6D29C;
    16
}
/// $06C964: jsr $28d70.l
pub fn i_06c964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C96A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C96A);
    cpu.pc = 0x28D70;
    16
}
/// $06C96A: bcc.w $6c978
pub fn i_06c96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C96E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6C978; return 10; }
    8
}
/// $06C96E: movea.l $c(a6), a0
pub fn i_06c96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C972;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C972: move.b #$1, $73(a0)
pub fn i_06c972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C978;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C978: rts
pub fn i_06c978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C97A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_RiderFire_06c97a  ($06C97A, 62 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06C97A: clr.b $88(a6)
pub fn i_06c97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C97E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C97E: movea.l $c(a6), a0
pub fn i_06c97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C982;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C982: move.w $8a(a0), $8a(a6)
pub fn i_06c982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C988;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06C988: jsr $6d1dc(pc)
pub fn i_06c988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C98C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6C98C);
    cpu.pc = 0x6D1DC;
    16
}
/// $06C98C: bcs.w $6c6ca
pub fn i_06c98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C990;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06C990: move.b d0, $97(a6)
pub fn i_06c990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C994;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06C994: andi.w #$3, d0
pub fn i_06c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C998;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06C998: movea.l #$6c9b8, a0
pub fn i_06c998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C99E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6C9B8;
    cpu.a[0] = v1;
    4
}
/// $06C99E: lsl.w #$2, d0
pub fn i_06c99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9A0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06C9A0: movea.l (a0, d0.w), a0
pub fn i_06c9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9A4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06C9A4: cmpa.l #$ffffffff, a0
pub fn i_06c9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9AA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06C9AA: beq.w $6c9b4
pub fn i_06c9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C9B4; return 10; }
    8
}
/// $06C9AE: jsr $28cd4.l
pub fn i_06c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6C9B4);
    cpu.pc = 0x28CD4;
    16
}
/// $06C9B4: bra.w $6c9c8
pub fn i_06c9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6C9C8;
    10
}
/// $06C9C8: lea.l $6c9ce(pc), a1
pub fn i_06c9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C9CE;
    16
}
/// $06C9CC: move.l a1, (a6)
pub fn i_06c9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06C9CE: addq.b #$1, $88(a6)
pub fn i_06c9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9D2;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06C9D2: andi.b #$3, $88(a6)
pub fn i_06c9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9D8;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x3) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06C9D8: bne.w $6ca04
pub fn i_06c9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6CA04; return 10; }
    8
}
/// $06C9DC: move.w #$1, d1
pub fn i_06c9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9E0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06C9E0: cmpi.w #$8, $34(a6)
pub fn i_06c9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $06C9E6: beq.w $6c9fa
pub fn i_06c9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9EA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6C9FA; return 10; }
    8
}
/// $06C9EA: blt.w $6c9f2
pub fn i_06c9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9EE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6C9F2; return 10; }
    8
}
/// $06C9EE: move.w #$ffff, d1
pub fn i_06c9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06C9F2: add.w d1, $34(a6)
pub fn i_06c9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9F6;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06C9F6: bra.w $6ca04
pub fn i_06c9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CA04;
    10
}
/// $06C9FA: movea.l $c(a6), a0
pub fn i_06c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6C9FE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06C9FE: move.b #$1, $73(a0)
pub fn i_06c9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA04;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CA04: movea.l $c(a6), a0
pub fn i_06ca04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA08;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CA08: move.b $72(a0), d0
pub fn i_06ca08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA0C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06CA0C: cmp.b $72(a6), d0
pub fn i_06ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA10;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06CA10: beq.w $6ca48
pub fn i_06ca10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CA48; return 10; }
    8
}
/// $06CA14: cmpi.b #$7, $72(a6)
pub fn i_06ca14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA1A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Byte, false);
    16
}
/// $06CA1A: beq.w $6cdba
pub fn i_06ca1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA1E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDBA; return 10; }
    8
}
/// $06CA1E: move.b d0, $72(a6)
pub fn i_06ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA22;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CA22: cmpi.b #$9, d0
pub fn i_06ca22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA26;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06CA26: bcs.w $6ca36
pub fn i_06ca26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA2A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6CA36; return 10; }
    8
}
/// $06CA2A: nop
pub fn i_06ca2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA2C;
    cpu.ir = 0x4E71;
    4
}
/// $06CA2C: nop
pub fn i_06ca2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA2E;
    cpu.ir = 0x4E71;
    4
}
/// $06CA2E: cmpi.b #$9, d0
pub fn i_06ca2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA32;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06CA32: nop
pub fn i_06ca32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA34;
    cpu.ir = 0x4E71;
    4
}
/// $06CA34: trap #$f
pub fn i_06ca34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA36;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06CA36: andi.w #$f, d0
pub fn i_06ca36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA3A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CA3A: lea.l $2cd606.l, a0
pub fn i_06ca3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD606;
    20
}
/// $06CA40: lsl.w #$2, d0
pub fn i_06ca40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA42;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CA42: movea.l (a0, d0.w), a1
pub fn i_06ca42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA46;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06CA46: jmp (a1)
pub fn i_06ca46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA48;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06CA48: move.b d0, $72(a6)
pub fn i_06ca48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA4C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CA4C: jsr $6d29c(pc)
pub fn i_06ca4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CA50);
    cpu.pc = 0x6D29C;
    16
}
/// $06CA50: jsr $6d1dc(pc)
pub fn i_06ca50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CA54);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CA54: bcs.w $6c6ca
pub fn i_06ca54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA58;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CA58: cmp.b $97(a6), d0
pub fn i_06ca58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA5C;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06CA5C: beq.w $6caae
pub fn i_06ca5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CAAE; return 10; }
    8
}
/// $06CA60: move.b d0, $97(a6)
pub fn i_06ca60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA64;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CA64: movea.l $c(a6), a0
pub fn i_06ca64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA68;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CA68: move.w $8a(a0), $8a(a6)
pub fn i_06ca68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA6E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CA6E: jsr $6d1dc(pc)
pub fn i_06ca6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CA72);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CA72: bcs.w $6c6ca
pub fn i_06ca72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA76;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CA76: move.b d0, $97(a6)
pub fn i_06ca76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA7A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CA7A: andi.w #$3, d0
pub fn i_06ca7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA7E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CA7E: movea.l #$6ca9e, a0
pub fn i_06ca7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA84;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6CA9E;
    cpu.a[0] = v1;
    4
}
/// $06CA84: lsl.w #$2, d0
pub fn i_06ca84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA86;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CA86: movea.l (a0, d0.w), a0
pub fn i_06ca86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA8A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06CA8A: cmpa.l #$ffffffff, a0
pub fn i_06ca8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA90;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06CA90: beq.w $6ca9a
pub fn i_06ca90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CA9A; return 10; }
    8
}
/// $06CA94: jsr $28cd4.l
pub fn i_06ca94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CA9A);
    cpu.pc = 0x28CD4;
    16
}
/// $06CA9A: bra.w $6caae
pub fn i_06ca9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CA9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CAAE;
    10
}
/// $06CAAE: jsr $28d70.l
pub fn i_06caae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CAB4);
    cpu.pc = 0x28D70;
    16
}
/// $06CAB4: bcc.w $6cb02
pub fn i_06cab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6CB02; return 10; }
    8
}
/// $06CAB8: movea.l $c(a6), a0
pub fn i_06cab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CABC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CABC: move.w $8a(a0), $8a(a6)
pub fn i_06cabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAC2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CAC2: jsr $6d1dc(pc)
pub fn i_06cac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CAC6);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CAC6: bcs.w $6c6ca
pub fn i_06cac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CACA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CACA: move.b d0, $97(a6)
pub fn i_06caca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CACE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CACE: andi.w #$3, d0
pub fn i_06cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAD2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CAD2: movea.l #$6caf2, a0
pub fn i_06cad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAD8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6CAF2;
    cpu.a[0] = v1;
    4
}
/// $06CAD8: lsl.w #$2, d0
pub fn i_06cad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CADA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CADA: movea.l (a0, d0.w), a0
pub fn i_06cada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CADE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06CADE: cmpa.l #$ffffffff, a0
pub fn i_06cade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAE4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06CAE4: beq.w $6caee
pub fn i_06cae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CAEE; return 10; }
    8
}
/// $06CAE8: jsr $28cd4.l
pub fn i_06cae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CAEE);
    cpu.pc = 0x28CD4;
    16
}
/// $06CAEE: bra.w $6cb02
pub fn i_06caee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CAF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CB02;
    10
}
/// $06CB02: bra.w $6cd62
pub fn i_06cb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB06;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CD62;
    10
}
// ===== RocketVehicle_RiderHurt_06cb06  ($06CB06, 62 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06CB06: clr.b $88(a6)
pub fn i_06cb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB0A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06CB0A: movea.l $c(a6), a0
pub fn i_06cb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB0E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CB0E: move.w $8a(a0), $8a(a6)
pub fn i_06cb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB14;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CB14: jsr $6d1dc(pc)
pub fn i_06cb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CB18);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CB18: bcs.w $6c6ca
pub fn i_06cb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB1C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CB1C: move.b d0, $97(a6)
pub fn i_06cb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB20;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CB20: andi.w #$3, d0
pub fn i_06cb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB24;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CB24: movea.l #$6cb44, a0
pub fn i_06cb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB2A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6CB44;
    cpu.a[0] = v1;
    4
}
/// $06CB2A: lsl.w #$2, d0
pub fn i_06cb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB2C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CB2C: movea.l (a0, d0.w), a0
pub fn i_06cb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB30;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06CB30: cmpa.l #$ffffffff, a0
pub fn i_06cb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB36;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06CB36: beq.w $6cb40
pub fn i_06cb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CB40; return 10; }
    8
}
/// $06CB3A: jsr $28cd4.l
pub fn i_06cb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CB40);
    cpu.pc = 0x28CD4;
    16
}
/// $06CB40: bra.w $6cb54
pub fn i_06cb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB44;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CB54;
    10
}
/// $06CB54: lea.l $6cb5a(pc), a1
pub fn i_06cb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CB5A;
    16
}
/// $06CB58: move.l a1, (a6)
pub fn i_06cb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CB5A: movea.l $c(a6), a0
pub fn i_06cb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB5E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CB5E: move.b $72(a0), d0
pub fn i_06cb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB62;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06CB62: cmp.b $72(a6), d0
pub fn i_06cb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB66;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06CB66: beq.w $6cb9e
pub fn i_06cb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB6A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CB9E; return 10; }
    8
}
/// $06CB6A: cmpi.b #$7, $72(a6)
pub fn i_06cb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB70;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Byte, false);
    16
}
/// $06CB70: beq.w $6cdba
pub fn i_06cb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDBA; return 10; }
    8
}
/// $06CB74: move.b d0, $72(a6)
pub fn i_06cb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB78;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CB78: cmpi.b #$9, d0
pub fn i_06cb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB7C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06CB7C: bcs.w $6cb8c
pub fn i_06cb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB80;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6CB8C; return 10; }
    8
}
/// $06CB80: nop
pub fn i_06cb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB82;
    cpu.ir = 0x4E71;
    4
}
/// $06CB82: nop
pub fn i_06cb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB84;
    cpu.ir = 0x4E71;
    4
}
/// $06CB84: cmpi.b #$9, d0
pub fn i_06cb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB88;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06CB88: nop
pub fn i_06cb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB8A;
    cpu.ir = 0x4E71;
    4
}
/// $06CB8A: trap #$f
pub fn i_06cb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB8C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06CB8C: andi.w #$f, d0
pub fn i_06cb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB90;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CB90: lea.l $2cd606.l, a0
pub fn i_06cb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB96;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD606;
    20
}
/// $06CB96: lsl.w #$2, d0
pub fn i_06cb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB98;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CB98: movea.l (a0, d0.w), a1
pub fn i_06cb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB9C;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06CB9C: jmp (a1)
pub fn i_06cb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CB9E;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06CB9E: move.b d0, $72(a6)
pub fn i_06cb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBA2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CBA2: jsr $6d29c(pc)
pub fn i_06cba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBA6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CBA6);
    cpu.pc = 0x6D29C;
    16
}
/// $06CBA6: jsr $6d1dc(pc)
pub fn i_06cba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CBAA);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CBAA: bcs.w $6c6ca
pub fn i_06cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBAE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CBAE: cmp.b $97(a6), d0
pub fn i_06cbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBB2;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06CBB2: beq.w $6cc04
pub fn i_06cbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBB6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CC04; return 10; }
    8
}
/// $06CBB6: move.b d0, $97(a6)
pub fn i_06cbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBBA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CBBA: movea.l $c(a6), a0
pub fn i_06cbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBBE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CBBE: move.w $8a(a0), $8a(a6)
pub fn i_06cbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBC4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CBC4: jsr $6d1dc(pc)
pub fn i_06cbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CBC8);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CBC8: bcs.w $6c6ca
pub fn i_06cbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBCC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CBCC: move.b d0, $97(a6)
pub fn i_06cbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBD0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CBD0: andi.w #$3, d0
pub fn i_06cbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBD4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CBD4: movea.l #$6cbf4, a0
pub fn i_06cbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBDA;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6CBF4;
    cpu.a[0] = v1;
    4
}
/// $06CBDA: lsl.w #$2, d0
pub fn i_06cbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBDC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CBDC: movea.l (a0, d0.w), a0
pub fn i_06cbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBE0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06CBE0: cmpa.l #$ffffffff, a0
pub fn i_06cbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBE6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06CBE6: beq.w $6cbf0
pub fn i_06cbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBEA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CBF0; return 10; }
    8
}
/// $06CBEA: jsr $28cd4.l
pub fn i_06cbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CBF0);
    cpu.pc = 0x28CD4;
    16
}
/// $06CBF0: bra.w $6cc04
pub fn i_06cbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CBF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CC04;
    10
}
/// $06CC04: jsr $28d70.l
pub fn i_06cc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CC0A);
    cpu.pc = 0x28D70;
    16
}
/// $06CC0A: bcc.w $6cc58
pub fn i_06cc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC0E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6CC58; return 10; }
    8
}
/// $06CC0E: movea.l $c(a6), a0
pub fn i_06cc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC12;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CC12: move.w $8a(a0), $8a(a6)
pub fn i_06cc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC18;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CC18: jsr $6d1dc(pc)
pub fn i_06cc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CC1C);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CC1C: bcs.w $6c6ca
pub fn i_06cc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC20;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CC20: move.b d0, $97(a6)
pub fn i_06cc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC24;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CC24: andi.w #$3, d0
pub fn i_06cc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC28;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CC28: movea.l #$6cc48, a0
pub fn i_06cc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC2E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6CC48;
    cpu.a[0] = v1;
    4
}
/// $06CC2E: lsl.w #$2, d0
pub fn i_06cc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC30;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CC30: movea.l (a0, d0.w), a0
pub fn i_06cc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC34;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06CC34: cmpa.l #$ffffffff, a0
pub fn i_06cc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC3A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06CC3A: beq.w $6cc44
pub fn i_06cc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC3E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CC44; return 10; }
    8
}
/// $06CC3E: jsr $28cd4.l
pub fn i_06cc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CC44);
    cpu.pc = 0x28CD4;
    16
}
/// $06CC44: bra.w $6cc58
pub fn i_06cc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC48;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CC58;
    10
}
/// $06CC58: bra.w $6cd62
pub fn i_06cc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CD62;
    10
}
// ===== RocketVehicle_RiderDie_06cc5c  ($06CC5C, 58 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06CC5C: movea.l $c(a6), a0
pub fn i_06cc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC60;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CC60: move.w $8a(a0), $8a(a6)
pub fn i_06cc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC66;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CC66: jsr $6d1dc(pc)
pub fn i_06cc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CC6A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CC6A: bcs.w $6c6ca
pub fn i_06cc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC6E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CC6E: move.b d0, $97(a6)
pub fn i_06cc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC72;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CC72: andi.w #$3, d0
pub fn i_06cc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC76;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CC76: movea.l #$6cc96, a0
pub fn i_06cc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC7C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6CC96;
    cpu.a[0] = v1;
    4
}
/// $06CC7C: lsl.w #$2, d0
pub fn i_06cc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC7E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CC7E: movea.l (a0, d0.w), a0
pub fn i_06cc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC82;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06CC82: cmpa.l #$ffffffff, a0
pub fn i_06cc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC88;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06CC88: beq.w $6cc92
pub fn i_06cc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CC92; return 10; }
    8
}
/// $06CC8C: jsr $28cd4.l
pub fn i_06cc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CC92);
    cpu.pc = 0x28CD4;
    16
}
/// $06CC92: bra.w $6cca6
pub fn i_06cc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CC96;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CCA6;
    10
}
/// $06CCA6: lea.l $6ccac(pc), a1
pub fn i_06cca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CCAC;
    16
}
/// $06CCAA: move.l a1, (a6)
pub fn i_06ccaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CCAC: movea.l $c(a6), a0
pub fn i_06ccac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCB0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CCB0: move.b $72(a0), d0
pub fn i_06ccb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCB4;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06CCB4: cmp.b $72(a6), d0
pub fn i_06ccb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCB8;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06CCB8: beq.w $6ccf0
pub fn i_06ccb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCBC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CCF0; return 10; }
    8
}
/// $06CCBC: cmpi.b #$7, $72(a6)
pub fn i_06ccbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCC2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x7, Size::Byte, false);
    16
}
/// $06CCC2: beq.w $6cdba
pub fn i_06ccc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCC6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDBA; return 10; }
    8
}
/// $06CCC6: move.b d0, $72(a6)
pub fn i_06ccc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCCA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CCCA: cmpi.b #$9, d0
pub fn i_06ccca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCCE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06CCCE: bcs.w $6ccde
pub fn i_06ccce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCD2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6CCDE; return 10; }
    8
}
/// $06CCD2: nop
pub fn i_06ccd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCD4;
    cpu.ir = 0x4E71;
    4
}
/// $06CCD4: nop
pub fn i_06ccd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCD6;
    cpu.ir = 0x4E71;
    4
}
/// $06CCD6: cmpi.b #$9, d0
pub fn i_06ccd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCDA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x9, Size::Byte, false);
    8
}
/// $06CCDA: nop
pub fn i_06ccda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCDC;
    cpu.ir = 0x4E71;
    4
}
/// $06CCDC: trap #$f
pub fn i_06ccdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCDE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $06CCDE: andi.w #$f, d0
pub fn i_06ccde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCE2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CCE2: lea.l $2cd606.l, a0
pub fn i_06cce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CD606;
    20
}
/// $06CCE8: lsl.w #$2, d0
pub fn i_06cce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCEA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CCEA: movea.l (a0, d0.w), a1
pub fn i_06ccea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCEE;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06CCEE: jmp (a1)
pub fn i_06ccee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCF0;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
/// $06CCF0: move.b d0, $72(a6)
pub fn i_06ccf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCF4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CCF4: jsr $6d29c(pc)
pub fn i_06ccf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CCF8);
    cpu.pc = 0x6D29C;
    16
}
/// $06CCF8: jsr $6d1dc(pc)
pub fn i_06ccf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CCFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CCFC);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CCFC: bcs.w $6c6ca
pub fn i_06ccfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD00;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CD00: cmp.b $97(a6), d0
pub fn i_06cd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD04;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06CD04: beq.w $6cd56
pub fn i_06cd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CD56; return 10; }
    8
}
/// $06CD08: move.b d0, $97(a6)
pub fn i_06cd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD0C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CD0C: movea.l $c(a6), a0
pub fn i_06cd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD10;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CD10: move.w $8a(a0), $8a(a6)
pub fn i_06cd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD16;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CD16: jsr $6d1dc(pc)
pub fn i_06cd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CD1A);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CD1A: bcs.w $6c6ca
pub fn i_06cd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD1E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CD1E: move.b d0, $97(a6)
pub fn i_06cd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD22;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CD22: andi.w #$3, d0
pub fn i_06cd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD26;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CD26: movea.l #$6cd46, a0
pub fn i_06cd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD2C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6CD46;
    cpu.a[0] = v1;
    4
}
/// $06CD2C: lsl.w #$2, d0
pub fn i_06cd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD2E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CD2E: movea.l (a0, d0.w), a0
pub fn i_06cd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD32;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06CD32: cmpa.l #$ffffffff, a0
pub fn i_06cd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD38;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06CD38: beq.w $6cd42
pub fn i_06cd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD3C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CD42; return 10; }
    8
}
/// $06CD3C: jsr $28cd4.l
pub fn i_06cd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CD42);
    cpu.pc = 0x28CD4;
    16
}
/// $06CD42: bra.w $6cd56
pub fn i_06cd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD46;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CD56;
    10
}
/// $06CD56: subq.w #$1, $38(a6)
pub fn i_06cd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD5A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== JsrAbsThunk_06cd5a  ($06CD5A, 8 B, thunks/jsr_abs_thunks.c)
/// $06CD5A: jsr $28d70.l
pub fn i_06cd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CD60);
    cpu.pc = 0x28D70;
    16
}
/// $06CD60: rts
pub fn i_06cd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_RiderCorpse_06cd62  ($06CD62, 70 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06CD62: movea.l $c(a6), a0
pub fn i_06cd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD66;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CD66: tst.b $94(a0)
pub fn i_06cd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD6A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06CD6A: beq.w $6cdb8
pub fn i_06cd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD6E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDB8; return 10; }
    8
}
/// $06CD6E: movea.l $c(a6), a0
pub fn i_06cd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD72;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06CD72: move.w $8a(a0), $8a(a6)
pub fn i_06cd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD78;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CD78: jsr $6d1dc(pc)
pub fn i_06cd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD7C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CD7C);
    cpu.pc = 0x6D1DC;
    16
}
/// $06CD7C: bcs.w $6c6ca
pub fn i_06cd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD80;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6C6CA; return 10; }
    8
}
/// $06CD80: move.b d0, $97(a6)
pub fn i_06cd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD84;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CD84: andi.w #$3, d0
pub fn i_06cd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD88;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CD88: movea.l #$6cda8, a0
pub fn i_06cd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD8E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x6CDA8;
    cpu.a[0] = v1;
    4
}
/// $06CD8E: lsl.w #$2, d0
pub fn i_06cd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD90;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CD90: movea.l (a0, d0.w), a0
pub fn i_06cd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD94;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06CD94: cmpa.l #$ffffffff, a0
pub fn i_06cd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD9A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06CD9A: beq.w $6cda4
pub fn i_06cd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CD9E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CDA4; return 10; }
    8
}
/// $06CD9E: jsr $28cd4.l
pub fn i_06cd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CDA4);
    cpu.pc = 0x28CD4;
    16
}
/// $06CDA4: bra.w $6cdb8
pub fn i_06cda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CDB8;
    10
}
/// $06CDB8: rts
pub fn i_06cdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_WeaponFlyOff_06cdba  ($06CDBA, 134 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06CDBA: move.w #$ffde, d0
pub fn i_06cdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDBE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFDE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CDBE: jsr $5dca4.l
pub fn i_06cdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CDC4);
    cpu.pc = 0x5DCA4;
    16
}
/// $06CDC4: move.w d0, $28(a6)
pub fn i_06cdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDC8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06CDC8: move.w #$663, $2a(a6)
pub fn i_06cdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDCE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x663;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06CDCE: move.w #$ff93, $2e(a6)
pub fn i_06cdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDD4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF93;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06CDD4: move.w #$0, $2c(a6)
pub fn i_06cdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDDA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06CDDA: move.w #$4000, d0
pub fn i_06cdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDDE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CDDE: jsr $28134.l
pub fn i_06cdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CDE4);
    cpu.pc = 0x28134;
    16
}
/// $06CDE4: andi.w #$ffe3, $38(a6)
pub fn i_06cde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDEA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CDEA: ori.w #$0, $38(a6)
pub fn i_06cdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDF0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CDF0: lea.l $6cdf6(pc), a1
pub fn i_06cdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CDF6;
    16
}
/// $06CDF4: move.l a1, (a6)
pub fn i_06cdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CDF6: move.w $34(a6), $5c(a6)
pub fn i_06cdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CDFC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CDFC: jsr $27bc8.l
pub fn i_06cdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CE02);
    cpu.pc = 0x27BC8;
    16
}
/// $06CE02: bcc.w $6ce0c
pub fn i_06ce02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6CE0C; return 10; }
    8
}
/// $06CE06: lea.l $6ce66(pc), a1
pub fn i_06ce06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CE66;
    16
}
/// $06CE0A: move.l a1, (a6)
pub fn i_06ce0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CE0C: move.w $5c(a6), $34(a6)
pub fn i_06ce0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE12;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CE12: jsr $28d70.l
pub fn i_06ce12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CE18);
    cpu.pc = 0x28D70;
    16
}
/// $06CE18: cmpi.w #$c0, $24(a6)
pub fn i_06ce18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE1E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC0, Size::Word, false);
    16
}
/// $06CE1E: bgt.w $6ce28
pub fn i_06ce1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE22;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6CE28; return 10; }
    8
}
/// $06CE22: lea.l $6c54c(pc), a1
pub fn i_06ce22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C54C;
    16
}
/// $06CE26: move.l a1, (a6)
pub fn i_06ce26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CE28: jsr $5e45a.l
pub fn i_06ce28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CE2E);
    cpu.pc = 0x5E45A;
    16
}
/// $06CE2E: bcc.w $6ce38
pub fn i_06ce2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE32;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6CE38; return 10; }
    8
}
/// $06CE32: lea.l $6ce66(pc), a1
pub fn i_06ce32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CE66;
    16
}
/// $06CE36: move.l a1, (a6)
pub fn i_06ce36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CE38: jsr $6d05c(pc)
pub fn i_06ce38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6CE3C);
    cpu.pc = 0x6D05C;
    16
}
/// $06CE3C: bcc.w $6ce46
pub fn i_06ce3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6CE46; return 10; }
    8
}
// ===== SetTaskHandler_06ce40  ($06CE40, 8 B, kernel/task_handlers.c)
/// $06CE40: lea.l $6c554(pc), a1
pub fn i_06ce40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C554;
    16
}
/// $06CE44: move.l a1, (a6)
pub fn i_06ce44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CE46: rts
pub fn i_06ce46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_WeaponFall_06ce48  ($06CE48, 22 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06CE48: lea.l $6ce4e(pc), a1
pub fn i_06ce48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CE4E;
    16
}
/// $06CE4C: move.l a1, (a6)
pub fn i_06ce4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CE4E: jsr $2783a.l
pub fn i_06ce4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CE54);
    cpu.pc = 0x2783A;
    16
}
/// $06CE54: jsr $28d70.l
pub fn i_06ce54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CE5A);
    cpu.pc = 0x28D70;
    16
}
/// $06CE5A: bcc.w $6ce64
pub fn i_06ce5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6CE64; return 10; }
    8
}
// ===== SetTaskHandler_06ce5e  ($06CE5E, 8 B, kernel/task_handlers.c)
/// $06CE5E: lea.l $6c554(pc), a1
pub fn i_06ce5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C554;
    16
}
/// $06CE62: move.l a1, (a6)
pub fn i_06ce62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CE64: rts
pub fn i_06ce64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_WeaponExplode_06ce66  ($06CE66, 36 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06CE66: move.w #$4000, d0
pub fn i_06ce66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CE6A: jsr $28134.l
pub fn i_06ce6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CE70);
    cpu.pc = 0x28134;
    16
}
/// $06CE70: andi.w #$ffe3, $38(a6)
pub fn i_06ce70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE76;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CE76: ori.w #$0, $38(a6)
pub fn i_06ce76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE7C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CE7C: move.l #$ffffffff, $48(a6)
pub fn i_06ce7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE84;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06CE84: jmp $77f6a.l
pub fn i_06ce84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE8A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== RocketVehicle_MuzzleFlash_06ce8a  ($06CE8A, 124 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06CE8A: move.w #$4000, d0
pub fn i_06ce8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CE8E: jsr $28134.l
pub fn i_06ce8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CE94);
    cpu.pc = 0x28134;
    16
}
/// $06CE94: andi.w #$ffe3, $38(a6)
pub fn i_06ce94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CE9A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CE9A: ori.w #$1c, $38(a6)
pub fn i_06ce9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEA0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CEA0: move.w #$8, d1
pub fn i_06cea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEA4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CEA4: jsr $236e.l
pub fn i_06cea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CEAA);
    cpu.pc = 0x236E;
    16
}
/// $06CEAA: cmpi.b #$1, $97(a6)
pub fn i_06ceaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEB0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06CEB0: bne.w $6cede
pub fn i_06ceb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6CEDE; return 10; }
    8
}
/// $06CEB4: move.w $34(a6), d0
pub fn i_06ceb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEB8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06CEB8: andi.w #$7, d0
pub fn i_06ceb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEBC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CEBC: bra.w $6cede
pub fn i_06cebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CEDE;
    10
}
/// $06CEC0: move.w $34(a6), d0
pub fn i_06cec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEC4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06CEC4: lsr.w #$3, d0
pub fn i_06cec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEC6;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06CEC6: andi.w #$1, d0
pub fn i_06cec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CECA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CECA: eor.b d0, $3a(a6)
pub fn i_06ceca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CECE;
    cpu.ir = 0xB12E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06CECE: lea.l $2dd6d2.l, a0
pub fn i_06cece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CED4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD6D2;
    20
}
/// $06CED4: jsr $28cd4.l
pub fn i_06ced4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CEDA);
    cpu.pc = 0x28CD4;
    16
}
/// $06CEDA: bra.w $6cf02
pub fn i_06ceda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CF02;
    10
}
/// $06CEDE: move.b $8e(a6), d0
pub fn i_06cede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEE2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06CEE2: move.b $8f(a6), d1
pub fn i_06cee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEE6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x8F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06CEE6: ext.w d0
pub fn i_06cee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEE8;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06CEE8: ext.w d1
pub fn i_06cee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEEA;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06CEEA: add.w d0, $22(a6)
pub fn i_06ceea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEEE;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06CEEE: add.w d1, $24(a6)
pub fn i_06ceee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEF2;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06CEF2: subq.w #$8, $24(a6)
pub fn i_06cef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEF6;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06CEF6: lea.l $2dd6d2.l, a0
pub fn i_06cef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CEFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD6D2;
    20
}
/// $06CEFC: jsr $28cd4.l
pub fn i_06cefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CF02);
    cpu.pc = 0x28CD4;
    16
}
/// $06CF02: bra.w $6ce48
pub fn i_06cf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF06;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CE48;
    10
}
// ===== RocketVehicle_Rocket_06cf06  ($06CF06, 320 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06CF06: lea.l $2cd10a.l, a1
pub fn i_06cf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF0C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CD10A;
    20
}
/// $06CF0C: btst.b #$0, $3a(a6)
pub fn i_06cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF12;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06CF12: beq.w $6cf1c
pub fn i_06cf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CF1C; return 10; }
    8
}
/// $06CF16: lea.l $2cd11a.l, a1
pub fn i_06cf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF1C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CD11A;
    20
}
/// $06CF1C: move.b $97(a6), d0
pub fn i_06cf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF20;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06CF20: andi.w #$3, d0
pub fn i_06cf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF24;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CF24: lsl.w #$2, d0
pub fn i_06cf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF26;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CF26: movea.l (a1, d0.w), a1
pub fn i_06cf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF2A;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06CF2A: move.b $98(a6), d0
pub fn i_06cf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF2E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06CF2E: andi.w #$7, d0
pub fn i_06cf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF32;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CF32: lsl.w #$2, d0
pub fn i_06cf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF34;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06CF34: movea.l (a1, d0.w), a1
pub fn i_06cf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF38;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06CF38: move.w $34(a6), d0
pub fn i_06cf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF3C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06CF3C: andi.w #$f, d0
pub fn i_06cf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF40;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CF40: move.w d0, d1
pub fn i_06cf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF42;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CF42: add.w d0, d0
pub fn i_06cf42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF44;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06CF44: add.w d1, d0
pub fn i_06cf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF46;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06CF46: add.w d0, d0
pub fn i_06cf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF48;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06CF48: ext.l d0
pub fn i_06cf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF4A;
    cpu.ir = 0x48C0;
    cpu.d[0] = cpu.d[0] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06CF4A: adda.l d0, a1
pub fn i_06cf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF4C;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $06CF4C: move.w (a1), $28(a6)
pub fn i_06cf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF50;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad3, v2 as u16);
    16
}
/// $06CF50: move.w $2(a1), $2e(a6)
pub fn i_06cf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF56;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CF56: move.w $4(a1), $2a(a6)
pub fn i_06cf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF5C;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06CF5C: andi.b #$fe, $3a(a6)
pub fn i_06cf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF62;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06CF62: move.w #$173, d1
pub fn i_06cf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF66;
    cpu.ir = 0x323C;
    let v1: u32 = 0x173;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CF66: jsr $236e.l
pub fn i_06cf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CF6C);
    cpu.pc = 0x236E;
    16
}
/// $06CF6C: bset.b #$4, $6b(a6)
pub fn i_06cf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF72;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06CF72: lea.l $2d3a8e.l, a0
pub fn i_06cf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D3A8E;
    20
}
/// $06CF78: jsr $28cd4.l
pub fn i_06cf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CF7E);
    cpu.pc = 0x28CD4;
    16
}
/// $06CF7E: move.w #$d000, d0
pub fn i_06cf7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF82;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CF82: jsr $28134.l
pub fn i_06cf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CF88);
    cpu.pc = 0x28134;
    16
}
/// $06CF88: andi.w #$ffe3, $38(a6)
pub fn i_06cf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF8E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CF8E: ori.w #$1c, $38(a6)
pub fn i_06cf8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF94;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CF94: clr.b $95(a6)
pub fn i_06cf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF98;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x95);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06CF98: move.w $34(a6), d0
pub fn i_06cf98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CF9C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06CF9C: andi.w #$f, d0
pub fn i_06cf9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFA0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06CFA0: cmpi.w #$0, d0
pub fn i_06cfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFA4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06CFA4: beq.w $6cfe6
pub fn i_06cfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6CFE6; return 10; }
    8
}
/// $06CFA8: cmpi.w #$8, d0
pub fn i_06cfa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFAC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $06CFAC: bge.w $6cfe6
pub fn i_06cfac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFB0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x6CFE6; return 10; }
    8
}
/// $06CFB0: move.w #$d000, d0
pub fn i_06cfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06CFB4: jsr $28134.l
pub fn i_06cfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CFBA);
    cpu.pc = 0x28134;
    16
}
/// $06CFBA: andi.w #$ffe3, $38(a6)
pub fn i_06cfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFC0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CFC0: ori.w #$8, $38(a6)
pub fn i_06cfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFC6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06CFC6: move.b #$1, $95(a6)
pub fn i_06cfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFCC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x95);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06CFCC: lea.l $6cfd2(pc), a1
pub fn i_06cfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CFD2;
    16
}
/// $06CFD0: move.l a1, (a6)
pub fn i_06cfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CFD2: jsr $27bc8.l
pub fn i_06cfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CFD8);
    cpu.pc = 0x27BC8;
    16
}
/// $06CFD8: bcc.w $6cfe2
pub fn i_06cfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6CFE2; return 10; }
    8
}
/// $06CFDC: lea.l $6d04e(pc), a1
pub fn i_06cfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D04E;
    16
}
/// $06CFE0: move.l a1, (a6)
pub fn i_06cfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CFE2: bra.w $6cffc
pub fn i_06cfe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CFFC;
    10
}
/// $06CFE6: lea.l $6cfec(pc), a1
pub fn i_06cfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CFEC;
    16
}
/// $06CFEA: move.l a1, (a6)
pub fn i_06cfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CFEC: jsr $27d50.l
pub fn i_06cfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6CFF2);
    cpu.pc = 0x27D50;
    16
}
/// $06CFF2: bcc.w $6cffc
pub fn i_06cff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6CFFC; return 10; }
    8
}
/// $06CFF6: lea.l $6d04e(pc), a1
pub fn i_06cff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6D04E;
    16
}
/// $06CFFA: move.l a1, (a6)
pub fn i_06cffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6CFFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06CFFC: move.w $28(a6), d0
pub fn i_06cffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D000;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D000: move.w $2a(a6), d1
pub fn i_06d000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D004;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D004: asr.w #$4, d0
pub fn i_06d004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D006;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06D006: asr.w #$4, d1
pub fn i_06d006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D008;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06D008: jsr $5e018.l
pub fn i_06d008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D00E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D00E);
    cpu.pc = 0x5E018;
    16
}
/// $06D00E: lsr.w #$3, d0
pub fn i_06d00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D010;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D010: move.w d0, $34(a6)
pub fn i_06d010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D014;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D014: jsr $28d70.l
pub fn i_06d014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D01A);
    cpu.pc = 0x28D70;
    16
}
/// $06D01A: jsr $283d8.l
pub fn i_06d01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D020;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D020);
    cpu.pc = 0x283D8;
    16
}
/// $06D020: btst.b #$1, $13(a6)
pub fn i_06d020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D026;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D026: beq.w $6d030
pub fn i_06d026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D02A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D030; return 10; }
    8
}
/// $06D02A: lea.l $6ce66(pc), a1
pub fn i_06d02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D02E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6CE66;
    16
}
/// $06D02E: move.l a1, (a6)
pub fn i_06d02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D030;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D030: movea.l #$ffffffff, a0
pub fn i_06d030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D036;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06D036: lea.l $2ccf42.l, a0
pub fn i_06d036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D03C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CCF42;
    20
}
/// $06D03C: jsr $5dd56.l
pub fn i_06d03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D042);
    cpu.pc = 0x5DD56;
    16
}
/// $06D042: bcc.w $6d04c
pub fn i_06d042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D046;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6D04C; return 10; }
    8
}
// ===== SetTaskHandler_06d046  ($06D046, 8 B, kernel/task_handlers.c)
/// $06D046: lea.l $6c554(pc), a1
pub fn i_06d046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D04A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6C554;
    16
}
/// $06D04A: move.l a1, (a6)
pub fn i_06d04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D04C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06D04C: rts
pub fn i_06d04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D04E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_RocketHit_06d04e  ($06D04E, 14 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D04E: move.w #$1025, d0
pub fn i_06d04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D052;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1025;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D052: jsr $2352.l
pub fn i_06d052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D058);
    cpu.pc = 0x2352;
    16
}
/// $06D058: bra.w $6ce66
pub fn i_06d058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D05C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6CE66;
    10
}
// ===== RocketVehicle_OffWorldOrLeft_06d05c  ($06D05C, 32 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D05C: movea.l #$ffffffff, a0
pub fn i_06d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D062;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06D062: lea.l $2ccf3a.l, a0
pub fn i_06d062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D068;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2CCF3A;
    20
}
/// $06D068: jsr $5dd5c.l
pub fn i_06d068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D06E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D06E);
    cpu.pc = 0x5DD5C;
    16
}
/// $06D06E: bcs.w $6d082
pub fn i_06d06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D072;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6D082; return 10; }
    8
}
/// $06D072: cmpi.w #$ffdc, $22(a6)
pub fn i_06d072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D078;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFDC, Size::Word, false);
    16
}
/// $06D078: blt.w $6d082
pub fn i_06d078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D07C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6D082; return 10; }
    8
}
// ===== ClearXN_06d07c  ($06D07C, 6 B, thunks/ccr_helpers.c)
/// $06D07C: andi.b #$ee, ccr
pub fn i_06d07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D080;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D080: rts
pub fn i_06d080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D082;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06d082  ($06D082, 6 B, thunks/ccr_helpers.c)
/// $06D082: ori.b #$11, ccr
pub fn i_06d082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D086;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D086: rts
pub fn i_06d086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D088;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_Chain3Step_06d088  ($06D088, 32 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D088: jsr $30704.l
pub fn i_06d088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D08E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D08E);
    cpu.pc = 0x30704;
    16
}
/// $06D08E: beq.w $6d098
pub fn i_06d08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D092;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D098; return 10; }
    8
}
/// $06D092: jsr $281b0.l
pub fn i_06d092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D098;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D098);
    cpu.pc = 0x281B0;
    16
}
/// $06D098: jsr $2788c.l
pub fn i_06d098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D09E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D09E);
    cpu.pc = 0x2788C;
    16
}
/// $06D09E: jsr $3076a.l
pub fn i_06d09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D0A4);
    cpu.pc = 0x3076A;
    16
}
/// $06D0A4: beq.w $6d0ae
pub fn i_06d0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D0AE; return 10; }
    8
}
// ===== SetXN_06d0a8  ($06D0A8, 6 B, thunks/ccr_helpers.c)
/// $06D0A8: ori.b #$11, ccr
pub fn i_06d0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0AC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D0AC: rts
pub fn i_06d0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06d0ae  ($06D0AE, 6 B, thunks/ccr_helpers.c)
/// $06D0AE: andi.b #$ee, ccr
pub fn i_06d0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0B2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D0B2: rts
pub fn i_06d0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_SlopeAngle_06d0b4  ($06D0B4, 66 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D0B4: movea.l $74(a6), a1
pub fn i_06d0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0B8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06D0B8: movea.l $78(a6), a2
pub fn i_06d0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0BC;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $06D0BC: move.w $22(a1), d0
pub fn i_06d0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0C0;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D0C0: sub.w $22(a2), d0
pub fn i_06d0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0C4;
    cpu.ir = 0x906A;
    let ad2 = cpu.a[2].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D0C4: move.w $24(a1), d1
pub fn i_06d0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0C8;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D0C8: sub.w $24(a2), d1
pub fn i_06d0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0CC;
    cpu.ir = 0x926A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D0CC: move.w $24(a1), d2
pub fn i_06d0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0D0;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D0D0: add.w $24(a2), d2
pub fn i_06d0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0D4;
    cpu.ir = 0xD46A;
    let ad2 = cpu.a[2].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D0D4: lsr.w #$1, d2
pub fn i_06d0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0D6;
    cpu.ir = 0xE24A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06D0D6: move.w d2, $24(a6)
pub fn i_06d0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0DA;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D0DA: jsr $5e018.l
pub fn i_06d0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D0E0);
    cpu.pc = 0x5E018;
    16
}
/// $06D0E0: move.w d0, d1
pub fn i_06d0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0E2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D0E2: addq.w #$8, d1
pub fn i_06d0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0E4;
    cpu.ir = 0x5041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06D0E4: lsr.w #$4, d1
pub fn i_06d0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0E6;
    cpu.ir = 0xE849;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06D0E6: btst.b #$0, d1
pub fn i_06d0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0EA;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $06D0EA: bne.w $6d0f4
pub fn i_06d0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D0F4; return 10; }
    8
}
/// $06D0EE: addq.w #$4, d0
pub fn i_06d0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0F0;
    cpu.ir = 0x5840;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06D0F0: andi.w #$f8, d0
pub fn i_06d0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0F4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D0F4: rts
pub fn i_06d0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_GroundPhysics_06d0f6  ($06D0F6, 58 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D0F6: btst.b #$6, $13(a6)
pub fn i_06d0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D0FC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D0FC: bne.w $6d136
pub fn i_06d0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D100;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D136; return 10; }
    8
}
/// $06D100: jsr $2785c.l
pub fn i_06d100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D106);
    cpu.pc = 0x2785C;
    16
}
/// $06D106: jsr $2a8c0.l
pub fn i_06d106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D10C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D10C);
    cpu.pc = 0x2A8C0;
    16
}
/// $06D10C: bcc.w $6d120
pub fn i_06d10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D110;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6D120; return 10; }
    8
}
/// $06D110: move.w #$ffc0, $2e(a6)
pub fn i_06d110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D116;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D116: jsr $27a18.l
pub fn i_06d116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D11C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D11C);
    cpu.pc = 0x27A18;
    16
}
/// $06D11C: bra.w $6d136
pub fn i_06d11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D120;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D136;
    10
}
/// $06D120: jsr $6d088(pc)
pub fn i_06d120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D124;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D124);
    cpu.pc = 0x6D088;
    16
}
/// $06D124: jsr $6d0b4(pc)
pub fn i_06d124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D128;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D128);
    cpu.pc = 0x6D0B4;
    16
}
/// $06D128: move.b d0, $89(a6)
pub fn i_06d128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D12C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D12C: jsr $6d1a4(pc)
pub fn i_06d12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D130;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D130);
    cpu.pc = 0x6D1A4;
    16
}
// ===== JsrPcThunk_06d130  ($06D130, 6 B, thunks/jsr_pc_thunks.c)
/// $06D130: jsr $6d13c(pc)
pub fn i_06d130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D134;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D134);
    cpu.pc = 0x6D13C;
    16
}
/// $06D134: rts
pub fn i_06d134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D136;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06d136  ($06D136, 6 B, thunks/ccr_helpers.c)
/// $06D136: andi.b #$ee, ccr
pub fn i_06d136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D13A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D13A: rts
pub fn i_06d13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D13C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_LinkBlocked_06d13c  ($06D13C, 42 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D13C: btst.b #$0, $8d(a6)
pub fn i_06d13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D142;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D142: beq.w $6d158
pub fn i_06d142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D146;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D158; return 10; }
    8
}
/// $06D146: movea.l $74(a6), a1
pub fn i_06d146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D14A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06D14A: btst.b #$5, $5a(a1)
pub fn i_06d14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D150;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D150: bne.w $6d16c
pub fn i_06d150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D154;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D16C; return 10; }
    8
}
/// $06D154: bra.w $6d166
pub fn i_06d154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D158;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D166;
    10
}
/// $06D158: movea.l $78(a6), a1
pub fn i_06d158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D15C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06D15C: btst.b #$5, $5a(a1)
pub fn i_06d15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D162;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D162: bne.w $6d16c
pub fn i_06d162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D166;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D16C; return 10; }
    8
}
// ===== ClearXN_06d166  ($06D166, 6 B, thunks/ccr_helpers.c)
/// $06D166: andi.b #$ee, ccr
pub fn i_06d166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D16A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D16A: rts
pub fn i_06d16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D16C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06d16c  ($06D16C, 6 B, thunks/ccr_helpers.c)
/// $06D16C: ori.b #$11, ccr
pub fn i_06d16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D170;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D170: rts
pub fn i_06d170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D172;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_WallAhead_06d172  ($06D172, 38 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D172: cmpi.w #$0, $28(a6)
pub fn i_06d172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D178;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06D178: beq.w $6d198
pub fn i_06d178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D17C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D198; return 10; }
    8
}
/// $06D17C: bgt.w $6d18e
pub fn i_06d17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D180;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D18E; return 10; }
    8
}
/// $06D180: btst.b #$1, $69(a6)
pub fn i_06d180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D186;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D186: bne.w $6d19e
pub fn i_06d186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D18A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D19E; return 10; }
    8
}
/// $06D18A: bra.w $6d198
pub fn i_06d18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D18E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6D198;
    10
}
/// $06D18E: btst.b #$0, $69(a6)
pub fn i_06d18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D194;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D194: bne.w $6d19e
pub fn i_06d194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D198;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D19E; return 10; }
    8
}
// ===== ClearXN_06d198  ($06D198, 6 B, thunks/ccr_helpers.c)
/// $06D198: andi.b #$ee, ccr
pub fn i_06d198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D19C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D19C: rts
pub fn i_06d19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D19E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06d19e  ($06D19E, 6 B, thunks/ccr_helpers.c)
/// $06D19E: ori.b #$11, ccr
pub fn i_06d19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1A2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D1A2: rts
pub fn i_06d1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_SlopeToSpeed_06d1a4  ($06D1A4, 50 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D1A4: addi.b #$20, d0
pub fn i_06d1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1A8;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06D1A8: cmpi.b #$0, d0
pub fn i_06d1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1AC;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $06D1AC: bge.w $6d1b2
pub fn i_06d1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1B0;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x6D1B2; return 10; }
    8
}
/// $06D1B0: clr.b d0
pub fn i_06d1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1B2;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $06D1B2: andi.w #$ff, d0
pub fn i_06d1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1B6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D1B6: cmpi.w #$40, d0
pub fn i_06d1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1BA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $06D1BA: blt.w $6d1c2
pub fn i_06d1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1BE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6D1C2; return 10; }
    8
}
/// $06D1BE: move.w #$40, d0
pub fn i_06d1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1C2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D1C2: btst.b #$0, $3a(a6)
pub fn i_06d1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1C8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D1C8: bne.w $6d1d4
pub fn i_06d1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1CC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D1D4; return 10; }
    8
}
/// $06D1CC: move.w #$40, d2
pub fn i_06d1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1D0;
    cpu.ir = 0x343C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D1D0: sub.w d0, d2
pub fn i_06d1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1D2;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06D1D2: move.w d2, d0
pub fn i_06d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1D4;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D1D4: lsr.w #$2, d0
pub fn i_06d1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1D6;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
// ===== SetTaskW_06d1d6  ($06D1D6, 6 B, kernel/task_setters_w.c)
/// $06D1D6: move.w d0, $8a(a6)
pub fn i_06d1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1DA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06D1DA: rts
pub fn i_06d1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_WheelPhase_06d1dc  ($06D1DC, 26 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D1DC: move.w $8a(a6), d0
pub fn i_06d1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1E0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D1E0: andi.w #$7, d0
pub fn i_06d1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1E4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D1E4: bne.w $6d1fc
pub fn i_06d1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1E8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D1FC; return 10; }
    8
}
/// $06D1E8: move.w $8a(a6), d0
pub fn i_06d1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1EC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D1EC: andi.w #$1f, d0
pub fn i_06d1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1F0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D1F0: lsr.w #$3, d0
pub fn i_06d1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1F2;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D1F2: andi.w #$3, d0
pub fn i_06d1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1F6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== ClearXN_06d1f6  ($06D1F6, 6 B, thunks/ccr_helpers.c)
/// $06D1F6: andi.b #$ee, ccr
pub fn i_06d1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1FA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D1FA: rts
pub fn i_06d1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D1FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_WheelPhase3_06d1fc  ($06D1FC, 4 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D1FC: move.w #$3, d0
pub fn i_06d1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D200;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== SetXN_06d200  ($06D200, 6 B, thunks/ccr_helpers.c)
/// $06D200: ori.b #$11, ccr
pub fn i_06d200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D204;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D204: rts
pub fn i_06d204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D206;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_FacingMismatch_06d206  ($06D206, 22 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D206: move.b $8d(a6), d0
pub fn i_06d206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D20A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D20A: move.b $3a(a6), d1
pub fn i_06d20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D20E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D20E: andi.b #$1, d0
pub fn i_06d20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D212;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06D212: andi.b #$1, d1
pub fn i_06d212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D216;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06D216: cmp.b d0, d1
pub fn i_06d216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D218;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $06D218: bne.w $6d222
pub fn i_06d218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D21C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D222; return 10; }
    8
}
// ===== ClearXN_06d21c  ($06D21C, 6 B, thunks/ccr_helpers.c)
/// $06D21C: andi.b #$ee, ccr
pub fn i_06d21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D220;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D220: rts
pub fn i_06d220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D222;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06d222  ($06D222, 6 B, thunks/ccr_helpers.c)
/// $06D222: ori.b #$11, ccr
pub fn i_06d222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D226;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D226: rts
pub fn i_06d226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D228;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_AttackTimerInRange_06d228  ($06D228, 104 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D228: subq.w #$1, $70(a6)
pub fn i_06d228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D22C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D22C: cmpi.w #$0, $70(a6)
pub fn i_06d22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D232;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06D232: bgt.w $6d296
pub fn i_06d232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D236;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D296; return 10; }
    8
}
/// $06D236: jsr $5e0d4.l
pub fn i_06d236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D23C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D23C);
    cpu.pc = 0x5E0D4;
    16
}
/// $06D23C: move.w $24(a0), d0
pub fn i_06d23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D240;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D240: sub.w $24(a6), d0
pub fn i_06d240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D244;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D244: cmpi.w #$0, d0
pub fn i_06d244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D248;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06D248: bgt.w $6d24e
pub fn i_06d248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D24C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D24E; return 10; }
    8
}
/// $06D24C: neg.w d0
pub fn i_06d24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D24E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06D24E: cmpi.w #$80, d0
pub fn i_06d24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D252;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $06D252: ble.w $6d25a
pub fn i_06d252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D256;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6D25A; return 10; }
    8
}
/// $06D256: move.w #$80, d0
pub fn i_06d256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D25A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D25A: lsr.w #$5, d0
pub fn i_06d25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D25C;
    cpu.ir = 0xEA48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $06D25C: lea.l $2ccfc6.l, a1
pub fn i_06d25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D262;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CCFC6;
    20
}
/// $06D262: move.b $98(a6), d1
pub fn i_06d262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D266;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D266: andi.w #$3, d1
pub fn i_06d266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D26A;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D26A: lsl.w #$2, d1
pub fn i_06d26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D26C;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06D26C: movea.l (a1, d1.w), a2
pub fn i_06d26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D270;
    cpu.ir = 0x2471;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $06D270: move.b (a2, d0.w), d1
pub fn i_06d270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D274;
    cpu.ir = 0x1232;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $06D274: andi.w #$ff, d1
pub fn i_06d274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D278;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D278: move.w d1, d2
pub fn i_06d278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D27A;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D27A: neg.w d1
pub fn i_06d27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D27C;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06D27C: move.w $22(a0), d0
pub fn i_06d27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D280;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D280: sub.w $22(a6), d0
pub fn i_06d280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D284;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D284: cmp.w d1, d0
pub fn i_06d284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D286;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $06D286: blt.w $6d296
pub fn i_06d286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D28A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6D296; return 10; }
    8
}
/// $06D28A: cmp.w d2, d0
pub fn i_06d28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D28C;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $06D28C: bgt.w $6d296
pub fn i_06d28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D290;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D296; return 10; }
    8
}
// ===== SetXN_06d290  ($06D290, 6 B, thunks/ccr_helpers.c)
/// $06D290: ori.b #$11, ccr
pub fn i_06d290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D294;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D294: rts
pub fn i_06d294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D296;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06d296  ($06D296, 6 B, thunks/ccr_helpers.c)
/// $06D296: andi.b #$ee, ccr
pub fn i_06d296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D29A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D29A: rts
pub fn i_06d29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D29C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_RiderFollowParent_06d29c  ($06D29C, 94 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D29C: jsr $5e506.l
pub fn i_06d29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D2A2);
    cpu.pc = 0x5E506;
    16
}
/// $06D2A2: movea.l $c(a6), a0
pub fn i_06d2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2A6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06D2A6: move.w $8a(a0), $8a(a6)
pub fn i_06d2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2AC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x8A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x8A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06D2AC: move.b $8e(a0), d0
pub fn i_06d2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2B0;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x8E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D2B0: ext.w d0
pub fn i_06d2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2B2;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06D2B2: btst.b #$0, $3a(a6)
pub fn i_06d2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2B8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D2B8: beq.w $6d2be
pub fn i_06d2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D2BE; return 10; }
    8
}
/// $06D2BC: neg.w d0
pub fn i_06d2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2BE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06D2BE: add.w d0, $22(a6)
pub fn i_06d2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2C2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D2C2: move.b $8f(a0), d0
pub fn i_06d2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2C6;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x8F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D2C6: ext.w d0
pub fn i_06d2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2C8;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06D2C8: add.w d0, $24(a6)
pub fn i_06d2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2CC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D2CC: move.w $34(a6), d0
pub fn i_06d2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2D0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D2D0: andi.w #$f, d0
pub fn i_06d2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2D4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D2D4: beq.w $6d2e0
pub fn i_06d2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D2E0; return 10; }
    8
}
/// $06D2D8: cmpi.w #$8, d0
pub fn i_06d2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2DC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $06D2DC: bcs.w $6d2e4
pub fn i_06d2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6D2E4; return 10; }
    8
}
/// $06D2E0: subq.w #$1, $38(a6)
pub fn i_06d2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2E4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06D2E4: movea.l $c(a6), a0
pub fn i_06d2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06D2E8: btst.b #$7, $5a(a0)
pub fn i_06d2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2EE;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D2EE: beq.w $6d2f8
pub fn i_06d2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D2F8; return 10; }
    8
}
/// $06D2F2: bset.b #$0, $5a(a6)
pub fn i_06d2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2F8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D2F8: rts
pub fn i_06d2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2FA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_RangeClass_06d2fa  ($06D2FA, 70 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D2FA: tst.b $9f(a6)
pub fn i_06d2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D2FE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9F);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06D2FE: beq.w $6d33e
pub fn i_06d2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D302;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D33E; return 10; }
    8
}
/// $06D302: movea.l $90(a6), a0
pub fn i_06d302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D306;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06D306: move.w $22(a6), d0
pub fn i_06d306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D30A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D30A: move.w $24(a6), d1
pub fn i_06d30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D30E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D30E: move.w $22(a0), d2
pub fn i_06d30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D312;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D312: move.w $24(a0), d3
pub fn i_06d312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D316;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D316: jsr $5e23a.l
pub fn i_06d316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D31C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D31C);
    cpu.pc = 0x5E23A;
    16
}
/// $06D31C: clr.b d1
pub fn i_06d31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D31E;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $06D31E: cmpi.w #$70, d0
pub fn i_06d31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D322;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x70, Size::Word, false);
    8
}
/// $06D322: blt.w $6d336
pub fn i_06d322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D326;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6D336; return 10; }
    8
}
/// $06D326: move.b #$1, d1
pub fn i_06d326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D32A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06D32A: cmpi.w #$b0, d0
pub fn i_06d32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D32E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xB0, Size::Word, false);
    8
}
/// $06D32E: blt.w $6d336
pub fn i_06d32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D332;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6D336; return 10; }
    8
}
/// $06D332: move.b #$2, d1
pub fn i_06d332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D336;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06D336: andi.b #$3, d1
pub fn i_06d336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D33A;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06D33A: move.b d1, $98(a6)
pub fn i_06d33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D33E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D33E: rts
pub fn i_06d33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D340;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_AimStep_06d340  ($06D340, 156 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D340: movea.l $90(a6), a0
pub fn i_06d340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D344;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06D344: lea.l $2cd4ea.l, a1
pub fn i_06d344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D34A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CD4EA;
    20
}
/// $06D34A: btst.b #$0, $3a(a6)
pub fn i_06d34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D350;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D350: beq.w $6d35a
pub fn i_06d350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D354;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D35A; return 10; }
    8
}
/// $06D354: lea.l $2cd50a.l, a1
pub fn i_06d354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D35A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CD50A;
    20
}
/// $06D35A: move.b $97(a6), d1
pub fn i_06d35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D35E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06D35E: andi.w #$3, d1
pub fn i_06d35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D362;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D362: move.w $24(a0), d0
pub fn i_06d362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D366;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D366: cmp.w $24(a6), d0
pub fn i_06d366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D36A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06D36A: bgt.w $6d370
pub fn i_06d36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D36E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D370; return 10; }
    8
}
/// $06D36E: addq.w #$4, d1
pub fn i_06d36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D370;
    cpu.ir = 0x5841;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06D370: lsl.w #$2, d1
pub fn i_06d370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D372;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06D372: movea.l (a1, d1.w), a1
pub fn i_06d372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D376;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $06D376: move.w $22(a0), d0
pub fn i_06d376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D37A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D37A: sub.w $22(a6), d0
pub fn i_06d37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D37E;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D37E: cmpi.w #$ff00, d0
pub fn i_06d37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D382;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF00, Size::Word, false);
    8
}
/// $06D382: bgt.w $6d38a
pub fn i_06d382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D386;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D38A; return 10; }
    8
}
/// $06D386: move.w #$ff00, d0
pub fn i_06d386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D38A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D38A: cmpi.w #$100, d0
pub fn i_06d38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D38E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $06D38E: blt.w $6d396
pub fn i_06d38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D392;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6D396; return 10; }
    8
}
/// $06D392: move.w #$100, d0
pub fn i_06d392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D396;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D396: addi.w #$100, d0
pub fn i_06d396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D39A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D39A: lsr.w #$4, d0
pub fn i_06d39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D39C;
    cpu.ir = 0xE848;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06D39C: move.b (a1, d0.w), d0
pub fn i_06d39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3A0;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $06D3A0: btst.b #$0, $3a(a6)
pub fn i_06d3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3A6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D3A6: beq.w $6d3b4
pub fn i_06d3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D3B4; return 10; }
    8
}
/// $06D3AA: lea.l $2cd5f6.l, a1
pub fn i_06d3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3B0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2CD5F6;
    20
}
/// $06D3B0: move.b (a1, d0.w), d0
pub fn i_06d3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3B4;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $06D3B4: jsr $6d3e8(pc)
pub fn i_06d3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6D3B8);
    cpu.pc = 0x6D3E8;
    16
}
/// $06D3B8: andi.w #$f, d0
pub fn i_06d3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3BC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06D3BC: cmp.w $34(a6), d0
pub fn i_06d3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3C0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06D3C0: beq.w $6d3e2
pub fn i_06d3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D3E2; return 10; }
    8
}
/// $06D3C4: sub.w $34(a6), d0
pub fn i_06d3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3C8;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D3C8: move.w #$1, d1
pub fn i_06d3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3CC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D3CC: cmpi.w #$0, d0
pub fn i_06d3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3D0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06D3D0: bgt.w $6d3d8
pub fn i_06d3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3D4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D3D8; return 10; }
    8
}
/// $06D3D4: move.w #$ffff, d1
pub fn i_06d3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3D8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06D3D8: add.w d1, $34(a6)
pub fn i_06d3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3DC;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== ClearXN_06d3dc  ($06D3DC, 6 B, thunks/ccr_helpers.c)
/// $06D3DC: andi.b #$ee, ccr
pub fn i_06d3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3E0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D3E0: rts
pub fn i_06d3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06d3e2  ($06D3E2, 6 B, thunks/ccr_helpers.c)
/// $06D3E2: ori.b #$11, ccr
pub fn i_06d3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3E6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D3E6: rts
pub fn i_06d3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_AimClampNear_06d3e8  ($06D3E8, 78 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D3E8: cmpi.b #$1, $97(a6)
pub fn i_06d3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3EE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06D3EE: bne.w $6d440
pub fn i_06d3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6D440; return 10; }
    8
}
/// $06D3F2: move.b d0, $5c(a6)
pub fn i_06d3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3F6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06D3F6: movea.l $90(a6), a0
pub fn i_06d3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3FA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06D3FA: move.w $24(a0), d0
pub fn i_06d3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D3FE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D3FE: sub.w $24(a6), d0
pub fn i_06d3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D402;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D402: cmpi.w #$fff8, d0
pub fn i_06d402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D406;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFF8, Size::Word, false);
    8
}
/// $06D406: blt.w $6d43c
pub fn i_06d406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D40A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6D43C; return 10; }
    8
}
/// $06D40A: cmpi.w #$28, d0
pub fn i_06d40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D40E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x28, Size::Word, false);
    8
}
/// $06D40E: bgt.w $6d43c
pub fn i_06d40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D412;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D43C; return 10; }
    8
}
/// $06D412: move.w $22(a0), d0
pub fn i_06d412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D416;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06D416: sub.w $22(a6), d0
pub fn i_06d416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D41A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06D41A: btst.b #$0, $3a(a6)
pub fn i_06d41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D420;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06D420: beq.w $6d426
pub fn i_06d420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D424;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6D426; return 10; }
    8
}
/// $06D424: neg.w d0
pub fn i_06d424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D426;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06D426: clr.b d1
pub fn i_06d426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D428;
    cpu.ir = 0x4201;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $06D428: cmpi.w #$0, d0
pub fn i_06d428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D42C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $06D42C: bgt.w $6d434
pub fn i_06d42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D430;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6D434; return 10; }
    8
}
/// $06D430: move.b #$8, d1
pub fn i_06d430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D434;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06D434: move.b d1, d0
pub fn i_06d434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D436;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== SetXN_06d436  ($06D436, 6 B, thunks/ccr_helpers.c)
/// $06D436: ori.b #$11, ccr
pub fn i_06d436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D43A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06D43A: rts
pub fn i_06d43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D43C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_AimRestore_06d43c  ($06D43C, 4 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D43C: move.b $5c(a6), d0
pub fn i_06d43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D440;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
// ===== ClearXN_06d440  ($06D440, 6 B, thunks/ccr_helpers.c)
/// $06D440: andi.b #$ee, ccr
pub fn i_06d440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D444;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06D444: rts
pub fn i_06d444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D446;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_SetFlags45_06d446  ($06D446, 14 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D446: bset.b #$5, $13(a6)
pub fn i_06d446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D44C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D44C: bset.b #$4, $13(a6)
pub fn i_06d44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D452;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D452: rts
pub fn i_06d452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RocketVehicle_RandFlags45_06d454  ($06D454, 48 B, enemies/soldiers/bazooka_rocketvehicle_walker_06a000.s)
/// $06D454: jsr $5e9b6.l
pub fn i_06d454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D45A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6D45A);
    cpu.pc = 0x5E9B6;
    16
}
/// $06D45A: bset.b #$4, $13(a6)
pub fn i_06d45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D460;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06D460: btst.b #$0, d0
pub fn i_06d460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6D464;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x06B9BA, i_06b9ba),
    (0x06B9C0, i_06b9c0),
    (0x06B9C6, i_06b9c6),
    (0x06B9CC, i_06b9cc),
    (0x06B9D2, i_06b9d2),
    (0x06B9D8, i_06b9d8),
    (0x06B9DA, i_06b9da),
    (0x06B9DE, i_06b9de),
    (0x06B9E4, i_06b9e4),
    (0x06B9EA, i_06b9ea),
    (0x06B9F0, i_06b9f0),
    (0x06B9F6, i_06b9f6),
    (0x06B9FC, i_06b9fc),
    (0x06BA02, i_06ba02),
    (0x06BA08, i_06ba08),
    (0x06BA0E, i_06ba0e),
    (0x06BA10, i_06ba10),
    (0x06BA14, i_06ba14),
    (0x06BA18, i_06ba18),
    (0x06BA1C, i_06ba1c),
    (0x06BA20, i_06ba20),
    (0x06BA24, i_06ba24),
    (0x06BA26, i_06ba26),
    (0x06BA2A, i_06ba2a),
    (0x06BA2C, i_06ba2c),
    (0x06BA30, i_06ba30),
    (0x06BA34, i_06ba34),
    (0x06BA3A, i_06ba3a),
    (0x06BA40, i_06ba40),
    (0x06BA46, i_06ba46),
    (0x06BA4A, i_06ba4a),
    (0x06BA4E, i_06ba4e),
    (0x06BA52, i_06ba52),
    (0x06BA56, i_06ba56),
    (0x06BA5A, i_06ba5a),
    (0x06BA60, i_06ba60),
    (0x06BA64, i_06ba64),
    (0x06BA6A, i_06ba6a),
    (0x06BA70, i_06ba70),
    (0x06BA74, i_06ba74),
    (0x06BA7A, i_06ba7a),
    (0x06BA80, i_06ba80),
    (0x06BA84, i_06ba84),
    (0x06BA8A, i_06ba8a),
    (0x06BA90, i_06ba90),
    (0x06BA94, i_06ba94),
    (0x06BA98, i_06ba98),
    (0x06BA9E, i_06ba9e),
    (0x06BAA4, i_06baa4),
    (0x06BAAA, i_06baaa),
    (0x06BAB2, i_06bab2),
    (0x06BABA, i_06baba),
    (0x06BABE, i_06babe),
    (0x06BAC4, i_06bac4),
    (0x06BACA, i_06baca),
    (0x06BAD0, i_06bad0),
    (0x06BAD6, i_06bad6),
    (0x06BADA, i_06bada),
    (0x06BAE0, i_06bae0),
    (0x06BAE6, i_06bae6),
    (0x06BAEC, i_06baec),
    (0x06BAF2, i_06baf2),
    (0x06BAF8, i_06baf8),
    (0x06BAFE, i_06bafe),
    (0x06BB02, i_06bb02),
    (0x06BB08, i_06bb08),
    (0x06BB0C, i_06bb0c),
    (0x06BB12, i_06bb12),
    (0x06BB16, i_06bb16),
    (0x06BB18, i_06bb18),
    (0x06BB1C, i_06bb1c),
    (0x06BB20, i_06bb20),
    (0x06BB24, i_06bb24),
    (0x06BB28, i_06bb28),
    (0x06BB2C, i_06bb2c),
    (0x06BB30, i_06bb30),
    (0x06BB34, i_06bb34),
    (0x06BB3A, i_06bb3a),
    (0x06BB3C, i_06bb3c),
    (0x06BB40, i_06bb40),
    (0x06BB46, i_06bb46),
    (0x06BB4A, i_06bb4a),
    (0x06BB50, i_06bb50),
    (0x06BB54, i_06bb54),
    (0x06BB68, i_06bb68),
    (0x06BB6C, i_06bb6c),
    (0x06BB70, i_06bb70),
    (0x06BB74, i_06bb74),
    (0x06BB78, i_06bb78),
    (0x06BB7C, i_06bb7c),
    (0x06BB82, i_06bb82),
    (0x06BB84, i_06bb84),
    (0x06BB88, i_06bb88),
    (0x06BB8E, i_06bb8e),
    (0x06BB92, i_06bb92),
    (0x06BB98, i_06bb98),
    (0x06BB9C, i_06bb9c),
    (0x06BBB0, i_06bbb0),
    (0x06BBB4, i_06bbb4),
    (0x06BBB6, i_06bbb6),
    (0x06BBBC, i_06bbbc),
    (0x06BBC0, i_06bbc0),
    (0x06BBC4, i_06bbc4),
    (0x06BBCA, i_06bbca),
    (0x06BBCE, i_06bbce),
    (0x06BBD0, i_06bbd0),
    (0x06BBD4, i_06bbd4),
    (0x06BBD8, i_06bbd8),
    (0x06BBDC, i_06bbdc),
    (0x06BBE0, i_06bbe0),
    (0x06BBE4, i_06bbe4),
    (0x06BBE8, i_06bbe8),
    (0x06BBEC, i_06bbec),
    (0x06BBF0, i_06bbf0),
    (0x06BBF4, i_06bbf4),
    (0x06BBF8, i_06bbf8),
    (0x06BBFE, i_06bbfe),
    (0x06BC00, i_06bc00),
    (0x06BC04, i_06bc04),
    (0x06BC0A, i_06bc0a),
    (0x06BC0E, i_06bc0e),
    (0x06BC14, i_06bc14),
    (0x06BC18, i_06bc18),
    (0x06BC2C, i_06bc2c),
    (0x06BC30, i_06bc30),
    (0x06BC34, i_06bc34),
    (0x06BC38, i_06bc38),
    (0x06BC3C, i_06bc3c),
    (0x06BC40, i_06bc40),
    (0x06BC46, i_06bc46),
    (0x06BC48, i_06bc48),
    (0x06BC4C, i_06bc4c),
    (0x06BC52, i_06bc52),
    (0x06BC56, i_06bc56),
    (0x06BC5C, i_06bc5c),
    (0x06BC60, i_06bc60),
    (0x06BC74, i_06bc74),
    (0x06BC7A, i_06bc7a),
    (0x06BC7E, i_06bc7e),
    (0x06BC82, i_06bc82),
    (0x06BC88, i_06bc88),
    (0x06BC8C, i_06bc8c),
    (0x06BC8E, i_06bc8e),
    (0x06BC92, i_06bc92),
    (0x06BC98, i_06bc98),
    (0x06BC9C, i_06bc9c),
    (0x06BCA2, i_06bca2),
    (0x06BCA6, i_06bca6),
    (0x06BCAC, i_06bcac),
    (0x06BCB0, i_06bcb0),
    (0x06BCB4, i_06bcb4),
    (0x06BCB6, i_06bcb6),
    (0x06BCBC, i_06bcbc),
    (0x06BCC0, i_06bcc0),
    (0x06BCC6, i_06bcc6),
    (0x06BCCA, i_06bcca),
    (0x06BCD0, i_06bcd0),
    (0x06BCD4, i_06bcd4),
    (0x06BCD8, i_06bcd8),
    (0x06BCDA, i_06bcda),
    (0x06BCDE, i_06bcde),
    (0x06BCE2, i_06bce2),
    (0x06BCE6, i_06bce6),
    (0x06BCEA, i_06bcea),
    (0x06BCEC, i_06bcec),
    (0x06BCF2, i_06bcf2),
    (0x06BCF8, i_06bcf8),
    (0x06BCFC, i_06bcfc),
    (0x06BD02, i_06bd02),
    (0x06BD06, i_06bd06),
    (0x06BD0A, i_06bd0a),
    (0x06BD0E, i_06bd0e),
    (0x06BD12, i_06bd12),
    (0x06BD16, i_06bd16),
    (0x06BD1A, i_06bd1a),
    (0x06BD1E, i_06bd1e),
    (0x06BD22, i_06bd22),
    (0x06BD28, i_06bd28),
    (0x06BD2A, i_06bd2a),
    (0x06BD2E, i_06bd2e),
    (0x06BD34, i_06bd34),
    (0x06BD38, i_06bd38),
    (0x06BD3E, i_06bd3e),
    (0x06BD42, i_06bd42),
    (0x06BD56, i_06bd56),
    (0x06BD5A, i_06bd5a),
    (0x06BD5E, i_06bd5e),
    (0x06BD62, i_06bd62),
    (0x06BD66, i_06bd66),
    (0x06BD6A, i_06bd6a),
    (0x06BD70, i_06bd70),
    (0x06BD72, i_06bd72),
    (0x06BD76, i_06bd76),
    (0x06BD7C, i_06bd7c),
    (0x06BD80, i_06bd80),
    (0x06BD86, i_06bd86),
    (0x06BD8A, i_06bd8a),
    (0x06BD9E, i_06bd9e),
    (0x06BDA2, i_06bda2),
    (0x06BDA6, i_06bda6),
    (0x06BDA8, i_06bda8),
    (0x06BDAE, i_06bdae),
    (0x06BDB2, i_06bdb2),
    (0x06BDB6, i_06bdb6),
    (0x06BDBA, i_06bdba),
    (0x06BDBE, i_06bdbe),
    (0x06BDC2, i_06bdc2),
    (0x06BDC6, i_06bdc6),
    (0x06BDC8, i_06bdc8),
    (0x06BDCE, i_06bdce),
    (0x06BDD2, i_06bdd2),
    (0x06BDD6, i_06bdd6),
    (0x06BDD8, i_06bdd8),
    (0x06BDDE, i_06bdde),
    (0x06BDE4, i_06bde4),
    (0x06BDE8, i_06bde8),
    (0x06BDEE, i_06bdee),
    (0x06BDF2, i_06bdf2),
    (0x06BDF6, i_06bdf6),
    (0x06BDFA, i_06bdfa),
    (0x06BDFE, i_06bdfe),
    (0x06BE02, i_06be02),
    (0x06BE06, i_06be06),
    (0x06BE0A, i_06be0a),
    (0x06BE0E, i_06be0e),
    (0x06BE14, i_06be14),
    (0x06BE16, i_06be16),
    (0x06BE1A, i_06be1a),
    (0x06BE20, i_06be20),
    (0x06BE24, i_06be24),
    (0x06BE2A, i_06be2a),
    (0x06BE2E, i_06be2e),
    (0x06BE42, i_06be42),
    (0x06BE46, i_06be46),
    (0x06BE4A, i_06be4a),
    (0x06BE4E, i_06be4e),
    (0x06BE52, i_06be52),
    (0x06BE56, i_06be56),
    (0x06BE5C, i_06be5c),
    (0x06BE5E, i_06be5e),
    (0x06BE62, i_06be62),
    (0x06BE68, i_06be68),
    (0x06BE6C, i_06be6c),
    (0x06BE72, i_06be72),
    (0x06BE76, i_06be76),
    (0x06BE8A, i_06be8a),
    (0x06BE8E, i_06be8e),
    (0x06BE90, i_06be90),
    (0x06BE96, i_06be96),
    (0x06BE9A, i_06be9a),
    (0x06BE9E, i_06be9e),
    (0x06BEA4, i_06bea4),
    (0x06BEA8, i_06bea8),
    (0x06BEAA, i_06beaa),
    (0x06BEAE, i_06beae),
    (0x06BEB2, i_06beb2),
    (0x06BEB6, i_06beb6),
    (0x06BEBA, i_06beba),
    (0x06BEBE, i_06bebe),
    (0x06BEC0, i_06bec0),
    (0x06BEC6, i_06bec6),
    (0x06BECA, i_06beca),
    (0x06BECE, i_06bece),
    (0x06BED0, i_06bed0),
    (0x06BED6, i_06bed6),
    (0x06BEDC, i_06bedc),
    (0x06BEE0, i_06bee0),
    (0x06BEE6, i_06bee6),
    (0x06BEEC, i_06beec),
    (0x06BEF2, i_06bef2),
    (0x06BEF6, i_06bef6),
    (0x06BEFA, i_06befa),
    (0x06BEFE, i_06befe),
    (0x06BF02, i_06bf02),
    (0x06BF06, i_06bf06),
    (0x06BF0A, i_06bf0a),
    (0x06BF0E, i_06bf0e),
    (0x06BF12, i_06bf12),
    (0x06BF18, i_06bf18),
    (0x06BF1A, i_06bf1a),
    (0x06BF1E, i_06bf1e),
    (0x06BF24, i_06bf24),
    (0x06BF28, i_06bf28),
    (0x06BF2E, i_06bf2e),
    (0x06BF32, i_06bf32),
    (0x06BF46, i_06bf46),
    (0x06BF4A, i_06bf4a),
    (0x06BF4C, i_06bf4c),
    (0x06BF52, i_06bf52),
    (0x06BF56, i_06bf56),
    (0x06BF5A, i_06bf5a),
    (0x06BF5E, i_06bf5e),
    (0x06BF62, i_06bf62),
    (0x06BF66, i_06bf66),
    (0x06BF6A, i_06bf6a),
    (0x06BF6E, i_06bf6e),
    (0x06BF72, i_06bf72),
    (0x06BF76, i_06bf76),
    (0x06BF7C, i_06bf7c),
    (0x06BF7E, i_06bf7e),
    (0x06BF82, i_06bf82),
    (0x06BF88, i_06bf88),
    (0x06BF8C, i_06bf8c),
    (0x06BF92, i_06bf92),
    (0x06BF96, i_06bf96),
    (0x06BFAA, i_06bfaa),
    (0x06BFB0, i_06bfb0),
    (0x06BFB4, i_06bfb4),
    (0x06BFB8, i_06bfb8),
    (0x06BFBC, i_06bfbc),
    (0x06BFBE, i_06bfbe),
    (0x06BFC2, i_06bfc2),
    (0x06BFC6, i_06bfc6),
    (0x06BFCA, i_06bfca),
    (0x06BFCC, i_06bfcc),
    (0x06BFD0, i_06bfd0),
    (0x06BFD6, i_06bfd6),
    (0x06BFDC, i_06bfdc),
    (0x06BFE0, i_06bfe0),
    (0x06BFE6, i_06bfe6),
    (0x06BFEA, i_06bfea),
    (0x06BFEE, i_06bfee),
    (0x06BFF2, i_06bff2),
    (0x06BFF6, i_06bff6),
    (0x06BFFA, i_06bffa),
    (0x06BFFE, i_06bffe),
    (0x06C002, i_06c002),
    (0x06C006, i_06c006),
    (0x06C00C, i_06c00c),
    (0x06C00E, i_06c00e),
    (0x06C012, i_06c012),
    (0x06C018, i_06c018),
    (0x06C01C, i_06c01c),
    (0x06C022, i_06c022),
    (0x06C026, i_06c026),
    (0x06C03A, i_06c03a),
    (0x06C03E, i_06c03e),
    (0x06C040, i_06c040),
    (0x06C046, i_06c046),
    (0x06C04A, i_06c04a),
    (0x06C050, i_06c050),
    (0x06C054, i_06c054),
    (0x06C058, i_06c058),
    (0x06C05C, i_06c05c),
    (0x06C05E, i_06c05e),
    (0x06C062, i_06c062),
    (0x06C068, i_06c068),
    (0x06C06E, i_06c06e),
    (0x06C072, i_06c072),
    (0x06C076, i_06c076),
    (0x06C07A, i_06c07a),
    (0x06C080, i_06c080),
    (0x06C084, i_06c084),
    (0x06C088, i_06c088),
    (0x06C08E, i_06c08e),
    (0x06C094, i_06c094),
    (0x06C09A, i_06c09a),
    (0x06C09E, i_06c09e),
    (0x06C0A2, i_06c0a2),
    (0x06C0A6, i_06c0a6),
    (0x06C0AA, i_06c0aa),
    (0x06C0AE, i_06c0ae),
    (0x06C0B4, i_06c0b4),
    (0x06C0B6, i_06c0b6),
    (0x06C0BA, i_06c0ba),
    (0x06C0C0, i_06c0c0),
    (0x06C0C4, i_06c0c4),
    (0x06C0CA, i_06c0ca),
    (0x06C0CE, i_06c0ce),
    (0x06C0E2, i_06c0e2),
    (0x06C0E6, i_06c0e6),
    (0x06C0E8, i_06c0e8),
    (0x06C0EE, i_06c0ee),
    (0x06C0F2, i_06c0f2),
    (0x06C0F8, i_06c0f8),
    (0x06C0FC, i_06c0fc),
    (0x06C100, i_06c100),
    (0x06C104, i_06c104),
    (0x06C10A, i_06c10a),
    (0x06C10E, i_06c10e),
    (0x06C114, i_06c114),
    (0x06C118, i_06c118),
    (0x06C11C, i_06c11c),
    (0x06C11E, i_06c11e),
    (0x06C122, i_06c122),
    (0x06C128, i_06c128),
    (0x06C12E, i_06c12e),
    (0x06C132, i_06c132),
    (0x06C138, i_06c138),
    (0x06C13C, i_06c13c),
    (0x06C140, i_06c140),
    (0x06C144, i_06c144),
    (0x06C148, i_06c148),
    (0x06C14E, i_06c14e),
    (0x06C152, i_06c152),
    (0x06C156, i_06c156),
    (0x06C15A, i_06c15a),
    (0x06C15E, i_06c15e),
    (0x06C164, i_06c164),
    (0x06C166, i_06c166),
    (0x06C16A, i_06c16a),
    (0x06C170, i_06c170),
    (0x06C174, i_06c174),
    (0x06C17A, i_06c17a),
    (0x06C17E, i_06c17e),
    (0x06C192, i_06c192),
    (0x06C196, i_06c196),
    (0x06C198, i_06c198),
    (0x06C19E, i_06c19e),
    (0x06C1A2, i_06c1a2),
    (0x06C1A8, i_06c1a8),
    (0x06C1AC, i_06c1ac),
    (0x06C1B0, i_06c1b0),
    (0x06C1B4, i_06c1b4),
    (0x06C1B6, i_06c1b6),
    (0x06C1BA, i_06c1ba),
    (0x06C1C0, i_06c1c0),
    (0x06C1C4, i_06c1c4),
    (0x06C1C8, i_06c1c8),
    (0x06C1CC, i_06c1cc),
    (0x06C1D2, i_06c1d2),
    (0x06C1D6, i_06c1d6),
    (0x06C1DA, i_06c1da),
    (0x06C1DE, i_06c1de),
    (0x06C1E2, i_06c1e2),
    (0x06C1E8, i_06c1e8),
    (0x06C1EA, i_06c1ea),
    (0x06C1EE, i_06c1ee),
    (0x06C1F4, i_06c1f4),
    (0x06C1F8, i_06c1f8),
    (0x06C1FE, i_06c1fe),
    (0x06C202, i_06c202),
    (0x06C216, i_06c216),
    (0x06C21A, i_06c21a),
    (0x06C21C, i_06c21c),
    (0x06C222, i_06c222),
    (0x06C226, i_06c226),
    (0x06C22A, i_06c22a),
    (0x06C22E, i_06c22e),
    (0x06C232, i_06c232),
    (0x06C236, i_06c236),
    (0x06C23A, i_06c23a),
    (0x06C23C, i_06c23c),
    (0x06C242, i_06c242),
    (0x06C246, i_06c246),
    (0x06C24C, i_06c24c),
    (0x06C250, i_06c250),
    (0x06C252, i_06c252),
    (0x06C258, i_06c258),
    (0x06C25C, i_06c25c),
    (0x06C262, i_06c262),
    (0x06C268, i_06c268),
    (0x06C26E, i_06c26e),
    (0x06C272, i_06c272),
    (0x06C276, i_06c276),
    (0x06C27C, i_06c27c),
    (0x06C280, i_06c280),
    (0x06C282, i_06c282),
    (0x06C286, i_06c286),
    (0x06C28C, i_06c28c),
    (0x06C294, i_06c294),
    (0x06C29C, i_06c29c),
    (0x06C2A2, i_06c2a2),
    (0x06C2A6, i_06c2a6),
    (0x06C2AA, i_06c2aa),
    (0x06C2B0, i_06c2b0),
    (0x06C2B4, i_06c2b4),
    (0x06C2BA, i_06c2ba),
    (0x06C2C0, i_06c2c0),
    (0x06C2C4, i_06c2c4),
    (0x06C2C6, i_06c2c6),
    (0x06C2CC, i_06c2cc),
    (0x06C2D0, i_06c2d0),
    (0x06C2D4, i_06c2d4),
    (0x06C2D8, i_06c2d8),
    (0x06C2DC, i_06c2dc),
    (0x06C2DE, i_06c2de),
    (0x06C2E4, i_06c2e4),
    (0x06C2EA, i_06c2ea),
    (0x06C2EE, i_06c2ee),
    (0x06C2F4, i_06c2f4),
    (0x06C2F8, i_06c2f8),
    (0x06C2FC, i_06c2fc),
    (0x06C300, i_06c300),
    (0x06C306, i_06c306),
    (0x06C30A, i_06c30a),
    (0x06C30C, i_06c30c),
    (0x06C310, i_06c310),
    (0x06C314, i_06c314),
    (0x06C318, i_06c318),
    (0x06C31E, i_06c31e),
    (0x06C322, i_06c322),
    (0x06C324, i_06c324),
    (0x06C328, i_06c328),
    (0x06C32E, i_06c32e),
    (0x06C332, i_06c332),
    (0x06C338, i_06c338),
    (0x06C33C, i_06c33c),
    (0x06C342, i_06c342),
    (0x06C346, i_06c346),
    (0x06C348, i_06c348),
    (0x06C34C, i_06c34c),
    (0x06C352, i_06c352),
    (0x06C356, i_06c356),
    (0x06C35C, i_06c35c),
    (0x06C360, i_06c360),
    (0x06C366, i_06c366),
    (0x06C36A, i_06c36a),
    (0x06C36C, i_06c36c),
    (0x06C372, i_06c372),
    (0x06C378, i_06c378),
    (0x06C37C, i_06c37c),
    (0x06C382, i_06c382),
    (0x06C388, i_06c388),
    (0x06C390, i_06c390),
    (0x06C398, i_06c398),
    (0x06C39E, i_06c39e),
    (0x06C3A2, i_06c3a2),
    (0x06C3A6, i_06c3a6),
    (0x06C3AC, i_06c3ac),
    (0x06C3B0, i_06c3b0),
    (0x06C3B6, i_06c3b6),
    (0x06C3BC, i_06c3bc),
    (0x06C3BE, i_06c3be),
    (0x06C3C2, i_06c3c2),
    (0x06C3C8, i_06c3c8),
    (0x06C3CE, i_06c3ce),
    (0x06C3D2, i_06c3d2),
    (0x06C3D4, i_06c3d4),
    (0x06C3DA, i_06c3da),
    (0x06C3DE, i_06c3de),
    (0x06C3E2, i_06c3e2),
    (0x06C3E6, i_06c3e6),
    (0x06C3EA, i_06c3ea),
    (0x06C3EC, i_06c3ec),
    (0x06C3F2, i_06c3f2),
    (0x06C3F8, i_06c3f8),
    (0x06C3FC, i_06c3fc),
    (0x06C402, i_06c402),
    (0x06C406, i_06c406),
    (0x06C40A, i_06c40a),
    (0x06C40E, i_06c40e),
    (0x06C412, i_06c412),
    (0x06C414, i_06c414),
    (0x06C418, i_06c418),
    (0x06C41C, i_06c41c),
    (0x06C422, i_06c422),
    (0x06C426, i_06c426),
    (0x06C42C, i_06c42c),
    (0x06C432, i_06c432),
    (0x06C438, i_06c438),
    (0x06C43E, i_06c43e),
    (0x06C442, i_06c442),
    (0x06C446, i_06c446),
    (0x06C448, i_06c448),
    (0x06C44E, i_06c44e),
    (0x06C452, i_06c452),
    (0x06C456, i_06c456),
    (0x06C458, i_06c458),
    (0x06C45C, i_06c45c),
    (0x06C460, i_06c460),
    (0x06C464, i_06c464),
    (0x06C466, i_06c466),
    (0x06C468, i_06c468),
    (0x06C46E, i_06c46e),
    (0x06C472, i_06c472),
    (0x06C478, i_06c478),
    (0x06C47C, i_06c47c),
    (0x06C480, i_06c480),
    (0x06C486, i_06c486),
    (0x06C488, i_06c488),
    (0x06C48C, i_06c48c),
    (0x06C492, i_06c492),
    (0x06C496, i_06c496),
    (0x06C49C, i_06c49c),
    (0x06C4A0, i_06c4a0),
    (0x06C4A2, i_06c4a2),
    (0x06C4A6, i_06c4a6),
    (0x06C4AC, i_06c4ac),
    (0x06C4B0, i_06c4b0),
    (0x06C4B4, i_06c4b4),
    (0x06C4B6, i_06c4b6),
    (0x06C4BA, i_06c4ba),
    (0x06C4BE, i_06c4be),
    (0x06C4C2, i_06c4c2),
    (0x06C4C4, i_06c4c4),
    (0x06C4C6, i_06c4c6),
    (0x06C4CA, i_06c4ca),
    (0x06C4D0, i_06c4d0),
    (0x06C4D4, i_06c4d4),
    (0x06C4D6, i_06c4d6),
    (0x06C4DC, i_06c4dc),
    (0x06C4E0, i_06c4e0),
    (0x06C4E6, i_06c4e6),
    (0x06C4EA, i_06c4ea),
    (0x06C4EC, i_06c4ec),
    (0x06C4F2, i_06c4f2),
    (0x06C4F6, i_06c4f6),
    (0x06C4FC, i_06c4fc),
    (0x06C500, i_06c500),
    (0x06C502, i_06c502),
    (0x06C508, i_06c508),
    (0x06C50E, i_06c50e),
    (0x06C516, i_06c516),
    (0x06C51C, i_06c51c),
    (0x06C522, i_06c522),
    (0x06C526, i_06c526),
    (0x06C528, i_06c528),
    (0x06C52C, i_06c52c),
    (0x06C532, i_06c532),
    (0x06C536, i_06c536),
    (0x06C53A, i_06c53a),
    (0x06C53C, i_06c53c),
    (0x06C53E, i_06c53e),
    (0x06C544, i_06c544),
    (0x06C54A, i_06c54a),
    (0x06C54C, i_06c54c),
    (0x06C552, i_06c552),
    (0x06C554, i_06c554),
    (0x06C55A, i_06c55a),
    (0x06C560, i_06c560),
    (0x06C562, i_06c562),
    (0x06C566, i_06c566),
    (0x06C56C, i_06c56c),
    (0x06C572, i_06c572),
    (0x06C578, i_06c578),
    (0x06C57C, i_06c57c),
    (0x06C582, i_06c582),
    (0x06C586, i_06c586),
    (0x06C58A, i_06c58a),
    (0x06C58E, i_06c58e),
    (0x06C592, i_06c592),
    (0x06C598, i_06c598),
    (0x06C59A, i_06c59a),
    (0x06C59E, i_06c59e),
    (0x06C5A4, i_06c5a4),
    (0x06C5A8, i_06c5a8),
    (0x06C5AE, i_06c5ae),
    (0x06C5C2, i_06c5c2),
    (0x06C5C6, i_06c5c6),
    (0x06C5C8, i_06c5c8),
    (0x06C5CC, i_06c5cc),
    (0x06C5D0, i_06c5d0),
    (0x06C5D4, i_06c5d4),
    (0x06C5D8, i_06c5d8),
    (0x06C5DE, i_06c5de),
    (0x06C5E2, i_06c5e2),
    (0x06C5E6, i_06c5e6),
    (0x06C5EA, i_06c5ea),
    (0x06C5EE, i_06c5ee),
    (0x06C5F0, i_06c5f0),
    (0x06C5F2, i_06c5f2),
    (0x06C5F6, i_06c5f6),
    (0x06C5F8, i_06c5f8),
    (0x06C5FA, i_06c5fa),
    (0x06C5FE, i_06c5fe),
    (0x06C604, i_06c604),
    (0x06C606, i_06c606),
    (0x06C60A, i_06c60a),
    (0x06C60C, i_06c60c),
    (0x06C610, i_06c610),
    (0x06C614, i_06c614),
    (0x06C618, i_06c618),
    (0x06C61C, i_06c61c),
    (0x06C620, i_06c620),
    (0x06C624, i_06c624),
    (0x06C628, i_06c628),
    (0x06C62C, i_06c62c),
    (0x06C632, i_06c632),
    (0x06C636, i_06c636),
    (0x06C63A, i_06c63a),
    (0x06C63E, i_06c63e),
    (0x06C642, i_06c642),
    (0x06C648, i_06c648),
    (0x06C64A, i_06c64a),
    (0x06C64E, i_06c64e),
    (0x06C654, i_06c654),
    (0x06C658, i_06c658),
    (0x06C65E, i_06c65e),
    (0x06C672, i_06c672),
    (0x06C678, i_06c678),
    (0x06C67C, i_06c67c),
    (0x06C680, i_06c680),
    (0x06C686, i_06c686),
    (0x06C68A, i_06c68a),
    (0x06C68E, i_06c68e),
    (0x06C692, i_06c692),
    (0x06C696, i_06c696),
    (0x06C69C, i_06c69c),
    (0x06C69E, i_06c69e),
    (0x06C6A2, i_06c6a2),
    (0x06C6A8, i_06c6a8),
    (0x06C6AC, i_06c6ac),
    (0x06C6B2, i_06c6b2),
    (0x06C6C6, i_06c6c6),
    (0x06C6CA, i_06c6ca),
    (0x06C6D0, i_06c6d0),
    (0x06C6D6, i_06c6d6),
    (0x06C6DC, i_06c6dc),
    (0x06C6E0, i_06c6e0),
    (0x06C6E2, i_06c6e2),
    (0x06C6E6, i_06c6e6),
    (0x06C6EA, i_06c6ea),
    (0x06C6EE, i_06c6ee),
    (0x06C6F2, i_06c6f2),
    (0x06C6F8, i_06c6f8),
    (0x06C6FC, i_06c6fc),
    (0x06C700, i_06c700),
    (0x06C704, i_06c704),
    (0x06C708, i_06c708),
    (0x06C70A, i_06c70a),
    (0x06C70C, i_06c70c),
    (0x06C710, i_06c710),
    (0x06C712, i_06c712),
    (0x06C714, i_06c714),
    (0x06C718, i_06c718),
    (0x06C71E, i_06c71e),
    (0x06C720, i_06c720),
    (0x06C724, i_06c724),
    (0x06C726, i_06c726),
    (0x06C72A, i_06c72a),
    (0x06C72E, i_06c72e),
    (0x06C732, i_06c732),
    (0x06C736, i_06c736),
    (0x06C73C, i_06c73c),
    (0x06C73E, i_06c73e),
    (0x06C742, i_06c742),
    (0x06C746, i_06c746),
    (0x06C74C, i_06c74c),
    (0x06C750, i_06c750),
    (0x06C756, i_06c756),
    (0x06C75A, i_06c75a),
    (0x06C75E, i_06c75e),
    (0x06C762, i_06c762),
    (0x06C766, i_06c766),
    (0x06C76C, i_06c76c),
    (0x06C76E, i_06c76e),
    (0x06C772, i_06c772),
    (0x06C778, i_06c778),
    (0x06C77C, i_06c77c),
    (0x06C782, i_06c782),
    (0x06C796, i_06c796),
    (0x06C79A, i_06c79a),
    (0x06C79C, i_06c79c),
    (0x06C7A0, i_06c7a0),
    (0x06C7A6, i_06c7a6),
    (0x06C7AA, i_06c7aa),
    (0x06C7AE, i_06c7ae),
    (0x06C7B2, i_06c7b2),
    (0x06C7B6, i_06c7b6),
    (0x06C7BA, i_06c7ba),
    (0x06C7BE, i_06c7be),
    (0x06C7C2, i_06c7c2),
    (0x06C7C8, i_06c7c8),
    (0x06C7CC, i_06c7cc),
    (0x06C7D0, i_06c7d0),
    (0x06C7D4, i_06c7d4),
    (0x06C7D8, i_06c7d8),
    (0x06C7DE, i_06c7de),
    (0x06C7E2, i_06c7e2),
    (0x06C7E6, i_06c7e6),
    (0x06C7EA, i_06c7ea),
    (0x06C7EE, i_06c7ee),
    (0x06C7F0, i_06c7f0),
    (0x06C7F2, i_06c7f2),
    (0x06C7F6, i_06c7f6),
    (0x06C7F8, i_06c7f8),
    (0x06C7FA, i_06c7fa),
    (0x06C7FE, i_06c7fe),
    (0x06C804, i_06c804),
    (0x06C806, i_06c806),
    (0x06C80A, i_06c80a),
    (0x06C80C, i_06c80c),
    (0x06C810, i_06c810),
    (0x06C814, i_06c814),
    (0x06C818, i_06c818),
    (0x06C81C, i_06c81c),
    (0x06C820, i_06c820),
    (0x06C824, i_06c824),
    (0x06C828, i_06c828),
    (0x06C82C, i_06c82c),
    (0x06C832, i_06c832),
    (0x06C836, i_06c836),
    (0x06C83A, i_06c83a),
    (0x06C83E, i_06c83e),
    (0x06C842, i_06c842),
    (0x06C848, i_06c848),
    (0x06C84A, i_06c84a),
    (0x06C84E, i_06c84e),
    (0x06C854, i_06c854),
    (0x06C858, i_06c858),
    (0x06C85E, i_06c85e),
    (0x06C872, i_06c872),
    (0x06C878, i_06c878),
    (0x06C87C, i_06c87c),
    (0x06C880, i_06c880),
    (0x06C886, i_06c886),
    (0x06C88A, i_06c88a),
    (0x06C88E, i_06c88e),
    (0x06C892, i_06c892),
    (0x06C896, i_06c896),
    (0x06C89C, i_06c89c),
    (0x06C89E, i_06c89e),
    (0x06C8A2, i_06c8a2),
    (0x06C8A8, i_06c8a8),
    (0x06C8AC, i_06c8ac),
    (0x06C8B2, i_06c8b2),
    (0x06C8C6, i_06c8c6),
    (0x06C8CA, i_06c8ca),
    (0x06C8CE, i_06c8ce),
    (0x06C8D4, i_06c8d4),
    (0x06C8D8, i_06c8d8),
    (0x06C8DC, i_06c8dc),
    (0x06C8E0, i_06c8e0),
    (0x06C8E4, i_06c8e4),
    (0x06C8EA, i_06c8ea),
    (0x06C8EC, i_06c8ec),
    (0x06C8F0, i_06c8f0),
    (0x06C8F4, i_06c8f4),
    (0x06C8F8, i_06c8f8),
    (0x06C8FA, i_06c8fa),
    (0x06C8FE, i_06c8fe),
    (0x06C904, i_06c904),
    (0x06C908, i_06c908),
    (0x06C90E, i_06c90e),
    (0x06C912, i_06c912),
    (0x06C916, i_06c916),
    (0x06C918, i_06c918),
    (0x06C91C, i_06c91c),
    (0x06C920, i_06c920),
    (0x06C924, i_06c924),
    (0x06C928, i_06c928),
    (0x06C92E, i_06c92e),
    (0x06C932, i_06c932),
    (0x06C936, i_06c936),
    (0x06C93A, i_06c93a),
    (0x06C93E, i_06c93e),
    (0x06C940, i_06c940),
    (0x06C942, i_06c942),
    (0x06C946, i_06c946),
    (0x06C948, i_06c948),
    (0x06C94A, i_06c94a),
    (0x06C94E, i_06c94e),
    (0x06C954, i_06c954),
    (0x06C956, i_06c956),
    (0x06C95A, i_06c95a),
    (0x06C95C, i_06c95c),
    (0x06C960, i_06c960),
    (0x06C964, i_06c964),
    (0x06C96A, i_06c96a),
    (0x06C96E, i_06c96e),
    (0x06C972, i_06c972),
    (0x06C978, i_06c978),
    (0x06C97A, i_06c97a),
    (0x06C97E, i_06c97e),
    (0x06C982, i_06c982),
    (0x06C988, i_06c988),
    (0x06C98C, i_06c98c),
    (0x06C990, i_06c990),
    (0x06C994, i_06c994),
    (0x06C998, i_06c998),
    (0x06C99E, i_06c99e),
    (0x06C9A0, i_06c9a0),
    (0x06C9A4, i_06c9a4),
    (0x06C9AA, i_06c9aa),
    (0x06C9AE, i_06c9ae),
    (0x06C9B4, i_06c9b4),
    (0x06C9C8, i_06c9c8),
    (0x06C9CC, i_06c9cc),
    (0x06C9CE, i_06c9ce),
    (0x06C9D2, i_06c9d2),
    (0x06C9D8, i_06c9d8),
    (0x06C9DC, i_06c9dc),
    (0x06C9E0, i_06c9e0),
    (0x06C9E6, i_06c9e6),
    (0x06C9EA, i_06c9ea),
    (0x06C9EE, i_06c9ee),
    (0x06C9F2, i_06c9f2),
    (0x06C9F6, i_06c9f6),
    (0x06C9FA, i_06c9fa),
    (0x06C9FE, i_06c9fe),
    (0x06CA04, i_06ca04),
    (0x06CA08, i_06ca08),
    (0x06CA0C, i_06ca0c),
    (0x06CA10, i_06ca10),
    (0x06CA14, i_06ca14),
    (0x06CA1A, i_06ca1a),
    (0x06CA1E, i_06ca1e),
    (0x06CA22, i_06ca22),
    (0x06CA26, i_06ca26),
    (0x06CA2A, i_06ca2a),
    (0x06CA2C, i_06ca2c),
    (0x06CA2E, i_06ca2e),
    (0x06CA32, i_06ca32),
    (0x06CA34, i_06ca34),
    (0x06CA36, i_06ca36),
    (0x06CA3A, i_06ca3a),
    (0x06CA40, i_06ca40),
    (0x06CA42, i_06ca42),
    (0x06CA46, i_06ca46),
    (0x06CA48, i_06ca48),
    (0x06CA4C, i_06ca4c),
    (0x06CA50, i_06ca50),
    (0x06CA54, i_06ca54),
    (0x06CA58, i_06ca58),
    (0x06CA5C, i_06ca5c),
    (0x06CA60, i_06ca60),
    (0x06CA64, i_06ca64),
    (0x06CA68, i_06ca68),
    (0x06CA6E, i_06ca6e),
    (0x06CA72, i_06ca72),
    (0x06CA76, i_06ca76),
    (0x06CA7A, i_06ca7a),
    (0x06CA7E, i_06ca7e),
    (0x06CA84, i_06ca84),
    (0x06CA86, i_06ca86),
    (0x06CA8A, i_06ca8a),
    (0x06CA90, i_06ca90),
    (0x06CA94, i_06ca94),
    (0x06CA9A, i_06ca9a),
    (0x06CAAE, i_06caae),
    (0x06CAB4, i_06cab4),
    (0x06CAB8, i_06cab8),
    (0x06CABC, i_06cabc),
    (0x06CAC2, i_06cac2),
    (0x06CAC6, i_06cac6),
    (0x06CACA, i_06caca),
    (0x06CACE, i_06cace),
    (0x06CAD2, i_06cad2),
    (0x06CAD8, i_06cad8),
    (0x06CADA, i_06cada),
    (0x06CADE, i_06cade),
    (0x06CAE4, i_06cae4),
    (0x06CAE8, i_06cae8),
    (0x06CAEE, i_06caee),
    (0x06CB02, i_06cb02),
    (0x06CB06, i_06cb06),
    (0x06CB0A, i_06cb0a),
    (0x06CB0E, i_06cb0e),
    (0x06CB14, i_06cb14),
    (0x06CB18, i_06cb18),
    (0x06CB1C, i_06cb1c),
    (0x06CB20, i_06cb20),
    (0x06CB24, i_06cb24),
    (0x06CB2A, i_06cb2a),
    (0x06CB2C, i_06cb2c),
    (0x06CB30, i_06cb30),
    (0x06CB36, i_06cb36),
    (0x06CB3A, i_06cb3a),
    (0x06CB40, i_06cb40),
    (0x06CB54, i_06cb54),
    (0x06CB58, i_06cb58),
    (0x06CB5A, i_06cb5a),
    (0x06CB5E, i_06cb5e),
    (0x06CB62, i_06cb62),
    (0x06CB66, i_06cb66),
    (0x06CB6A, i_06cb6a),
    (0x06CB70, i_06cb70),
    (0x06CB74, i_06cb74),
    (0x06CB78, i_06cb78),
    (0x06CB7C, i_06cb7c),
    (0x06CB80, i_06cb80),
    (0x06CB82, i_06cb82),
    (0x06CB84, i_06cb84),
    (0x06CB88, i_06cb88),
    (0x06CB8A, i_06cb8a),
    (0x06CB8C, i_06cb8c),
    (0x06CB90, i_06cb90),
    (0x06CB96, i_06cb96),
    (0x06CB98, i_06cb98),
    (0x06CB9C, i_06cb9c),
    (0x06CB9E, i_06cb9e),
    (0x06CBA2, i_06cba2),
    (0x06CBA6, i_06cba6),
    (0x06CBAA, i_06cbaa),
    (0x06CBAE, i_06cbae),
    (0x06CBB2, i_06cbb2),
    (0x06CBB6, i_06cbb6),
    (0x06CBBA, i_06cbba),
    (0x06CBBE, i_06cbbe),
    (0x06CBC4, i_06cbc4),
    (0x06CBC8, i_06cbc8),
    (0x06CBCC, i_06cbcc),
    (0x06CBD0, i_06cbd0),
    (0x06CBD4, i_06cbd4),
    (0x06CBDA, i_06cbda),
    (0x06CBDC, i_06cbdc),
    (0x06CBE0, i_06cbe0),
    (0x06CBE6, i_06cbe6),
    (0x06CBEA, i_06cbea),
    (0x06CBF0, i_06cbf0),
    (0x06CC04, i_06cc04),
    (0x06CC0A, i_06cc0a),
    (0x06CC0E, i_06cc0e),
    (0x06CC12, i_06cc12),
    (0x06CC18, i_06cc18),
    (0x06CC1C, i_06cc1c),
    (0x06CC20, i_06cc20),
    (0x06CC24, i_06cc24),
    (0x06CC28, i_06cc28),
    (0x06CC2E, i_06cc2e),
    (0x06CC30, i_06cc30),
    (0x06CC34, i_06cc34),
    (0x06CC3A, i_06cc3a),
    (0x06CC3E, i_06cc3e),
    (0x06CC44, i_06cc44),
    (0x06CC58, i_06cc58),
    (0x06CC5C, i_06cc5c),
    (0x06CC60, i_06cc60),
    (0x06CC66, i_06cc66),
    (0x06CC6A, i_06cc6a),
    (0x06CC6E, i_06cc6e),
    (0x06CC72, i_06cc72),
    (0x06CC76, i_06cc76),
    (0x06CC7C, i_06cc7c),
    (0x06CC7E, i_06cc7e),
    (0x06CC82, i_06cc82),
    (0x06CC88, i_06cc88),
    (0x06CC8C, i_06cc8c),
    (0x06CC92, i_06cc92),
    (0x06CCA6, i_06cca6),
    (0x06CCAA, i_06ccaa),
    (0x06CCAC, i_06ccac),
    (0x06CCB0, i_06ccb0),
    (0x06CCB4, i_06ccb4),
    (0x06CCB8, i_06ccb8),
    (0x06CCBC, i_06ccbc),
    (0x06CCC2, i_06ccc2),
    (0x06CCC6, i_06ccc6),
    (0x06CCCA, i_06ccca),
    (0x06CCCE, i_06ccce),
    (0x06CCD2, i_06ccd2),
    (0x06CCD4, i_06ccd4),
    (0x06CCD6, i_06ccd6),
    (0x06CCDA, i_06ccda),
    (0x06CCDC, i_06ccdc),
    (0x06CCDE, i_06ccde),
    (0x06CCE2, i_06cce2),
    (0x06CCE8, i_06cce8),
    (0x06CCEA, i_06ccea),
    (0x06CCEE, i_06ccee),
    (0x06CCF0, i_06ccf0),
    (0x06CCF4, i_06ccf4),
    (0x06CCF8, i_06ccf8),
    (0x06CCFC, i_06ccfc),
    (0x06CD00, i_06cd00),
    (0x06CD04, i_06cd04),
    (0x06CD08, i_06cd08),
    (0x06CD0C, i_06cd0c),
    (0x06CD10, i_06cd10),
    (0x06CD16, i_06cd16),
    (0x06CD1A, i_06cd1a),
    (0x06CD1E, i_06cd1e),
    (0x06CD22, i_06cd22),
    (0x06CD26, i_06cd26),
    (0x06CD2C, i_06cd2c),
    (0x06CD2E, i_06cd2e),
    (0x06CD32, i_06cd32),
    (0x06CD38, i_06cd38),
    (0x06CD3C, i_06cd3c),
    (0x06CD42, i_06cd42),
    (0x06CD56, i_06cd56),
    (0x06CD5A, i_06cd5a),
    (0x06CD60, i_06cd60),
    (0x06CD62, i_06cd62),
    (0x06CD66, i_06cd66),
    (0x06CD6A, i_06cd6a),
    (0x06CD6E, i_06cd6e),
    (0x06CD72, i_06cd72),
    (0x06CD78, i_06cd78),
    (0x06CD7C, i_06cd7c),
    (0x06CD80, i_06cd80),
    (0x06CD84, i_06cd84),
    (0x06CD88, i_06cd88),
    (0x06CD8E, i_06cd8e),
    (0x06CD90, i_06cd90),
    (0x06CD94, i_06cd94),
    (0x06CD9A, i_06cd9a),
    (0x06CD9E, i_06cd9e),
    (0x06CDA4, i_06cda4),
    (0x06CDB8, i_06cdb8),
    (0x06CDBA, i_06cdba),
    (0x06CDBE, i_06cdbe),
    (0x06CDC4, i_06cdc4),
    (0x06CDC8, i_06cdc8),
    (0x06CDCE, i_06cdce),
    (0x06CDD4, i_06cdd4),
    (0x06CDDA, i_06cdda),
    (0x06CDDE, i_06cdde),
    (0x06CDE4, i_06cde4),
    (0x06CDEA, i_06cdea),
    (0x06CDF0, i_06cdf0),
    (0x06CDF4, i_06cdf4),
    (0x06CDF6, i_06cdf6),
    (0x06CDFC, i_06cdfc),
    (0x06CE02, i_06ce02),
    (0x06CE06, i_06ce06),
    (0x06CE0A, i_06ce0a),
    (0x06CE0C, i_06ce0c),
    (0x06CE12, i_06ce12),
    (0x06CE18, i_06ce18),
    (0x06CE1E, i_06ce1e),
    (0x06CE22, i_06ce22),
    (0x06CE26, i_06ce26),
    (0x06CE28, i_06ce28),
    (0x06CE2E, i_06ce2e),
    (0x06CE32, i_06ce32),
    (0x06CE36, i_06ce36),
    (0x06CE38, i_06ce38),
    (0x06CE3C, i_06ce3c),
    (0x06CE40, i_06ce40),
    (0x06CE44, i_06ce44),
    (0x06CE46, i_06ce46),
    (0x06CE48, i_06ce48),
    (0x06CE4C, i_06ce4c),
    (0x06CE4E, i_06ce4e),
    (0x06CE54, i_06ce54),
    (0x06CE5A, i_06ce5a),
    (0x06CE5E, i_06ce5e),
    (0x06CE62, i_06ce62),
    (0x06CE64, i_06ce64),
    (0x06CE66, i_06ce66),
    (0x06CE6A, i_06ce6a),
    (0x06CE70, i_06ce70),
    (0x06CE76, i_06ce76),
    (0x06CE7C, i_06ce7c),
    (0x06CE84, i_06ce84),
    (0x06CE8A, i_06ce8a),
    (0x06CE8E, i_06ce8e),
    (0x06CE94, i_06ce94),
    (0x06CE9A, i_06ce9a),
    (0x06CEA0, i_06cea0),
    (0x06CEA4, i_06cea4),
    (0x06CEAA, i_06ceaa),
    (0x06CEB0, i_06ceb0),
    (0x06CEB4, i_06ceb4),
    (0x06CEB8, i_06ceb8),
    (0x06CEBC, i_06cebc),
    (0x06CEC0, i_06cec0),
    (0x06CEC4, i_06cec4),
    (0x06CEC6, i_06cec6),
    (0x06CECA, i_06ceca),
    (0x06CECE, i_06cece),
    (0x06CED4, i_06ced4),
    (0x06CEDA, i_06ceda),
    (0x06CEDE, i_06cede),
    (0x06CEE2, i_06cee2),
    (0x06CEE6, i_06cee6),
    (0x06CEE8, i_06cee8),
    (0x06CEEA, i_06ceea),
    (0x06CEEE, i_06ceee),
    (0x06CEF2, i_06cef2),
    (0x06CEF6, i_06cef6),
    (0x06CEFC, i_06cefc),
    (0x06CF02, i_06cf02),
    (0x06CF06, i_06cf06),
    (0x06CF0C, i_06cf0c),
    (0x06CF12, i_06cf12),
    (0x06CF16, i_06cf16),
    (0x06CF1C, i_06cf1c),
    (0x06CF20, i_06cf20),
    (0x06CF24, i_06cf24),
    (0x06CF26, i_06cf26),
    (0x06CF2A, i_06cf2a),
    (0x06CF2E, i_06cf2e),
    (0x06CF32, i_06cf32),
    (0x06CF34, i_06cf34),
    (0x06CF38, i_06cf38),
    (0x06CF3C, i_06cf3c),
    (0x06CF40, i_06cf40),
    (0x06CF42, i_06cf42),
    (0x06CF44, i_06cf44),
    (0x06CF46, i_06cf46),
    (0x06CF48, i_06cf48),
    (0x06CF4A, i_06cf4a),
    (0x06CF4C, i_06cf4c),
    (0x06CF50, i_06cf50),
    (0x06CF56, i_06cf56),
    (0x06CF5C, i_06cf5c),
    (0x06CF62, i_06cf62),
    (0x06CF66, i_06cf66),
    (0x06CF6C, i_06cf6c),
    (0x06CF72, i_06cf72),
    (0x06CF78, i_06cf78),
    (0x06CF7E, i_06cf7e),
    (0x06CF82, i_06cf82),
    (0x06CF88, i_06cf88),
    (0x06CF8E, i_06cf8e),
    (0x06CF94, i_06cf94),
    (0x06CF98, i_06cf98),
    (0x06CF9C, i_06cf9c),
    (0x06CFA0, i_06cfa0),
    (0x06CFA4, i_06cfa4),
    (0x06CFA8, i_06cfa8),
    (0x06CFAC, i_06cfac),
    (0x06CFB0, i_06cfb0),
    (0x06CFB4, i_06cfb4),
    (0x06CFBA, i_06cfba),
    (0x06CFC0, i_06cfc0),
    (0x06CFC6, i_06cfc6),
    (0x06CFCC, i_06cfcc),
    (0x06CFD0, i_06cfd0),
    (0x06CFD2, i_06cfd2),
    (0x06CFD8, i_06cfd8),
    (0x06CFDC, i_06cfdc),
    (0x06CFE0, i_06cfe0),
    (0x06CFE2, i_06cfe2),
    (0x06CFE6, i_06cfe6),
    (0x06CFEA, i_06cfea),
    (0x06CFEC, i_06cfec),
    (0x06CFF2, i_06cff2),
    (0x06CFF6, i_06cff6),
    (0x06CFFA, i_06cffa),
    (0x06CFFC, i_06cffc),
    (0x06D000, i_06d000),
    (0x06D004, i_06d004),
    (0x06D006, i_06d006),
    (0x06D008, i_06d008),
    (0x06D00E, i_06d00e),
    (0x06D010, i_06d010),
    (0x06D014, i_06d014),
    (0x06D01A, i_06d01a),
    (0x06D020, i_06d020),
    (0x06D026, i_06d026),
    (0x06D02A, i_06d02a),
    (0x06D02E, i_06d02e),
    (0x06D030, i_06d030),
    (0x06D036, i_06d036),
    (0x06D03C, i_06d03c),
    (0x06D042, i_06d042),
    (0x06D046, i_06d046),
    (0x06D04A, i_06d04a),
    (0x06D04C, i_06d04c),
    (0x06D04E, i_06d04e),
    (0x06D052, i_06d052),
    (0x06D058, i_06d058),
    (0x06D05C, i_06d05c),
    (0x06D062, i_06d062),
    (0x06D068, i_06d068),
    (0x06D06E, i_06d06e),
    (0x06D072, i_06d072),
    (0x06D078, i_06d078),
    (0x06D07C, i_06d07c),
    (0x06D080, i_06d080),
    (0x06D082, i_06d082),
    (0x06D086, i_06d086),
    (0x06D088, i_06d088),
    (0x06D08E, i_06d08e),
    (0x06D092, i_06d092),
    (0x06D098, i_06d098),
    (0x06D09E, i_06d09e),
    (0x06D0A4, i_06d0a4),
    (0x06D0A8, i_06d0a8),
    (0x06D0AC, i_06d0ac),
    (0x06D0AE, i_06d0ae),
    (0x06D0B2, i_06d0b2),
    (0x06D0B4, i_06d0b4),
    (0x06D0B8, i_06d0b8),
    (0x06D0BC, i_06d0bc),
    (0x06D0C0, i_06d0c0),
    (0x06D0C4, i_06d0c4),
    (0x06D0C8, i_06d0c8),
    (0x06D0CC, i_06d0cc),
    (0x06D0D0, i_06d0d0),
    (0x06D0D4, i_06d0d4),
    (0x06D0D6, i_06d0d6),
    (0x06D0DA, i_06d0da),
    (0x06D0E0, i_06d0e0),
    (0x06D0E2, i_06d0e2),
    (0x06D0E4, i_06d0e4),
    (0x06D0E6, i_06d0e6),
    (0x06D0EA, i_06d0ea),
    (0x06D0EE, i_06d0ee),
    (0x06D0F0, i_06d0f0),
    (0x06D0F4, i_06d0f4),
    (0x06D0F6, i_06d0f6),
    (0x06D0FC, i_06d0fc),
    (0x06D100, i_06d100),
    (0x06D106, i_06d106),
    (0x06D10C, i_06d10c),
    (0x06D110, i_06d110),
    (0x06D116, i_06d116),
    (0x06D11C, i_06d11c),
    (0x06D120, i_06d120),
    (0x06D124, i_06d124),
    (0x06D128, i_06d128),
    (0x06D12C, i_06d12c),
    (0x06D130, i_06d130),
    (0x06D134, i_06d134),
    (0x06D136, i_06d136),
    (0x06D13A, i_06d13a),
    (0x06D13C, i_06d13c),
    (0x06D142, i_06d142),
    (0x06D146, i_06d146),
    (0x06D14A, i_06d14a),
    (0x06D150, i_06d150),
    (0x06D154, i_06d154),
    (0x06D158, i_06d158),
    (0x06D15C, i_06d15c),
    (0x06D162, i_06d162),
    (0x06D166, i_06d166),
    (0x06D16A, i_06d16a),
    (0x06D16C, i_06d16c),
    (0x06D170, i_06d170),
    (0x06D172, i_06d172),
    (0x06D178, i_06d178),
    (0x06D17C, i_06d17c),
    (0x06D180, i_06d180),
    (0x06D186, i_06d186),
    (0x06D18A, i_06d18a),
    (0x06D18E, i_06d18e),
    (0x06D194, i_06d194),
    (0x06D198, i_06d198),
    (0x06D19C, i_06d19c),
    (0x06D19E, i_06d19e),
    (0x06D1A2, i_06d1a2),
    (0x06D1A4, i_06d1a4),
    (0x06D1A8, i_06d1a8),
    (0x06D1AC, i_06d1ac),
    (0x06D1B0, i_06d1b0),
    (0x06D1B2, i_06d1b2),
    (0x06D1B6, i_06d1b6),
    (0x06D1BA, i_06d1ba),
    (0x06D1BE, i_06d1be),
    (0x06D1C2, i_06d1c2),
    (0x06D1C8, i_06d1c8),
    (0x06D1CC, i_06d1cc),
    (0x06D1D0, i_06d1d0),
    (0x06D1D2, i_06d1d2),
    (0x06D1D4, i_06d1d4),
    (0x06D1D6, i_06d1d6),
    (0x06D1DA, i_06d1da),
    (0x06D1DC, i_06d1dc),
    (0x06D1E0, i_06d1e0),
    (0x06D1E4, i_06d1e4),
    (0x06D1E8, i_06d1e8),
    (0x06D1EC, i_06d1ec),
    (0x06D1F0, i_06d1f0),
    (0x06D1F2, i_06d1f2),
    (0x06D1F6, i_06d1f6),
    (0x06D1FA, i_06d1fa),
    (0x06D1FC, i_06d1fc),
    (0x06D200, i_06d200),
    (0x06D204, i_06d204),
    (0x06D206, i_06d206),
    (0x06D20A, i_06d20a),
    (0x06D20E, i_06d20e),
    (0x06D212, i_06d212),
    (0x06D216, i_06d216),
    (0x06D218, i_06d218),
    (0x06D21C, i_06d21c),
    (0x06D220, i_06d220),
    (0x06D222, i_06d222),
    (0x06D226, i_06d226),
    (0x06D228, i_06d228),
    (0x06D22C, i_06d22c),
    (0x06D232, i_06d232),
    (0x06D236, i_06d236),
    (0x06D23C, i_06d23c),
    (0x06D240, i_06d240),
    (0x06D244, i_06d244),
    (0x06D248, i_06d248),
    (0x06D24C, i_06d24c),
    (0x06D24E, i_06d24e),
    (0x06D252, i_06d252),
    (0x06D256, i_06d256),
    (0x06D25A, i_06d25a),
    (0x06D25C, i_06d25c),
    (0x06D262, i_06d262),
    (0x06D266, i_06d266),
    (0x06D26A, i_06d26a),
    (0x06D26C, i_06d26c),
    (0x06D270, i_06d270),
    (0x06D274, i_06d274),
    (0x06D278, i_06d278),
    (0x06D27A, i_06d27a),
    (0x06D27C, i_06d27c),
    (0x06D280, i_06d280),
    (0x06D284, i_06d284),
    (0x06D286, i_06d286),
    (0x06D28A, i_06d28a),
    (0x06D28C, i_06d28c),
    (0x06D290, i_06d290),
    (0x06D294, i_06d294),
    (0x06D296, i_06d296),
    (0x06D29A, i_06d29a),
    (0x06D29C, i_06d29c),
    (0x06D2A2, i_06d2a2),
    (0x06D2A6, i_06d2a6),
    (0x06D2AC, i_06d2ac),
    (0x06D2B0, i_06d2b0),
    (0x06D2B2, i_06d2b2),
    (0x06D2B8, i_06d2b8),
    (0x06D2BC, i_06d2bc),
    (0x06D2BE, i_06d2be),
    (0x06D2C2, i_06d2c2),
    (0x06D2C6, i_06d2c6),
    (0x06D2C8, i_06d2c8),
    (0x06D2CC, i_06d2cc),
    (0x06D2D0, i_06d2d0),
    (0x06D2D4, i_06d2d4),
    (0x06D2D8, i_06d2d8),
    (0x06D2DC, i_06d2dc),
    (0x06D2E0, i_06d2e0),
    (0x06D2E4, i_06d2e4),
    (0x06D2E8, i_06d2e8),
    (0x06D2EE, i_06d2ee),
    (0x06D2F2, i_06d2f2),
    (0x06D2F8, i_06d2f8),
    (0x06D2FA, i_06d2fa),
    (0x06D2FE, i_06d2fe),
    (0x06D302, i_06d302),
    (0x06D306, i_06d306),
    (0x06D30A, i_06d30a),
    (0x06D30E, i_06d30e),
    (0x06D312, i_06d312),
    (0x06D316, i_06d316),
    (0x06D31C, i_06d31c),
    (0x06D31E, i_06d31e),
    (0x06D322, i_06d322),
    (0x06D326, i_06d326),
    (0x06D32A, i_06d32a),
    (0x06D32E, i_06d32e),
    (0x06D332, i_06d332),
    (0x06D336, i_06d336),
    (0x06D33A, i_06d33a),
    (0x06D33E, i_06d33e),
    (0x06D340, i_06d340),
    (0x06D344, i_06d344),
    (0x06D34A, i_06d34a),
    (0x06D350, i_06d350),
    (0x06D354, i_06d354),
    (0x06D35A, i_06d35a),
    (0x06D35E, i_06d35e),
    (0x06D362, i_06d362),
    (0x06D366, i_06d366),
    (0x06D36A, i_06d36a),
    (0x06D36E, i_06d36e),
    (0x06D370, i_06d370),
    (0x06D372, i_06d372),
    (0x06D376, i_06d376),
    (0x06D37A, i_06d37a),
    (0x06D37E, i_06d37e),
    (0x06D382, i_06d382),
    (0x06D386, i_06d386),
    (0x06D38A, i_06d38a),
    (0x06D38E, i_06d38e),
    (0x06D392, i_06d392),
    (0x06D396, i_06d396),
    (0x06D39A, i_06d39a),
    (0x06D39C, i_06d39c),
    (0x06D3A0, i_06d3a0),
    (0x06D3A6, i_06d3a6),
    (0x06D3AA, i_06d3aa),
    (0x06D3B0, i_06d3b0),
    (0x06D3B4, i_06d3b4),
    (0x06D3B8, i_06d3b8),
    (0x06D3BC, i_06d3bc),
    (0x06D3C0, i_06d3c0),
    (0x06D3C4, i_06d3c4),
    (0x06D3C8, i_06d3c8),
    (0x06D3CC, i_06d3cc),
    (0x06D3D0, i_06d3d0),
    (0x06D3D4, i_06d3d4),
    (0x06D3D8, i_06d3d8),
    (0x06D3DC, i_06d3dc),
    (0x06D3E0, i_06d3e0),
    (0x06D3E2, i_06d3e2),
    (0x06D3E6, i_06d3e6),
    (0x06D3E8, i_06d3e8),
    (0x06D3EE, i_06d3ee),
    (0x06D3F2, i_06d3f2),
    (0x06D3F6, i_06d3f6),
    (0x06D3FA, i_06d3fa),
    (0x06D3FE, i_06d3fe),
    (0x06D402, i_06d402),
    (0x06D406, i_06d406),
    (0x06D40A, i_06d40a),
    (0x06D40E, i_06d40e),
    (0x06D412, i_06d412),
    (0x06D416, i_06d416),
    (0x06D41A, i_06d41a),
    (0x06D420, i_06d420),
    (0x06D424, i_06d424),
    (0x06D426, i_06d426),
    (0x06D428, i_06d428),
    (0x06D42C, i_06d42c),
    (0x06D430, i_06d430),
    (0x06D434, i_06d434),
    (0x06D436, i_06d436),
    (0x06D43A, i_06d43a),
    (0x06D43C, i_06d43c),
    (0x06D440, i_06d440),
    (0x06D444, i_06d444),
    (0x06D446, i_06d446),
    (0x06D44C, i_06d44c),
    (0x06D452, i_06d452),
    (0x06D454, i_06d454),
    (0x06D45A, i_06d45a),
    (0x06D460, i_06d460),
];
