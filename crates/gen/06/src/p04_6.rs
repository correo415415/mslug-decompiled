// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p04_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $04A878: move.b d1, $33(a6)
pub fn i_04a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A87C;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04A87C: jsr $27cee.l
pub fn i_04a87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A882);
    cpu.pc = 0x27CEE;
    16
}
/// $04A882: jsr $28d70.l
pub fn i_04a882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A888);
    cpu.pc = 0x28D70;
    16
}
/// $04A888: bra.w $4a35e
pub fn i_04a888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A88C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
/// $04A948: move.w #$8000, d0
pub fn i_04a948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A94C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A94C: jsr $28134.l
pub fn i_04a94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A952);
    cpu.pc = 0x28134;
    16
}
/// $04A952: andi.w #$ffe3, $38(a6)
pub fn i_04a952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A958;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A958: ori.w #$0, $38(a6)
pub fn i_04a958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A95E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04A95E: jsr $5e9b6.l
pub fn i_04a95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A964);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A964: andi.b #$3, d0
pub fn i_04a964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A968;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04A968: beq.w $4a9be
pub fn i_04a968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A96C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A9BE; return 10; }
    8
}
/// $04A96C: lea.l $4b82a(pc), a0
pub fn i_04a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A970;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B82A;
    16
}
/// $04A970: jsr $28cd4.l
pub fn i_04a970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A976);
    cpu.pc = 0x28CD4;
    16
}
/// $04A976: move.w #$1e, $30(a6)
pub fn i_04a976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A97C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A97C: lea.l $4a982(pc), a1
pub fn i_04a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A980;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A982;
    16
}
/// $04A980: move.l a1, (a6)
pub fn i_04a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A982;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A982: bsr.w $4a218
pub fn i_04a982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A986;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A986);
    cpu.pc = 0x4A218;
    18
}
/// $04A986: jsr $28d70.l
pub fn i_04a986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A98C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A98C);
    cpu.pc = 0x28D70;
    16
}
/// $04A98C: move.w $30(a6), d0
pub fn i_04a98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A990;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04A990: beq.w $4a99e
pub fn i_04a990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A994;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A99E; return 10; }
    8
}
/// $04A994: subq.w #$1, d0
pub fn i_04a994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A996;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04A996: move.w d0, $30(a6)
pub fn i_04a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A99A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A99A: bra.w $4a9ac
pub fn i_04a99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A99E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A9AC;
    10
}
/// $04A99E: tst.b $70(a6)
pub fn i_04a99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9A2;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04A9A2: beq.w $4a9ac
pub fn i_04a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4A9AC; return 10; }
    8
}
/// $04A9A6: lea.l $4a9b0(pc), a1
pub fn i_04a9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4A9B0;
    16
}
/// $04A9AA: move.l a1, (a6)
pub fn i_04a9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04A9AC: bra.w $4a35e
pub fn i_04a9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9B0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A35E;
    10
}
// ===== HumanDeath_Burning_04a9b0  ($04A9B0, 178 B, pow/human_death_049ff2.s)
/// $04A9B0: lea.l $4b898(pc), a0
pub fn i_04a9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9B4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B898;
    16
}
/// $04A9B4: jsr $28cd4.l
pub fn i_04a9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9BA);
    cpu.pc = 0x28CD4;
    16
}
/// $04A9BA: bra.w $4a382
pub fn i_04a9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
/// $04A9BE: lea.l $2b604a.l, a0
pub fn i_04a9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B604A;
    20
}
/// $04A9C4: jsr $28cd4.l
pub fn i_04a9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9CA);
    cpu.pc = 0x28CD4;
    16
}
/// $04A9CA: move.w #$1042, d0
pub fn i_04a9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1042;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A9CE: jsr $2352.l
pub fn i_04a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9D4);
    cpu.pc = 0x2352;
    16
}
/// $04A9D4: clr.b $21(a6)
pub fn i_04a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9D8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04A9D8: lea.l $4aa76(pc), a1
pub fn i_04a9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA76;
    16
}
/// $04A9DC: jsr $4ae.l
pub fn i_04a9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9E2);
    cpu.pc = 0x4AE;
    16
}
/// $04A9E2: move.w #$200, d0
pub fn i_04a9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9E6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04A9E6: bsr.w $4a1f0
pub fn i_04a9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9EA;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x4A9EA);
    cpu.pc = 0x4A1F0;
    18
}
/// $04A9EA: move.w #$28, $30(a6)
pub fn i_04a9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04A9F0: jsr $5e9b6.l
pub fn i_04a9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4A9F6);
    cpu.pc = 0x5E9B6;
    16
}
/// $04A9F6: tst.w d0
pub fn i_04a9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9F8;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $04A9F8: bpl.w $4aa06
pub fn i_04a9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4A9FC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4AA06; return 10; }
    8
}
/// $04A9FC: move.b #$a, $72(a6)
pub fn i_04a9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA02;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04AA02: bra.w $4aa0c
pub fn i_04aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA06;
    cpu.ir = 0x6000;
    cpu.pc = 0x4AA0C;
    10
}
/// $04AA06: move.b #$ff, $72(a6)
pub fn i_04aa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04AA0C: move.b $72(a6), $73(a6)
pub fn i_04aa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA12;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04AA12: move.w #$ffc0, $2e(a6)
pub fn i_04aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA18;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04AA18: lea.l $4aa1e(pc), a1
pub fn i_04aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA1E;
    16
}
/// $04AA1C: move.l a1, (a6)
pub fn i_04aa1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AA1E: jsr $4a268(pc)
pub fn i_04aa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4AA22);
    cpu.pc = 0x4A268;
    16
}
/// $04AA22: jsr $28d70.l
pub fn i_04aa22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA28);
    cpu.pc = 0x28D70;
    16
}
/// $04AA28: btst.b #$5, $5a(a6)
pub fn i_04aa28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA2E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04AA2E: beq.w $4aa38
pub fn i_04aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4AA38; return 10; }
    8
}
/// $04AA32: lea.l $4aa62(pc), a1
pub fn i_04aa32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA62;
    16
}
/// $04AA36: move.l a1, (a6)
pub fn i_04aa36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AA38: subq.w #$1, $30(a6)
pub fn i_04aa38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA3C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x30);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04AA3C: bne.w $4aa46
pub fn i_04aa3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AA46; return 10; }
    8
}
/// $04AA40: lea.l $4aa62(pc), a1
pub fn i_04aa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA62;
    16
}
/// $04AA44: move.l a1, (a6)
pub fn i_04aa44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AA46: subq.b #$1, $73(a6)
pub fn i_04aa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA4A;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04AA4A: bne.w $4aa5e
pub fn i_04aa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AA5E; return 10; }
    8
}
/// $04AA4E: bchg.b #$0, $3a(a6)
pub fn i_04aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA54;
    cpu.ir = 0x86E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 ^ mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04AA54: move.b $72(a6), $73(a6)
pub fn i_04aa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA5A;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $04AA5A: neg.w $28(a6)
pub fn i_04aa5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA5E;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04AA5E: jmp $4a35e(pc)
pub fn i_04aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA62;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x4A35E;
    8
}
// ===== HumanDeath_BurnedDown_04aa62  ($04AA62, 20 B, pow/human_death_049ff2.s)
/// $04AA62: lea.l $4b0c6(pc), a0
pub fn i_04aa62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA66;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B0C6;
    16
}
/// $04AA66: jsr $28cd4.l
pub fn i_04aa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA6C);
    cpu.pc = 0x28CD4;
    16
}
/// $04AA6C: move.b #$ff, $21(a6)
pub fn i_04aa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA72;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04AA72: bra.w $4a382
pub fn i_04aa72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA76;
    cpu.ir = 0x6000;
    cpu.pc = 0x4A382;
    10
}
// ===== HumanDeath_FlameChild_04aa76  ($04AA76, 84 B, pow/human_death_049ff2.s)
/// $04AA76: move.w #$4, d1
pub fn i_04aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA7A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AA7A: jsr $236e.l
pub fn i_04aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA80);
    cpu.pc = 0x236E;
    16
}
/// $04AA80: lea.l $4b926(pc), a0
pub fn i_04aa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA84;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B926;
    16
}
/// $04AA84: jsr $28cd4.l
pub fn i_04aa84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA8A);
    cpu.pc = 0x28CD4;
    16
}
/// $04AA8A: lea.l $4aa90(pc), a1
pub fn i_04aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AA90;
    16
}
/// $04AA8E: move.l a1, (a6)
pub fn i_04aa8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AA90: jsr $7b2.l
pub fn i_04aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AA96);
    cpu.pc = 0x7B2;
    16
}
/// $04AA96: bcc.w $4aaa0
pub fn i_04aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AA9A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4AAA0; return 10; }
    8
}
/// $04AA9A: jmp $518.l
pub fn i_04aa9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAA0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04AAA0: movea.l $c(a6), a0
pub fn i_04aaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAA4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04AAA4: tst.b $21(a0)
pub fn i_04aaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAA8;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04AAA8: beq.w $4aab2
pub fn i_04aaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4AAB2; return 10; }
    8
}
/// $04AAAC: jmp $518.l
pub fn i_04aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAB2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04AAB2: move.w $22(a0), $22(a6)
pub fn i_04aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAB8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04AAB8: move.w $24(a0), d0
pub fn i_04aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AABC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04AABC: addi.w #$1e, d0
pub fn i_04aabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAC0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04AAC0: move.w d0, $24(a6)
pub fn i_04aac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAC4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04AAC4: move.w $38(a0), $38(a6)
pub fn i_04aac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AACA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== JsrAbsThunk_04aaca  ($04AACA, 8 B, thunks/jsr_abs_thunks.c)
/// $04AACA: jsr $28d70.l
pub fn i_04aaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AAD0);
    cpu.pc = 0x28D70;
    16
}
/// $04AAD0: rts
pub fn i_04aad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SpawnBloodSplash_04aad2  ($04AAD2, 62 B, pow/human_death_049ff2.s)
/// $04AAD2: lea.l $4ab10(pc), a1
pub fn i_04aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AB10;
    16
}
/// $04AAD6: jsr $4ae.l
pub fn i_04aad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AADC);
    cpu.pc = 0x4AE;
    16
}
/// $04AADC: bra.w $4aaea
pub fn i_04aadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4AAEA;
    10
}
/// $04AAE0: lea.l $4ab52(pc), a1
pub fn i_04aae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AB52;
    16
}
/// $04AAE4: jsr $4ae.l
pub fn i_04aae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AAEA);
    cpu.pc = 0x4AE;
    16
}
/// $04AAEA: move.w $24(a6), $24(a0)
pub fn i_04aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAF0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04AAF0: move.w #$8, d0
pub fn i_04aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAF4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AAF4: btst.b #$0, $3a(a6)
pub fn i_04aaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAFA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04AAFA: bne.w $4ab06
pub fn i_04aafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AAFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AB06; return 10; }
    8
}
/// $04AAFE: bset.b #$0, $3a(a0)
pub fn i_04aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB04;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04AB04: neg.w d0
pub fn i_04ab04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB06;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04AB06: add.w $22(a6), d0
pub fn i_04ab06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB0A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $04AB0A: move.w d0, $22(a0)
pub fn i_04ab0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB0E;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04AB0E: rts
pub fn i_04ab0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_BloodSplashA_04ab10  ($04AB10, 66 B, pow/human_death_049ff2.s)
/// $04AB10: move.w #$17a, d1
pub fn i_04ab10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB14: tst.b $10fd8f.l
pub fn i_04ab14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB1A;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04AB1A: bne.w $4ab22
pub fn i_04ab1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AB22; return 10; }
    8
}
/// $04AB1E: move.w #$1ba, d1
pub fn i_04ab1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB22;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB22: jsr $236e.l
pub fn i_04ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB28);
    cpu.pc = 0x236E;
    16
}
/// $04AB28: move.w #$c000, d0
pub fn i_04ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB2C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB2C: jsr $28134.l
pub fn i_04ab2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB32);
    cpu.pc = 0x28134;
    16
}
/// $04AB32: andi.w #$ffe3, $38(a6)
pub fn i_04ab32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB38;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04AB38: ori.w #$10, $38(a6)
pub fn i_04ab38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB3E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04AB3E: lea.l $4bad2(pc), a0
pub fn i_04ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB42;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4BAD2;
    16
}
/// $04AB42: jsr $28cd4.l
pub fn i_04ab42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB48);
    cpu.pc = 0x28CD4;
    16
}
/// $04AB48: lea.l $4ab90(pc), a1
pub fn i_04ab48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AB90;
    16
}
/// $04AB4C: move.l a1, (a6)
pub fn i_04ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AB4E: bra.w $4ab90
pub fn i_04ab4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB52;
    cpu.ir = 0x6000;
    cpu.pc = 0x4AB90;
    10
}
// ===== HumanDeath_BloodSplashB_04ab52  ($04AB52, 62 B, pow/human_death_049ff2.s)
/// $04AB52: move.w #$17a, d1
pub fn i_04ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB56;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB56: tst.b $10fd8f.l
pub fn i_04ab56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB5C;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04AB5C: bne.w $4ab64
pub fn i_04ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB60;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4AB64; return 10; }
    8
}
/// $04AB60: move.w #$1ba, d1
pub fn i_04ab60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB64;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB64: jsr $236e.l
pub fn i_04ab64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB6A);
    cpu.pc = 0x236E;
    16
}
/// $04AB6A: move.w #$c000, d0
pub fn i_04ab6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB6E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AB6E: jsr $28134.l
pub fn i_04ab6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB74);
    cpu.pc = 0x28134;
    16
}
/// $04AB74: andi.w #$ffe3, $38(a6)
pub fn i_04ab74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB7A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04AB7A: ori.w #$10, $38(a6)
pub fn i_04ab7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB80;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04AB80: lea.l $4b9f0(pc), a0
pub fn i_04ab80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB84;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4B9F0;
    16
}
/// $04AB84: jsr $28cd4.l
pub fn i_04ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB8A);
    cpu.pc = 0x28CD4;
    16
}
/// $04AB8A: lea.l $4ab90(pc), a1
pub fn i_04ab8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AB90;
    16
}
/// $04AB8E: move.l a1, (a6)
pub fn i_04ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== HumanDeath_BloodSplash_Loop_04ab90  ($04AB90, 24 B, pow/human_death_049ff2.s)
/// $04AB90: jsr $2783a.l
pub fn i_04ab90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB96);
    cpu.pc = 0x2783A;
    16
}
/// $04AB96: jsr $28d70.l
pub fn i_04ab96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AB9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AB9C);
    cpu.pc = 0x28D70;
    16
}
/// $04AB9C: bcc.w $4aba6
pub fn i_04ab9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4ABA6; return 10; }
    8
}
/// $04ABA0: jmp $518.l
pub fn i_04aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABA6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04ABA6: rts
pub fn i_04aba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_PickGibPtr_04aba8  ($04ABA8, 16 B, pow/human_death_049ff2.s)
/// $04ABA8: andi.w #$7, d0
pub fn i_04aba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABAC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04ABAC: lea.l $4af5a(pc), a0
pub fn i_04abac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABB0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4AF5A;
    16
}
/// $04ABB0: asl.w #$2, d0
pub fn i_04abb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABB2;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04ABB2: move.l (a0, d0.w), $76(a6)
pub fn i_04abb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABB8;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
// ===== JsrAbsThunk_04abb8  ($04ABB8, 8 B, thunks/jsr_abs_thunks.c)
/// $04ABB8: jsr $28d70.l
pub fn i_04abb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABBE);
    cpu.pc = 0x28D70;
    16
}
/// $04ABBE: rts
pub fn i_04abbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== HumanDeath_SpawnSmokePair_04abc0  ($04ABC0, 106 B, pow/human_death_049ff2.s)
/// $04ABC0: lea.l $4ac3a(pc), a1
pub fn i_04abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC3A;
    16
}
/// $04ABC4: jsr $4ae.l
pub fn i_04abc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABCA);
    cpu.pc = 0x4AE;
    16
}
/// $04ABCA: jsr $5dd02.l
pub fn i_04abca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABD0);
    cpu.pc = 0x5DD02;
    16
}
/// $04ABD0: addi.w #$18, $24(a0)
pub fn i_04abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABD6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04ABD6: jsr $13600.l
pub fn i_04abd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABDC);
    cpu.pc = 0x13600;
    16
}
/// $04ABDC: move.w #$19f, d1
pub fn i_04abdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABE0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04ABE0: tst.b $10fd8f.l
pub fn i_04abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABE6;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10FD8F) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $04ABE6: bne.w $4abee
pub fn i_04abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4ABEE; return 10; }
    8
}
/// $04ABEA: move.w #$1a3, d1
pub fn i_04abea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABEE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1A3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04ABEE: jsr $236e.l
pub fn i_04abee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABF4);
    cpu.pc = 0x236E;
    16
}
/// $04ABF4: lea.l $4bb44(pc), a0
pub fn i_04abf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABF8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4BB44;
    16
}
/// $04ABF8: jsr $28cd4.l
pub fn i_04abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4ABFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4ABFE);
    cpu.pc = 0x28CD4;
    16
}
/// $04ABFE: lea.l $4ac04(pc), a1
pub fn i_04abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC02;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC04;
    16
}
/// $04AC02: move.l a1, (a6)
pub fn i_04ac02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC04;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AC04: jsr $2783a.l
pub fn i_04ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC0A);
    cpu.pc = 0x2783A;
    16
}
/// $04AC0A: jsr $28d70.l
pub fn i_04ac0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC10);
    cpu.pc = 0x28D70;
    16
}
/// $04AC10: bcc.w $4ac1a
pub fn i_04ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4AC1A; return 10; }
    8
}
/// $04AC14: lea.l $4ac32(pc), a1
pub fn i_04ac14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC32;
    16
}
/// $04AC18: move.l a1, (a6)
pub fn i_04ac18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC1A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AC1A: movea.l #$ffffffff, a0
pub fn i_04ac1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC20;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04AC20: jsr $5dd56.l
pub fn i_04ac20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC26);
    cpu.pc = 0x5DD56;
    16
}
/// $04AC26: bcc.w $4ac30
pub fn i_04ac26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4AC30; return 10; }
    8
}
// ===== SetTaskHandler_04ac2a  ($04AC2A, 8 B, kernel/task_handlers.c)
/// $04AC2A: lea.l $4ac32(pc), a1
pub fn i_04ac2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC32;
    16
}
/// $04AC2E: move.l a1, (a6)
pub fn i_04ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AC30: rts
pub fn i_04ac30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_04ac32  ($04AC32, 8 B, kernel/jmp_to_scheduler.c)
/// $04AC32: jmp $518.l
pub fn i_04ac32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC38;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== HumanDeath_SmokeChild_04ac3a  ($04AC3A, 28 B, pow/human_death_049ff2.s)
/// $04AC3A: move.w #$d, d1
pub fn i_04ac3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC3E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04AC3E: jsr $236e.l
pub fn i_04ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC44);
    cpu.pc = 0x236E;
    16
}
/// $04AC44: lea.l $4bb4a(pc), a0
pub fn i_04ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC48;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4BB4A;
    16
}
/// $04AC48: jsr $28cd4.l
pub fn i_04ac48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4AC4E);
    cpu.pc = 0x28CD4;
    16
}
/// $04AC4E: lea.l $4ac04(pc), a1
pub fn i_04ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4AC04;
    16
}
/// $04AC52: move.l a1, (a6)
pub fn i_04ac52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04AC54: bra.b $4ac04
pub fn i_04ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4AC56;
    cpu.ir = 0x60AE;
    cpu.pc = 0x4AC04;
    10
}
/// $04B052: addq.w #$2, $24(a6)
pub fn i_04b052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B056;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04B056: move.w #$fff8, d0
pub fn i_04b056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B05A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04B05A: jsr $5dca4.l
pub fn i_04b05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B060;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4B060);
    cpu.pc = 0x5DCA4;
    16
}
/// $04B060: move.w d0, $28(a6)
pub fn i_04b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B064;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04B064: move.w #$180, $2a(a6)
pub fn i_04b064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B06A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04B06A: move.w #$ffe8, $2e(a6)
pub fn i_04b06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B070;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04B070: move.w #$0, $2c(a6)
pub fn i_04b070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B076;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04B076: rts
pub fn i_04b076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4B078;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04bb8e  ($04BB8E, 6 B, thunks/ccr_helpers.c)
/// $04BB8E: andi.b #$ee, ccr
pub fn i_04bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BB92;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04BB92: rts
pub fn i_04bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BB94;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04bb94  ($04BB94, 6 B, thunks/ccr_helpers.c)
/// $04BB94: ori.b #$11, ccr
pub fn i_04bb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BB98;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04BB98: rts
pub fn i_04bb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BB9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Spawn_04bb9a  ($04BB9A, 166 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BB9A: move.w #$0, $70(a6)
pub fn i_04bb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBA0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BBA0: bra.w $4bbbe
pub fn i_04bba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BBBE;
    10
}
/// $04BBBE: move.w #$84, d1
pub fn i_04bbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBC2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04BBC2: jsr $236e.l
pub fn i_04bbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BBC8);
    cpu.pc = 0x236E;
    16
}
/// $04BBC8: move.w #$19, $1c(a6)
pub fn i_04bbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBCE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BBCE: jsr $138fe.l
pub fn i_04bbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BBD4);
    cpu.pc = 0x138FE;
    16
}
/// $04BBD4: move.w #$4000, d0
pub fn i_04bbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBD8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04BBD8: jsr $28134.l
pub fn i_04bbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BBDE);
    cpu.pc = 0x28134;
    16
}
/// $04BBDE: andi.w #$ffe3, $38(a6)
pub fn i_04bbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBE4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BBE4: ori.w #$8, $38(a6)
pub fn i_04bbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBEA;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BBEA: moveq #$0, d0
pub fn i_04bbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBEC;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04BBEC: move.b d0, $20(a6)
pub fn i_04bbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBF0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BBF0: move.b d0, $21(a6)
pub fn i_04bbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBF4;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BBF4: lea.l $2bd0da.l, a0
pub fn i_04bbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BBFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD0DA;
    20
}
/// $04BBFA: jsr $799de.l
pub fn i_04bbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC00);
    cpu.pc = 0x799DE;
    16
}
/// $04BC00: move.w d0, $66(a6)
pub fn i_04bc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC04;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BC04: jsr $267e2.l
pub fn i_04bc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC0A);
    cpu.pc = 0x267E2;
    16
}
/// $04BC0A: jsr $5e7c0.l
pub fn i_04bc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC10);
    cpu.pc = 0x5E7C0;
    16
}
/// $04BC10: lea.l $2902d8.l, a0
pub fn i_04bc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902D8;
    20
}
/// $04BC16: move.w $70(a6), d0
pub fn i_04bc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC1A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BC1A: andi.w #$3, d0
pub fn i_04bc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC1E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04BC1E: lsl.w #$2, d0
pub fn i_04bc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC20;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04BC20: movea.l (a0, d0.w), a1
pub fn i_04bc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC24;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $04BC24: jsr $43fac.l
pub fn i_04bc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC2A);
    cpu.pc = 0x43FAC;
    16
}
/// $04BC2A: lea.l $4c776(pc), a1
pub fn i_04bc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C776;
    16
}
/// $04BC2E: jsr $4ae.l
pub fn i_04bc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC34);
    cpu.pc = 0x4AE;
    16
}
/// $04BC34: jsr $5dd02.l
pub fn i_04bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC3A);
    cpu.pc = 0x5DD02;
    16
}
/// $04BC3A: move.w $70(a6), $70(a0)
pub fn i_04bc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC40;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
// ===== SetTaskHandler_04bc40  ($04BC40, 8 B, kernel/task_handlers.c)
/// $04BC40: lea.l $4bc48(pc), a1
pub fn i_04bc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BC48;
    16
}
/// $04BC44: move.l a1, (a6)
pub fn i_04bc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BC46: rts
pub fn i_04bc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpawnCrewAndIdle_04bc48  ($04BC48, 270 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BC48: lea.l $2902f0.l, a0
pub fn i_04bc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902F0;
    20
}
/// $04BC4E: move.w $70(a6), d0
pub fn i_04bc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC52;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BC52: cmpi.b #$1, (a0, d0.w)
pub fn i_04bc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC58;
    cpu.ir = 0xC30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    18
}
/// $04BC58: bne.w $4bc76
pub fn i_04bc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC5C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BC76; return 10; }
    8
}
/// $04BC5C: lea.l $4c44a(pc), a1
pub fn i_04bc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C44A;
    16
}
/// $04BC60: jsr $4ae.l
pub fn i_04bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC66);
    cpu.pc = 0x4AE;
    16
}
/// $04BC66: jsr $5dd02.l
pub fn i_04bc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC6C);
    cpu.pc = 0x5DD02;
    16
}
/// $04BC6C: addi.w #$18, $24(a0)
pub fn i_04bc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC72;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BC72: bra.w $4bc96
pub fn i_04bc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC76;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BC96;
    10
}
/// $04BC76: cmpi.b #$2, (a0, d0.w)
pub fn i_04bc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC7C;
    cpu.ir = 0xC30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    18
}
/// $04BC7C: bne.w $4bc96
pub fn i_04bc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC80;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BC96; return 10; }
    8
}
/// $04BC80: lea.l $4c2e4(pc), a1
pub fn i_04bc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C2E4;
    16
}
/// $04BC84: jsr $4ae.l
pub fn i_04bc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC8A);
    cpu.pc = 0x4AE;
    16
}
/// $04BC8A: jsr $5dd02.l
pub fn i_04bc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BC90);
    cpu.pc = 0x5DD02;
    16
}
/// $04BC90: addi.w #$18, $24(a0)
pub fn i_04bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC96;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BC96: lea.l $2902e8.l, a0
pub fn i_04bc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BC9C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902E8;
    20
}
/// $04BC9C: move.w $70(a6), d0
pub fn i_04bc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCA0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BCA0: tst.b (a0, d0.w)
pub fn i_04bca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCA4;
    cpu.ir = 0x4A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $04BCA4: beq.w $4bcd0
pub fn i_04bca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4BCD0; return 10; }
    8
}
/// $04BCA8: lea.l $4bee0(pc), a1
pub fn i_04bca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BEE0;
    16
}
/// $04BCAC: jsr $4ae.l
pub fn i_04bcac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BCB2);
    cpu.pc = 0x4AE;
    16
}
/// $04BCB2: jsr $5dd02.l
pub fn i_04bcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BCB8);
    cpu.pc = 0x5DD02;
    16
}
/// $04BCB8: lea.l $2902f4.l, a1
pub fn i_04bcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCBE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2902F4;
    20
}
/// $04BCBE: move.w $70(a6), d1
pub fn i_04bcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCC2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BCC2: move.w d1, $70(a0)
pub fn i_04bcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCC6;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BCC6: add.w d1, d1
pub fn i_04bcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCC8;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04BCC8: add.w d1, d1
pub fn i_04bcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCCA;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04BCCA: move.w $2(a1, d1.w), $72(a0)
pub fn i_04bcca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCD0;
    cpu.ir = 0x3171;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    22
}
/// $04BCD0: lea.l $2902ec.l, a0
pub fn i_04bcd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2902EC;
    20
}
/// $04BCD6: move.w $70(a6), d0
pub fn i_04bcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCDA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BCDA: tst.b (a0, d0.w)
pub fn i_04bcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCDE;
    cpu.ir = 0x4A30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    14
}
/// $04BCDE: beq.w $4bcf8
pub fn i_04bcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCE2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4BCF8; return 10; }
    8
}
/// $04BCE2: lea.l $4c1b0(pc), a1
pub fn i_04bce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C1B0;
    16
}
/// $04BCE6: jsr $4ae.l
pub fn i_04bce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BCEC);
    cpu.pc = 0x4AE;
    16
}
/// $04BCEC: jsr $5dd02.l
pub fn i_04bcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BCF2);
    cpu.pc = 0x5DD02;
    16
}
/// $04BCF2: move.w $70(a6), $70(a0)
pub fn i_04bcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCF8;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04BCF8: jsr $4ca0e(pc)
pub fn i_04bcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BCFC);
    cpu.pc = 0x4CA0E;
    16
}
/// $04BCFC: moveq #$0, d0
pub fn i_04bcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BCFE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04BCFE: move.b d0, $20(a6)
pub fn i_04bcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD02;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BD02: move.w d0, $88(a6)
pub fn i_04bd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD06;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BD06: lea.l $4bd0c(pc), a1
pub fn i_04bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD0C;
    16
}
/// $04BD0A: move.l a1, (a6)
pub fn i_04bd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD0C: jsr $2783a.l
pub fn i_04bd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BD12);
    cpu.pc = 0x2783A;
    16
}
/// $04BD12: lea.l $290304.l, a1
pub fn i_04bd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD18;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290304;
    20
}
/// $04BD18: jsr $4c98a(pc)
pub fn i_04bd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BD1C);
    cpu.pc = 0x4C98A;
    16
}
/// $04BD1C: jsr $28d70.l
pub fn i_04bd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BD22);
    cpu.pc = 0x28D70;
    16
}
/// $04BD22: jsr $4c9d6(pc)
pub fn i_04bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BD26);
    cpu.pc = 0x4C9D6;
    16
}
/// $04BD26: bcc.w $4bd36
pub fn i_04bd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BD36; return 10; }
    8
}
/// $04BD2A: move.b #$1, $20(a6)
pub fn i_04bd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD30;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BD30: lea.l $4bd6c(pc), a1
pub fn i_04bd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD6C;
    16
}
/// $04BD34: move.l a1, (a6)
pub fn i_04bd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD36: clr.b $10e39a.l
pub fn i_04bd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD3C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04BD3C: jsr $4c6d4(pc)
pub fn i_04bd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD40;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BD40);
    cpu.pc = 0x4C6D4;
    16
}
/// $04BD40: movea.l #$ffffffff, a0
pub fn i_04bd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD46;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BD46: lea.l $290912.l, a0
pub fn i_04bd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BD4C: jsr $5dd5c.l
pub fn i_04bd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BD52);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BD52: bcc.w $4bd5c
pub fn i_04bd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BD5C; return 10; }
    8
}
// ===== SetTaskHandler_04bd56  ($04BD56, 8 B, kernel/task_handlers.c)
/// $04BD56: lea.l $4c578(pc), a1
pub fn i_04bd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C578;
    16
}
/// $04BD5A: move.l a1, (a6)
pub fn i_04bd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD5C: rts
pub fn i_04bd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Rearm_04bd5e  ($04BD5E, 50 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BD5E: clr.w $88(a6)
pub fn i_04bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD62;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BD62: lea.l $4bd90(pc), a1
pub fn i_04bd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD90;
    16
}
/// $04BD66: move.l a1, (a6)
pub fn i_04bd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BD68: bra.w $4bd90
pub fn i_04bd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BD90;
    10
}
// ===== GunPlatform_Aim_04bd90  ($04BD90, 108 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BD90: jsr $2783a.l
pub fn i_04bd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BD96);
    cpu.pc = 0x2783A;
    16
}
/// $04BD96: lea.l $290304.l, a1
pub fn i_04bd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BD9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290304;
    20
}
/// $04BD9C: jsr $4c98a(pc)
pub fn i_04bd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BDA0);
    cpu.pc = 0x4C98A;
    16
}
/// $04BDA0: jsr $28d70.l
pub fn i_04bda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BDA6);
    cpu.pc = 0x28D70;
    16
}
/// $04BDA6: tst.w $88(a6)
pub fn i_04bda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDAA;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $04BDAA: bne.w $4bddc
pub fn i_04bdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BDDC; return 10; }
    8
}
/// $04BDAE: jsr $4c9ee(pc)
pub fn i_04bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BDB2);
    cpu.pc = 0x4C9EE;
    16
}
/// $04BDB2: bcc.w $4bddc
pub fn i_04bdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BDDC; return 10; }
    8
}
/// $04BDB6: btst.b #$0, $8a(a6)
pub fn i_04bdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDBC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04BDBC: bne.w $4bdd0
pub fn i_04bdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDC0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BDD0; return 10; }
    8
}
/// $04BDC0: move.b #$2, $20(a6)
pub fn i_04bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BDC6: lea.l $4be04(pc), a1
pub fn i_04bdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BE04;
    16
}
/// $04BDCA: move.l a1, (a6)
pub fn i_04bdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BDCC: bra.w $4bddc
pub fn i_04bdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BDDC;
    10
}
/// $04BDD0: move.b #$3, $20(a6)
pub fn i_04bdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDD6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BDD6: lea.l $4be72(pc), a1
pub fn i_04bdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BE72;
    16
}
/// $04BDDA: move.l a1, (a6)
pub fn i_04bdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BDDC: clr.b $10e39a.l
pub fn i_04bddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDE2;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04BDE2: jsr $4c6d4(pc)
pub fn i_04bde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BDE6);
    cpu.pc = 0x4C6D4;
    16
}
/// $04BDE6: movea.l #$ffffffff, a0
pub fn i_04bde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDEC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BDEC: lea.l $290912.l, a0
pub fn i_04bdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BDF2: jsr $5dd5c.l
pub fn i_04bdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BDF8);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BDF8: bcc.w $4be02
pub fn i_04bdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BDFC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BE02; return 10; }
    8
}
// ===== SetTaskHandler_04bdfc  ($04BDFC, 8 B, kernel/task_handlers.c)
/// $04BDFC: lea.l $4c578(pc), a1
pub fn i_04bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C578;
    16
}
/// $04BE00: move.l a1, (a6)
pub fn i_04be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE02: rts
pub fn i_04be02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_FireA_04be04  ($04BE04, 102 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BE04: move.w #$1b, $88(a6)
pub fn i_04be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE0A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BE0A: lea.l $4be10(pc), a1
pub fn i_04be0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BE10;
    16
}
/// $04BE0E: move.l a1, (a6)
pub fn i_04be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE10: jsr $2783a.l
pub fn i_04be10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE16);
    cpu.pc = 0x2783A;
    16
}
/// $04BE16: lea.l $290304.l, a1
pub fn i_04be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE1C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290304;
    20
}
/// $04BE1C: jsr $4c98a(pc)
pub fn i_04be1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BE20);
    cpu.pc = 0x4C98A;
    16
}
/// $04BE20: jsr $28d70.l
pub fn i_04be20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE26);
    cpu.pc = 0x28D70;
    16
}
/// $04BE26: subq.w #$1, $88(a6)
pub fn i_04be26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE2A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BE2A: bne.w $4be4a
pub fn i_04be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BE4A; return 10; }
    8
}
/// $04BE2E: addq.b #$1, $8a(a6)
pub fn i_04be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE32;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04BE32: subq.b #$1, $82(a6)
pub fn i_04be32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE36;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04BE36: bne.w $4be44
pub fn i_04be36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BE44; return 10; }
    8
}
/// $04BE3A: lea.l $4bcf8(pc), a1
pub fn i_04be3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BCF8;
    16
}
/// $04BE3E: move.l a1, (a6)
pub fn i_04be3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE40: bra.w $4be4a
pub fn i_04be40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE44;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BE4A;
    10
}
/// $04BE44: lea.l $4bd5e(pc), a1
pub fn i_04be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD5E;
    16
}
/// $04BE48: move.l a1, (a6)
pub fn i_04be48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE4A: clr.b $10e39a.l
pub fn i_04be4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE50;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04BE50: jsr $4c6d4(pc)
pub fn i_04be50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BE54);
    cpu.pc = 0x4C6D4;
    16
}
/// $04BE54: movea.l #$ffffffff, a0
pub fn i_04be54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE5A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BE5A: lea.l $290912.l, a0
pub fn i_04be5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BE60: jsr $5dd5c.l
pub fn i_04be60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE66);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BE66: bcc.w $4be70
pub fn i_04be66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BE70; return 10; }
    8
}
// ===== SetTaskHandler_04be6a  ($04BE6A, 8 B, kernel/task_handlers.c)
/// $04BE6A: lea.l $4c578(pc), a1
pub fn i_04be6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C578;
    16
}
/// $04BE6E: move.l a1, (a6)
pub fn i_04be6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE70: rts
pub fn i_04be70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE72;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_FireB_04be72  ($04BE72, 102 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BE72: move.w #$1b, $88(a6)
pub fn i_04be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE78;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BE78: lea.l $4be7e(pc), a1
pub fn i_04be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BE7E;
    16
}
/// $04BE7C: move.l a1, (a6)
pub fn i_04be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BE7E: jsr $2783a.l
pub fn i_04be7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE84);
    cpu.pc = 0x2783A;
    16
}
/// $04BE84: lea.l $290304.l, a1
pub fn i_04be84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE8A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290304;
    20
}
/// $04BE8A: jsr $4c98a(pc)
pub fn i_04be8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BE8E);
    cpu.pc = 0x4C98A;
    16
}
/// $04BE8E: jsr $28d70.l
pub fn i_04be8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BE94);
    cpu.pc = 0x28D70;
    16
}
/// $04BE94: subq.w #$1, $88(a6)
pub fn i_04be94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE98;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BE98: bne.w $4beb8
pub fn i_04be98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BE9C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BEB8; return 10; }
    8
}
/// $04BE9C: addq.b #$1, $8a(a6)
pub fn i_04be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEA0;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04BEA0: subq.b #$1, $82(a6)
pub fn i_04bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEA4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04BEA4: bne.w $4beb2
pub fn i_04bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEA8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BEB2; return 10; }
    8
}
/// $04BEA8: lea.l $4bcf8(pc), a1
pub fn i_04bea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BCF8;
    16
}
/// $04BEAC: move.l a1, (a6)
pub fn i_04beac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BEAE: bra.w $4beb8
pub fn i_04beae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEB2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4BEB8;
    10
}
/// $04BEB2: lea.l $4bd5e(pc), a1
pub fn i_04beb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BD5E;
    16
}
/// $04BEB6: move.l a1, (a6)
pub fn i_04beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BEB8: clr.b $10e39a.l
pub fn i_04beb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEBE;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $04BEBE: jsr $4c6d4(pc)
pub fn i_04bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BEC2);
    cpu.pc = 0x4C6D4;
    16
}
/// $04BEC2: movea.l #$ffffffff, a0
pub fn i_04bec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEC8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BEC8: lea.l $290912.l, a0
pub fn i_04bec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BECE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BECE: jsr $5dd5c.l
pub fn i_04bece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BED4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BED4);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BED4: bcc.w $4bede
pub fn i_04bed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BED8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BEDE; return 10; }
    8
}
// ===== SetTaskHandler_04bed8  ($04BED8, 8 B, kernel/task_handlers.c)
/// $04BED8: lea.l $4c578(pc), a1
pub fn i_04bed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C578;
    16
}
/// $04BEDC: move.l a1, (a6)
pub fn i_04bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BEDE: rts
pub fn i_04bede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Hatch_04bee0  ($04BEE0, 112 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BEE0: move.w #$84, d1
pub fn i_04bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEE4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04BEE4: jsr $236e.l
pub fn i_04bee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BEEA);
    cpu.pc = 0x236E;
    16
}
/// $04BEEA: move.w #$19, $1c(a6)
pub fn i_04beea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEF0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04BEF0: jsr $138fe.l
pub fn i_04bef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BEF6);
    cpu.pc = 0x138FE;
    16
}
/// $04BEF6: move.w #$8000, d0
pub fn i_04bef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BEFA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04BEFA: jsr $28134.l
pub fn i_04befa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BF00);
    cpu.pc = 0x28134;
    16
}
/// $04BF00: andi.w #$ffe3, $38(a6)
pub fn i_04bf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF06;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BF06: ori.w #$8, $38(a6)
pub fn i_04bf06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF0C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04BF0C: moveq #$0, d0
pub fn i_04bf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF0E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04BF0E: move.b d0, $20(a6)
pub fn i_04bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF12;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BF12: move.b d0, $21(a6)
pub fn i_04bf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF16;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04BF16: jsr $267e2.l
pub fn i_04bf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BF1C);
    cpu.pc = 0x267E2;
    16
}
/// $04BF1C: lea.l $4bf22(pc), a1
pub fn i_04bf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BF22;
    16
}
/// $04BF20: move.l a1, (a6)
pub fn i_04bf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BF22: movea.l $c(a6), a0
pub fn i_04bf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF26;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04BF26: cmpi.b #$1, $20(a0)
pub fn i_04bf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF2C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04BF2C: bne.w $4bf36
pub fn i_04bf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF30;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BF36; return 10; }
    8
}
/// $04BF30: lea.l $4bf58(pc), a1
pub fn i_04bf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BF58;
    16
}
/// $04BF34: move.l a1, (a6)
pub fn i_04bf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BF36: jsr $4c942(pc)
pub fn i_04bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BF3A);
    cpu.pc = 0x4C942;
    16
}
/// $04BF3A: movea.l #$ffffffff, a0
pub fn i_04bf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF40;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BF40: lea.l $290912.l, a0
pub fn i_04bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BF46: jsr $5dd5c.l
pub fn i_04bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BF4C);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BF4C: bcc.w $4bf56
pub fn i_04bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BF56; return 10; }
    8
}
// ===== SetTaskHandler_04bf50  ($04BF50, 8 B, kernel/task_handlers.c)
/// $04BF50: lea.l $4c58c(pc), a1
pub fn i_04bf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04BF54: move.l a1, (a6)
pub fn i_04bf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BF56: rts
pub fn i_04bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HatchOpen_04bf58  ($04BF58, 144 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BF58: move.b $21(a6), d0
pub fn i_04bf58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF5C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04BF5C: andi.w #$ff, d0
pub fn i_04bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF60;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04BF60: movea.l #$290344, a0
pub fn i_04bf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF66;
    cpu.ir = 0x207C;
    let v1: u32 = 0x290344;
    cpu.a[0] = v1;
    4
}
/// $04BF66: lsl.w #$2, d0
pub fn i_04bf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF68;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04BF68: movea.l (a0, d0.w), a0
pub fn i_04bf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF6C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04BF6C: cmpa.l #$ffffffff, a0
pub fn i_04bf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF72;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04BF72: beq.w $4bf7c
pub fn i_04bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4BF7C; return 10; }
    8
}
/// $04BF76: jsr $28cd4.l
pub fn i_04bf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BF7C);
    cpu.pc = 0x28CD4;
    16
}
/// $04BF7C: lea.l $4bf82(pc), a1
pub fn i_04bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BF82;
    16
}
/// $04BF80: move.l a1, (a6)
pub fn i_04bf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BF82: movea.l $c(a6), a0
pub fn i_04bf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF86;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04BF86: move.w $22(a0), $22(a6)
pub fn i_04bf86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF8C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04BF8C: move.w $24(a0), $24(a6)
pub fn i_04bf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF92;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04BF92: move.w $72(a6), d0
pub fn i_04bf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF96;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04BF96: add.w d0, $24(a6)
pub fn i_04bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BF9A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04BF9A: jsr $28108.l
pub fn i_04bf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BFA0);
    cpu.pc = 0x28108;
    16
}
/// $04BFA0: movea.l $c(a6), a0
pub fn i_04bfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFA4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04BFA4: btst.b #$7, $5a(a0)
pub fn i_04bfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFAA;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04BFAA: beq.w $4bfb4
pub fn i_04bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4BFB4; return 10; }
    8
}
/// $04BFAE: bset.b #$0, $5a(a6)
pub fn i_04bfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFB4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04BFB4: jsr $28d70.l
pub fn i_04bfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BFBA);
    cpu.pc = 0x28D70;
    16
}
/// $04BFBA: movea.l $c(a6), a0
pub fn i_04bfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFBE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04BFBE: cmpi.b #$2, $20(a0)
pub fn i_04bfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFC4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04BFC4: bne.w $4bfce
pub fn i_04bfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFC8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4BFCE; return 10; }
    8
}
/// $04BFC8: lea.l $4bff0(pc), a1
pub fn i_04bfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BFF0;
    16
}
/// $04BFCC: move.l a1, (a6)
pub fn i_04bfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BFCE: jsr $4c942(pc)
pub fn i_04bfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4BFD2);
    cpu.pc = 0x4C942;
    16
}
/// $04BFD2: movea.l #$ffffffff, a0
pub fn i_04bfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFD8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04BFD8: lea.l $290912.l, a0
pub fn i_04bfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFDE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04BFDE: jsr $5dd5c.l
pub fn i_04bfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4BFE4);
    cpu.pc = 0x5DD5C;
    16
}
/// $04BFE4: bcc.w $4bfee
pub fn i_04bfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFE8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4BFEE; return 10; }
    8
}
// ===== SetTaskHandler_04bfe8  ($04BFE8, 8 B, kernel/task_handlers.c)
/// $04BFE8: lea.l $4c58c(pc), a1
pub fn i_04bfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04BFEC: move.l a1, (a6)
pub fn i_04bfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04BFEE: rts
pub fn i_04bfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HatchFire_04bff0  ($04BFF0, 144 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04BFF0: move.b $21(a6), d0
pub fn i_04bff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFF4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04BFF4: andi.w #$ff, d0
pub fn i_04bff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04BFF8: movea.l #$29033c, a0
pub fn i_04bff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4BFFE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29033C;
    cpu.a[0] = v1;
    4
}
/// $04BFFE: lsl.w #$2, d0
pub fn i_04bffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C000;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04C000: movea.l (a0, d0.w), a0
pub fn i_04c000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C004;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C004: cmpa.l #$ffffffff, a0
pub fn i_04c004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C00A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04C00A: beq.w $4c014
pub fn i_04c00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C00E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C014; return 10; }
    8
}
/// $04C00E: jsr $28cd4.l
pub fn i_04c00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C014);
    cpu.pc = 0x28CD4;
    16
}
/// $04C014: lea.l $4c01a(pc), a1
pub fn i_04c014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C018;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C01A;
    16
}
/// $04C018: move.l a1, (a6)
pub fn i_04c018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C01A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C01A: movea.l $c(a6), a0
pub fn i_04c01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C01E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C01E: move.w $22(a0), $22(a6)
pub fn i_04c01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C024;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C024: move.w $24(a0), $24(a6)
pub fn i_04c024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C02A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C02A: move.w $72(a6), d0
pub fn i_04c02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C02E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C02E: add.w d0, $24(a6)
pub fn i_04c02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C032;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C032: jsr $28108.l
pub fn i_04c032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C038;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C038);
    cpu.pc = 0x28108;
    16
}
/// $04C038: movea.l $c(a6), a0
pub fn i_04c038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C03C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C03C: btst.b #$7, $5a(a0)
pub fn i_04c03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C042;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C042: beq.w $4c04c
pub fn i_04c042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C046;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C04C; return 10; }
    8
}
/// $04C046: bset.b #$0, $5a(a6)
pub fn i_04c046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C04C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C04C: jsr $28d70.l
pub fn i_04c04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C052);
    cpu.pc = 0x28D70;
    16
}
/// $04C052: movea.l $c(a6), a0
pub fn i_04c052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C056;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C056: cmpi.b #$2, $20(a0)
pub fn i_04c056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C05C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04C05C: beq.w $4c066
pub fn i_04c05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C060;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C066; return 10; }
    8
}
/// $04C060: lea.l $4c088(pc), a1
pub fn i_04c060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C064;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C088;
    16
}
/// $04C064: move.l a1, (a6)
pub fn i_04c064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C066;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C066: jsr $4c942(pc)
pub fn i_04c066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C06A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C06A);
    cpu.pc = 0x4C942;
    16
}
/// $04C06A: movea.l #$ffffffff, a0
pub fn i_04c06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C070;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C070: lea.l $290912.l, a0
pub fn i_04c070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C076;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C076: jsr $5dd5c.l
pub fn i_04c076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C07C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C07C);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C07C: bcc.w $4c086
pub fn i_04c07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C080;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C086; return 10; }
    8
}
// ===== SetTaskHandler_04c080  ($04C080, 8 B, kernel/task_handlers.c)
/// $04C080: lea.l $4c58c(pc), a1
pub fn i_04c080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C084;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C084: move.l a1, (a6)
pub fn i_04c084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C086;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C086: rts
pub fn i_04c086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C088;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HatchHold_04c088  ($04C088, 152 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C088: movea.l $c(a6), a0
pub fn i_04c088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C08C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C08C: move.w $22(a0), $22(a6)
pub fn i_04c08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C092;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C092: move.w $24(a0), $24(a6)
pub fn i_04c092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C098;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C098: move.w $72(a6), d0
pub fn i_04c098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C09C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C09C: add.w d0, $24(a6)
pub fn i_04c09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0A0;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C0A0: jsr $28108.l
pub fn i_04c0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C0A6);
    cpu.pc = 0x28108;
    16
}
/// $04C0A6: move.b $21(a6), d0
pub fn i_04c0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0AA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04C0AA: andi.w #$ff, d0
pub fn i_04c0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0AE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C0AE: movea.l #$290334, a0
pub fn i_04c0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0B4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x290334;
    cpu.a[0] = v1;
    4
}
/// $04C0B4: lsl.w #$2, d0
pub fn i_04c0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0B6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04C0B6: movea.l (a0, d0.w), a0
pub fn i_04c0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0BA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C0BA: cmpa.l #$ffffffff, a0
pub fn i_04c0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0C0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04C0C0: beq.w $4c0ca
pub fn i_04c0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C0CA; return 10; }
    8
}
/// $04C0C4: jsr $28cd4.l
pub fn i_04c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C0CA);
    cpu.pc = 0x28CD4;
    16
}
/// $04C0CA: movea.l $c(a6), a0
pub fn i_04c0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0CE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C0CE: btst.b #$7, $5a(a0)
pub fn i_04c0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0D4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C0D4: beq.w $4c0de
pub fn i_04c0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C0DE; return 10; }
    8
}
/// $04C0D8: bset.b #$0, $5a(a6)
pub fn i_04c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C0DE: jsr $28d70.l
pub fn i_04c0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C0E4);
    cpu.pc = 0x28D70;
    16
}
/// $04C0E4: movea.l $c(a6), a0
pub fn i_04c0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C0E8: tst.b $20(a0)
pub fn i_04c0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0EC;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C0EC: bne.w $4c0f6
pub fn i_04c0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C0F6; return 10; }
    8
}
/// $04C0F0: lea.l $4c128(pc), a1
pub fn i_04c0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C128;
    16
}
/// $04C0F4: move.l a1, (a6)
pub fn i_04c0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C0F6: cmpi.b #$2, $20(a0)
pub fn i_04c0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C0FC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04C0FC: bne.w $4c106
pub fn i_04c0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C100;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C106; return 10; }
    8
}
/// $04C100: lea.l $4bff0(pc), a1
pub fn i_04c100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C104;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BFF0;
    16
}
/// $04C104: move.l a1, (a6)
pub fn i_04c104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C106;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C106: jsr $4c942(pc)
pub fn i_04c106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C10A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C10A);
    cpu.pc = 0x4C942;
    16
}
/// $04C10A: movea.l #$ffffffff, a0
pub fn i_04c10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C110;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C110: lea.l $290912.l, a0
pub fn i_04c110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C116;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C116: jsr $5dd5c.l
pub fn i_04c116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C11C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C11C);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C11C: bcc.w $4c126
pub fn i_04c11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C120;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C126; return 10; }
    8
}
// ===== SetTaskHandler_04c120  ($04C120, 8 B, kernel/task_handlers.c)
/// $04C120: lea.l $4c58c(pc), a1
pub fn i_04c120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C124;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C124: move.l a1, (a6)
pub fn i_04c124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C126;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C126: rts
pub fn i_04c126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C128;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HatchClose_04c128  ($04C128, 128 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C128: move.b $21(a6), d0
pub fn i_04c128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C12C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04C12C: andi.w #$ff, d0
pub fn i_04c12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C130;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C130: movea.l #$29034c, a0
pub fn i_04c130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C136;
    cpu.ir = 0x207C;
    let v1: u32 = 0x29034C;
    cpu.a[0] = v1;
    4
}
/// $04C136: lsl.w #$2, d0
pub fn i_04c136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C138;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04C138: movea.l (a0, d0.w), a0
pub fn i_04c138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C13C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C13C: cmpa.l #$ffffffff, a0
pub fn i_04c13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C142;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04C142: beq.w $4c14c
pub fn i_04c142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C146;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C14C; return 10; }
    8
}
/// $04C146: jsr $28cd4.l
pub fn i_04c146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C14C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C14C);
    cpu.pc = 0x28CD4;
    16
}
/// $04C14C: lea.l $4c152(pc), a1
pub fn i_04c14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C150;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C152;
    16
}
/// $04C150: move.l a1, (a6)
pub fn i_04c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C152;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C152: movea.l $c(a6), a0
pub fn i_04c152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C156;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C156: move.w $22(a0), $22(a6)
pub fn i_04c156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C15C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C15C: move.w $24(a0), $24(a6)
pub fn i_04c15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C162;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C162: move.w $72(a6), d0
pub fn i_04c162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C166;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C166: add.w d0, $24(a6)
pub fn i_04c166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C16A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C16A: movea.l $c(a6), a0
pub fn i_04c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C16E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C16E: btst.b #$7, $5a(a0)
pub fn i_04c16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C174;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C174: beq.w $4c17e
pub fn i_04c174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C178;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C17E; return 10; }
    8
}
/// $04C178: bset.b #$0, $5a(a6)
pub fn i_04c178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C17E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C17E: jsr $28d70.l
pub fn i_04c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C184);
    cpu.pc = 0x28D70;
    16
}
/// $04C184: bcc.w $4c18e
pub fn i_04c184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C188;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C18E; return 10; }
    8
}
/// $04C188: lea.l $4bf1c(pc), a1
pub fn i_04c188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C18C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4BF1C;
    16
}
/// $04C18C: move.l a1, (a6)
pub fn i_04c18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C18E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C18E: jsr $4c942(pc)
pub fn i_04c18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C192;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C192);
    cpu.pc = 0x4C942;
    16
}
/// $04C192: movea.l #$ffffffff, a0
pub fn i_04c192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C198;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C198: lea.l $290912.l, a0
pub fn i_04c198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C19E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C19E: jsr $5dd5c.l
pub fn i_04c19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1A4);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C1A4: bcc.w $4c1ae
pub fn i_04c1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C1AE; return 10; }
    8
}
// ===== SetTaskHandler_04c1a8  ($04C1A8, 8 B, kernel/task_handlers.c)
/// $04C1A8: lea.l $4c58c(pc), a1
pub fn i_04c1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C1AC: move.l a1, (a6)
pub fn i_04c1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C1AE: rts
pub fn i_04c1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Shield_04c1b0  ($04C1B0, 190 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C1B0: move.w #$84, d1
pub fn i_04c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C1B4: jsr $236e.l
pub fn i_04c1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1BA);
    cpu.pc = 0x236E;
    16
}
/// $04C1BA: move.w #$19, $1c(a6)
pub fn i_04c1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C1C0: jsr $138fe.l
pub fn i_04c1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1C6);
    cpu.pc = 0x138FE;
    16
}
/// $04C1C6: bset.b #$1, $12(a6)
pub fn i_04c1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1CC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C1CC: move.w #$0, d0
pub fn i_04c1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C1D0: jsr $28134.l
pub fn i_04c1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1D6);
    cpu.pc = 0x28134;
    16
}
/// $04C1D6: andi.w #$ffe3, $38(a6)
pub fn i_04c1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1DC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C1DC: ori.w #$8, $38(a6)
pub fn i_04c1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1E2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C1E2: moveq #$0, d0
pub fn i_04c1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1E4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04C1E4: move.b d0, $20(a6)
pub fn i_04c1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1E8;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C1E8: move.b d0, $21(a6)
pub fn i_04c1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1EC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C1EC: lea.l $2bd15c.l, a0
pub fn i_04c1ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD15C;
    20
}
/// $04C1F2: jsr $799de.l
pub fn i_04c1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C1F8);
    cpu.pc = 0x799DE;
    16
}
/// $04C1F8: move.w d0, $66(a6)
pub fn i_04c1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C1FC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C1FC: jsr $267e2.l
pub fn i_04c1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C202;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C202);
    cpu.pc = 0x267E2;
    16
}
/// $04C202: lea.l $4c208(pc), a1
pub fn i_04c202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C206;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C208;
    16
}
/// $04C206: move.l a1, (a6)
pub fn i_04c206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C208;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C208: movea.l $c(a6), a0
pub fn i_04c208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C20C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C20C: move.w $22(a0), $22(a6)
pub fn i_04c20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C212;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C212: move.w $24(a0), $24(a6)
pub fn i_04c212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C218;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C218: lea.l $29030c.l, a1
pub fn i_04c218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C21E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29030C;
    20
}
/// $04C21E: jsr $4c9ae(pc)
pub fn i_04c21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C222;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C222);
    cpu.pc = 0x4C9AE;
    16
}
/// $04C222: movea.l $c(a6), a0
pub fn i_04c222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C226;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C226: btst.b #$7, $5a(a0)
pub fn i_04c226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C22C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C22C: beq.w $4c236
pub fn i_04c22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C230;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C236; return 10; }
    8
}
/// $04C230: bset.b #$0, $5a(a6)
pub fn i_04c230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C236;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C236: jsr $28d70.l
pub fn i_04c236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C23C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C23C);
    cpu.pc = 0x28D70;
    16
}
/// $04C23C: movea.l $c(a6), a0
pub fn i_04c23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C240;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C240: cmpi.b #$2, $20(a0)
pub fn i_04c240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C246;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04C246: bne.w $4c250
pub fn i_04c246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C24A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C250; return 10; }
    8
}
/// $04C24A: lea.l $4c276(pc), a1
pub fn i_04c24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C24E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C276;
    16
}
/// $04C24E: move.l a1, (a6)
pub fn i_04c24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C250;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C250: jsr $4c942(pc)
pub fn i_04c250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C254;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C254);
    cpu.pc = 0x4C942;
    16
}
/// $04C254: jsr $4c958(pc)
pub fn i_04c254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C258;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C258);
    cpu.pc = 0x4C958;
    16
}
/// $04C258: movea.l #$ffffffff, a0
pub fn i_04c258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C25E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C25E: lea.l $290912.l, a0
pub fn i_04c25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C264;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C264: jsr $5dd5c.l
pub fn i_04c264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C26A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C26A);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C26A: bcc.w $4c274
pub fn i_04c26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C26E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C274; return 10; }
    8
}
// ===== SetTaskHandler_04c26e  ($04C26E, 8 B, kernel/task_handlers.c)
/// $04C26E: lea.l $4c58c(pc), a1
pub fn i_04c26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C272;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C272: move.l a1, (a6)
pub fn i_04c272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C274;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C274: rts
pub fn i_04c274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C276;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_ShieldFire_04c276  ($04C276, 102 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C276: movea.l $c(a6), a0
pub fn i_04c276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C27A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C27A: move.w $22(a0), $22(a6)
pub fn i_04c27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C280;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C280: move.w $24(a0), $24(a6)
pub fn i_04c280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C286;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04C286: movea.l $c(a6), a0
pub fn i_04c286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C28A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C28A: btst.b #$7, $5a(a0)
pub fn i_04c28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C290;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C290: beq.w $4c29a
pub fn i_04c290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C294;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C29A; return 10; }
    8
}
/// $04C294: bset.b #$0, $5a(a6)
pub fn i_04c294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C29A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C29A: lea.l $29030c.l, a1
pub fn i_04c29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2A0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x29030C;
    20
}
/// $04C2A0: jsr $4c9ae(pc)
pub fn i_04c2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C2A4);
    cpu.pc = 0x4C9AE;
    16
}
/// $04C2A4: jsr $28d70.l
pub fn i_04c2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C2AA);
    cpu.pc = 0x28D70;
    16
}
/// $04C2AA: movea.l $c(a6), a0
pub fn i_04c2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2AE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C2AE: cmpi.b #$2, $20(a0)
pub fn i_04c2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2B4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $04C2B4: beq.w $4c2be
pub fn i_04c2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C2BE; return 10; }
    8
}
/// $04C2B8: lea.l $4c208(pc), a1
pub fn i_04c2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C208;
    16
}
/// $04C2BC: move.l a1, (a6)
pub fn i_04c2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C2BE: jsr $4c942(pc)
pub fn i_04c2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C2C2);
    cpu.pc = 0x4C942;
    16
}
/// $04C2C2: jsr $4c958(pc)
pub fn i_04c2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C2C6);
    cpu.pc = 0x4C958;
    16
}
/// $04C2C6: movea.l #$ffffffff, a0
pub fn i_04c2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2CC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C2CC: lea.l $290912.l, a0
pub fn i_04c2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C2D2: jsr $5dd5c.l
pub fn i_04c2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C2D8);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C2D8: bcc.w $4c2e2
pub fn i_04c2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C2E2; return 10; }
    8
}
// ===== SetTaskHandler_04c2dc  ($04C2DC, 8 B, kernel/task_handlers.c)
/// $04C2DC: lea.l $4c58c(pc), a1
pub fn i_04c2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C2E0: move.l a1, (a6)
pub fn i_04c2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C2E2: rts
pub fn i_04c2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_RiderB_Init_04c2e4  ($04C2E4, 80 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C2E4: clr.b $20(a6)
pub fn i_04c2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2E8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C2E8: clr.b $21(a6)
pub fn i_04c2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2EC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C2EC: move.w #$1, $66(a6)
pub fn i_04c2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C2F2: move.w #$38, d1
pub fn i_04c2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2F6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C2F6: jsr $236e.l
pub fn i_04c2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C2FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C2FC);
    cpu.pc = 0x236E;
    16
}
/// $04C2FC: jsr $5e7c0.l
pub fn i_04c2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C302;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C302);
    cpu.pc = 0x5E7C0;
    16
}
/// $04C302: jsr $267e2.l
pub fn i_04c302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C308);
    cpu.pc = 0x267E2;
    16
}
/// $04C308: addi.w #$a, $22(a6)
pub fn i_04c308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C30E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C30E: subi.w #$b, $24(a6)
pub fn i_04c30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C314;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xB, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C314: lea.l $290c74.l, a0
pub fn i_04c314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C31A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290C74;
    20
}
/// $04C31A: move.l a0, $48(a6)
pub fn i_04c31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C31E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04C31E: move.w #$2000, d0
pub fn i_04c31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C322;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C322: jsr $28134.l
pub fn i_04c322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C328;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C328);
    cpu.pc = 0x28134;
    16
}
/// $04C328: andi.w #$ffe3, $38(a6)
pub fn i_04c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C32E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C32E: ori.w #$8, $38(a6)
pub fn i_04c32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C334;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== GunPlatform_RiderB_Idle_04c334  ($04C334, 52 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C334: lea.l $28e312.l, a0
pub fn i_04c334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C33A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E312;
    20
}
/// $04C33A: jsr $28cd4.l
pub fn i_04c33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C340);
    cpu.pc = 0x28CD4;
    16
}
/// $04C340: lea.l $4c346(pc), a1
pub fn i_04c340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C344;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C346;
    16
}
/// $04C344: move.l a1, (a6)
pub fn i_04c344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C346;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C346: jsr $2783a.l
pub fn i_04c346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C34C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C34C);
    cpu.pc = 0x2783A;
    16
}
/// $04C34C: jsr $28d70.l
pub fn i_04c34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C352;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C352);
    cpu.pc = 0x28D70;
    16
}
/// $04C352: movea.l $c(a6), a0
pub fn i_04c352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C356;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C356: tst.b $20(a0)
pub fn i_04c356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C35A;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C35A: beq.w $4c364
pub fn i_04c35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C35E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C364; return 10; }
    8
}
/// $04C35E: lea.l $4c368(pc), a1
pub fn i_04c35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C362;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C368;
    16
}
/// $04C362: move.l a1, (a6)
pub fn i_04c362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C364;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C364: bra.w $4c416
pub fn i_04c364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C368;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C416;
    10
}
// ===== GunPlatform_RiderB_Alert_04c368  ($04C368, 12 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C368: lea.l $28e8f4.l, a0
pub fn i_04c368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C36E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E8F4;
    20
}
/// $04C36E: jsr $28cd4.l
pub fn i_04c36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C374);
    cpu.pc = 0x28CD4;
    16
}
// ===== GunPlatform_RiderB_Anim_04c374  ($04C374, 64 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C374: lea.l $4c37a(pc), a1
pub fn i_04c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C378;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C37A;
    16
}
/// $04C378: move.l a1, (a6)
pub fn i_04c378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C37A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C37A: jsr $2783a.l
pub fn i_04c37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C380;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C380);
    cpu.pc = 0x2783A;
    16
}
/// $04C380: jsr $28d70.l
pub fn i_04c380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C386;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C386);
    cpu.pc = 0x28D70;
    16
}
/// $04C386: bcc.w $4c3b0
pub fn i_04c386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C38A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C3B0; return 10; }
    8
}
/// $04C38A: movea.l $c(a6), a0
pub fn i_04c38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C38E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C38E: tst.b $20(a0)
pub fn i_04c38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C392;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C392: bne.w $4c39c
pub fn i_04c392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C396;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C39C; return 10; }
    8
}
/// $04C396: lea.l $4c3ea(pc), a1
pub fn i_04c396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C39A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C3EA;
    16
}
/// $04C39A: move.l a1, (a6)
pub fn i_04c39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C39C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C39C: movea.l $c(a6), a0
pub fn i_04c39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3A0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C3A0: cmpi.b #$3, $20(a0)
pub fn i_04c3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3A6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $04C3A6: bne.w $4c3b0
pub fn i_04c3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3AA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C3B0; return 10; }
    8
}
/// $04C3AA: lea.l $4c3b4(pc), a1
pub fn i_04c3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C3B4;
    16
}
/// $04C3AE: move.l a1, (a6)
pub fn i_04c3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C3B0: bra.w $4c416
pub fn i_04c3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C416;
    10
}
// ===== GunPlatform_RiderB_Fire_04c3b4  ($04C3B4, 54 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C3B4: lea.l $28ec54.l, a0
pub fn i_04c3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28EC54;
    20
}
/// $04C3BA: jsr $28cd4.l
pub fn i_04c3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C3C0);
    cpu.pc = 0x28CD4;
    16
}
/// $04C3C0: lea.l $4c3c6(pc), a1
pub fn i_04c3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C3C6;
    16
}
/// $04C3C4: move.l a1, (a6)
pub fn i_04c3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C3C6: jsr $2783a.l
pub fn i_04c3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C3CC);
    cpu.pc = 0x2783A;
    16
}
/// $04C3CC: jsr $28d70.l
pub fn i_04c3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C3D2);
    cpu.pc = 0x28D70;
    16
}
/// $04C3D2: movea.l $c(a6), a0
pub fn i_04c3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3D6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C3D6: cmpi.b #$3, $20(a0)
pub fn i_04c3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3DC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $04C3DC: beq.w $4c3e6
pub fn i_04c3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C3E6; return 10; }
    8
}
/// $04C3E0: lea.l $4c374(pc), a1
pub fn i_04c3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C374;
    16
}
/// $04C3E4: move.l a1, (a6)
pub fn i_04c3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C3E6: bra.w $4c416
pub fn i_04c3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3EA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C416;
    10
}
// ===== GunPlatform_RiderB_Return_04c3ea  ($04C3EA, 88 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C3EA: lea.l $28e934.l, a0
pub fn i_04c3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E934;
    20
}
/// $04C3F0: jsr $28cd4.l
pub fn i_04c3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C3F6);
    cpu.pc = 0x28CD4;
    16
}
/// $04C3F6: lea.l $4c3fc(pc), a1
pub fn i_04c3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C3FC;
    16
}
/// $04C3FA: move.l a1, (a6)
pub fn i_04c3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C3FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C3FC: jsr $2783a.l
pub fn i_04c3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C402;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C402);
    cpu.pc = 0x2783A;
    16
}
/// $04C402: jsr $28d70.l
pub fn i_04c402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C408;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C408);
    cpu.pc = 0x28D70;
    16
}
/// $04C408: bcc.w $4c412
pub fn i_04c408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C40C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C412; return 10; }
    8
}
/// $04C40C: lea.l $4c334(pc), a1
pub fn i_04c40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C410;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C334;
    16
}
/// $04C410: move.l a1, (a6)
pub fn i_04c410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C412;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C412: bra.w $4c416
pub fn i_04c412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C416;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C416;
    10
}
/// $04C416: jsr $2870a.l
pub fn i_04c416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C41C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C41C);
    cpu.pc = 0x2870A;
    16
}
/// $04C41C: bcc.w $4c428
pub fn i_04c41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C420;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C428; return 10; }
    8
}
/// $04C420: lea.l $4a146.l, a1
pub fn i_04c420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C426;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4A146;
    20
}
/// $04C426: move.l a1, (a6)
pub fn i_04c426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C428;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C428: jsr $4c942(pc)
pub fn i_04c428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C42C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C42C);
    cpu.pc = 0x4C942;
    16
}
/// $04C42C: movea.l #$ffffffff, a0
pub fn i_04c42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C432;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C432: lea.l $290912.l, a0
pub fn i_04c432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C438;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C438: jsr $5dd5c.l
pub fn i_04c438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C43E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C43E);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C43E: bcc.w $4c448
pub fn i_04c43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C442;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C448; return 10; }
    8
}
// ===== SetTaskHandler_04c442  ($04C442, 8 B, kernel/task_handlers.c)
/// $04C442: lea.l $4c58c(pc), a1
pub fn i_04c442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C446;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C446: move.l a1, (a6)
pub fn i_04c446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C448;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C448: rts
pub fn i_04c448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C44A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_RiderA_Init_04c44a  ($04C44A, 68 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C44A: clr.b $20(a6)
pub fn i_04c44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C44E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C44E: clr.b $21(a6)
pub fn i_04c44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C452;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C452: move.w #$1, $66(a6)
pub fn i_04c452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C458;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C458: move.w #$38, d1
pub fn i_04c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C45C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C45C: jsr $236e.l
pub fn i_04c45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C462;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C462);
    cpu.pc = 0x236E;
    16
}
/// $04C462: jsr $5e7c0.l
pub fn i_04c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C468);
    cpu.pc = 0x5E7C0;
    16
}
/// $04C468: jsr $267e2.l
pub fn i_04c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C46E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C46E);
    cpu.pc = 0x267E2;
    16
}
/// $04C46E: lea.l $290c20.l, a0
pub fn i_04c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C474;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290C20;
    20
}
/// $04C474: move.l a0, $48(a6)
pub fn i_04c474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C478;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04C478: move.w #$2000, d0
pub fn i_04c478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C47C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C47C: jsr $28134.l
pub fn i_04c47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C482);
    cpu.pc = 0x28134;
    16
}
/// $04C482: andi.w #$ffe3, $38(a6)
pub fn i_04c482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C488;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C488: ori.w #$8, $38(a6)
pub fn i_04c488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C48E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
// ===== GunPlatform_RiderA_Idle_04c48e  ($04C48E, 52 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C48E: lea.l $28e6b0.l, a0
pub fn i_04c48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C494;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6B0;
    20
}
/// $04C494: jsr $28cd4.l
pub fn i_04c494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C49A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C49A);
    cpu.pc = 0x28CD4;
    16
}
/// $04C49A: lea.l $4c4a0(pc), a1
pub fn i_04c49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C49E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C4A0;
    16
}
/// $04C49E: move.l a1, (a6)
pub fn i_04c49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C4A0: jsr $2783a.l
pub fn i_04c4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4A6);
    cpu.pc = 0x2783A;
    16
}
/// $04C4A6: jsr $28d70.l
pub fn i_04c4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4AC);
    cpu.pc = 0x28D70;
    16
}
/// $04C4AC: movea.l $c(a6), a0
pub fn i_04c4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4B0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C4B0: tst.b $20(a0)
pub fn i_04c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4B4;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C4B4: beq.w $4c4be
pub fn i_04c4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C4BE; return 10; }
    8
}
/// $04C4B8: lea.l $4c4c2(pc), a1
pub fn i_04c4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C4C2;
    16
}
/// $04C4BC: move.l a1, (a6)
pub fn i_04c4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C4BE: bra.w $4c544
pub fn i_04c4be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4C2;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C544;
    10
}
// ===== GunPlatform_RiderA_Alert_04c4c2  ($04C4C2, 12 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C4C2: lea.l $28e6bc.l, a0
pub fn i_04c4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6BC;
    20
}
/// $04C4C8: jsr $28cd4.l
pub fn i_04c4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4CE);
    cpu.pc = 0x28CD4;
    16
}
// ===== GunPlatform_RiderA_Anim_04c4ce  ($04C4CE, 64 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C4CE: lea.l $4c4d4(pc), a1
pub fn i_04c4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C4D4;
    16
}
/// $04C4D2: move.l a1, (a6)
pub fn i_04c4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C4D4: jsr $2783a.l
pub fn i_04c4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4DA);
    cpu.pc = 0x2783A;
    16
}
/// $04C4DA: jsr $28d70.l
pub fn i_04c4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C4E0);
    cpu.pc = 0x28D70;
    16
}
/// $04C4E0: bcc.w $4c50a
pub fn i_04c4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C50A; return 10; }
    8
}
/// $04C4E4: movea.l $c(a6), a0
pub fn i_04c4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4E8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C4E8: tst.b $20(a0)
pub fn i_04c4e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4EC;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C4EC: bne.w $4c4f6
pub fn i_04c4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4F0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C4F6; return 10; }
    8
}
/// $04C4F0: lea.l $4c48e(pc), a1
pub fn i_04c4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C48E;
    16
}
/// $04C4F4: move.l a1, (a6)
pub fn i_04c4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C4F6: movea.l $c(a6), a0
pub fn i_04c4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C4FA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C4FA: cmpi.b #$3, $20(a0)
pub fn i_04c4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C500;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $04C500: bne.w $4c50a
pub fn i_04c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C504;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C50A; return 10; }
    8
}
/// $04C504: lea.l $4c50e(pc), a1
pub fn i_04c504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C508;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C50E;
    16
}
/// $04C508: move.l a1, (a6)
pub fn i_04c508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C50A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C50A: bra.w $4c544
pub fn i_04c50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C50E;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C544;
    10
}
// ===== GunPlatform_RiderA_Fire_04c50e  ($04C50E, 98 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C50E: lea.l $28e6c8.l, a0
pub fn i_04c50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C514;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28E6C8;
    20
}
/// $04C514: jsr $28cd4.l
pub fn i_04c514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C51A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C51A);
    cpu.pc = 0x28CD4;
    16
}
/// $04C51A: lea.l $4c520(pc), a1
pub fn i_04c51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C51E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C520;
    16
}
/// $04C51E: move.l a1, (a6)
pub fn i_04c51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C520;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C520: jsr $2783a.l
pub fn i_04c520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C526;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C526);
    cpu.pc = 0x2783A;
    16
}
/// $04C526: jsr $28d70.l
pub fn i_04c526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C52C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C52C);
    cpu.pc = 0x28D70;
    16
}
/// $04C52C: movea.l $c(a6), a0
pub fn i_04c52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C530;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C530: cmpi.b #$3, $20(a0)
pub fn i_04c530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C536;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $04C536: beq.w $4c540
pub fn i_04c536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C53A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C540; return 10; }
    8
}
/// $04C53A: lea.l $4c4ce(pc), a1
pub fn i_04c53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C53E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C4CE;
    16
}
/// $04C53E: move.l a1, (a6)
pub fn i_04c53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C540;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C540: bra.w $4c544
pub fn i_04c540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C544;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C544;
    10
}
/// $04C544: jsr $2870a.l
pub fn i_04c544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C54A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C54A);
    cpu.pc = 0x2870A;
    16
}
/// $04C54A: bcc.w $4c556
pub fn i_04c54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C54E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C556; return 10; }
    8
}
/// $04C54E: lea.l $4a154.l, a1
pub fn i_04c54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C554;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x4A154;
    20
}
/// $04C554: move.l a1, (a6)
pub fn i_04c554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C556;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C556: jsr $4c942(pc)
pub fn i_04c556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C55A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C55A);
    cpu.pc = 0x4C942;
    16
}
/// $04C55A: movea.l #$ffffffff, a0
pub fn i_04c55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C560;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C560: lea.l $290912.l, a0
pub fn i_04c560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C566;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C566: jsr $5dd5c.l
pub fn i_04c566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C56C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C56C);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C56C: bcc.w $4c576
pub fn i_04c56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C570;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C576; return 10; }
    8
}
// ===== SetTaskHandler_04c570  ($04C570, 8 B, kernel/task_handlers.c)
/// $04C570: lea.l $4c58c(pc), a1
pub fn i_04c570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C574;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C574: move.l a1, (a6)
pub fn i_04c574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C576;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C576: rts
pub fn i_04c576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C578;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_MarkDead_04c578  ($04C578, 6 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C578: move.b #$ff, $20(a6)
pub fn i_04c578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C57E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Jsr5B6ThenJmpScheduler_04c57e  ($04C57E, 14 B, kernel/jsr5b6_jmp518.c)
/// $04C57E: jsr $5b6.l
pub fn i_04c57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C584);
    cpu.pc = 0x5B6;
    16
}
/// $04C584: jmp $518.l
pub fn i_04c584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C58A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== GunPlatform_FreeClearBit1_04c58c  ($04C58C, 22 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C58C: btst.b #$1, $12(a6)
pub fn i_04c58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C592;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C592: beq.w $4c59c
pub fn i_04c592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C596;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C59C; return 10; }
    8
}
/// $04C596: bclr.b #$1, $12(a6)
pub fn i_04c596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C59C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C59C: jmp $518.l
pub fn i_04c59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5A2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== GunPlatform_Nop_04c5a2  ($04C5A2, 2 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C5A2: rts
pub fn i_04c5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Destroyed_04c5a4  ($04C5A4, 90 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C5A4: clr.b $78(a6)
pub fn i_04c5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5A8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C5A8: jsr $4ca98(pc)
pub fn i_04c5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C5AC);
    cpu.pc = 0x4CA98;
    16
}
/// $04C5AC: move.w #$1023, d0
pub fn i_04c5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C5B0: jsr $2352.l
pub fn i_04c5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5B6);
    cpu.pc = 0x2352;
    16
}
/// $04C5B6: lea.l $290ea4.l, a1
pub fn i_04c5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EA4;
    20
}
/// $04C5BC: jsr $77c7e.l
pub fn i_04c5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5C2);
    cpu.pc = 0x77C7E;
    16
}
/// $04C5C2: lea.l $290eb6.l, a1
pub fn i_04c5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5C8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EB6;
    20
}
/// $04C5C8: jsr $77c7e.l
pub fn i_04c5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5CE);
    cpu.pc = 0x77C7E;
    16
}
/// $04C5CE: move.w #$a, $88(a6)
pub fn i_04c5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5D4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C5D4: lea.l $4c5da(pc), a1
pub fn i_04c5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C5DA;
    16
}
/// $04C5D8: move.l a1, (a6)
pub fn i_04c5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C5DA: jsr $2783a.l
pub fn i_04c5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5E0);
    cpu.pc = 0x2783A;
    16
}
/// $04C5E0: jsr $28d70.l
pub fn i_04c5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C5E6);
    cpu.pc = 0x28D70;
    16
}
/// $04C5E6: subq.w #$1, $88(a6)
pub fn i_04c5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5EA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C5EA: bne.w $4c604
pub fn i_04c5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C604; return 10; }
    8
}
/// $04C5EE: move.b #$ff, $20(a6)
pub fn i_04c5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C5F4: jsr $4ca4a(pc)
pub fn i_04c5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C5F8);
    cpu.pc = 0x4CA4A;
    16
}
/// $04C5F8: bclr.b #$1, $12(a6)
pub fn i_04c5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C5FE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetTaskHandler_04c5fe  ($04C5FE, 8 B, kernel/task_handlers.c)
/// $04C5FE: lea.l $4c68a(pc), a1
pub fn i_04c5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C602;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C68A;
    16
}
/// $04C602: move.l a1, (a6)
pub fn i_04c602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C604;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C604: rts
pub fn i_04c604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C606;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_DestroyedWithWreck_04c606  ($04C606, 124 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C606: clr.b $78(a6)
pub fn i_04c606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C60A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C60A: jsr $4ca98(pc)
pub fn i_04c60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C60E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C60E);
    cpu.pc = 0x4CA98;
    16
}
/// $04C60E: bclr.b #$1, $12(a6)
pub fn i_04c60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C614;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C614: lea.l $290374.l, a1
pub fn i_04c614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C61A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290374;
    20
}
/// $04C61A: move.w $70(a6), d1
pub fn i_04c61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C61E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C61E: add.w d1, d1
pub fn i_04c61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C620;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C620: add.w d1, d1
pub fn i_04c620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C622;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C622: movea.l (a1, d1.w), a0
pub fn i_04c622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C626;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C626: cmpa.l #$ffffffff, a0
pub fn i_04c626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C62C;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $04C62C: beq.w $4c5a4
pub fn i_04c62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C630;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C5A4; return 10; }
    8
}
/// $04C630: jsr $28cd4.l
pub fn i_04c630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C636);
    cpu.pc = 0x28CD4;
    16
}
/// $04C636: lea.l $4c68a(pc), a1
pub fn i_04c636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C63A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C68A;
    16
}
/// $04C63A: jsr $4ae.l
pub fn i_04c63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C640;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C640);
    cpu.pc = 0x4AE;
    16
}
/// $04C640: jsr $5dd02.l
pub fn i_04c640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C646);
    cpu.pc = 0x5DD02;
    16
}
/// $04C646: move.w $38(a6), d0
pub fn i_04c646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C64A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C64A: subq.w #$8, d0
pub fn i_04c64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C64C;
    cpu.ir = 0x5140;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04C64C: move.w d0, $38(a0)
pub fn i_04c64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C650;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C650: lea.l $4c656(pc), a1
pub fn i_04c650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C654;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C656;
    16
}
/// $04C654: move.l a1, (a6)
pub fn i_04c654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C656;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C656: jsr $2783a.l
pub fn i_04c656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C65C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C65C);
    cpu.pc = 0x2783A;
    16
}
/// $04C65C: jsr $28d70.l
pub fn i_04c65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C662);
    cpu.pc = 0x28D70;
    16
}
/// $04C662: bcc.w $4c66c
pub fn i_04c662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C666;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C66C; return 10; }
    8
}
/// $04C666: lea.l $4c68a(pc), a1
pub fn i_04c666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C66A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C68A;
    16
}
/// $04C66A: move.l a1, (a6)
pub fn i_04c66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C66C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C66C: movea.l #$ffffffff, a0
pub fn i_04c66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C672;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C672: lea.l $290912.l, a0
pub fn i_04c672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C678;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C678: jsr $5dd5c.l
pub fn i_04c678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C67E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C67E);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C67E: bcc.w $4c688
pub fn i_04c67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C682;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C688; return 10; }
    8
}
// ===== SetTaskHandler_04c682  ($04C682, 8 B, kernel/task_handlers.c)
/// $04C682: lea.l $4c58c(pc), a1
pub fn i_04c682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C686;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C686: move.l a1, (a6)
pub fn i_04c686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C688;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C688: rts
pub fn i_04c688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C68A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Wreck_04c68a  ($04C68A, 66 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C68A: clr.b $78(a6)
pub fn i_04c68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C68E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C68E: move.w #$84, d1
pub fn i_04c68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C692;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C692: jsr $236e.l
pub fn i_04c692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C698);
    cpu.pc = 0x236E;
    16
}
/// $04C698: lea.l $290864.l, a0
pub fn i_04c698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C69E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290864;
    20
}
/// $04C69E: jsr $28cd4.l
pub fn i_04c69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6A4);
    cpu.pc = 0x28CD4;
    16
}
/// $04C6A4: lea.l $4c6aa(pc), a1
pub fn i_04c6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C6AA;
    16
}
/// $04C6A8: move.l a1, (a6)
pub fn i_04c6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C6AA: jsr $2783a.l
pub fn i_04c6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6B0);
    cpu.pc = 0x2783A;
    16
}
/// $04C6B0: jsr $28d70.l
pub fn i_04c6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6B6);
    cpu.pc = 0x28D70;
    16
}
/// $04C6B6: movea.l #$ffffffff, a0
pub fn i_04c6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6BC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C6BC: lea.l $290912.l, a0
pub fn i_04c6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290912;
    20
}
/// $04C6C2: jsr $5dd5c.l
pub fn i_04c6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6C8);
    cpu.pc = 0x5DD5C;
    16
}
/// $04C6C8: bcc.w $4c6d2
pub fn i_04c6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C6D2; return 10; }
    8
}
// ===== SetTaskHandler_04c6cc  ($04C6CC, 8 B, kernel/task_handlers.c)
/// $04C6CC: lea.l $4c58c(pc), a1
pub fn i_04c6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C6D0: move.l a1, (a6)
pub fn i_04c6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C6D2: rts
pub fn i_04c6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HitCheckUnlessDead_04c6d4  ($04C6D4, 14 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C6D4: cmpi.b #$ff, $20(a6)
pub fn i_04c6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6DA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04C6DA: bne.w $4c6ea
pub fn i_04c6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6DE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C6EA; return 10; }
    8
}
/// $04C6DE: jsr $4ca4a(pc)
pub fn i_04c6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4C6E2);
    cpu.pc = 0x4CA4A;
    16
}
// ===== SetTaskHandler_04c6e2  ($04C6E2, 8 B, kernel/task_handlers.c)
/// $04C6E2: lea.l $4c606(pc), a1
pub fn i_04c6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C606;
    16
}
/// $04C6E6: move.l a1, (a6)
pub fn i_04c6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C6E8: rts
pub fn i_04c6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_HitCheck_04c6ea  ($04C6EA, 128 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C6EA: clr.b $78(a6)
pub fn i_04c6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6EE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04C6EE: jsr $2870a.l
pub fn i_04c6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C6F4);
    cpu.pc = 0x2870A;
    16
}
/// $04C6F4: bcc.w $4c75a
pub fn i_04c6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C75A; return 10; }
    8
}
/// $04C6F8: move.b #$1, $78(a6)
pub fn i_04c6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C6FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C6FE: bclr.b #$3, $13(a6)
pub fn i_04c6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C704;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C704: cmpi.b #$1, $21(a6)
pub fn i_04c704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C70A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04C70A: beq.w $4c75a
pub fn i_04c70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C70E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C75A; return 10; }
    8
}
/// $04C70E: cmpi.w #$32, $66(a6)
pub fn i_04c70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C714;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x32, Size::Word, false);
    16
}
/// $04C714: bge.w $4c75a
pub fn i_04c714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C718;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x4C75A; return 10; }
    8
}
/// $04C718: move.b #$1, $21(a6)
pub fn i_04c718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C71E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C71E: move.w #$1027, d0
pub fn i_04c71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C722;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C722: jsr $2352.l
pub fn i_04c722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C728;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C728);
    cpu.pc = 0x2352;
    16
}
/// $04C728: cmpi.b #$1, $58(a6)
pub fn i_04c728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C72E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04C72E: beq.w $4c742
pub fn i_04c72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C732;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C742; return 10; }
    8
}
/// $04C732: lea.l $290eec.l, a1
pub fn i_04c732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C738;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EEC;
    20
}
/// $04C738: jsr $77c7e.l
pub fn i_04c738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C73E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C73E);
    cpu.pc = 0x77C7E;
    16
}
/// $04C73E: bra.w $4c74e
pub fn i_04c73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C742;
    cpu.ir = 0x6000;
    cpu.pc = 0x4C74E;
    10
}
/// $04C742: lea.l $290ec8.l, a1
pub fn i_04c742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C748;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EC8;
    20
}
/// $04C748: jsr $77c7e.l
pub fn i_04c748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C74E);
    cpu.pc = 0x77C7E;
    16
}
/// $04C74E: lea.l $290eda.l, a1
pub fn i_04c74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C754;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EDA;
    20
}
/// $04C754: jsr $77c7e.l
pub fn i_04c754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C75A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C75A);
    cpu.pc = 0x77C7E;
    16
}
/// $04C75A: jsr $28758.l
pub fn i_04c75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C760);
    cpu.pc = 0x28758;
    16
}
/// $04C760: bcc.w $4c770
pub fn i_04c760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C764;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C770; return 10; }
    8
}
/// $04C764: lea.l $4c5a4(pc), a1
pub fn i_04c764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C768;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C5A4;
    16
}
/// $04C768: move.l a1, (a6)
pub fn i_04c768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C76A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== SetXN_04c76a  ($04C76A, 6 B, thunks/ccr_helpers.c)
/// $04C76A: ori.b #$11, ccr
pub fn i_04c76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C76E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04C76E: rts
pub fn i_04c76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C770;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04c770  ($04C770, 6 B, thunks/ccr_helpers.c)
/// $04C770: andi.b #$ee, ccr
pub fn i_04c770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C774;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04C774: rts
pub fn i_04c774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C776;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_Gun_04c776  ($04C776, 180 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C776: move.w #$84, d1
pub fn i_04c776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C77A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x84;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C77A: jsr $236e.l
pub fn i_04c77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C780);
    cpu.pc = 0x236E;
    16
}
/// $04C780: move.w #$19, $1c(a6)
pub fn i_04c780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C786;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C786: jsr $138fe.l
pub fn i_04c786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C78C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C78C);
    cpu.pc = 0x138FE;
    16
}
/// $04C78C: move.w $70(a6), d0
pub fn i_04c78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C790;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C790: andi.w #$3, d0
pub fn i_04c790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C794;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C794: lsl.w #$2, d0
pub fn i_04c794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C796;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $04C796: lea.l $290efe.l, a1
pub fn i_04c796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C79C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290EFE;
    20
}
/// $04C79C: move.w (a1, d0.w), $74(a6)
pub fn i_04c79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7A2;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad4, v3 as u16);
    22
}
/// $04C7A2: move.w $2(a1, d0.w), $76(a6)
pub fn i_04c7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7A8;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    22
}
/// $04C7A8: move.l #$290d70, $4c(a6)
pub fn i_04c7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7B0;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x290D70;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04C7B0: lea.l $290876.l, a0
pub fn i_04c7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7B6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290876;
    20
}
/// $04C7B6: jsr $28cd4.l
pub fn i_04c7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C7BC);
    cpu.pc = 0x28CD4;
    16
}
/// $04C7BC: lea.l $4c7c2(pc), a1
pub fn i_04c7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C7C2;
    16
}
/// $04C7C0: move.l a1, (a6)
pub fn i_04c7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C7C2: movea.l $c(a6), a0
pub fn i_04c7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7C6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C7C6: cmpi.b #$1, $21(a0)
pub fn i_04c7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7CC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $04C7CC: bne.w $4c7d6
pub fn i_04c7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C7D6; return 10; }
    8
}
/// $04C7D0: lea.l $4c832(pc), a1
pub fn i_04c7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C832;
    16
}
/// $04C7D4: move.l a1, (a6)
pub fn i_04c7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C7D6: jsr $5e506.l
pub fn i_04c7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C7DC);
    cpu.pc = 0x5E506;
    16
}
/// $04C7DC: tst.b $78(a0)
pub fn i_04c7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7E0;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04C7E0: beq.w $4c7ea
pub fn i_04c7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C7EA; return 10; }
    8
}
/// $04C7E4: bset.b #$0, $5a(a6)
pub fn i_04c7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7EA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C7EA: move.w $74(a6), d0
pub fn i_04c7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C7EE: add.w d0, $22(a6)
pub fn i_04c7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7F2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C7F2: move.w $76(a6), d0
pub fn i_04c7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7F6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C7F6: add.w d0, $24(a6)
pub fn i_04c7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C7FA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C7FA: jsr $28d70.l
pub fn i_04c7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C800;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C800);
    cpu.pc = 0x28D70;
    16
}
/// $04C800: jsr $283ca.l
pub fn i_04c800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C806);
    cpu.pc = 0x283CA;
    16
}
/// $04C806: jsr $283d8.l
pub fn i_04c806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C80C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C80C);
    cpu.pc = 0x283D8;
    16
}
/// $04C80C: movea.l $c(a6), a0
pub fn i_04c80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C810;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C810: cmpi.b #$ff, $20(a0)
pub fn i_04c810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C816;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04C816: bne.w $4c820
pub fn i_04c816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C81A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C820; return 10; }
    8
}
/// $04C81A: lea.l $4c934(pc), a1
pub fn i_04c81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C81E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C934;
    16
}
/// $04C81E: move.l a1, (a6)
pub fn i_04c81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C820: jsr $5e45a.l
pub fn i_04c820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C826);
    cpu.pc = 0x5E45A;
    16
}
/// $04C826: bcc.w $4c830
pub fn i_04c826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C82A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C830; return 10; }
    8
}
// ===== SetTaskHandler_04c82a  ($04C82A, 8 B, kernel/task_handlers.c)
/// $04C82A: lea.l $4c934(pc), a1
pub fn i_04c82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C82E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C934;
    16
}
/// $04C82E: move.l a1, (a6)
pub fn i_04c82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C830;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C830: rts
pub fn i_04c830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C832;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_GunDamaged_04c832  ($04C832, 20 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C832: lea.l $290882.l, a0
pub fn i_04c832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C838;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290882;
    20
}
/// $04C838: jsr $28cd4.l
pub fn i_04c838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C83E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C83E);
    cpu.pc = 0x28CD4;
    16
}
/// $04C83E: lea.l $4c844(pc), a1
pub fn i_04c83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C842;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C844;
    16
}
/// $04C842: move.l a1, (a6)
pub fn i_04c842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C844;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C844: bra.b $4c7d6
pub fn i_04c844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C846;
    cpu.ir = 0x6090;
    cpu.pc = 0x4C7D6;
    10
}
// ===== GunPlatform_FlyingPart_04c846  ($04C846, 194 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C846: lea.l $2908c8.l, a0
pub fn i_04c846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C84C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2908C8;
    20
}
/// $04C84C: jsr $28cd4.l
pub fn i_04c84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C852);
    cpu.pc = 0x28CD4;
    16
}
/// $04C852: subq.w #$8, $24(a6)
pub fn i_04c852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C856;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C856: move.w $36(a6), d0
pub fn i_04c856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C85A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C85A: cmpi.w #$0, d0
pub fn i_04c85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C85E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $04C85E: bgt.w $4c886
pub fn i_04c85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C862;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4C886; return 10; }
    8
}
/// $04C862: move.w $36(a6), d0
pub fn i_04c862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C866;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C866: cmpi.w #$0, d0
pub fn i_04c866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C86A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $04C86A: bgt.w $4c87a
pub fn i_04c86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C86E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4C87A; return 10; }
    8
}
/// $04C86E: lea.l $2bd1de.l, a0
pub fn i_04c86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C874;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD1DE;
    20
}
/// $04C874: jsr $799de.l
pub fn i_04c874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C87A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C87A);
    cpu.pc = 0x799DE;
    16
}
/// $04C87A: lea.l $29088e.l, a0
pub fn i_04c87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C880;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29088E;
    20
}
/// $04C880: jsr $28cd4.l
pub fn i_04c880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C886;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C886);
    cpu.pc = 0x28CD4;
    16
}
/// $04C886: btst.b #$0, $3a(a6)
pub fn i_04c886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C88C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C88C: bne.w $4c892
pub fn i_04c88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C890;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C892; return 10; }
    8
}
/// $04C890: neg.w d0
pub fn i_04c890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C892;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04C892: move.w d0, $28(a6)
pub fn i_04c892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C896;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04C896: move.w #$1cf, d1
pub fn i_04c896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C89A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C89A: jsr $236e.l
pub fn i_04c89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8A0);
    cpu.pc = 0x236E;
    16
}
/// $04C8A0: bset.b #$4, $6b(a6)
pub fn i_04c8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8A6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C8A6: move.w #$d000, d0
pub fn i_04c8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C8AA: jsr $28134.l
pub fn i_04c8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8B0);
    cpu.pc = 0x28134;
    16
}
/// $04C8B0: andi.w #$ffe3, $38(a6)
pub fn i_04c8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8B6;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C8B6: ori.w #$14, $38(a6)
pub fn i_04c8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8BC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C8BC: lea.l $4c8c2(pc), a1
pub fn i_04c8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C8C2;
    16
}
/// $04C8C0: move.l a1, (a6)
pub fn i_04c8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C8C2: jsr $27cee.l
pub fn i_04c8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8C8);
    cpu.pc = 0x27CEE;
    16
}
/// $04C8C8: bcs.w $4c8d6
pub fn i_04c8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4C8D6; return 10; }
    8
}
/// $04C8CC: jsr $27cee.l
pub fn i_04c8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8D2);
    cpu.pc = 0x27CEE;
    16
}
/// $04C8D2: bcc.w $4c8dc
pub fn i_04c8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C8DC; return 10; }
    8
}
/// $04C8D6: lea.l $4c910(pc), a1
pub fn i_04c8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C910;
    16
}
/// $04C8DA: move.l a1, (a6)
pub fn i_04c8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C8DC: jsr $28d70.l
pub fn i_04c8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8E2);
    cpu.pc = 0x28D70;
    16
}
/// $04C8E2: jsr $283d8.l
pub fn i_04c8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C8E8);
    cpu.pc = 0x283D8;
    16
}
/// $04C8E8: btst.b #$1, $13(a6)
pub fn i_04c8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8EE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04C8EE: beq.w $4c8f8
pub fn i_04c8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4C8F8; return 10; }
    8
}
/// $04C8F2: lea.l $4c910(pc), a1
pub fn i_04c8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C910;
    16
}
/// $04C8F6: move.l a1, (a6)
pub fn i_04c8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C8F8: movea.l #$ffffffff, a0
pub fn i_04c8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C8FE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $04C8FE: jsr $5dd56.l
pub fn i_04c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C904);
    cpu.pc = 0x5DD56;
    16
}
/// $04C904: bcc.w $4c90e
pub fn i_04c904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C908;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C90E; return 10; }
    8
}
// ===== SetTaskHandler_04c908  ($04C908, 8 B, kernel/task_handlers.c)
/// $04C908: lea.l $4c934(pc), a1
pub fn i_04c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C90C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C934;
    16
}
/// $04C90C: move.l a1, (a6)
pub fn i_04c90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C90E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C90E: rts
pub fn i_04c90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C910;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_FlyingPartLand_04c910  ($04C910, 36 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C910: move.w #$4000, d0
pub fn i_04c910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C914;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04C914: jsr $28134.l
pub fn i_04c914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C91A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C91A);
    cpu.pc = 0x28134;
    16
}
/// $04C91A: andi.w #$ffe3, $38(a6)
pub fn i_04c91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C920;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C920: ori.w #$0, $38(a6)
pub fn i_04c920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C926;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04C926: move.l #$ffffffff, $48(a6)
pub fn i_04c926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C92E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04C92E: jmp $77f6a.l
pub fn i_04c92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C934;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Jsr5B6ThenJmpScheduler_04c934  ($04C934, 14 B, kernel/jsr5b6_jmp518.c)
/// $04C934: jsr $5b6.l
pub fn i_04c934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C93A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C93A);
    cpu.pc = 0x5B6;
    16
}
/// $04C93A: jmp $518.l
pub fn i_04c93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C940;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== GunPlatform_FreeIfParentDead_04c942  ($04C942, 14 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C942: movea.l $c(a6), a0
pub fn i_04c942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C946;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C946: cmpi.b #$ff, $20(a0)
pub fn i_04c946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C94C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04C94C: bne.w $4c956
pub fn i_04c94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C950;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4C956; return 10; }
    8
}
// ===== SetTaskHandler_04c950  ($04C950, 8 B, kernel/task_handlers.c)
/// $04C950: lea.l $4c58c(pc), a1
pub fn i_04c950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C954;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C58C;
    16
}
/// $04C954: move.l a1, (a6)
pub fn i_04c954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C956;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04C956: rts
pub fn i_04c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C958;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_PartHitCheck_04c958  ($04C958, 50 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C958: jsr $2870a.l
pub fn i_04c958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C95E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C95E);
    cpu.pc = 0x2870A;
    16
}
/// $04C95E: bcc.w $4c974
pub fn i_04c95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C962;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C974; return 10; }
    8
}
/// $04C962: lea.l $5e766.l, a0
pub fn i_04c962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C968;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04C968: jsr $5e770.l
pub fn i_04c968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C96E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C96E);
    cpu.pc = 0x5E770;
    16
}
/// $04C96E: bclr.b #$3, $13(a6)
pub fn i_04c96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C974;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04C974: jsr $28758.l
pub fn i_04c974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C97A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C97A);
    cpu.pc = 0x28758;
    16
}
/// $04C97A: bcc.w $4c988
pub fn i_04c97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C97E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4C988; return 10; }
    8
}
/// $04C97E: movea.l $c(a6), a0
pub fn i_04c97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C982;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C982: move.b #$ff, $20(a0)
pub fn i_04c982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C988;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04C988: rts
pub fn i_04c988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C98A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpriteByStateVariant_04c98a  ($04C98A, 28 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C98A: move.b $21(a6), d1
pub fn i_04c98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C98E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04C98E: andi.w #$ff, d1
pub fn i_04c98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C992;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C992: add.w d1, d1
pub fn i_04c992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C994;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C994: add.w d1, d1
pub fn i_04c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C996;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C996: movea.l (a1, d1.w), a0
pub fn i_04c996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C99A;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C99A: move.w $70(a6), d0
pub fn i_04c99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C99E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C99E: add.w d0, d0
pub fn i_04c99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9A0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9A0: add.w d0, d0
pub fn i_04c9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9A2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9A2: movea.l (a0, d0.w), a0
pub fn i_04c9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9A6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
// ===== JsrAbsThunk_04c9a6  ($04C9A6, 8 B, thunks/jsr_abs_thunks.c)
/// $04C9A6: jsr $28cd4.l
pub fn i_04c9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C9AC);
    cpu.pc = 0x28CD4;
    16
}
/// $04C9AC: rts
pub fn i_04c9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpriteByParentState_04c9ae  ($04C9AE, 32 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C9AE: movea.l $c(a6), a0
pub fn i_04c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9B2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04C9B2: move.b $21(a0), d1
pub fn i_04c9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9B6;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04C9B6: andi.w #$ff, d1
pub fn i_04c9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9BA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04C9BA: add.w d1, d1
pub fn i_04c9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9BC;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9BC: add.w d1, d1
pub fn i_04c9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9BE;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9BE: movea.l (a1, d1.w), a0
pub fn i_04c9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9C2;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $04C9C2: move.w $70(a6), d0
pub fn i_04c9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9C6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C9C6: add.w d0, d0
pub fn i_04c9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9C8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9C8: add.w d0, d0
pub fn i_04c9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9CA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04C9CA: movea.l (a0, d0.w), a0
pub fn i_04c9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9CE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
// ===== JsrAbsThunk_04c9ce  ($04C9CE, 8 B, thunks/jsr_abs_thunks.c)
/// $04C9CE: jsr $28cd4.l
pub fn i_04c9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4C9D4);
    cpu.pc = 0x28CD4;
    16
}
/// $04C9D4: rts
pub fn i_04c9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_TickTimer_04c9d6  ($04C9D6, 12 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C9D6: subq.w #$1, $80(a6)
pub fn i_04c9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9DA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C9DA: bgt.w $4c9e8
pub fn i_04c9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4C9E8; return 10; }
    8
}
/// $04C9DE: clr.w $80(a6)
pub fn i_04c9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9E2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetXN_04c9e2  ($04C9E2, 6 B, thunks/ccr_helpers.c)
/// $04C9E2: ori.b #$11, ccr
pub fn i_04c9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9E6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04C9E6: rts
pub fn i_04c9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04c9e8  ($04C9E8, 6 B, thunks/ccr_helpers.c)
/// $04C9E8: andi.b #$ee, ccr
pub fn i_04c9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04C9EC: rts
pub fn i_04c9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_TickFireInterval_04c9ee  ($04C9EE, 20 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04C9EE: addq.w #$1, $86(a6)
pub fn i_04c9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9F2;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04C9F2: move.w $86(a6), d0
pub fn i_04c9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9F6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04C9F6: cmp.w $84(a6), d0
pub fn i_04c9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9FA;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04C9FA: blt.w $4ca08
pub fn i_04c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4C9FE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4CA08; return 10; }
    8
}
/// $04C9FE: clr.w $86(a6)
pub fn i_04c9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetXN_04ca02  ($04CA02, 6 B, thunks/ccr_helpers.c)
/// $04CA02: ori.b #$11, ccr
pub fn i_04ca02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA06;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04CA06: rts
pub fn i_04ca06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_04ca08  ($04CA08, 6 B, thunks/ccr_helpers.c)
/// $04CA08: andi.b #$ee, ccr
pub fn i_04ca08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA0C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04CA0C: rts
pub fn i_04ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_LoadIdleTimer_04ca0e  ($04CA0E, 22 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA0E: lea.l $2bd260.l, a0
pub fn i_04ca0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA14;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD260;
    20
}
/// $04CA14: jsr $799de.l
pub fn i_04ca14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA1A);
    cpu.pc = 0x799DE;
    16
}
/// $04CA1A: move.w d0, $80(a6)
pub fn i_04ca1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CA1E: clr.b $8a(a6)
pub fn i_04ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA22;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CA22: rts
pub fn i_04ca22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_LoadBurstParams_04ca24  ($04CA24, 38 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA24: lea.l $2bd364.l, a0
pub fn i_04ca24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD364;
    20
}
/// $04CA2A: jsr $799de.l
pub fn i_04ca2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA30);
    cpu.pc = 0x799DE;
    16
}
/// $04CA30: move.b d0, $82(a6)
pub fn i_04ca30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA34;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CA34: lea.l $2bd2e2.l, a0
pub fn i_04ca34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD2E2;
    20
}
/// $04CA3A: jsr $799de.l
pub fn i_04ca3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA40);
    cpu.pc = 0x799DE;
    16
}
/// $04CA40: move.w d0, $84(a6)
pub fn i_04ca40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA44;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CA44: clr.w $86(a6)
pub fn i_04ca44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA48;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CA48: rts
pub fn i_04ca48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_RegisterKill_04ca4a  ($04CA4A, 6 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA4A: lea.l $290e6c.l, a1
pub fn i_04ca4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x290E6C;
    20
}
// ===== JsrAbsThunk_04ca50  ($04CA50, 8 B, thunks/jsr_abs_thunks.c)
/// $04CA50: jsr $43fac.l
pub fn i_04ca50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA56);
    cpu.pc = 0x43FAC;
    16
}
/// $04CA56: rts
pub fn i_04ca56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpawnExplosionMusic_04ca58  ($04CA58, 36 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA58: move.w #$1067, d0
pub fn i_04ca58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA5C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1067;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CA5C: jsr $2352.l
pub fn i_04ca5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA62);
    cpu.pc = 0x2352;
    16
}
/// $04CA62: lea.l $620da.l, a1
pub fn i_04ca62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x620DA;
    20
}
/// $04CA68: jsr $4ae.l
pub fn i_04ca68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA6E);
    cpu.pc = 0x4AE;
    16
}
/// $04CA6E: jsr $5dd02.l
pub fn i_04ca6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA74);
    cpu.pc = 0x5DD02;
    16
}
/// $04CA74: subi.w #$20, $24(a0)
pub fn i_04ca74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA7A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04CA7A: rts
pub fn i_04ca7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SpawnFlyingPart_04ca7c  ($04CA7C, 28 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA7C: lea.l $4c86e(pc), a1
pub fn i_04ca7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4C86E;
    16
}
/// $04CA80: jsr $4ae.l
pub fn i_04ca80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA86);
    cpu.pc = 0x4AE;
    16
}
/// $04CA86: jsr $5dd02.l
pub fn i_04ca86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CA8C);
    cpu.pc = 0x5DD02;
    16
}
/// $04CA8C: subi.w #$20, $22(a0)
pub fn i_04ca8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA92;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $04CA92: addq.w #$4, $24(a0)
pub fn i_04ca92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA96;
    cpu.ir = 0x5868;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CA96: rts
pub fn i_04ca96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SndByVariant_04ca98  ($04CA98, 8 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CA98: move.b $98(a6), d0
pub fn i_04ca98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CA9C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CA9C: move.w #$99, d1
pub fn i_04ca9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAA0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_04caa0  ($04CAA0, 8 B, thunks/jsr_abs_thunks.c)
/// $04CAA0: jsr $9a7aa.l
pub fn i_04caa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CAA6);
    cpu.pc = 0x9A7AA;
    16
}
/// $04CAA6: rts
pub fn i_04caa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== GunPlatform_SlotPrioCheck_04caa8  ($04CAA8, 16 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CAA8: movea.l $8(a6), a1
pub fn i_04caa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAAC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04CAAC: move.b $10(a6), d0
pub fn i_04caac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAB0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CAB0: cmp.b $10(a1), d0
pub fn i_04cab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAB4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04CAB4: bcs.w $4cabe
pub fn i_04cab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAB8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CABE; return 10; }
    8
}
// ===== ClearXN_04cab8  ($04CAB8, 6 B, thunks/ccr_helpers.c)
/// $04CAB8: andi.b #$ee, ccr
pub fn i_04cab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CABC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04CABC: rts
pub fn i_04cabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CABE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04cabe  ($04CABE, 6 B, thunks/ccr_helpers.c)
/// $04CABE: ori.b #$11, ccr
pub fn i_04cabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAC2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04CAC2: rts
pub fn i_04cac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpawnStream_ReadNext_04cac4  ($04CAC4, 128 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CAC4: movea.l $1081b2.l, a0
pub fn i_04cac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CACA;
    cpu.ir = 0x2079;
    let v2 = bus.read32(0x1081B2);
    cpu.a[0] = v2;
    20
}
/// $04CACA: cmpa.w #$0, a0
pub fn i_04caca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CACE;
    cpu.ir = 0xB0FC;
    let v1: u32 = 0x0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, (v1 as u16 as i16 as i32 as u32), Size::Long, false);
    6
}
/// $04CACE: bpl.w $4caec
pub fn i_04cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAD2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x4CAEC; return 10; }
    8
}
/// $04CAD2: move.w #$50, d0
pub fn i_04cad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAD6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CAD6: move.w #$1f8, d1
pub fn i_04cad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CADA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CADA: move.b #$0, d2
pub fn i_04cada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CADE;
    cpu.ir = 0x143C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v1 & 0xFF);
    4
}
/// $04CADE: move.w #$10, d3
pub fn i_04cade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE2;
    cpu.ir = 0x363C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CAE2: clr.b d4
pub fn i_04cae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE4;
    cpu.ir = 0x4204;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE4: clr.b d5
pub fn i_04cae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE6;
    cpu.ir = 0x4205;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE6: clr.b d6
pub fn i_04cae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAE8;
    cpu.ir = 0x4206;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $04CAE8: bra.w $4cb42
pub fn i_04cae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x4CB42;
    10
}
/// $04CAEC: move.w $106f5c.l, d0
pub fn i_04caec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAF2;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F5C) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $04CAF2: cmp.w $10(a0), d0
pub fn i_04caf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAF6;
    cpu.ir = 0xB068;
    let ad2 = cpu.a[0].wrapping_add(0x10);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04CAF6: bcs.w $4cb04
pub fn i_04caf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAFA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CB04; return 10; }
    8
}
/// $04CAFA: adda.w #$10, a0
pub fn i_04cafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CAFE;
    cpu.ir = 0xD0FC;
    let v1: u32 = 0x10;
    cpu.a[0] = cpu.a[0].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $04CAFE: move.l a0, $1081b2.l
pub fn i_04cafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB04;
    cpu.ir = 0x23C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    bus.write32(0x1081B2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04CB04: move.w $4(a0), d0
pub fn i_04cb04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB08;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB08: tst.b $2(a0)
pub fn i_04cb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB0C;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04CB0C: beq.w $4cb16
pub fn i_04cb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB10;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CB16; return 10; }
    8
}
/// $04CB10: sub.w $106f50.l, d0
pub fn i_04cb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB16;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x106F50) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04CB16: move.w $6(a0), d1
pub fn i_04cb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB1A;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB1A: tst.b $3(a0)
pub fn i_04cb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB1E;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x3);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $04CB1E: beq.w $4cb28
pub fn i_04cb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CB28; return 10; }
    8
}
/// $04CB22: sub.w $106f54.l, d1
pub fn i_04cb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB28;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x106F54) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $04CB28: neg.w d1
pub fn i_04cb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB2A;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04CB2A: addi.w #$200, d1
pub fn i_04cb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB2E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04CB2E: move.w $8(a0), d2
pub fn i_04cb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB32;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB32: move.w $a(a0), d3
pub fn i_04cb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB36;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CB36: move.b $c(a0), d4
pub fn i_04cb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB3A;
    cpu.ir = 0x1828;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB3A: move.b $d(a0), d5
pub fn i_04cb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB3E;
    cpu.ir = 0x1A28;
    let ad2 = cpu.a[0].wrapping_add(0xD);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB3E: move.b $e(a0), d6
pub fn i_04cb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB42;
    cpu.ir = 0x1C28;
    let ad2 = cpu.a[0].wrapping_add(0xE);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CB42: rts
pub fn i_04cb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Table_LoadPtrByIdxClamp6_04CB5C  ($04CB5C, 44 B, kernel/table_load_ptr_by_idx_04cb5c.s)
/// $04CB5C: move.l #$ffffffff, $1081b2.l
pub fn i_04cb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB66;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xFFFFFFFF;
    bus.write32(0x1081B2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $04CB66: moveq #$0, d0
pub fn i_04cb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB68;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CB68: move.b $106ece.l, d0
pub fn i_04cb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB6E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106ECE) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $04CB6E: cmpi.b #$6, d0
pub fn i_04cb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB72;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x6, Size::Byte, false);
    8
}
/// $04CB72: bcc.w $4cb86
pub fn i_04cb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CB86; return 10; }
    8
}
/// $04CB76: lea.l $4cb44(pc), a0
pub fn i_04cb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x4CB44;
    16
}
/// $04CB7A: add.w d0, d0
pub fn i_04cb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04CB7C: add.w d0, d0
pub fn i_04cb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB7E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $04CB7E: move.l (a0, d0.w), $1081b2.l
pub fn i_04cb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB86;
    cpu.ir = 0x23F0;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    bus.write32(0x1081B2, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    34
}
/// $04CB86: rts
pub fn i_04cb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpawnStream_Dispatch_04cb88  ($04CB88, 40 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CB88: jsr $4cac4(pc)
pub fn i_04cb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB8C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CB8C);
    cpu.pc = 0x4CAC4;
    16
}
/// $04CB8C: btst.b #$0, $100001.l
pub fn i_04cb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB94;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $04CB94: beq.w $4cbb6
pub fn i_04cb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CBB6; return 10; }
    8
}
/// $04CB98: swap d0
pub fn i_04cb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB9A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CB9A: move.w d1, d0
pub fn i_04cb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CB9C;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CB9C: move.w #$ffff, d1
pub fn i_04cb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CBA0: move.w #$0, d2
pub fn i_04cba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CBA4: moveq #$ff, d3
pub fn i_04cba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA6;
    cpu.ir = 0x76FF;
    cpu.d[3] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBA6: moveq #$ff, d4
pub fn i_04cba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBA8;
    cpu.ir = 0x78FF;
    cpu.d[4] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBA8: moveq #$0, d5
pub fn i_04cba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBAA;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $04CBAA: lea.l $243608.l, a0
pub fn i_04cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x243608;
    20
}
// ===== JsrAbsThunk_04cbb0  ($04CBB0, 8 B, thunks/jsr_abs_thunks.c)
/// $04CBB0: jsr $5a9d6.l
pub fn i_04cbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CBB6);
    cpu.pc = 0x5A9D6;
    16
}
/// $04CBB6: rts
pub fn i_04cbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SlotPrioCheck_04cbb8  ($04CBB8, 16 B, enemies/vehicles/gun_platform_04bb9a.s)
/// $04CBB8: movea.l $8(a6), a1
pub fn i_04cbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBBC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $04CBBC: move.b $10(a6), d0
pub fn i_04cbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBC0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04CBC0: cmp.b $10(a1), d0
pub fn i_04cbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBC4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $04CBC4: bcs.w $4cbce
pub fn i_04cbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBC8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CBCE; return 10; }
    8
}
// ===== ClearXN_04cbc8  ($04CBC8, 6 B, thunks/ccr_helpers.c)
/// $04CBC8: andi.b #$ee, ccr
pub fn i_04cbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBCC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $04CBCC: rts
pub fn i_04cbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_04cbce  ($04CBCE, 6 B, thunks/ccr_helpers.c)
/// $04CBCE: ori.b #$11, ccr
pub fn i_04cbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBD2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $04CBD2: rts
pub fn i_04cbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Spawn_04cbd4  ($04CBD4, 114 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CBD4: move.b #$44, $20(a6)
pub fn i_04cbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x44;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CBDA: move.b #$ff, $80(a6)
pub fn i_04cbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBE0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CBE0: move.b #$1, $98(a6)
pub fn i_04cbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CBE6: bra.w $4cbfc
pub fn i_04cbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CBEA;
    cpu.ir = 0x6000;
    cpu.pc = 0x4CBFC;
    10
}
/// $04CBFC: move.w #$e, d1
pub fn i_04cbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC00;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC00: jsr $236e.l
pub fn i_04cc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC06);
    cpu.pc = 0x236E;
    16
}
/// $04CC06: move.b #$ff, $32(a6)
pub fn i_04cc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC0C: move.b #$ff, $33(a6)
pub fn i_04cc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC12: move.w #$8000, $38(a6)
pub fn i_04cc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC18;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CC18: move.b #$ff, $7d(a6)
pub fn i_04cc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC1E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC1E: move.b #$0, $7c(a6)
pub fn i_04cc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC24;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC24: move.b #$0, $21(a6)
pub fn i_04cc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC2A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC2A: move.w #$0, $34(a6)
pub fn i_04cc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CC30: jsr $267e2.l
pub fn i_04cc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC36);
    cpu.pc = 0x267E2;
    16
}
/// $04CC36: lea.l $4cf2e(pc), a1
pub fn i_04cc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CF2E;
    16
}
/// $04CC3A: jsr $4ae.l
pub fn i_04cc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC40);
    cpu.pc = 0x4AE;
    16
}
/// $04CC40: jsr $5dd02.l
pub fn i_04cc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC46);
    cpu.pc = 0x5DD02;
    16
}
// ===== TurretCar_Idle_04cc46  ($04CC46, 314 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CC46: move.w #$1f, d0
pub fn i_04cc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC4A: move.w $34(a6), d1
pub fn i_04cc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC4E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CC4E: btst.b #$3, d1
pub fn i_04cc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC52;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $04CC52: bne.w $4cc5a
pub fn i_04cc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC56;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CC5A; return 10; }
    8
}
/// $04CC56: move.w #$11, d0
pub fn i_04cc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC5A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC5A: move.w d0, $78(a6)
pub fn i_04cc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC5E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CC5E: move.b #$0, $7b(a6)
pub fn i_04cc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC64;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC64: move.b #$0, $75(a6)
pub fn i_04cc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC6A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CC6A: move.w #$0, $70(a6)
pub fn i_04cc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC70;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CC70: lea.l $290f6e.l, a0
pub fn i_04cc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290F6E;
    20
}
/// $04CC76: jsr $28cd4.l
pub fn i_04cc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CC7C);
    cpu.pc = 0x28CD4;
    16
}
/// $04CC7C: lea.l $4cc82(pc), a1
pub fn i_04cc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CC82;
    16
}
/// $04CC80: move.l a1, (a6)
pub fn i_04cc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CC82: jsr $4d3ce(pc)
pub fn i_04cc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CC86);
    cpu.pc = 0x4D3CE;
    16
}
/// $04CC86: addq.w #$1, $70(a6)
pub fn i_04cc86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC8A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CC8A: move.w #$7, d0
pub fn i_04cc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC8E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC8E: cmpi.b #$0, $98(a6)
pub fn i_04cc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC94;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04CC94: beq.w $4cc9c
pub fn i_04cc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CC9C; return 10; }
    8
}
/// $04CC98: move.w #$1, d0
pub fn i_04cc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CC9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CC9C: and.w d0, $70(a6)
pub fn i_04cc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCA0;
    cpu.ir = 0xC16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CCA0: bne.w $4ccba
pub fn i_04cca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CCBA; return 10; }
    8
}
/// $04CCA4: move.w $78(a6), d0
pub fn i_04cca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCA8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CCA8: jsr $4d4b0(pc)
pub fn i_04cca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCAC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CCAC);
    cpu.pc = 0x4D4B0;
    16
}
/// $04CCAC: move.w $34(a6), d1
pub fn i_04ccac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCB0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CCB0: add.w d0, $34(a6)
pub fn i_04ccb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCB4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CCB4: andi.w #$1f, $34(a6)
pub fn i_04ccb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCBA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04CCBA: jsr $28d70.l
pub fn i_04ccba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CCC0);
    cpu.pc = 0x28D70;
    16
}
/// $04CCC0: cmpi.b #$ff, $21(a6)
pub fn i_04ccc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCC6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CCC6: bne.w $4cd04
pub fn i_04ccc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCCA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CD04; return 10; }
    8
}
/// $04CCCA: cmpi.b #$88, $20(a6)
pub fn i_04ccca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCD0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04CCD0: bne.w $4ccdc
pub fn i_04ccd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CCDC; return 10; }
    8
}
/// $04CCD4: movea.l $c(a6), a0
pub fn i_04ccd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCD8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CCD8: addq.b #$1, $21(a0)
pub fn i_04ccd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCDC;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CCDC: move.b #$ff, $7c(a6)
pub fn i_04ccdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCE2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CCE2: lea.l $58f82.l, a1
pub fn i_04cce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $04CCE8: move.l a1, (a6)
pub fn i_04cce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CCEA: lea.l $776e2.l, a1
pub fn i_04ccea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCF0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $04CCF0: jsr $4ae.l
pub fn i_04ccf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CCF6);
    cpu.pc = 0x4AE;
    16
}
/// $04CCF6: jsr $5dd02.l
pub fn i_04ccf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CCFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CCFC);
    cpu.pc = 0x5DD02;
    16
}
/// $04CCFC: jsr $4d646(pc)
pub fn i_04ccfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD00);
    cpu.pc = 0x4D646;
    16
}
/// $04CD00: jsr $4d474(pc)
pub fn i_04cd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD04);
    cpu.pc = 0x4D474;
    16
}
/// $04CD04: move.w $78(a6), d0
pub fn i_04cd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD08;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CD08: cmp.w $34(a6), d0
pub fn i_04cd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD0C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $04CD0C: bne.w $4cd5e
pub fn i_04cd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CD5E; return 10; }
    8
}
/// $04CD10: cmpi.b #$ff, $75(a6)
pub fn i_04cd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD16;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CD16: bne.w $4cd24
pub fn i_04cd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD1A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CD24; return 10; }
    8
}
/// $04CD1A: lea.l $4cd80(pc), a1
pub fn i_04cd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CD80;
    16
}
/// $04CD1E: move.l a1, (a6)
pub fn i_04cd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CD20: bra.w $4cd5e
pub fn i_04cd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD24;
    cpu.ir = 0x6000;
    cpu.pc = 0x4CD5E;
    10
}
/// $04CD24: cmpi.b #$80, $75(a6)
pub fn i_04cd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD2A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $04CD2A: beq.w $4cd5e
pub fn i_04cd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CD5E; return 10; }
    8
}
/// $04CD2E: lea.l $290f0e.l, a0
pub fn i_04cd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x290F0E;
    20
}
/// $04CD34: jsr $28cd4.l
pub fn i_04cd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CD3A);
    cpu.pc = 0x28CD4;
    16
}
/// $04CD3A: move.w $34(a6), d0
pub fn i_04cd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD3E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CD3E: asr.b #$3, d0
pub fn i_04cd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD40;
    cpu.ir = 0xE600;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $04CD40: andi.b #$1, d0
pub fn i_04cd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD44;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04CD44: or.b d0, $3a(a6)
pub fn i_04cd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD48;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CD48: lea.l $4d048(pc), a1
pub fn i_04cd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D048;
    16
}
/// $04CD4C: jsr $6fe.l
pub fn i_04cd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CD52);
    cpu.pc = 0x6FE;
    16
}
/// $04CD52: jsr $5dd02.l
pub fn i_04cd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CD58);
    cpu.pc = 0x5DD02;
    16
}
/// $04CD58: move.b #$80, $75(a6)
pub fn i_04cd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD5E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CD5E: jsr $4d454(pc)
pub fn i_04cd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD62);
    cpu.pc = 0x4D454;
    16
}
/// $04CD62: bcs.w $4cd6e
pub fn i_04cd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD66;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CD6E; return 10; }
    8
}
/// $04CD66: jsr $4d5ac(pc)
pub fn i_04cd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD6A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD6A);
    cpu.pc = 0x4D5AC;
    16
}
/// $04CD6A: bcc.w $4cd7e
pub fn i_04cd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CD7E; return 10; }
    8
}
/// $04CD6E: move.b #$ff, $7c(a6)
pub fn i_04cd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CD74: jsr $4d474(pc)
pub fn i_04cd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD78);
    cpu.pc = 0x4D474;
    16
}
/// $04CD78: jmp $518.l
pub fn i_04cd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD7E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04CD7E: rts
pub fn i_04cd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Track_04cd80  ($04CD80, 226 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CD80: lea.l $2b791e.l, a0
pub fn i_04cd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B791E;
    20
}
/// $04CD86: jsr $799de.l
pub fn i_04cd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CD8C);
    cpu.pc = 0x799DE;
    16
}
/// $04CD8C: move.w d0, $72(a6)
pub fn i_04cd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD90;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CD90: lea.l $4cd96(pc), a1
pub fn i_04cd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CD96;
    16
}
/// $04CD94: move.l a1, (a6)
pub fn i_04cd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CD96: jsr $4d3ce(pc)
pub fn i_04cd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD9A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CD9A);
    cpu.pc = 0x4D3CE;
    16
}
/// $04CD9A: addq.w #$1, $70(a6)
pub fn i_04cd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CD9E;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CD9E: move.w #$7, d0
pub fn i_04cd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDA2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CDA2: cmpi.b #$0, $98(a6)
pub fn i_04cda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDA8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $04CDA8: beq.w $4cdb0
pub fn i_04cda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4CDB0; return 10; }
    8
}
/// $04CDAC: move.w #$1, d0
pub fn i_04cdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDB0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CDB0: and.w d0, $70(a6)
pub fn i_04cdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDB4;
    cpu.ir = 0xC16E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CDB4: bne.w $4cdd2
pub fn i_04cdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CDD2; return 10; }
    8
}
/// $04CDB8: jsr $4d58a(pc)
pub fn i_04cdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CDBC);
    cpu.pc = 0x4D58A;
    16
}
/// $04CDBC: jsr $4d612(pc)
pub fn i_04cdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CDC0);
    cpu.pc = 0x4D612;
    16
}
/// $04CDC0: jsr $4d4b0(pc)
pub fn i_04cdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CDC4);
    cpu.pc = 0x4D4B0;
    16
}
/// $04CDC4: move.w $34(a6), d1
pub fn i_04cdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDC8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CDC8: add.w d0, $34(a6)
pub fn i_04cdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDCC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CDCC: andi.w #$1f, $34(a6)
pub fn i_04cdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDD2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04CDD2: jsr $28d70.l
pub fn i_04cdd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CDD8);
    cpu.pc = 0x28D70;
    16
}
/// $04CDD8: cmpi.b #$ff, $21(a6)
pub fn i_04cdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDDE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CDDE: bne.w $4ce1c
pub fn i_04cdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDE2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CE1C; return 10; }
    8
}
/// $04CDE2: cmpi.b #$88, $20(a6)
pub fn i_04cde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDE8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04CDE8: bne.w $4cdf4
pub fn i_04cde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CDF4; return 10; }
    8
}
/// $04CDEC: movea.l $c(a6), a0
pub fn i_04cdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDF0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CDF0: addq.b #$1, $21(a0)
pub fn i_04cdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDF4;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CDF4: move.b #$ff, $7c(a6)
pub fn i_04cdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CDFA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CDFA: lea.l $58f82.l, a1
pub fn i_04cdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $04CE00: move.l a1, (a6)
pub fn i_04ce00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CE02: lea.l $776e2.l, a1
pub fn i_04ce02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE08;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $04CE08: jsr $4ae.l
pub fn i_04ce08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CE0E);
    cpu.pc = 0x4AE;
    16
}
/// $04CE0E: jsr $5dd02.l
pub fn i_04ce0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CE14);
    cpu.pc = 0x5DD02;
    16
}
/// $04CE14: jsr $4d646(pc)
pub fn i_04ce14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE18);
    cpu.pc = 0x4D646;
    16
}
/// $04CE18: jsr $4d474(pc)
pub fn i_04ce18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE1C);
    cpu.pc = 0x4D474;
    16
}
/// $04CE1C: cmpi.b #$ff, $7d(a6)
pub fn i_04ce1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE22;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CE22: bne.w $4ce40
pub fn i_04ce22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CE40; return 10; }
    8
}
/// $04CE26: subq.w #$1, $72(a6)
pub fn i_04ce26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE2A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CE2A: bne.w $4ce40
pub fn i_04ce2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CE40; return 10; }
    8
}
/// $04CE2E: move.b #$0, $7d(a6)
pub fn i_04ce2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CE34: move.b #$ff, $7b(a6)
pub fn i_04ce34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE3A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CE3A: lea.l $4ce62(pc), a1
pub fn i_04ce3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CE62;
    16
}
/// $04CE3E: move.l a1, (a6)
pub fn i_04ce3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CE40: jsr $4d454(pc)
pub fn i_04ce40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE44);
    cpu.pc = 0x4D454;
    16
}
/// $04CE44: bcs.w $4ce50
pub fn i_04ce44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE48;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CE50; return 10; }
    8
}
/// $04CE48: jsr $4d5ac(pc)
pub fn i_04ce48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE4C);
    cpu.pc = 0x4D5AC;
    16
}
/// $04CE4C: bcc.w $4ce60
pub fn i_04ce4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CE60; return 10; }
    8
}
/// $04CE50: move.b #$ff, $7c(a6)
pub fn i_04ce50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE56;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CE56: jsr $4d474(pc)
pub fn i_04ce56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE5A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE5A);
    cpu.pc = 0x4D474;
    16
}
/// $04CE5A: jmp $518.l
pub fn i_04ce5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE60;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04CE60: rts
pub fn i_04ce60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Recoil_04ce62  ($04CE62, 204 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CE62: move.b #$0, $7b(a6)
pub fn i_04ce62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CE68: move.w $34(a6), d0
pub fn i_04ce68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE6C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CE6C: asr.b #$3, d0
pub fn i_04ce6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE6E;
    cpu.ir = 0xE600;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $04CE6E: andi.b #$1, d0
pub fn i_04ce6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE72;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04CE72: or.b d0, $3a(a6)
pub fn i_04ce72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE76;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CE76: move.w #$32, $7e(a6)
pub fn i_04ce76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CE7C: lea.l $29bfc4.l, a0
pub fn i_04ce7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFC4;
    20
}
/// $04CE82: jsr $28cd4.l
pub fn i_04ce82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CE88);
    cpu.pc = 0x28CD4;
    16
}
/// $04CE88: lea.l $4ce8e(pc), a1
pub fn i_04ce88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CE8E;
    16
}
/// $04CE8C: move.l a1, (a6)
pub fn i_04ce8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CE8E: jsr $4d3ce(pc)
pub fn i_04ce8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CE92);
    cpu.pc = 0x4D3CE;
    16
}
/// $04CE92: move.w $22(a6), d0
pub fn i_04ce92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE96;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CE96: move.w $24(a6), d1
pub fn i_04ce96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE9A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04CE9A: movem.w d0-d1, -(a7)
pub fn i_04ce9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CE9E;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $04CE9E: jsr $4d646(pc)
pub fn i_04ce9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CEA2);
    cpu.pc = 0x4D646;
    16
}
/// $04CEA2: jsr $28d70.l
pub fn i_04cea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CEA8);
    cpu.pc = 0x28D70;
    16
}
/// $04CEA8: subq.w #$1, $7e(a6)
pub fn i_04cea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEAC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04CEAC: bne.w $4cebc
pub fn i_04ceac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEB0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CEBC; return 10; }
    8
}
/// $04CEB0: move.b #$0, $3a(a6)
pub fn i_04ceb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CEB6: lea.l $4cc46(pc), a1
pub fn i_04ceb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CC46;
    16
}
/// $04CEBA: move.l a1, (a6)
pub fn i_04ceba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CEBC: movem.w (a7)+, d0-d1
pub fn i_04cebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEC0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $04CEC0: move.w d0, $22(a6)
pub fn i_04cec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEC4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CEC4: move.w d1, $24(a6)
pub fn i_04cec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEC8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CEC8: cmpi.b #$ff, $21(a6)
pub fn i_04cec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CECE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CECE: bne.w $4cf0c
pub fn i_04cece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CED2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CF0C; return 10; }
    8
}
/// $04CED2: cmpi.b #$88, $20(a6)
pub fn i_04ced2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CED8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04CED8: bne.w $4cee4
pub fn i_04ced8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CEE4; return 10; }
    8
}
/// $04CEDC: movea.l $c(a6), a0
pub fn i_04cedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEE0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CEE0: addq.b #$1, $21(a0)
pub fn i_04cee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEE4;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $04CEE4: move.b #$ff, $7c(a6)
pub fn i_04cee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CEEA: lea.l $58f82.l, a1
pub fn i_04ceea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEF0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x58F82;
    20
}
/// $04CEF0: move.l a1, (a6)
pub fn i_04cef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CEF2: lea.l $776e2.l, a1
pub fn i_04cef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEF8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $04CEF8: jsr $4ae.l
pub fn i_04cef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CEFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CEFE);
    cpu.pc = 0x4AE;
    16
}
/// $04CEFE: jsr $5dd02.l
pub fn i_04cefe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF04);
    cpu.pc = 0x5DD02;
    16
}
/// $04CF04: jsr $4d646(pc)
pub fn i_04cf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF08;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF08);
    cpu.pc = 0x4D646;
    16
}
/// $04CF08: jsr $4d474(pc)
pub fn i_04cf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF0C);
    cpu.pc = 0x4D474;
    16
}
/// $04CF0C: jsr $4d454(pc)
pub fn i_04cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF10);
    cpu.pc = 0x4D454;
    16
}
/// $04CF10: bcs.w $4cf1c
pub fn i_04cf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF14;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CF1C; return 10; }
    8
}
/// $04CF14: jsr $4d5ac(pc)
pub fn i_04cf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF18);
    cpu.pc = 0x4D5AC;
    16
}
/// $04CF18: bcc.w $4cf2c
pub fn i_04cf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CF2C; return 10; }
    8
}
/// $04CF1C: move.b #$ff, $7c(a6)
pub fn i_04cf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF22;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CF22: jsr $4d474(pc)
pub fn i_04cf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF26);
    cpu.pc = 0x4D474;
    16
}
/// $04CF26: jmp $518.l
pub fn i_04cf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF2C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04CF2C: rts
pub fn i_04cf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF2E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Body_04cf2e  ($04CF2E, 282 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04CF2E: move.w #$22, d1
pub fn i_04cf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF32;
    cpu.ir = 0x323C;
    let v1: u32 = 0x22;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CF32: jsr $236e.l
pub fn i_04cf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF38);
    cpu.pc = 0x236E;
    16
}
/// $04CF38: move.b #$ff, $32(a6)
pub fn i_04cf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF3E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CF3E: move.b #$ff, $33(a6)
pub fn i_04cf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF44;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04CF44: move.w #$c, $66(a6)
pub fn i_04cf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF4A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CF4A: lea.l $2b7798.l, a0
pub fn i_04cf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF50;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7798;
    20
}
/// $04CF50: jsr $799de.l
pub fn i_04cf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF56);
    cpu.pc = 0x799DE;
    16
}
/// $04CF56: move.w d0, $66(a6)
pub fn i_04cf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF5A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04CF5A: jsr $267e2.l
pub fn i_04cf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF60);
    cpu.pc = 0x267E2;
    16
}
/// $04CF60: jsr $4d5e4(pc)
pub fn i_04cf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF64);
    cpu.pc = 0x4D5E4;
    16
}
/// $04CF64: lea.l $2912d6.l, a0
pub fn i_04cf64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2912D6;
    20
}
/// $04CF6A: jsr $28cd4.l
pub fn i_04cf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CF70);
    cpu.pc = 0x28CD4;
    16
}
/// $04CF70: lea.l $4cf76(pc), a1
pub fn i_04cf70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4CF76;
    16
}
/// $04CF74: move.l a1, (a6)
pub fn i_04cf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04CF76: movea.l $c(a6), a0
pub fn i_04cf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF7A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CF7A: move.w $22(a0), $22(a6)
pub fn i_04cf7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF80;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04CF80: move.w $24(a0), $24(a6)
pub fn i_04cf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF86;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04CF86: move.w $38(a0), $38(a6)
pub fn i_04cf86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF8C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $04CF8C: jsr $4d5e4(pc)
pub fn i_04cf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CF90);
    cpu.pc = 0x4D5E4;
    16
}
/// $04CF90: movea.l $c(a6), a0
pub fn i_04cf90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF94;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CF94: cmpi.b #$ff, $7b(a0)
pub fn i_04cf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF9A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CF9A: bne.w $4cfaa
pub fn i_04cf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CF9E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CFAA; return 10; }
    8
}
/// $04CF9E: lea.l $291526.l, a0
pub fn i_04cf9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x291526;
    20
}
/// $04CFA4: jsr $28cd4.l
pub fn i_04cfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFAA);
    cpu.pc = 0x28CD4;
    16
}
/// $04CFAA: jsr $28d70.l
pub fn i_04cfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFB0);
    cpu.pc = 0x28D70;
    16
}
/// $04CFB0: movea.l $c(a6), a0
pub fn i_04cfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFB4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CFB4: cmpi.b #$44, $20(a0)
pub fn i_04cfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFBA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x44, Size::Byte, false);
    16
}
/// $04CFBA: bne.w $4cfc6
pub fn i_04cfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4CFC6; return 10; }
    8
}
/// $04CFBE: jsr $4d48a(pc)
pub fn i_04cfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4CFC2);
    cpu.pc = 0x4D48A;
    16
}
/// $04CFC2: bcs.w $4cfde
pub fn i_04cfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFC6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4CFDE; return 10; }
    8
}
/// $04CFC6: movea.l $c(a6), a0
pub fn i_04cfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFCA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04CFCA: cmpi.b #$ff, $7c(a0)
pub fn i_04cfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFD0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04CFD0: beq.w $4d040
pub fn i_04cfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D040; return 10; }
    8
}
/// $04CFD4: jsr $2870a.l
pub fn i_04cfd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFDA);
    cpu.pc = 0x2870A;
    16
}
/// $04CFDA: bcc.w $4cff0
pub fn i_04cfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFDE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4CFF0; return 10; }
    8
}
/// $04CFDE: lea.l $5e766.l, a0
pub fn i_04cfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04CFE4: jsr $5e770.l
pub fn i_04cfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFEA);
    cpu.pc = 0x5E770;
    16
}
/// $04CFEA: bclr.b #$3, $13(a6)
pub fn i_04cfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFF0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04CFF0: jsr $28758.l
pub fn i_04cff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4CFF6);
    cpu.pc = 0x28758;
    16
}
/// $04CFF6: bcc.w $4d046
pub fn i_04cff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFFA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D046; return 10; }
    8
}
/// $04CFFA: move.w #$1023, d0
pub fn i_04cffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4CFFE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04CFFE: jsr $2352.l
pub fn i_04cffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D004;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D004);
    cpu.pc = 0x2352;
    16
}
/// $04D004: movea.l $c(a6), a0
pub fn i_04d004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D008;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D008: move.b #$ff, $21(a0)
pub fn i_04d008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D00E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D00E: lea.l $293eb4.l, a1
pub fn i_04d00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D014;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x293EB4;
    20
}
/// $04D014: jsr $77c7e.l
pub fn i_04d014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D01A);
    cpu.pc = 0x77C7E;
    16
}
/// $04D01A: lea.l $293ec6.l, a1
pub fn i_04d01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D020;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x293EC6;
    20
}
/// $04D020: jsr $77c7e.l
pub fn i_04d020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D026);
    cpu.pc = 0x77C7E;
    16
}
/// $04D026: movea.l $c(a6), a0
pub fn i_04d026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D02A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D02A: cmpi.b #$88, $20(a0)
pub fn i_04d02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D030;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x88, Size::Byte, false);
    16
}
/// $04D030: bne.w $4d040
pub fn i_04d030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D034;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D040; return 10; }
    8
}
/// $04D034: lea.l $295682.l, a2
pub fn i_04d034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D03A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x295682;
    20
}
/// $04D03A: jsr $5022a.l
pub fn i_04d03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D040;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D040);
    cpu.pc = 0x5022A;
    16
}
/// $04D040: jmp $518.l
pub fn i_04d040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D046;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D046: rts
pub fn i_04d046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D048;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Driver_04d048  ($04D048, 154 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D048: move.w #$23, d1
pub fn i_04d048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D04C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x23;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D04C: jsr $236e.l
pub fn i_04d04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D052;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D052);
    cpu.pc = 0x236E;
    16
}
/// $04D052: movea.l $c(a6), a0
pub fn i_04d052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D056;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D056: move.w $34(a0), d0
pub fn i_04d056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D05A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D05A: asr.b #$3, d0
pub fn i_04d05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D05C;
    cpu.ir = 0xE600;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Byte, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $04D05C: andi.b #$1, d0
pub fn i_04d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D060;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $04D060: or.b d0, $3a(a6)
pub fn i_04d060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D064;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $04D064: move.b #$ff, $32(a6)
pub fn i_04d064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D06A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D06A: move.b #$ff, $33(a6)
pub fn i_04d06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D070;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D070: lea.l $2b789c.l, a0
pub fn i_04d070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D076;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B789C;
    20
}
/// $04D076: jsr $799de.l
pub fn i_04d076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D07C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D07C);
    cpu.pc = 0x799DE;
    16
}
/// $04D07C: move.w d0, $66(a6)
pub fn i_04d07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D080;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D080: lea.l $293ed8.l, a0
pub fn i_04d080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D086;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x293ED8;
    20
}
/// $04D086: move.l a0, $4c(a6)
pub fn i_04d086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D08A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $04D08A: jsr $283ca.l
pub fn i_04d08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D090;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D090);
    cpu.pc = 0x283CA;
    16
}
/// $04D090: jsr $267e2.l
pub fn i_04d090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D096);
    cpu.pc = 0x267E2;
    16
}
/// $04D096: lea.l $29276a.l, a0
pub fn i_04d096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D09C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29276A;
    20
}
/// $04D09C: jsr $28cd4.l
pub fn i_04d09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0A2);
    cpu.pc = 0x28CD4;
    16
}
/// $04D0A2: lea.l $4d0a8(pc), a1
pub fn i_04d0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D0A8;
    16
}
/// $04D0A6: move.l a1, (a6)
pub fn i_04d0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D0A8: jsr $5e4dc.l
pub fn i_04d0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0AE);
    cpu.pc = 0x5E4DC;
    16
}
/// $04D0AE: jsr $4d5e4(pc)
pub fn i_04d0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D0B2);
    cpu.pc = 0x4D5E4;
    16
}
/// $04D0B2: jsr $28d70.l
pub fn i_04d0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0B8);
    cpu.pc = 0x28D70;
    16
}
/// $04D0B8: bcc.w $4d0cc
pub fn i_04d0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D0CC; return 10; }
    8
}
/// $04D0BC: movea.l $c(a6), a0
pub fn i_04d0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0C0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D0C0: move.b #$ff, $75(a0)
pub fn i_04d0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0C6;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D0C6: lea.l $4d0e2(pc), a1
pub fn i_04d0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D0E2;
    16
}
/// $04D0CA: move.l a1, (a6)
pub fn i_04d0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D0CC: movea.l $c(a6), a0
pub fn i_04d0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0D0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D0D0: cmpi.b #$ff, $7c(a0)
pub fn i_04d0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0D6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D0D6: bne.w $4d0e0
pub fn i_04d0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D0E0; return 10; }
    8
}
/// $04D0DA: jmp $518.l
pub fn i_04d0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0E0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D0E0: rts
pub fn i_04d0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_DriverPanic_04d0e2  ($04D0E2, 88 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D0E2: move.b #$0, $3a(a6)
pub fn i_04d0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D0E8: move.w #$12c, $70(a6)
pub fn i_04d0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D0EE: lea.l $292a48.l, a0
pub fn i_04d0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x292A48;
    20
}
/// $04D0F4: jsr $28cd4.l
pub fn i_04d0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D0FA);
    cpu.pc = 0x28CD4;
    16
}
/// $04D0FA: lea.l $4d100(pc), a1
pub fn i_04d0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D0FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D100;
    16
}
/// $04D0FE: move.l a1, (a6)
pub fn i_04d0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D100;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D100: jsr $5e4dc.l
pub fn i_04d100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D106;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D106);
    cpu.pc = 0x5E4DC;
    16
}
/// $04D106: jsr $4d5e4(pc)
pub fn i_04d106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D10A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D10A);
    cpu.pc = 0x4D5E4;
    16
}
/// $04D10A: jsr $28d70.l
pub fn i_04d10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D110;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D110);
    cpu.pc = 0x28D70;
    16
}
/// $04D110: movea.l $c(a6), a0
pub fn i_04d110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D114;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D114: cmpi.b #$ff, $7b(a0)
pub fn i_04d114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D11A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D11A: bne.w $4d124
pub fn i_04d11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D11E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D124; return 10; }
    8
}
/// $04D11E: lea.l $4d13a(pc), a1
pub fn i_04d11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D122;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D13A;
    16
}
/// $04D122: move.l a1, (a6)
pub fn i_04d122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D124;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D124: movea.l $c(a6), a0
pub fn i_04d124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D128;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D128: cmpi.b #$ff, $7c(a0)
pub fn i_04d128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D12E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D12E: bne.w $4d138
pub fn i_04d12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D132;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D138; return 10; }
    8
}
/// $04D132: jmp $518.l
pub fn i_04d132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D138;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D138: rts
pub fn i_04d138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D13A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_DriverFlee_04d13a  ($04D13A, 76 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D13A: move.w #$10a3, d0
pub fn i_04d13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D13E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D13E: jsr $2352.l
pub fn i_04d13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D144;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D144);
    cpu.pc = 0x2352;
    16
}
/// $04D144: lea.l $292c92.l, a0
pub fn i_04d144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D14A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x292C92;
    20
}
/// $04D14A: jsr $28cd4.l
pub fn i_04d14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D150);
    cpu.pc = 0x28CD4;
    16
}
/// $04D150: lea.l $4d156(pc), a1
pub fn i_04d150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D154;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D156;
    16
}
/// $04D154: move.l a1, (a6)
pub fn i_04d154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D156;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D156: jsr $5e4dc.l
pub fn i_04d156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D15C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D15C);
    cpu.pc = 0x5E4DC;
    16
}
/// $04D15C: jsr $4d5e4(pc)
pub fn i_04d15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D160;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D160);
    cpu.pc = 0x4D5E4;
    16
}
/// $04D160: jsr $28d70.l
pub fn i_04d160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D166;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D166);
    cpu.pc = 0x28D70;
    16
}
/// $04D166: bcc.w $4d170
pub fn i_04d166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D16A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D170; return 10; }
    8
}
/// $04D16A: lea.l $4d186(pc), a1
pub fn i_04d16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D16E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D186;
    16
}
/// $04D16E: move.l a1, (a6)
pub fn i_04d16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D170;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D170: movea.l $c(a6), a0
pub fn i_04d170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D174;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $04D174: cmpi.b #$ff, $7c(a0)
pub fn i_04d174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D17A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D17A: bne.w $4d184
pub fn i_04d17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D17E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D184; return 10; }
    8
}
/// $04D17E: jmp $518.l
pub fn i_04d17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D184;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $04D184: rts
pub fn i_04d184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D186;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TurretCar_Cannon_04d186  ($04D186, 442 B, enemies/vehicles/turret_car_props_04cbd4.s)
/// $04D186: jsr $4d624(pc)
pub fn i_04d186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D18A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D18A);
    cpu.pc = 0x4D624;
    16
}
/// $04D18A: addq.w #$2, $38(a6)
pub fn i_04d18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D18E;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D18E: move.b #$0, $7a(a6)
pub fn i_04d18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D194;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D194: move.w #$ffff, $70(a6)
pub fn i_04d194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D19A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D19A: lea.l $2b781a.l, a0
pub fn i_04d19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B781A;
    20
}
/// $04D1A0: jsr $799de.l
pub fn i_04d1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D1A6);
    cpu.pc = 0x799DE;
    16
}
/// $04D1A6: move.w d0, $36(a6)
pub fn i_04d1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D1AA: jsr $4d5e4(pc)
pub fn i_04d1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D1AE);
    cpu.pc = 0x4D5E4;
    16
}
/// $04D1AE: lea.l $2927f8.l, a0
pub fn i_04d1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2927F8;
    20
}
/// $04D1B4: jsr $28cd4.l
pub fn i_04d1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D1BA);
    cpu.pc = 0x28CD4;
    16
}
/// $04D1BA: move.w $34(a6), d0
pub fn i_04d1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1BE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D1BE: move.w #$97, d1
pub fn i_04d1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1C2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x97;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D1C2: move.b d0, (a6, d1.w)
pub fn i_04d1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1C6;
    cpu.ir = 0x1D80;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad2, v1 as u8);
    14
}
/// $04D1C6: subq.w #$1, d1
pub fn i_04d1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1C8;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $04D1C8: cmpi.w #$88, d1
pub fn i_04d1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1CC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x88, Size::Word, false);
    8
}
/// $04D1CC: bgt.b $4d1c2
pub fn i_04d1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1CE;
    cpu.ir = 0x6EF4;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D1C2; return 10; }
    8
}
/// $04D1CE: jsr $4d6ac(pc)
pub fn i_04d1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D1D2);
    cpu.pc = 0x4D6AC;
    16
}
/// $04D1D2: lea.l $4d1d8(pc), a1
pub fn i_04d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D1D8;
    16
}
/// $04D1D6: move.l a1, (a6)
pub fn i_04d1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $04D1D8: addq.w #$1, $70(a6)
pub fn i_04d1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1DC;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D1DC: move.w $78(a6), d0
pub fn i_04d1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1E0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D1E0: cmpi.w #$258, $70(a6)
pub fn i_04d1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x258, Size::Word, false);
    16
}
/// $04D1E6: bgt.w $4d228
pub fn i_04d1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D228; return 10; }
    8
}
/// $04D1EA: move.w $70(a6), d0
pub fn i_04d1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D1EE: andi.w #$f, d0
pub fn i_04d1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1F2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D1F2: bne.w $4d240
pub fn i_04d1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D240; return 10; }
    8
}
/// $04D1F6: jsr $4d4f0(pc)
pub fn i_04d1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D1FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D1FA);
    cpu.pc = 0x4D4F0;
    16
}
/// $04D1FA: cmpi.b #$ff, $7a(a6)
pub fn i_04d1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D200;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $04D200: beq.w $4d218
pub fn i_04d200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D204;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x4D218; return 10; }
    8
}
/// $04D204: jsr $4d57e(pc)
pub fn i_04d204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D208;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D208);
    cpu.pc = 0x4D57E;
    16
}
/// $04D208: cmpi.w #$1c0, $24(a6)
pub fn i_04d208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D20E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1C0, Size::Word, false);
    16
}
/// $04D20E: blt.w $4d218
pub fn i_04d20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D212;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x4D218; return 10; }
    8
}
/// $04D212: move.b #$ff, $7a(a6)
pub fn i_04d212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D218;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $04D218: jsr $4d668(pc)
pub fn i_04d218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D21C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D21C);
    cpu.pc = 0x4D668;
    16
}
/// $04D21C: bcc.w $4d224
pub fn i_04d21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D220;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D224; return 10; }
    8
}
/// $04D220: move.w #$8, d0
pub fn i_04d220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D224;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D224: move.w d0, $78(a6)
pub fn i_04d224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D228;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D228: jsr $4d4b0(pc)
pub fn i_04d228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D22C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D22C);
    cpu.pc = 0x4D4B0;
    16
}
/// $04D22C: ori.w #$1, d0
pub fn i_04d22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D230;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D230: move.w $70(a6), d1
pub fn i_04d230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D234;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D234: andi.w #$1f, d1
pub fn i_04d234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D238;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D238: bne.w $4d240
pub fn i_04d238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D23C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D240; return 10; }
    8
}
/// $04D23C: move.w d0, $76(a6)
pub fn i_04d23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D240;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D240: move.w $70(a6), d0
pub fn i_04d240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D244;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D244: andi.w #$3, d0
pub fn i_04d244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D248;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D248: bne.w $4d25e
pub fn i_04d248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D24C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D25E; return 10; }
    8
}
/// $04D24C: move.w $34(a6), d1
pub fn i_04d24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D250;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D250: move.w $76(a6), d0
pub fn i_04d250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D254;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D254: add.w d0, $34(a6)
pub fn i_04d254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D258;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D258: andi.w #$1f, $34(a6)
pub fn i_04d258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D25E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $04D25E: jsr $4d51c(pc)
pub fn i_04d25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D262;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D262);
    cpu.pc = 0x4D51C;
    16
}
/// $04D262: jsr $4d3fe(pc)
pub fn i_04d262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D266;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D266);
    cpu.pc = 0x4D3FE;
    16
}
/// $04D266: bcs.w $4d2f4
pub fn i_04d266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D26A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x4D2F4; return 10; }
    8
}
/// $04D26A: jsr $28d70.l
pub fn i_04d26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D270);
    cpu.pc = 0x28D70;
    16
}
/// $04D270: move.w $70(a6), d0
pub fn i_04d270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D274;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $04D274: andi.w #$7, d0
pub fn i_04d274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D278;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D278: bne.w $4d2be
pub fn i_04d278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D27C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D2BE; return 10; }
    8
}
/// $04D27C: cmpi.w #$10, $34(a6)
pub fn i_04d27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D282;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $04D282: bgt.w $4d29a
pub fn i_04d282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D286;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x4D29A; return 10; }
    8
}
/// $04D286: lea.l $4d340(pc), a1
pub fn i_04d286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D28A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D340;
    16
}
/// $04D28A: jsr $4ae.l
pub fn i_04d28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D290;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D290);
    cpu.pc = 0x4AE;
    16
}
/// $04D290: jsr $5dd02.l
pub fn i_04d290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D296;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D296);
    cpu.pc = 0x5DD02;
    16
}
/// $04D296: bra.w $4d2ae
pub fn i_04d296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D29A;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D2AE;
    10
}
/// $04D29A: lea.l $4d340(pc), a1
pub fn i_04d29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D29E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x4D340;
    16
}
/// $04D29E: jsr $6fe.l
pub fn i_04d29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2A4);
    cpu.pc = 0x6FE;
    16
}
/// $04D2A4: jsr $5dd02.l
pub fn i_04d2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2AA);
    cpu.pc = 0x5DD02;
    16
}
/// $04D2AA: subq.w #$1, $38(a0)
pub fn i_04d2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2AE;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $04D2AE: jsr $4d5f0(pc)
pub fn i_04d2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D2B2);
    cpu.pc = 0x4D5F0;
    16
}
/// $04D2B2: move.b $97(a6), d1
pub fn i_04d2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2B6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $04D2B6: andi.w #$1f, d1
pub fn i_04d2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2BA;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $04D2BA: move.w d1, $34(a0)
pub fn i_04d2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2BE;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D2BE: jsr $283d8.l
pub fn i_04d2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2C4);
    cpu.pc = 0x283D8;
    16
}
/// $04D2C4: btst.b #$1, $13(a6)
pub fn i_04d2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $04D2CA: bne.w $4d2f4
pub fn i_04d2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2CE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x4D2F4; return 10; }
    8
}
/// $04D2CE: jsr $2870a.l
pub fn i_04d2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2D4);
    cpu.pc = 0x2870A;
    16
}
/// $04D2D4: bcc.w $4d2ea
pub fn i_04d2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D2EA; return 10; }
    8
}
/// $04D2D8: lea.l $5e766.l, a0
pub fn i_04d2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $04D2DE: jsr $5e770.l
pub fn i_04d2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2E4);
    cpu.pc = 0x5E770;
    16
}
/// $04D2E4: bclr.b #$3, $13(a6)
pub fn i_04d2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2EA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $04D2EA: jsr $28758.l
pub fn i_04d2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D2F0);
    cpu.pc = 0x28758;
    16
}
/// $04D2F0: bcc.w $4d326
pub fn i_04d2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x4D326; return 10; }
    8
}
/// $04D2F4: lea.l $77f6a.l, a1
pub fn i_04d2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D2FA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $04D2FA: jsr $4ae.l
pub fn i_04d2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D300;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D300);
    cpu.pc = 0x4AE;
    16
}
/// $04D300: jsr $5dd02.l
pub fn i_04d300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D306;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D306);
    cpu.pc = 0x5DD02;
    16
}
/// $04D306: move.w #$4000, $38(a0)
pub fn i_04d306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D30C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $04D30C: lea.l $293ff2.l, a1
pub fn i_04d30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D312;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x293FF2;
    20
}
/// $04D312: jsr $77c7e.l
pub fn i_04d312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D318);
    cpu.pc = 0x77C7E;
    16
}
/// $04D318: move.w #$1022, d0
pub fn i_04d318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D31C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $04D31C: jsr $2352.l
pub fn i_04d31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x4D322);
    cpu.pc = 0x2352;
    16
}
/// $04D322: bra.w $4d32e
pub fn i_04d322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D326;
    cpu.ir = 0x6000;
    cpu.pc = 0x4D32E;
    10
}
/// $04D326: jsr $4d5c8(pc)
pub fn i_04d326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x4D32A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x4D32A);
    cpu.pc = 0x4D5C8;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x04A878, i_04a878),
    (0x04A87C, i_04a87c),
    (0x04A882, i_04a882),
    (0x04A888, i_04a888),
    (0x04A948, i_04a948),
    (0x04A94C, i_04a94c),
    (0x04A952, i_04a952),
    (0x04A958, i_04a958),
    (0x04A95E, i_04a95e),
    (0x04A964, i_04a964),
    (0x04A968, i_04a968),
    (0x04A96C, i_04a96c),
    (0x04A970, i_04a970),
    (0x04A976, i_04a976),
    (0x04A97C, i_04a97c),
    (0x04A980, i_04a980),
    (0x04A982, i_04a982),
    (0x04A986, i_04a986),
    (0x04A98C, i_04a98c),
    (0x04A990, i_04a990),
    (0x04A994, i_04a994),
    (0x04A996, i_04a996),
    (0x04A99A, i_04a99a),
    (0x04A99E, i_04a99e),
    (0x04A9A2, i_04a9a2),
    (0x04A9A6, i_04a9a6),
    (0x04A9AA, i_04a9aa),
    (0x04A9AC, i_04a9ac),
    (0x04A9B0, i_04a9b0),
    (0x04A9B4, i_04a9b4),
    (0x04A9BA, i_04a9ba),
    (0x04A9BE, i_04a9be),
    (0x04A9C4, i_04a9c4),
    (0x04A9CA, i_04a9ca),
    (0x04A9CE, i_04a9ce),
    (0x04A9D4, i_04a9d4),
    (0x04A9D8, i_04a9d8),
    (0x04A9DC, i_04a9dc),
    (0x04A9E2, i_04a9e2),
    (0x04A9E6, i_04a9e6),
    (0x04A9EA, i_04a9ea),
    (0x04A9F0, i_04a9f0),
    (0x04A9F6, i_04a9f6),
    (0x04A9F8, i_04a9f8),
    (0x04A9FC, i_04a9fc),
    (0x04AA02, i_04aa02),
    (0x04AA06, i_04aa06),
    (0x04AA0C, i_04aa0c),
    (0x04AA12, i_04aa12),
    (0x04AA18, i_04aa18),
    (0x04AA1C, i_04aa1c),
    (0x04AA1E, i_04aa1e),
    (0x04AA22, i_04aa22),
    (0x04AA28, i_04aa28),
    (0x04AA2E, i_04aa2e),
    (0x04AA32, i_04aa32),
    (0x04AA36, i_04aa36),
    (0x04AA38, i_04aa38),
    (0x04AA3C, i_04aa3c),
    (0x04AA40, i_04aa40),
    (0x04AA44, i_04aa44),
    (0x04AA46, i_04aa46),
    (0x04AA4A, i_04aa4a),
    (0x04AA4E, i_04aa4e),
    (0x04AA54, i_04aa54),
    (0x04AA5A, i_04aa5a),
    (0x04AA5E, i_04aa5e),
    (0x04AA62, i_04aa62),
    (0x04AA66, i_04aa66),
    (0x04AA6C, i_04aa6c),
    (0x04AA72, i_04aa72),
    (0x04AA76, i_04aa76),
    (0x04AA7A, i_04aa7a),
    (0x04AA80, i_04aa80),
    (0x04AA84, i_04aa84),
    (0x04AA8A, i_04aa8a),
    (0x04AA8E, i_04aa8e),
    (0x04AA90, i_04aa90),
    (0x04AA96, i_04aa96),
    (0x04AA9A, i_04aa9a),
    (0x04AAA0, i_04aaa0),
    (0x04AAA4, i_04aaa4),
    (0x04AAA8, i_04aaa8),
    (0x04AAAC, i_04aaac),
    (0x04AAB2, i_04aab2),
    (0x04AAB8, i_04aab8),
    (0x04AABC, i_04aabc),
    (0x04AAC0, i_04aac0),
    (0x04AAC4, i_04aac4),
    (0x04AACA, i_04aaca),
    (0x04AAD0, i_04aad0),
    (0x04AAD2, i_04aad2),
    (0x04AAD6, i_04aad6),
    (0x04AADC, i_04aadc),
    (0x04AAE0, i_04aae0),
    (0x04AAE4, i_04aae4),
    (0x04AAEA, i_04aaea),
    (0x04AAF0, i_04aaf0),
    (0x04AAF4, i_04aaf4),
    (0x04AAFA, i_04aafa),
    (0x04AAFE, i_04aafe),
    (0x04AB04, i_04ab04),
    (0x04AB06, i_04ab06),
    (0x04AB0A, i_04ab0a),
    (0x04AB0E, i_04ab0e),
    (0x04AB10, i_04ab10),
    (0x04AB14, i_04ab14),
    (0x04AB1A, i_04ab1a),
    (0x04AB1E, i_04ab1e),
    (0x04AB22, i_04ab22),
    (0x04AB28, i_04ab28),
    (0x04AB2C, i_04ab2c),
    (0x04AB32, i_04ab32),
    (0x04AB38, i_04ab38),
    (0x04AB3E, i_04ab3e),
    (0x04AB42, i_04ab42),
    (0x04AB48, i_04ab48),
    (0x04AB4C, i_04ab4c),
    (0x04AB4E, i_04ab4e),
    (0x04AB52, i_04ab52),
    (0x04AB56, i_04ab56),
    (0x04AB5C, i_04ab5c),
    (0x04AB60, i_04ab60),
    (0x04AB64, i_04ab64),
    (0x04AB6A, i_04ab6a),
    (0x04AB6E, i_04ab6e),
    (0x04AB74, i_04ab74),
    (0x04AB7A, i_04ab7a),
    (0x04AB80, i_04ab80),
    (0x04AB84, i_04ab84),
    (0x04AB8A, i_04ab8a),
    (0x04AB8E, i_04ab8e),
    (0x04AB90, i_04ab90),
    (0x04AB96, i_04ab96),
    (0x04AB9C, i_04ab9c),
    (0x04ABA0, i_04aba0),
    (0x04ABA6, i_04aba6),
    (0x04ABA8, i_04aba8),
    (0x04ABAC, i_04abac),
    (0x04ABB0, i_04abb0),
    (0x04ABB2, i_04abb2),
    (0x04ABB8, i_04abb8),
    (0x04ABBE, i_04abbe),
    (0x04ABC0, i_04abc0),
    (0x04ABC4, i_04abc4),
    (0x04ABCA, i_04abca),
    (0x04ABD0, i_04abd0),
    (0x04ABD6, i_04abd6),
    (0x04ABDC, i_04abdc),
    (0x04ABE0, i_04abe0),
    (0x04ABE6, i_04abe6),
    (0x04ABEA, i_04abea),
    (0x04ABEE, i_04abee),
    (0x04ABF4, i_04abf4),
    (0x04ABF8, i_04abf8),
    (0x04ABFE, i_04abfe),
    (0x04AC02, i_04ac02),
    (0x04AC04, i_04ac04),
    (0x04AC0A, i_04ac0a),
    (0x04AC10, i_04ac10),
    (0x04AC14, i_04ac14),
    (0x04AC18, i_04ac18),
    (0x04AC1A, i_04ac1a),
    (0x04AC20, i_04ac20),
    (0x04AC26, i_04ac26),
    (0x04AC2A, i_04ac2a),
    (0x04AC2E, i_04ac2e),
    (0x04AC30, i_04ac30),
    (0x04AC32, i_04ac32),
    (0x04AC3A, i_04ac3a),
    (0x04AC3E, i_04ac3e),
    (0x04AC44, i_04ac44),
    (0x04AC48, i_04ac48),
    (0x04AC4E, i_04ac4e),
    (0x04AC52, i_04ac52),
    (0x04AC54, i_04ac54),
    (0x04B052, i_04b052),
    (0x04B056, i_04b056),
    (0x04B05A, i_04b05a),
    (0x04B060, i_04b060),
    (0x04B064, i_04b064),
    (0x04B06A, i_04b06a),
    (0x04B070, i_04b070),
    (0x04B076, i_04b076),
    (0x04BB8E, i_04bb8e),
    (0x04BB92, i_04bb92),
    (0x04BB94, i_04bb94),
    (0x04BB98, i_04bb98),
    (0x04BB9A, i_04bb9a),
    (0x04BBA0, i_04bba0),
    (0x04BBBE, i_04bbbe),
    (0x04BBC2, i_04bbc2),
    (0x04BBC8, i_04bbc8),
    (0x04BBCE, i_04bbce),
    (0x04BBD4, i_04bbd4),
    (0x04BBD8, i_04bbd8),
    (0x04BBDE, i_04bbde),
    (0x04BBE4, i_04bbe4),
    (0x04BBEA, i_04bbea),
    (0x04BBEC, i_04bbec),
    (0x04BBF0, i_04bbf0),
    (0x04BBF4, i_04bbf4),
    (0x04BBFA, i_04bbfa),
    (0x04BC00, i_04bc00),
    (0x04BC04, i_04bc04),
    (0x04BC0A, i_04bc0a),
    (0x04BC10, i_04bc10),
    (0x04BC16, i_04bc16),
    (0x04BC1A, i_04bc1a),
    (0x04BC1E, i_04bc1e),
    (0x04BC20, i_04bc20),
    (0x04BC24, i_04bc24),
    (0x04BC2A, i_04bc2a),
    (0x04BC2E, i_04bc2e),
    (0x04BC34, i_04bc34),
    (0x04BC3A, i_04bc3a),
    (0x04BC40, i_04bc40),
    (0x04BC44, i_04bc44),
    (0x04BC46, i_04bc46),
    (0x04BC48, i_04bc48),
    (0x04BC4E, i_04bc4e),
    (0x04BC52, i_04bc52),
    (0x04BC58, i_04bc58),
    (0x04BC5C, i_04bc5c),
    (0x04BC60, i_04bc60),
    (0x04BC66, i_04bc66),
    (0x04BC6C, i_04bc6c),
    (0x04BC72, i_04bc72),
    (0x04BC76, i_04bc76),
    (0x04BC7C, i_04bc7c),
    (0x04BC80, i_04bc80),
    (0x04BC84, i_04bc84),
    (0x04BC8A, i_04bc8a),
    (0x04BC90, i_04bc90),
    (0x04BC96, i_04bc96),
    (0x04BC9C, i_04bc9c),
    (0x04BCA0, i_04bca0),
    (0x04BCA4, i_04bca4),
    (0x04BCA8, i_04bca8),
    (0x04BCAC, i_04bcac),
    (0x04BCB2, i_04bcb2),
    (0x04BCB8, i_04bcb8),
    (0x04BCBE, i_04bcbe),
    (0x04BCC2, i_04bcc2),
    (0x04BCC6, i_04bcc6),
    (0x04BCC8, i_04bcc8),
    (0x04BCCA, i_04bcca),
    (0x04BCD0, i_04bcd0),
    (0x04BCD6, i_04bcd6),
    (0x04BCDA, i_04bcda),
    (0x04BCDE, i_04bcde),
    (0x04BCE2, i_04bce2),
    (0x04BCE6, i_04bce6),
    (0x04BCEC, i_04bcec),
    (0x04BCF2, i_04bcf2),
    (0x04BCF8, i_04bcf8),
    (0x04BCFC, i_04bcfc),
    (0x04BCFE, i_04bcfe),
    (0x04BD02, i_04bd02),
    (0x04BD06, i_04bd06),
    (0x04BD0A, i_04bd0a),
    (0x04BD0C, i_04bd0c),
    (0x04BD12, i_04bd12),
    (0x04BD18, i_04bd18),
    (0x04BD1C, i_04bd1c),
    (0x04BD22, i_04bd22),
    (0x04BD26, i_04bd26),
    (0x04BD2A, i_04bd2a),
    (0x04BD30, i_04bd30),
    (0x04BD34, i_04bd34),
    (0x04BD36, i_04bd36),
    (0x04BD3C, i_04bd3c),
    (0x04BD40, i_04bd40),
    (0x04BD46, i_04bd46),
    (0x04BD4C, i_04bd4c),
    (0x04BD52, i_04bd52),
    (0x04BD56, i_04bd56),
    (0x04BD5A, i_04bd5a),
    (0x04BD5C, i_04bd5c),
    (0x04BD5E, i_04bd5e),
    (0x04BD62, i_04bd62),
    (0x04BD66, i_04bd66),
    (0x04BD68, i_04bd68),
    (0x04BD90, i_04bd90),
    (0x04BD96, i_04bd96),
    (0x04BD9C, i_04bd9c),
    (0x04BDA0, i_04bda0),
    (0x04BDA6, i_04bda6),
    (0x04BDAA, i_04bdaa),
    (0x04BDAE, i_04bdae),
    (0x04BDB2, i_04bdb2),
    (0x04BDB6, i_04bdb6),
    (0x04BDBC, i_04bdbc),
    (0x04BDC0, i_04bdc0),
    (0x04BDC6, i_04bdc6),
    (0x04BDCA, i_04bdca),
    (0x04BDCC, i_04bdcc),
    (0x04BDD0, i_04bdd0),
    (0x04BDD6, i_04bdd6),
    (0x04BDDA, i_04bdda),
    (0x04BDDC, i_04bddc),
    (0x04BDE2, i_04bde2),
    (0x04BDE6, i_04bde6),
    (0x04BDEC, i_04bdec),
    (0x04BDF2, i_04bdf2),
    (0x04BDF8, i_04bdf8),
    (0x04BDFC, i_04bdfc),
    (0x04BE00, i_04be00),
    (0x04BE02, i_04be02),
    (0x04BE04, i_04be04),
    (0x04BE0A, i_04be0a),
    (0x04BE0E, i_04be0e),
    (0x04BE10, i_04be10),
    (0x04BE16, i_04be16),
    (0x04BE1C, i_04be1c),
    (0x04BE20, i_04be20),
    (0x04BE26, i_04be26),
    (0x04BE2A, i_04be2a),
    (0x04BE2E, i_04be2e),
    (0x04BE32, i_04be32),
    (0x04BE36, i_04be36),
    (0x04BE3A, i_04be3a),
    (0x04BE3E, i_04be3e),
    (0x04BE40, i_04be40),
    (0x04BE44, i_04be44),
    (0x04BE48, i_04be48),
    (0x04BE4A, i_04be4a),
    (0x04BE50, i_04be50),
    (0x04BE54, i_04be54),
    (0x04BE5A, i_04be5a),
    (0x04BE60, i_04be60),
    (0x04BE66, i_04be66),
    (0x04BE6A, i_04be6a),
    (0x04BE6E, i_04be6e),
    (0x04BE70, i_04be70),
    (0x04BE72, i_04be72),
    (0x04BE78, i_04be78),
    (0x04BE7C, i_04be7c),
    (0x04BE7E, i_04be7e),
    (0x04BE84, i_04be84),
    (0x04BE8A, i_04be8a),
    (0x04BE8E, i_04be8e),
    (0x04BE94, i_04be94),
    (0x04BE98, i_04be98),
    (0x04BE9C, i_04be9c),
    (0x04BEA0, i_04bea0),
    (0x04BEA4, i_04bea4),
    (0x04BEA8, i_04bea8),
    (0x04BEAC, i_04beac),
    (0x04BEAE, i_04beae),
    (0x04BEB2, i_04beb2),
    (0x04BEB6, i_04beb6),
    (0x04BEB8, i_04beb8),
    (0x04BEBE, i_04bebe),
    (0x04BEC2, i_04bec2),
    (0x04BEC8, i_04bec8),
    (0x04BECE, i_04bece),
    (0x04BED4, i_04bed4),
    (0x04BED8, i_04bed8),
    (0x04BEDC, i_04bedc),
    (0x04BEDE, i_04bede),
    (0x04BEE0, i_04bee0),
    (0x04BEE4, i_04bee4),
    (0x04BEEA, i_04beea),
    (0x04BEF0, i_04bef0),
    (0x04BEF6, i_04bef6),
    (0x04BEFA, i_04befa),
    (0x04BF00, i_04bf00),
    (0x04BF06, i_04bf06),
    (0x04BF0C, i_04bf0c),
    (0x04BF0E, i_04bf0e),
    (0x04BF12, i_04bf12),
    (0x04BF16, i_04bf16),
    (0x04BF1C, i_04bf1c),
    (0x04BF20, i_04bf20),
    (0x04BF22, i_04bf22),
    (0x04BF26, i_04bf26),
    (0x04BF2C, i_04bf2c),
    (0x04BF30, i_04bf30),
    (0x04BF34, i_04bf34),
    (0x04BF36, i_04bf36),
    (0x04BF3A, i_04bf3a),
    (0x04BF40, i_04bf40),
    (0x04BF46, i_04bf46),
    (0x04BF4C, i_04bf4c),
    (0x04BF50, i_04bf50),
    (0x04BF54, i_04bf54),
    (0x04BF56, i_04bf56),
    (0x04BF58, i_04bf58),
    (0x04BF5C, i_04bf5c),
    (0x04BF60, i_04bf60),
    (0x04BF66, i_04bf66),
    (0x04BF68, i_04bf68),
    (0x04BF6C, i_04bf6c),
    (0x04BF72, i_04bf72),
    (0x04BF76, i_04bf76),
    (0x04BF7C, i_04bf7c),
    (0x04BF80, i_04bf80),
    (0x04BF82, i_04bf82),
    (0x04BF86, i_04bf86),
    (0x04BF8C, i_04bf8c),
    (0x04BF92, i_04bf92),
    (0x04BF96, i_04bf96),
    (0x04BF9A, i_04bf9a),
    (0x04BFA0, i_04bfa0),
    (0x04BFA4, i_04bfa4),
    (0x04BFAA, i_04bfaa),
    (0x04BFAE, i_04bfae),
    (0x04BFB4, i_04bfb4),
    (0x04BFBA, i_04bfba),
    (0x04BFBE, i_04bfbe),
    (0x04BFC4, i_04bfc4),
    (0x04BFC8, i_04bfc8),
    (0x04BFCC, i_04bfcc),
    (0x04BFCE, i_04bfce),
    (0x04BFD2, i_04bfd2),
    (0x04BFD8, i_04bfd8),
    (0x04BFDE, i_04bfde),
    (0x04BFE4, i_04bfe4),
    (0x04BFE8, i_04bfe8),
    (0x04BFEC, i_04bfec),
    (0x04BFEE, i_04bfee),
    (0x04BFF0, i_04bff0),
    (0x04BFF4, i_04bff4),
    (0x04BFF8, i_04bff8),
    (0x04BFFE, i_04bffe),
    (0x04C000, i_04c000),
    (0x04C004, i_04c004),
    (0x04C00A, i_04c00a),
    (0x04C00E, i_04c00e),
    (0x04C014, i_04c014),
    (0x04C018, i_04c018),
    (0x04C01A, i_04c01a),
    (0x04C01E, i_04c01e),
    (0x04C024, i_04c024),
    (0x04C02A, i_04c02a),
    (0x04C02E, i_04c02e),
    (0x04C032, i_04c032),
    (0x04C038, i_04c038),
    (0x04C03C, i_04c03c),
    (0x04C042, i_04c042),
    (0x04C046, i_04c046),
    (0x04C04C, i_04c04c),
    (0x04C052, i_04c052),
    (0x04C056, i_04c056),
    (0x04C05C, i_04c05c),
    (0x04C060, i_04c060),
    (0x04C064, i_04c064),
    (0x04C066, i_04c066),
    (0x04C06A, i_04c06a),
    (0x04C070, i_04c070),
    (0x04C076, i_04c076),
    (0x04C07C, i_04c07c),
    (0x04C080, i_04c080),
    (0x04C084, i_04c084),
    (0x04C086, i_04c086),
    (0x04C088, i_04c088),
    (0x04C08C, i_04c08c),
    (0x04C092, i_04c092),
    (0x04C098, i_04c098),
    (0x04C09C, i_04c09c),
    (0x04C0A0, i_04c0a0),
    (0x04C0A6, i_04c0a6),
    (0x04C0AA, i_04c0aa),
    (0x04C0AE, i_04c0ae),
    (0x04C0B4, i_04c0b4),
    (0x04C0B6, i_04c0b6),
    (0x04C0BA, i_04c0ba),
    (0x04C0C0, i_04c0c0),
    (0x04C0C4, i_04c0c4),
    (0x04C0CA, i_04c0ca),
    (0x04C0CE, i_04c0ce),
    (0x04C0D4, i_04c0d4),
    (0x04C0D8, i_04c0d8),
    (0x04C0DE, i_04c0de),
    (0x04C0E4, i_04c0e4),
    (0x04C0E8, i_04c0e8),
    (0x04C0EC, i_04c0ec),
    (0x04C0F0, i_04c0f0),
    (0x04C0F4, i_04c0f4),
    (0x04C0F6, i_04c0f6),
    (0x04C0FC, i_04c0fc),
    (0x04C100, i_04c100),
    (0x04C104, i_04c104),
    (0x04C106, i_04c106),
    (0x04C10A, i_04c10a),
    (0x04C110, i_04c110),
    (0x04C116, i_04c116),
    (0x04C11C, i_04c11c),
    (0x04C120, i_04c120),
    (0x04C124, i_04c124),
    (0x04C126, i_04c126),
    (0x04C128, i_04c128),
    (0x04C12C, i_04c12c),
    (0x04C130, i_04c130),
    (0x04C136, i_04c136),
    (0x04C138, i_04c138),
    (0x04C13C, i_04c13c),
    (0x04C142, i_04c142),
    (0x04C146, i_04c146),
    (0x04C14C, i_04c14c),
    (0x04C150, i_04c150),
    (0x04C152, i_04c152),
    (0x04C156, i_04c156),
    (0x04C15C, i_04c15c),
    (0x04C162, i_04c162),
    (0x04C166, i_04c166),
    (0x04C16A, i_04c16a),
    (0x04C16E, i_04c16e),
    (0x04C174, i_04c174),
    (0x04C178, i_04c178),
    (0x04C17E, i_04c17e),
    (0x04C184, i_04c184),
    (0x04C188, i_04c188),
    (0x04C18C, i_04c18c),
    (0x04C18E, i_04c18e),
    (0x04C192, i_04c192),
    (0x04C198, i_04c198),
    (0x04C19E, i_04c19e),
    (0x04C1A4, i_04c1a4),
    (0x04C1A8, i_04c1a8),
    (0x04C1AC, i_04c1ac),
    (0x04C1AE, i_04c1ae),
    (0x04C1B0, i_04c1b0),
    (0x04C1B4, i_04c1b4),
    (0x04C1BA, i_04c1ba),
    (0x04C1C0, i_04c1c0),
    (0x04C1C6, i_04c1c6),
    (0x04C1CC, i_04c1cc),
    (0x04C1D0, i_04c1d0),
    (0x04C1D6, i_04c1d6),
    (0x04C1DC, i_04c1dc),
    (0x04C1E2, i_04c1e2),
    (0x04C1E4, i_04c1e4),
    (0x04C1E8, i_04c1e8),
    (0x04C1EC, i_04c1ec),
    (0x04C1F2, i_04c1f2),
    (0x04C1F8, i_04c1f8),
    (0x04C1FC, i_04c1fc),
    (0x04C202, i_04c202),
    (0x04C206, i_04c206),
    (0x04C208, i_04c208),
    (0x04C20C, i_04c20c),
    (0x04C212, i_04c212),
    (0x04C218, i_04c218),
    (0x04C21E, i_04c21e),
    (0x04C222, i_04c222),
    (0x04C226, i_04c226),
    (0x04C22C, i_04c22c),
    (0x04C230, i_04c230),
    (0x04C236, i_04c236),
    (0x04C23C, i_04c23c),
    (0x04C240, i_04c240),
    (0x04C246, i_04c246),
    (0x04C24A, i_04c24a),
    (0x04C24E, i_04c24e),
    (0x04C250, i_04c250),
    (0x04C254, i_04c254),
    (0x04C258, i_04c258),
    (0x04C25E, i_04c25e),
    (0x04C264, i_04c264),
    (0x04C26A, i_04c26a),
    (0x04C26E, i_04c26e),
    (0x04C272, i_04c272),
    (0x04C274, i_04c274),
    (0x04C276, i_04c276),
    (0x04C27A, i_04c27a),
    (0x04C280, i_04c280),
    (0x04C286, i_04c286),
    (0x04C28A, i_04c28a),
    (0x04C290, i_04c290),
    (0x04C294, i_04c294),
    (0x04C29A, i_04c29a),
    (0x04C2A0, i_04c2a0),
    (0x04C2A4, i_04c2a4),
    (0x04C2AA, i_04c2aa),
    (0x04C2AE, i_04c2ae),
    (0x04C2B4, i_04c2b4),
    (0x04C2B8, i_04c2b8),
    (0x04C2BC, i_04c2bc),
    (0x04C2BE, i_04c2be),
    (0x04C2C2, i_04c2c2),
    (0x04C2C6, i_04c2c6),
    (0x04C2CC, i_04c2cc),
    (0x04C2D2, i_04c2d2),
    (0x04C2D8, i_04c2d8),
    (0x04C2DC, i_04c2dc),
    (0x04C2E0, i_04c2e0),
    (0x04C2E2, i_04c2e2),
    (0x04C2E4, i_04c2e4),
    (0x04C2E8, i_04c2e8),
    (0x04C2EC, i_04c2ec),
    (0x04C2F2, i_04c2f2),
    (0x04C2F6, i_04c2f6),
    (0x04C2FC, i_04c2fc),
    (0x04C302, i_04c302),
    (0x04C308, i_04c308),
    (0x04C30E, i_04c30e),
    (0x04C314, i_04c314),
    (0x04C31A, i_04c31a),
    (0x04C31E, i_04c31e),
    (0x04C322, i_04c322),
    (0x04C328, i_04c328),
    (0x04C32E, i_04c32e),
    (0x04C334, i_04c334),
    (0x04C33A, i_04c33a),
    (0x04C340, i_04c340),
    (0x04C344, i_04c344),
    (0x04C346, i_04c346),
    (0x04C34C, i_04c34c),
    (0x04C352, i_04c352),
    (0x04C356, i_04c356),
    (0x04C35A, i_04c35a),
    (0x04C35E, i_04c35e),
    (0x04C362, i_04c362),
    (0x04C364, i_04c364),
    (0x04C368, i_04c368),
    (0x04C36E, i_04c36e),
    (0x04C374, i_04c374),
    (0x04C378, i_04c378),
    (0x04C37A, i_04c37a),
    (0x04C380, i_04c380),
    (0x04C386, i_04c386),
    (0x04C38A, i_04c38a),
    (0x04C38E, i_04c38e),
    (0x04C392, i_04c392),
    (0x04C396, i_04c396),
    (0x04C39A, i_04c39a),
    (0x04C39C, i_04c39c),
    (0x04C3A0, i_04c3a0),
    (0x04C3A6, i_04c3a6),
    (0x04C3AA, i_04c3aa),
    (0x04C3AE, i_04c3ae),
    (0x04C3B0, i_04c3b0),
    (0x04C3B4, i_04c3b4),
    (0x04C3BA, i_04c3ba),
    (0x04C3C0, i_04c3c0),
    (0x04C3C4, i_04c3c4),
    (0x04C3C6, i_04c3c6),
    (0x04C3CC, i_04c3cc),
    (0x04C3D2, i_04c3d2),
    (0x04C3D6, i_04c3d6),
    (0x04C3DC, i_04c3dc),
    (0x04C3E0, i_04c3e0),
    (0x04C3E4, i_04c3e4),
    (0x04C3E6, i_04c3e6),
    (0x04C3EA, i_04c3ea),
    (0x04C3F0, i_04c3f0),
    (0x04C3F6, i_04c3f6),
    (0x04C3FA, i_04c3fa),
    (0x04C3FC, i_04c3fc),
    (0x04C402, i_04c402),
    (0x04C408, i_04c408),
    (0x04C40C, i_04c40c),
    (0x04C410, i_04c410),
    (0x04C412, i_04c412),
    (0x04C416, i_04c416),
    (0x04C41C, i_04c41c),
    (0x04C420, i_04c420),
    (0x04C426, i_04c426),
    (0x04C428, i_04c428),
    (0x04C42C, i_04c42c),
    (0x04C432, i_04c432),
    (0x04C438, i_04c438),
    (0x04C43E, i_04c43e),
    (0x04C442, i_04c442),
    (0x04C446, i_04c446),
    (0x04C448, i_04c448),
    (0x04C44A, i_04c44a),
    (0x04C44E, i_04c44e),
    (0x04C452, i_04c452),
    (0x04C458, i_04c458),
    (0x04C45C, i_04c45c),
    (0x04C462, i_04c462),
    (0x04C468, i_04c468),
    (0x04C46E, i_04c46e),
    (0x04C474, i_04c474),
    (0x04C478, i_04c478),
    (0x04C47C, i_04c47c),
    (0x04C482, i_04c482),
    (0x04C488, i_04c488),
    (0x04C48E, i_04c48e),
    (0x04C494, i_04c494),
    (0x04C49A, i_04c49a),
    (0x04C49E, i_04c49e),
    (0x04C4A0, i_04c4a0),
    (0x04C4A6, i_04c4a6),
    (0x04C4AC, i_04c4ac),
    (0x04C4B0, i_04c4b0),
    (0x04C4B4, i_04c4b4),
    (0x04C4B8, i_04c4b8),
    (0x04C4BC, i_04c4bc),
    (0x04C4BE, i_04c4be),
    (0x04C4C2, i_04c4c2),
    (0x04C4C8, i_04c4c8),
    (0x04C4CE, i_04c4ce),
    (0x04C4D2, i_04c4d2),
    (0x04C4D4, i_04c4d4),
    (0x04C4DA, i_04c4da),
    (0x04C4E0, i_04c4e0),
    (0x04C4E4, i_04c4e4),
    (0x04C4E8, i_04c4e8),
    (0x04C4EC, i_04c4ec),
    (0x04C4F0, i_04c4f0),
    (0x04C4F4, i_04c4f4),
    (0x04C4F6, i_04c4f6),
    (0x04C4FA, i_04c4fa),
    (0x04C500, i_04c500),
    (0x04C504, i_04c504),
    (0x04C508, i_04c508),
    (0x04C50A, i_04c50a),
    (0x04C50E, i_04c50e),
    (0x04C514, i_04c514),
    (0x04C51A, i_04c51a),
    (0x04C51E, i_04c51e),
    (0x04C520, i_04c520),
    (0x04C526, i_04c526),
    (0x04C52C, i_04c52c),
    (0x04C530, i_04c530),
    (0x04C536, i_04c536),
    (0x04C53A, i_04c53a),
    (0x04C53E, i_04c53e),
    (0x04C540, i_04c540),
    (0x04C544, i_04c544),
    (0x04C54A, i_04c54a),
    (0x04C54E, i_04c54e),
    (0x04C554, i_04c554),
    (0x04C556, i_04c556),
    (0x04C55A, i_04c55a),
    (0x04C560, i_04c560),
    (0x04C566, i_04c566),
    (0x04C56C, i_04c56c),
    (0x04C570, i_04c570),
    (0x04C574, i_04c574),
    (0x04C576, i_04c576),
    (0x04C578, i_04c578),
    (0x04C57E, i_04c57e),
    (0x04C584, i_04c584),
    (0x04C58C, i_04c58c),
    (0x04C592, i_04c592),
    (0x04C596, i_04c596),
    (0x04C59C, i_04c59c),
    (0x04C5A2, i_04c5a2),
    (0x04C5A4, i_04c5a4),
    (0x04C5A8, i_04c5a8),
    (0x04C5AC, i_04c5ac),
    (0x04C5B0, i_04c5b0),
    (0x04C5B6, i_04c5b6),
    (0x04C5BC, i_04c5bc),
    (0x04C5C2, i_04c5c2),
    (0x04C5C8, i_04c5c8),
    (0x04C5CE, i_04c5ce),
    (0x04C5D4, i_04c5d4),
    (0x04C5D8, i_04c5d8),
    (0x04C5DA, i_04c5da),
    (0x04C5E0, i_04c5e0),
    (0x04C5E6, i_04c5e6),
    (0x04C5EA, i_04c5ea),
    (0x04C5EE, i_04c5ee),
    (0x04C5F4, i_04c5f4),
    (0x04C5F8, i_04c5f8),
    (0x04C5FE, i_04c5fe),
    (0x04C602, i_04c602),
    (0x04C604, i_04c604),
    (0x04C606, i_04c606),
    (0x04C60A, i_04c60a),
    (0x04C60E, i_04c60e),
    (0x04C614, i_04c614),
    (0x04C61A, i_04c61a),
    (0x04C61E, i_04c61e),
    (0x04C620, i_04c620),
    (0x04C622, i_04c622),
    (0x04C626, i_04c626),
    (0x04C62C, i_04c62c),
    (0x04C630, i_04c630),
    (0x04C636, i_04c636),
    (0x04C63A, i_04c63a),
    (0x04C640, i_04c640),
    (0x04C646, i_04c646),
    (0x04C64A, i_04c64a),
    (0x04C64C, i_04c64c),
    (0x04C650, i_04c650),
    (0x04C654, i_04c654),
    (0x04C656, i_04c656),
    (0x04C65C, i_04c65c),
    (0x04C662, i_04c662),
    (0x04C666, i_04c666),
    (0x04C66A, i_04c66a),
    (0x04C66C, i_04c66c),
    (0x04C672, i_04c672),
    (0x04C678, i_04c678),
    (0x04C67E, i_04c67e),
    (0x04C682, i_04c682),
    (0x04C686, i_04c686),
    (0x04C688, i_04c688),
    (0x04C68A, i_04c68a),
    (0x04C68E, i_04c68e),
    (0x04C692, i_04c692),
    (0x04C698, i_04c698),
    (0x04C69E, i_04c69e),
    (0x04C6A4, i_04c6a4),
    (0x04C6A8, i_04c6a8),
    (0x04C6AA, i_04c6aa),
    (0x04C6B0, i_04c6b0),
    (0x04C6B6, i_04c6b6),
    (0x04C6BC, i_04c6bc),
    (0x04C6C2, i_04c6c2),
    (0x04C6C8, i_04c6c8),
    (0x04C6CC, i_04c6cc),
    (0x04C6D0, i_04c6d0),
    (0x04C6D2, i_04c6d2),
    (0x04C6D4, i_04c6d4),
    (0x04C6DA, i_04c6da),
    (0x04C6DE, i_04c6de),
    (0x04C6E2, i_04c6e2),
    (0x04C6E6, i_04c6e6),
    (0x04C6E8, i_04c6e8),
    (0x04C6EA, i_04c6ea),
    (0x04C6EE, i_04c6ee),
    (0x04C6F4, i_04c6f4),
    (0x04C6F8, i_04c6f8),
    (0x04C6FE, i_04c6fe),
    (0x04C704, i_04c704),
    (0x04C70A, i_04c70a),
    (0x04C70E, i_04c70e),
    (0x04C714, i_04c714),
    (0x04C718, i_04c718),
    (0x04C71E, i_04c71e),
    (0x04C722, i_04c722),
    (0x04C728, i_04c728),
    (0x04C72E, i_04c72e),
    (0x04C732, i_04c732),
    (0x04C738, i_04c738),
    (0x04C73E, i_04c73e),
    (0x04C742, i_04c742),
    (0x04C748, i_04c748),
    (0x04C74E, i_04c74e),
    (0x04C754, i_04c754),
    (0x04C75A, i_04c75a),
    (0x04C760, i_04c760),
    (0x04C764, i_04c764),
    (0x04C768, i_04c768),
    (0x04C76A, i_04c76a),
    (0x04C76E, i_04c76e),
    (0x04C770, i_04c770),
    (0x04C774, i_04c774),
    (0x04C776, i_04c776),
    (0x04C77A, i_04c77a),
    (0x04C780, i_04c780),
    (0x04C786, i_04c786),
    (0x04C78C, i_04c78c),
    (0x04C790, i_04c790),
    (0x04C794, i_04c794),
    (0x04C796, i_04c796),
    (0x04C79C, i_04c79c),
    (0x04C7A2, i_04c7a2),
    (0x04C7A8, i_04c7a8),
    (0x04C7B0, i_04c7b0),
    (0x04C7B6, i_04c7b6),
    (0x04C7BC, i_04c7bc),
    (0x04C7C0, i_04c7c0),
    (0x04C7C2, i_04c7c2),
    (0x04C7C6, i_04c7c6),
    (0x04C7CC, i_04c7cc),
    (0x04C7D0, i_04c7d0),
    (0x04C7D4, i_04c7d4),
    (0x04C7D6, i_04c7d6),
    (0x04C7DC, i_04c7dc),
    (0x04C7E0, i_04c7e0),
    (0x04C7E4, i_04c7e4),
    (0x04C7EA, i_04c7ea),
    (0x04C7EE, i_04c7ee),
    (0x04C7F2, i_04c7f2),
    (0x04C7F6, i_04c7f6),
    (0x04C7FA, i_04c7fa),
    (0x04C800, i_04c800),
    (0x04C806, i_04c806),
    (0x04C80C, i_04c80c),
    (0x04C810, i_04c810),
    (0x04C816, i_04c816),
    (0x04C81A, i_04c81a),
    (0x04C81E, i_04c81e),
    (0x04C820, i_04c820),
    (0x04C826, i_04c826),
    (0x04C82A, i_04c82a),
    (0x04C82E, i_04c82e),
    (0x04C830, i_04c830),
    (0x04C832, i_04c832),
    (0x04C838, i_04c838),
    (0x04C83E, i_04c83e),
    (0x04C842, i_04c842),
    (0x04C844, i_04c844),
    (0x04C846, i_04c846),
    (0x04C84C, i_04c84c),
    (0x04C852, i_04c852),
    (0x04C856, i_04c856),
    (0x04C85A, i_04c85a),
    (0x04C85E, i_04c85e),
    (0x04C862, i_04c862),
    (0x04C866, i_04c866),
    (0x04C86A, i_04c86a),
    (0x04C86E, i_04c86e),
    (0x04C874, i_04c874),
    (0x04C87A, i_04c87a),
    (0x04C880, i_04c880),
    (0x04C886, i_04c886),
    (0x04C88C, i_04c88c),
    (0x04C890, i_04c890),
    (0x04C892, i_04c892),
    (0x04C896, i_04c896),
    (0x04C89A, i_04c89a),
    (0x04C8A0, i_04c8a0),
    (0x04C8A6, i_04c8a6),
    (0x04C8AA, i_04c8aa),
    (0x04C8B0, i_04c8b0),
    (0x04C8B6, i_04c8b6),
    (0x04C8BC, i_04c8bc),
    (0x04C8C0, i_04c8c0),
    (0x04C8C2, i_04c8c2),
    (0x04C8C8, i_04c8c8),
    (0x04C8CC, i_04c8cc),
    (0x04C8D2, i_04c8d2),
    (0x04C8D6, i_04c8d6),
    (0x04C8DA, i_04c8da),
    (0x04C8DC, i_04c8dc),
    (0x04C8E2, i_04c8e2),
    (0x04C8E8, i_04c8e8),
    (0x04C8EE, i_04c8ee),
    (0x04C8F2, i_04c8f2),
    (0x04C8F6, i_04c8f6),
    (0x04C8F8, i_04c8f8),
    (0x04C8FE, i_04c8fe),
    (0x04C904, i_04c904),
    (0x04C908, i_04c908),
    (0x04C90C, i_04c90c),
    (0x04C90E, i_04c90e),
    (0x04C910, i_04c910),
    (0x04C914, i_04c914),
    (0x04C91A, i_04c91a),
    (0x04C920, i_04c920),
    (0x04C926, i_04c926),
    (0x04C92E, i_04c92e),
    (0x04C934, i_04c934),
    (0x04C93A, i_04c93a),
    (0x04C942, i_04c942),
    (0x04C946, i_04c946),
    (0x04C94C, i_04c94c),
    (0x04C950, i_04c950),
    (0x04C954, i_04c954),
    (0x04C956, i_04c956),
    (0x04C958, i_04c958),
    (0x04C95E, i_04c95e),
    (0x04C962, i_04c962),
    (0x04C968, i_04c968),
    (0x04C96E, i_04c96e),
    (0x04C974, i_04c974),
    (0x04C97A, i_04c97a),
    (0x04C97E, i_04c97e),
    (0x04C982, i_04c982),
    (0x04C988, i_04c988),
    (0x04C98A, i_04c98a),
    (0x04C98E, i_04c98e),
    (0x04C992, i_04c992),
    (0x04C994, i_04c994),
    (0x04C996, i_04c996),
    (0x04C99A, i_04c99a),
    (0x04C99E, i_04c99e),
    (0x04C9A0, i_04c9a0),
    (0x04C9A2, i_04c9a2),
    (0x04C9A6, i_04c9a6),
    (0x04C9AC, i_04c9ac),
    (0x04C9AE, i_04c9ae),
    (0x04C9B2, i_04c9b2),
    (0x04C9B6, i_04c9b6),
    (0x04C9BA, i_04c9ba),
    (0x04C9BC, i_04c9bc),
    (0x04C9BE, i_04c9be),
    (0x04C9C2, i_04c9c2),
    (0x04C9C6, i_04c9c6),
    (0x04C9C8, i_04c9c8),
    (0x04C9CA, i_04c9ca),
    (0x04C9CE, i_04c9ce),
    (0x04C9D4, i_04c9d4),
    (0x04C9D6, i_04c9d6),
    (0x04C9DA, i_04c9da),
    (0x04C9DE, i_04c9de),
    (0x04C9E2, i_04c9e2),
    (0x04C9E6, i_04c9e6),
    (0x04C9E8, i_04c9e8),
    (0x04C9EC, i_04c9ec),
    (0x04C9EE, i_04c9ee),
    (0x04C9F2, i_04c9f2),
    (0x04C9F6, i_04c9f6),
    (0x04C9FA, i_04c9fa),
    (0x04C9FE, i_04c9fe),
    (0x04CA02, i_04ca02),
    (0x04CA06, i_04ca06),
    (0x04CA08, i_04ca08),
    (0x04CA0C, i_04ca0c),
    (0x04CA0E, i_04ca0e),
    (0x04CA14, i_04ca14),
    (0x04CA1A, i_04ca1a),
    (0x04CA1E, i_04ca1e),
    (0x04CA22, i_04ca22),
    (0x04CA24, i_04ca24),
    (0x04CA2A, i_04ca2a),
    (0x04CA30, i_04ca30),
    (0x04CA34, i_04ca34),
    (0x04CA3A, i_04ca3a),
    (0x04CA40, i_04ca40),
    (0x04CA44, i_04ca44),
    (0x04CA48, i_04ca48),
    (0x04CA4A, i_04ca4a),
    (0x04CA50, i_04ca50),
    (0x04CA56, i_04ca56),
    (0x04CA58, i_04ca58),
    (0x04CA5C, i_04ca5c),
    (0x04CA62, i_04ca62),
    (0x04CA68, i_04ca68),
    (0x04CA6E, i_04ca6e),
    (0x04CA74, i_04ca74),
    (0x04CA7A, i_04ca7a),
    (0x04CA7C, i_04ca7c),
    (0x04CA80, i_04ca80),
    (0x04CA86, i_04ca86),
    (0x04CA8C, i_04ca8c),
    (0x04CA92, i_04ca92),
    (0x04CA96, i_04ca96),
    (0x04CA98, i_04ca98),
    (0x04CA9C, i_04ca9c),
    (0x04CAA0, i_04caa0),
    (0x04CAA6, i_04caa6),
    (0x04CAA8, i_04caa8),
    (0x04CAAC, i_04caac),
    (0x04CAB0, i_04cab0),
    (0x04CAB4, i_04cab4),
    (0x04CAB8, i_04cab8),
    (0x04CABC, i_04cabc),
    (0x04CABE, i_04cabe),
    (0x04CAC2, i_04cac2),
    (0x04CAC4, i_04cac4),
    (0x04CACA, i_04caca),
    (0x04CACE, i_04cace),
    (0x04CAD2, i_04cad2),
    (0x04CAD6, i_04cad6),
    (0x04CADA, i_04cada),
    (0x04CADE, i_04cade),
    (0x04CAE2, i_04cae2),
    (0x04CAE4, i_04cae4),
    (0x04CAE6, i_04cae6),
    (0x04CAE8, i_04cae8),
    (0x04CAEC, i_04caec),
    (0x04CAF2, i_04caf2),
    (0x04CAF6, i_04caf6),
    (0x04CAFA, i_04cafa),
    (0x04CAFE, i_04cafe),
    (0x04CB04, i_04cb04),
    (0x04CB08, i_04cb08),
    (0x04CB0C, i_04cb0c),
    (0x04CB10, i_04cb10),
    (0x04CB16, i_04cb16),
    (0x04CB1A, i_04cb1a),
    (0x04CB1E, i_04cb1e),
    (0x04CB22, i_04cb22),
    (0x04CB28, i_04cb28),
    (0x04CB2A, i_04cb2a),
    (0x04CB2E, i_04cb2e),
    (0x04CB32, i_04cb32),
    (0x04CB36, i_04cb36),
    (0x04CB3A, i_04cb3a),
    (0x04CB3E, i_04cb3e),
    (0x04CB42, i_04cb42),
    (0x04CB5C, i_04cb5c),
    (0x04CB66, i_04cb66),
    (0x04CB68, i_04cb68),
    (0x04CB6E, i_04cb6e),
    (0x04CB72, i_04cb72),
    (0x04CB76, i_04cb76),
    (0x04CB7A, i_04cb7a),
    (0x04CB7C, i_04cb7c),
    (0x04CB7E, i_04cb7e),
    (0x04CB86, i_04cb86),
    (0x04CB88, i_04cb88),
    (0x04CB8C, i_04cb8c),
    (0x04CB94, i_04cb94),
    (0x04CB98, i_04cb98),
    (0x04CB9A, i_04cb9a),
    (0x04CB9C, i_04cb9c),
    (0x04CBA0, i_04cba0),
    (0x04CBA4, i_04cba4),
    (0x04CBA6, i_04cba6),
    (0x04CBA8, i_04cba8),
    (0x04CBAA, i_04cbaa),
    (0x04CBB0, i_04cbb0),
    (0x04CBB6, i_04cbb6),
    (0x04CBB8, i_04cbb8),
    (0x04CBBC, i_04cbbc),
    (0x04CBC0, i_04cbc0),
    (0x04CBC4, i_04cbc4),
    (0x04CBC8, i_04cbc8),
    (0x04CBCC, i_04cbcc),
    (0x04CBCE, i_04cbce),
    (0x04CBD2, i_04cbd2),
    (0x04CBD4, i_04cbd4),
    (0x04CBDA, i_04cbda),
    (0x04CBE0, i_04cbe0),
    (0x04CBE6, i_04cbe6),
    (0x04CBFC, i_04cbfc),
    (0x04CC00, i_04cc00),
    (0x04CC06, i_04cc06),
    (0x04CC0C, i_04cc0c),
    (0x04CC12, i_04cc12),
    (0x04CC18, i_04cc18),
    (0x04CC1E, i_04cc1e),
    (0x04CC24, i_04cc24),
    (0x04CC2A, i_04cc2a),
    (0x04CC30, i_04cc30),
    (0x04CC36, i_04cc36),
    (0x04CC3A, i_04cc3a),
    (0x04CC40, i_04cc40),
    (0x04CC46, i_04cc46),
    (0x04CC4A, i_04cc4a),
    (0x04CC4E, i_04cc4e),
    (0x04CC52, i_04cc52),
    (0x04CC56, i_04cc56),
    (0x04CC5A, i_04cc5a),
    (0x04CC5E, i_04cc5e),
    (0x04CC64, i_04cc64),
    (0x04CC6A, i_04cc6a),
    (0x04CC70, i_04cc70),
    (0x04CC76, i_04cc76),
    (0x04CC7C, i_04cc7c),
    (0x04CC80, i_04cc80),
    (0x04CC82, i_04cc82),
    (0x04CC86, i_04cc86),
    (0x04CC8A, i_04cc8a),
    (0x04CC8E, i_04cc8e),
    (0x04CC94, i_04cc94),
    (0x04CC98, i_04cc98),
    (0x04CC9C, i_04cc9c),
    (0x04CCA0, i_04cca0),
    (0x04CCA4, i_04cca4),
    (0x04CCA8, i_04cca8),
    (0x04CCAC, i_04ccac),
    (0x04CCB0, i_04ccb0),
    (0x04CCB4, i_04ccb4),
    (0x04CCBA, i_04ccba),
    (0x04CCC0, i_04ccc0),
    (0x04CCC6, i_04ccc6),
    (0x04CCCA, i_04ccca),
    (0x04CCD0, i_04ccd0),
    (0x04CCD4, i_04ccd4),
    (0x04CCD8, i_04ccd8),
    (0x04CCDC, i_04ccdc),
    (0x04CCE2, i_04cce2),
    (0x04CCE8, i_04cce8),
    (0x04CCEA, i_04ccea),
    (0x04CCF0, i_04ccf0),
    (0x04CCF6, i_04ccf6),
    (0x04CCFC, i_04ccfc),
    (0x04CD00, i_04cd00),
    (0x04CD04, i_04cd04),
    (0x04CD08, i_04cd08),
    (0x04CD0C, i_04cd0c),
    (0x04CD10, i_04cd10),
    (0x04CD16, i_04cd16),
    (0x04CD1A, i_04cd1a),
    (0x04CD1E, i_04cd1e),
    (0x04CD20, i_04cd20),
    (0x04CD24, i_04cd24),
    (0x04CD2A, i_04cd2a),
    (0x04CD2E, i_04cd2e),
    (0x04CD34, i_04cd34),
    (0x04CD3A, i_04cd3a),
    (0x04CD3E, i_04cd3e),
    (0x04CD40, i_04cd40),
    (0x04CD44, i_04cd44),
    (0x04CD48, i_04cd48),
    (0x04CD4C, i_04cd4c),
    (0x04CD52, i_04cd52),
    (0x04CD58, i_04cd58),
    (0x04CD5E, i_04cd5e),
    (0x04CD62, i_04cd62),
    (0x04CD66, i_04cd66),
    (0x04CD6A, i_04cd6a),
    (0x04CD6E, i_04cd6e),
    (0x04CD74, i_04cd74),
    (0x04CD78, i_04cd78),
    (0x04CD7E, i_04cd7e),
    (0x04CD80, i_04cd80),
    (0x04CD86, i_04cd86),
    (0x04CD8C, i_04cd8c),
    (0x04CD90, i_04cd90),
    (0x04CD94, i_04cd94),
    (0x04CD96, i_04cd96),
    (0x04CD9A, i_04cd9a),
    (0x04CD9E, i_04cd9e),
    (0x04CDA2, i_04cda2),
    (0x04CDA8, i_04cda8),
    (0x04CDAC, i_04cdac),
    (0x04CDB0, i_04cdb0),
    (0x04CDB4, i_04cdb4),
    (0x04CDB8, i_04cdb8),
    (0x04CDBC, i_04cdbc),
    (0x04CDC0, i_04cdc0),
    (0x04CDC4, i_04cdc4),
    (0x04CDC8, i_04cdc8),
    (0x04CDCC, i_04cdcc),
    (0x04CDD2, i_04cdd2),
    (0x04CDD8, i_04cdd8),
    (0x04CDDE, i_04cdde),
    (0x04CDE2, i_04cde2),
    (0x04CDE8, i_04cde8),
    (0x04CDEC, i_04cdec),
    (0x04CDF0, i_04cdf0),
    (0x04CDF4, i_04cdf4),
    (0x04CDFA, i_04cdfa),
    (0x04CE00, i_04ce00),
    (0x04CE02, i_04ce02),
    (0x04CE08, i_04ce08),
    (0x04CE0E, i_04ce0e),
    (0x04CE14, i_04ce14),
    (0x04CE18, i_04ce18),
    (0x04CE1C, i_04ce1c),
    (0x04CE22, i_04ce22),
    (0x04CE26, i_04ce26),
    (0x04CE2A, i_04ce2a),
    (0x04CE2E, i_04ce2e),
    (0x04CE34, i_04ce34),
    (0x04CE3A, i_04ce3a),
    (0x04CE3E, i_04ce3e),
    (0x04CE40, i_04ce40),
    (0x04CE44, i_04ce44),
    (0x04CE48, i_04ce48),
    (0x04CE4C, i_04ce4c),
    (0x04CE50, i_04ce50),
    (0x04CE56, i_04ce56),
    (0x04CE5A, i_04ce5a),
    (0x04CE60, i_04ce60),
    (0x04CE62, i_04ce62),
    (0x04CE68, i_04ce68),
    (0x04CE6C, i_04ce6c),
    (0x04CE6E, i_04ce6e),
    (0x04CE72, i_04ce72),
    (0x04CE76, i_04ce76),
    (0x04CE7C, i_04ce7c),
    (0x04CE82, i_04ce82),
    (0x04CE88, i_04ce88),
    (0x04CE8C, i_04ce8c),
    (0x04CE8E, i_04ce8e),
    (0x04CE92, i_04ce92),
    (0x04CE96, i_04ce96),
    (0x04CE9A, i_04ce9a),
    (0x04CE9E, i_04ce9e),
    (0x04CEA2, i_04cea2),
    (0x04CEA8, i_04cea8),
    (0x04CEAC, i_04ceac),
    (0x04CEB0, i_04ceb0),
    (0x04CEB6, i_04ceb6),
    (0x04CEBA, i_04ceba),
    (0x04CEBC, i_04cebc),
    (0x04CEC0, i_04cec0),
    (0x04CEC4, i_04cec4),
    (0x04CEC8, i_04cec8),
    (0x04CECE, i_04cece),
    (0x04CED2, i_04ced2),
    (0x04CED8, i_04ced8),
    (0x04CEDC, i_04cedc),
    (0x04CEE0, i_04cee0),
    (0x04CEE4, i_04cee4),
    (0x04CEEA, i_04ceea),
    (0x04CEF0, i_04cef0),
    (0x04CEF2, i_04cef2),
    (0x04CEF8, i_04cef8),
    (0x04CEFE, i_04cefe),
    (0x04CF04, i_04cf04),
    (0x04CF08, i_04cf08),
    (0x04CF0C, i_04cf0c),
    (0x04CF10, i_04cf10),
    (0x04CF14, i_04cf14),
    (0x04CF18, i_04cf18),
    (0x04CF1C, i_04cf1c),
    (0x04CF22, i_04cf22),
    (0x04CF26, i_04cf26),
    (0x04CF2C, i_04cf2c),
    (0x04CF2E, i_04cf2e),
    (0x04CF32, i_04cf32),
    (0x04CF38, i_04cf38),
    (0x04CF3E, i_04cf3e),
    (0x04CF44, i_04cf44),
    (0x04CF4A, i_04cf4a),
    (0x04CF50, i_04cf50),
    (0x04CF56, i_04cf56),
    (0x04CF5A, i_04cf5a),
    (0x04CF60, i_04cf60),
    (0x04CF64, i_04cf64),
    (0x04CF6A, i_04cf6a),
    (0x04CF70, i_04cf70),
    (0x04CF74, i_04cf74),
    (0x04CF76, i_04cf76),
    (0x04CF7A, i_04cf7a),
    (0x04CF80, i_04cf80),
    (0x04CF86, i_04cf86),
    (0x04CF8C, i_04cf8c),
    (0x04CF90, i_04cf90),
    (0x04CF94, i_04cf94),
    (0x04CF9A, i_04cf9a),
    (0x04CF9E, i_04cf9e),
    (0x04CFA4, i_04cfa4),
    (0x04CFAA, i_04cfaa),
    (0x04CFB0, i_04cfb0),
    (0x04CFB4, i_04cfb4),
    (0x04CFBA, i_04cfba),
    (0x04CFBE, i_04cfbe),
    (0x04CFC2, i_04cfc2),
    (0x04CFC6, i_04cfc6),
    (0x04CFCA, i_04cfca),
    (0x04CFD0, i_04cfd0),
    (0x04CFD4, i_04cfd4),
    (0x04CFDA, i_04cfda),
    (0x04CFDE, i_04cfde),
    (0x04CFE4, i_04cfe4),
    (0x04CFEA, i_04cfea),
    (0x04CFF0, i_04cff0),
    (0x04CFF6, i_04cff6),
    (0x04CFFA, i_04cffa),
    (0x04CFFE, i_04cffe),
    (0x04D004, i_04d004),
    (0x04D008, i_04d008),
    (0x04D00E, i_04d00e),
    (0x04D014, i_04d014),
    (0x04D01A, i_04d01a),
    (0x04D020, i_04d020),
    (0x04D026, i_04d026),
    (0x04D02A, i_04d02a),
    (0x04D030, i_04d030),
    (0x04D034, i_04d034),
    (0x04D03A, i_04d03a),
    (0x04D040, i_04d040),
    (0x04D046, i_04d046),
    (0x04D048, i_04d048),
    (0x04D04C, i_04d04c),
    (0x04D052, i_04d052),
    (0x04D056, i_04d056),
    (0x04D05A, i_04d05a),
    (0x04D05C, i_04d05c),
    (0x04D060, i_04d060),
    (0x04D064, i_04d064),
    (0x04D06A, i_04d06a),
    (0x04D070, i_04d070),
    (0x04D076, i_04d076),
    (0x04D07C, i_04d07c),
    (0x04D080, i_04d080),
    (0x04D086, i_04d086),
    (0x04D08A, i_04d08a),
    (0x04D090, i_04d090),
    (0x04D096, i_04d096),
    (0x04D09C, i_04d09c),
    (0x04D0A2, i_04d0a2),
    (0x04D0A6, i_04d0a6),
    (0x04D0A8, i_04d0a8),
    (0x04D0AE, i_04d0ae),
    (0x04D0B2, i_04d0b2),
    (0x04D0B8, i_04d0b8),
    (0x04D0BC, i_04d0bc),
    (0x04D0C0, i_04d0c0),
    (0x04D0C6, i_04d0c6),
    (0x04D0CA, i_04d0ca),
    (0x04D0CC, i_04d0cc),
    (0x04D0D0, i_04d0d0),
    (0x04D0D6, i_04d0d6),
    (0x04D0DA, i_04d0da),
    (0x04D0E0, i_04d0e0),
    (0x04D0E2, i_04d0e2),
    (0x04D0E8, i_04d0e8),
    (0x04D0EE, i_04d0ee),
    (0x04D0F4, i_04d0f4),
    (0x04D0FA, i_04d0fa),
    (0x04D0FE, i_04d0fe),
    (0x04D100, i_04d100),
    (0x04D106, i_04d106),
    (0x04D10A, i_04d10a),
    (0x04D110, i_04d110),
    (0x04D114, i_04d114),
    (0x04D11A, i_04d11a),
    (0x04D11E, i_04d11e),
    (0x04D122, i_04d122),
    (0x04D124, i_04d124),
    (0x04D128, i_04d128),
    (0x04D12E, i_04d12e),
    (0x04D132, i_04d132),
    (0x04D138, i_04d138),
    (0x04D13A, i_04d13a),
    (0x04D13E, i_04d13e),
    (0x04D144, i_04d144),
    (0x04D14A, i_04d14a),
    (0x04D150, i_04d150),
    (0x04D154, i_04d154),
    (0x04D156, i_04d156),
    (0x04D15C, i_04d15c),
    (0x04D160, i_04d160),
    (0x04D166, i_04d166),
    (0x04D16A, i_04d16a),
    (0x04D16E, i_04d16e),
    (0x04D170, i_04d170),
    (0x04D174, i_04d174),
    (0x04D17A, i_04d17a),
    (0x04D17E, i_04d17e),
    (0x04D184, i_04d184),
    (0x04D186, i_04d186),
    (0x04D18A, i_04d18a),
    (0x04D18E, i_04d18e),
    (0x04D194, i_04d194),
    (0x04D19A, i_04d19a),
    (0x04D1A0, i_04d1a0),
    (0x04D1A6, i_04d1a6),
    (0x04D1AA, i_04d1aa),
    (0x04D1AE, i_04d1ae),
    (0x04D1B4, i_04d1b4),
    (0x04D1BA, i_04d1ba),
    (0x04D1BE, i_04d1be),
    (0x04D1C2, i_04d1c2),
    (0x04D1C6, i_04d1c6),
    (0x04D1C8, i_04d1c8),
    (0x04D1CC, i_04d1cc),
    (0x04D1CE, i_04d1ce),
    (0x04D1D2, i_04d1d2),
    (0x04D1D6, i_04d1d6),
    (0x04D1D8, i_04d1d8),
    (0x04D1DC, i_04d1dc),
    (0x04D1E0, i_04d1e0),
    (0x04D1E6, i_04d1e6),
    (0x04D1EA, i_04d1ea),
    (0x04D1EE, i_04d1ee),
    (0x04D1F2, i_04d1f2),
    (0x04D1F6, i_04d1f6),
    (0x04D1FA, i_04d1fa),
    (0x04D200, i_04d200),
    (0x04D204, i_04d204),
    (0x04D208, i_04d208),
    (0x04D20E, i_04d20e),
    (0x04D212, i_04d212),
    (0x04D218, i_04d218),
    (0x04D21C, i_04d21c),
    (0x04D220, i_04d220),
    (0x04D224, i_04d224),
    (0x04D228, i_04d228),
    (0x04D22C, i_04d22c),
    (0x04D230, i_04d230),
    (0x04D234, i_04d234),
    (0x04D238, i_04d238),
    (0x04D23C, i_04d23c),
    (0x04D240, i_04d240),
    (0x04D244, i_04d244),
    (0x04D248, i_04d248),
    (0x04D24C, i_04d24c),
    (0x04D250, i_04d250),
    (0x04D254, i_04d254),
    (0x04D258, i_04d258),
    (0x04D25E, i_04d25e),
    (0x04D262, i_04d262),
    (0x04D266, i_04d266),
    (0x04D26A, i_04d26a),
    (0x04D270, i_04d270),
    (0x04D274, i_04d274),
    (0x04D278, i_04d278),
    (0x04D27C, i_04d27c),
    (0x04D282, i_04d282),
    (0x04D286, i_04d286),
    (0x04D28A, i_04d28a),
    (0x04D290, i_04d290),
    (0x04D296, i_04d296),
    (0x04D29A, i_04d29a),
    (0x04D29E, i_04d29e),
    (0x04D2A4, i_04d2a4),
    (0x04D2AA, i_04d2aa),
    (0x04D2AE, i_04d2ae),
    (0x04D2B2, i_04d2b2),
    (0x04D2B6, i_04d2b6),
    (0x04D2BA, i_04d2ba),
    (0x04D2BE, i_04d2be),
    (0x04D2C4, i_04d2c4),
    (0x04D2CA, i_04d2ca),
    (0x04D2CE, i_04d2ce),
    (0x04D2D4, i_04d2d4),
    (0x04D2D8, i_04d2d8),
    (0x04D2DE, i_04d2de),
    (0x04D2E4, i_04d2e4),
    (0x04D2EA, i_04d2ea),
    (0x04D2F0, i_04d2f0),
    (0x04D2F4, i_04d2f4),
    (0x04D2FA, i_04d2fa),
    (0x04D300, i_04d300),
    (0x04D306, i_04d306),
    (0x04D30C, i_04d30c),
    (0x04D312, i_04d312),
    (0x04D318, i_04d318),
    (0x04D31C, i_04d31c),
    (0x04D322, i_04d322),
    (0x04D326, i_04d326),
];
