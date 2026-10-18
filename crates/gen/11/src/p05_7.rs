// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_7) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $05B934: move.l d5, (a4)
pub fn i_05b934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B936;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B936: add.l d4, d5
pub fn i_05b936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B938;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B938: move.w -(a6), d7
pub fn i_05b938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05B93A: move.l d7, (a4)
pub fn i_05b93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B93C: add.l d4, d7
pub fn i_05b93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B93E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05B93E: move.l d5, (a4)
pub fn i_05b93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B940;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B940: add.l d4, d5
pub fn i_05b940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B942;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05B942: move.w #$8401, d6
pub fn i_05b942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B946;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B946: add.w d0, d6
pub fn i_05b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B948;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B948: swap d6
pub fn i_05b948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B94A;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B94A: move.w $a(a3), d6
pub fn i_05b94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B94E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B94E: move.l #$2000000, d5
pub fn i_05b94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B954;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B954: move.l d6, (a4)
pub fn i_05b954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B956;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B956: sub.l d5, d6
pub fn i_05b956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B958;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B958: sub.l d5, d6
pub fn i_05b958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B95A;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05B95A: move.w $6(a3), d6
pub fn i_05b95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B95E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B95E: move.l d6, (a4)
pub fn i_05b95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B960;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05B960: rts
pub fn i_05b960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B962;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05B962: move.w #$40, d7
pub fn i_05b962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B966;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B966: add.w d1, d7
pub fn i_05b966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B968;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B968: move.w d7, d5
pub fn i_05b968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96A;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05B96A: addq.w #$1, d5
pub fn i_05b96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96C;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05B96C: swap d5
pub fn i_05b96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B96E;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B96E: move.w $4(a3), d5
pub fn i_05b96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B972;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B972: swap d7
pub fn i_05b972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B974;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B974: moveq #$2, d4
pub fn i_05b974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B976;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B976: swap d4
pub fn i_05b976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B978;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05B978: movea.l (a3), a6
pub fn i_05b978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B97A;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05B97A: move.w $a(a3), d6
pub fn i_05b97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B97E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05B97E: subq.b #$1, d6
pub fn i_05b97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B980;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05B980: andi.w #$1f, d6
pub fn i_05b980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B984;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05B984: add.w d6, d6
pub fn i_05b984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B986;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B986: add.w d6, d6
pub fn i_05b986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B988;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05B988: jmp $5b98c(pc, d6.w)
pub fn i_05b988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B98C;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5B98C_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05B98C: bra.w $5bb3e
pub fn i_05b98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B990;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB3E;
    10
}
/// $05B990: bra.w $5bb34
pub fn i_05b990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B994;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB34;
    10
}
/// $05B994: bra.w $5bb2a
pub fn i_05b994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B998;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB2A;
    10
}
/// $05B998: bra.w $5bb20
pub fn i_05b998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B99C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB20;
    10
}
/// $05B99C: bra.w $5bb16
pub fn i_05b99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB16;
    10
}
/// $05B9A0: bra.w $5bb0c
pub fn i_05b9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB0C;
    10
}
/// $05B9A4: bra.w $5bb02
pub fn i_05b9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BB02;
    10
}
/// $05B9A8: bra.w $5baf8
pub fn i_05b9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAF8;
    10
}
/// $05B9AC: bra.w $5baee
pub fn i_05b9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAEE;
    10
}
/// $05B9B0: bra.w $5bae4
pub fn i_05b9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAE4;
    10
}
/// $05B9B4: bra.w $5bada
pub fn i_05b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BADA;
    10
}
/// $05B9B8: bra.w $5bad0
pub fn i_05b9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAD0;
    10
}
/// $05B9BC: bra.w $5bac6
pub fn i_05b9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAC6;
    10
}
/// $05B9C0: bra.w $5babc
pub fn i_05b9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9C4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BABC;
    10
}
/// $05B9C4: bra.w $5bab2
pub fn i_05b9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAB2;
    10
}
/// $05B9C8: bra.w $5baa8
pub fn i_05b9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9CC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BAA8;
    10
}
/// $05B9CC: bra.w $5ba9e
pub fn i_05b9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA9E;
    10
}
/// $05B9D0: bra.w $5ba94
pub fn i_05b9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA94;
    10
}
/// $05B9D4: bra.w $5ba8a
pub fn i_05b9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA8A;
    10
}
/// $05B9D8: bra.w $5ba80
pub fn i_05b9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA80;
    10
}
/// $05B9DC: bra.w $5ba76
pub fn i_05b9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA76;
    10
}
/// $05B9E0: bra.w $5ba6c
pub fn i_05b9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA6C;
    10
}
/// $05B9E4: bra.w $5ba62
pub fn i_05b9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA62;
    10
}
/// $05B9E8: bra.w $5ba58
pub fn i_05b9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA58;
    10
}
/// $05B9EC: bra.w $5ba4e
pub fn i_05b9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA4E;
    10
}
/// $05B9F0: bra.w $5ba44
pub fn i_05b9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA44;
    10
}
/// $05B9F4: bra.w $5ba3a
pub fn i_05b9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA3A;
    10
}
/// $05B9F8: bra.w $5ba30
pub fn i_05b9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5B9FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA30;
    10
}
/// $05B9FC: bra.w $5ba26
pub fn i_05b9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA00;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA26;
    10
}
/// $05BA00: bra.w $5ba1c
pub fn i_05ba00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA04;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA1C;
    10
}
/// $05BA04: bra.w $5ba12
pub fn i_05ba04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA08;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BA12;
    10
}
/// $05BA08: move.w (a6)+, d7
pub fn i_05ba08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA0A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA0A: move.l d7, (a4)
pub fn i_05ba0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA0C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA0C: add.l d4, d7
pub fn i_05ba0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA0E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA0E: move.l d5, (a4)
pub fn i_05ba0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA10;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA10: add.l d4, d5
pub fn i_05ba10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA12;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA12: move.w (a6)+, d7
pub fn i_05ba12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA14;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA14: move.l d7, (a4)
pub fn i_05ba14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA16;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA16: add.l d4, d7
pub fn i_05ba16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA18;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA18: move.l d5, (a4)
pub fn i_05ba18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA1A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA1A: add.l d4, d5
pub fn i_05ba1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA1C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA1C: move.w (a6)+, d7
pub fn i_05ba1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA1E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA1E: move.l d7, (a4)
pub fn i_05ba1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA20;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA20: add.l d4, d7
pub fn i_05ba20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA22;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA22: move.l d5, (a4)
pub fn i_05ba22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA24;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA24: add.l d4, d5
pub fn i_05ba24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA26;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA26: move.w (a6)+, d7
pub fn i_05ba26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA28;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA28: move.l d7, (a4)
pub fn i_05ba28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA2A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA2A: add.l d4, d7
pub fn i_05ba2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA2C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA2C: move.l d5, (a4)
pub fn i_05ba2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA2E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA2E: add.l d4, d5
pub fn i_05ba2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA30;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA30: move.w (a6)+, d7
pub fn i_05ba30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA32;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA32: move.l d7, (a4)
pub fn i_05ba32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA34;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA34: add.l d4, d7
pub fn i_05ba34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA36;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA36: move.l d5, (a4)
pub fn i_05ba36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA38;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA38: add.l d4, d5
pub fn i_05ba38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA3A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA3A: move.w (a6)+, d7
pub fn i_05ba3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA3C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA3C: move.l d7, (a4)
pub fn i_05ba3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA3E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA3E: add.l d4, d7
pub fn i_05ba3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA40;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA40: move.l d5, (a4)
pub fn i_05ba40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA42;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA42: add.l d4, d5
pub fn i_05ba42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA44;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA44: move.w (a6)+, d7
pub fn i_05ba44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA46;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA46: move.l d7, (a4)
pub fn i_05ba46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA48;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA48: add.l d4, d7
pub fn i_05ba48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA4A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA4A: move.l d5, (a4)
pub fn i_05ba4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA4C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA4C: add.l d4, d5
pub fn i_05ba4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA4E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA4E: move.w (a6)+, d7
pub fn i_05ba4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA50;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA50: move.l d7, (a4)
pub fn i_05ba50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA52;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA52: add.l d4, d7
pub fn i_05ba52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA54;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA54: move.l d5, (a4)
pub fn i_05ba54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA56;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA56: add.l d4, d5
pub fn i_05ba56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA58;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA58: move.w (a6)+, d7
pub fn i_05ba58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA5A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA5A: move.l d7, (a4)
pub fn i_05ba5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA5C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA5C: add.l d4, d7
pub fn i_05ba5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA5E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA5E: move.l d5, (a4)
pub fn i_05ba5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA60;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA60: add.l d4, d5
pub fn i_05ba60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA62;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA62: move.w (a6)+, d7
pub fn i_05ba62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA64;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA64: move.l d7, (a4)
pub fn i_05ba64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA66;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA66: add.l d4, d7
pub fn i_05ba66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA68;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA68: move.l d5, (a4)
pub fn i_05ba68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA6A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA6A: add.l d4, d5
pub fn i_05ba6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA6C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA6C: move.w (a6)+, d7
pub fn i_05ba6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA6E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA6E: move.l d7, (a4)
pub fn i_05ba6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA70;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA70: add.l d4, d7
pub fn i_05ba70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA72;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA72: move.l d5, (a4)
pub fn i_05ba72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA74;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA74: add.l d4, d5
pub fn i_05ba74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA76;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA76: move.w (a6)+, d7
pub fn i_05ba76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA78;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA78: move.l d7, (a4)
pub fn i_05ba78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA7A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA7A: add.l d4, d7
pub fn i_05ba7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA7C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA7C: move.l d5, (a4)
pub fn i_05ba7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA7E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA7E: add.l d4, d5
pub fn i_05ba7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA80;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA80: move.w (a6)+, d7
pub fn i_05ba80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA82;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA82: move.l d7, (a4)
pub fn i_05ba82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA84;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA84: add.l d4, d7
pub fn i_05ba84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA86;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA86: move.l d5, (a4)
pub fn i_05ba86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA88;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA88: add.l d4, d5
pub fn i_05ba88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA8A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA8A: move.w (a6)+, d7
pub fn i_05ba8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA8C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA8C: move.l d7, (a4)
pub fn i_05ba8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA8E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA8E: add.l d4, d7
pub fn i_05ba8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA90;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA90: move.l d5, (a4)
pub fn i_05ba90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA92;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA92: add.l d4, d5
pub fn i_05ba92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA94;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA94: move.w (a6)+, d7
pub fn i_05ba94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA96;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BA96: move.l d7, (a4)
pub fn i_05ba96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA98;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA98: add.l d4, d7
pub fn i_05ba98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA9A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BA9A: move.l d5, (a4)
pub fn i_05ba9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA9C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BA9C: add.l d4, d5
pub fn i_05ba9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BA9E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BA9E: move.w (a6)+, d7
pub fn i_05ba9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAA0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAA0: move.l d7, (a4)
pub fn i_05baa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAA2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAA2: add.l d4, d7
pub fn i_05baa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAA4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAA4: move.l d5, (a4)
pub fn i_05baa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAA6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAA6: add.l d4, d5
pub fn i_05baa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAA8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAA8: move.w (a6)+, d7
pub fn i_05baa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAAA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAAA: move.l d7, (a4)
pub fn i_05baaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAAC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAAC: add.l d4, d7
pub fn i_05baac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAAE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAAE: move.l d5, (a4)
pub fn i_05baae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAB0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAB0: add.l d4, d5
pub fn i_05bab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAB2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAB2: move.w (a6)+, d7
pub fn i_05bab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAB4;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAB4: move.l d7, (a4)
pub fn i_05bab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAB6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAB6: add.l d4, d7
pub fn i_05bab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAB8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAB8: move.l d5, (a4)
pub fn i_05bab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BABA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BABA: add.l d4, d5
pub fn i_05baba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BABC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BABC: move.w (a6)+, d7
pub fn i_05babc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BABE;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BABE: move.l d7, (a4)
pub fn i_05babe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAC0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAC0: add.l d4, d7
pub fn i_05bac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAC2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAC2: move.l d5, (a4)
pub fn i_05bac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAC4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAC4: add.l d4, d5
pub fn i_05bac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAC6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAC6: move.w (a6)+, d7
pub fn i_05bac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAC8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAC8: move.l d7, (a4)
pub fn i_05bac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BACA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BACA: add.l d4, d7
pub fn i_05baca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BACC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BACC: move.l d5, (a4)
pub fn i_05bacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BACE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BACE: add.l d4, d5
pub fn i_05bace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAD0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAD0: move.w (a6)+, d7
pub fn i_05bad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAD2;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAD2: move.l d7, (a4)
pub fn i_05bad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAD4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAD4: add.l d4, d7
pub fn i_05bad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAD6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAD6: move.l d5, (a4)
pub fn i_05bad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAD8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAD8: add.l d4, d5
pub fn i_05bad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BADA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BADA: move.w (a6)+, d7
pub fn i_05bada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BADC;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BADC: move.l d7, (a4)
pub fn i_05badc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BADE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BADE: add.l d4, d7
pub fn i_05bade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAE0: move.l d5, (a4)
pub fn i_05bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAE2: add.l d4, d5
pub fn i_05bae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAE4: move.w (a6)+, d7
pub fn i_05bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE6;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAE6: move.l d7, (a4)
pub fn i_05bae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAE8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAE8: add.l d4, d7
pub fn i_05bae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAEA: move.l d5, (a4)
pub fn i_05baea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAEC: add.l d4, d5
pub fn i_05baec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAEE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAEE: move.w (a6)+, d7
pub fn i_05baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAF0: move.l d7, (a4)
pub fn i_05baf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAF2: add.l d4, d7
pub fn i_05baf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAF4: move.l d5, (a4)
pub fn i_05baf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAF6: add.l d4, d5
pub fn i_05baf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAF8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BAF8: move.w (a6)+, d7
pub fn i_05baf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BAFA: move.l d7, (a4)
pub fn i_05bafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BAFC: add.l d4, d7
pub fn i_05bafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BAFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BAFE: move.l d5, (a4)
pub fn i_05bafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB00;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB00: add.l d4, d5
pub fn i_05bb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB02;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB02: move.w (a6)+, d7
pub fn i_05bb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB04;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB04: move.l d7, (a4)
pub fn i_05bb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB06: add.l d4, d7
pub fn i_05bb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB08: move.l d5, (a4)
pub fn i_05bb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB0A: add.l d4, d5
pub fn i_05bb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB0C: move.w (a6)+, d7
pub fn i_05bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB0E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB0E: move.l d7, (a4)
pub fn i_05bb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB10: add.l d4, d7
pub fn i_05bb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB12: move.l d5, (a4)
pub fn i_05bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB14;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB14: add.l d4, d5
pub fn i_05bb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB16;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB16: move.w (a6)+, d7
pub fn i_05bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB18;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB18: move.l d7, (a4)
pub fn i_05bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB1A: add.l d4, d7
pub fn i_05bb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB1C: move.l d5, (a4)
pub fn i_05bb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB1E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB1E: add.l d4, d5
pub fn i_05bb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB20;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB20: move.w (a6)+, d7
pub fn i_05bb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB22;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB22: move.l d7, (a4)
pub fn i_05bb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB24: add.l d4, d7
pub fn i_05bb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB26: move.l d5, (a4)
pub fn i_05bb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB28;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB28: add.l d4, d5
pub fn i_05bb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB2A: move.w (a6)+, d7
pub fn i_05bb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB2C: move.l d7, (a4)
pub fn i_05bb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB2E: add.l d4, d7
pub fn i_05bb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB30: move.l d5, (a4)
pub fn i_05bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB32;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB32: add.l d4, d5
pub fn i_05bb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB34;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB34: move.w (a6)+, d7
pub fn i_05bb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB36;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB36: move.l d7, (a4)
pub fn i_05bb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB38: add.l d4, d7
pub fn i_05bb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB3A: move.l d5, (a4)
pub fn i_05bb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB3C: add.l d4, d5
pub fn i_05bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB3E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB3E: move.w (a6)+, d7
pub fn i_05bb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB40;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05BB40: move.l d7, (a4)
pub fn i_05bb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB42: add.l d4, d7
pub fn i_05bb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB44: move.l d5, (a4)
pub fn i_05bb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB46;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB46: add.l d4, d5
pub fn i_05bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB48;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BB48: clr.w d7
pub fn i_05bb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4A;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BB4A: moveq #$1, d4
pub fn i_05bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4C;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BB4C: swap d4
pub fn i_05bb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB4E;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BB4E: add.w d6, d6
pub fn i_05bb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB50;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BB50: jmp $5bb54(pc, d6.w)
pub fn i_05bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB54;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BB54_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BB54: move.l d7, (a4)
pub fn i_05bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB56;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB56: add.l d4, d7
pub fn i_05bb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB58;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB58: move.l d7, (a4)
pub fn i_05bb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB5A: add.l d4, d7
pub fn i_05bb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB5C: move.l d7, (a4)
pub fn i_05bb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB5E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB5E: add.l d4, d7
pub fn i_05bb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB60;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB60: move.l d7, (a4)
pub fn i_05bb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB62;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB62: add.l d4, d7
pub fn i_05bb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB64;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB64: move.l d7, (a4)
pub fn i_05bb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB66;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB66: add.l d4, d7
pub fn i_05bb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB68;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB68: move.l d7, (a4)
pub fn i_05bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB6A: add.l d4, d7
pub fn i_05bb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB6C: move.l d7, (a4)
pub fn i_05bb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB6E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB6E: add.l d4, d7
pub fn i_05bb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB70;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB70: move.l d7, (a4)
pub fn i_05bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB72;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB72: add.l d4, d7
pub fn i_05bb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB74;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB74: move.l d7, (a4)
pub fn i_05bb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB76;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB76: add.l d4, d7
pub fn i_05bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB78;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB78: move.l d7, (a4)
pub fn i_05bb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB7A: add.l d4, d7
pub fn i_05bb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB7C: move.l d7, (a4)
pub fn i_05bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB7E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB7E: add.l d4, d7
pub fn i_05bb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB80;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB80: move.l d7, (a4)
pub fn i_05bb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB82;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB82: add.l d4, d7
pub fn i_05bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB84;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB84: move.l d7, (a4)
pub fn i_05bb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB86;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB86: add.l d4, d7
pub fn i_05bb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB88;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB88: move.l d7, (a4)
pub fn i_05bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB8A: add.l d4, d7
pub fn i_05bb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB8C: move.l d7, (a4)
pub fn i_05bb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB8E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB8E: add.l d4, d7
pub fn i_05bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB90;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB90: move.l d7, (a4)
pub fn i_05bb90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB92;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB92: add.l d4, d7
pub fn i_05bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB94;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB94: move.l d7, (a4)
pub fn i_05bb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB96;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB96: add.l d4, d7
pub fn i_05bb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB98;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB98: move.l d7, (a4)
pub fn i_05bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB9A: add.l d4, d7
pub fn i_05bb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BB9C: move.l d7, (a4)
pub fn i_05bb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BB9E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BB9E: add.l d4, d7
pub fn i_05bb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA0: move.l d7, (a4)
pub fn i_05bba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBA2: add.l d4, d7
pub fn i_05bba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA4: move.l d7, (a4)
pub fn i_05bba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBA6: add.l d4, d7
pub fn i_05bba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBA8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBA8: move.l d7, (a4)
pub fn i_05bba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBAA: add.l d4, d7
pub fn i_05bbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBAC: move.l d7, (a4)
pub fn i_05bbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBAE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBAE: add.l d4, d7
pub fn i_05bbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB0: move.l d7, (a4)
pub fn i_05bbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBB2: add.l d4, d7
pub fn i_05bbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB4: move.l d7, (a4)
pub fn i_05bbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBB6: add.l d4, d7
pub fn i_05bbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBB8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBB8: move.l d7, (a4)
pub fn i_05bbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBBA: add.l d4, d7
pub fn i_05bbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBBC: move.l d7, (a4)
pub fn i_05bbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBBE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBBE: add.l d4, d7
pub fn i_05bbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC0: move.l d7, (a4)
pub fn i_05bbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBC2: add.l d4, d7
pub fn i_05bbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC4: move.l d7, (a4)
pub fn i_05bbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBC6: add.l d4, d7
pub fn i_05bbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBC8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBC8: move.l d7, (a4)
pub fn i_05bbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBCA: add.l d4, d7
pub fn i_05bbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBCC: move.l d7, (a4)
pub fn i_05bbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBCE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBCE: add.l d4, d7
pub fn i_05bbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD0: move.l d7, (a4)
pub fn i_05bbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBD2: add.l d4, d7
pub fn i_05bbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD4: move.l d7, (a4)
pub fn i_05bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBD6: add.l d4, d7
pub fn i_05bbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBD8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBD8: move.l d7, (a4)
pub fn i_05bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBDA: add.l d4, d7
pub fn i_05bbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBDC: move.l d7, (a4)
pub fn i_05bbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBDE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBDE: add.l d4, d7
pub fn i_05bbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE0: move.l d7, (a4)
pub fn i_05bbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBE2: add.l d4, d7
pub fn i_05bbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE4: move.l d7, (a4)
pub fn i_05bbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBE6: add.l d4, d7
pub fn i_05bbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBE8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBE8: move.l d7, (a4)
pub fn i_05bbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBEA: add.l d4, d7
pub fn i_05bbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBEC: move.l d7, (a4)
pub fn i_05bbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBEE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBEE: add.l d4, d7
pub fn i_05bbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF0: move.l d7, (a4)
pub fn i_05bbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBF2: add.l d4, d7
pub fn i_05bbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF4: move.l d7, (a4)
pub fn i_05bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBF6: add.l d4, d7
pub fn i_05bbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBF8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBF8: move.l d7, (a4)
pub fn i_05bbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBFA: add.l d4, d7
pub fn i_05bbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BBFC: move.l d7, (a4)
pub fn i_05bbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BBFE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BBFE: add.l d4, d7
pub fn i_05bbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC00;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC00: move.l d7, (a4)
pub fn i_05bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC02;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC02: add.l d4, d7
pub fn i_05bc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC04;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC04: move.l d7, (a4)
pub fn i_05bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC06: add.l d4, d7
pub fn i_05bc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC08: move.l d7, (a4)
pub fn i_05bc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC0A: add.l d4, d7
pub fn i_05bc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC0C: move.l d7, (a4)
pub fn i_05bc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC0E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC0E: add.l d4, d7
pub fn i_05bc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC10;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC10: move.l d7, (a4)
pub fn i_05bc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC12;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC12: add.l d4, d7
pub fn i_05bc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC14;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC14: move.l d7, (a4)
pub fn i_05bc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC16;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC16: add.l d4, d7
pub fn i_05bc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC18;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC18: move.l d7, (a4)
pub fn i_05bc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC1A: add.l d4, d7
pub fn i_05bc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC1C: move.l d7, (a4)
pub fn i_05bc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC1E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC1E: add.l d4, d7
pub fn i_05bc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC20;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC20: move.l d7, (a4)
pub fn i_05bc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC22;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC22: add.l d4, d7
pub fn i_05bc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC24;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC24: move.l d7, (a4)
pub fn i_05bc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC26;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC26: add.l d4, d7
pub fn i_05bc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC28;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC28: move.l d7, (a4)
pub fn i_05bc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC2A: add.l d4, d7
pub fn i_05bc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC2C: move.l d7, (a4)
pub fn i_05bc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC2E: add.l d4, d7
pub fn i_05bc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC30: move.l d7, (a4)
pub fn i_05bc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC32;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC32: add.l d4, d7
pub fn i_05bc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC34;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC34: move.l d7, (a4)
pub fn i_05bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC36;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC36: add.l d4, d7
pub fn i_05bc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC38;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC38: move.l d7, (a4)
pub fn i_05bc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC3A: add.l d4, d7
pub fn i_05bc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC3C: move.l d7, (a4)
pub fn i_05bc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC3E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC3E: add.l d4, d7
pub fn i_05bc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC40;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC40: move.l d7, (a4)
pub fn i_05bc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC42: add.l d4, d7
pub fn i_05bc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC44: move.l d7, (a4)
pub fn i_05bc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC46;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC46: add.l d4, d7
pub fn i_05bc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC48;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC48: move.l d7, (a4)
pub fn i_05bc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC4A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC4A: add.l d4, d7
pub fn i_05bc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC4C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BC4C: move.w #$8401, d6
pub fn i_05bc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC50;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC50: add.w d0, d6
pub fn i_05bc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC52;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC52: swap d6
pub fn i_05bc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC54;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC54: move.w $a(a3), d6
pub fn i_05bc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC58;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC58: move.l #$2000000, d5
pub fn i_05bc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC5E;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC5E: move.l d6, (a4)
pub fn i_05bc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC60;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC60: sub.l d5, d6
pub fn i_05bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC62;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BC62: sub.l d5, d6
pub fn i_05bc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC64;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BC64: move.w $6(a3), d6
pub fn i_05bc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC68;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC68: move.l d6, (a4)
pub fn i_05bc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC6A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BC6A: rts
pub fn i_05bc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05BC6C: move.w #$40, d7
pub fn i_05bc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC70;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC70: add.w d1, d7
pub fn i_05bc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC72;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC72: move.w d7, d5
pub fn i_05bc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC74;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BC74: addq.w #$1, d5
pub fn i_05bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC76;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BC76: swap d5
pub fn i_05bc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC78;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC78: move.w $4(a3), d5
pub fn i_05bc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC7C;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC7C: swap d7
pub fn i_05bc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC7E;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC7E: moveq #$2, d4
pub fn i_05bc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC80;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC80: swap d4
pub fn i_05bc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC82;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BC82: movea.l (a3), a6
pub fn i_05bc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC84;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05BC84: move.w $a(a3), d6
pub fn i_05bc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC88;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BC88: subq.b #$1, d6
pub fn i_05bc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC8A;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05BC8A: andi.w #$1f, d6
pub fn i_05bc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC8E;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05BC8E: add.w d6, d6
pub fn i_05bc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC90;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC90: add.w d6, d6
pub fn i_05bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC92;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BC92: jmp $5bc96(pc, d6.w)
pub fn i_05bc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC96;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BC96_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BC96: bra.w $5be48
pub fn i_05bc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE48;
    10
}
/// $05BC9A: bra.w $5be3e
pub fn i_05bc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BC9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE3E;
    10
}
/// $05BC9E: bra.w $5be34
pub fn i_05bc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE34;
    10
}
/// $05BCA2: bra.w $5be2a
pub fn i_05bca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCA6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE2A;
    10
}
/// $05BCA6: bra.w $5be20
pub fn i_05bca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE20;
    10
}
/// $05BCAA: bra.w $5be16
pub fn i_05bcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE16;
    10
}
/// $05BCAE: bra.w $5be0c
pub fn i_05bcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE0C;
    10
}
/// $05BCB2: bra.w $5be02
pub fn i_05bcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BE02;
    10
}
/// $05BCB6: bra.w $5bdf8
pub fn i_05bcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCBA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDF8;
    10
}
/// $05BCBA: bra.w $5bdee
pub fn i_05bcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDEE;
    10
}
/// $05BCBE: bra.w $5bde4
pub fn i_05bcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDE4;
    10
}
/// $05BCC2: bra.w $5bdda
pub fn i_05bcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDDA;
    10
}
/// $05BCC6: bra.w $5bdd0
pub fn i_05bcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCCA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDD0;
    10
}
/// $05BCCA: bra.w $5bdc6
pub fn i_05bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCCE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDC6;
    10
}
/// $05BCCE: bra.w $5bdbc
pub fn i_05bcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDBC;
    10
}
/// $05BCD2: bra.w $5bdb2
pub fn i_05bcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCD6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDB2;
    10
}
/// $05BCD6: bra.w $5bda8
pub fn i_05bcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCDA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BDA8;
    10
}
/// $05BCDA: bra.w $5bd9e
pub fn i_05bcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD9E;
    10
}
/// $05BCDE: bra.w $5bd94
pub fn i_05bcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD94;
    10
}
/// $05BCE2: bra.w $5bd8a
pub fn i_05bce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCE6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD8A;
    10
}
/// $05BCE6: bra.w $5bd80
pub fn i_05bce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD80;
    10
}
/// $05BCEA: bra.w $5bd76
pub fn i_05bcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD76;
    10
}
/// $05BCEE: bra.w $5bd6c
pub fn i_05bcee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD6C;
    10
}
/// $05BCF2: bra.w $5bd62
pub fn i_05bcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD62;
    10
}
/// $05BCF6: bra.w $5bd58
pub fn i_05bcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD58;
    10
}
/// $05BCFA: bra.w $5bd4e
pub fn i_05bcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BCFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD4E;
    10
}
/// $05BCFE: bra.w $5bd44
pub fn i_05bcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD02;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD44;
    10
}
/// $05BD02: bra.w $5bd3a
pub fn i_05bd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD06;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD3A;
    10
}
/// $05BD06: bra.w $5bd30
pub fn i_05bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD0A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD30;
    10
}
/// $05BD0A: bra.w $5bd26
pub fn i_05bd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD26;
    10
}
/// $05BD0E: bra.w $5bd1c
pub fn i_05bd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD12;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BD1C;
    10
}
/// $05BD12: move.w -(a6), d7
pub fn i_05bd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD14;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD14: move.l d7, (a4)
pub fn i_05bd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD16;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD16: add.l d4, d7
pub fn i_05bd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD18;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD18: move.l d5, (a4)
pub fn i_05bd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD1A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD1A: add.l d4, d5
pub fn i_05bd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD1C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD1C: move.w -(a6), d7
pub fn i_05bd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD1E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD1E: move.l d7, (a4)
pub fn i_05bd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD20;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD20: add.l d4, d7
pub fn i_05bd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD22;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD22: move.l d5, (a4)
pub fn i_05bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD24;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD24: add.l d4, d5
pub fn i_05bd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD26;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD26: move.w -(a6), d7
pub fn i_05bd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD28;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD28: move.l d7, (a4)
pub fn i_05bd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD2A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD2A: add.l d4, d7
pub fn i_05bd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD2C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD2C: move.l d5, (a4)
pub fn i_05bd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD2E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD2E: add.l d4, d5
pub fn i_05bd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD30;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD30: move.w -(a6), d7
pub fn i_05bd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD32;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD32: move.l d7, (a4)
pub fn i_05bd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD34;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD34: add.l d4, d7
pub fn i_05bd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD36;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD36: move.l d5, (a4)
pub fn i_05bd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD38;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD38: add.l d4, d5
pub fn i_05bd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD3A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD3A: move.w -(a6), d7
pub fn i_05bd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD3C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD3C: move.l d7, (a4)
pub fn i_05bd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD3E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD3E: add.l d4, d7
pub fn i_05bd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD40;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD40: move.l d5, (a4)
pub fn i_05bd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD42;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD42: add.l d4, d5
pub fn i_05bd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD44;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD44: move.w -(a6), d7
pub fn i_05bd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD46;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD46: move.l d7, (a4)
pub fn i_05bd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD48;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD48: add.l d4, d7
pub fn i_05bd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD4A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD4A: move.l d5, (a4)
pub fn i_05bd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD4C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD4C: add.l d4, d5
pub fn i_05bd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD4E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD4E: move.w -(a6), d7
pub fn i_05bd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD50;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD50: move.l d7, (a4)
pub fn i_05bd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD52;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD52: add.l d4, d7
pub fn i_05bd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD54;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD54: move.l d5, (a4)
pub fn i_05bd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD56;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD56: add.l d4, d5
pub fn i_05bd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD58;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD58: move.w -(a6), d7
pub fn i_05bd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD5A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD5A: move.l d7, (a4)
pub fn i_05bd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD5C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD5C: add.l d4, d7
pub fn i_05bd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD5E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD5E: move.l d5, (a4)
pub fn i_05bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD60;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD60: add.l d4, d5
pub fn i_05bd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD62;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD62: move.w -(a6), d7
pub fn i_05bd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD64;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD64: move.l d7, (a4)
pub fn i_05bd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD66;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD66: add.l d4, d7
pub fn i_05bd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD68;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD68: move.l d5, (a4)
pub fn i_05bd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD6A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD6A: add.l d4, d5
pub fn i_05bd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD6C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD6C: move.w -(a6), d7
pub fn i_05bd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD6E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD6E: move.l d7, (a4)
pub fn i_05bd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD70;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD70: add.l d4, d7
pub fn i_05bd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD72;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD72: move.l d5, (a4)
pub fn i_05bd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD74;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD74: add.l d4, d5
pub fn i_05bd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD76;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD76: move.w -(a6), d7
pub fn i_05bd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD78;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD78: move.l d7, (a4)
pub fn i_05bd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD7A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD7A: add.l d4, d7
pub fn i_05bd7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD7C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD7C: move.l d5, (a4)
pub fn i_05bd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD7E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD7E: add.l d4, d5
pub fn i_05bd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD80;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD80: move.w -(a6), d7
pub fn i_05bd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD82;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD82: move.l d7, (a4)
pub fn i_05bd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD84;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD84: add.l d4, d7
pub fn i_05bd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD86;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD86: move.l d5, (a4)
pub fn i_05bd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD88;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD88: add.l d4, d5
pub fn i_05bd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD8A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD8A: move.w -(a6), d7
pub fn i_05bd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD8C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD8C: move.l d7, (a4)
pub fn i_05bd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD8E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD8E: add.l d4, d7
pub fn i_05bd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD90;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD90: move.l d5, (a4)
pub fn i_05bd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD92;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD92: add.l d4, d5
pub fn i_05bd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD94;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD94: move.w -(a6), d7
pub fn i_05bd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD96;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BD96: move.l d7, (a4)
pub fn i_05bd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD98;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD98: add.l d4, d7
pub fn i_05bd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD9A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BD9A: move.l d5, (a4)
pub fn i_05bd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD9C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BD9C: add.l d4, d5
pub fn i_05bd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BD9E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BD9E: move.w -(a6), d7
pub fn i_05bd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDA0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDA0: move.l d7, (a4)
pub fn i_05bda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDA2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDA2: add.l d4, d7
pub fn i_05bda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDA4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDA4: move.l d5, (a4)
pub fn i_05bda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDA6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDA6: add.l d4, d5
pub fn i_05bda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDA8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDA8: move.w -(a6), d7
pub fn i_05bda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDAA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDAA: move.l d7, (a4)
pub fn i_05bdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDAC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDAC: add.l d4, d7
pub fn i_05bdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDAE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDAE: move.l d5, (a4)
pub fn i_05bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDB0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDB0: add.l d4, d5
pub fn i_05bdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDB2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDB2: move.w -(a6), d7
pub fn i_05bdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDB4;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDB4: move.l d7, (a4)
pub fn i_05bdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDB6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDB6: add.l d4, d7
pub fn i_05bdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDB8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDB8: move.l d5, (a4)
pub fn i_05bdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDBA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDBA: add.l d4, d5
pub fn i_05bdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDBC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDBC: move.w -(a6), d7
pub fn i_05bdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDBE;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDBE: move.l d7, (a4)
pub fn i_05bdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDC0: add.l d4, d7
pub fn i_05bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDC2: move.l d5, (a4)
pub fn i_05bdc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDC4: add.l d4, d5
pub fn i_05bdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDC6: move.w -(a6), d7
pub fn i_05bdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDC8;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDC8: move.l d7, (a4)
pub fn i_05bdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDCA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDCA: add.l d4, d7
pub fn i_05bdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDCC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDCC: move.l d5, (a4)
pub fn i_05bdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDCE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDCE: add.l d4, d5
pub fn i_05bdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDD0: move.w -(a6), d7
pub fn i_05bdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD2;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDD2: move.l d7, (a4)
pub fn i_05bdd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDD4: add.l d4, d7
pub fn i_05bdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDD6: move.l d5, (a4)
pub fn i_05bdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDD8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDD8: add.l d4, d5
pub fn i_05bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDDA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDDA: move.w -(a6), d7
pub fn i_05bdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDDC;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDDC: move.l d7, (a4)
pub fn i_05bddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDDE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDDE: add.l d4, d7
pub fn i_05bdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDE0: move.l d5, (a4)
pub fn i_05bde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDE2: add.l d4, d5
pub fn i_05bde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDE4: move.w -(a6), d7
pub fn i_05bde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE6;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDE6: move.l d7, (a4)
pub fn i_05bde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDE8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDE8: add.l d4, d7
pub fn i_05bde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDEA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDEA: move.l d5, (a4)
pub fn i_05bdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDEC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDEC: add.l d4, d5
pub fn i_05bdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDEE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDEE: move.w -(a6), d7
pub fn i_05bdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDF0: move.l d7, (a4)
pub fn i_05bdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDF2: add.l d4, d7
pub fn i_05bdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDF4: move.l d5, (a4)
pub fn i_05bdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDF6: add.l d4, d5
pub fn i_05bdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDF8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BDF8: move.w -(a6), d7
pub fn i_05bdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BDFA: move.l d7, (a4)
pub fn i_05bdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BDFC: add.l d4, d7
pub fn i_05bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BDFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BDFE: move.l d5, (a4)
pub fn i_05bdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE00;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE00: add.l d4, d5
pub fn i_05be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE02;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE02: move.w -(a6), d7
pub fn i_05be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE04;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE04: move.l d7, (a4)
pub fn i_05be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE06;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE06: add.l d4, d7
pub fn i_05be06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE08;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE08: move.l d5, (a4)
pub fn i_05be08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE0A: add.l d4, d5
pub fn i_05be0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE0C: move.w -(a6), d7
pub fn i_05be0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE0E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE0E: move.l d7, (a4)
pub fn i_05be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE10: add.l d4, d7
pub fn i_05be10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE12: move.l d5, (a4)
pub fn i_05be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE14;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE14: add.l d4, d5
pub fn i_05be14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE16;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE16: move.w -(a6), d7
pub fn i_05be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE18;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE18: move.l d7, (a4)
pub fn i_05be18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE1A: add.l d4, d7
pub fn i_05be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE1C: move.l d5, (a4)
pub fn i_05be1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE1E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE1E: add.l d4, d5
pub fn i_05be1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE20;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE20: move.w -(a6), d7
pub fn i_05be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE22;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE22: move.l d7, (a4)
pub fn i_05be22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE24: add.l d4, d7
pub fn i_05be24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE26: move.l d5, (a4)
pub fn i_05be26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE28;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE28: add.l d4, d5
pub fn i_05be28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE2A: move.w -(a6), d7
pub fn i_05be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE2C: move.l d7, (a4)
pub fn i_05be2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE2E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE2E: add.l d4, d7
pub fn i_05be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE30;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE30: move.l d5, (a4)
pub fn i_05be30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE32;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE32: add.l d4, d5
pub fn i_05be32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE34;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE34: move.w -(a6), d7
pub fn i_05be34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE36;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE36: move.l d7, (a4)
pub fn i_05be36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE38: add.l d4, d7
pub fn i_05be38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE3A: move.l d5, (a4)
pub fn i_05be3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE3C: add.l d4, d5
pub fn i_05be3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE3E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE3E: move.w -(a6), d7
pub fn i_05be3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE40;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE40: move.l d7, (a4)
pub fn i_05be40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE42;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE42: add.l d4, d7
pub fn i_05be42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE44;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE44: move.l d5, (a4)
pub fn i_05be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE46;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE46: add.l d4, d5
pub fn i_05be46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE48;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE48: move.w -(a6), d7
pub fn i_05be48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05BE4A: move.l d7, (a4)
pub fn i_05be4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE4C: add.l d4, d7
pub fn i_05be4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE4E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE4E: move.l d5, (a4)
pub fn i_05be4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE50;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE50: add.l d4, d5
pub fn i_05be50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE52;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05BE52: clr.w d7
pub fn i_05be52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE54;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BE54: moveq #$1, d4
pub fn i_05be54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE56;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BE56: swap d4
pub fn i_05be56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE58;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BE58: add.w d6, d6
pub fn i_05be58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE5A;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BE5A: jmp $5be5e(pc, d6.w)
pub fn i_05be5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE5E;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BE5E_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BE5E: move.l d7, (a4)
pub fn i_05be5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE60;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE60: add.l d4, d7
pub fn i_05be60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE62;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE62: move.l d7, (a4)
pub fn i_05be62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE64;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE64: add.l d4, d7
pub fn i_05be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE66;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE66: move.l d7, (a4)
pub fn i_05be66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE68;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE68: add.l d4, d7
pub fn i_05be68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE6A: move.l d7, (a4)
pub fn i_05be6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE6C: add.l d4, d7
pub fn i_05be6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE6E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE6E: move.l d7, (a4)
pub fn i_05be6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE70;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE70: add.l d4, d7
pub fn i_05be70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE72;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE72: move.l d7, (a4)
pub fn i_05be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE74;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE74: add.l d4, d7
pub fn i_05be74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE76;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE76: move.l d7, (a4)
pub fn i_05be76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE78;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE78: add.l d4, d7
pub fn i_05be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE7A: move.l d7, (a4)
pub fn i_05be7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE7C: add.l d4, d7
pub fn i_05be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE7E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE7E: move.l d7, (a4)
pub fn i_05be7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE80;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE80: add.l d4, d7
pub fn i_05be80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE82;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE82: move.l d7, (a4)
pub fn i_05be82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE84;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE84: add.l d4, d7
pub fn i_05be84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE86;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE86: move.l d7, (a4)
pub fn i_05be86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE88;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE88: add.l d4, d7
pub fn i_05be88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE8A: move.l d7, (a4)
pub fn i_05be8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE8C: add.l d4, d7
pub fn i_05be8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE8E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE8E: move.l d7, (a4)
pub fn i_05be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE90;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE90: add.l d4, d7
pub fn i_05be90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE92;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE92: move.l d7, (a4)
pub fn i_05be92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE94;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE94: add.l d4, d7
pub fn i_05be94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE96;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE96: move.l d7, (a4)
pub fn i_05be96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE98;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE98: add.l d4, d7
pub fn i_05be98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE9A: move.l d7, (a4)
pub fn i_05be9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BE9C: add.l d4, d7
pub fn i_05be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BE9E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BE9E: move.l d7, (a4)
pub fn i_05be9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA0: add.l d4, d7
pub fn i_05bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEA2: move.l d7, (a4)
pub fn i_05bea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA4: add.l d4, d7
pub fn i_05bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEA6: move.l d7, (a4)
pub fn i_05bea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEA8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEA8: add.l d4, d7
pub fn i_05bea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEAA: move.l d7, (a4)
pub fn i_05beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEAC: add.l d4, d7
pub fn i_05beac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEAE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEAE: move.l d7, (a4)
pub fn i_05beae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB0: add.l d4, d7
pub fn i_05beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEB2: move.l d7, (a4)
pub fn i_05beb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB4: add.l d4, d7
pub fn i_05beb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEB6: move.l d7, (a4)
pub fn i_05beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEB8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEB8: add.l d4, d7
pub fn i_05beb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEBA: move.l d7, (a4)
pub fn i_05beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEBC: add.l d4, d7
pub fn i_05bebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEBE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEBE: move.l d7, (a4)
pub fn i_05bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC0: add.l d4, d7
pub fn i_05bec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEC2: move.l d7, (a4)
pub fn i_05bec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC4: add.l d4, d7
pub fn i_05bec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEC6: move.l d7, (a4)
pub fn i_05bec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEC8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEC8: add.l d4, d7
pub fn i_05bec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BECA: move.l d7, (a4)
pub fn i_05beca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BECC: add.l d4, d7
pub fn i_05becc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BECE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BECE: move.l d7, (a4)
pub fn i_05bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED0: add.l d4, d7
pub fn i_05bed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BED2: move.l d7, (a4)
pub fn i_05bed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED4: add.l d4, d7
pub fn i_05bed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BED6: move.l d7, (a4)
pub fn i_05bed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BED8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BED8: add.l d4, d7
pub fn i_05bed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEDA: move.l d7, (a4)
pub fn i_05beda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEDC: add.l d4, d7
pub fn i_05bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEDE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEDE: move.l d7, (a4)
pub fn i_05bede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE0: add.l d4, d7
pub fn i_05bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEE2: move.l d7, (a4)
pub fn i_05bee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE4: add.l d4, d7
pub fn i_05bee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEE6: move.l d7, (a4)
pub fn i_05bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEE8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEE8: add.l d4, d7
pub fn i_05bee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEEA: move.l d7, (a4)
pub fn i_05beea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEEC: add.l d4, d7
pub fn i_05beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEEE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEEE: move.l d7, (a4)
pub fn i_05beee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF0: add.l d4, d7
pub fn i_05bef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEF2: move.l d7, (a4)
pub fn i_05bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF4: add.l d4, d7
pub fn i_05bef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEF6: move.l d7, (a4)
pub fn i_05bef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEF8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEF8: add.l d4, d7
pub fn i_05bef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEFA: move.l d7, (a4)
pub fn i_05befa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BEFC: add.l d4, d7
pub fn i_05befc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BEFE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BEFE: move.l d7, (a4)
pub fn i_05befe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF00;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF00: add.l d4, d7
pub fn i_05bf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF02;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF02: move.l d7, (a4)
pub fn i_05bf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF04;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF04: add.l d4, d7
pub fn i_05bf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF06;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF06: move.l d7, (a4)
pub fn i_05bf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF08;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF08: add.l d4, d7
pub fn i_05bf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF0A: move.l d7, (a4)
pub fn i_05bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF0C: add.l d4, d7
pub fn i_05bf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF0E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF0E: move.l d7, (a4)
pub fn i_05bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF10;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF10: add.l d4, d7
pub fn i_05bf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF12;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF12: move.l d7, (a4)
pub fn i_05bf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF14;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF14: add.l d4, d7
pub fn i_05bf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF16;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF16: move.l d7, (a4)
pub fn i_05bf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF18;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF18: add.l d4, d7
pub fn i_05bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF1A: move.l d7, (a4)
pub fn i_05bf1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF1C: add.l d4, d7
pub fn i_05bf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF1E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF1E: move.l d7, (a4)
pub fn i_05bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF20;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF20: add.l d4, d7
pub fn i_05bf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF22;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF22: move.l d7, (a4)
pub fn i_05bf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF24;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF24: add.l d4, d7
pub fn i_05bf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF26;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF26: move.l d7, (a4)
pub fn i_05bf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF28;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF28: add.l d4, d7
pub fn i_05bf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF2A: move.l d7, (a4)
pub fn i_05bf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF2C: add.l d4, d7
pub fn i_05bf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF2E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF2E: move.l d7, (a4)
pub fn i_05bf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF30;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF30: add.l d4, d7
pub fn i_05bf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF32;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF32: move.l d7, (a4)
pub fn i_05bf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF34;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF34: add.l d4, d7
pub fn i_05bf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF36;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF36: move.l d7, (a4)
pub fn i_05bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF38;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF38: add.l d4, d7
pub fn i_05bf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF3A: move.l d7, (a4)
pub fn i_05bf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF3C: add.l d4, d7
pub fn i_05bf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF3E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF3E: move.l d7, (a4)
pub fn i_05bf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF40;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF40: add.l d4, d7
pub fn i_05bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF42;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF42: move.l d7, (a4)
pub fn i_05bf42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF44;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF44: add.l d4, d7
pub fn i_05bf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF46;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF46: move.l d7, (a4)
pub fn i_05bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF48;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF48: add.l d4, d7
pub fn i_05bf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF4A: move.l d7, (a4)
pub fn i_05bf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF4C: add.l d4, d7
pub fn i_05bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF4E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF4E: move.l d7, (a4)
pub fn i_05bf4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF50;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF50: add.l d4, d7
pub fn i_05bf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF52;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF52: move.l d7, (a4)
pub fn i_05bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF54;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF54: add.l d4, d7
pub fn i_05bf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF56;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05BF56: move.w #$8401, d6
pub fn i_05bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5A;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BF5A: add.w d0, d6
pub fn i_05bf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5C;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BF5C: swap d6
pub fn i_05bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF5E;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BF5E: move.w $a(a3), d6
pub fn i_05bf5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF62;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF62: move.l #$2000000, d5
pub fn i_05bf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF68;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BF68: move.l d6, (a4)
pub fn i_05bf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6A;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF6A: sub.l d5, d6
pub fn i_05bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6C;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BF6C: sub.l d5, d6
pub fn i_05bf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF6E;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05BF6E: move.w $6(a3), d6
pub fn i_05bf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF72;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF72: move.l d6, (a4)
pub fn i_05bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF74;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05BF74: rts
pub fn i_05bf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SCB1_WriteTileColumnTerm_05bf76  ($05BF76, 2656 B, video/sprite_queue_render_05aa96.s)
/// $05BF76: move.w $6(a3), d7
pub fn i_05bf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7A;
    cpu.ir = 0x3E2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BF7A: addq.b #$1, d7
pub fn i_05bf7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7C;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05BF7C: sne.b d6
pub fn i_05bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF7E;
    cpu.ir = 0x56C6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ne) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05BF7E: andi.b #$4, d6
pub fn i_05bf7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF82;
    cpu.ir = 0x206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05BF82: rol.w #$7, d7
pub fn i_05bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF84;
    cpu.ir = 0xEF5F;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 3, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $05BF84: andi.w #$8, d7
pub fn i_05bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF88;
    cpu.ir = 0x247;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = (v1 & 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05BF88: or.b d6, d7
pub fn i_05bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF8A;
    cpu.ir = 0x8E06;
    let v1 = cpu.d[6] & 0xFF;
    let r = (v1 | cpu.d[7]) & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05BF8A: jmp $5bf8e(pc, d7.w)
pub fn i_05bf8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF8E;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BF8E_u32.wrapping_add((cpu.d[7] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BF8E: bra.w $5bf9e
pub fn i_05bf8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF92;
    cpu.ir = 0x6000;
    cpu.pc = 0x5BF9E;
    10
}
/// $05BF92: bra.w $5c3b6
pub fn i_05bf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF96;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C3B6;
    10
}
/// $05BF96: bra.w $5c1aa
pub fn i_05bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF9A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C1AA;
    10
}
/// $05BF9A: bra.w $5c6c6
pub fn i_05bf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BF9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C6C6;
    10
}
/// $05BF9E: move.w #$40, d7
pub fn i_05bf9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFA2;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BFA2: add.w d1, d7
pub fn i_05bfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFA4;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BFA4: move.w d7, d5
pub fn i_05bfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFA6;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05BFA6: addq.w #$1, d5
pub fn i_05bfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFA8;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05BFA8: swap d5
pub fn i_05bfa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFAA;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BFAA: move.w $4(a3), d5
pub fn i_05bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFAE;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BFAE: swap d7
pub fn i_05bfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFB0;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BFB0: moveq #$2, d4
pub fn i_05bfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFB2;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BFB2: swap d4
pub fn i_05bfb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFB4;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05BFB4: movea.l (a3), a6
pub fn i_05bfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFB6;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05BFB6: move.w $a(a3), d6
pub fn i_05bfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFBA;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05BFBA: subq.b #$1, d6
pub fn i_05bfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFBC;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05BFBC: andi.w #$1f, d6
pub fn i_05bfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFC0;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05BFC0: add.w d6, d6
pub fn i_05bfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFC2;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BFC2: add.w d6, d6
pub fn i_05bfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFC4;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05BFC4: jmp $5bfc8(pc, d6.w)
pub fn i_05bfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFC8;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5BFC8_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05BFC8: bra.w $5c17a
pub fn i_05bfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C17A;
    10
}
/// $05BFCC: bra.w $5c170
pub fn i_05bfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C170;
    10
}
/// $05BFD0: bra.w $5c166
pub fn i_05bfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFD4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C166;
    10
}
/// $05BFD4: bra.w $5c15c
pub fn i_05bfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C15C;
    10
}
/// $05BFD8: bra.w $5c152
pub fn i_05bfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFDC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C152;
    10
}
/// $05BFDC: bra.w $5c148
pub fn i_05bfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C148;
    10
}
/// $05BFE0: bra.w $5c13e
pub fn i_05bfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C13E;
    10
}
/// $05BFE4: bra.w $5c134
pub fn i_05bfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C134;
    10
}
/// $05BFE8: bra.w $5c12a
pub fn i_05bfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C12A;
    10
}
/// $05BFEC: bra.w $5c120
pub fn i_05bfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C120;
    10
}
/// $05BFF0: bra.w $5c116
pub fn i_05bff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C116;
    10
}
/// $05BFF4: bra.w $5c10c
pub fn i_05bff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C10C;
    10
}
/// $05BFF8: bra.w $5c102
pub fn i_05bff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5BFFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C102;
    10
}
/// $05BFFC: bra.w $5c0f8
pub fn i_05bffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C000;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0F8;
    10
}
/// $05C000: bra.w $5c0ee
pub fn i_05c000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C004;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0EE;
    10
}
/// $05C004: bra.w $5c0e4
pub fn i_05c004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C008;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0E4;
    10
}
/// $05C008: bra.w $5c0da
pub fn i_05c008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C00C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0DA;
    10
}
/// $05C00C: bra.w $5c0d0
pub fn i_05c00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C010;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0D0;
    10
}
/// $05C010: bra.w $5c0c6
pub fn i_05c010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C014;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0C6;
    10
}
/// $05C014: bra.w $5c0bc
pub fn i_05c014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C018;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0BC;
    10
}
/// $05C018: bra.w $5c0b2
pub fn i_05c018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C01C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0B2;
    10
}
/// $05C01C: bra.w $5c0a8
pub fn i_05c01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C020;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C0A8;
    10
}
/// $05C020: bra.w $5c09e
pub fn i_05c020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C024;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C09E;
    10
}
/// $05C024: bra.w $5c094
pub fn i_05c024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C028;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C094;
    10
}
/// $05C028: bra.w $5c08a
pub fn i_05c028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C02C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C08A;
    10
}
/// $05C02C: bra.w $5c080
pub fn i_05c02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C030;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C080;
    10
}
/// $05C030: bra.w $5c076
pub fn i_05c030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C034;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C076;
    10
}
/// $05C034: bra.w $5c06c
pub fn i_05c034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C038;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C06C;
    10
}
/// $05C038: bra.w $5c062
pub fn i_05c038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C03C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C062;
    10
}
/// $05C03C: bra.w $5c058
pub fn i_05c03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C040;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C058;
    10
}
/// $05C040: bra.w $5c04e
pub fn i_05c040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C044;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C04E;
    10
}
/// $05C044: move.w (a6)+, d7
pub fn i_05c044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C046;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C046: move.l d7, (a4)
pub fn i_05c046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C048;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C048: add.l d4, d7
pub fn i_05c048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C04A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C04A: move.l d5, (a4)
pub fn i_05c04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C04C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C04C: add.l d4, d5
pub fn i_05c04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C04E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C04E: move.w (a6)+, d7
pub fn i_05c04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C050;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C050: move.l d7, (a4)
pub fn i_05c050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C052;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C052: add.l d4, d7
pub fn i_05c052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C054;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C054: move.l d5, (a4)
pub fn i_05c054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C056;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C056: add.l d4, d5
pub fn i_05c056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C058;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C058: move.w (a6)+, d7
pub fn i_05c058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C05A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C05A: move.l d7, (a4)
pub fn i_05c05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C05C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C05C: add.l d4, d7
pub fn i_05c05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C05E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C05E: move.l d5, (a4)
pub fn i_05c05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C060;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C060: add.l d4, d5
pub fn i_05c060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C062;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C062: move.w (a6)+, d7
pub fn i_05c062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C064;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C064: move.l d7, (a4)
pub fn i_05c064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C066;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C066: add.l d4, d7
pub fn i_05c066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C068;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C068: move.l d5, (a4)
pub fn i_05c068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C06A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C06A: add.l d4, d5
pub fn i_05c06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C06C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C06C: move.w (a6)+, d7
pub fn i_05c06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C06E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C06E: move.l d7, (a4)
pub fn i_05c06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C070;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C070: add.l d4, d7
pub fn i_05c070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C072;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C072: move.l d5, (a4)
pub fn i_05c072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C074;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C074: add.l d4, d5
pub fn i_05c074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C076;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C076: move.w (a6)+, d7
pub fn i_05c076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C078;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C078: move.l d7, (a4)
pub fn i_05c078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C07A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C07A: add.l d4, d7
pub fn i_05c07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C07C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C07C: move.l d5, (a4)
pub fn i_05c07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C07E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C07E: add.l d4, d5
pub fn i_05c07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C080;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C080: move.w (a6)+, d7
pub fn i_05c080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C082;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C082: move.l d7, (a4)
pub fn i_05c082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C084;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C084: add.l d4, d7
pub fn i_05c084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C086;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C086: move.l d5, (a4)
pub fn i_05c086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C088;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C088: add.l d4, d5
pub fn i_05c088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C08A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C08A: move.w (a6)+, d7
pub fn i_05c08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C08C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C08C: move.l d7, (a4)
pub fn i_05c08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C08E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C08E: add.l d4, d7
pub fn i_05c08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C090;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C090: move.l d5, (a4)
pub fn i_05c090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C092;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C092: add.l d4, d5
pub fn i_05c092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C094;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C094: move.w (a6)+, d7
pub fn i_05c094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C096;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C096: move.l d7, (a4)
pub fn i_05c096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C098;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C098: add.l d4, d7
pub fn i_05c098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C09A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C09A: move.l d5, (a4)
pub fn i_05c09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C09C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C09C: add.l d4, d5
pub fn i_05c09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C09E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C09E: move.w (a6)+, d7
pub fn i_05c09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0A0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0A0: move.l d7, (a4)
pub fn i_05c0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0A2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0A2: add.l d4, d7
pub fn i_05c0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0A4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0A4: move.l d5, (a4)
pub fn i_05c0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0A6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0A6: add.l d4, d5
pub fn i_05c0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0A8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0A8: move.w (a6)+, d7
pub fn i_05c0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0AA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0AA: move.l d7, (a4)
pub fn i_05c0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0AC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0AC: add.l d4, d7
pub fn i_05c0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0AE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0AE: move.l d5, (a4)
pub fn i_05c0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0B0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0B0: add.l d4, d5
pub fn i_05c0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0B2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0B2: move.w (a6)+, d7
pub fn i_05c0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0B4;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0B4: move.l d7, (a4)
pub fn i_05c0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0B6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0B6: add.l d4, d7
pub fn i_05c0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0B8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0B8: move.l d5, (a4)
pub fn i_05c0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0BA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0BA: add.l d4, d5
pub fn i_05c0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0BC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0BC: move.w (a6)+, d7
pub fn i_05c0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0BE;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0BE: move.l d7, (a4)
pub fn i_05c0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0C0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0C0: add.l d4, d7
pub fn i_05c0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0C2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0C2: move.l d5, (a4)
pub fn i_05c0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0C4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0C4: add.l d4, d5
pub fn i_05c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0C6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0C6: move.w (a6)+, d7
pub fn i_05c0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0C8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0C8: move.l d7, (a4)
pub fn i_05c0c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0CA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0CA: add.l d4, d7
pub fn i_05c0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0CC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0CC: move.l d5, (a4)
pub fn i_05c0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0CE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0CE: add.l d4, d5
pub fn i_05c0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0D0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0D0: move.w (a6)+, d7
pub fn i_05c0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0D2;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0D2: move.l d7, (a4)
pub fn i_05c0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0D4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0D4: add.l d4, d7
pub fn i_05c0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0D6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0D6: move.l d5, (a4)
pub fn i_05c0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0D8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0D8: add.l d4, d5
pub fn i_05c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0DA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0DA: move.w (a6)+, d7
pub fn i_05c0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0DC;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0DC: move.l d7, (a4)
pub fn i_05c0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0DE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0DE: add.l d4, d7
pub fn i_05c0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0E0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0E0: move.l d5, (a4)
pub fn i_05c0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0E2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0E2: add.l d4, d5
pub fn i_05c0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0E4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0E4: move.w (a6)+, d7
pub fn i_05c0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0E6;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0E6: move.l d7, (a4)
pub fn i_05c0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0E8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0E8: add.l d4, d7
pub fn i_05c0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0EA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0EA: move.l d5, (a4)
pub fn i_05c0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0EC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0EC: add.l d4, d5
pub fn i_05c0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0EE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0EE: move.w (a6)+, d7
pub fn i_05c0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0F0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0F0: move.l d7, (a4)
pub fn i_05c0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0F2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0F2: add.l d4, d7
pub fn i_05c0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0F4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0F4: move.l d5, (a4)
pub fn i_05c0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0F6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0F6: add.l d4, d5
pub fn i_05c0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0F8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C0F8: move.w (a6)+, d7
pub fn i_05c0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0FA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C0FA: move.l d7, (a4)
pub fn i_05c0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0FC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C0FC: add.l d4, d7
pub fn i_05c0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C0FE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C0FE: move.l d5, (a4)
pub fn i_05c0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C100;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C100: add.l d4, d5
pub fn i_05c100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C102;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C102: move.w (a6)+, d7
pub fn i_05c102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C104;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C104: move.l d7, (a4)
pub fn i_05c104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C106;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C106: add.l d4, d7
pub fn i_05c106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C108;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C108: move.l d5, (a4)
pub fn i_05c108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C10A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C10A: add.l d4, d5
pub fn i_05c10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C10C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C10C: move.w (a6)+, d7
pub fn i_05c10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C10E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C10E: move.l d7, (a4)
pub fn i_05c10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C110;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C110: add.l d4, d7
pub fn i_05c110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C112;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C112: move.l d5, (a4)
pub fn i_05c112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C114;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C114: add.l d4, d5
pub fn i_05c114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C116;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C116: move.w (a6)+, d7
pub fn i_05c116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C118;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C118: move.l d7, (a4)
pub fn i_05c118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C11A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C11A: add.l d4, d7
pub fn i_05c11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C11C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C11C: move.l d5, (a4)
pub fn i_05c11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C11E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C11E: add.l d4, d5
pub fn i_05c11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C120;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C120: move.w (a6)+, d7
pub fn i_05c120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C122;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C122: move.l d7, (a4)
pub fn i_05c122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C124;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C124: add.l d4, d7
pub fn i_05c124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C126;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C126: move.l d5, (a4)
pub fn i_05c126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C128;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C128: add.l d4, d5
pub fn i_05c128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C12A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C12A: move.w (a6)+, d7
pub fn i_05c12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C12C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C12C: move.l d7, (a4)
pub fn i_05c12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C12E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C12E: add.l d4, d7
pub fn i_05c12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C130;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C130: move.l d5, (a4)
pub fn i_05c130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C132;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C132: add.l d4, d5
pub fn i_05c132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C134;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C134: move.w (a6)+, d7
pub fn i_05c134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C136;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C136: move.l d7, (a4)
pub fn i_05c136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C138;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C138: add.l d4, d7
pub fn i_05c138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C13A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C13A: move.l d5, (a4)
pub fn i_05c13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C13C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C13C: add.l d4, d5
pub fn i_05c13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C13E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C13E: move.w (a6)+, d7
pub fn i_05c13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C140;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C140: move.l d7, (a4)
pub fn i_05c140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C142;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C142: add.l d4, d7
pub fn i_05c142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C144;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C144: move.l d5, (a4)
pub fn i_05c144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C146;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C146: add.l d4, d5
pub fn i_05c146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C148;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C148: move.w (a6)+, d7
pub fn i_05c148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C14A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C14A: move.l d7, (a4)
pub fn i_05c14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C14C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C14C: add.l d4, d7
pub fn i_05c14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C14E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C14E: move.l d5, (a4)
pub fn i_05c14e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C150;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C150: add.l d4, d5
pub fn i_05c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C152;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C152: move.w (a6)+, d7
pub fn i_05c152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C154;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C154: move.l d7, (a4)
pub fn i_05c154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C156;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C156: add.l d4, d7
pub fn i_05c156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C158;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C158: move.l d5, (a4)
pub fn i_05c158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C15A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C15A: add.l d4, d5
pub fn i_05c15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C15C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C15C: move.w (a6)+, d7
pub fn i_05c15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C15E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C15E: move.l d7, (a4)
pub fn i_05c15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C160;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C160: add.l d4, d7
pub fn i_05c160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C162;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C162: move.l d5, (a4)
pub fn i_05c162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C164;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C164: add.l d4, d5
pub fn i_05c164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C166;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C166: move.w (a6)+, d7
pub fn i_05c166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C168;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C168: move.l d7, (a4)
pub fn i_05c168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C16A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C16A: add.l d4, d7
pub fn i_05c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C16C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C16C: move.l d5, (a4)
pub fn i_05c16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C16E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C16E: add.l d4, d5
pub fn i_05c16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C170;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C170: move.w (a6)+, d7
pub fn i_05c170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C172;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C172: move.l d7, (a4)
pub fn i_05c172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C174;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C174: add.l d4, d7
pub fn i_05c174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C176;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C176: move.l d5, (a4)
pub fn i_05c176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C178;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C178: add.l d4, d5
pub fn i_05c178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C17A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C17A: move.w (a6)+, d7
pub fn i_05c17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C17C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C17C: move.l d7, (a4)
pub fn i_05c17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C17E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C17E: add.l d4, d7
pub fn i_05c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C180;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C180: move.l d5, (a4)
pub fn i_05c180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C182;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C182: add.l d4, d5
pub fn i_05c182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C184;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C184: move.w #$8401, d6
pub fn i_05c184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C188;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C188: add.w d0, d6
pub fn i_05c188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C18A;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C18A: swap d6
pub fn i_05c18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C18C;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C18C: move.w $a(a3), d6
pub fn i_05c18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C190;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C190: move.l #$2000000, d5
pub fn i_05c190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C196;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C196: move.l d6, (a4)
pub fn i_05c196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C198;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C198: sub.l d5, d6
pub fn i_05c198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C19A;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C19A: move.w $8(a3), d6
pub fn i_05c19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C19E;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C19E: move.l d6, (a4)
pub fn i_05c19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1A0;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C1A0: sub.l d5, d6
pub fn i_05c1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1A2;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C1A2: move.w $6(a3), d6
pub fn i_05c1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1A6;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C1A6: move.l d6, (a4)
pub fn i_05c1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1A8;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C1A8: rts
pub fn i_05c1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05C1AA: move.w #$40, d7
pub fn i_05c1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1AE;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C1AE: add.w d1, d7
pub fn i_05c1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1B0;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C1B0: move.w d7, d5
pub fn i_05c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1B2;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C1B2: addq.w #$1, d5
pub fn i_05c1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1B4;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05C1B4: swap d5
pub fn i_05c1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1B6;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C1B6: move.w $4(a3), d5
pub fn i_05c1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1BA;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C1BA: swap d7
pub fn i_05c1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1BC;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C1BC: moveq #$2, d4
pub fn i_05c1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1BE;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C1BE: swap d4
pub fn i_05c1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1C0;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C1C0: movea.l (a3), a6
pub fn i_05c1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1C2;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05C1C2: move.w $a(a3), d6
pub fn i_05c1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1C6;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C1C6: subq.b #$1, d6
pub fn i_05c1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1C8;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05C1C8: andi.w #$1f, d6
pub fn i_05c1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1CC;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05C1CC: add.w d6, d6
pub fn i_05c1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1CE;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C1CE: add.w d6, d6
pub fn i_05c1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1D0;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C1D0: jmp $5c1d4(pc, d6.w)
pub fn i_05c1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1D4;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5C1D4_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05C1D4: bra.w $5c386
pub fn i_05c1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C386;
    10
}
/// $05C1D8: bra.w $5c37c
pub fn i_05c1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C37C;
    10
}
/// $05C1DC: bra.w $5c372
pub fn i_05c1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C372;
    10
}
/// $05C1E0: bra.w $5c368
pub fn i_05c1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C368;
    10
}
/// $05C1E4: bra.w $5c35e
pub fn i_05c1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C35E;
    10
}
/// $05C1E8: bra.w $5c354
pub fn i_05c1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C354;
    10
}
/// $05C1EC: bra.w $5c34a
pub fn i_05c1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C34A;
    10
}
/// $05C1F0: bra.w $5c340
pub fn i_05c1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C340;
    10
}
/// $05C1F4: bra.w $5c336
pub fn i_05c1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C336;
    10
}
/// $05C1F8: bra.w $5c32c
pub fn i_05c1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C1FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C32C;
    10
}
/// $05C1FC: bra.w $5c322
pub fn i_05c1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C200;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C322;
    10
}
/// $05C200: bra.w $5c318
pub fn i_05c200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C204;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C318;
    10
}
/// $05C204: bra.w $5c30e
pub fn i_05c204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C208;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C30E;
    10
}
/// $05C208: bra.w $5c304
pub fn i_05c208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C20C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C304;
    10
}
/// $05C20C: bra.w $5c2fa
pub fn i_05c20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C210;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2FA;
    10
}
/// $05C210: bra.w $5c2f0
pub fn i_05c210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C214;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2F0;
    10
}
/// $05C214: bra.w $5c2e6
pub fn i_05c214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C218;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2E6;
    10
}
/// $05C218: bra.w $5c2dc
pub fn i_05c218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C21C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2DC;
    10
}
/// $05C21C: bra.w $5c2d2
pub fn i_05c21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C220;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2D2;
    10
}
/// $05C220: bra.w $5c2c8
pub fn i_05c220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C224;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2C8;
    10
}
/// $05C224: bra.w $5c2be
pub fn i_05c224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C228;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2BE;
    10
}
/// $05C228: bra.w $5c2b4
pub fn i_05c228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C22C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2B4;
    10
}
/// $05C22C: bra.w $5c2aa
pub fn i_05c22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C230;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2AA;
    10
}
/// $05C230: bra.w $5c2a0
pub fn i_05c230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C234;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C2A0;
    10
}
/// $05C234: bra.w $5c296
pub fn i_05c234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C238;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C296;
    10
}
/// $05C238: bra.w $5c28c
pub fn i_05c238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C23C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C28C;
    10
}
/// $05C23C: bra.w $5c282
pub fn i_05c23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C240;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C282;
    10
}
/// $05C240: bra.w $5c278
pub fn i_05c240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C244;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C278;
    10
}
/// $05C244: bra.w $5c26e
pub fn i_05c244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C248;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C26E;
    10
}
/// $05C248: bra.w $5c264
pub fn i_05c248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C24C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C264;
    10
}
/// $05C24C: bra.w $5c25a
pub fn i_05c24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C250;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C25A;
    10
}
/// $05C250: move.w -(a6), d7
pub fn i_05c250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C252;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C252: move.l d7, (a4)
pub fn i_05c252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C254;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C254: add.l d4, d7
pub fn i_05c254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C256;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C256: move.l d5, (a4)
pub fn i_05c256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C258;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C258: add.l d4, d5
pub fn i_05c258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C25A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C25A: move.w -(a6), d7
pub fn i_05c25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C25C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C25C: move.l d7, (a4)
pub fn i_05c25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C25E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C25E: add.l d4, d7
pub fn i_05c25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C260;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C260: move.l d5, (a4)
pub fn i_05c260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C262;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C262: add.l d4, d5
pub fn i_05c262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C264;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C264: move.w -(a6), d7
pub fn i_05c264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C266;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C266: move.l d7, (a4)
pub fn i_05c266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C268;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C268: add.l d4, d7
pub fn i_05c268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C26A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C26A: move.l d5, (a4)
pub fn i_05c26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C26C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C26C: add.l d4, d5
pub fn i_05c26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C26E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C26E: move.w -(a6), d7
pub fn i_05c26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C270;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C270: move.l d7, (a4)
pub fn i_05c270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C272;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C272: add.l d4, d7
pub fn i_05c272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C274;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C274: move.l d5, (a4)
pub fn i_05c274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C276;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C276: add.l d4, d5
pub fn i_05c276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C278;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C278: move.w -(a6), d7
pub fn i_05c278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C27A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C27A: move.l d7, (a4)
pub fn i_05c27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C27C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C27C: add.l d4, d7
pub fn i_05c27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C27E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C27E: move.l d5, (a4)
pub fn i_05c27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C280;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C280: add.l d4, d5
pub fn i_05c280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C282;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C282: move.w -(a6), d7
pub fn i_05c282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C284;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C284: move.l d7, (a4)
pub fn i_05c284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C286;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C286: add.l d4, d7
pub fn i_05c286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C288;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C288: move.l d5, (a4)
pub fn i_05c288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C28A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C28A: add.l d4, d5
pub fn i_05c28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C28C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C28C: move.w -(a6), d7
pub fn i_05c28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C28E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C28E: move.l d7, (a4)
pub fn i_05c28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C290;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C290: add.l d4, d7
pub fn i_05c290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C292;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C292: move.l d5, (a4)
pub fn i_05c292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C294;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C294: add.l d4, d5
pub fn i_05c294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C296;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C296: move.w -(a6), d7
pub fn i_05c296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C298;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C298: move.l d7, (a4)
pub fn i_05c298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C29A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C29A: add.l d4, d7
pub fn i_05c29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C29C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C29C: move.l d5, (a4)
pub fn i_05c29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C29E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C29E: add.l d4, d5
pub fn i_05c29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2A0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2A0: move.w -(a6), d7
pub fn i_05c2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2A2;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2A2: move.l d7, (a4)
pub fn i_05c2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2A4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2A4: add.l d4, d7
pub fn i_05c2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2A6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2A6: move.l d5, (a4)
pub fn i_05c2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2A8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2A8: add.l d4, d5
pub fn i_05c2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2AA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2AA: move.w -(a6), d7
pub fn i_05c2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2AC;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2AC: move.l d7, (a4)
pub fn i_05c2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2AE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2AE: add.l d4, d7
pub fn i_05c2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2B0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2B0: move.l d5, (a4)
pub fn i_05c2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2B2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2B2: add.l d4, d5
pub fn i_05c2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2B4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2B4: move.w -(a6), d7
pub fn i_05c2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2B6;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2B6: move.l d7, (a4)
pub fn i_05c2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2B8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2B8: add.l d4, d7
pub fn i_05c2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2BA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2BA: move.l d5, (a4)
pub fn i_05c2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2BC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2BC: add.l d4, d5
pub fn i_05c2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2BE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2BE: move.w -(a6), d7
pub fn i_05c2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2C0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2C0: move.l d7, (a4)
pub fn i_05c2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2C2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2C2: add.l d4, d7
pub fn i_05c2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2C4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2C4: move.l d5, (a4)
pub fn i_05c2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2C6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2C6: add.l d4, d5
pub fn i_05c2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2C8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2C8: move.w -(a6), d7
pub fn i_05c2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2CA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2CA: move.l d7, (a4)
pub fn i_05c2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2CC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2CC: add.l d4, d7
pub fn i_05c2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2CE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2CE: move.l d5, (a4)
pub fn i_05c2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2D0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2D0: add.l d4, d5
pub fn i_05c2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2D2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2D2: move.w -(a6), d7
pub fn i_05c2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2D4;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2D4: move.l d7, (a4)
pub fn i_05c2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2D6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2D6: add.l d4, d7
pub fn i_05c2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2D8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2D8: move.l d5, (a4)
pub fn i_05c2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2DA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2DA: add.l d4, d5
pub fn i_05c2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2DC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2DC: move.w -(a6), d7
pub fn i_05c2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2DE;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2DE: move.l d7, (a4)
pub fn i_05c2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2E0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2E0: add.l d4, d7
pub fn i_05c2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2E2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2E2: move.l d5, (a4)
pub fn i_05c2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2E4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2E4: add.l d4, d5
pub fn i_05c2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2E6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2E6: move.w -(a6), d7
pub fn i_05c2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2E8;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2E8: move.l d7, (a4)
pub fn i_05c2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2EA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2EA: add.l d4, d7
pub fn i_05c2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2EC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2EC: move.l d5, (a4)
pub fn i_05c2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2EE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2EE: add.l d4, d5
pub fn i_05c2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2F0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2F0: move.w -(a6), d7
pub fn i_05c2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2F2;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2F2: move.l d7, (a4)
pub fn i_05c2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2F4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2F4: add.l d4, d7
pub fn i_05c2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2F6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C2F6: move.l d5, (a4)
pub fn i_05c2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2F8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2F8: add.l d4, d5
pub fn i_05c2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2FA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C2FA: move.w -(a6), d7
pub fn i_05c2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2FC;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C2FC: move.l d7, (a4)
pub fn i_05c2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C2FE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C2FE: add.l d4, d7
pub fn i_05c2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C300;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C300: move.l d5, (a4)
pub fn i_05c300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C302;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C302: add.l d4, d5
pub fn i_05c302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C304;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C304: move.w -(a6), d7
pub fn i_05c304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C306;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C306: move.l d7, (a4)
pub fn i_05c306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C308;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C308: add.l d4, d7
pub fn i_05c308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C30A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C30A: move.l d5, (a4)
pub fn i_05c30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C30C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C30C: add.l d4, d5
pub fn i_05c30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C30E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C30E: move.w -(a6), d7
pub fn i_05c30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C310;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C310: move.l d7, (a4)
pub fn i_05c310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C312;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C312: add.l d4, d7
pub fn i_05c312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C314;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C314: move.l d5, (a4)
pub fn i_05c314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C316;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C316: add.l d4, d5
pub fn i_05c316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C318;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C318: move.w -(a6), d7
pub fn i_05c318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C31A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C31A: move.l d7, (a4)
pub fn i_05c31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C31C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C31C: add.l d4, d7
pub fn i_05c31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C31E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C31E: move.l d5, (a4)
pub fn i_05c31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C320;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C320: add.l d4, d5
pub fn i_05c320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C322;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C322: move.w -(a6), d7
pub fn i_05c322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C324;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C324: move.l d7, (a4)
pub fn i_05c324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C326;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C326: add.l d4, d7
pub fn i_05c326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C328;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C328: move.l d5, (a4)
pub fn i_05c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C32A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C32A: add.l d4, d5
pub fn i_05c32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C32C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C32C: move.w -(a6), d7
pub fn i_05c32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C32E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C32E: move.l d7, (a4)
pub fn i_05c32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C330;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C330: add.l d4, d7
pub fn i_05c330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C332;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C332: move.l d5, (a4)
pub fn i_05c332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C334;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C334: add.l d4, d5
pub fn i_05c334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C336;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C336: move.w -(a6), d7
pub fn i_05c336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C338;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C338: move.l d7, (a4)
pub fn i_05c338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C33A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C33A: add.l d4, d7
pub fn i_05c33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C33C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C33C: move.l d5, (a4)
pub fn i_05c33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C33E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C33E: add.l d4, d5
pub fn i_05c33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C340;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C340: move.w -(a6), d7
pub fn i_05c340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C342;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C342: move.l d7, (a4)
pub fn i_05c342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C344;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C344: add.l d4, d7
pub fn i_05c344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C346;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C346: move.l d5, (a4)
pub fn i_05c346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C348;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C348: add.l d4, d5
pub fn i_05c348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C34A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C34A: move.w -(a6), d7
pub fn i_05c34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C34C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C34C: move.l d7, (a4)
pub fn i_05c34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C34E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C34E: add.l d4, d7
pub fn i_05c34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C350;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C350: move.l d5, (a4)
pub fn i_05c350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C352;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C352: add.l d4, d5
pub fn i_05c352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C354;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C354: move.w -(a6), d7
pub fn i_05c354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C356;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C356: move.l d7, (a4)
pub fn i_05c356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C358;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C358: add.l d4, d7
pub fn i_05c358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C35A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C35A: move.l d5, (a4)
pub fn i_05c35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C35C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C35C: add.l d4, d5
pub fn i_05c35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C35E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C35E: move.w -(a6), d7
pub fn i_05c35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C360;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C360: move.l d7, (a4)
pub fn i_05c360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C362;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C362: add.l d4, d7
pub fn i_05c362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C364;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C364: move.l d5, (a4)
pub fn i_05c364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C366;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C366: add.l d4, d5
pub fn i_05c366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C368;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C368: move.w -(a6), d7
pub fn i_05c368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C36A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C36A: move.l d7, (a4)
pub fn i_05c36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C36C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C36C: add.l d4, d7
pub fn i_05c36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C36E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C36E: move.l d5, (a4)
pub fn i_05c36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C370;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C370: add.l d4, d5
pub fn i_05c370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C372;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C372: move.w -(a6), d7
pub fn i_05c372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C374;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C374: move.l d7, (a4)
pub fn i_05c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C376;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C376: add.l d4, d7
pub fn i_05c376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C378;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C378: move.l d5, (a4)
pub fn i_05c378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C37A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C37A: add.l d4, d5
pub fn i_05c37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C37C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C37C: move.w -(a6), d7
pub fn i_05c37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C37E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C37E: move.l d7, (a4)
pub fn i_05c37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C380;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C380: add.l d4, d7
pub fn i_05c380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C382;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C382: move.l d5, (a4)
pub fn i_05c382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C384;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C384: add.l d4, d5
pub fn i_05c384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C386;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C386: move.w -(a6), d7
pub fn i_05c386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C388;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C388: move.l d7, (a4)
pub fn i_05c388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C38A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C38A: add.l d4, d7
pub fn i_05c38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C38C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C38C: move.l d5, (a4)
pub fn i_05c38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C38E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C38E: add.l d4, d5
pub fn i_05c38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C390;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C390: move.w #$8401, d6
pub fn i_05c390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C394;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C394: add.w d0, d6
pub fn i_05c394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C396;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C396: swap d6
pub fn i_05c396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C398;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C398: move.w $a(a3), d6
pub fn i_05c398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C39C;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C39C: move.l #$2000000, d5
pub fn i_05c39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3A2;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C3A2: move.l d6, (a4)
pub fn i_05c3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3A4;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C3A4: sub.l d5, d6
pub fn i_05c3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3A6;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C3A6: move.w $8(a3), d6
pub fn i_05c3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3AA;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C3AA: move.l d6, (a4)
pub fn i_05c3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3AC;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C3AC: sub.l d5, d6
pub fn i_05c3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3AE;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C3AE: move.w $6(a3), d6
pub fn i_05c3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3B2;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C3B2: move.l d6, (a4)
pub fn i_05c3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3B4;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C3B4: rts
pub fn i_05c3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05C3B6: move.w #$40, d7
pub fn i_05c3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3BA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C3BA: add.w d1, d7
pub fn i_05c3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3BC;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C3BC: move.w d7, d5
pub fn i_05c3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3BE;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C3BE: addq.w #$1, d5
pub fn i_05c3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3C0;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05C3C0: swap d5
pub fn i_05c3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3C2;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C3C2: move.w $4(a3), d5
pub fn i_05c3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3C6;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C3C6: swap d7
pub fn i_05c3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3C8;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C3C8: moveq #$2, d4
pub fn i_05c3c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3CA;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C3CA: swap d4
pub fn i_05c3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3CC;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C3CC: movea.l (a3), a6
pub fn i_05c3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3CE;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05C3CE: move.w $a(a3), d6
pub fn i_05c3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3D2;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C3D2: subq.b #$1, d6
pub fn i_05c3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3D4;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05C3D4: andi.w #$1f, d6
pub fn i_05c3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3D8;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05C3D8: add.w d6, d6
pub fn i_05c3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3DA;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C3DA: add.w d6, d6
pub fn i_05c3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3DC;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C3DC: jmp $5c3e0(pc, d6.w)
pub fn i_05c3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3E0;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5C3E0_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05C3E0: bra.w $5c592
pub fn i_05c3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3E4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C592;
    10
}
/// $05C3E4: bra.w $5c588
pub fn i_05c3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C588;
    10
}
/// $05C3E8: bra.w $5c57e
pub fn i_05c3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3EC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C57E;
    10
}
/// $05C3EC: bra.w $5c574
pub fn i_05c3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C574;
    10
}
/// $05C3F0: bra.w $5c56a
pub fn i_05c3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C56A;
    10
}
/// $05C3F4: bra.w $5c560
pub fn i_05c3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C560;
    10
}
/// $05C3F8: bra.w $5c556
pub fn i_05c3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C3FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C556;
    10
}
/// $05C3FC: bra.w $5c54c
pub fn i_05c3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C400;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C54C;
    10
}
/// $05C400: bra.w $5c542
pub fn i_05c400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C404;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C542;
    10
}
/// $05C404: bra.w $5c538
pub fn i_05c404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C408;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C538;
    10
}
/// $05C408: bra.w $5c52e
pub fn i_05c408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C40C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C52E;
    10
}
/// $05C40C: bra.w $5c524
pub fn i_05c40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C410;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C524;
    10
}
/// $05C410: bra.w $5c51a
pub fn i_05c410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C414;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C51A;
    10
}
/// $05C414: bra.w $5c510
pub fn i_05c414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C418;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C510;
    10
}
/// $05C418: bra.w $5c506
pub fn i_05c418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C41C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C506;
    10
}
/// $05C41C: bra.w $5c4fc
pub fn i_05c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C420;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4FC;
    10
}
/// $05C420: bra.w $5c4f2
pub fn i_05c420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C424;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4F2;
    10
}
/// $05C424: bra.w $5c4e8
pub fn i_05c424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C428;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4E8;
    10
}
/// $05C428: bra.w $5c4de
pub fn i_05c428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C42C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4DE;
    10
}
/// $05C42C: bra.w $5c4d4
pub fn i_05c42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C430;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4D4;
    10
}
/// $05C430: bra.w $5c4ca
pub fn i_05c430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C434;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4CA;
    10
}
/// $05C434: bra.w $5c4c0
pub fn i_05c434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C438;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4C0;
    10
}
/// $05C438: bra.w $5c4b6
pub fn i_05c438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C43C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4B6;
    10
}
/// $05C43C: bra.w $5c4ac
pub fn i_05c43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C440;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4AC;
    10
}
/// $05C440: bra.w $5c4a2
pub fn i_05c440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C444;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C4A2;
    10
}
/// $05C444: bra.w $5c498
pub fn i_05c444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C448;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C498;
    10
}
/// $05C448: bra.w $5c48e
pub fn i_05c448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C44C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C48E;
    10
}
/// $05C44C: bra.w $5c484
pub fn i_05c44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C450;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C484;
    10
}
/// $05C450: bra.w $5c47a
pub fn i_05c450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C454;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C47A;
    10
}
/// $05C454: bra.w $5c470
pub fn i_05c454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C458;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C470;
    10
}
/// $05C458: bra.w $5c466
pub fn i_05c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C45C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C466;
    10
}
/// $05C45C: move.w (a6)+, d7
pub fn i_05c45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C45E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C45E: move.l d7, (a4)
pub fn i_05c45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C460;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C460: add.l d4, d7
pub fn i_05c460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C462;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C462: move.l d5, (a4)
pub fn i_05c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C464;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C464: add.l d4, d5
pub fn i_05c464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C466;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C466: move.w (a6)+, d7
pub fn i_05c466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C468;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C468: move.l d7, (a4)
pub fn i_05c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C46A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C46A: add.l d4, d7
pub fn i_05c46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C46C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C46C: move.l d5, (a4)
pub fn i_05c46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C46E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C46E: add.l d4, d5
pub fn i_05c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C470;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C470: move.w (a6)+, d7
pub fn i_05c470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C472;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C472: move.l d7, (a4)
pub fn i_05c472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C474;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C474: add.l d4, d7
pub fn i_05c474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C476;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C476: move.l d5, (a4)
pub fn i_05c476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C478;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C478: add.l d4, d5
pub fn i_05c478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C47A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C47A: move.w (a6)+, d7
pub fn i_05c47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C47C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C47C: move.l d7, (a4)
pub fn i_05c47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C47E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C47E: add.l d4, d7
pub fn i_05c47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C480;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C480: move.l d5, (a4)
pub fn i_05c480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C482;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C482: add.l d4, d5
pub fn i_05c482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C484;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C484: move.w (a6)+, d7
pub fn i_05c484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C486;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C486: move.l d7, (a4)
pub fn i_05c486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C488;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C488: add.l d4, d7
pub fn i_05c488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C48A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C48A: move.l d5, (a4)
pub fn i_05c48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C48C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C48C: add.l d4, d5
pub fn i_05c48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C48E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C48E: move.w (a6)+, d7
pub fn i_05c48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C490;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C490: move.l d7, (a4)
pub fn i_05c490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C492;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C492: add.l d4, d7
pub fn i_05c492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C494;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C494: move.l d5, (a4)
pub fn i_05c494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C496;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C496: add.l d4, d5
pub fn i_05c496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C498;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C498: move.w (a6)+, d7
pub fn i_05c498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C49A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C49A: move.l d7, (a4)
pub fn i_05c49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C49C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C49C: add.l d4, d7
pub fn i_05c49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C49E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C49E: move.l d5, (a4)
pub fn i_05c49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4A0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4A0: add.l d4, d5
pub fn i_05c4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4A2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4A2: move.w (a6)+, d7
pub fn i_05c4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4A4;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4A4: move.l d7, (a4)
pub fn i_05c4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4A6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4A6: add.l d4, d7
pub fn i_05c4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4A8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4A8: move.l d5, (a4)
pub fn i_05c4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4AA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4AA: add.l d4, d5
pub fn i_05c4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4AC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4AC: move.w (a6)+, d7
pub fn i_05c4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4AE;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4AE: move.l d7, (a4)
pub fn i_05c4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4B0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4B0: add.l d4, d7
pub fn i_05c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4B2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4B2: move.l d5, (a4)
pub fn i_05c4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4B4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4B4: add.l d4, d5
pub fn i_05c4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4B6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4B6: move.w (a6)+, d7
pub fn i_05c4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4B8;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4B8: move.l d7, (a4)
pub fn i_05c4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4BA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4BA: add.l d4, d7
pub fn i_05c4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4BC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4BC: move.l d5, (a4)
pub fn i_05c4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4BE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4BE: add.l d4, d5
pub fn i_05c4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4C0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4C0: move.w (a6)+, d7
pub fn i_05c4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4C2;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4C2: move.l d7, (a4)
pub fn i_05c4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4C4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4C4: add.l d4, d7
pub fn i_05c4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4C6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4C6: move.l d5, (a4)
pub fn i_05c4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4C8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4C8: add.l d4, d5
pub fn i_05c4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4CA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4CA: move.w (a6)+, d7
pub fn i_05c4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4CC;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4CC: move.l d7, (a4)
pub fn i_05c4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4CE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4CE: add.l d4, d7
pub fn i_05c4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4D0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4D0: move.l d5, (a4)
pub fn i_05c4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4D2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4D2: add.l d4, d5
pub fn i_05c4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4D4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4D4: move.w (a6)+, d7
pub fn i_05c4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4D6;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4D6: move.l d7, (a4)
pub fn i_05c4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4D8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4D8: add.l d4, d7
pub fn i_05c4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4DA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4DA: move.l d5, (a4)
pub fn i_05c4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4DC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4DC: add.l d4, d5
pub fn i_05c4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4DE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4DE: move.w (a6)+, d7
pub fn i_05c4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4E0;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4E0: move.l d7, (a4)
pub fn i_05c4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4E2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4E2: add.l d4, d7
pub fn i_05c4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4E4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4E4: move.l d5, (a4)
pub fn i_05c4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4E6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4E6: add.l d4, d5
pub fn i_05c4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4E8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4E8: move.w (a6)+, d7
pub fn i_05c4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4EA;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4EA: move.l d7, (a4)
pub fn i_05c4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4EC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4EC: add.l d4, d7
pub fn i_05c4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4EE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4EE: move.l d5, (a4)
pub fn i_05c4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4F0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4F0: add.l d4, d5
pub fn i_05c4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4F2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4F2: move.w (a6)+, d7
pub fn i_05c4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4F4;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4F4: move.l d7, (a4)
pub fn i_05c4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4F6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4F6: add.l d4, d7
pub fn i_05c4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4F8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C4F8: move.l d5, (a4)
pub fn i_05c4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4FA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C4FA: add.l d4, d5
pub fn i_05c4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4FC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C4FC: move.w (a6)+, d7
pub fn i_05c4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C4FE;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C4FE: move.l d7, (a4)
pub fn i_05c4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C500;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C500: add.l d4, d7
pub fn i_05c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C502;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C502: move.l d5, (a4)
pub fn i_05c502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C504;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C504: add.l d4, d5
pub fn i_05c504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C506;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C506: move.w (a6)+, d7
pub fn i_05c506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C508;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C508: move.l d7, (a4)
pub fn i_05c508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C50A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C50A: add.l d4, d7
pub fn i_05c50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C50C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C50C: move.l d5, (a4)
pub fn i_05c50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C50E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C50E: add.l d4, d5
pub fn i_05c50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C510;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C510: move.w (a6)+, d7
pub fn i_05c510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C512;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C512: move.l d7, (a4)
pub fn i_05c512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C514;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C514: add.l d4, d7
pub fn i_05c514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C516;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C516: move.l d5, (a4)
pub fn i_05c516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C518;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C518: add.l d4, d5
pub fn i_05c518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C51A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C51A: move.w (a6)+, d7
pub fn i_05c51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C51C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C51C: move.l d7, (a4)
pub fn i_05c51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C51E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C51E: add.l d4, d7
pub fn i_05c51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C520;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C520: move.l d5, (a4)
pub fn i_05c520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C522;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C522: add.l d4, d5
pub fn i_05c522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C524;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C524: move.w (a6)+, d7
pub fn i_05c524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C526;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C526: move.l d7, (a4)
pub fn i_05c526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C528;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C528: add.l d4, d7
pub fn i_05c528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C52A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C52A: move.l d5, (a4)
pub fn i_05c52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C52C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C52C: add.l d4, d5
pub fn i_05c52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C52E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C52E: move.w (a6)+, d7
pub fn i_05c52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C530;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C530: move.l d7, (a4)
pub fn i_05c530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C532;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C532: add.l d4, d7
pub fn i_05c532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C534;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C534: move.l d5, (a4)
pub fn i_05c534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C536;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C536: add.l d4, d5
pub fn i_05c536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C538;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C538: move.w (a6)+, d7
pub fn i_05c538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C53A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C53A: move.l d7, (a4)
pub fn i_05c53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C53C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C53C: add.l d4, d7
pub fn i_05c53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C53E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C53E: move.l d5, (a4)
pub fn i_05c53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C540;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C540: add.l d4, d5
pub fn i_05c540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C542;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C542: move.w (a6)+, d7
pub fn i_05c542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C544;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C544: move.l d7, (a4)
pub fn i_05c544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C546;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C546: add.l d4, d7
pub fn i_05c546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C548;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C548: move.l d5, (a4)
pub fn i_05c548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C54A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C54A: add.l d4, d5
pub fn i_05c54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C54C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C54C: move.w (a6)+, d7
pub fn i_05c54c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C54E;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C54E: move.l d7, (a4)
pub fn i_05c54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C550;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C550: add.l d4, d7
pub fn i_05c550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C552;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C552: move.l d5, (a4)
pub fn i_05c552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C554;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C554: add.l d4, d5
pub fn i_05c554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C556;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C556: move.w (a6)+, d7
pub fn i_05c556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C558;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C558: move.l d7, (a4)
pub fn i_05c558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C55A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C55A: add.l d4, d7
pub fn i_05c55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C55C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C55C: move.l d5, (a4)
pub fn i_05c55c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C55E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C55E: add.l d4, d5
pub fn i_05c55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C560;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C560: move.w (a6)+, d7
pub fn i_05c560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C562;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C562: move.l d7, (a4)
pub fn i_05c562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C564;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C564: add.l d4, d7
pub fn i_05c564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C566;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C566: move.l d5, (a4)
pub fn i_05c566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C568;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C568: add.l d4, d5
pub fn i_05c568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C56A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C56A: move.w (a6)+, d7
pub fn i_05c56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C56C;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C56C: move.l d7, (a4)
pub fn i_05c56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C56E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C56E: add.l d4, d7
pub fn i_05c56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C570;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C570: move.l d5, (a4)
pub fn i_05c570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C572;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C572: add.l d4, d5
pub fn i_05c572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C574;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C574: move.w (a6)+, d7
pub fn i_05c574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C576;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C576: move.l d7, (a4)
pub fn i_05c576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C578;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C578: add.l d4, d7
pub fn i_05c578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C57A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C57A: move.l d5, (a4)
pub fn i_05c57a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C57C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C57C: add.l d4, d5
pub fn i_05c57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C57E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C57E: move.w (a6)+, d7
pub fn i_05c57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C580;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C580: move.l d7, (a4)
pub fn i_05c580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C582;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C582: add.l d4, d7
pub fn i_05c582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C584;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C584: move.l d5, (a4)
pub fn i_05c584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C586;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C586: add.l d4, d5
pub fn i_05c586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C588;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C588: move.w (a6)+, d7
pub fn i_05c588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C58A;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C58A: move.l d7, (a4)
pub fn i_05c58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C58C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C58C: add.l d4, d7
pub fn i_05c58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C58E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C58E: move.l d5, (a4)
pub fn i_05c58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C590;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C590: add.l d4, d5
pub fn i_05c590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C592;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C592: move.w (a6)+, d7
pub fn i_05c592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C594;
    cpu.ir = 0x3E1E;
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05C594: move.l d7, (a4)
pub fn i_05c594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C596;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C596: add.l d4, d7
pub fn i_05c596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C598;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C598: move.l d5, (a4)
pub fn i_05c598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C59A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C59A: add.l d4, d5
pub fn i_05c59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C59C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C59C: clr.w d7
pub fn i_05c59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C59E;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05C59E: moveq #$1, d4
pub fn i_05c59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5A0;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C5A0: swap d4
pub fn i_05c5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5A2;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C5A2: add.w d6, d6
pub fn i_05c5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5A4;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C5A4: jmp $5c5a8(pc, d6.w)
pub fn i_05c5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5A8;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5C5A8_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05C5A8: move.l d7, (a4)
pub fn i_05c5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5AA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5AA: add.l d4, d7
pub fn i_05c5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5AC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5AC: move.l d7, (a4)
pub fn i_05c5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5AE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5AE: add.l d4, d7
pub fn i_05c5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5B0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5B0: move.l d7, (a4)
pub fn i_05c5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5B2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5B2: add.l d4, d7
pub fn i_05c5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5B4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5B4: move.l d7, (a4)
pub fn i_05c5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5B6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5B6: add.l d4, d7
pub fn i_05c5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5B8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5B8: move.l d7, (a4)
pub fn i_05c5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5BA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5BA: add.l d4, d7
pub fn i_05c5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5BC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5BC: move.l d7, (a4)
pub fn i_05c5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5BE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5BE: add.l d4, d7
pub fn i_05c5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5C0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5C0: move.l d7, (a4)
pub fn i_05c5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5C2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5C2: add.l d4, d7
pub fn i_05c5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5C4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5C4: move.l d7, (a4)
pub fn i_05c5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5C6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5C6: add.l d4, d7
pub fn i_05c5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5C8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5C8: move.l d7, (a4)
pub fn i_05c5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5CA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5CA: add.l d4, d7
pub fn i_05c5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5CC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5CC: move.l d7, (a4)
pub fn i_05c5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5CE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5CE: add.l d4, d7
pub fn i_05c5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5D0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5D0: move.l d7, (a4)
pub fn i_05c5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5D2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5D2: add.l d4, d7
pub fn i_05c5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5D4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5D4: move.l d7, (a4)
pub fn i_05c5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5D6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5D6: add.l d4, d7
pub fn i_05c5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5D8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5D8: move.l d7, (a4)
pub fn i_05c5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5DA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5DA: add.l d4, d7
pub fn i_05c5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5DC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5DC: move.l d7, (a4)
pub fn i_05c5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5DE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5DE: add.l d4, d7
pub fn i_05c5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5E0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5E0: move.l d7, (a4)
pub fn i_05c5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5E2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5E2: add.l d4, d7
pub fn i_05c5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5E4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5E4: move.l d7, (a4)
pub fn i_05c5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5E6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5E6: add.l d4, d7
pub fn i_05c5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5E8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5E8: move.l d7, (a4)
pub fn i_05c5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5EA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5EA: add.l d4, d7
pub fn i_05c5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5EC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5EC: move.l d7, (a4)
pub fn i_05c5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5EE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5EE: add.l d4, d7
pub fn i_05c5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5F0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5F0: move.l d7, (a4)
pub fn i_05c5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5F2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5F2: add.l d4, d7
pub fn i_05c5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5F4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5F4: move.l d7, (a4)
pub fn i_05c5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5F6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5F6: add.l d4, d7
pub fn i_05c5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5F8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5F8: move.l d7, (a4)
pub fn i_05c5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5FA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5FA: add.l d4, d7
pub fn i_05c5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5FC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C5FC: move.l d7, (a4)
pub fn i_05c5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C5FE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C5FE: add.l d4, d7
pub fn i_05c5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C600;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C600: move.l d7, (a4)
pub fn i_05c600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C602;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C602: add.l d4, d7
pub fn i_05c602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C604;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C604: move.l d7, (a4)
pub fn i_05c604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C606;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C606: add.l d4, d7
pub fn i_05c606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C608;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C608: move.l d7, (a4)
pub fn i_05c608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C60A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C60A: add.l d4, d7
pub fn i_05c60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C60C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C60C: move.l d7, (a4)
pub fn i_05c60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C60E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C60E: add.l d4, d7
pub fn i_05c60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C610;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C610: move.l d7, (a4)
pub fn i_05c610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C612;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C612: add.l d4, d7
pub fn i_05c612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C614;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C614: move.l d7, (a4)
pub fn i_05c614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C616;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C616: add.l d4, d7
pub fn i_05c616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C618;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C618: move.l d7, (a4)
pub fn i_05c618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C61A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C61A: add.l d4, d7
pub fn i_05c61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C61C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C61C: move.l d7, (a4)
pub fn i_05c61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C61E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C61E: add.l d4, d7
pub fn i_05c61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C620;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C620: move.l d7, (a4)
pub fn i_05c620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C622;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C622: add.l d4, d7
pub fn i_05c622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C624;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C624: move.l d7, (a4)
pub fn i_05c624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C626;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C626: add.l d4, d7
pub fn i_05c626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C628;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C628: move.l d7, (a4)
pub fn i_05c628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C62A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C62A: add.l d4, d7
pub fn i_05c62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C62C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C62C: move.l d7, (a4)
pub fn i_05c62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C62E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C62E: add.l d4, d7
pub fn i_05c62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C630;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C630: move.l d7, (a4)
pub fn i_05c630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C632;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C632: add.l d4, d7
pub fn i_05c632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C634;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C634: move.l d7, (a4)
pub fn i_05c634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C636;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C636: add.l d4, d7
pub fn i_05c636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C638;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C638: move.l d7, (a4)
pub fn i_05c638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C63A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C63A: add.l d4, d7
pub fn i_05c63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C63C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C63C: move.l d7, (a4)
pub fn i_05c63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C63E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C63E: add.l d4, d7
pub fn i_05c63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C640;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C640: move.l d7, (a4)
pub fn i_05c640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C642;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C642: add.l d4, d7
pub fn i_05c642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C644;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C644: move.l d7, (a4)
pub fn i_05c644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C646;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C646: add.l d4, d7
pub fn i_05c646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C648;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C648: move.l d7, (a4)
pub fn i_05c648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C64A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C64A: add.l d4, d7
pub fn i_05c64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C64C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C64C: move.l d7, (a4)
pub fn i_05c64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C64E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C64E: add.l d4, d7
pub fn i_05c64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C650;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C650: move.l d7, (a4)
pub fn i_05c650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C652;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C652: add.l d4, d7
pub fn i_05c652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C654;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C654: move.l d7, (a4)
pub fn i_05c654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C656;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C656: add.l d4, d7
pub fn i_05c656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C658;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C658: move.l d7, (a4)
pub fn i_05c658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C65A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C65A: add.l d4, d7
pub fn i_05c65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C65C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C65C: move.l d7, (a4)
pub fn i_05c65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C65E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C65E: add.l d4, d7
pub fn i_05c65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C660;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C660: move.l d7, (a4)
pub fn i_05c660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C662;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C662: add.l d4, d7
pub fn i_05c662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C664;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C664: move.l d7, (a4)
pub fn i_05c664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C666;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C666: add.l d4, d7
pub fn i_05c666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C668;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C668: move.l d7, (a4)
pub fn i_05c668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C66A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C66A: add.l d4, d7
pub fn i_05c66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C66C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C66C: move.l d7, (a4)
pub fn i_05c66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C66E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C66E: add.l d4, d7
pub fn i_05c66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C670;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C670: move.l d7, (a4)
pub fn i_05c670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C672;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C672: add.l d4, d7
pub fn i_05c672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C674;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C674: move.l d7, (a4)
pub fn i_05c674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C676;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C676: add.l d4, d7
pub fn i_05c676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C678;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C678: move.l d7, (a4)
pub fn i_05c678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C67A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C67A: add.l d4, d7
pub fn i_05c67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C67C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C67C: move.l d7, (a4)
pub fn i_05c67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C67E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C67E: add.l d4, d7
pub fn i_05c67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C680;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C680: move.l d7, (a4)
pub fn i_05c680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C682;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C682: add.l d4, d7
pub fn i_05c682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C684;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C684: move.l d7, (a4)
pub fn i_05c684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C686;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C686: add.l d4, d7
pub fn i_05c686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C688;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C688: move.l d7, (a4)
pub fn i_05c688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C68A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C68A: add.l d4, d7
pub fn i_05c68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C68C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C68C: move.l d7, (a4)
pub fn i_05c68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C68E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C68E: add.l d4, d7
pub fn i_05c68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C690;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C690: move.l d7, (a4)
pub fn i_05c690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C692;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C692: add.l d4, d7
pub fn i_05c692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C694;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C694: move.l d7, (a4)
pub fn i_05c694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C696;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C696: add.l d4, d7
pub fn i_05c696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C698;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C698: move.l d7, (a4)
pub fn i_05c698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C69A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C69A: add.l d4, d7
pub fn i_05c69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C69C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C69C: move.l d7, (a4)
pub fn i_05c69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C69E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C69E: add.l d4, d7
pub fn i_05c69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6A0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x05B934, i_05b934),
    (0x05B936, i_05b936),
    (0x05B938, i_05b938),
    (0x05B93A, i_05b93a),
    (0x05B93C, i_05b93c),
    (0x05B93E, i_05b93e),
    (0x05B940, i_05b940),
    (0x05B942, i_05b942),
    (0x05B946, i_05b946),
    (0x05B948, i_05b948),
    (0x05B94A, i_05b94a),
    (0x05B94E, i_05b94e),
    (0x05B954, i_05b954),
    (0x05B956, i_05b956),
    (0x05B958, i_05b958),
    (0x05B95A, i_05b95a),
    (0x05B95E, i_05b95e),
    (0x05B960, i_05b960),
    (0x05B962, i_05b962),
    (0x05B966, i_05b966),
    (0x05B968, i_05b968),
    (0x05B96A, i_05b96a),
    (0x05B96C, i_05b96c),
    (0x05B96E, i_05b96e),
    (0x05B972, i_05b972),
    (0x05B974, i_05b974),
    (0x05B976, i_05b976),
    (0x05B978, i_05b978),
    (0x05B97A, i_05b97a),
    (0x05B97E, i_05b97e),
    (0x05B980, i_05b980),
    (0x05B984, i_05b984),
    (0x05B986, i_05b986),
    (0x05B988, i_05b988),
    (0x05B98C, i_05b98c),
    (0x05B990, i_05b990),
    (0x05B994, i_05b994),
    (0x05B998, i_05b998),
    (0x05B99C, i_05b99c),
    (0x05B9A0, i_05b9a0),
    (0x05B9A4, i_05b9a4),
    (0x05B9A8, i_05b9a8),
    (0x05B9AC, i_05b9ac),
    (0x05B9B0, i_05b9b0),
    (0x05B9B4, i_05b9b4),
    (0x05B9B8, i_05b9b8),
    (0x05B9BC, i_05b9bc),
    (0x05B9C0, i_05b9c0),
    (0x05B9C4, i_05b9c4),
    (0x05B9C8, i_05b9c8),
    (0x05B9CC, i_05b9cc),
    (0x05B9D0, i_05b9d0),
    (0x05B9D4, i_05b9d4),
    (0x05B9D8, i_05b9d8),
    (0x05B9DC, i_05b9dc),
    (0x05B9E0, i_05b9e0),
    (0x05B9E4, i_05b9e4),
    (0x05B9E8, i_05b9e8),
    (0x05B9EC, i_05b9ec),
    (0x05B9F0, i_05b9f0),
    (0x05B9F4, i_05b9f4),
    (0x05B9F8, i_05b9f8),
    (0x05B9FC, i_05b9fc),
    (0x05BA00, i_05ba00),
    (0x05BA04, i_05ba04),
    (0x05BA08, i_05ba08),
    (0x05BA0A, i_05ba0a),
    (0x05BA0C, i_05ba0c),
    (0x05BA0E, i_05ba0e),
    (0x05BA10, i_05ba10),
    (0x05BA12, i_05ba12),
    (0x05BA14, i_05ba14),
    (0x05BA16, i_05ba16),
    (0x05BA18, i_05ba18),
    (0x05BA1A, i_05ba1a),
    (0x05BA1C, i_05ba1c),
    (0x05BA1E, i_05ba1e),
    (0x05BA20, i_05ba20),
    (0x05BA22, i_05ba22),
    (0x05BA24, i_05ba24),
    (0x05BA26, i_05ba26),
    (0x05BA28, i_05ba28),
    (0x05BA2A, i_05ba2a),
    (0x05BA2C, i_05ba2c),
    (0x05BA2E, i_05ba2e),
    (0x05BA30, i_05ba30),
    (0x05BA32, i_05ba32),
    (0x05BA34, i_05ba34),
    (0x05BA36, i_05ba36),
    (0x05BA38, i_05ba38),
    (0x05BA3A, i_05ba3a),
    (0x05BA3C, i_05ba3c),
    (0x05BA3E, i_05ba3e),
    (0x05BA40, i_05ba40),
    (0x05BA42, i_05ba42),
    (0x05BA44, i_05ba44),
    (0x05BA46, i_05ba46),
    (0x05BA48, i_05ba48),
    (0x05BA4A, i_05ba4a),
    (0x05BA4C, i_05ba4c),
    (0x05BA4E, i_05ba4e),
    (0x05BA50, i_05ba50),
    (0x05BA52, i_05ba52),
    (0x05BA54, i_05ba54),
    (0x05BA56, i_05ba56),
    (0x05BA58, i_05ba58),
    (0x05BA5A, i_05ba5a),
    (0x05BA5C, i_05ba5c),
    (0x05BA5E, i_05ba5e),
    (0x05BA60, i_05ba60),
    (0x05BA62, i_05ba62),
    (0x05BA64, i_05ba64),
    (0x05BA66, i_05ba66),
    (0x05BA68, i_05ba68),
    (0x05BA6A, i_05ba6a),
    (0x05BA6C, i_05ba6c),
    (0x05BA6E, i_05ba6e),
    (0x05BA70, i_05ba70),
    (0x05BA72, i_05ba72),
    (0x05BA74, i_05ba74),
    (0x05BA76, i_05ba76),
    (0x05BA78, i_05ba78),
    (0x05BA7A, i_05ba7a),
    (0x05BA7C, i_05ba7c),
    (0x05BA7E, i_05ba7e),
    (0x05BA80, i_05ba80),
    (0x05BA82, i_05ba82),
    (0x05BA84, i_05ba84),
    (0x05BA86, i_05ba86),
    (0x05BA88, i_05ba88),
    (0x05BA8A, i_05ba8a),
    (0x05BA8C, i_05ba8c),
    (0x05BA8E, i_05ba8e),
    (0x05BA90, i_05ba90),
    (0x05BA92, i_05ba92),
    (0x05BA94, i_05ba94),
    (0x05BA96, i_05ba96),
    (0x05BA98, i_05ba98),
    (0x05BA9A, i_05ba9a),
    (0x05BA9C, i_05ba9c),
    (0x05BA9E, i_05ba9e),
    (0x05BAA0, i_05baa0),
    (0x05BAA2, i_05baa2),
    (0x05BAA4, i_05baa4),
    (0x05BAA6, i_05baa6),
    (0x05BAA8, i_05baa8),
    (0x05BAAA, i_05baaa),
    (0x05BAAC, i_05baac),
    (0x05BAAE, i_05baae),
    (0x05BAB0, i_05bab0),
    (0x05BAB2, i_05bab2),
    (0x05BAB4, i_05bab4),
    (0x05BAB6, i_05bab6),
    (0x05BAB8, i_05bab8),
    (0x05BABA, i_05baba),
    (0x05BABC, i_05babc),
    (0x05BABE, i_05babe),
    (0x05BAC0, i_05bac0),
    (0x05BAC2, i_05bac2),
    (0x05BAC4, i_05bac4),
    (0x05BAC6, i_05bac6),
    (0x05BAC8, i_05bac8),
    (0x05BACA, i_05baca),
    (0x05BACC, i_05bacc),
    (0x05BACE, i_05bace),
    (0x05BAD0, i_05bad0),
    (0x05BAD2, i_05bad2),
    (0x05BAD4, i_05bad4),
    (0x05BAD6, i_05bad6),
    (0x05BAD8, i_05bad8),
    (0x05BADA, i_05bada),
    (0x05BADC, i_05badc),
    (0x05BADE, i_05bade),
    (0x05BAE0, i_05bae0),
    (0x05BAE2, i_05bae2),
    (0x05BAE4, i_05bae4),
    (0x05BAE6, i_05bae6),
    (0x05BAE8, i_05bae8),
    (0x05BAEA, i_05baea),
    (0x05BAEC, i_05baec),
    (0x05BAEE, i_05baee),
    (0x05BAF0, i_05baf0),
    (0x05BAF2, i_05baf2),
    (0x05BAF4, i_05baf4),
    (0x05BAF6, i_05baf6),
    (0x05BAF8, i_05baf8),
    (0x05BAFA, i_05bafa),
    (0x05BAFC, i_05bafc),
    (0x05BAFE, i_05bafe),
    (0x05BB00, i_05bb00),
    (0x05BB02, i_05bb02),
    (0x05BB04, i_05bb04),
    (0x05BB06, i_05bb06),
    (0x05BB08, i_05bb08),
    (0x05BB0A, i_05bb0a),
    (0x05BB0C, i_05bb0c),
    (0x05BB0E, i_05bb0e),
    (0x05BB10, i_05bb10),
    (0x05BB12, i_05bb12),
    (0x05BB14, i_05bb14),
    (0x05BB16, i_05bb16),
    (0x05BB18, i_05bb18),
    (0x05BB1A, i_05bb1a),
    (0x05BB1C, i_05bb1c),
    (0x05BB1E, i_05bb1e),
    (0x05BB20, i_05bb20),
    (0x05BB22, i_05bb22),
    (0x05BB24, i_05bb24),
    (0x05BB26, i_05bb26),
    (0x05BB28, i_05bb28),
    (0x05BB2A, i_05bb2a),
    (0x05BB2C, i_05bb2c),
    (0x05BB2E, i_05bb2e),
    (0x05BB30, i_05bb30),
    (0x05BB32, i_05bb32),
    (0x05BB34, i_05bb34),
    (0x05BB36, i_05bb36),
    (0x05BB38, i_05bb38),
    (0x05BB3A, i_05bb3a),
    (0x05BB3C, i_05bb3c),
    (0x05BB3E, i_05bb3e),
    (0x05BB40, i_05bb40),
    (0x05BB42, i_05bb42),
    (0x05BB44, i_05bb44),
    (0x05BB46, i_05bb46),
    (0x05BB48, i_05bb48),
    (0x05BB4A, i_05bb4a),
    (0x05BB4C, i_05bb4c),
    (0x05BB4E, i_05bb4e),
    (0x05BB50, i_05bb50),
    (0x05BB54, i_05bb54),
    (0x05BB56, i_05bb56),
    (0x05BB58, i_05bb58),
    (0x05BB5A, i_05bb5a),
    (0x05BB5C, i_05bb5c),
    (0x05BB5E, i_05bb5e),
    (0x05BB60, i_05bb60),
    (0x05BB62, i_05bb62),
    (0x05BB64, i_05bb64),
    (0x05BB66, i_05bb66),
    (0x05BB68, i_05bb68),
    (0x05BB6A, i_05bb6a),
    (0x05BB6C, i_05bb6c),
    (0x05BB6E, i_05bb6e),
    (0x05BB70, i_05bb70),
    (0x05BB72, i_05bb72),
    (0x05BB74, i_05bb74),
    (0x05BB76, i_05bb76),
    (0x05BB78, i_05bb78),
    (0x05BB7A, i_05bb7a),
    (0x05BB7C, i_05bb7c),
    (0x05BB7E, i_05bb7e),
    (0x05BB80, i_05bb80),
    (0x05BB82, i_05bb82),
    (0x05BB84, i_05bb84),
    (0x05BB86, i_05bb86),
    (0x05BB88, i_05bb88),
    (0x05BB8A, i_05bb8a),
    (0x05BB8C, i_05bb8c),
    (0x05BB8E, i_05bb8e),
    (0x05BB90, i_05bb90),
    (0x05BB92, i_05bb92),
    (0x05BB94, i_05bb94),
    (0x05BB96, i_05bb96),
    (0x05BB98, i_05bb98),
    (0x05BB9A, i_05bb9a),
    (0x05BB9C, i_05bb9c),
    (0x05BB9E, i_05bb9e),
    (0x05BBA0, i_05bba0),
    (0x05BBA2, i_05bba2),
    (0x05BBA4, i_05bba4),
    (0x05BBA6, i_05bba6),
    (0x05BBA8, i_05bba8),
    (0x05BBAA, i_05bbaa),
    (0x05BBAC, i_05bbac),
    (0x05BBAE, i_05bbae),
    (0x05BBB0, i_05bbb0),
    (0x05BBB2, i_05bbb2),
    (0x05BBB4, i_05bbb4),
    (0x05BBB6, i_05bbb6),
    (0x05BBB8, i_05bbb8),
    (0x05BBBA, i_05bbba),
    (0x05BBBC, i_05bbbc),
    (0x05BBBE, i_05bbbe),
    (0x05BBC0, i_05bbc0),
    (0x05BBC2, i_05bbc2),
    (0x05BBC4, i_05bbc4),
    (0x05BBC6, i_05bbc6),
    (0x05BBC8, i_05bbc8),
    (0x05BBCA, i_05bbca),
    (0x05BBCC, i_05bbcc),
    (0x05BBCE, i_05bbce),
    (0x05BBD0, i_05bbd0),
    (0x05BBD2, i_05bbd2),
    (0x05BBD4, i_05bbd4),
    (0x05BBD6, i_05bbd6),
    (0x05BBD8, i_05bbd8),
    (0x05BBDA, i_05bbda),
    (0x05BBDC, i_05bbdc),
    (0x05BBDE, i_05bbde),
    (0x05BBE0, i_05bbe0),
    (0x05BBE2, i_05bbe2),
    (0x05BBE4, i_05bbe4),
    (0x05BBE6, i_05bbe6),
    (0x05BBE8, i_05bbe8),
    (0x05BBEA, i_05bbea),
    (0x05BBEC, i_05bbec),
    (0x05BBEE, i_05bbee),
    (0x05BBF0, i_05bbf0),
    (0x05BBF2, i_05bbf2),
    (0x05BBF4, i_05bbf4),
    (0x05BBF6, i_05bbf6),
    (0x05BBF8, i_05bbf8),
    (0x05BBFA, i_05bbfa),
    (0x05BBFC, i_05bbfc),
    (0x05BBFE, i_05bbfe),
    (0x05BC00, i_05bc00),
    (0x05BC02, i_05bc02),
    (0x05BC04, i_05bc04),
    (0x05BC06, i_05bc06),
    (0x05BC08, i_05bc08),
    (0x05BC0A, i_05bc0a),
    (0x05BC0C, i_05bc0c),
    (0x05BC0E, i_05bc0e),
    (0x05BC10, i_05bc10),
    (0x05BC12, i_05bc12),
    (0x05BC14, i_05bc14),
    (0x05BC16, i_05bc16),
    (0x05BC18, i_05bc18),
    (0x05BC1A, i_05bc1a),
    (0x05BC1C, i_05bc1c),
    (0x05BC1E, i_05bc1e),
    (0x05BC20, i_05bc20),
    (0x05BC22, i_05bc22),
    (0x05BC24, i_05bc24),
    (0x05BC26, i_05bc26),
    (0x05BC28, i_05bc28),
    (0x05BC2A, i_05bc2a),
    (0x05BC2C, i_05bc2c),
    (0x05BC2E, i_05bc2e),
    (0x05BC30, i_05bc30),
    (0x05BC32, i_05bc32),
    (0x05BC34, i_05bc34),
    (0x05BC36, i_05bc36),
    (0x05BC38, i_05bc38),
    (0x05BC3A, i_05bc3a),
    (0x05BC3C, i_05bc3c),
    (0x05BC3E, i_05bc3e),
    (0x05BC40, i_05bc40),
    (0x05BC42, i_05bc42),
    (0x05BC44, i_05bc44),
    (0x05BC46, i_05bc46),
    (0x05BC48, i_05bc48),
    (0x05BC4A, i_05bc4a),
    (0x05BC4C, i_05bc4c),
    (0x05BC50, i_05bc50),
    (0x05BC52, i_05bc52),
    (0x05BC54, i_05bc54),
    (0x05BC58, i_05bc58),
    (0x05BC5E, i_05bc5e),
    (0x05BC60, i_05bc60),
    (0x05BC62, i_05bc62),
    (0x05BC64, i_05bc64),
    (0x05BC68, i_05bc68),
    (0x05BC6A, i_05bc6a),
    (0x05BC6C, i_05bc6c),
    (0x05BC70, i_05bc70),
    (0x05BC72, i_05bc72),
    (0x05BC74, i_05bc74),
    (0x05BC76, i_05bc76),
    (0x05BC78, i_05bc78),
    (0x05BC7C, i_05bc7c),
    (0x05BC7E, i_05bc7e),
    (0x05BC80, i_05bc80),
    (0x05BC82, i_05bc82),
    (0x05BC84, i_05bc84),
    (0x05BC88, i_05bc88),
    (0x05BC8A, i_05bc8a),
    (0x05BC8E, i_05bc8e),
    (0x05BC90, i_05bc90),
    (0x05BC92, i_05bc92),
    (0x05BC96, i_05bc96),
    (0x05BC9A, i_05bc9a),
    (0x05BC9E, i_05bc9e),
    (0x05BCA2, i_05bca2),
    (0x05BCA6, i_05bca6),
    (0x05BCAA, i_05bcaa),
    (0x05BCAE, i_05bcae),
    (0x05BCB2, i_05bcb2),
    (0x05BCB6, i_05bcb6),
    (0x05BCBA, i_05bcba),
    (0x05BCBE, i_05bcbe),
    (0x05BCC2, i_05bcc2),
    (0x05BCC6, i_05bcc6),
    (0x05BCCA, i_05bcca),
    (0x05BCCE, i_05bcce),
    (0x05BCD2, i_05bcd2),
    (0x05BCD6, i_05bcd6),
    (0x05BCDA, i_05bcda),
    (0x05BCDE, i_05bcde),
    (0x05BCE2, i_05bce2),
    (0x05BCE6, i_05bce6),
    (0x05BCEA, i_05bcea),
    (0x05BCEE, i_05bcee),
    (0x05BCF2, i_05bcf2),
    (0x05BCF6, i_05bcf6),
    (0x05BCFA, i_05bcfa),
    (0x05BCFE, i_05bcfe),
    (0x05BD02, i_05bd02),
    (0x05BD06, i_05bd06),
    (0x05BD0A, i_05bd0a),
    (0x05BD0E, i_05bd0e),
    (0x05BD12, i_05bd12),
    (0x05BD14, i_05bd14),
    (0x05BD16, i_05bd16),
    (0x05BD18, i_05bd18),
    (0x05BD1A, i_05bd1a),
    (0x05BD1C, i_05bd1c),
    (0x05BD1E, i_05bd1e),
    (0x05BD20, i_05bd20),
    (0x05BD22, i_05bd22),
    (0x05BD24, i_05bd24),
    (0x05BD26, i_05bd26),
    (0x05BD28, i_05bd28),
    (0x05BD2A, i_05bd2a),
    (0x05BD2C, i_05bd2c),
    (0x05BD2E, i_05bd2e),
    (0x05BD30, i_05bd30),
    (0x05BD32, i_05bd32),
    (0x05BD34, i_05bd34),
    (0x05BD36, i_05bd36),
    (0x05BD38, i_05bd38),
    (0x05BD3A, i_05bd3a),
    (0x05BD3C, i_05bd3c),
    (0x05BD3E, i_05bd3e),
    (0x05BD40, i_05bd40),
    (0x05BD42, i_05bd42),
    (0x05BD44, i_05bd44),
    (0x05BD46, i_05bd46),
    (0x05BD48, i_05bd48),
    (0x05BD4A, i_05bd4a),
    (0x05BD4C, i_05bd4c),
    (0x05BD4E, i_05bd4e),
    (0x05BD50, i_05bd50),
    (0x05BD52, i_05bd52),
    (0x05BD54, i_05bd54),
    (0x05BD56, i_05bd56),
    (0x05BD58, i_05bd58),
    (0x05BD5A, i_05bd5a),
    (0x05BD5C, i_05bd5c),
    (0x05BD5E, i_05bd5e),
    (0x05BD60, i_05bd60),
    (0x05BD62, i_05bd62),
    (0x05BD64, i_05bd64),
    (0x05BD66, i_05bd66),
    (0x05BD68, i_05bd68),
    (0x05BD6A, i_05bd6a),
    (0x05BD6C, i_05bd6c),
    (0x05BD6E, i_05bd6e),
    (0x05BD70, i_05bd70),
    (0x05BD72, i_05bd72),
    (0x05BD74, i_05bd74),
    (0x05BD76, i_05bd76),
    (0x05BD78, i_05bd78),
    (0x05BD7A, i_05bd7a),
    (0x05BD7C, i_05bd7c),
    (0x05BD7E, i_05bd7e),
    (0x05BD80, i_05bd80),
    (0x05BD82, i_05bd82),
    (0x05BD84, i_05bd84),
    (0x05BD86, i_05bd86),
    (0x05BD88, i_05bd88),
    (0x05BD8A, i_05bd8a),
    (0x05BD8C, i_05bd8c),
    (0x05BD8E, i_05bd8e),
    (0x05BD90, i_05bd90),
    (0x05BD92, i_05bd92),
    (0x05BD94, i_05bd94),
    (0x05BD96, i_05bd96),
    (0x05BD98, i_05bd98),
    (0x05BD9A, i_05bd9a),
    (0x05BD9C, i_05bd9c),
    (0x05BD9E, i_05bd9e),
    (0x05BDA0, i_05bda0),
    (0x05BDA2, i_05bda2),
    (0x05BDA4, i_05bda4),
    (0x05BDA6, i_05bda6),
    (0x05BDA8, i_05bda8),
    (0x05BDAA, i_05bdaa),
    (0x05BDAC, i_05bdac),
    (0x05BDAE, i_05bdae),
    (0x05BDB0, i_05bdb0),
    (0x05BDB2, i_05bdb2),
    (0x05BDB4, i_05bdb4),
    (0x05BDB6, i_05bdb6),
    (0x05BDB8, i_05bdb8),
    (0x05BDBA, i_05bdba),
    (0x05BDBC, i_05bdbc),
    (0x05BDBE, i_05bdbe),
    (0x05BDC0, i_05bdc0),
    (0x05BDC2, i_05bdc2),
    (0x05BDC4, i_05bdc4),
    (0x05BDC6, i_05bdc6),
    (0x05BDC8, i_05bdc8),
    (0x05BDCA, i_05bdca),
    (0x05BDCC, i_05bdcc),
    (0x05BDCE, i_05bdce),
    (0x05BDD0, i_05bdd0),
    (0x05BDD2, i_05bdd2),
    (0x05BDD4, i_05bdd4),
    (0x05BDD6, i_05bdd6),
    (0x05BDD8, i_05bdd8),
    (0x05BDDA, i_05bdda),
    (0x05BDDC, i_05bddc),
    (0x05BDDE, i_05bdde),
    (0x05BDE0, i_05bde0),
    (0x05BDE2, i_05bde2),
    (0x05BDE4, i_05bde4),
    (0x05BDE6, i_05bde6),
    (0x05BDE8, i_05bde8),
    (0x05BDEA, i_05bdea),
    (0x05BDEC, i_05bdec),
    (0x05BDEE, i_05bdee),
    (0x05BDF0, i_05bdf0),
    (0x05BDF2, i_05bdf2),
    (0x05BDF4, i_05bdf4),
    (0x05BDF6, i_05bdf6),
    (0x05BDF8, i_05bdf8),
    (0x05BDFA, i_05bdfa),
    (0x05BDFC, i_05bdfc),
    (0x05BDFE, i_05bdfe),
    (0x05BE00, i_05be00),
    (0x05BE02, i_05be02),
    (0x05BE04, i_05be04),
    (0x05BE06, i_05be06),
    (0x05BE08, i_05be08),
    (0x05BE0A, i_05be0a),
    (0x05BE0C, i_05be0c),
    (0x05BE0E, i_05be0e),
    (0x05BE10, i_05be10),
    (0x05BE12, i_05be12),
    (0x05BE14, i_05be14),
    (0x05BE16, i_05be16),
    (0x05BE18, i_05be18),
    (0x05BE1A, i_05be1a),
    (0x05BE1C, i_05be1c),
    (0x05BE1E, i_05be1e),
    (0x05BE20, i_05be20),
    (0x05BE22, i_05be22),
    (0x05BE24, i_05be24),
    (0x05BE26, i_05be26),
    (0x05BE28, i_05be28),
    (0x05BE2A, i_05be2a),
    (0x05BE2C, i_05be2c),
    (0x05BE2E, i_05be2e),
    (0x05BE30, i_05be30),
    (0x05BE32, i_05be32),
    (0x05BE34, i_05be34),
    (0x05BE36, i_05be36),
    (0x05BE38, i_05be38),
    (0x05BE3A, i_05be3a),
    (0x05BE3C, i_05be3c),
    (0x05BE3E, i_05be3e),
    (0x05BE40, i_05be40),
    (0x05BE42, i_05be42),
    (0x05BE44, i_05be44),
    (0x05BE46, i_05be46),
    (0x05BE48, i_05be48),
    (0x05BE4A, i_05be4a),
    (0x05BE4C, i_05be4c),
    (0x05BE4E, i_05be4e),
    (0x05BE50, i_05be50),
    (0x05BE52, i_05be52),
    (0x05BE54, i_05be54),
    (0x05BE56, i_05be56),
    (0x05BE58, i_05be58),
    (0x05BE5A, i_05be5a),
    (0x05BE5E, i_05be5e),
    (0x05BE60, i_05be60),
    (0x05BE62, i_05be62),
    (0x05BE64, i_05be64),
    (0x05BE66, i_05be66),
    (0x05BE68, i_05be68),
    (0x05BE6A, i_05be6a),
    (0x05BE6C, i_05be6c),
    (0x05BE6E, i_05be6e),
    (0x05BE70, i_05be70),
    (0x05BE72, i_05be72),
    (0x05BE74, i_05be74),
    (0x05BE76, i_05be76),
    (0x05BE78, i_05be78),
    (0x05BE7A, i_05be7a),
    (0x05BE7C, i_05be7c),
    (0x05BE7E, i_05be7e),
    (0x05BE80, i_05be80),
    (0x05BE82, i_05be82),
    (0x05BE84, i_05be84),
    (0x05BE86, i_05be86),
    (0x05BE88, i_05be88),
    (0x05BE8A, i_05be8a),
    (0x05BE8C, i_05be8c),
    (0x05BE8E, i_05be8e),
    (0x05BE90, i_05be90),
    (0x05BE92, i_05be92),
    (0x05BE94, i_05be94),
    (0x05BE96, i_05be96),
    (0x05BE98, i_05be98),
    (0x05BE9A, i_05be9a),
    (0x05BE9C, i_05be9c),
    (0x05BE9E, i_05be9e),
    (0x05BEA0, i_05bea0),
    (0x05BEA2, i_05bea2),
    (0x05BEA4, i_05bea4),
    (0x05BEA6, i_05bea6),
    (0x05BEA8, i_05bea8),
    (0x05BEAA, i_05beaa),
    (0x05BEAC, i_05beac),
    (0x05BEAE, i_05beae),
    (0x05BEB0, i_05beb0),
    (0x05BEB2, i_05beb2),
    (0x05BEB4, i_05beb4),
    (0x05BEB6, i_05beb6),
    (0x05BEB8, i_05beb8),
    (0x05BEBA, i_05beba),
    (0x05BEBC, i_05bebc),
    (0x05BEBE, i_05bebe),
    (0x05BEC0, i_05bec0),
    (0x05BEC2, i_05bec2),
    (0x05BEC4, i_05bec4),
    (0x05BEC6, i_05bec6),
    (0x05BEC8, i_05bec8),
    (0x05BECA, i_05beca),
    (0x05BECC, i_05becc),
    (0x05BECE, i_05bece),
    (0x05BED0, i_05bed0),
    (0x05BED2, i_05bed2),
    (0x05BED4, i_05bed4),
    (0x05BED6, i_05bed6),
    (0x05BED8, i_05bed8),
    (0x05BEDA, i_05beda),
    (0x05BEDC, i_05bedc),
    (0x05BEDE, i_05bede),
    (0x05BEE0, i_05bee0),
    (0x05BEE2, i_05bee2),
    (0x05BEE4, i_05bee4),
    (0x05BEE6, i_05bee6),
    (0x05BEE8, i_05bee8),
    (0x05BEEA, i_05beea),
    (0x05BEEC, i_05beec),
    (0x05BEEE, i_05beee),
    (0x05BEF0, i_05bef0),
    (0x05BEF2, i_05bef2),
    (0x05BEF4, i_05bef4),
    (0x05BEF6, i_05bef6),
    (0x05BEF8, i_05bef8),
    (0x05BEFA, i_05befa),
    (0x05BEFC, i_05befc),
    (0x05BEFE, i_05befe),
    (0x05BF00, i_05bf00),
    (0x05BF02, i_05bf02),
    (0x05BF04, i_05bf04),
    (0x05BF06, i_05bf06),
    (0x05BF08, i_05bf08),
    (0x05BF0A, i_05bf0a),
    (0x05BF0C, i_05bf0c),
    (0x05BF0E, i_05bf0e),
    (0x05BF10, i_05bf10),
    (0x05BF12, i_05bf12),
    (0x05BF14, i_05bf14),
    (0x05BF16, i_05bf16),
    (0x05BF18, i_05bf18),
    (0x05BF1A, i_05bf1a),
    (0x05BF1C, i_05bf1c),
    (0x05BF1E, i_05bf1e),
    (0x05BF20, i_05bf20),
    (0x05BF22, i_05bf22),
    (0x05BF24, i_05bf24),
    (0x05BF26, i_05bf26),
    (0x05BF28, i_05bf28),
    (0x05BF2A, i_05bf2a),
    (0x05BF2C, i_05bf2c),
    (0x05BF2E, i_05bf2e),
    (0x05BF30, i_05bf30),
    (0x05BF32, i_05bf32),
    (0x05BF34, i_05bf34),
    (0x05BF36, i_05bf36),
    (0x05BF38, i_05bf38),
    (0x05BF3A, i_05bf3a),
    (0x05BF3C, i_05bf3c),
    (0x05BF3E, i_05bf3e),
    (0x05BF40, i_05bf40),
    (0x05BF42, i_05bf42),
    (0x05BF44, i_05bf44),
    (0x05BF46, i_05bf46),
    (0x05BF48, i_05bf48),
    (0x05BF4A, i_05bf4a),
    (0x05BF4C, i_05bf4c),
    (0x05BF4E, i_05bf4e),
    (0x05BF50, i_05bf50),
    (0x05BF52, i_05bf52),
    (0x05BF54, i_05bf54),
    (0x05BF56, i_05bf56),
    (0x05BF5A, i_05bf5a),
    (0x05BF5C, i_05bf5c),
    (0x05BF5E, i_05bf5e),
    (0x05BF62, i_05bf62),
    (0x05BF68, i_05bf68),
    (0x05BF6A, i_05bf6a),
    (0x05BF6C, i_05bf6c),
    (0x05BF6E, i_05bf6e),
    (0x05BF72, i_05bf72),
    (0x05BF74, i_05bf74),
    (0x05BF76, i_05bf76),
    (0x05BF7A, i_05bf7a),
    (0x05BF7C, i_05bf7c),
    (0x05BF7E, i_05bf7e),
    (0x05BF82, i_05bf82),
    (0x05BF84, i_05bf84),
    (0x05BF88, i_05bf88),
    (0x05BF8A, i_05bf8a),
    (0x05BF8E, i_05bf8e),
    (0x05BF92, i_05bf92),
    (0x05BF96, i_05bf96),
    (0x05BF9A, i_05bf9a),
    (0x05BF9E, i_05bf9e),
    (0x05BFA2, i_05bfa2),
    (0x05BFA4, i_05bfa4),
    (0x05BFA6, i_05bfa6),
    (0x05BFA8, i_05bfa8),
    (0x05BFAA, i_05bfaa),
    (0x05BFAE, i_05bfae),
    (0x05BFB0, i_05bfb0),
    (0x05BFB2, i_05bfb2),
    (0x05BFB4, i_05bfb4),
    (0x05BFB6, i_05bfb6),
    (0x05BFBA, i_05bfba),
    (0x05BFBC, i_05bfbc),
    (0x05BFC0, i_05bfc0),
    (0x05BFC2, i_05bfc2),
    (0x05BFC4, i_05bfc4),
    (0x05BFC8, i_05bfc8),
    (0x05BFCC, i_05bfcc),
    (0x05BFD0, i_05bfd0),
    (0x05BFD4, i_05bfd4),
    (0x05BFD8, i_05bfd8),
    (0x05BFDC, i_05bfdc),
    (0x05BFE0, i_05bfe0),
    (0x05BFE4, i_05bfe4),
    (0x05BFE8, i_05bfe8),
    (0x05BFEC, i_05bfec),
    (0x05BFF0, i_05bff0),
    (0x05BFF4, i_05bff4),
    (0x05BFF8, i_05bff8),
    (0x05BFFC, i_05bffc),
    (0x05C000, i_05c000),
    (0x05C004, i_05c004),
    (0x05C008, i_05c008),
    (0x05C00C, i_05c00c),
    (0x05C010, i_05c010),
    (0x05C014, i_05c014),
    (0x05C018, i_05c018),
    (0x05C01C, i_05c01c),
    (0x05C020, i_05c020),
    (0x05C024, i_05c024),
    (0x05C028, i_05c028),
    (0x05C02C, i_05c02c),
    (0x05C030, i_05c030),
    (0x05C034, i_05c034),
    (0x05C038, i_05c038),
    (0x05C03C, i_05c03c),
    (0x05C040, i_05c040),
    (0x05C044, i_05c044),
    (0x05C046, i_05c046),
    (0x05C048, i_05c048),
    (0x05C04A, i_05c04a),
    (0x05C04C, i_05c04c),
    (0x05C04E, i_05c04e),
    (0x05C050, i_05c050),
    (0x05C052, i_05c052),
    (0x05C054, i_05c054),
    (0x05C056, i_05c056),
    (0x05C058, i_05c058),
    (0x05C05A, i_05c05a),
    (0x05C05C, i_05c05c),
    (0x05C05E, i_05c05e),
    (0x05C060, i_05c060),
    (0x05C062, i_05c062),
    (0x05C064, i_05c064),
    (0x05C066, i_05c066),
    (0x05C068, i_05c068),
    (0x05C06A, i_05c06a),
    (0x05C06C, i_05c06c),
    (0x05C06E, i_05c06e),
    (0x05C070, i_05c070),
    (0x05C072, i_05c072),
    (0x05C074, i_05c074),
    (0x05C076, i_05c076),
    (0x05C078, i_05c078),
    (0x05C07A, i_05c07a),
    (0x05C07C, i_05c07c),
    (0x05C07E, i_05c07e),
    (0x05C080, i_05c080),
    (0x05C082, i_05c082),
    (0x05C084, i_05c084),
    (0x05C086, i_05c086),
    (0x05C088, i_05c088),
    (0x05C08A, i_05c08a),
    (0x05C08C, i_05c08c),
    (0x05C08E, i_05c08e),
    (0x05C090, i_05c090),
    (0x05C092, i_05c092),
    (0x05C094, i_05c094),
    (0x05C096, i_05c096),
    (0x05C098, i_05c098),
    (0x05C09A, i_05c09a),
    (0x05C09C, i_05c09c),
    (0x05C09E, i_05c09e),
    (0x05C0A0, i_05c0a0),
    (0x05C0A2, i_05c0a2),
    (0x05C0A4, i_05c0a4),
    (0x05C0A6, i_05c0a6),
    (0x05C0A8, i_05c0a8),
    (0x05C0AA, i_05c0aa),
    (0x05C0AC, i_05c0ac),
    (0x05C0AE, i_05c0ae),
    (0x05C0B0, i_05c0b0),
    (0x05C0B2, i_05c0b2),
    (0x05C0B4, i_05c0b4),
    (0x05C0B6, i_05c0b6),
    (0x05C0B8, i_05c0b8),
    (0x05C0BA, i_05c0ba),
    (0x05C0BC, i_05c0bc),
    (0x05C0BE, i_05c0be),
    (0x05C0C0, i_05c0c0),
    (0x05C0C2, i_05c0c2),
    (0x05C0C4, i_05c0c4),
    (0x05C0C6, i_05c0c6),
    (0x05C0C8, i_05c0c8),
    (0x05C0CA, i_05c0ca),
    (0x05C0CC, i_05c0cc),
    (0x05C0CE, i_05c0ce),
    (0x05C0D0, i_05c0d0),
    (0x05C0D2, i_05c0d2),
    (0x05C0D4, i_05c0d4),
    (0x05C0D6, i_05c0d6),
    (0x05C0D8, i_05c0d8),
    (0x05C0DA, i_05c0da),
    (0x05C0DC, i_05c0dc),
    (0x05C0DE, i_05c0de),
    (0x05C0E0, i_05c0e0),
    (0x05C0E2, i_05c0e2),
    (0x05C0E4, i_05c0e4),
    (0x05C0E6, i_05c0e6),
    (0x05C0E8, i_05c0e8),
    (0x05C0EA, i_05c0ea),
    (0x05C0EC, i_05c0ec),
    (0x05C0EE, i_05c0ee),
    (0x05C0F0, i_05c0f0),
    (0x05C0F2, i_05c0f2),
    (0x05C0F4, i_05c0f4),
    (0x05C0F6, i_05c0f6),
    (0x05C0F8, i_05c0f8),
    (0x05C0FA, i_05c0fa),
    (0x05C0FC, i_05c0fc),
    (0x05C0FE, i_05c0fe),
    (0x05C100, i_05c100),
    (0x05C102, i_05c102),
    (0x05C104, i_05c104),
    (0x05C106, i_05c106),
    (0x05C108, i_05c108),
    (0x05C10A, i_05c10a),
    (0x05C10C, i_05c10c),
    (0x05C10E, i_05c10e),
    (0x05C110, i_05c110),
    (0x05C112, i_05c112),
    (0x05C114, i_05c114),
    (0x05C116, i_05c116),
    (0x05C118, i_05c118),
    (0x05C11A, i_05c11a),
    (0x05C11C, i_05c11c),
    (0x05C11E, i_05c11e),
    (0x05C120, i_05c120),
    (0x05C122, i_05c122),
    (0x05C124, i_05c124),
    (0x05C126, i_05c126),
    (0x05C128, i_05c128),
    (0x05C12A, i_05c12a),
    (0x05C12C, i_05c12c),
    (0x05C12E, i_05c12e),
    (0x05C130, i_05c130),
    (0x05C132, i_05c132),
    (0x05C134, i_05c134),
    (0x05C136, i_05c136),
    (0x05C138, i_05c138),
    (0x05C13A, i_05c13a),
    (0x05C13C, i_05c13c),
    (0x05C13E, i_05c13e),
    (0x05C140, i_05c140),
    (0x05C142, i_05c142),
    (0x05C144, i_05c144),
    (0x05C146, i_05c146),
    (0x05C148, i_05c148),
    (0x05C14A, i_05c14a),
    (0x05C14C, i_05c14c),
    (0x05C14E, i_05c14e),
    (0x05C150, i_05c150),
    (0x05C152, i_05c152),
    (0x05C154, i_05c154),
    (0x05C156, i_05c156),
    (0x05C158, i_05c158),
    (0x05C15A, i_05c15a),
    (0x05C15C, i_05c15c),
    (0x05C15E, i_05c15e),
    (0x05C160, i_05c160),
    (0x05C162, i_05c162),
    (0x05C164, i_05c164),
    (0x05C166, i_05c166),
    (0x05C168, i_05c168),
    (0x05C16A, i_05c16a),
    (0x05C16C, i_05c16c),
    (0x05C16E, i_05c16e),
    (0x05C170, i_05c170),
    (0x05C172, i_05c172),
    (0x05C174, i_05c174),
    (0x05C176, i_05c176),
    (0x05C178, i_05c178),
    (0x05C17A, i_05c17a),
    (0x05C17C, i_05c17c),
    (0x05C17E, i_05c17e),
    (0x05C180, i_05c180),
    (0x05C182, i_05c182),
    (0x05C184, i_05c184),
    (0x05C188, i_05c188),
    (0x05C18A, i_05c18a),
    (0x05C18C, i_05c18c),
    (0x05C190, i_05c190),
    (0x05C196, i_05c196),
    (0x05C198, i_05c198),
    (0x05C19A, i_05c19a),
    (0x05C19E, i_05c19e),
    (0x05C1A0, i_05c1a0),
    (0x05C1A2, i_05c1a2),
    (0x05C1A6, i_05c1a6),
    (0x05C1A8, i_05c1a8),
    (0x05C1AA, i_05c1aa),
    (0x05C1AE, i_05c1ae),
    (0x05C1B0, i_05c1b0),
    (0x05C1B2, i_05c1b2),
    (0x05C1B4, i_05c1b4),
    (0x05C1B6, i_05c1b6),
    (0x05C1BA, i_05c1ba),
    (0x05C1BC, i_05c1bc),
    (0x05C1BE, i_05c1be),
    (0x05C1C0, i_05c1c0),
    (0x05C1C2, i_05c1c2),
    (0x05C1C6, i_05c1c6),
    (0x05C1C8, i_05c1c8),
    (0x05C1CC, i_05c1cc),
    (0x05C1CE, i_05c1ce),
    (0x05C1D0, i_05c1d0),
    (0x05C1D4, i_05c1d4),
    (0x05C1D8, i_05c1d8),
    (0x05C1DC, i_05c1dc),
    (0x05C1E0, i_05c1e0),
    (0x05C1E4, i_05c1e4),
    (0x05C1E8, i_05c1e8),
    (0x05C1EC, i_05c1ec),
    (0x05C1F0, i_05c1f0),
    (0x05C1F4, i_05c1f4),
    (0x05C1F8, i_05c1f8),
    (0x05C1FC, i_05c1fc),
    (0x05C200, i_05c200),
    (0x05C204, i_05c204),
    (0x05C208, i_05c208),
    (0x05C20C, i_05c20c),
    (0x05C210, i_05c210),
    (0x05C214, i_05c214),
    (0x05C218, i_05c218),
    (0x05C21C, i_05c21c),
    (0x05C220, i_05c220),
    (0x05C224, i_05c224),
    (0x05C228, i_05c228),
    (0x05C22C, i_05c22c),
    (0x05C230, i_05c230),
    (0x05C234, i_05c234),
    (0x05C238, i_05c238),
    (0x05C23C, i_05c23c),
    (0x05C240, i_05c240),
    (0x05C244, i_05c244),
    (0x05C248, i_05c248),
    (0x05C24C, i_05c24c),
    (0x05C250, i_05c250),
    (0x05C252, i_05c252),
    (0x05C254, i_05c254),
    (0x05C256, i_05c256),
    (0x05C258, i_05c258),
    (0x05C25A, i_05c25a),
    (0x05C25C, i_05c25c),
    (0x05C25E, i_05c25e),
    (0x05C260, i_05c260),
    (0x05C262, i_05c262),
    (0x05C264, i_05c264),
    (0x05C266, i_05c266),
    (0x05C268, i_05c268),
    (0x05C26A, i_05c26a),
    (0x05C26C, i_05c26c),
    (0x05C26E, i_05c26e),
    (0x05C270, i_05c270),
    (0x05C272, i_05c272),
    (0x05C274, i_05c274),
    (0x05C276, i_05c276),
    (0x05C278, i_05c278),
    (0x05C27A, i_05c27a),
    (0x05C27C, i_05c27c),
    (0x05C27E, i_05c27e),
    (0x05C280, i_05c280),
    (0x05C282, i_05c282),
    (0x05C284, i_05c284),
    (0x05C286, i_05c286),
    (0x05C288, i_05c288),
    (0x05C28A, i_05c28a),
    (0x05C28C, i_05c28c),
    (0x05C28E, i_05c28e),
    (0x05C290, i_05c290),
    (0x05C292, i_05c292),
    (0x05C294, i_05c294),
    (0x05C296, i_05c296),
    (0x05C298, i_05c298),
    (0x05C29A, i_05c29a),
    (0x05C29C, i_05c29c),
    (0x05C29E, i_05c29e),
    (0x05C2A0, i_05c2a0),
    (0x05C2A2, i_05c2a2),
    (0x05C2A4, i_05c2a4),
    (0x05C2A6, i_05c2a6),
    (0x05C2A8, i_05c2a8),
    (0x05C2AA, i_05c2aa),
    (0x05C2AC, i_05c2ac),
    (0x05C2AE, i_05c2ae),
    (0x05C2B0, i_05c2b0),
    (0x05C2B2, i_05c2b2),
    (0x05C2B4, i_05c2b4),
    (0x05C2B6, i_05c2b6),
    (0x05C2B8, i_05c2b8),
    (0x05C2BA, i_05c2ba),
    (0x05C2BC, i_05c2bc),
    (0x05C2BE, i_05c2be),
    (0x05C2C0, i_05c2c0),
    (0x05C2C2, i_05c2c2),
    (0x05C2C4, i_05c2c4),
    (0x05C2C6, i_05c2c6),
    (0x05C2C8, i_05c2c8),
    (0x05C2CA, i_05c2ca),
    (0x05C2CC, i_05c2cc),
    (0x05C2CE, i_05c2ce),
    (0x05C2D0, i_05c2d0),
    (0x05C2D2, i_05c2d2),
    (0x05C2D4, i_05c2d4),
    (0x05C2D6, i_05c2d6),
    (0x05C2D8, i_05c2d8),
    (0x05C2DA, i_05c2da),
    (0x05C2DC, i_05c2dc),
    (0x05C2DE, i_05c2de),
    (0x05C2E0, i_05c2e0),
    (0x05C2E2, i_05c2e2),
    (0x05C2E4, i_05c2e4),
    (0x05C2E6, i_05c2e6),
    (0x05C2E8, i_05c2e8),
    (0x05C2EA, i_05c2ea),
    (0x05C2EC, i_05c2ec),
    (0x05C2EE, i_05c2ee),
    (0x05C2F0, i_05c2f0),
    (0x05C2F2, i_05c2f2),
    (0x05C2F4, i_05c2f4),
    (0x05C2F6, i_05c2f6),
    (0x05C2F8, i_05c2f8),
    (0x05C2FA, i_05c2fa),
    (0x05C2FC, i_05c2fc),
    (0x05C2FE, i_05c2fe),
    (0x05C300, i_05c300),
    (0x05C302, i_05c302),
    (0x05C304, i_05c304),
    (0x05C306, i_05c306),
    (0x05C308, i_05c308),
    (0x05C30A, i_05c30a),
    (0x05C30C, i_05c30c),
    (0x05C30E, i_05c30e),
    (0x05C310, i_05c310),
    (0x05C312, i_05c312),
    (0x05C314, i_05c314),
    (0x05C316, i_05c316),
    (0x05C318, i_05c318),
    (0x05C31A, i_05c31a),
    (0x05C31C, i_05c31c),
    (0x05C31E, i_05c31e),
    (0x05C320, i_05c320),
    (0x05C322, i_05c322),
    (0x05C324, i_05c324),
    (0x05C326, i_05c326),
    (0x05C328, i_05c328),
    (0x05C32A, i_05c32a),
    (0x05C32C, i_05c32c),
    (0x05C32E, i_05c32e),
    (0x05C330, i_05c330),
    (0x05C332, i_05c332),
    (0x05C334, i_05c334),
    (0x05C336, i_05c336),
    (0x05C338, i_05c338),
    (0x05C33A, i_05c33a),
    (0x05C33C, i_05c33c),
    (0x05C33E, i_05c33e),
    (0x05C340, i_05c340),
    (0x05C342, i_05c342),
    (0x05C344, i_05c344),
    (0x05C346, i_05c346),
    (0x05C348, i_05c348),
    (0x05C34A, i_05c34a),
    (0x05C34C, i_05c34c),
    (0x05C34E, i_05c34e),
    (0x05C350, i_05c350),
    (0x05C352, i_05c352),
    (0x05C354, i_05c354),
    (0x05C356, i_05c356),
    (0x05C358, i_05c358),
    (0x05C35A, i_05c35a),
    (0x05C35C, i_05c35c),
    (0x05C35E, i_05c35e),
    (0x05C360, i_05c360),
    (0x05C362, i_05c362),
    (0x05C364, i_05c364),
    (0x05C366, i_05c366),
    (0x05C368, i_05c368),
    (0x05C36A, i_05c36a),
    (0x05C36C, i_05c36c),
    (0x05C36E, i_05c36e),
    (0x05C370, i_05c370),
    (0x05C372, i_05c372),
    (0x05C374, i_05c374),
    (0x05C376, i_05c376),
    (0x05C378, i_05c378),
    (0x05C37A, i_05c37a),
    (0x05C37C, i_05c37c),
    (0x05C37E, i_05c37e),
    (0x05C380, i_05c380),
    (0x05C382, i_05c382),
    (0x05C384, i_05c384),
    (0x05C386, i_05c386),
    (0x05C388, i_05c388),
    (0x05C38A, i_05c38a),
    (0x05C38C, i_05c38c),
    (0x05C38E, i_05c38e),
    (0x05C390, i_05c390),
    (0x05C394, i_05c394),
    (0x05C396, i_05c396),
    (0x05C398, i_05c398),
    (0x05C39C, i_05c39c),
    (0x05C3A2, i_05c3a2),
    (0x05C3A4, i_05c3a4),
    (0x05C3A6, i_05c3a6),
    (0x05C3AA, i_05c3aa),
    (0x05C3AC, i_05c3ac),
    (0x05C3AE, i_05c3ae),
    (0x05C3B2, i_05c3b2),
    (0x05C3B4, i_05c3b4),
    (0x05C3B6, i_05c3b6),
    (0x05C3BA, i_05c3ba),
    (0x05C3BC, i_05c3bc),
    (0x05C3BE, i_05c3be),
    (0x05C3C0, i_05c3c0),
    (0x05C3C2, i_05c3c2),
    (0x05C3C6, i_05c3c6),
    (0x05C3C8, i_05c3c8),
    (0x05C3CA, i_05c3ca),
    (0x05C3CC, i_05c3cc),
    (0x05C3CE, i_05c3ce),
    (0x05C3D2, i_05c3d2),
    (0x05C3D4, i_05c3d4),
    (0x05C3D8, i_05c3d8),
    (0x05C3DA, i_05c3da),
    (0x05C3DC, i_05c3dc),
    (0x05C3E0, i_05c3e0),
    (0x05C3E4, i_05c3e4),
    (0x05C3E8, i_05c3e8),
    (0x05C3EC, i_05c3ec),
    (0x05C3F0, i_05c3f0),
    (0x05C3F4, i_05c3f4),
    (0x05C3F8, i_05c3f8),
    (0x05C3FC, i_05c3fc),
    (0x05C400, i_05c400),
    (0x05C404, i_05c404),
    (0x05C408, i_05c408),
    (0x05C40C, i_05c40c),
    (0x05C410, i_05c410),
    (0x05C414, i_05c414),
    (0x05C418, i_05c418),
    (0x05C41C, i_05c41c),
    (0x05C420, i_05c420),
    (0x05C424, i_05c424),
    (0x05C428, i_05c428),
    (0x05C42C, i_05c42c),
    (0x05C430, i_05c430),
    (0x05C434, i_05c434),
    (0x05C438, i_05c438),
    (0x05C43C, i_05c43c),
    (0x05C440, i_05c440),
    (0x05C444, i_05c444),
    (0x05C448, i_05c448),
    (0x05C44C, i_05c44c),
    (0x05C450, i_05c450),
    (0x05C454, i_05c454),
    (0x05C458, i_05c458),
    (0x05C45C, i_05c45c),
    (0x05C45E, i_05c45e),
    (0x05C460, i_05c460),
    (0x05C462, i_05c462),
    (0x05C464, i_05c464),
    (0x05C466, i_05c466),
    (0x05C468, i_05c468),
    (0x05C46A, i_05c46a),
    (0x05C46C, i_05c46c),
    (0x05C46E, i_05c46e),
    (0x05C470, i_05c470),
    (0x05C472, i_05c472),
    (0x05C474, i_05c474),
    (0x05C476, i_05c476),
    (0x05C478, i_05c478),
    (0x05C47A, i_05c47a),
    (0x05C47C, i_05c47c),
    (0x05C47E, i_05c47e),
    (0x05C480, i_05c480),
    (0x05C482, i_05c482),
    (0x05C484, i_05c484),
    (0x05C486, i_05c486),
    (0x05C488, i_05c488),
    (0x05C48A, i_05c48a),
    (0x05C48C, i_05c48c),
    (0x05C48E, i_05c48e),
    (0x05C490, i_05c490),
    (0x05C492, i_05c492),
    (0x05C494, i_05c494),
    (0x05C496, i_05c496),
    (0x05C498, i_05c498),
    (0x05C49A, i_05c49a),
    (0x05C49C, i_05c49c),
    (0x05C49E, i_05c49e),
    (0x05C4A0, i_05c4a0),
    (0x05C4A2, i_05c4a2),
    (0x05C4A4, i_05c4a4),
    (0x05C4A6, i_05c4a6),
    (0x05C4A8, i_05c4a8),
    (0x05C4AA, i_05c4aa),
    (0x05C4AC, i_05c4ac),
    (0x05C4AE, i_05c4ae),
    (0x05C4B0, i_05c4b0),
    (0x05C4B2, i_05c4b2),
    (0x05C4B4, i_05c4b4),
    (0x05C4B6, i_05c4b6),
    (0x05C4B8, i_05c4b8),
    (0x05C4BA, i_05c4ba),
    (0x05C4BC, i_05c4bc),
    (0x05C4BE, i_05c4be),
    (0x05C4C0, i_05c4c0),
    (0x05C4C2, i_05c4c2),
    (0x05C4C4, i_05c4c4),
    (0x05C4C6, i_05c4c6),
    (0x05C4C8, i_05c4c8),
    (0x05C4CA, i_05c4ca),
    (0x05C4CC, i_05c4cc),
    (0x05C4CE, i_05c4ce),
    (0x05C4D0, i_05c4d0),
    (0x05C4D2, i_05c4d2),
    (0x05C4D4, i_05c4d4),
    (0x05C4D6, i_05c4d6),
    (0x05C4D8, i_05c4d8),
    (0x05C4DA, i_05c4da),
    (0x05C4DC, i_05c4dc),
    (0x05C4DE, i_05c4de),
    (0x05C4E0, i_05c4e0),
    (0x05C4E2, i_05c4e2),
    (0x05C4E4, i_05c4e4),
    (0x05C4E6, i_05c4e6),
    (0x05C4E8, i_05c4e8),
    (0x05C4EA, i_05c4ea),
    (0x05C4EC, i_05c4ec),
    (0x05C4EE, i_05c4ee),
    (0x05C4F0, i_05c4f0),
    (0x05C4F2, i_05c4f2),
    (0x05C4F4, i_05c4f4),
    (0x05C4F6, i_05c4f6),
    (0x05C4F8, i_05c4f8),
    (0x05C4FA, i_05c4fa),
    (0x05C4FC, i_05c4fc),
    (0x05C4FE, i_05c4fe),
    (0x05C500, i_05c500),
    (0x05C502, i_05c502),
    (0x05C504, i_05c504),
    (0x05C506, i_05c506),
    (0x05C508, i_05c508),
    (0x05C50A, i_05c50a),
    (0x05C50C, i_05c50c),
    (0x05C50E, i_05c50e),
    (0x05C510, i_05c510),
    (0x05C512, i_05c512),
    (0x05C514, i_05c514),
    (0x05C516, i_05c516),
    (0x05C518, i_05c518),
    (0x05C51A, i_05c51a),
    (0x05C51C, i_05c51c),
    (0x05C51E, i_05c51e),
    (0x05C520, i_05c520),
    (0x05C522, i_05c522),
    (0x05C524, i_05c524),
    (0x05C526, i_05c526),
    (0x05C528, i_05c528),
    (0x05C52A, i_05c52a),
    (0x05C52C, i_05c52c),
    (0x05C52E, i_05c52e),
    (0x05C530, i_05c530),
    (0x05C532, i_05c532),
    (0x05C534, i_05c534),
    (0x05C536, i_05c536),
    (0x05C538, i_05c538),
    (0x05C53A, i_05c53a),
    (0x05C53C, i_05c53c),
    (0x05C53E, i_05c53e),
    (0x05C540, i_05c540),
    (0x05C542, i_05c542),
    (0x05C544, i_05c544),
    (0x05C546, i_05c546),
    (0x05C548, i_05c548),
    (0x05C54A, i_05c54a),
    (0x05C54C, i_05c54c),
    (0x05C54E, i_05c54e),
    (0x05C550, i_05c550),
    (0x05C552, i_05c552),
    (0x05C554, i_05c554),
    (0x05C556, i_05c556),
    (0x05C558, i_05c558),
    (0x05C55A, i_05c55a),
    (0x05C55C, i_05c55c),
    (0x05C55E, i_05c55e),
    (0x05C560, i_05c560),
    (0x05C562, i_05c562),
    (0x05C564, i_05c564),
    (0x05C566, i_05c566),
    (0x05C568, i_05c568),
    (0x05C56A, i_05c56a),
    (0x05C56C, i_05c56c),
    (0x05C56E, i_05c56e),
    (0x05C570, i_05c570),
    (0x05C572, i_05c572),
    (0x05C574, i_05c574),
    (0x05C576, i_05c576),
    (0x05C578, i_05c578),
    (0x05C57A, i_05c57a),
    (0x05C57C, i_05c57c),
    (0x05C57E, i_05c57e),
    (0x05C580, i_05c580),
    (0x05C582, i_05c582),
    (0x05C584, i_05c584),
    (0x05C586, i_05c586),
    (0x05C588, i_05c588),
    (0x05C58A, i_05c58a),
    (0x05C58C, i_05c58c),
    (0x05C58E, i_05c58e),
    (0x05C590, i_05c590),
    (0x05C592, i_05c592),
    (0x05C594, i_05c594),
    (0x05C596, i_05c596),
    (0x05C598, i_05c598),
    (0x05C59A, i_05c59a),
    (0x05C59C, i_05c59c),
    (0x05C59E, i_05c59e),
    (0x05C5A0, i_05c5a0),
    (0x05C5A2, i_05c5a2),
    (0x05C5A4, i_05c5a4),
    (0x05C5A8, i_05c5a8),
    (0x05C5AA, i_05c5aa),
    (0x05C5AC, i_05c5ac),
    (0x05C5AE, i_05c5ae),
    (0x05C5B0, i_05c5b0),
    (0x05C5B2, i_05c5b2),
    (0x05C5B4, i_05c5b4),
    (0x05C5B6, i_05c5b6),
    (0x05C5B8, i_05c5b8),
    (0x05C5BA, i_05c5ba),
    (0x05C5BC, i_05c5bc),
    (0x05C5BE, i_05c5be),
    (0x05C5C0, i_05c5c0),
    (0x05C5C2, i_05c5c2),
    (0x05C5C4, i_05c5c4),
    (0x05C5C6, i_05c5c6),
    (0x05C5C8, i_05c5c8),
    (0x05C5CA, i_05c5ca),
    (0x05C5CC, i_05c5cc),
    (0x05C5CE, i_05c5ce),
    (0x05C5D0, i_05c5d0),
    (0x05C5D2, i_05c5d2),
    (0x05C5D4, i_05c5d4),
    (0x05C5D6, i_05c5d6),
    (0x05C5D8, i_05c5d8),
    (0x05C5DA, i_05c5da),
    (0x05C5DC, i_05c5dc),
    (0x05C5DE, i_05c5de),
    (0x05C5E0, i_05c5e0),
    (0x05C5E2, i_05c5e2),
    (0x05C5E4, i_05c5e4),
    (0x05C5E6, i_05c5e6),
    (0x05C5E8, i_05c5e8),
    (0x05C5EA, i_05c5ea),
    (0x05C5EC, i_05c5ec),
    (0x05C5EE, i_05c5ee),
    (0x05C5F0, i_05c5f0),
    (0x05C5F2, i_05c5f2),
    (0x05C5F4, i_05c5f4),
    (0x05C5F6, i_05c5f6),
    (0x05C5F8, i_05c5f8),
    (0x05C5FA, i_05c5fa),
    (0x05C5FC, i_05c5fc),
    (0x05C5FE, i_05c5fe),
    (0x05C600, i_05c600),
    (0x05C602, i_05c602),
    (0x05C604, i_05c604),
    (0x05C606, i_05c606),
    (0x05C608, i_05c608),
    (0x05C60A, i_05c60a),
    (0x05C60C, i_05c60c),
    (0x05C60E, i_05c60e),
    (0x05C610, i_05c610),
    (0x05C612, i_05c612),
    (0x05C614, i_05c614),
    (0x05C616, i_05c616),
    (0x05C618, i_05c618),
    (0x05C61A, i_05c61a),
    (0x05C61C, i_05c61c),
    (0x05C61E, i_05c61e),
    (0x05C620, i_05c620),
    (0x05C622, i_05c622),
    (0x05C624, i_05c624),
    (0x05C626, i_05c626),
    (0x05C628, i_05c628),
    (0x05C62A, i_05c62a),
    (0x05C62C, i_05c62c),
    (0x05C62E, i_05c62e),
    (0x05C630, i_05c630),
    (0x05C632, i_05c632),
    (0x05C634, i_05c634),
    (0x05C636, i_05c636),
    (0x05C638, i_05c638),
    (0x05C63A, i_05c63a),
    (0x05C63C, i_05c63c),
    (0x05C63E, i_05c63e),
    (0x05C640, i_05c640),
    (0x05C642, i_05c642),
    (0x05C644, i_05c644),
    (0x05C646, i_05c646),
    (0x05C648, i_05c648),
    (0x05C64A, i_05c64a),
    (0x05C64C, i_05c64c),
    (0x05C64E, i_05c64e),
    (0x05C650, i_05c650),
    (0x05C652, i_05c652),
    (0x05C654, i_05c654),
    (0x05C656, i_05c656),
    (0x05C658, i_05c658),
    (0x05C65A, i_05c65a),
    (0x05C65C, i_05c65c),
    (0x05C65E, i_05c65e),
    (0x05C660, i_05c660),
    (0x05C662, i_05c662),
    (0x05C664, i_05c664),
    (0x05C666, i_05c666),
    (0x05C668, i_05c668),
    (0x05C66A, i_05c66a),
    (0x05C66C, i_05c66c),
    (0x05C66E, i_05c66e),
    (0x05C670, i_05c670),
    (0x05C672, i_05c672),
    (0x05C674, i_05c674),
    (0x05C676, i_05c676),
    (0x05C678, i_05c678),
    (0x05C67A, i_05c67a),
    (0x05C67C, i_05c67c),
    (0x05C67E, i_05c67e),
    (0x05C680, i_05c680),
    (0x05C682, i_05c682),
    (0x05C684, i_05c684),
    (0x05C686, i_05c686),
    (0x05C688, i_05c688),
    (0x05C68A, i_05c68a),
    (0x05C68C, i_05c68c),
    (0x05C68E, i_05c68e),
    (0x05C690, i_05c690),
    (0x05C692, i_05c692),
    (0x05C694, i_05c694),
    (0x05C696, i_05c696),
    (0x05C698, i_05c698),
    (0x05C69A, i_05c69a),
    (0x05C69C, i_05c69c),
    (0x05C69E, i_05c69e),
];
