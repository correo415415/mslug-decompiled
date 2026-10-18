// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p09_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $09B0C6: jsr $9b920(pc)
pub fn i_09b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B0CA);
    cpu.pc = 0x9B920;
    16
}
/// $09B0CA: bcc.w $9b0d4
pub fn i_09b0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0CE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B0D4; return 10; }
    8
}
/// $09B0CE: lea.l $9b474(pc), a1
pub fn i_09b0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B0D2: move.l a1, (a6)
pub fn i_09b0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B0D4: bra.w $9a75a
pub fn i_09b0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Food_Blink_09b0d8  ($09B0D8, 6 B, props/items_score_crates_09a0bc.s)
/// $09B0D8: move.b #$1e, $70(a6)
pub fn i_09b0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0DE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Item_Food_BlinkLoop_09b0de  ($09B0DE, 368 B, props/items_score_crates_09a0bc.s)
/// $09B0DE: move.w #$8000, d0
pub fn i_09b0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0E2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B0E2: jsr $28134.l
pub fn i_09b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B0E8);
    cpu.pc = 0x28134;
    16
}
/// $09B0E8: andi.w #$ffe3, $38(a6)
pub fn i_09b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0EE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B0EE: ori.w #$0, $38(a6)
pub fn i_09b0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0F4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B0F4: lea.l $9b0fa(pc), a1
pub fn i_09b0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0FA;
    16
}
/// $09B0F8: move.l a1, (a6)
pub fn i_09b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B0FA: jsr $9b8c0(pc)
pub fn i_09b0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B0FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B0FE);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B0FE: btst.b #$0, $70(a6)
pub fn i_09b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B104;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09B104: beq.w $9b10e
pub fn i_09b104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B108;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B10E; return 10; }
    8
}
/// $09B108: jsr $28d70.l
pub fn i_09b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B10E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B10E);
    cpu.pc = 0x28D70;
    16
}
/// $09B10E: subq.b #$1, $70(a6)
pub fn i_09b10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B112;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B112: bcc.w $9b11c
pub fn i_09b112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B116;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B11C; return 10; }
    8
}
/// $09B116: lea.l $9b474(pc), a1
pub fn i_09b116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B11A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B11A: move.l a1, (a6)
pub fn i_09b11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B11C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B11C: bra.b $9b0c6
pub fn i_09b11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B11E;
    cpu.ir = 0x60A8;
    cpu.pc = 0x9B0C6;
    10
}
/// $09B1CA: clr.b $9c(a6)
pub fn i_09b1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1CE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B1CE: cmpi.b #$ff, $9a(a6)
pub fn i_09b1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B1D4: beq.w $9b1de
pub fn i_09b1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B1DE; return 10; }
    8
}
/// $09B1D8: eori.b #$1, $3a(a6)
pub fn i_09b1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1DE;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B1DE: jsr $9bb3c(pc)
pub fn i_09b1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B1E2);
    cpu.pc = 0x9BB3C;
    16
}
/// $09B1E2: cmpi.w #$46, d0
pub fn i_09b1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1E6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x46, Size::Word, false);
    8
}
/// $09B1E6: bgt.w $9b1ee
pub fn i_09b1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B1EE; return 10; }
    8
}
/// $09B1EA: move.w #$46, d0
pub fn i_09b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x46;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B1EE: subi.w #$46, d0
pub fn i_09b1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1F2;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x46, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B1F2: move.w d0, $70(a6)
pub fn i_09b1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1F6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B1F6: clr.w $72(a6)
pub fn i_09b1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1FA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B1FA: move.w #$1c6, d1
pub fn i_09b1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B1FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B1FE: jsr $236e.l
pub fn i_09b1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B204);
    cpu.pc = 0x236E;
    16
}
/// $09B204: move.w #$8000, d0
pub fn i_09b204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B208;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B208: jsr $28134.l
pub fn i_09b208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B20E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B20E);
    cpu.pc = 0x28134;
    16
}
/// $09B20E: andi.w #$ffe3, $38(a6)
pub fn i_09b20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B214;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B214: ori.w #$0, $38(a6)
pub fn i_09b214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B21A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B21A: lea.l $2f6f06.l, a0
pub fn i_09b21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B220;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6F06;
    20
}
/// $09B220: jsr $28cd4.l
pub fn i_09b220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B226);
    cpu.pc = 0x28CD4;
    16
}
/// $09B226: lea.l $9b22c(pc), a1
pub fn i_09b226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B22A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B22C;
    16
}
/// $09B22A: move.l a1, (a6)
pub fn i_09b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B22C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B22C: jsr $9b8c0(pc)
pub fn i_09b22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B230;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B230);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B230: jsr $28d70.l
pub fn i_09b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B236);
    cpu.pc = 0x28D70;
    16
}
/// $09B236: subq.w #$1, $70(a6)
pub fn i_09b236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B23A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B23A: cmpi.w #$0, $70(a6)
pub fn i_09b23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B240;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B240: bgt.w $9b24a
pub fn i_09b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B244;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B24A; return 10; }
    8
}
/// $09B244: lea.l $9b24e(pc), a1
pub fn i_09b244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B248;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B24E;
    16
}
/// $09B248: move.l a1, (a6)
pub fn i_09b248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B24A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B24A: bra.w $9b0c6
pub fn i_09b24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B24E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B0C6;
    10
}
// ===== Item_Food_Rot_09b24e  ($09B24E, 64 B, props/items_score_crates_09a0bc.s)
/// $09B24E: lea.l $2f6f34.l, a0
pub fn i_09b24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B254;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F6F34;
    20
}
/// $09B254: jsr $28cd4.l
pub fn i_09b254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B25A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B25A);
    cpu.pc = 0x28CD4;
    16
}
/// $09B25A: move.w #$8000, d0
pub fn i_09b25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B25E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B25E: jsr $28134.l
pub fn i_09b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B264);
    cpu.pc = 0x28134;
    16
}
/// $09B264: andi.w #$ffe3, $38(a6)
pub fn i_09b264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B26A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B26A: ori.w #$0, $38(a6)
pub fn i_09b26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B270;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B270: lea.l $9b276(pc), a1
pub fn i_09b270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B276;
    16
}
/// $09B274: move.l a1, (a6)
pub fn i_09b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B276;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B276: jsr $9b8c0(pc)
pub fn i_09b276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B27A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B27A);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B27A: jsr $28d70.l
pub fn i_09b27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B280);
    cpu.pc = 0x28D70;
    16
}
/// $09B280: bcc.w $9b28a
pub fn i_09b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B284;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B28A; return 10; }
    8
}
/// $09B284: lea.l $9b474(pc), a1
pub fn i_09b284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B288;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B288: move.l a1, (a6)
pub fn i_09b288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B28A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B28A: bra.w $9b0c6
pub fn i_09b28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B28E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B0C6;
    10
}
// ===== Item_Food_Thrown_09b28e  ($09B28E, 200 B, props/items_score_crates_09a0bc.s)
/// $09B28E: move.w #$1ac, d1
pub fn i_09b28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B292;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B292: jsr $236e.l
pub fn i_09b292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B298);
    cpu.pc = 0x236E;
    16
}
/// $09B298: jsr $5e9b6.l
pub fn i_09b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B29E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B29E);
    cpu.pc = 0x5E9B6;
    16
}
/// $09B29E: andi.b #$1, d0
pub fn i_09b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2A2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09B2A2: move.b d0, $3a(a6)
pub fn i_09b2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2A6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B2A6: jsr $5e9b6.l
pub fn i_09b2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B2AC);
    cpu.pc = 0x5E9B6;
    16
}
/// $09B2AC: andi.w #$3, d0
pub fn i_09b2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B2B0: move.w d0, d1
pub fn i_09b2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B2B2: add.w d0, d0
pub fn i_09b2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B2B4: add.w d1, d0
pub fn i_09b2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B6;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B2B6: add.w d0, d0
pub fn i_09b2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2B8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B2B8: move.w d0, $5c(a6)
pub fn i_09b2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2BC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2BC: movea.l #$2f7a40, a0
pub fn i_09b2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2C2;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F7A40;
    cpu.a[0] = v1;
    4
}
/// $09B2C2: move.w (a0, d0.w), d1
pub fn i_09b2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2C6;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B2C6: move.w $2(a0, d0.w), d2
pub fn i_09b2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2CA;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B2CA: move.w $4(a0, d0.w), d3
pub fn i_09b2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2CE;
    cpu.ir = 0x3630;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B2CE: btst.b #$0, $3a(a6)
pub fn i_09b2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2D4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09B2D4: bne.w $9b2da
pub fn i_09b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B2DA; return 10; }
    8
}
/// $09B2D8: neg.w d1
pub fn i_09b2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2DA;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09B2DA: move.w d1, $28(a6)
pub fn i_09b2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2DE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2DE: move.w d2, $2e(a6)
pub fn i_09b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2E2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2E2: move.w d3, $2a(a6)
pub fn i_09b2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2E6;
    cpu.ir = 0x3D43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2E6: move.b #$1, $9c(a6)
pub fn i_09b2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B2EC: move.b #$a, $99(a6)
pub fn i_09b2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B2F2: jsr $9bb3c(pc)
pub fn i_09b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B2F6);
    cpu.pc = 0x9BB3C;
    16
}
/// $09B2F6: move.w d0, $70(a6)
pub fn i_09b2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B2FA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B2FA: move.b #$8, $45(a6)
pub fn i_09b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B300;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B300: move.w #$8000, d0
pub fn i_09b300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B304;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B304: jsr $28134.l
pub fn i_09b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B30A);
    cpu.pc = 0x28134;
    16
}
/// $09B30A: andi.w #$ffe3, $38(a6)
pub fn i_09b30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B310;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B310: ori.w #$0, $38(a6)
pub fn i_09b310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B316;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B316: lea.l $2f7002.l, a0
pub fn i_09b316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B31C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7002;
    20
}
/// $09B31C: jsr $28cd4.l
pub fn i_09b31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B322);
    cpu.pc = 0x28CD4;
    16
}
/// $09B322: lea.l $9b328(pc), a1
pub fn i_09b322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B326;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B328;
    16
}
/// $09B326: move.l a1, (a6)
pub fn i_09b326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B328;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B328: jsr $27d50.l
pub fn i_09b328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B32E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B32E);
    cpu.pc = 0x27D50;
    16
}
/// $09B32E: bcc.w $9b338
pub fn i_09b32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B332;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B338; return 10; }
    8
}
/// $09B332: lea.l $9b356(pc), a1
pub fn i_09b332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B336;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B356;
    16
}
/// $09B336: move.l a1, (a6)
pub fn i_09b336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B338;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B338: jsr $28d70.l
pub fn i_09b338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B33E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B33E);
    cpu.pc = 0x28D70;
    16
}
/// $09B33E: subq.w #$1, $70(a6)
pub fn i_09b33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B342;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B342: cmpi.w #$0, $70(a6)
pub fn i_09b342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B348;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B348: bgt.w $9b352
pub fn i_09b348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B34C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B352; return 10; }
    8
}
/// $09B34C: lea.l $9b0d8(pc), a1
pub fn i_09b34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B350;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0D8;
    16
}
/// $09B350: move.l a1, (a6)
pub fn i_09b350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B352;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B352: bra.w $9a75a
pub fn i_09b352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B356;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Food_Bounce_09b356  ($09B356, 112 B, props/items_score_crates_09a0bc.s)
/// $09B356: movea.l #$2f7a58, a0
pub fn i_09b356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B35C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F7A58;
    cpu.a[0] = v1;
    4
}
/// $09B35C: move.w $5c(a6), d0
pub fn i_09b35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B360;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B360: move.w (a0, d0.w), d1
pub fn i_09b360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B364;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B364: move.w $2(a0, d0.w), d2
pub fn i_09b364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B368;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B368: move.w $4(a0, d0.w), d3
pub fn i_09b368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B36C;
    cpu.ir = 0x3630;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $09B36C: btst.b #$0, $3a(a6)
pub fn i_09b36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B372;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09B372: bne.w $9b378
pub fn i_09b372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B376;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B378; return 10; }
    8
}
/// $09B376: neg.w d1
pub fn i_09b376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B378;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09B378: move.w d1, $28(a6)
pub fn i_09b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B37C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B37C: move.w d2, $2e(a6)
pub fn i_09b37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B380;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B380: move.w d3, $2a(a6)
pub fn i_09b380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B384;
    cpu.ir = 0x3D43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B384: lea.l $9b38a(pc), a1
pub fn i_09b384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B388;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B38A;
    16
}
/// $09B388: move.l a1, (a6)
pub fn i_09b388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B38A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B38A: jsr $27d50.l
pub fn i_09b38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B390;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B390);
    cpu.pc = 0x27D50;
    16
}
/// $09B390: bcc.w $9b39a
pub fn i_09b390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B394;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B39A; return 10; }
    8
}
/// $09B394: lea.l $9b0a8(pc), a1
pub fn i_09b394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B398;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0A8;
    16
}
/// $09B398: move.l a1, (a6)
pub fn i_09b398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B39A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B39A: jsr $28d70.l
pub fn i_09b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3A0);
    cpu.pc = 0x28D70;
    16
}
/// $09B3A0: subq.w #$1, $70(a6)
pub fn i_09b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3A4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B3A4: cmpi.w #$0, $70(a6)
pub fn i_09b3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3AA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B3AA: bgt.w $9b3b4
pub fn i_09b3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3AE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B3B4; return 10; }
    8
}
/// $09B3AE: lea.l $9b0d8(pc), a1
pub fn i_09b3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B0D8;
    16
}
/// $09B3B2: move.l a1, (a6)
pub fn i_09b3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B3B4: jsr $9b920(pc)
pub fn i_09b3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B3B8);
    cpu.pc = 0x9B920;
    16
}
/// $09B3B8: bcc.w $9b3c2
pub fn i_09b3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B3C2; return 10; }
    8
}
/// $09B3BC: lea.l $9b474(pc), a1
pub fn i_09b3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B3C0: move.l a1, (a6)
pub fn i_09b3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B3C2: bra.w $9a75a
pub fn i_09b3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A75A;
    10
}
// ===== Item_Tmpl293_Pow_09b3c6  ($09B3C6, 120 B, props/items_score_crates_09a0bc.s)
/// $09B3C6: move.w #$1ab, d1
pub fn i_09b3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3CA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1AB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B3CA: jsr $236e.l
pub fn i_09b3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3D0);
    cpu.pc = 0x236E;
    16
}
/// $09B3D0: move.l #$9a672, $60(a6)
pub fn i_09b3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3D8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x9A672;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B3D8: move.w #$8000, d0
pub fn i_09b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3DC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B3DC: jsr $28134.l
pub fn i_09b3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3E2);
    cpu.pc = 0x28134;
    16
}
/// $09B3E2: andi.w #$ffe3, $38(a6)
pub fn i_09b3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3E8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B3E8: ori.w #$0, $38(a6)
pub fn i_09b3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3EE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B3EE: lea.l $2f772a.l, a0
pub fn i_09b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3F4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F772A;
    20
}
/// $09B3F4: jsr $28cd4.l
pub fn i_09b3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B3FA);
    cpu.pc = 0x28CD4;
    16
}
/// $09B3FA: lea.l $9b400(pc), a1
pub fn i_09b3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B3FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B400;
    16
}
/// $09B3FE: move.l a1, (a6)
pub fn i_09b3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B400;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B400: jsr $9b8c0(pc)
pub fn i_09b400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B404;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B404);
    cpu.pc = 0x9B8C0;
    16
}
/// $09B404: jsr $9b8ee(pc)
pub fn i_09b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B408;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B408);
    cpu.pc = 0x9B8EE;
    16
}
/// $09B408: jsr $28998.l
pub fn i_09b408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B40E);
    cpu.pc = 0x28998;
    16
}
/// $09B40E: move.w #$8000, $38(a6)
pub fn i_09b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B414;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B414: jsr $28d70.l
pub fn i_09b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B41A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B41A);
    cpu.pc = 0x28D70;
    16
}
/// $09B41A: jsr $2870a.l
pub fn i_09b41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B420);
    cpu.pc = 0x2870A;
    16
}
/// $09B420: bcc.w $9b42a
pub fn i_09b420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B42A; return 10; }
    8
}
/// $09B424: bclr.b #$3, $13(a6)
pub fn i_09b424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B42A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09B42A: jsr $28758.l
pub fn i_09b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B430);
    cpu.pc = 0x28758;
    16
}
/// $09B430: bcc.w $9b43a
pub fn i_09b430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B434;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B43A; return 10; }
    8
}
/// $09B434: lea.l $9b43e(pc), a1
pub fn i_09b434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B438;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B43E;
    16
}
/// $09B438: move.l a1, (a6)
pub fn i_09b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B43A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B43A: bra.w $9a76a
pub fn i_09b43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B43E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Pow_Die_09b43e  ($09B43E, 54 B, props/items_score_crates_09a0bc.s)
/// $09B43E: move.w #$102e, d0
pub fn i_09b43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B442;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B442: jsr $2352.l
pub fn i_09b442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B448);
    cpu.pc = 0x2352;
    16
}
/// $09B448: lea.l $2f773c.l, a0
pub fn i_09b448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B44E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F773C;
    20
}
/// $09B44E: jsr $28cd4.l
pub fn i_09b44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B454);
    cpu.pc = 0x28CD4;
    16
}
/// $09B454: lea.l $9b45a(pc), a1
pub fn i_09b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B458;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B45A;
    16
}
/// $09B458: move.l a1, (a6)
pub fn i_09b458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B45A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B45A: jsr $2783a.l
pub fn i_09b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B460);
    cpu.pc = 0x2783A;
    16
}
/// $09B460: jsr $28d70.l
pub fn i_09b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B466;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B466);
    cpu.pc = 0x28D70;
    16
}
/// $09B466: bcc.w $9b470
pub fn i_09b466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B46A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B470; return 10; }
    8
}
/// $09B46A: lea.l $9b474(pc), a1
pub fn i_09b46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B46E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B474;
    16
}
/// $09B46E: move.l a1, (a6)
pub fn i_09b46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B470;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B470: bra.w $9a76a
pub fn i_09b470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B474;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Free_09b474  ($09B474, 6 B, props/items_score_crates_09a0bc.s)
/// $09B474: jmp $518.l
pub fn i_09b474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B47A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Item_Rts_09b47a  ($09B47A, 2 B, props/items_score_crates_09a0bc.s)
/// $09B47A: rts
pub fn i_09b47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B47C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_09b47c  ($09B47C, 14 B, kernel/jsr5b6_jmp518.c)
/// $09B47C: jsr $5b6.l
pub fn i_09b47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B482);
    cpu.pc = 0x5B6;
    16
}
/// $09B482: jmp $518.l
pub fn i_09b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B488;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Score_Popup_Value_09b51e  ($09B51E, 324 B, props/items_score_crates_09a0bc.s)
/// $09B51E: move.b $68(a6), $6e(a6)
pub fn i_09b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B524;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x6E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B524: move.b $98(a6), d1
pub fn i_09b524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B528;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B528: lea.l $2f792c.l, a1
pub fn i_09b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B52E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F792C;
    20
}
/// $09B52E: tst.b $9c(a6)
pub fn i_09b52e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B532;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B532: beq.w $9b53c
pub fn i_09b532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B536;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B53C; return 10; }
    8
}
/// $09B536: lea.l $2f794c.l, a1
pub fn i_09b536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B53C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F794C;
    20
}
/// $09B53C: andi.w #$f, d1
pub fn i_09b53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B540;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B540: lsl.w #$2, d1
pub fn i_09b540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B542;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09B542: move.l (a1, d1.w), d0
pub fn i_09b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B546;
    cpu.ir = 0x2031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $09B546: move.l d0, $80(a6)
pub fn i_09b546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B54A;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B54A: jsr $51a44.l
pub fn i_09b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B550);
    cpu.pc = 0x51A44;
    16
}
/// $09B550: move.b $98(a6), d1
pub fn i_09b550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B554;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B554: lea.l $2f798c.l, a1
pub fn i_09b554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B55A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F798C;
    20
}
/// $09B55A: tst.b $9c(a6)
pub fn i_09b55a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B55E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B55E: beq.w $9b568
pub fn i_09b55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B562;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B568; return 10; }
    8
}
/// $09B562: lea.l $2f7994.l, a1
pub fn i_09b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B568;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2F7994;
    20
}
/// $09B568: andi.w #$f, d1
pub fn i_09b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B56C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B56C: move.b (a1, d1.w), d1
pub fn i_09b56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B570;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $09B570: move.b d1, $72(a6)
pub fn i_09b570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B574;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B574: move.b d1, $21(a6)
pub fn i_09b574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B578;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B578: andi.w #$f, d1
pub fn i_09b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B57C;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B57C: move.w #$8, d0
pub fn i_09b57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B580;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B580: muls.w d1, d0
pub fn i_09b580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B582;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $09B582: add.w d0, $22(a6)
pub fn i_09b582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B586;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B586: jsr $5e9b6.l
pub fn i_09b586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B58C);
    cpu.pc = 0x5E9B6;
    16
}
/// $09B58C: andi.w #$e, d0
pub fn i_09b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B590;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xE) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B590: addi.w #$9, d0
pub fn i_09b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B594;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x9, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B594: add.w d0, $24(a6)
pub fn i_09b594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B598;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B598: clr.b $3a(a6)
pub fn i_09b598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B59C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B59C: lea.l $9b66a(pc), a1
pub fn i_09b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B66A;
    16
}
/// $09B5A0: jsr $4ae.l
pub fn i_09b5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5A6);
    cpu.pc = 0x4AE;
    16
}
/// $09B5A6: jsr $5dd02.l
pub fn i_09b5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5AC);
    cpu.pc = 0x5DD02;
    16
}
/// $09B5AC: move.b $68(a6), $68(a0)
pub fn i_09b5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5B2;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B5B2: subq.w #$8, $22(a6)
pub fn i_09b5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5B6;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B5B6: move.l $80(a6), d0
pub fn i_09b5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BA;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B5BA: move.l d0, d1
pub fn i_09b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BC;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09B5BC: lsr.l #$4, d1
pub fn i_09b5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5BE;
    cpu.ir = 0xE889;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 1, false);
    cpu.d[1] = r;
    14
}
/// $09B5BE: move.l d1, $80(a6)
pub fn i_09b5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5C2;
    cpu.ir = 0x2D41;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09B5C2: andi.w #$f, d0
pub fn i_09b5c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5C6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B5C6: move.w d0, $74(a0)
pub fn i_09b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5CA;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5CA: move.b $72(a6), d0
pub fn i_09b5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5CE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B5CE: andi.w #$7, d0
pub fn i_09b5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B5D2: lsl.w #$1, d0
pub fn i_09b5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D4;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $09B5D4: move.w d0, $70(a0)
pub fn i_09b5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5D8;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5D8: subq.b #$1, $72(a6)
pub fn i_09b5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5DC;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B5DC: cmpi.b #$0, $72(a6)
pub fn i_09b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5E2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $09B5E2: bgt.b $9b59c
pub fn i_09b5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5E4;
    cpu.ir = 0x6EB8;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B59C; return 10; }
    8
}
/// $09B5E4: move.w #$1e, $70(a6)
pub fn i_09b5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5EA: move.w #$96, $84(a6)
pub fn i_09b5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x96;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B5F0: lea.l $9b5f6(pc), a1
pub fn i_09b5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B5F6;
    16
}
/// $09B5F4: move.l a1, (a6)
pub fn i_09b5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B5F6: jsr $2783a.l
pub fn i_09b5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B5FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B5FC);
    cpu.pc = 0x2783A;
    16
}
/// $09B5FC: cmpi.b #$ff, $21(a6)
pub fn i_09b5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B602;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B602: beq.w $9b60e
pub fn i_09b602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B606;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B60E; return 10; }
    8
}
/// $09B606: tst.b $21(a6)
pub fn i_09b606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B60A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B60A: bne.w $9b63a
pub fn i_09b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B60E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B63A; return 10; }
    8
}
/// $09B60E: subq.w #$1, $70(a6)
pub fn i_09b60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B612;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B612: cmpi.w #$0, $70(a6)
pub fn i_09b612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B618;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B618: bgt.w $9b63a
pub fn i_09b618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B61C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B63A; return 10; }
    8
}
/// $09B61C: tst.b $21(a6)
pub fn i_09b61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B620;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B620: bne.w $9b634
pub fn i_09b620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B624;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B634; return 10; }
    8
}
/// $09B624: move.b #$ff, $21(a6)
pub fn i_09b624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B62A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B62A: move.w #$a, $70(a6)
pub fn i_09b62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B630;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B630: bra.w $9b63a
pub fn i_09b630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B634;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B63A;
    10
}
/// $09B634: lea.l $9b47c(pc), a1
pub fn i_09b634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B638;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B638: move.l a1, (a6)
pub fn i_09b638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B63A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B63A: subq.w #$1, $84(a6)
pub fn i_09b63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B63E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B63E: cmpi.w #$0, $84(a6)
pub fn i_09b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B644;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B644: bgt.w $9b64e
pub fn i_09b644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B648;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B64E; return 10; }
    8
}
/// $09B648: lea.l $9b47c(pc), a1
pub fn i_09b648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B64C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B64C: move.l a1, (a6)
pub fn i_09b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B64E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B64E: movea.l #$ffffffff, a0
pub fn i_09b64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B654;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09B654: lea.l $9a6c6(pc), a0
pub fn i_09b654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B658;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9A6C6;
    16
}
/// $09B658: jsr $5dd56.l
pub fn i_09b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B65E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B65E);
    cpu.pc = 0x5DD56;
    16
}
/// $09B65E: bcc.w $9b668
pub fn i_09b65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B662;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B668; return 10; }
    8
}
// ===== SetTaskHandler_09b662  ($09B662, 8 B, kernel/task_handlers.c)
/// $09B662: lea.l $9b47c(pc), a1
pub fn i_09b662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B666;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B666: move.l a1, (a6)
pub fn i_09b666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B668;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B668: rts
pub fn i_09b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B66A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_09b66a  ($09B66A, 232 B, props/items_score_crates_09a0bc.s)
/// $09B66A: move.b $68(a6), d0
pub fn i_09b66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B66E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B66E: cmpi.b #$0, d0
pub fn i_09b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B672;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $09B672: bne.w $9b68e
pub fn i_09b672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B676;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B68E; return 10; }
    8
}
/// $09B676: move.w #$1b8, d1
pub fn i_09b676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B67A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B67A: jsr $236e.l
pub fn i_09b67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B680);
    cpu.pc = 0x236E;
    16
}
/// $09B680: move.w #$1b9, d1
pub fn i_09b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B684;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B684: jsr $236e.l
pub fn i_09b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B68A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B68A);
    cpu.pc = 0x236E;
    16
}
/// $09B68A: bra.w $9b6a2
pub fn i_09b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B68E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B6A2;
    10
}
/// $09B68E: move.w #$1cd, d1
pub fn i_09b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B692;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B692: jsr $236e.l
pub fn i_09b692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B698);
    cpu.pc = 0x236E;
    16
}
/// $09B698: move.w #$1ce, d1
pub fn i_09b698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B69C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1CE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B69C: jsr $236e.l
pub fn i_09b69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6A2);
    cpu.pc = 0x236E;
    16
}
/// $09B6A2: move.w #$c000, d0
pub fn i_09b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B6A6: jsr $28134.l
pub fn i_09b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6AC);
    cpu.pc = 0x28134;
    16
}
/// $09B6AC: andi.w #$ffe3, $38(a6)
pub fn i_09b6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6B2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B6B2: ori.w #$1c, $38(a6)
pub fn i_09b6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6B8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09B6B8: move.w #$0, d0
pub fn i_09b6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6BC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B6BC: jsr $5dca4.l
pub fn i_09b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6C2);
    cpu.pc = 0x5DCA4;
    16
}
/// $09B6C2: move.w d0, $28(a6)
pub fn i_09b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6C6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6C6: move.w #$886, $2a(a6)
pub fn i_09b6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x886;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6CC: move.w #$fedd, $2e(a6)
pub fn i_09b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEDD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6D2: move.w #$0, $2c(a6)
pub fn i_09b6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B6D8: move.w $74(a6), d0
pub fn i_09b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6DC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B6DC: andi.w #$1f, d0
pub fn i_09b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B6E0: movea.l #$2f77a4, a0
pub fn i_09b6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F77A4;
    cpu.a[0] = v1;
    4
}
/// $09B6E6: lsl.w #$2, d0
pub fn i_09b6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6E8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09B6E8: movea.l (a0, d0.w), a0
pub fn i_09b6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6EC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $09B6EC: cmpa.l #$ffffffff, a0
pub fn i_09b6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6F2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $09B6F2: beq.w $9b6fc
pub fn i_09b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B6FC; return 10; }
    8
}
/// $09B6F6: jsr $28cd4.l
pub fn i_09b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B6FC);
    cpu.pc = 0x28CD4;
    16
}
/// $09B6FC: lea.l $9b702(pc), a1
pub fn i_09b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B700;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B702;
    16
}
/// $09B700: move.l a1, (a6)
pub fn i_09b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B702;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B702: jsr $2783a.l
pub fn i_09b702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B708);
    cpu.pc = 0x2783A;
    16
}
/// $09B708: subq.w #$1, $70(a6)
pub fn i_09b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B70C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B70C: cmpi.w #$0, $70(a6)
pub fn i_09b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B712;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B712: bgt.w $9b748
pub fn i_09b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B716;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B748; return 10; }
    8
}
/// $09B716: jsr $27cee.l
pub fn i_09b716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B71C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B71C);
    cpu.pc = 0x27CEE;
    16
}
/// $09B71C: movea.l $c(a6), a0
pub fn i_09b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B720;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B720: move.w $24(a0), d0
pub fn i_09b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B724;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B724: cmp.w $24(a6), d0
pub fn i_09b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B728;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $09B728: ble.w $9b73e
pub fn i_09b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B72C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B73E; return 10; }
    8
}
/// $09B72C: addi.w #$10, d0
pub fn i_09b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B730;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B730: move.w d0, $24(a6)
pub fn i_09b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B734;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B734: subq.b #$1, $21(a0)
pub fn i_09b734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B738;
    cpu.ir = 0x5328;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09B738: lea.l $9b75a(pc), a1
pub fn i_09b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B73C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B75A;
    16
}
/// $09B73C: move.l a1, (a6)
pub fn i_09b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B73E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B73E: jsr $9bb22(pc)
pub fn i_09b73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B742;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B742);
    cpu.pc = 0x9BB22;
    16
}
/// $09B742: jsr $28d70.l
pub fn i_09b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B748);
    cpu.pc = 0x28D70;
    16
}
/// $09B748: jsr $5e45a.l
pub fn i_09b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B74E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B74E);
    cpu.pc = 0x5E45A;
    16
}
/// $09B74E: bcc.w $9b758
pub fn i_09b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B752;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B758; return 10; }
    8
}
// ===== SetTaskHandler_09b752  ($09B752, 8 B, kernel/task_handlers.c)
/// $09B752: lea.l $9b47c(pc), a1
pub fn i_09b752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B756;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B756: move.l a1, (a6)
pub fn i_09b756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B758;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B758: rts
pub fn i_09b758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B75A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Lift_09b75a  ($09B75A, 90 B, props/items_score_crates_09a0bc.s)
/// $09B75A: jsr $267e2.l
pub fn i_09b75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B760;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B760);
    cpu.pc = 0x267E2;
    16
}
/// $09B760: move.w #$400, $2a(a6)
pub fn i_09b760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B766;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B766: move.w #$2, $70(a6)
pub fn i_09b766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B76C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B76C: lea.l $9b772(pc), a1
pub fn i_09b76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B770;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B772;
    16
}
/// $09B770: move.l a1, (a6)
pub fn i_09b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B772;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B772: jsr $27cee.l
pub fn i_09b772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B778);
    cpu.pc = 0x27CEE;
    16
}
/// $09B778: jsr $9bb22(pc)
pub fn i_09b778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B77C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B77C);
    cpu.pc = 0x9BB22;
    16
}
/// $09B77C: jsr $28d70.l
pub fn i_09b77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B782);
    cpu.pc = 0x28D70;
    16
}
/// $09B782: subq.w #$1, $70(a6)
pub fn i_09b782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B786;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B786: cmpi.w #$0, $70(a6)
pub fn i_09b786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B78C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09B78C: bgt.w $9b796
pub fn i_09b78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B790;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B796; return 10; }
    8
}
/// $09B790: lea.l $9b7bc(pc), a1
pub fn i_09b790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B794;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7BC;
    16
}
/// $09B794: move.l a1, (a6)
pub fn i_09b794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B796;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B796: movea.l $c(a6), a0
pub fn i_09b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B79A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B79A: cmpi.b #$ff, $21(a0)
pub fn i_09b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B7A0: bne.w $9b7aa
pub fn i_09b7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B7AA; return 10; }
    8
}
/// $09B7A4: lea.l $9b7f8(pc), a1
pub fn i_09b7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7F8;
    16
}
/// $09B7A8: move.l a1, (a6)
pub fn i_09b7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7AA: jsr $5e45a.l
pub fn i_09b7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7B0);
    cpu.pc = 0x5E45A;
    16
}
/// $09B7B0: bcc.w $9b7ba
pub fn i_09b7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B7BA; return 10; }
    8
}
// ===== SetTaskHandler_09b7b4  ($09B7B4, 8 B, kernel/task_handlers.c)
/// $09B7B4: lea.l $9b47c(pc), a1
pub fn i_09b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B7B8: move.l a1, (a6)
pub fn i_09b7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7BA: rts
pub fn i_09b7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Hold_09b7bc  ($09B7BC, 52 B, props/items_score_crates_09a0bc.s)
/// $09B7BC: lea.l $9b7c2(pc), a1
pub fn i_09b7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7C2;
    16
}
/// $09B7C0: move.l a1, (a6)
pub fn i_09b7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7C2: jsr $2783a.l
pub fn i_09b7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7C8);
    cpu.pc = 0x2783A;
    16
}
/// $09B7C8: jsr $9bb22(pc)
pub fn i_09b7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B7CC);
    cpu.pc = 0x9BB22;
    16
}
/// $09B7CC: jsr $28d70.l
pub fn i_09b7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7D2);
    cpu.pc = 0x28D70;
    16
}
/// $09B7D2: movea.l $c(a6), a0
pub fn i_09b7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7D6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09B7D6: cmpi.b #$ff, $21(a0)
pub fn i_09b7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7DC;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B7DC: bne.w $9b7e6
pub fn i_09b7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B7E6; return 10; }
    8
}
/// $09B7E0: lea.l $9b7f8(pc), a1
pub fn i_09b7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7F8;
    16
}
/// $09B7E4: move.l a1, (a6)
pub fn i_09b7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7E6: jsr $5e45a.l
pub fn i_09b7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B7EC);
    cpu.pc = 0x5E45A;
    16
}
/// $09B7EC: bcc.w $9b7f6
pub fn i_09b7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B7F6; return 10; }
    8
}
// ===== SetTaskHandler_09b7f0  ($09B7F0, 8 B, kernel/task_handlers.c)
/// $09B7F0: lea.l $9b47c(pc), a1
pub fn i_09b7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B7F4: move.l a1, (a6)
pub fn i_09b7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7F6: rts
pub fn i_09b7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Digit_Blink_09b7f8  ($09B7F8, 44 B, props/items_score_crates_09a0bc.s)
/// $09B7F8: lea.l $9b7fe(pc), a1
pub fn i_09b7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B7FE;
    16
}
/// $09B7FC: move.l a1, (a6)
pub fn i_09b7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B7FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B7FE: jsr $2783a.l
pub fn i_09b7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B804);
    cpu.pc = 0x2783A;
    16
}
/// $09B804: btst.b #$0, $106f28.l
pub fn i_09b804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B80C;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $09B80C: beq.w $9b81a
pub fn i_09b80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B810;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B81A; return 10; }
    8
}
/// $09B810: jsr $9bb22(pc)
pub fn i_09b810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B814;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B814);
    cpu.pc = 0x9BB22;
    16
}
/// $09B814: jsr $28d70.l
pub fn i_09b814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B81A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B81A);
    cpu.pc = 0x28D70;
    16
}
/// $09B81A: jsr $5e45a.l
pub fn i_09b81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B820;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B820);
    cpu.pc = 0x5E45A;
    16
}
/// $09B820: bcc.w $9b82a
pub fn i_09b820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B824;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B82A; return 10; }
    8
}
// ===== SetTaskHandler_09b824  ($09B824, 8 B, kernel/task_handlers.c)
/// $09B824: lea.l $9b47c(pc), a1
pub fn i_09b824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B828;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B47C;
    16
}
/// $09B828: move.l a1, (a6)
pub fn i_09b828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B82A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B82A: rts
pub fn i_09b82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B82C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Tmpl316_ComboTimer_09b82c  ($09B82C, 70 B, props/items_score_crates_09a0bc.s)
/// $09B82C: clr.b $10e488.l
pub fn i_09b82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B832;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E488) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E488, r2 as u8);
    16
}
/// $09B832: clr.b $10e489.l
pub fn i_09b832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B838;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E489) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E489, r2 as u8);
    16
}
/// $09B838: lea.l $9b83e(pc), a1
pub fn i_09b838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B83C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B83E;
    16
}
/// $09B83C: move.l a1, (a6)
pub fn i_09b83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B83E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B83E: subq.b #$1, $10e488.l
pub fn i_09b83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B844;
    cpu.ir = 0x5339;
    let v1 = bus.read8(0x10E488) as u32;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    bus.write8(0x10E488, r2 as u8);
    16
}
/// $09B844: cmpi.b #$0, $10e488.l
pub fn i_09b844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B84C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E488) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $09B84C: bgt.w $9b85c
pub fn i_09b84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B850;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x9B85C; return 10; }
    8
}
/// $09B850: clr.b $10e488.l
pub fn i_09b850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B856;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E488) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E488, r2 as u8);
    16
}
/// $09B856: clr.b $10e489.l
pub fn i_09b856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B85C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E489) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E489, r2 as u8);
    16
}
/// $09B85C: cmpi.b #$e, $10e489.l
pub fn i_09b85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B864;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E489) as u32;
    sub_with_flags(cpu, v2, 0xE, Size::Byte, false);
    20
}
/// $09B864: ble.w $9b870
pub fn i_09b864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B868;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B870; return 10; }
    8
}
/// $09B868: move.b #$e, $10e489.l
pub fn i_09b868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B870;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E489, v1 as u8);
    16
}
/// $09B870: rts
pub fn i_09b870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B872;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SpawnAtOffset_09b872  ($09B872, 76 B, props/items_score_crates_09a0bc.s)
/// $09B872: jsr $4cac4.l
pub fn i_09b872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B878;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B878);
    cpu.pc = 0x4CAC4;
    16
}
/// $09B878: tst.w $98(a6)
pub fn i_09b878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B87C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $09B87C: bne.w $9b886
pub fn i_09b87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B880;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B886; return 10; }
    8
}
/// $09B880: add.w d3, d0
pub fn i_09b880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B882;
    cpu.ir = 0xD043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B882: bra.w $9b888
pub fn i_09b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B886;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B888;
    10
}
/// $09B886: sub.w d3, d0
pub fn i_09b886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B888;
    cpu.ir = 0x9043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09B888: addq.w #$8, d1
pub fn i_09b888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B88A;
    cpu.ir = 0x5041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09B88A: move.w d0, $22(a6)
pub fn i_09b88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B88E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B88E: move.w d1, $24(a6)
pub fn i_09b88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B892;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09B892: move.b d4, $98(a6)
pub fn i_09b892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B896;
    cpu.ir = 0x1D44;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B896: move.b d5, $99(a6)
pub fn i_09b896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B89A;
    cpu.ir = 0x1D45;
    let v1 = cpu.d[5] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B89A: move.b d6, $9a(a6)
pub fn i_09b89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B89E;
    cpu.ir = 0x1D46;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B89E: move.b $98(a6), d0
pub fn i_09b89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8A2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B8A2: move.w #$99, d1
pub fn i_09b8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B8A6: jsr $9a7aa(pc)
pub fn i_09b8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B8AA);
    cpu.pc = 0x9A7AA;
    16
}
/// $09B8AA: bcs.w $9b8b8
pub fn i_09b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8AE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9B8B8; return 10; }
    8
}
/// $09B8AE: move.l (a0), $5c(a0)
pub fn i_09b8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8B2;
    cpu.ir = 0x2150;
    let v2 = bus.read32(cpu.a[0]);
    let ad3 = cpu.a[0].wrapping_add(0x5C);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $09B8B2: move.l #$9a8be, (a0)
pub fn i_09b8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8B8;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x9A8BE;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B8B8: jmp $518.l
pub fn i_09b8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8BE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Item_Rts_09b8be  ($09B8BE, 2 B, props/items_score_crates_09a0bc.s)
/// $09B8BE: rts
pub fn i_09b8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_PhysAndProbe_09b8c0  ($09B8C0, 40 B, props/items_score_crates_09a0bc.s)
/// $09B8C0: jsr $2783a.l
pub fn i_09b8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B8C6);
    cpu.pc = 0x2783A;
    16
}
/// $09B8C6: tst.b $9b(a6)
pub fn i_09b8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8CA;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09B8CA: beq.w $9b8d8
pub fn i_09b8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B8D8; return 10; }
    8
}
/// $09B8CE: cmpi.b #$ff, $9b(a6)
pub fn i_09b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $09B8D4: bne.w $9b8e8
pub fn i_09b8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B8E8; return 10; }
    8
}
/// $09B8D8: jsr $27eba.l
pub fn i_09b8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B8DE);
    cpu.pc = 0x27EBA;
    16
}
/// $09B8DE: bcc.w $9b8e8
pub fn i_09b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B8E8; return 10; }
    8
}
/// $09B8E2: jsr $27c8c.l
pub fn i_09b8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B8E8);
    cpu.pc = 0x27C8C;
    16
}
// ===== ClearXN_09b8e8  ($09B8E8, 6 B, thunks/ccr_helpers.c)
/// $09B8E8: andi.b #$ee, ccr
pub fn i_09b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8EC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09B8EC: rts
pub fn i_09b8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_TrailRecord_09b8ee  ($09B8EE, 20 B, props/items_score_crates_09a0bc.s)
/// $09B8EE: move.w $22(a6), d0
pub fn i_09b8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8F2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B8F2: move.w $24(a6), d1
pub fn i_09b8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8F6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09B8F6: subi.w #$10, d0
pub fn i_09b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8FA;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B8FA: addi.w #$10, d1
pub fn i_09b8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B8FE;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09B8FE: move.w #$20, d2
pub fn i_09b8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B902;
    cpu.ir = 0x343C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_09b902  ($09B902, 8 B, thunks/jsr_abs_thunks.c)
/// $09B902: jsr $99812.l
pub fn i_09b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B908;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B908);
    cpu.pc = 0x99812;
    16
}
/// $09B908: rts
pub fn i_09b908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B90A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_RespawnBelow_09b90a  ($09B90A, 22 B, props/items_score_crates_09a0bc.s)
/// $09B90A: move.b $98(a6), d0
pub fn i_09b90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B90E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B90E: move.w #$99, d1
pub fn i_09b90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B912;
    cpu.ir = 0x323C;
    let v1: u32 = 0x99;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B912: jsr $9a7aa(pc)
pub fn i_09b912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B916;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B916);
    cpu.pc = 0x9A7AA;
    16
}
/// $09B916: bcs.w $9b91e
pub fn i_09b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B91A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9B91E; return 10; }
    8
}
/// $09B91A: addq.w #$4, $24(a0)
pub fn i_09b91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B91E;
    cpu.ir = 0x5868;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09B91E: rts
pub fn i_09b91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B920;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Food_Pickup_09b920  ($09B920, 142 B, props/items_score_crates_09a0bc.s)
/// $09B920: jsr $9a84e(pc)
pub fn i_09b920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B924;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9B924);
    cpu.pc = 0x9A84E;
    16
}
/// $09B924: bcc.w $9b9b4
pub fn i_09b924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B928;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9B9B4; return 10; }
    8
}
/// $09B928: move.b d0, $68(a6)
pub fn i_09b928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B92C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B92C: move.w #$1121, d0
pub fn i_09b92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B930;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1121;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B930: jsr $2352.l
pub fn i_09b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B936);
    cpu.pc = 0x2352;
    16
}
/// $09B936: lea.l $9b51e(pc), a1
pub fn i_09b936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B93A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B51E;
    16
}
/// $09B93A: jsr $4ae.l
pub fn i_09b93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B940);
    cpu.pc = 0x4AE;
    16
}
/// $09B940: jsr $5dd02.l
pub fn i_09b940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B946);
    cpu.pc = 0x5DD02;
    16
}
/// $09B946: move.b $68(a6), $68(a0)
pub fn i_09b946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B94C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09B94C: move.b $98(a6), d1
pub fn i_09b94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B950;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B950: move.b $9c(a6), d0
pub fn i_09b950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B954;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09B954: move.b d0, $9c(a0)
pub fn i_09b954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B958;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B958: tst.b d0
pub fn i_09b958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B95A;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $09B95A: beq.w $9b99e
pub fn i_09b95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B95E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B99E; return 10; }
    8
}
/// $09B95E: cmpi.b #$2, d0
pub fn i_09b95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B962;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $09B962: bne.w $9b97e
pub fn i_09b962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B966;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9B97E; return 10; }
    8
}
/// $09B966: move.b #$f, d1
pub fn i_09b966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B96A;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09B96A: move.b #$e, $10e489.l
pub fn i_09b96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B972;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E489, v1 as u8);
    16
}
/// $09B972: move.b #$2d, $10e488.l
pub fn i_09b972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B97A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E488, v1 as u8);
    16
}
/// $09B97A: bra.w $9b9aa
pub fn i_09b97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B97E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9B9AA;
    10
}
/// $09B97E: move.b $10e489.l, d2
pub fn i_09b97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B984;
    cpu.ir = 0x1439;
    let v2 = bus.read8(0x10E489) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    16
}
/// $09B984: cmp.b d1, d2
pub fn i_09b984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B986;
    cpu.ir = 0xB401;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $09B986: ble.w $9b98c
pub fn i_09b986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B98A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B98C; return 10; }
    8
}
/// $09B98A: move.b d2, d1
pub fn i_09b98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B98C;
    cpu.ir = 0x1202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09B98C: move.w d1, d2
pub fn i_09b98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B98E;
    cpu.ir = 0x3401;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09B98E: addq.b #$1, d2
pub fn i_09b98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B990;
    cpu.ir = 0x5202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09B990: move.b d2, $10e489.l
pub fn i_09b990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B996;
    cpu.ir = 0x13C2;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E489, v1 as u8);
    16
}
/// $09B996: move.b #$2d, $10e488.l
pub fn i_09b996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B99E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E488, v1 as u8);
    16
}
/// $09B99E: cmpi.b #$e, d1
pub fn i_09b99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9A2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xE, Size::Byte, false);
    8
}
/// $09B9A2: ble.w $9b9aa
pub fn i_09b9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9A6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x9B9AA; return 10; }
    8
}
/// $09B9A6: move.b #$e, d1
pub fn i_09b9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9AA;
    cpu.ir = 0x123C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $09B9AA: move.b d1, $98(a0)
pub fn i_09b9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9AE;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_09b9ae  ($09B9AE, 6 B, thunks/ccr_helpers.c)
/// $09B9AE: ori.b #$11, ccr
pub fn i_09b9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9B2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09B9B2: rts
pub fn i_09b9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_09b9b4  ($09B9B4, 6 B, thunks/ccr_helpers.c)
/// $09B9B4: andi.b #$ee, ccr
pub fn i_09b9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9B8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09B9B8: rts
pub fn i_09b9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Popup_ForPlayer_09b9ba  ($09B9BA, 60 B, props/items_score_crates_09a0bc.s)
/// $09B9BA: move.l a1, d1
pub fn i_09b9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9BC;
    cpu.ir = 0x2209;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09B9BC: clr.b d0
pub fn i_09b9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9BE;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09B9BE: cmpi.l #$100440, d1
pub fn i_09b9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9C4;
    cpu.ir = 0xC81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100440, Size::Long, false);
    8
}
/// $09B9C4: beq.w $9b9d6
pub fn i_09b9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B9D6; return 10; }
    8
}
/// $09B9C8: addq.b #$1, d0
pub fn i_09b9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9CA;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09B9CA: cmpi.l #$1004e0, d1
pub fn i_09b9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9D0;
    cpu.ir = 0xC81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x1004E0, Size::Long, false);
    8
}
/// $09B9D0: beq.w $9b9d6
pub fn i_09b9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9B9D6; return 10; }
    8
}
/// $09B9D4: rts
pub fn i_09b9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09B9D6: move.l d0, -(a7)
pub fn i_09b9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9D8;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09B9D8: lea.l $9b48a(pc), a1
pub fn i_09b9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B48A;
    16
}
/// $09B9DC: jsr $4ae.l
pub fn i_09b9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B9E2);
    cpu.pc = 0x4AE;
    16
}
/// $09B9E2: jsr $5dd02.l
pub fn i_09b9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9B9E8);
    cpu.pc = 0x5DD02;
    16
}
/// $09B9E8: move.l (a7)+, d0
pub fn i_09b9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9EA;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09B9EA: move.b d0, $68(a0)
pub fn i_09b9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9EE;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B9EE: move.b #$2, $98(a0)
pub fn i_09b9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9F4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09B9F4: rts
pub fn i_09b9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_AllocByPlayerSlot_09B9F6  ($09B9F6, 62 B, entity/alloc_by_player_slot_09b9f6.s)
/// $09B9F6: move.l a1, d1
pub fn i_09b9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9F8;
    cpu.ir = 0x2209;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $09B9F8: clr.b d0
pub fn i_09b9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9B9FA;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09B9FA: cmpi.l #$100440, d1
pub fn i_09b9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA00;
    cpu.ir = 0xC81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x100440, Size::Long, false);
    8
}
/// $09BA00: beq.w $9ba10
pub fn i_09ba00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9BA10; return 10; }
    8
}
/// $09BA04: addq.b #$1, d0
pub fn i_09ba04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA06;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09BA06: cmpi.l #$1004e0, d1
pub fn i_09ba06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA0C;
    cpu.ir = 0xC81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x1004E0, Size::Long, false);
    8
}
/// $09BA0C: bne.w $9ba32
pub fn i_09ba0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9BA32; return 10; }
    8
}
/// $09BA10: move.l d0, -(a7)
pub fn i_09ba10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA12;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $09BA12: lea.l $9b51e(pc), a1
pub fn i_09ba12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B51E;
    16
}
/// $09BA16: jsr $4ae.l
pub fn i_09ba16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BA1C);
    cpu.pc = 0x4AE;
    16
}
/// $09BA1C: jsr $5dd02.l
pub fn i_09ba1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BA22);
    cpu.pc = 0x5DD02;
    16
}
/// $09BA22: move.l (a7)+, d0
pub fn i_09ba22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA24;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09BA24: move.b d0, $68(a0)
pub fn i_09ba24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA28;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BA28: move.b #$3, $98(a0)
pub fn i_09ba28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA2E;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BA2E: clr.b $9c(a0)
pub fn i_09ba2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA32;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09BA32: rts
pub fn i_09ba32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Popup_Kind3_09ba34  ($09BA34, 34 B, props/items_score_crates_09a0bc.s)
/// $09BA34: lea.l $9b51e(pc), a1
pub fn i_09ba34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B51E;
    16
}
/// $09BA38: jsr $4ae.l
pub fn i_09ba38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BA3E);
    cpu.pc = 0x4AE;
    16
}
/// $09BA3E: jsr $5dd02.l
pub fn i_09ba3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BA44);
    cpu.pc = 0x5DD02;
    16
}
/// $09BA44: move.b $68(a6), $68(a0)
pub fn i_09ba44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA4A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09BA4A: move.b #$3, $98(a0)
pub fn i_09ba4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA50;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BA50: clr.b $9c(a0)
pub fn i_09ba50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA54;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09BA54: rts
pub fn i_09ba54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Popup_Kind4_09ba56  ($09BA56, 34 B, props/items_score_crates_09a0bc.s)
/// $09BA56: lea.l $9b51e(pc), a1
pub fn i_09ba56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B51E;
    16
}
/// $09BA5A: jsr $4ae.l
pub fn i_09ba5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BA60);
    cpu.pc = 0x4AE;
    16
}
/// $09BA60: jsr $5dd02.l
pub fn i_09ba60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BA66);
    cpu.pc = 0x5DD02;
    16
}
/// $09BA66: move.b $6e(a6), $68(a0)
pub fn i_09ba66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA6C;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09BA6C: move.b #$4, $98(a0)
pub fn i_09ba6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA72;
    cpu.ir = 0x117C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BA72: clr.b $9c(a0)
pub fn i_09ba72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA76;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x9C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $09BA76: rts
pub fn i_09ba76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Food_Spawn_09ba78  ($09BA78, 30 B, props/items_score_crates_09a0bc.s)
/// $09BA78: lea.l $9b28e(pc), a1
pub fn i_09ba78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B28E;
    16
}
/// $09BA7C: jsr $4ae.l
pub fn i_09ba7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BA82);
    cpu.pc = 0x4AE;
    16
}
/// $09BA82: jsr $5dd02.l
pub fn i_09ba82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BA88);
    cpu.pc = 0x5DD02;
    16
}
/// $09BA88: move.b $68(a6), $68(a0)
pub fn i_09ba88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA8E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x68);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09BA8E: move.b $6e(a6), $6e(a0)
pub fn i_09ba8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA94;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x6E);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09BA94: rts
pub fn i_09ba94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Popup_Value_Alloc_09ba96  ($09BA96, 60 B, props/items_score_crates_09a0bc.s)
/// $09BA96: movem.l d0-d2, -(a7)
pub fn i_09ba96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA9A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $09BA9A: lea.l $9b586(pc), a1
pub fn i_09ba9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BA9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B586;
    16
}
/// $09BA9E: jsr $4ae.l
pub fn i_09ba9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BAA4);
    cpu.pc = 0x4AE;
    16
}
/// $09BAA4: jsr $5dd02.l
pub fn i_09baa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BAAA);
    cpu.pc = 0x5DD02;
    16
}
/// $09BAAA: movem.l (a7)+, d0-d2
pub fn i_09baaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAAE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $09BAAE: move.l d0, $80(a6)
pub fn i_09baae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAB2;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09BAB2: move.b d2, $68(a6)
pub fn i_09bab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAB6;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BAB6: move.b d2, $6e(a6)
pub fn i_09bab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BABA;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BABA: move.b d1, $72(a0)
pub fn i_09baba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BABE;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BABE: move.b d1, $21(a0)
pub fn i_09babe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAC2;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BAC2: andi.w #$f, d1
pub fn i_09bac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAC6;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09BAC6: move.w #$8, d0
pub fn i_09bac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BACA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BACA: muls.w d1, d0
pub fn i_09baca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BACC;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $09BACC: add.w d0, $22(a0)
pub fn i_09bacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAD0;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09BAD0: rts
pub fn i_09bad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Score_Popup_Value_AllocAt_09bad2  ($09BAD2, 80 B, props/items_score_crates_09a0bc.s)
/// $09BAD2: movem.l d0-d2, -(a7)
pub fn i_09bad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAD6;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $09BAD6: lea.l $9b586(pc), a1
pub fn i_09bad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BADA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9B586;
    16
}
/// $09BADA: jsr $4ae.l
pub fn i_09bada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BAE0);
    cpu.pc = 0x4AE;
    16
}
/// $09BAE0: move.w d3, $22(a0)
pub fn i_09bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAE4;
    cpu.ir = 0x3143;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BAE4: move.w d4, $24(a0)
pub fn i_09bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAE8;
    cpu.ir = 0x3144;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BAE8: move.w $38(a6), $38(a0)
pub fn i_09bae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAEE;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09BAEE: move.b $3a(a6), $3a(a0)
pub fn i_09baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAF4;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09BAF4: move.b $11(a6), $11(a0)
pub fn i_09baf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAFA;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09BAFA: movem.l (a7)+, d0-d2
pub fn i_09bafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BAFE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $09BAFE: move.l d0, $80(a6)
pub fn i_09bafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB02;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09BB02: move.b d2, $68(a6)
pub fn i_09bb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB06;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x68);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BB06: move.b d2, $6e(a6)
pub fn i_09bb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB0A;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x6E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BB0A: move.b d1, $72(a0)
pub fn i_09bb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB0E;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BB0E: move.b d1, $21(a0)
pub fn i_09bb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB12;
    cpu.ir = 0x1141;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $09BB12: andi.w #$f, d1
pub fn i_09bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB16;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09BB16: move.w #$8, d0
pub fn i_09bb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB1A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BB1A: muls.w d1, d0
pub fn i_09bb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB1C;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $09BB1C: move.w d0, $5c(a0)
pub fn i_09bb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB20;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BB20: rts
pub fn i_09bb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_SelectAnimParity_09bb22  ($09BB22, 20 B, props/items_score_crates_09a0bc.s)
/// $09BB22: move.w $16(a6), d0
pub fn i_09bb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB26;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09BB26: btst.b #$0, $106f28.l
pub fn i_09bb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB2E;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $09BB2E: beq.w $9bb36
pub fn i_09bb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9BB36; return 10; }
    8
}
/// $09BB32: move.w $18(a6), d0
pub fn i_09bb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB36;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
// ===== SetTaskW_09bb36  ($09BB36, 6 B, kernel/task_setters_w.c)
/// $09BB36: move.w d0, $14(a6)
pub fn i_09bb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB3A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BB3A: rts
pub fn i_09bb3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_LifetimeFromParam_09bb3c  ($09BB3C, 26 B, props/items_score_crates_09a0bc.s)
/// $09BB3C: move.b $99(a6), d0
pub fn i_09bb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB40;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09BB40: move.b $99(a6), d0
pub fn i_09bb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB44;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09BB44: andi.w #$ff, d0
pub fn i_09bb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB48;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09BB48: lsl.w #$4, d0
pub fn i_09bb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB4A;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $09BB4A: tst.w d0
pub fn i_09bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB4C;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $09BB4C: bne.w $9bb54
pub fn i_09bb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9BB54; return 10; }
    8
}
/// $09BB50: move.w #$7fff, d0
pub fn i_09bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB54;
    cpu.ir = 0x303C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BB54: rts
pub fn i_09bb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Item_Tmpl317_Static_09bb56  ($09BB56, 60 B, props/items_score_crates_09a0bc.s)
/// $09BB56: jsr $5e7c0.l
pub fn i_09bb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BB5C);
    cpu.pc = 0x5E7C0;
    16
}
/// $09BB5C: lea.l $9bb92(pc), a1
pub fn i_09bb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9BB92;
    16
}
/// $09BB60: jsr $43fac.l
pub fn i_09bb60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BB66);
    cpu.pc = 0x43FAC;
    16
}
/// $09BB66: move.w #$1c6, d1
pub fn i_09bb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB6A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BB6A: jsr $236e.l
pub fn i_09bb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BB70);
    cpu.pc = 0x236E;
    16
}
/// $09BB70: lea.l $2f73b8.l, a0
pub fn i_09bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F73B8;
    20
}
/// $09BB76: jsr $28cd4.l
pub fn i_09bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BB7C);
    cpu.pc = 0x28CD4;
    16
}
/// $09BB7C: lea.l $9bb82(pc), a1
pub fn i_09bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9BB82;
    16
}
/// $09BB80: move.l a1, (a6)
pub fn i_09bb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09BB82: jsr $2783a.l
pub fn i_09bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BB88);
    cpu.pc = 0x2783A;
    16
}
/// $09BB88: jsr $28d70.l
pub fn i_09bb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BB8E);
    cpu.pc = 0x28D70;
    16
}
/// $09BB8E: bra.w $9a76a
pub fn i_09bb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB92;
    cpu.ir = 0x6000;
    cpu.pc = 0x9A76A;
    10
}
// ===== Item_Tmpl317_BlitList_09bb92  ($09BB92, 536 B, props/items_score_crates_09a0bc.s)
/// $09BB92: dc.w $ff80
pub fn i_09bb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BB94;
    cpu.ir = 0xFF80;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Entity_CmpPrioWithSibling_09bdaa  ($09BDAA, 16 B, props/items_score_crates_09a0bc.s)
/// $09BDAA: movea.l $8(a6), a1
pub fn i_09bdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDAE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09BDAE: move.b $10(a6), d0
pub fn i_09bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDB2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09BDB2: cmp.b $10(a1), d0
pub fn i_09bdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDB6;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $09BDB6: bcs.w $9bdc0
pub fn i_09bdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDBA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9BDC0; return 10; }
    8
}
// ===== ClearXN_09bdba  ($09BDBA, 6 B, thunks/ccr_helpers.c)
/// $09BDBA: andi.b #$ee, ccr
pub fn i_09bdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDBE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09BDBE: rts
pub fn i_09bdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_09bdc0  ($09BDC0, 6 B, thunks/ccr_helpers.c)
/// $09BDC0: ori.b #$11, ccr
pub fn i_09bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDC4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09BDC4: rts
pub fn i_09bdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chute_SpawnByKind_09bdc6  ($09BDC6, 88 B, props/items_score_crates_09a0bc.s)
/// $09BDC6: lea.l $9be1e(pc), a1
pub fn i_09bdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9BE1E;
    16
}
/// $09BDCA: jsr $4ae.l
pub fn i_09bdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BDD0);
    cpu.pc = 0x4AE;
    16
}
/// $09BDD0: bra.w $9bdec
pub fn i_09bdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDD4;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BDEC;
    10
}
/// $09BDD4: lea.l $9be2e(pc), a1
pub fn i_09bdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9BE2E;
    16
}
/// $09BDD8: jsr $4ae.l
pub fn i_09bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BDDE);
    cpu.pc = 0x4AE;
    16
}
/// $09BDDE: bra.w $9bdec
pub fn i_09bdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BDEC;
    10
}
/// $09BDE2: lea.l $9be3e(pc), a1
pub fn i_09bde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9BE3E;
    16
}
/// $09BDE6: jsr $4ae.l
pub fn i_09bde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BDEC);
    cpu.pc = 0x4AE;
    16
}
/// $09BDEC: move.w $70(a6), d0
pub fn i_09bdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDF0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09BDF0: btst.b #$0, $3a(a6)
pub fn i_09bdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDF6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09BDF6: beq.w $9bdfc
pub fn i_09bdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDFA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9BDFC; return 10; }
    8
}
/// $09BDFA: neg.w d0
pub fn i_09bdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BDFC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09BDFC: add.w $22(a6), d0
pub fn i_09bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE00;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09BE00: move.w d0, $22(a0)
pub fn i_09be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE04;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BE04: move.w $72(a6), d0
pub fn i_09be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE08;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09BE08: btst.b #$1, $3a(a6)
pub fn i_09be08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE0E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09BE0E: beq.w $9be14
pub fn i_09be0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE12;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9BE14; return 10; }
    8
}
/// $09BE12: neg.w d0
pub fn i_09be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE14;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09BE14: add.w $24(a6), d0
pub fn i_09be14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE18;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09BE18: move.w d0, $24(a0)
pub fn i_09be18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE1C;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BE1C: rts
pub fn i_09be1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Chute_A_09be1e  ($09BE1E, 16 B, props/items_score_crates_09a0bc.s)
/// $09BE1E: lea.l $2dd4ba.l, a0
pub fn i_09be1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD4BA;
    20
}
/// $09BE24: jsr $28cd4.l
pub fn i_09be24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BE2A);
    cpu.pc = 0x28CD4;
    16
}
/// $09BE2A: bra.w $9be4a
pub fn i_09be2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE2E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BE4A;
    10
}
// ===== Chute_B_09be2e  ($09BE2E, 16 B, props/items_score_crates_09a0bc.s)
/// $09BE2E: lea.l $2dd594.l, a0
pub fn i_09be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD594;
    20
}
/// $09BE34: jsr $28cd4.l
pub fn i_09be34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BE3A);
    cpu.pc = 0x28CD4;
    16
}
/// $09BE3A: bra.w $9be4a
pub fn i_09be3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE3E;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BE4A;
    10
}
// ===== Chute_C_09be3e  ($09BE3E, 96 B, props/items_score_crates_09a0bc.s)
/// $09BE3E: lea.l $2dd37e.l, a0
pub fn i_09be3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DD37E;
    20
}
/// $09BE44: jsr $28cd4.l
pub fn i_09be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BE4A);
    cpu.pc = 0x28CD4;
    16
}
/// $09BE4A: move.w #$163, d1
pub fn i_09be4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE4E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x163;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BE4E: jsr $236e.l
pub fn i_09be4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BE54);
    cpu.pc = 0x236E;
    16
}
/// $09BE54: move.w #$2000, d0
pub fn i_09be54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE58;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BE58: jsr $28134.l
pub fn i_09be58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BE5E);
    cpu.pc = 0x28134;
    16
}
/// $09BE5E: andi.w #$ffe3, $38(a6)
pub fn i_09be5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE64;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09BE64: ori.w #$10, $38(a6)
pub fn i_09be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE6A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09BE6A: lea.l $9be70(pc), a1
pub fn i_09be6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9BE70;
    16
}
/// $09BE6E: move.l a1, (a6)
pub fn i_09be6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09BE70: jsr $2783a.l
pub fn i_09be70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BE76);
    cpu.pc = 0x2783A;
    16
}
/// $09BE76: jsr $28d70.l
pub fn i_09be76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BE7C);
    cpu.pc = 0x28D70;
    16
}
/// $09BE7C: bcc.w $9be86
pub fn i_09be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9BE86; return 10; }
    8
}
/// $09BE80: jmp $518.l
pub fn i_09be80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE86;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09BE86: movea.l #$ffffffff, a0
pub fn i_09be86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE8C;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $09BE8C: jsr $5dd56.l
pub fn i_09be8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BE92);
    cpu.pc = 0x5DD56;
    16
}
/// $09BE92: bcc.w $9be9c
pub fn i_09be92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9BE9C; return 10; }
    8
}
/// $09BE96: jmp $518.l
pub fn i_09be96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE9C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09BE9C: rts
pub fn i_09be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BE9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Hitbox_09be9e  ($09BE9E, 8 B, props/items_score_crates_09a0bc.s)
/// $09BE9E: dc.w $ffd0
pub fn i_09be9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEA0;
    cpu.ir = 0xFFD0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Crate_Init_09bea6  ($09BEA6, 344 B, props/items_score_crates_09a0bc.s)
/// $09BEA6: jsr $32b36.l
pub fn i_09bea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BEAC);
    cpu.pc = 0x32B36;
    16
}
/// $09BEAC: lea.l $9be9e(pc), a0
pub fn i_09beac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEB0;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9BE9E;
    16
}
/// $09BEB0: jsr $5dd56.l
pub fn i_09beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BEB6);
    cpu.pc = 0x5DD56;
    16
}
/// $09BEB6: bcc.w $9bec0
pub fn i_09beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9BEC0; return 10; }
    8
}
/// $09BEBA: jmp $518.l
pub fn i_09beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEC0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09BEC0: rts
pub fn i_09bec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $09BEC2: move.w #$d000, d0
pub fn i_09bec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BEC6: jsr $28134.l
pub fn i_09bec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BECC);
    cpu.pc = 0x28134;
    16
}
/// $09BECC: andi.w #$ffe3, $38(a6)
pub fn i_09becc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BED2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09BED2: ori.w #$10, $38(a6)
pub fn i_09bed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BED8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09BED8: move.w #$17c, d1
pub fn i_09bed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEDC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09BEDC: jsr $236e.l
pub fn i_09bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BEE2);
    cpu.pc = 0x236E;
    16
}
/// $09BEE2: move.b $98(a6), d0
pub fn i_09bee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEE6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09BEE6: andi.b #$1, d0
pub fn i_09bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEEA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09BEEA: beq.w $9bf22
pub fn i_09beea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEEE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9BF22; return 10; }
    8
}
/// $09BEEE: lea.l $2f7a70.l, a0
pub fn i_09beee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7A70;
    20
}
/// $09BEF4: jsr $28cd4.l
pub fn i_09bef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BEFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BEFA);
    cpu.pc = 0x28CD4;
    16
}
/// $09BEFA: move.w #$20, $2e(a6)
pub fn i_09befa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF00;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BF00: clr.w $2c(a6)
pub fn i_09bf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF04;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09BF04: lea.l $2f7ed4.l, a0
pub fn i_09bf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7ED4;
    20
}
/// $09BF0A: move.l a0, $4c(a6)
pub fn i_09bf0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF0E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09BF0E: jsr $283ca.l
pub fn i_09bf0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BF14);
    cpu.pc = 0x283CA;
    16
}
/// $09BF14: lea.l $9c1ac(pc), a1
pub fn i_09bf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C1AC;
    16
}
/// $09BF18: jsr $4ae.l
pub fn i_09bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BF1E);
    cpu.pc = 0x4AE;
    16
}
/// $09BF1E: bra.w $9bf6a
pub fn i_09bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF22;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BF6A;
    10
}
/// $09BF22: tst.b $9a(a6)
pub fn i_09bf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF26;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $09BF26: bne.w $9bf3a
pub fn i_09bf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9BF3A; return 10; }
    8
}
/// $09BF2A: lea.l $2f7ac6.l, a0
pub fn i_09bf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7AC6;
    20
}
/// $09BF30: jsr $28cd4.l
pub fn i_09bf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BF36);
    cpu.pc = 0x28CD4;
    16
}
/// $09BF36: bra.w $9bf46
pub fn i_09bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF3A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BF46;
    10
}
/// $09BF3A: lea.l $2f7aac.l, a0
pub fn i_09bf3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7AAC;
    20
}
/// $09BF40: jsr $28cd4.l
pub fn i_09bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BF46);
    cpu.pc = 0x28CD4;
    16
}
/// $09BF46: move.w #$20, $2c(a6)
pub fn i_09bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF4C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BF4C: clr.w $2e(a6)
pub fn i_09bf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF50;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09BF50: lea.l $2f7ddc.l, a0
pub fn i_09bf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF56;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7DDC;
    20
}
/// $09BF56: move.l a0, $4c(a6)
pub fn i_09bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF5A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09BF5A: jsr $283ca.l
pub fn i_09bf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BF60);
    cpu.pc = 0x283CA;
    16
}
/// $09BF60: lea.l $9c13c(pc), a1
pub fn i_09bf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C13C;
    16
}
/// $09BF64: jsr $4ae.l
pub fn i_09bf64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BF6A);
    cpu.pc = 0x4AE;
    16
}
/// $09BF6A: move.b $98(a6), d0
pub fn i_09bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF6E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09BF6E: cmpi.b #$3, d0
pub fn i_09bf6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF72;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $09BF72: bne.w $9bf84
pub fn i_09bf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9BF84; return 10; }
    8
}
/// $09BF76: bset.b #$1, $3a(a6)
pub fn i_09bf76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF7C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09BF7C: neg.w $2e(a6)
pub fn i_09bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF80;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09BF80: bra.w $9bf9c
pub fn i_09bf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF84;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BF9C;
    10
}
/// $09BF84: move.b $99(a6), d1
pub fn i_09bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF88;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09BF88: eor.b d1, d0
pub fn i_09bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF8A;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09BF8A: andi.b #$2, d0
pub fn i_09bf8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF8E;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09BF8E: beq.w $9bf9c
pub fn i_09bf8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9BF9C; return 10; }
    8
}
/// $09BF92: bset.b #$0, $3a(a6)
pub fn i_09bf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF98;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09BF98: neg.w $2c(a6)
pub fn i_09bf98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BF9C;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09BF9C: clr.w $76(a6)
pub fn i_09bf9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFA0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09BFA0: lea.l $9bfa6(pc), a1
pub fn i_09bfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9BFA6;
    16
}
/// $09BFA4: move.l a1, (a6)
pub fn i_09bfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09BFA6: jsr $27d50.l
pub fn i_09bfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BFAC);
    cpu.pc = 0x27D50;
    16
}
/// $09BFAC: bcs.w $9bfb6
pub fn i_09bfac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFB0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9BFB6; return 10; }
    8
}
/// $09BFB0: jsr $27d50.l
pub fn i_09bfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BFB6);
    cpu.pc = 0x27D50;
    16
}
/// $09BFB6: bcc.w $9bfc0
pub fn i_09bfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9BFC0; return 10; }
    8
}
/// $09BFBA: lea.l $9bffe(pc), a1
pub fn i_09bfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9BFFE;
    16
}
/// $09BFBE: move.l a1, (a6)
pub fn i_09bfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09BFC0: subq.w #$1, $76(a6)
pub fn i_09bfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFC4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09BFC4: bcc.w $9bfde
pub fn i_09bfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9BFDE; return 10; }
    8
}
/// $09BFC8: move.w #$2, $76(a6)
pub fn i_09bfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFCE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09BFCE: lea.l $9c0a4(pc), a1
pub fn i_09bfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C0A4;
    16
}
/// $09BFD2: jsr $4ae.l
pub fn i_09bfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BFD8);
    cpu.pc = 0x4AE;
    16
}
/// $09BFD8: jsr $5dd02.l
pub fn i_09bfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BFDE);
    cpu.pc = 0x5DD02;
    16
}
/// $09BFDE: jsr $28d70.l
pub fn i_09bfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BFE4);
    cpu.pc = 0x28D70;
    16
}
/// $09BFE4: jsr $283d8.l
pub fn i_09bfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9BFEA);
    cpu.pc = 0x283D8;
    16
}
/// $09BFEA: btst.b #$1, $13(a6)
pub fn i_09bfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFF0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09BFF0: beq.w $9bffa
pub fn i_09bff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9BFFA; return 10; }
    8
}
/// $09BFF4: lea.l $9c00a(pc), a1
pub fn i_09bff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C00A;
    16
}
/// $09BFF8: move.l a1, (a6)
pub fn i_09bff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09BFFA: bra.w $9bea6
pub fn i_09bffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9BFFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BEA6;
    10
}
// ===== Crate_Destroyed_09bffe  ($09BFFE, 12 B, props/items_score_crates_09a0bc.s)
/// $09BFFE: jsr $13600.l
pub fn i_09bffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C004;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C004);
    cpu.pc = 0x13600;
    16
}
/// $09C004: jmp $77efe.l
pub fn i_09c004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C00A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77EFE;
    8
}
// ===== Crate_Opened_09c00a  ($09C00A, 104 B, props/items_score_crates_09a0bc.s)
/// $09C00A: move.w #$1025, d0
pub fn i_09c00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C00E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1025;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C00E: jsr $2352.l
pub fn i_09c00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C014);
    cpu.pc = 0x2352;
    16
}
/// $09C014: jsr $13600.l
pub fn i_09c014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C01A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C01A);
    cpu.pc = 0x13600;
    16
}
/// $09C01A: move.w #$4, d1
pub fn i_09c01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C01E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C01E: jsr $236e.l
pub fn i_09c01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C024;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C024);
    cpu.pc = 0x236E;
    16
}
/// $09C024: move.b $98(a6), d0
pub fn i_09c024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C028;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C028: andi.b #$1, d0
pub fn i_09c028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C02C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09C02C: beq.w $9c040
pub fn i_09c02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C030;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C040; return 10; }
    8
}
/// $09C030: lea.l $2f7cc0.l, a0
pub fn i_09c030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C036;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7CC0;
    20
}
/// $09C036: jsr $28cd4.l
pub fn i_09c036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C03C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C03C);
    cpu.pc = 0x28CD4;
    16
}
/// $09C03C: bra.w $9c04c
pub fn i_09c03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C040;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C04C;
    10
}
/// $09C040: lea.l $2f7c46.l, a0
pub fn i_09c040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C046;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7C46;
    20
}
/// $09C046: jsr $28cd4.l
pub fn i_09c046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C04C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C04C);
    cpu.pc = 0x28CD4;
    16
}
/// $09C04C: lea.l $9c052(pc), a1
pub fn i_09c04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C050;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C052;
    16
}
/// $09C050: move.l a1, (a6)
pub fn i_09c050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C052;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C052: jsr $283ca.l
pub fn i_09c052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C058);
    cpu.pc = 0x283CA;
    16
}
/// $09C058: jsr $2783a.l
pub fn i_09c058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C05E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C05E);
    cpu.pc = 0x2783A;
    16
}
/// $09C05E: jsr $28d70.l
pub fn i_09c05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C064;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C064);
    cpu.pc = 0x28D70;
    16
}
/// $09C064: bcc.w $9c06e
pub fn i_09c064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C068;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C06E; return 10; }
    8
}
/// $09C068: jmp $518.l
pub fn i_09c068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C06E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09C06E: bra.w $9bea6
pub fn i_09c06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C072;
    cpu.ir = 0x6000;
    cpu.pc = 0x9BEA6;
    10
}
// ===== Entity_IntegrateVelFrac_09c072  ($09C072, 50 B, props/items_score_crates_09a0bc.s)
/// $09C072: move.b $29(a6), d0
pub fn i_09c072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C076;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x29);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C076: move.b $28(a6), d1
pub fn i_09c076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C07A;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C07A: ext.w d1
pub fn i_09c07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C07C;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $09C07C: move.w $22(a6), d2
pub fn i_09c07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C080;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C080: add.b d0, $26(a6)
pub fn i_09c080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C084;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09C084: addx.w d1, d2
pub fn i_09c084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C086;
    cpu.ir = 0xD541;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let h2 = cpu.d[1] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C086: move.w d2, $22(a6)
pub fn i_09c086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C08A;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C08A: move.b $2b(a6), d0
pub fn i_09c08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C08E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x2B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C08E: move.b $2a(a6), d1
pub fn i_09c08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C092;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C092: ext.w d1
pub fn i_09c092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C094;
    cpu.ir = 0x4881;
    let v = cpu.d[1] as i8 as i16 as u16 as u32; cpu.d[1] = (cpu.d[1] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $09C094: move.w $24(a6), d2
pub fn i_09c094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C098;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C098: add.b d0, $27(a6)
pub fn i_09c098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C09C;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $09C09C: addx.w d1, d2
pub fn i_09c09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C09E;
    cpu.ir = 0xD541;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let h2 = cpu.d[1] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C09E: move.w d2, $24(a6)
pub fn i_09c09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0A2;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C0A2: rts
pub fn i_09c0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0A4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_Debris_09c0a4  ($09C0A4, 152 B, props/items_score_crates_09a0bc.s)
/// $09C0A4: move.w #$17c, d1
pub fn i_09c0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0A8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x17C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C0A8: jsr $236e.l
pub fn i_09c0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C0AE);
    cpu.pc = 0x236E;
    16
}
/// $09C0AE: move.w #$2000, d0
pub fn i_09c0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C0B2: jsr $28134.l
pub fn i_09c0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C0B8);
    cpu.pc = 0x28134;
    16
}
/// $09C0B8: andi.w #$ffe3, $38(a6)
pub fn i_09c0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0BE;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09C0BE: ori.w #$10, $38(a6)
pub fn i_09c0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0C4;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09C0C4: lea.l $2f7d3a.l, a0
pub fn i_09c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7D3A;
    20
}
/// $09C0CA: jsr $28cd4.l
pub fn i_09c0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C0D0);
    cpu.pc = 0x28CD4;
    16
}
/// $09C0D0: movea.l $c(a6), a0
pub fn i_09c0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0D4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C0D4: move.w $74(a0), d0
pub fn i_09c0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0D8;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C0D8: move.w #$400, d1
pub fn i_09c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0DC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C0DC: jsr $13c0e.l
pub fn i_09c0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C0E2);
    cpu.pc = 0x13C0E;
    16
}
/// $09C0E2: btst.b #$0, $3a(a0)
pub fn i_09c0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0E8;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09C0E8: beq.w $9c0ee
pub fn i_09c0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C0EE; return 10; }
    8
}
/// $09C0EC: neg.w d1
pub fn i_09c0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0EE;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09C0EE: move.w $28(a0), d3
pub fn i_09c0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0F2;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C0F2: add.w d3, d3
pub fn i_09c0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0F4;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C0F4: add.w d3, d1
pub fn i_09c0f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0F6;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C0F6: move.w $2a(a6), d3
pub fn i_09c0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0FA;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C0FA: add.w d3, d3
pub fn i_09c0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0FC;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C0FC: add.w d3, d2
pub fn i_09c0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C0FE;
    cpu.ir = 0xD443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C0FE: move.w d1, $28(a6)
pub fn i_09c0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C102;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C102: move.w d2, $2a(a6)
pub fn i_09c102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C106;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C106: lea.l $9c10c(pc), a1
pub fn i_09c106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C10A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C10C;
    16
}
/// $09C10A: move.l a1, (a6)
pub fn i_09c10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C10C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C10C: jsr $2783a.l
pub fn i_09c10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C112;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C112);
    cpu.pc = 0x2783A;
    16
}
/// $09C112: jsr $9c072(pc)
pub fn i_09c112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C116;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x9C116);
    cpu.pc = 0x9C072;
    16
}
/// $09C116: move.w $28(a6), d0
pub fn i_09c116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C11A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C11A: asr.w #$4, d0
pub fn i_09c11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C11C;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $09C11C: sub.w d0, $28(a6)
pub fn i_09c11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C120;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C120: move.w $2a(a6), d0
pub fn i_09c120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C124;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C124: asr.w #$4, d0
pub fn i_09c124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C126;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $09C126: sub.w d0, $2a(a6)
pub fn i_09c126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C12A;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C12A: jsr $28d70.l
pub fn i_09c12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C130);
    cpu.pc = 0x28D70;
    16
}
/// $09C130: bcc.w $9c13a
pub fn i_09c130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C134;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C13A; return 10; }
    8
}
/// $09C134: jmp $518.l
pub fn i_09c134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C13A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09C13A: rts
pub fn i_09c13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C13C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_ChuteFollow_09c13c  ($09C13C, 112 B, props/items_score_crates_09a0bc.s)
/// $09C13C: lea.l $2f7e80.l, a0
pub fn i_09c13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C142;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7E80;
    20
}
/// $09C142: move.l a0, $4c(a6)
pub fn i_09c142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C146;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09C146: jsr $283ca.l
pub fn i_09c146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C14C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C14C);
    cpu.pc = 0x283CA;
    16
}
/// $09C14C: lea.l $9c152(pc), a1
pub fn i_09c14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C150;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C152;
    16
}
/// $09C150: move.l a1, (a6)
pub fn i_09c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C152;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C152: jsr $7b2.l
pub fn i_09c152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C158;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C158);
    cpu.pc = 0x7B2;
    16
}
/// $09C158: bcc.w $9c162
pub fn i_09c158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C15C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C162; return 10; }
    8
}
/// $09C15C: jmp $518.l
pub fn i_09c15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C162;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09C162: movea.l $c(a6), a0
pub fn i_09c162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C166;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C166: move.w $22(a0), $22(a6)
pub fn i_09c166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C16C;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09C16C: move.w $24(a0), $24(a6)
pub fn i_09c16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C172;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09C172: move.b $3a(a0), $3a(a6)
pub fn i_09c172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C178;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09C178: jsr $283ca.l
pub fn i_09c178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C17E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C17E);
    cpu.pc = 0x283CA;
    16
}
/// $09C17E: jsr $283d8.l
pub fn i_09c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C184);
    cpu.pc = 0x283D8;
    16
}
/// $09C184: btst.b #$1, $13(a6)
pub fn i_09c184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C18A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09C18A: beq.w $9c1aa
pub fn i_09c18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C18E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C1AA; return 10; }
    8
}
/// $09C18E: movea.l $c(a6), a0
pub fn i_09c18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C192;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C192: move.w $24(a1), d0
pub fn i_09c192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C196;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C196: addi.w #$1e, d0
pub fn i_09c196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C19A;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1E, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $09C19A: sub.w $24(a0), d0
pub fn i_09c19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C19E;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C19E: asl.w #$3, d0
pub fn i_09c19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1A0;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C1A0: sub.w $2a(a0), d0
pub fn i_09c1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1A4;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C1A4: asr.w #$3, d0
pub fn i_09c1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1A6;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C1A6: move.w d0, $2e(a0)
pub fn i_09c1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1AA;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C1AA: rts
pub fn i_09c1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crate_ChuteFollowX_09c1ac  ($09C1AC, 108 B, props/items_score_crates_09a0bc.s)
/// $09C1AC: lea.l $2f7f78.l, a0
pub fn i_09c1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F7F78;
    20
}
/// $09C1B2: move.l a0, $4c(a6)
pub fn i_09c1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1B6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09C1B6: jsr $283ca.l
pub fn i_09c1b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C1BC);
    cpu.pc = 0x283CA;
    16
}
/// $09C1BC: lea.l $9c1c2(pc), a1
pub fn i_09c1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C1C2;
    16
}
/// $09C1C0: move.l a1, (a6)
pub fn i_09c1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C1C2: jsr $7b2.l
pub fn i_09c1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C1C8);
    cpu.pc = 0x7B2;
    16
}
/// $09C1C8: bcc.w $9c1d2
pub fn i_09c1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C1D2; return 10; }
    8
}
/// $09C1CC: jmp $518.l
pub fn i_09c1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1D2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09C1D2: movea.l $c(a6), a0
pub fn i_09c1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1D6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C1D6: move.w $22(a0), $22(a6)
pub fn i_09c1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1DC;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09C1DC: move.w $24(a0), $24(a6)
pub fn i_09c1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1E2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09C1E2: move.b $3a(a0), $3a(a6)
pub fn i_09c1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1E8;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $09C1E8: jsr $283ca.l
pub fn i_09c1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C1EE);
    cpu.pc = 0x283CA;
    16
}
/// $09C1EE: jsr $283d8.l
pub fn i_09c1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C1F4);
    cpu.pc = 0x283D8;
    16
}
/// $09C1F4: btst.b #$1, $13(a6)
pub fn i_09c1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1FA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09C1FA: beq.w $9c216
pub fn i_09c1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C1FE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C216; return 10; }
    8
}
/// $09C1FE: movea.l $c(a6), a0
pub fn i_09c1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C202;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C202: move.w $22(a1), d0
pub fn i_09c202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C206;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C206: sub.w $22(a0), d0
pub fn i_09c206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C20A;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C20A: asl.w #$3, d0
pub fn i_09c20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C20C;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C20C: sub.w $28(a0), d0
pub fn i_09c20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C210;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C210: asr.w #$3, d0
pub fn i_09c210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C212;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C212: move.w d0, $2c(a0)
pub fn i_09c212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C216;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C216: rts
pub fn i_09c216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C218;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_09c218  ($09C218, 16 B, props/items_score_crates_09a0bc.s)
/// $09C218: movea.l $8(a6), a1
pub fn i_09c218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C21C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09C21C: move.b $10(a6), d0
pub fn i_09c21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C220;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C220: cmp.b $10(a1), d0
pub fn i_09c220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C224;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $09C224: bcs.w $9c22e
pub fn i_09c224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C228;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9C22E; return 10; }
    8
}
// ===== ClearXN_09c228  ($09C228, 6 B, thunks/ccr_helpers.c)
/// $09C228: andi.b #$ee, ccr
pub fn i_09c228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C22C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09C22C: rts
pub fn i_09c22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C22E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_09c22e  ($09C22E, 6 B, thunks/ccr_helpers.c)
/// $09C22E: ori.b #$11, ccr
pub fn i_09c22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C232;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09C232: rts
pub fn i_09c232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C234;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Thrown_Hitbox_09c234  ($09C234, 8 B, props/items_score_crates_09a0bc.s)
/// $09C234: dc.w $ffd0
pub fn i_09c234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C236;
    cpu.ir = 0xFFD0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Thrown_GroundTail_09c23c  ($09C23C, 20 B, props/items_score_crates_09a0bc.s)
/// $09C23C: jsr $32b36.l
pub fn i_09c23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C242);
    cpu.pc = 0x32B36;
    16
}
/// $09C242: lea.l $9c234(pc), a0
pub fn i_09c242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C246;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x9C234;
    16
}
/// $09C246: jsr $5dd56.l
pub fn i_09c246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C24C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C24C);
    cpu.pc = 0x5DD56;
    16
}
/// $09C24C: bcc.w $9c25c
pub fn i_09c24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C250;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C25C; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_09c250  ($09C250, 14 B, kernel/jsr5b6_jmp518.c)
/// $09C250: jsr $5b6.l
pub fn i_09c250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C256);
    cpu.pc = 0x5B6;
    16
}
/// $09C256: jmp $518.l
pub fn i_09c256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C25C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09C25C: rts
pub fn i_09c25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C25E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Thrown_Init_09c25e  ($09C25E, 316 B, props/items_score_crates_09a0bc.s)
/// $09C25E: move.w #$d000, d0
pub fn i_09c25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C262;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C262: jsr $28134.l
pub fn i_09c262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C268;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C268);
    cpu.pc = 0x28134;
    16
}
/// $09C268: andi.w #$ffe3, $38(a6)
pub fn i_09c268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C26E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09C26E: ori.w #$10, $38(a6)
pub fn i_09c26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C274;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09C274: move.w #$184, d1
pub fn i_09c274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C278;
    cpu.ir = 0x323C;
    let v1: u32 = 0x184;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C278: jsr $236e.l
pub fn i_09c278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C27E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C27E);
    cpu.pc = 0x236E;
    16
}
/// $09C27E: move.b $98(a6), d0
pub fn i_09c27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C282;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C282: andi.b #$1, d0
pub fn i_09c282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C286;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09C286: beq.w $9c2a0
pub fn i_09c286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C28A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C2A0; return 10; }
    8
}
/// $09C28A: lea.l $2f83a4.l, a0
pub fn i_09c28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C290;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F83A4;
    20
}
/// $09C290: jsr $28cd4.l
pub fn i_09c290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C296;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C296);
    cpu.pc = 0x28CD4;
    16
}
/// $09C296: move.w #$800, $2a(a6)
pub fn i_09c296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C29C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C29C: bra.w $9c2b8
pub fn i_09c29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C2B8;
    10
}
/// $09C2A0: lea.l $2f84ae.l, a0
pub fn i_09c2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F84AE;
    20
}
/// $09C2A6: jsr $28cd4.l
pub fn i_09c2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C2AC);
    cpu.pc = 0x28CD4;
    16
}
/// $09C2AC: move.w #$800, $28(a6)
pub fn i_09c2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C2B2: move.w #$fe00, $2a(a6)
pub fn i_09c2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C2B8: move.b $98(a6), d0
pub fn i_09c2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2BC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C2BC: cmpi.b #$3, d0
pub fn i_09c2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2C0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $09C2C0: bne.w $9c2d2
pub fn i_09c2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9C2D2; return 10; }
    8
}
/// $09C2C4: bset.b #$1, $3a(a6)
pub fn i_09c2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2CA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09C2CA: neg.w $2a(a6)
pub fn i_09c2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2CE;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C2CE: bra.w $9c2ea
pub fn i_09c2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C2EA;
    10
}
/// $09C2D2: move.b $99(a6), d1
pub fn i_09c2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2D6;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C2D6: eor.b d1, d0
pub fn i_09c2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2D8;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09C2D8: andi.b #$2, d0
pub fn i_09c2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2DC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09C2DC: beq.w $9c2ea
pub fn i_09c2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C2EA; return 10; }
    8
}
/// $09C2E0: bset.b #$0, $3a(a6)
pub fn i_09c2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2E6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09C2E6: neg.w $28(a6)
pub fn i_09c2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2EA;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C2EA: lea.l $2f85b8.l, a0
pub fn i_09c2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2F0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F85B8;
    20
}
/// $09C2F0: move.l a0, $4c(a6)
pub fn i_09c2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2F4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $09C2F4: jsr $283ca.l
pub fn i_09c2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C2FA);
    cpu.pc = 0x283CA;
    16
}
/// $09C2FA: lea.l $9c3f0(pc), a1
pub fn i_09c2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C2FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C3F0;
    16
}
/// $09C2FE: jsr $4ae.l
pub fn i_09c2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C304);
    cpu.pc = 0x4AE;
    16
}
/// $09C304: lea.l $9c30a(pc), a1
pub fn i_09c304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C308;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C30A;
    16
}
/// $09C308: move.l a1, (a6)
pub fn i_09c308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C30A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C30A: move.b $98(a6), d0
pub fn i_09c30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C30E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C30E: andi.b #$1, d0
pub fn i_09c30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C312;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09C312: beq.w $9c332
pub fn i_09c312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C316;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C332; return 10; }
    8
}
/// $09C316: move.w $28(a6), d0
pub fn i_09c316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C31A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C31A: asr.w #$4, d0
pub fn i_09c31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C31C;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $09C31C: sub.w d0, $28(a6)
pub fn i_09c31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C320;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C320: move.w #$200, d0
pub fn i_09c320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C324;
    cpu.ir = 0x303C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C324: sub.w $2a(a6), d0
pub fn i_09c324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C328;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C328: asr.w #$6, d0
pub fn i_09c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C32A;
    cpu.ir = 0xEC40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 6, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    18
}
/// $09C32A: add.w d0, $2a(a6)
pub fn i_09c32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C32E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C32E: bra.w $9c34a
pub fn i_09c32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C332;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C34A;
    10
}
/// $09C332: move.w $28(a6), d0
pub fn i_09c332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C336;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C336: asr.w #$5, d0
pub fn i_09c336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C338;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $09C338: sub.w d0, $28(a6)
pub fn i_09c338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C33C;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C33C: move.w #$200, d0
pub fn i_09c33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C340;
    cpu.ir = 0x303C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C340: sub.w $2a(a6), d0
pub fn i_09c340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C344;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C344: asr.w #$3, d0
pub fn i_09c344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C346;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C346: add.w d0, $2a(a6)
pub fn i_09c346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C34A;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C34A: jsr $283ca.l
pub fn i_09c34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C350);
    cpu.pc = 0x283CA;
    16
}
/// $09C350: jsr $27cee.l
pub fn i_09c350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C356;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C356);
    cpu.pc = 0x27CEE;
    16
}
/// $09C356: bcs.w $9c360
pub fn i_09c356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C35A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9C360; return 10; }
    8
}
/// $09C35A: jsr $27cee.l
pub fn i_09c35a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C360);
    cpu.pc = 0x27CEE;
    16
}
/// $09C360: bcc.w $9c36a
pub fn i_09c360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C364;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C36A; return 10; }
    8
}
/// $09C364: lea.l $9c39a(pc), a1
pub fn i_09c364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C368;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C39A;
    16
}
/// $09C368: move.l a1, (a6)
pub fn i_09c368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C36A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C36A: jsr $28d70.l
pub fn i_09c36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C370;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C370);
    cpu.pc = 0x28D70;
    16
}
/// $09C370: bcc.w $9c380
pub fn i_09c370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C374;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C380; return 10; }
    8
}
/// $09C374: jsr $5b6.l
pub fn i_09c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C37A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C37A);
    cpu.pc = 0x5B6;
    16
}
/// $09C37A: jmp $518.l
pub fn i_09c37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C380;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09C380: jsr $283d8.l
pub fn i_09c380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C386;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C386);
    cpu.pc = 0x283D8;
    16
}
/// $09C386: btst.b #$1, $13(a6)
pub fn i_09c386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C38C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09C38C: beq.w $9c396
pub fn i_09c38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C390;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C396; return 10; }
    8
}
/// $09C390: lea.l $9c39a(pc), a1
pub fn i_09c390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C394;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C39A;
    16
}
/// $09C394: move.l a1, (a6)
pub fn i_09c394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C396;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C396: bra.w $9c23c
pub fn i_09c396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C39A;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C23C;
    10
}
// ===== Thrown_Landed_09c39a  ($09C39A, 86 B, props/items_score_crates_09a0bc.s)
/// $09C39A: move.b $98(a6), d0
pub fn i_09c39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C39E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C39E: andi.b #$1, d0
pub fn i_09c39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3A2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09C3A2: beq.w $9c3ba
pub fn i_09c3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C3BA; return 10; }
    8
}
/// $09C3A6: clr.w $2a(a6)
pub fn i_09c3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3AA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C3AA: lea.l $2f8416.l, a0
pub fn i_09c3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F8416;
    20
}
/// $09C3B0: jsr $28cd4.l
pub fn i_09c3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C3B6);
    cpu.pc = 0x28CD4;
    16
}
/// $09C3B6: bra.w $9c3ca
pub fn i_09c3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C3CA;
    10
}
/// $09C3BA: clr.w $28(a6)
pub fn i_09c3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3BE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C3BE: lea.l $2f8520.l, a0
pub fn i_09c3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F8520;
    20
}
/// $09C3C4: jsr $28cd4.l
pub fn i_09c3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C3CA);
    cpu.pc = 0x28CD4;
    16
}
/// $09C3CA: lea.l $9c3d0(pc), a1
pub fn i_09c3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C3D0;
    16
}
/// $09C3CE: move.l a1, (a6)
pub fn i_09c3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C3D0: jsr $27cee.l
pub fn i_09c3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C3D6);
    cpu.pc = 0x27CEE;
    16
}
/// $09C3D6: jsr $28d70.l
pub fn i_09c3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C3DC);
    cpu.pc = 0x28D70;
    16
}
/// $09C3DC: bcc.w $9c3ec
pub fn i_09c3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C3EC; return 10; }
    8
}
/// $09C3E0: jsr $5b6.l
pub fn i_09c3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C3E6);
    cpu.pc = 0x5B6;
    16
}
/// $09C3E6: jmp $518.l
pub fn i_09c3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3EC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09C3EC: bra.w $9c23c
pub fn i_09c3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C23C;
    10
}
// ===== Thrown_Shadow_09c3f0  ($09C3F0, 92 B, props/items_score_crates_09a0bc.s)
/// $09C3F0: move.w #$184, d1
pub fn i_09c3f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x184;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C3F4: jsr $236e.l
pub fn i_09c3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C3FA);
    cpu.pc = 0x236E;
    16
}
/// $09C3FA: movea.l $c(a6), a0
pub fn i_09c3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C3FE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C3FE: move.w $22(a0), $22(a6)
pub fn i_09c3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C404;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09C404: move.w $24(a0), $24(a6)
pub fn i_09c404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C40A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09C40A: lea.l $9c410(pc), a1
pub fn i_09c40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C40E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C410;
    16
}
/// $09C40E: move.l a1, (a6)
pub fn i_09c40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C410;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C410: movea.l $c(a6), a0
pub fn i_09c410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C414;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C414: move.w $38(a0), d0
pub fn i_09c414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C418;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C418: subq.w #$1, d0
pub fn i_09c418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C41A;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09C41A: move.w d0, $38(a6)
pub fn i_09c41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C41E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C41E: move.w $22(a0), d0
pub fn i_09c41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C422;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C422: sub.w $22(a6), d0
pub fn i_09c422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C426;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C426: asr.w #$3, d0
pub fn i_09c426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C428;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C428: move.w d0, d1
pub fn i_09c428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C42A;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C42A: add.w d1, d0
pub fn i_09c42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C42C;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C42C: add.w d1, d0
pub fn i_09c42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C42E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C42E: add.w d0, $22(a6)
pub fn i_09c42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C432;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C432: move.w $24(a0), d0
pub fn i_09c432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C436;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C436: sub.w $24(a6), d0
pub fn i_09c436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C43A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C43A: asr.w #$3, d0
pub fn i_09c43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C43C;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C43C: move.w d0, d1
pub fn i_09c43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C43E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C43E: add.w d1, d0
pub fn i_09c43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C440;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C440: add.w d1, d0
pub fn i_09c440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C442;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C442: add.w d0, $24(a6)
pub fn i_09c442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C446;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C446: move.l $70(a0), $3c(a6)
pub fn i_09c446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C44C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
// ===== JsrAbsThunk_09c44c  ($09C44C, 8 B, thunks/jsr_abs_thunks.c)
/// $09C44C: jsr $5ca2a.l
pub fn i_09c44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C452;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C452);
    cpu.pc = 0x5CA2A;
    16
}
/// $09C452: rts
pub fn i_09c452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C454;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Thrown_ShadowB_09c454  ($09C454, 92 B, props/items_score_crates_09a0bc.s)
/// $09C454: move.w #$184, d1
pub fn i_09c454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C458;
    cpu.ir = 0x323C;
    let v1: u32 = 0x184;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C458: jsr $236e.l
pub fn i_09c458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C45E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C45E);
    cpu.pc = 0x236E;
    16
}
/// $09C45E: movea.l $c(a6), a0
pub fn i_09c45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C462;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C462: move.w $22(a0), $22(a6)
pub fn i_09c462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C468;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09C468: move.w $24(a0), $24(a6)
pub fn i_09c468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C46E;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $09C46E: lea.l $9c474(pc), a1
pub fn i_09c46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C472;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C474;
    16
}
/// $09C472: move.l a1, (a6)
pub fn i_09c472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C474;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C474: movea.l $c(a6), a0
pub fn i_09c474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C478;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $09C478: move.w $38(a0), d0
pub fn i_09c478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C47C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C47C: subq.w #$1, d0
pub fn i_09c47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C47E;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09C47E: move.w d0, $38(a6)
pub fn i_09c47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C482;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C482: move.w $22(a0), d0
pub fn i_09c482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C486;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C486: sub.w $22(a6), d0
pub fn i_09c486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C48A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C48A: asr.w #$2, d0
pub fn i_09c48a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C48C;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09C48C: move.w d0, d1
pub fn i_09c48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C48E;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C48E: add.w d1, d0
pub fn i_09c48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C490;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C490: add.w d1, d0
pub fn i_09c490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C492;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C492: add.w d0, $22(a6)
pub fn i_09c492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C496;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C496: move.w $24(a0), d0
pub fn i_09c496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C49A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $09C49A: sub.w $24(a6), d0
pub fn i_09c49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C49E;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $09C49E: asr.w #$2, d0
pub fn i_09c49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4A0;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $09C4A0: move.w d0, d1
pub fn i_09c4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4A2;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C4A2: add.w d1, d0
pub fn i_09c4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4A4;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C4A4: add.w d1, d0
pub fn i_09c4a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4A6;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $09C4A6: add.w d0, $24(a6)
pub fn i_09c4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4AA;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C4AA: move.l $70(a0), $3c(a6)
pub fn i_09c4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4B0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
// ===== JsrAbsThunk_09c4b0  ($09C4B0, 8 B, thunks/jsr_abs_thunks.c)
/// $09C4B0: jsr $5ca2a.l
pub fn i_09c4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C4B6);
    cpu.pc = 0x5CA2A;
    16
}
/// $09C4B6: rts
pub fn i_09c4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_09c4b8  ($09C4B8, 16 B, props/items_score_crates_09a0bc.s)
/// $09C4B8: movea.l $8(a6), a1
pub fn i_09c4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4BC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $09C4BC: move.b $10(a6), d0
pub fn i_09c4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4C0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C4C0: cmp.b $10(a1), d0
pub fn i_09c4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4C4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $09C4C4: bcs.w $9c4ce
pub fn i_09c4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4C8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x9C4CE; return 10; }
    8
}
// ===== ClearXN_09c4c8  ($09C4C8, 6 B, thunks/ccr_helpers.c)
/// $09C4C8: andi.b #$ee, ccr
pub fn i_09c4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4CC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $09C4CC: rts
pub fn i_09c4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_09c4ce  ($09C4CE, 6 B, thunks/ccr_helpers.c)
/// $09C4CE: ori.b #$11, ccr
pub fn i_09c4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4D2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $09C4D2: rts
pub fn i_09c4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flag_Init_09c4d4  ($09C4D4, 300 B, props/items_score_crates_09a0bc.s)
/// $09C4D4: move.b #$1, $10a2d1.l
pub fn i_09c4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4DC;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $09C4DC: move.w #$d000, d0
pub fn i_09c4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C4E0: jsr $28134.l
pub fn i_09c4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C4E6);
    cpu.pc = 0x28134;
    16
}
/// $09C4E6: andi.w #$ffe3, $38(a6)
pub fn i_09c4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4EC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09C4EC: ori.w #$10, $38(a6)
pub fn i_09c4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4F2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $09C4F2: move.w #$19b, d1
pub fn i_09c4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4F6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x19B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C4F6: jsr $236e.l
pub fn i_09c4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C4FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C4FC);
    cpu.pc = 0x236E;
    16
}
/// $09C4FC: move.b $98(a6), d0
pub fn i_09c4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C500;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C500: andi.b #$1, d0
pub fn i_09c500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C504;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09C504: beq.w $9c524
pub fn i_09c504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C508;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C524; return 10; }
    8
}
/// $09C508: lea.l $2f86ec.l, a0
pub fn i_09c508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C50E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F86EC;
    20
}
/// $09C50E: jsr $28cd4.l
pub fn i_09c50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C514;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C514);
    cpu.pc = 0x28CD4;
    16
}
/// $09C514: move.w #$ffff, $5c(a6)
pub fn i_09c514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C51A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C51A: move.w #$0, $5e(a6)
pub fn i_09c51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C520;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C520: bra.w $9c53c
pub fn i_09c520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C524;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C53C;
    10
}
/// $09C524: lea.l $2f865c.l, a0
pub fn i_09c524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C52A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F865C;
    20
}
/// $09C52A: jsr $28cd4.l
pub fn i_09c52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C530;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C530);
    cpu.pc = 0x28CD4;
    16
}
/// $09C530: move.w #$0, $5c(a6)
pub fn i_09c530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C536;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C536: move.w #$ffff, $5e(a6)
pub fn i_09c536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C53C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C53C: move.b $98(a6), d0
pub fn i_09c53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C540;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C540: cmpi.b #$3, d0
pub fn i_09c540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C544;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $09C544: bne.w $9c562
pub fn i_09c544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C548;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9C562; return 10; }
    8
}
/// $09C548: bset.b #$1, $3a(a6)
pub fn i_09c548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C54E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09C54E: cmpi.w #$0, $5e(a6)
pub fn i_09c54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C554;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09C554: bne.w $9c55e
pub fn i_09c554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C558;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9C55E; return 10; }
    8
}
/// $09C558: move.w #$1, $5e(a6)
pub fn i_09c558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C55E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C55E: bra.w $9c586
pub fn i_09c55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C562;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C586;
    10
}
/// $09C562: move.b $99(a6), d1
pub fn i_09c562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C566;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C566: eor.b d1, d0
pub fn i_09c566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C568;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $09C568: andi.b #$2, d0
pub fn i_09c568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C56C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $09C56C: beq.w $9c586
pub fn i_09c56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C570;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C586; return 10; }
    8
}
/// $09C570: bset.b #$0, $3a(a6)
pub fn i_09c570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C576;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $09C576: cmpi.w #$0, $5c(a6)
pub fn i_09c576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C57C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $09C57C: bne.w $9c586
pub fn i_09c57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C580;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9C586; return 10; }
    8
}
/// $09C580: move.w #$1, $5c(a6)
pub fn i_09c580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C586;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C586: move.b $98(a6), d0
pub fn i_09c586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C58A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $09C58A: cmpi.b #$1, d0
pub fn i_09c58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C58E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $09C58E: bne.w $9c5a0
pub fn i_09c58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C592;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9C5A0; return 10; }
    8
}
/// $09C592: addq.w #$2, $22(a6)
pub fn i_09c592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C596;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C596: addi.w #$3e, $24(a6)
pub fn i_09c596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C59C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x3E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $09C59C: bra.w $9c5ca
pub fn i_09c59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5A0;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C5CA;
    10
}
/// $09C5A0: cmpi.b #$3, d0
pub fn i_09c5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5A4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $09C5A4: bne.w $9c5ac
pub fn i_09c5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x9C5AC; return 10; }
    8
}
/// $09C5A8: bra.w $9c5ca
pub fn i_09c5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x9C5CA;
    10
}
/// $09C5AC: move.w #$19, d0
pub fn i_09c5ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C5B0: btst.b #$0, $3a(a6)
pub fn i_09c5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5B6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09C5B6: beq.w $9c5bc
pub fn i_09c5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C5BC; return 10; }
    8
}
/// $09C5BA: neg.w d0
pub fn i_09c5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5BC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $09C5BC: add.w d0, $22(a6)
pub fn i_09c5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5C0;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $09C5C0: move.w d0, $28(a6)
pub fn i_09c5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5C4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $09C5C4: addi.w #$14, $24(a6)
pub fn i_09c5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5CA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x14, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $09C5CA: lea.l $9c5d0(pc), a1
pub fn i_09c5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x9C5D0;
    16
}
/// $09C5CE: move.l a1, (a6)
pub fn i_09c5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $09C5D0: jsr $2783a.l
pub fn i_09c5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C5D6);
    cpu.pc = 0x2783A;
    16
}
/// $09C5D6: jsr $28d70.l
pub fn i_09c5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C5DC);
    cpu.pc = 0x28D70;
    16
}
/// $09C5DC: bcc.w $9c5e6
pub fn i_09c5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x9C5E6; return 10; }
    8
}
/// $09C5E0: jmp $518.l
pub fn i_09c5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5E6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $09C5E6: jsr $283d8.l
pub fn i_09c5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C5EC);
    cpu.pc = 0x283D8;
    16
}
/// $09C5EC: btst.b #$1, $5a(a6)
pub fn i_09c5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5F2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $09C5F2: beq.w $9c600
pub fn i_09c5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5F6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x9C600; return 10; }
    8
}
/// $09C5F6: move.w #$1086, d0
pub fn i_09c5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C5FA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1086;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $09C5FA: jsr $2352.l
pub fn i_09c5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C600;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C600);
    cpu.pc = 0x2352;
    16
}
// ===== JsrAbsThunk_09c600  ($09C600, 8 B, thunks/jsr_abs_thunks.c)
/// $09C600: jsr $32b36.l
pub fn i_09c600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C606;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x9C606);
    cpu.pc = 0x32B36;
    16
}
/// $09C606: rts
pub fn i_09c606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x9C608;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x09B0C6, i_09b0c6),
    (0x09B0CA, i_09b0ca),
    (0x09B0CE, i_09b0ce),
    (0x09B0D2, i_09b0d2),
    (0x09B0D4, i_09b0d4),
    (0x09B0D8, i_09b0d8),
    (0x09B0DE, i_09b0de),
    (0x09B0E2, i_09b0e2),
    (0x09B0E8, i_09b0e8),
    (0x09B0EE, i_09b0ee),
    (0x09B0F4, i_09b0f4),
    (0x09B0F8, i_09b0f8),
    (0x09B0FA, i_09b0fa),
    (0x09B0FE, i_09b0fe),
    (0x09B104, i_09b104),
    (0x09B108, i_09b108),
    (0x09B10E, i_09b10e),
    (0x09B112, i_09b112),
    (0x09B116, i_09b116),
    (0x09B11A, i_09b11a),
    (0x09B11C, i_09b11c),
    (0x09B1CA, i_09b1ca),
    (0x09B1CE, i_09b1ce),
    (0x09B1D4, i_09b1d4),
    (0x09B1D8, i_09b1d8),
    (0x09B1DE, i_09b1de),
    (0x09B1E2, i_09b1e2),
    (0x09B1E6, i_09b1e6),
    (0x09B1EA, i_09b1ea),
    (0x09B1EE, i_09b1ee),
    (0x09B1F2, i_09b1f2),
    (0x09B1F6, i_09b1f6),
    (0x09B1FA, i_09b1fa),
    (0x09B1FE, i_09b1fe),
    (0x09B204, i_09b204),
    (0x09B208, i_09b208),
    (0x09B20E, i_09b20e),
    (0x09B214, i_09b214),
    (0x09B21A, i_09b21a),
    (0x09B220, i_09b220),
    (0x09B226, i_09b226),
    (0x09B22A, i_09b22a),
    (0x09B22C, i_09b22c),
    (0x09B230, i_09b230),
    (0x09B236, i_09b236),
    (0x09B23A, i_09b23a),
    (0x09B240, i_09b240),
    (0x09B244, i_09b244),
    (0x09B248, i_09b248),
    (0x09B24A, i_09b24a),
    (0x09B24E, i_09b24e),
    (0x09B254, i_09b254),
    (0x09B25A, i_09b25a),
    (0x09B25E, i_09b25e),
    (0x09B264, i_09b264),
    (0x09B26A, i_09b26a),
    (0x09B270, i_09b270),
    (0x09B274, i_09b274),
    (0x09B276, i_09b276),
    (0x09B27A, i_09b27a),
    (0x09B280, i_09b280),
    (0x09B284, i_09b284),
    (0x09B288, i_09b288),
    (0x09B28A, i_09b28a),
    (0x09B28E, i_09b28e),
    (0x09B292, i_09b292),
    (0x09B298, i_09b298),
    (0x09B29E, i_09b29e),
    (0x09B2A2, i_09b2a2),
    (0x09B2A6, i_09b2a6),
    (0x09B2AC, i_09b2ac),
    (0x09B2B0, i_09b2b0),
    (0x09B2B2, i_09b2b2),
    (0x09B2B4, i_09b2b4),
    (0x09B2B6, i_09b2b6),
    (0x09B2B8, i_09b2b8),
    (0x09B2BC, i_09b2bc),
    (0x09B2C2, i_09b2c2),
    (0x09B2C6, i_09b2c6),
    (0x09B2CA, i_09b2ca),
    (0x09B2CE, i_09b2ce),
    (0x09B2D4, i_09b2d4),
    (0x09B2D8, i_09b2d8),
    (0x09B2DA, i_09b2da),
    (0x09B2DE, i_09b2de),
    (0x09B2E2, i_09b2e2),
    (0x09B2E6, i_09b2e6),
    (0x09B2EC, i_09b2ec),
    (0x09B2F2, i_09b2f2),
    (0x09B2F6, i_09b2f6),
    (0x09B2FA, i_09b2fa),
    (0x09B300, i_09b300),
    (0x09B304, i_09b304),
    (0x09B30A, i_09b30a),
    (0x09B310, i_09b310),
    (0x09B316, i_09b316),
    (0x09B31C, i_09b31c),
    (0x09B322, i_09b322),
    (0x09B326, i_09b326),
    (0x09B328, i_09b328),
    (0x09B32E, i_09b32e),
    (0x09B332, i_09b332),
    (0x09B336, i_09b336),
    (0x09B338, i_09b338),
    (0x09B33E, i_09b33e),
    (0x09B342, i_09b342),
    (0x09B348, i_09b348),
    (0x09B34C, i_09b34c),
    (0x09B350, i_09b350),
    (0x09B352, i_09b352),
    (0x09B356, i_09b356),
    (0x09B35C, i_09b35c),
    (0x09B360, i_09b360),
    (0x09B364, i_09b364),
    (0x09B368, i_09b368),
    (0x09B36C, i_09b36c),
    (0x09B372, i_09b372),
    (0x09B376, i_09b376),
    (0x09B378, i_09b378),
    (0x09B37C, i_09b37c),
    (0x09B380, i_09b380),
    (0x09B384, i_09b384),
    (0x09B388, i_09b388),
    (0x09B38A, i_09b38a),
    (0x09B390, i_09b390),
    (0x09B394, i_09b394),
    (0x09B398, i_09b398),
    (0x09B39A, i_09b39a),
    (0x09B3A0, i_09b3a0),
    (0x09B3A4, i_09b3a4),
    (0x09B3AA, i_09b3aa),
    (0x09B3AE, i_09b3ae),
    (0x09B3B2, i_09b3b2),
    (0x09B3B4, i_09b3b4),
    (0x09B3B8, i_09b3b8),
    (0x09B3BC, i_09b3bc),
    (0x09B3C0, i_09b3c0),
    (0x09B3C2, i_09b3c2),
    (0x09B3C6, i_09b3c6),
    (0x09B3CA, i_09b3ca),
    (0x09B3D0, i_09b3d0),
    (0x09B3D8, i_09b3d8),
    (0x09B3DC, i_09b3dc),
    (0x09B3E2, i_09b3e2),
    (0x09B3E8, i_09b3e8),
    (0x09B3EE, i_09b3ee),
    (0x09B3F4, i_09b3f4),
    (0x09B3FA, i_09b3fa),
    (0x09B3FE, i_09b3fe),
    (0x09B400, i_09b400),
    (0x09B404, i_09b404),
    (0x09B408, i_09b408),
    (0x09B40E, i_09b40e),
    (0x09B414, i_09b414),
    (0x09B41A, i_09b41a),
    (0x09B420, i_09b420),
    (0x09B424, i_09b424),
    (0x09B42A, i_09b42a),
    (0x09B430, i_09b430),
    (0x09B434, i_09b434),
    (0x09B438, i_09b438),
    (0x09B43A, i_09b43a),
    (0x09B43E, i_09b43e),
    (0x09B442, i_09b442),
    (0x09B448, i_09b448),
    (0x09B44E, i_09b44e),
    (0x09B454, i_09b454),
    (0x09B458, i_09b458),
    (0x09B45A, i_09b45a),
    (0x09B460, i_09b460),
    (0x09B466, i_09b466),
    (0x09B46A, i_09b46a),
    (0x09B46E, i_09b46e),
    (0x09B470, i_09b470),
    (0x09B474, i_09b474),
    (0x09B47A, i_09b47a),
    (0x09B47C, i_09b47c),
    (0x09B482, i_09b482),
    (0x09B51E, i_09b51e),
    (0x09B524, i_09b524),
    (0x09B528, i_09b528),
    (0x09B52E, i_09b52e),
    (0x09B532, i_09b532),
    (0x09B536, i_09b536),
    (0x09B53C, i_09b53c),
    (0x09B540, i_09b540),
    (0x09B542, i_09b542),
    (0x09B546, i_09b546),
    (0x09B54A, i_09b54a),
    (0x09B550, i_09b550),
    (0x09B554, i_09b554),
    (0x09B55A, i_09b55a),
    (0x09B55E, i_09b55e),
    (0x09B562, i_09b562),
    (0x09B568, i_09b568),
    (0x09B56C, i_09b56c),
    (0x09B570, i_09b570),
    (0x09B574, i_09b574),
    (0x09B578, i_09b578),
    (0x09B57C, i_09b57c),
    (0x09B580, i_09b580),
    (0x09B582, i_09b582),
    (0x09B586, i_09b586),
    (0x09B58C, i_09b58c),
    (0x09B590, i_09b590),
    (0x09B594, i_09b594),
    (0x09B598, i_09b598),
    (0x09B59C, i_09b59c),
    (0x09B5A0, i_09b5a0),
    (0x09B5A6, i_09b5a6),
    (0x09B5AC, i_09b5ac),
    (0x09B5B2, i_09b5b2),
    (0x09B5B6, i_09b5b6),
    (0x09B5BA, i_09b5ba),
    (0x09B5BC, i_09b5bc),
    (0x09B5BE, i_09b5be),
    (0x09B5C2, i_09b5c2),
    (0x09B5C6, i_09b5c6),
    (0x09B5CA, i_09b5ca),
    (0x09B5CE, i_09b5ce),
    (0x09B5D2, i_09b5d2),
    (0x09B5D4, i_09b5d4),
    (0x09B5D8, i_09b5d8),
    (0x09B5DC, i_09b5dc),
    (0x09B5E2, i_09b5e2),
    (0x09B5E4, i_09b5e4),
    (0x09B5EA, i_09b5ea),
    (0x09B5F0, i_09b5f0),
    (0x09B5F4, i_09b5f4),
    (0x09B5F6, i_09b5f6),
    (0x09B5FC, i_09b5fc),
    (0x09B602, i_09b602),
    (0x09B606, i_09b606),
    (0x09B60A, i_09b60a),
    (0x09B60E, i_09b60e),
    (0x09B612, i_09b612),
    (0x09B618, i_09b618),
    (0x09B61C, i_09b61c),
    (0x09B620, i_09b620),
    (0x09B624, i_09b624),
    (0x09B62A, i_09b62a),
    (0x09B630, i_09b630),
    (0x09B634, i_09b634),
    (0x09B638, i_09b638),
    (0x09B63A, i_09b63a),
    (0x09B63E, i_09b63e),
    (0x09B644, i_09b644),
    (0x09B648, i_09b648),
    (0x09B64C, i_09b64c),
    (0x09B64E, i_09b64e),
    (0x09B654, i_09b654),
    (0x09B658, i_09b658),
    (0x09B65E, i_09b65e),
    (0x09B662, i_09b662),
    (0x09B666, i_09b666),
    (0x09B668, i_09b668),
    (0x09B66A, i_09b66a),
    (0x09B66E, i_09b66e),
    (0x09B672, i_09b672),
    (0x09B676, i_09b676),
    (0x09B67A, i_09b67a),
    (0x09B680, i_09b680),
    (0x09B684, i_09b684),
    (0x09B68A, i_09b68a),
    (0x09B68E, i_09b68e),
    (0x09B692, i_09b692),
    (0x09B698, i_09b698),
    (0x09B69C, i_09b69c),
    (0x09B6A2, i_09b6a2),
    (0x09B6A6, i_09b6a6),
    (0x09B6AC, i_09b6ac),
    (0x09B6B2, i_09b6b2),
    (0x09B6B8, i_09b6b8),
    (0x09B6BC, i_09b6bc),
    (0x09B6C2, i_09b6c2),
    (0x09B6C6, i_09b6c6),
    (0x09B6CC, i_09b6cc),
    (0x09B6D2, i_09b6d2),
    (0x09B6D8, i_09b6d8),
    (0x09B6DC, i_09b6dc),
    (0x09B6E0, i_09b6e0),
    (0x09B6E6, i_09b6e6),
    (0x09B6E8, i_09b6e8),
    (0x09B6EC, i_09b6ec),
    (0x09B6F2, i_09b6f2),
    (0x09B6F6, i_09b6f6),
    (0x09B6FC, i_09b6fc),
    (0x09B700, i_09b700),
    (0x09B702, i_09b702),
    (0x09B708, i_09b708),
    (0x09B70C, i_09b70c),
    (0x09B712, i_09b712),
    (0x09B716, i_09b716),
    (0x09B71C, i_09b71c),
    (0x09B720, i_09b720),
    (0x09B724, i_09b724),
    (0x09B728, i_09b728),
    (0x09B72C, i_09b72c),
    (0x09B730, i_09b730),
    (0x09B734, i_09b734),
    (0x09B738, i_09b738),
    (0x09B73C, i_09b73c),
    (0x09B73E, i_09b73e),
    (0x09B742, i_09b742),
    (0x09B748, i_09b748),
    (0x09B74E, i_09b74e),
    (0x09B752, i_09b752),
    (0x09B756, i_09b756),
    (0x09B758, i_09b758),
    (0x09B75A, i_09b75a),
    (0x09B760, i_09b760),
    (0x09B766, i_09b766),
    (0x09B76C, i_09b76c),
    (0x09B770, i_09b770),
    (0x09B772, i_09b772),
    (0x09B778, i_09b778),
    (0x09B77C, i_09b77c),
    (0x09B782, i_09b782),
    (0x09B786, i_09b786),
    (0x09B78C, i_09b78c),
    (0x09B790, i_09b790),
    (0x09B794, i_09b794),
    (0x09B796, i_09b796),
    (0x09B79A, i_09b79a),
    (0x09B7A0, i_09b7a0),
    (0x09B7A4, i_09b7a4),
    (0x09B7A8, i_09b7a8),
    (0x09B7AA, i_09b7aa),
    (0x09B7B0, i_09b7b0),
    (0x09B7B4, i_09b7b4),
    (0x09B7B8, i_09b7b8),
    (0x09B7BA, i_09b7ba),
    (0x09B7BC, i_09b7bc),
    (0x09B7C0, i_09b7c0),
    (0x09B7C2, i_09b7c2),
    (0x09B7C8, i_09b7c8),
    (0x09B7CC, i_09b7cc),
    (0x09B7D2, i_09b7d2),
    (0x09B7D6, i_09b7d6),
    (0x09B7DC, i_09b7dc),
    (0x09B7E0, i_09b7e0),
    (0x09B7E4, i_09b7e4),
    (0x09B7E6, i_09b7e6),
    (0x09B7EC, i_09b7ec),
    (0x09B7F0, i_09b7f0),
    (0x09B7F4, i_09b7f4),
    (0x09B7F6, i_09b7f6),
    (0x09B7F8, i_09b7f8),
    (0x09B7FC, i_09b7fc),
    (0x09B7FE, i_09b7fe),
    (0x09B804, i_09b804),
    (0x09B80C, i_09b80c),
    (0x09B810, i_09b810),
    (0x09B814, i_09b814),
    (0x09B81A, i_09b81a),
    (0x09B820, i_09b820),
    (0x09B824, i_09b824),
    (0x09B828, i_09b828),
    (0x09B82A, i_09b82a),
    (0x09B82C, i_09b82c),
    (0x09B832, i_09b832),
    (0x09B838, i_09b838),
    (0x09B83C, i_09b83c),
    (0x09B83E, i_09b83e),
    (0x09B844, i_09b844),
    (0x09B84C, i_09b84c),
    (0x09B850, i_09b850),
    (0x09B856, i_09b856),
    (0x09B85C, i_09b85c),
    (0x09B864, i_09b864),
    (0x09B868, i_09b868),
    (0x09B870, i_09b870),
    (0x09B872, i_09b872),
    (0x09B878, i_09b878),
    (0x09B87C, i_09b87c),
    (0x09B880, i_09b880),
    (0x09B882, i_09b882),
    (0x09B886, i_09b886),
    (0x09B888, i_09b888),
    (0x09B88A, i_09b88a),
    (0x09B88E, i_09b88e),
    (0x09B892, i_09b892),
    (0x09B896, i_09b896),
    (0x09B89A, i_09b89a),
    (0x09B89E, i_09b89e),
    (0x09B8A2, i_09b8a2),
    (0x09B8A6, i_09b8a6),
    (0x09B8AA, i_09b8aa),
    (0x09B8AE, i_09b8ae),
    (0x09B8B2, i_09b8b2),
    (0x09B8B8, i_09b8b8),
    (0x09B8BE, i_09b8be),
    (0x09B8C0, i_09b8c0),
    (0x09B8C6, i_09b8c6),
    (0x09B8CA, i_09b8ca),
    (0x09B8CE, i_09b8ce),
    (0x09B8D4, i_09b8d4),
    (0x09B8D8, i_09b8d8),
    (0x09B8DE, i_09b8de),
    (0x09B8E2, i_09b8e2),
    (0x09B8E8, i_09b8e8),
    (0x09B8EC, i_09b8ec),
    (0x09B8EE, i_09b8ee),
    (0x09B8F2, i_09b8f2),
    (0x09B8F6, i_09b8f6),
    (0x09B8FA, i_09b8fa),
    (0x09B8FE, i_09b8fe),
    (0x09B902, i_09b902),
    (0x09B908, i_09b908),
    (0x09B90A, i_09b90a),
    (0x09B90E, i_09b90e),
    (0x09B912, i_09b912),
    (0x09B916, i_09b916),
    (0x09B91A, i_09b91a),
    (0x09B91E, i_09b91e),
    (0x09B920, i_09b920),
    (0x09B924, i_09b924),
    (0x09B928, i_09b928),
    (0x09B92C, i_09b92c),
    (0x09B930, i_09b930),
    (0x09B936, i_09b936),
    (0x09B93A, i_09b93a),
    (0x09B940, i_09b940),
    (0x09B946, i_09b946),
    (0x09B94C, i_09b94c),
    (0x09B950, i_09b950),
    (0x09B954, i_09b954),
    (0x09B958, i_09b958),
    (0x09B95A, i_09b95a),
    (0x09B95E, i_09b95e),
    (0x09B962, i_09b962),
    (0x09B966, i_09b966),
    (0x09B96A, i_09b96a),
    (0x09B972, i_09b972),
    (0x09B97A, i_09b97a),
    (0x09B97E, i_09b97e),
    (0x09B984, i_09b984),
    (0x09B986, i_09b986),
    (0x09B98A, i_09b98a),
    (0x09B98C, i_09b98c),
    (0x09B98E, i_09b98e),
    (0x09B990, i_09b990),
    (0x09B996, i_09b996),
    (0x09B99E, i_09b99e),
    (0x09B9A2, i_09b9a2),
    (0x09B9A6, i_09b9a6),
    (0x09B9AA, i_09b9aa),
    (0x09B9AE, i_09b9ae),
    (0x09B9B2, i_09b9b2),
    (0x09B9B4, i_09b9b4),
    (0x09B9B8, i_09b9b8),
    (0x09B9BA, i_09b9ba),
    (0x09B9BC, i_09b9bc),
    (0x09B9BE, i_09b9be),
    (0x09B9C4, i_09b9c4),
    (0x09B9C8, i_09b9c8),
    (0x09B9CA, i_09b9ca),
    (0x09B9D0, i_09b9d0),
    (0x09B9D4, i_09b9d4),
    (0x09B9D6, i_09b9d6),
    (0x09B9D8, i_09b9d8),
    (0x09B9DC, i_09b9dc),
    (0x09B9E2, i_09b9e2),
    (0x09B9E8, i_09b9e8),
    (0x09B9EA, i_09b9ea),
    (0x09B9EE, i_09b9ee),
    (0x09B9F4, i_09b9f4),
    (0x09B9F6, i_09b9f6),
    (0x09B9F8, i_09b9f8),
    (0x09B9FA, i_09b9fa),
    (0x09BA00, i_09ba00),
    (0x09BA04, i_09ba04),
    (0x09BA06, i_09ba06),
    (0x09BA0C, i_09ba0c),
    (0x09BA10, i_09ba10),
    (0x09BA12, i_09ba12),
    (0x09BA16, i_09ba16),
    (0x09BA1C, i_09ba1c),
    (0x09BA22, i_09ba22),
    (0x09BA24, i_09ba24),
    (0x09BA28, i_09ba28),
    (0x09BA2E, i_09ba2e),
    (0x09BA32, i_09ba32),
    (0x09BA34, i_09ba34),
    (0x09BA38, i_09ba38),
    (0x09BA3E, i_09ba3e),
    (0x09BA44, i_09ba44),
    (0x09BA4A, i_09ba4a),
    (0x09BA50, i_09ba50),
    (0x09BA54, i_09ba54),
    (0x09BA56, i_09ba56),
    (0x09BA5A, i_09ba5a),
    (0x09BA60, i_09ba60),
    (0x09BA66, i_09ba66),
    (0x09BA6C, i_09ba6c),
    (0x09BA72, i_09ba72),
    (0x09BA76, i_09ba76),
    (0x09BA78, i_09ba78),
    (0x09BA7C, i_09ba7c),
    (0x09BA82, i_09ba82),
    (0x09BA88, i_09ba88),
    (0x09BA8E, i_09ba8e),
    (0x09BA94, i_09ba94),
    (0x09BA96, i_09ba96),
    (0x09BA9A, i_09ba9a),
    (0x09BA9E, i_09ba9e),
    (0x09BAA4, i_09baa4),
    (0x09BAAA, i_09baaa),
    (0x09BAAE, i_09baae),
    (0x09BAB2, i_09bab2),
    (0x09BAB6, i_09bab6),
    (0x09BABA, i_09baba),
    (0x09BABE, i_09babe),
    (0x09BAC2, i_09bac2),
    (0x09BAC6, i_09bac6),
    (0x09BACA, i_09baca),
    (0x09BACC, i_09bacc),
    (0x09BAD0, i_09bad0),
    (0x09BAD2, i_09bad2),
    (0x09BAD6, i_09bad6),
    (0x09BADA, i_09bada),
    (0x09BAE0, i_09bae0),
    (0x09BAE4, i_09bae4),
    (0x09BAE8, i_09bae8),
    (0x09BAEE, i_09baee),
    (0x09BAF4, i_09baf4),
    (0x09BAFA, i_09bafa),
    (0x09BAFE, i_09bafe),
    (0x09BB02, i_09bb02),
    (0x09BB06, i_09bb06),
    (0x09BB0A, i_09bb0a),
    (0x09BB0E, i_09bb0e),
    (0x09BB12, i_09bb12),
    (0x09BB16, i_09bb16),
    (0x09BB1A, i_09bb1a),
    (0x09BB1C, i_09bb1c),
    (0x09BB20, i_09bb20),
    (0x09BB22, i_09bb22),
    (0x09BB26, i_09bb26),
    (0x09BB2E, i_09bb2e),
    (0x09BB32, i_09bb32),
    (0x09BB36, i_09bb36),
    (0x09BB3A, i_09bb3a),
    (0x09BB3C, i_09bb3c),
    (0x09BB40, i_09bb40),
    (0x09BB44, i_09bb44),
    (0x09BB48, i_09bb48),
    (0x09BB4A, i_09bb4a),
    (0x09BB4C, i_09bb4c),
    (0x09BB50, i_09bb50),
    (0x09BB54, i_09bb54),
    (0x09BB56, i_09bb56),
    (0x09BB5C, i_09bb5c),
    (0x09BB60, i_09bb60),
    (0x09BB66, i_09bb66),
    (0x09BB6A, i_09bb6a),
    (0x09BB70, i_09bb70),
    (0x09BB76, i_09bb76),
    (0x09BB7C, i_09bb7c),
    (0x09BB80, i_09bb80),
    (0x09BB82, i_09bb82),
    (0x09BB88, i_09bb88),
    (0x09BB8E, i_09bb8e),
    (0x09BB92, i_09bb92),
    (0x09BDAA, i_09bdaa),
    (0x09BDAE, i_09bdae),
    (0x09BDB2, i_09bdb2),
    (0x09BDB6, i_09bdb6),
    (0x09BDBA, i_09bdba),
    (0x09BDBE, i_09bdbe),
    (0x09BDC0, i_09bdc0),
    (0x09BDC4, i_09bdc4),
    (0x09BDC6, i_09bdc6),
    (0x09BDCA, i_09bdca),
    (0x09BDD0, i_09bdd0),
    (0x09BDD4, i_09bdd4),
    (0x09BDD8, i_09bdd8),
    (0x09BDDE, i_09bdde),
    (0x09BDE2, i_09bde2),
    (0x09BDE6, i_09bde6),
    (0x09BDEC, i_09bdec),
    (0x09BDF0, i_09bdf0),
    (0x09BDF6, i_09bdf6),
    (0x09BDFA, i_09bdfa),
    (0x09BDFC, i_09bdfc),
    (0x09BE00, i_09be00),
    (0x09BE04, i_09be04),
    (0x09BE08, i_09be08),
    (0x09BE0E, i_09be0e),
    (0x09BE12, i_09be12),
    (0x09BE14, i_09be14),
    (0x09BE18, i_09be18),
    (0x09BE1C, i_09be1c),
    (0x09BE1E, i_09be1e),
    (0x09BE24, i_09be24),
    (0x09BE2A, i_09be2a),
    (0x09BE2E, i_09be2e),
    (0x09BE34, i_09be34),
    (0x09BE3A, i_09be3a),
    (0x09BE3E, i_09be3e),
    (0x09BE44, i_09be44),
    (0x09BE4A, i_09be4a),
    (0x09BE4E, i_09be4e),
    (0x09BE54, i_09be54),
    (0x09BE58, i_09be58),
    (0x09BE5E, i_09be5e),
    (0x09BE64, i_09be64),
    (0x09BE6A, i_09be6a),
    (0x09BE6E, i_09be6e),
    (0x09BE70, i_09be70),
    (0x09BE76, i_09be76),
    (0x09BE7C, i_09be7c),
    (0x09BE80, i_09be80),
    (0x09BE86, i_09be86),
    (0x09BE8C, i_09be8c),
    (0x09BE92, i_09be92),
    (0x09BE96, i_09be96),
    (0x09BE9C, i_09be9c),
    (0x09BE9E, i_09be9e),
    (0x09BEA6, i_09bea6),
    (0x09BEAC, i_09beac),
    (0x09BEB0, i_09beb0),
    (0x09BEB6, i_09beb6),
    (0x09BEBA, i_09beba),
    (0x09BEC0, i_09bec0),
    (0x09BEC2, i_09bec2),
    (0x09BEC6, i_09bec6),
    (0x09BECC, i_09becc),
    (0x09BED2, i_09bed2),
    (0x09BED8, i_09bed8),
    (0x09BEDC, i_09bedc),
    (0x09BEE2, i_09bee2),
    (0x09BEE6, i_09bee6),
    (0x09BEEA, i_09beea),
    (0x09BEEE, i_09beee),
    (0x09BEF4, i_09bef4),
    (0x09BEFA, i_09befa),
    (0x09BF00, i_09bf00),
    (0x09BF04, i_09bf04),
    (0x09BF0A, i_09bf0a),
    (0x09BF0E, i_09bf0e),
    (0x09BF14, i_09bf14),
    (0x09BF18, i_09bf18),
    (0x09BF1E, i_09bf1e),
    (0x09BF22, i_09bf22),
    (0x09BF26, i_09bf26),
    (0x09BF2A, i_09bf2a),
    (0x09BF30, i_09bf30),
    (0x09BF36, i_09bf36),
    (0x09BF3A, i_09bf3a),
    (0x09BF40, i_09bf40),
    (0x09BF46, i_09bf46),
    (0x09BF4C, i_09bf4c),
    (0x09BF50, i_09bf50),
    (0x09BF56, i_09bf56),
    (0x09BF5A, i_09bf5a),
    (0x09BF60, i_09bf60),
    (0x09BF64, i_09bf64),
    (0x09BF6A, i_09bf6a),
    (0x09BF6E, i_09bf6e),
    (0x09BF72, i_09bf72),
    (0x09BF76, i_09bf76),
    (0x09BF7C, i_09bf7c),
    (0x09BF80, i_09bf80),
    (0x09BF84, i_09bf84),
    (0x09BF88, i_09bf88),
    (0x09BF8A, i_09bf8a),
    (0x09BF8E, i_09bf8e),
    (0x09BF92, i_09bf92),
    (0x09BF98, i_09bf98),
    (0x09BF9C, i_09bf9c),
    (0x09BFA0, i_09bfa0),
    (0x09BFA4, i_09bfa4),
    (0x09BFA6, i_09bfa6),
    (0x09BFAC, i_09bfac),
    (0x09BFB0, i_09bfb0),
    (0x09BFB6, i_09bfb6),
    (0x09BFBA, i_09bfba),
    (0x09BFBE, i_09bfbe),
    (0x09BFC0, i_09bfc0),
    (0x09BFC4, i_09bfc4),
    (0x09BFC8, i_09bfc8),
    (0x09BFCE, i_09bfce),
    (0x09BFD2, i_09bfd2),
    (0x09BFD8, i_09bfd8),
    (0x09BFDE, i_09bfde),
    (0x09BFE4, i_09bfe4),
    (0x09BFEA, i_09bfea),
    (0x09BFF0, i_09bff0),
    (0x09BFF4, i_09bff4),
    (0x09BFF8, i_09bff8),
    (0x09BFFA, i_09bffa),
    (0x09BFFE, i_09bffe),
    (0x09C004, i_09c004),
    (0x09C00A, i_09c00a),
    (0x09C00E, i_09c00e),
    (0x09C014, i_09c014),
    (0x09C01A, i_09c01a),
    (0x09C01E, i_09c01e),
    (0x09C024, i_09c024),
    (0x09C028, i_09c028),
    (0x09C02C, i_09c02c),
    (0x09C030, i_09c030),
    (0x09C036, i_09c036),
    (0x09C03C, i_09c03c),
    (0x09C040, i_09c040),
    (0x09C046, i_09c046),
    (0x09C04C, i_09c04c),
    (0x09C050, i_09c050),
    (0x09C052, i_09c052),
    (0x09C058, i_09c058),
    (0x09C05E, i_09c05e),
    (0x09C064, i_09c064),
    (0x09C068, i_09c068),
    (0x09C06E, i_09c06e),
    (0x09C072, i_09c072),
    (0x09C076, i_09c076),
    (0x09C07A, i_09c07a),
    (0x09C07C, i_09c07c),
    (0x09C080, i_09c080),
    (0x09C084, i_09c084),
    (0x09C086, i_09c086),
    (0x09C08A, i_09c08a),
    (0x09C08E, i_09c08e),
    (0x09C092, i_09c092),
    (0x09C094, i_09c094),
    (0x09C098, i_09c098),
    (0x09C09C, i_09c09c),
    (0x09C09E, i_09c09e),
    (0x09C0A2, i_09c0a2),
    (0x09C0A4, i_09c0a4),
    (0x09C0A8, i_09c0a8),
    (0x09C0AE, i_09c0ae),
    (0x09C0B2, i_09c0b2),
    (0x09C0B8, i_09c0b8),
    (0x09C0BE, i_09c0be),
    (0x09C0C4, i_09c0c4),
    (0x09C0CA, i_09c0ca),
    (0x09C0D0, i_09c0d0),
    (0x09C0D4, i_09c0d4),
    (0x09C0D8, i_09c0d8),
    (0x09C0DC, i_09c0dc),
    (0x09C0E2, i_09c0e2),
    (0x09C0E8, i_09c0e8),
    (0x09C0EC, i_09c0ec),
    (0x09C0EE, i_09c0ee),
    (0x09C0F2, i_09c0f2),
    (0x09C0F4, i_09c0f4),
    (0x09C0F6, i_09c0f6),
    (0x09C0FA, i_09c0fa),
    (0x09C0FC, i_09c0fc),
    (0x09C0FE, i_09c0fe),
    (0x09C102, i_09c102),
    (0x09C106, i_09c106),
    (0x09C10A, i_09c10a),
    (0x09C10C, i_09c10c),
    (0x09C112, i_09c112),
    (0x09C116, i_09c116),
    (0x09C11A, i_09c11a),
    (0x09C11C, i_09c11c),
    (0x09C120, i_09c120),
    (0x09C124, i_09c124),
    (0x09C126, i_09c126),
    (0x09C12A, i_09c12a),
    (0x09C130, i_09c130),
    (0x09C134, i_09c134),
    (0x09C13A, i_09c13a),
    (0x09C13C, i_09c13c),
    (0x09C142, i_09c142),
    (0x09C146, i_09c146),
    (0x09C14C, i_09c14c),
    (0x09C150, i_09c150),
    (0x09C152, i_09c152),
    (0x09C158, i_09c158),
    (0x09C15C, i_09c15c),
    (0x09C162, i_09c162),
    (0x09C166, i_09c166),
    (0x09C16C, i_09c16c),
    (0x09C172, i_09c172),
    (0x09C178, i_09c178),
    (0x09C17E, i_09c17e),
    (0x09C184, i_09c184),
    (0x09C18A, i_09c18a),
    (0x09C18E, i_09c18e),
    (0x09C192, i_09c192),
    (0x09C196, i_09c196),
    (0x09C19A, i_09c19a),
    (0x09C19E, i_09c19e),
    (0x09C1A0, i_09c1a0),
    (0x09C1A4, i_09c1a4),
    (0x09C1A6, i_09c1a6),
    (0x09C1AA, i_09c1aa),
    (0x09C1AC, i_09c1ac),
    (0x09C1B2, i_09c1b2),
    (0x09C1B6, i_09c1b6),
    (0x09C1BC, i_09c1bc),
    (0x09C1C0, i_09c1c0),
    (0x09C1C2, i_09c1c2),
    (0x09C1C8, i_09c1c8),
    (0x09C1CC, i_09c1cc),
    (0x09C1D2, i_09c1d2),
    (0x09C1D6, i_09c1d6),
    (0x09C1DC, i_09c1dc),
    (0x09C1E2, i_09c1e2),
    (0x09C1E8, i_09c1e8),
    (0x09C1EE, i_09c1ee),
    (0x09C1F4, i_09c1f4),
    (0x09C1FA, i_09c1fa),
    (0x09C1FE, i_09c1fe),
    (0x09C202, i_09c202),
    (0x09C206, i_09c206),
    (0x09C20A, i_09c20a),
    (0x09C20C, i_09c20c),
    (0x09C210, i_09c210),
    (0x09C212, i_09c212),
    (0x09C216, i_09c216),
    (0x09C218, i_09c218),
    (0x09C21C, i_09c21c),
    (0x09C220, i_09c220),
    (0x09C224, i_09c224),
    (0x09C228, i_09c228),
    (0x09C22C, i_09c22c),
    (0x09C22E, i_09c22e),
    (0x09C232, i_09c232),
    (0x09C234, i_09c234),
    (0x09C23C, i_09c23c),
    (0x09C242, i_09c242),
    (0x09C246, i_09c246),
    (0x09C24C, i_09c24c),
    (0x09C250, i_09c250),
    (0x09C256, i_09c256),
    (0x09C25C, i_09c25c),
    (0x09C25E, i_09c25e),
    (0x09C262, i_09c262),
    (0x09C268, i_09c268),
    (0x09C26E, i_09c26e),
    (0x09C274, i_09c274),
    (0x09C278, i_09c278),
    (0x09C27E, i_09c27e),
    (0x09C282, i_09c282),
    (0x09C286, i_09c286),
    (0x09C28A, i_09c28a),
    (0x09C290, i_09c290),
    (0x09C296, i_09c296),
    (0x09C29C, i_09c29c),
    (0x09C2A0, i_09c2a0),
    (0x09C2A6, i_09c2a6),
    (0x09C2AC, i_09c2ac),
    (0x09C2B2, i_09c2b2),
    (0x09C2B8, i_09c2b8),
    (0x09C2BC, i_09c2bc),
    (0x09C2C0, i_09c2c0),
    (0x09C2C4, i_09c2c4),
    (0x09C2CA, i_09c2ca),
    (0x09C2CE, i_09c2ce),
    (0x09C2D2, i_09c2d2),
    (0x09C2D6, i_09c2d6),
    (0x09C2D8, i_09c2d8),
    (0x09C2DC, i_09c2dc),
    (0x09C2E0, i_09c2e0),
    (0x09C2E6, i_09c2e6),
    (0x09C2EA, i_09c2ea),
    (0x09C2F0, i_09c2f0),
    (0x09C2F4, i_09c2f4),
    (0x09C2FA, i_09c2fa),
    (0x09C2FE, i_09c2fe),
    (0x09C304, i_09c304),
    (0x09C308, i_09c308),
    (0x09C30A, i_09c30a),
    (0x09C30E, i_09c30e),
    (0x09C312, i_09c312),
    (0x09C316, i_09c316),
    (0x09C31A, i_09c31a),
    (0x09C31C, i_09c31c),
    (0x09C320, i_09c320),
    (0x09C324, i_09c324),
    (0x09C328, i_09c328),
    (0x09C32A, i_09c32a),
    (0x09C32E, i_09c32e),
    (0x09C332, i_09c332),
    (0x09C336, i_09c336),
    (0x09C338, i_09c338),
    (0x09C33C, i_09c33c),
    (0x09C340, i_09c340),
    (0x09C344, i_09c344),
    (0x09C346, i_09c346),
    (0x09C34A, i_09c34a),
    (0x09C350, i_09c350),
    (0x09C356, i_09c356),
    (0x09C35A, i_09c35a),
    (0x09C360, i_09c360),
    (0x09C364, i_09c364),
    (0x09C368, i_09c368),
    (0x09C36A, i_09c36a),
    (0x09C370, i_09c370),
    (0x09C374, i_09c374),
    (0x09C37A, i_09c37a),
    (0x09C380, i_09c380),
    (0x09C386, i_09c386),
    (0x09C38C, i_09c38c),
    (0x09C390, i_09c390),
    (0x09C394, i_09c394),
    (0x09C396, i_09c396),
    (0x09C39A, i_09c39a),
    (0x09C39E, i_09c39e),
    (0x09C3A2, i_09c3a2),
    (0x09C3A6, i_09c3a6),
    (0x09C3AA, i_09c3aa),
    (0x09C3B0, i_09c3b0),
    (0x09C3B6, i_09c3b6),
    (0x09C3BA, i_09c3ba),
    (0x09C3BE, i_09c3be),
    (0x09C3C4, i_09c3c4),
    (0x09C3CA, i_09c3ca),
    (0x09C3CE, i_09c3ce),
    (0x09C3D0, i_09c3d0),
    (0x09C3D6, i_09c3d6),
    (0x09C3DC, i_09c3dc),
    (0x09C3E0, i_09c3e0),
    (0x09C3E6, i_09c3e6),
    (0x09C3EC, i_09c3ec),
    (0x09C3F0, i_09c3f0),
    (0x09C3F4, i_09c3f4),
    (0x09C3FA, i_09c3fa),
    (0x09C3FE, i_09c3fe),
    (0x09C404, i_09c404),
    (0x09C40A, i_09c40a),
    (0x09C40E, i_09c40e),
    (0x09C410, i_09c410),
    (0x09C414, i_09c414),
    (0x09C418, i_09c418),
    (0x09C41A, i_09c41a),
    (0x09C41E, i_09c41e),
    (0x09C422, i_09c422),
    (0x09C426, i_09c426),
    (0x09C428, i_09c428),
    (0x09C42A, i_09c42a),
    (0x09C42C, i_09c42c),
    (0x09C42E, i_09c42e),
    (0x09C432, i_09c432),
    (0x09C436, i_09c436),
    (0x09C43A, i_09c43a),
    (0x09C43C, i_09c43c),
    (0x09C43E, i_09c43e),
    (0x09C440, i_09c440),
    (0x09C442, i_09c442),
    (0x09C446, i_09c446),
    (0x09C44C, i_09c44c),
    (0x09C452, i_09c452),
    (0x09C454, i_09c454),
    (0x09C458, i_09c458),
    (0x09C45E, i_09c45e),
    (0x09C462, i_09c462),
    (0x09C468, i_09c468),
    (0x09C46E, i_09c46e),
    (0x09C472, i_09c472),
    (0x09C474, i_09c474),
    (0x09C478, i_09c478),
    (0x09C47C, i_09c47c),
    (0x09C47E, i_09c47e),
    (0x09C482, i_09c482),
    (0x09C486, i_09c486),
    (0x09C48A, i_09c48a),
    (0x09C48C, i_09c48c),
    (0x09C48E, i_09c48e),
    (0x09C490, i_09c490),
    (0x09C492, i_09c492),
    (0x09C496, i_09c496),
    (0x09C49A, i_09c49a),
    (0x09C49E, i_09c49e),
    (0x09C4A0, i_09c4a0),
    (0x09C4A2, i_09c4a2),
    (0x09C4A4, i_09c4a4),
    (0x09C4A6, i_09c4a6),
    (0x09C4AA, i_09c4aa),
    (0x09C4B0, i_09c4b0),
    (0x09C4B6, i_09c4b6),
    (0x09C4B8, i_09c4b8),
    (0x09C4BC, i_09c4bc),
    (0x09C4C0, i_09c4c0),
    (0x09C4C4, i_09c4c4),
    (0x09C4C8, i_09c4c8),
    (0x09C4CC, i_09c4cc),
    (0x09C4CE, i_09c4ce),
    (0x09C4D2, i_09c4d2),
    (0x09C4D4, i_09c4d4),
    (0x09C4DC, i_09c4dc),
    (0x09C4E0, i_09c4e0),
    (0x09C4E6, i_09c4e6),
    (0x09C4EC, i_09c4ec),
    (0x09C4F2, i_09c4f2),
    (0x09C4F6, i_09c4f6),
    (0x09C4FC, i_09c4fc),
    (0x09C500, i_09c500),
    (0x09C504, i_09c504),
    (0x09C508, i_09c508),
    (0x09C50E, i_09c50e),
    (0x09C514, i_09c514),
    (0x09C51A, i_09c51a),
    (0x09C520, i_09c520),
    (0x09C524, i_09c524),
    (0x09C52A, i_09c52a),
    (0x09C530, i_09c530),
    (0x09C536, i_09c536),
    (0x09C53C, i_09c53c),
    (0x09C540, i_09c540),
    (0x09C544, i_09c544),
    (0x09C548, i_09c548),
    (0x09C54E, i_09c54e),
    (0x09C554, i_09c554),
    (0x09C558, i_09c558),
    (0x09C55E, i_09c55e),
    (0x09C562, i_09c562),
    (0x09C566, i_09c566),
    (0x09C568, i_09c568),
    (0x09C56C, i_09c56c),
    (0x09C570, i_09c570),
    (0x09C576, i_09c576),
    (0x09C57C, i_09c57c),
    (0x09C580, i_09c580),
    (0x09C586, i_09c586),
    (0x09C58A, i_09c58a),
    (0x09C58E, i_09c58e),
    (0x09C592, i_09c592),
    (0x09C596, i_09c596),
    (0x09C59C, i_09c59c),
    (0x09C5A0, i_09c5a0),
    (0x09C5A4, i_09c5a4),
    (0x09C5A8, i_09c5a8),
    (0x09C5AC, i_09c5ac),
    (0x09C5B0, i_09c5b0),
    (0x09C5B6, i_09c5b6),
    (0x09C5BA, i_09c5ba),
    (0x09C5BC, i_09c5bc),
    (0x09C5C0, i_09c5c0),
    (0x09C5C4, i_09c5c4),
    (0x09C5CA, i_09c5ca),
    (0x09C5CE, i_09c5ce),
    (0x09C5D0, i_09c5d0),
    (0x09C5D6, i_09c5d6),
    (0x09C5DC, i_09c5dc),
    (0x09C5E0, i_09c5e0),
    (0x09C5E6, i_09c5e6),
    (0x09C5EC, i_09c5ec),
    (0x09C5F2, i_09c5f2),
    (0x09C5F6, i_09c5f6),
    (0x09C5FA, i_09c5fa),
    (0x09C600, i_09c600),
    (0x09C606, i_09c606),
];
