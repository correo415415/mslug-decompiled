// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $0636AA: move.b #$0, d0
pub fn i_0636aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636AE;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0636AE: btst.b #$1, $98(a6)
pub fn i_0636ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636B4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0636B4: beq.w $636bc
pub fn i_0636b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x636BC; return 10; }
    8
}
/// $0636B8: move.b #$1, d0
pub fn i_0636b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636BC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0636BC: move.b d0, $72(a6)
pub fn i_0636bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636C0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0636C0: move.b $99(a6), d1
pub fn i_0636c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636C4;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0636C4: andi.w #$ff, d1
pub fn i_0636c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636C8;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0636C8: lsl.w #$4, d1
pub fn i_0636c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636CA;
    cpu.ir = 0xE949;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0636CA: move.w d1, $74(a6)
pub fn i_0636ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636CE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0636CE: lea.l $2c56a8.l, a0
pub fn i_0636ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C56A8;
    20
}
/// $0636D4: move.b $9a(a6), d0
pub fn i_0636d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0636D8: andi.w #$3, d0
pub fn i_0636d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636DC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0636DC: lsl.w #$2, d0
pub fn i_0636dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636DE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0636DE: movea.l (a0, d0.w), a0
pub fn i_0636de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636E2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0636E2: jsr $63c76(pc)
pub fn i_0636e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x636E6);
    cpu.pc = 0x63C76;
    16
}
/// $0636E6: clr.b $20(a6)
pub fn i_0636e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636EA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0636EA: move.l #$2c5356, $48(a6)
pub fn i_0636ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636F2;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C5356;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0636F2: move.l #$2c55ee, $60(a6)
pub fn i_0636f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x636FA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C55EE;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0636FA: lea.l $723d2.l, a1
pub fn i_0636fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63700;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723D2;
    20
}
/// $063700: jsr $4ae.l
pub fn i_063700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63706;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63706);
    cpu.pc = 0x4AE;
    16
}
/// $063706: jsr $5dd02.l
pub fn i_063706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6370C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6370C);
    cpu.pc = 0x5DD02;
    16
}
/// $06370C: clr.w $98(a0)
pub fn i_06370c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63710;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $063710: clr.b $90(a6)
pub fn i_063710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63714;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $063714: clr.b $91(a6)
pub fn i_063714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63718;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $063718: move.w #$10a5, d0
pub fn i_063718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6371C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06371C: jsr $2352.l
pub fn i_06371c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63722;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63722);
    cpu.pc = 0x2352;
    16
}
/// $063722: bra.w $6376e
pub fn i_063722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63726;
    cpu.ir = 0x6000;
    cpu.pc = 0x6376E;
    10
}
// ===== Mortar_Active_063726  ($063726, 348 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063726: jsr $267e2.l
pub fn i_063726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6372C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6372C);
    cpu.pc = 0x267E2;
    16
}
/// $06372C: lea.l $2c58e0.l, a0
pub fn i_06372c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63732;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C58E0;
    20
}
/// $063732: move.l #$2c58fe, d0
pub fn i_063732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63738;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2C58FE;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $063738: move.l d0, $7c(a6)
pub fn i_063738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6373C;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06373C: cmpi.b #$0, $79(a6)
pub fn i_06373c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63742;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $063742: beq.w $63748
pub fn i_063742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63746;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63748; return 10; }
    8
}
/// $063746: movea.l d0, a0
pub fn i_063746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63748;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $063748: jsr $28cd4.l
pub fn i_063748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6374E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6374E);
    cpu.pc = 0x28CD4;
    16
}
/// $06374E: lea.l $63754(pc), a1
pub fn i_06374e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63752;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63754;
    16
}
/// $063752: move.l a1, (a6)
pub fn i_063752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63754;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063754: jsr $63d6e(pc)
pub fn i_063754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63758;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63758);
    cpu.pc = 0x63D6E;
    16
}
/// $063758: jsr $28998.l
pub fn i_063758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6375E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6375E);
    cpu.pc = 0x28998;
    16
}
/// $06375E: jsr $2783a.l
pub fn i_06375e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63764);
    cpu.pc = 0x2783A;
    16
}
/// $063764: jsr $28d70.l
pub fn i_063764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6376A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6376A);
    cpu.pc = 0x28D70;
    16
}
/// $06376A: bra.w $637e8
pub fn i_06376a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6376E;
    cpu.ir = 0x6000;
    cpu.pc = 0x637E8;
    10
}
/// $06376E: move.l #$2c544e, $4c(a6)
pub fn i_06376e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63776;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C544E;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $063776: move.w #$ff40, d0
pub fn i_063776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6377A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06377A: btst.b #$0, $3a(a6)
pub fn i_06377a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63780;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063780: beq.w $63788
pub fn i_063780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63784;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63788; return 10; }
    8
}
/// $063784: move.w #$180, d0
pub fn i_063784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63788;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063788: move.w d0, $28(a6)
pub fn i_063788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6378C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06378C: lea.l $2c5828.l, a0
pub fn i_06378c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63792;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5828;
    20
}
/// $063792: move.l #$2c5856, d0
pub fn i_063792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63798;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2C5856;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $063798: move.l d0, $7c(a6)
pub fn i_063798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6379C;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06379C: cmpi.b #$0, $79(a6)
pub fn i_06379c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637A2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0637A2: beq.w $637a8
pub fn i_0637a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x637A8; return 10; }
    8
}
/// $0637A6: movea.l d0, a0
pub fn i_0637a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637A8;
    cpu.ir = 0x2040;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0637A8: jsr $28cd4.l
pub fn i_0637a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x637AE);
    cpu.pc = 0x28CD4;
    16
}
/// $0637AE: lea.l $637b4(pc), a1
pub fn i_0637ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x637B4;
    16
}
/// $0637B2: move.l a1, (a6)
pub fn i_0637b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0637B4: jsr $63d6e(pc)
pub fn i_0637b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x637B8);
    cpu.pc = 0x63D6E;
    16
}
/// $0637B8: jsr $28998.l
pub fn i_0637b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x637BE);
    cpu.pc = 0x28998;
    16
}
/// $0637BE: jsr $27cee.l
pub fn i_0637be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x637C4);
    cpu.pc = 0x27CEE;
    16
}
/// $0637C4: jsr $28d70.l
pub fn i_0637c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x637CA);
    cpu.pc = 0x28D70;
    16
}
/// $0637CA: jsr $63c44(pc)
pub fn i_0637ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637CE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x637CE);
    cpu.pc = 0x63C44;
    16
}
/// $0637CE: bcc.w $637d8
pub fn i_0637ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x637D8; return 10; }
    8
}
/// $0637D2: lea.l $63726(pc), a1
pub fn i_0637d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63726;
    16
}
/// $0637D6: move.l a1, (a6)
pub fn i_0637d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0637D8: jsr $283ca.l
pub fn i_0637d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x637DE);
    cpu.pc = 0x283CA;
    16
}
/// $0637DE: jsr $283d8.l
pub fn i_0637de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x637E4);
    cpu.pc = 0x283D8;
    16
}
/// $0637E4: bra.w $637e8
pub fn i_0637e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637E8;
    cpu.ir = 0x6000;
    cpu.pc = 0x637E8;
    10
}
/// $0637E8: lea.l $5e766.l, a0
pub fn i_0637e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0637EE: jsr $5e770.l
pub fn i_0637ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x637F4);
    cpu.pc = 0x5E770;
    16
}
/// $0637F4: jsr $2870a.l
pub fn i_0637f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x637FA);
    cpu.pc = 0x2870A;
    16
}
/// $0637FA: bcc.w $6383c
pub fn i_0637fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x637FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6383C; return 10; }
    8
}
/// $0637FE: bclr.b #$3, $13(a6)
pub fn i_0637fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63804;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $063804: cmpi.b #$4, $58(a6)
pub fn i_063804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6380A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $06380A: bne.w $63812
pub fn i_06380a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6380E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x63812; return 10; }
    8
}
/// $06380E: addq.b #$1, $90(a6)
pub fn i_06380e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63812;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $063812: cmpi.b #$0, $79(a6)
pub fn i_063812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63818;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $063818: bne.w $6383c
pub fn i_063818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6381C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6383C; return 10; }
    8
}
/// $06381C: move.w $7a(a6), d0
pub fn i_06381c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63820;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063820: cmp.w $66(a6), d0
pub fn i_063820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63824;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $063824: blt.w $6383c
pub fn i_063824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63828;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6383C; return 10; }
    8
}
/// $063828: move.b #$1, $79(a6)
pub fn i_063828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6382E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06382E: movea.l $7c(a6), a0
pub fn i_06382e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63832;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $063832: jsr $28cd4.l
pub fn i_063832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63838);
    cpu.pc = 0x28CD4;
    16
}
/// $063838: jsr $63df2(pc)
pub fn i_063838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6383C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6383C);
    cpu.pc = 0x63DF2;
    16
}
/// $06383C: cmpi.b #$1, $90(a6)
pub fn i_06383c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63842;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $063842: blt.w $6384c
pub fn i_063842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63846;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6384C; return 10; }
    8
}
/// $063846: lea.l $6388a(pc), a1
pub fn i_063846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6384A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6388A;
    16
}
/// $06384A: move.l a1, (a6)
pub fn i_06384a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6384C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06384C: jsr $28758.l
pub fn i_06384c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63852);
    cpu.pc = 0x28758;
    16
}
/// $063852: bcc.w $6385c
pub fn i_063852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63856;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6385C; return 10; }
    8
}
/// $063856: lea.l $6388a(pc), a1
pub fn i_063856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6385A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6388A;
    16
}
/// $06385A: move.l a1, (a6)
pub fn i_06385a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6385C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06385C: cmpi.w #$ff80, $22(a6)
pub fn i_06385c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63862;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $063862: bgt.w $6386c
pub fn i_063862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63866;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6386C; return 10; }
    8
}
/// $063866: lea.l $63942(pc), a1
pub fn i_063866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6386A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63942;
    16
}
/// $06386A: move.l a1, (a6)
pub fn i_06386a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6386C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06386C: movea.l #$ffffffff, a0
pub fn i_06386c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63872;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $063872: lea.l $2c55e6.l, a0
pub fn i_063872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63878;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C55E6;
    20
}
/// $063878: jsr $5dd5c.l
pub fn i_063878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6387E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6387E);
    cpu.pc = 0x5DD5C;
    16
}
/// $06387E: bcc.w $63888
pub fn i_06387e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63882;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63888; return 10; }
    8
}
// ===== SetTaskHandler_063882  ($063882, 8 B, kernel/task_handlers.c)
/// $063882: lea.l $63942(pc), a1
pub fn i_063882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63886;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63942;
    16
}
/// $063886: move.l a1, (a6)
pub fn i_063886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63888;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063888: rts
pub fn i_063888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6388A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_Die_06388a  ($06388A, 114 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06388A: bset.b #$0, $13(a6)
pub fn i_06388a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63890;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $063890: move.w #$10a5, d0
pub fn i_063890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63894;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063894: jsr $2222.l
pub fn i_063894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6389A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6389A);
    cpu.pc = 0x2222;
    16
}
/// $06389A: bclr.b #$1, $12(a6)
pub fn i_06389a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638A0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0638A0: jsr $267e2.l
pub fn i_0638a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x638A6);
    cpu.pc = 0x267E2;
    16
}
/// $0638A6: lea.l $2c5918.l, a0
pub fn i_0638a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5918;
    20
}
/// $0638AC: jsr $28cd4.l
pub fn i_0638ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x638B2);
    cpu.pc = 0x28CD4;
    16
}
/// $0638B2: lea.l $638b8(pc), a1
pub fn i_0638b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x638B8;
    16
}
/// $0638B6: move.l a1, (a6)
pub fn i_0638b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0638B8: jsr $2783a.l
pub fn i_0638b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x638BE);
    cpu.pc = 0x2783A;
    16
}
/// $0638BE: jsr $28d70.l
pub fn i_0638be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x638C4);
    cpu.pc = 0x28D70;
    16
}
/// $0638C4: bcc.w $638d6
pub fn i_0638c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x638D6; return 10; }
    8
}
/// $0638C8: tst.b $9b(a6)
pub fn i_0638c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638CC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0638CC: beq.w $638d6
pub fn i_0638cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638D0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x638D6; return 10; }
    8
}
/// $0638D0: lea.l $63904(pc), a1
pub fn i_0638d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63904;
    16
}
/// $0638D4: move.l a1, (a6)
pub fn i_0638d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0638D6: cmpi.w #$ff80, $22(a6)
pub fn i_0638d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638DC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $0638DC: bgt.w $638e6
pub fn i_0638dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638E0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x638E6; return 10; }
    8
}
/// $0638E0: lea.l $63942(pc), a1
pub fn i_0638e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63942;
    16
}
/// $0638E4: move.l a1, (a6)
pub fn i_0638e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0638E6: movea.l #$ffffffff, a0
pub fn i_0638e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638EC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0638EC: lea.l $2c55e6.l, a0
pub fn i_0638ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C55E6;
    20
}
/// $0638F2: jsr $5dd5c.l
pub fn i_0638f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x638F8);
    cpu.pc = 0x5DD5C;
    16
}
/// $0638F8: bcc.w $63902
pub fn i_0638f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x638FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63902; return 10; }
    8
}
// ===== SetTaskHandler_0638fc  ($0638FC, 8 B, kernel/task_handlers.c)
/// $0638FC: lea.l $63942(pc), a1
pub fn i_0638fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63900;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63942;
    16
}
/// $063900: move.l a1, (a6)
pub fn i_063900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63902;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063902: rts
pub fn i_063902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63904;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_Wreck_063904  ($063904, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063904: move.w #$102f, d0
pub fn i_063904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63908;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063908: jsr $2352.l
pub fn i_063908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6390E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6390E);
    cpu.pc = 0x2352;
    16
}
/// $06390E: lea.l $77fd6.l, a1
pub fn i_06390e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63914;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $063914: jsr $4ae.l
pub fn i_063914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6391A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6391A);
    cpu.pc = 0x4AE;
    16
}
/// $06391A: jsr $5dd02.l
pub fn i_06391a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63920;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63920);
    cpu.pc = 0x5DD02;
    16
}
/// $063920: addi.w #$ffe0, $22(a0)
pub fn i_063920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63926;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $063926: lea.l $77fd6.l, a1
pub fn i_063926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6392C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06392C: jsr $4ae.l
pub fn i_06392c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63932);
    cpu.pc = 0x4AE;
    16
}
/// $063932: jsr $5dd02.l
pub fn i_063932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63938;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63938);
    cpu.pc = 0x5DD02;
    16
}
/// $063938: addi.w #$20, $22(a0)
pub fn i_063938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6393E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06393E: jsr $63d82(pc)
pub fn i_06393e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63942;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63942);
    cpu.pc = 0x63D82;
    16
}
// ===== JmpScheduler_063942  ($063942, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063942: jmp $518.l
pub fn i_063942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63948;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_063948  ($063948, 2 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063948: rts
pub fn i_063948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6394A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06394a  ($06394A, 8 B, kernel/jmp_to_scheduler.c)
/// $06394A: jmp $518.l
pub fn i_06394a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63950;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Jsr5B6ThenJmpScheduler_063952  ($063952, 14 B, kernel/jsr5b6_jmp518.c)
/// $063952: jsr $5b6.l
pub fn i_063952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63958;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63958);
    cpu.pc = 0x5B6;
    16
}
/// $063958: jmp $518.l
pub fn i_063958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6395E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Mortar_ToJump6A7D6_063960  ($063960, 48 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063960: movea.l $c(a6), a0
pub fn i_063960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63964;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $063964: move.w #$120, $98(a6)
pub fn i_063964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6396A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06396A: move.w $80(a6), d0
pub fn i_06396a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6396E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06396E: move.w $82(a6), d1
pub fn i_06396e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63972;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063972: move.b d0, $9a(a6)
pub fn i_063972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63976;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $063976: move.b d1, $9b(a6)
pub fn i_063976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6397A;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06397A: move.b #$ff, $9c(a6)
pub fn i_06397a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63980;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $063980: clr.b $9e(a6)
pub fn i_063980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63984;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $063984: move.b $72(a0), $9d(a6)
pub fn i_063984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6398A;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9D);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06398A: jmp $6a7d6.l
pub fn i_06398a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63990;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6A7D6;
    8
}
// ===== Mortar_Explode_063990  ($063990, 36 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063990: move.w #$4000, d0
pub fn i_063990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63994;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063994: jsr $28134.l
pub fn i_063994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6399A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6399A);
    cpu.pc = 0x28134;
    16
}
/// $06399A: andi.w #$ffe3, $38(a6)
pub fn i_06399a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639A0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0639A0: ori.w #$0, $38(a6)
pub fn i_0639a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639A6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0639A6: move.l #$ffffffff, $48(a6)
pub fn i_0639a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639AE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0639AE: jmp $77f6a.l
pub fn i_0639ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639B4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Mortar_RngDelay_0639b4  ($0639B4, 30 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0639B4: move.w #$e, d1
pub fn i_0639b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0639B8: jsr $236e.l
pub fn i_0639b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x639BE);
    cpu.pc = 0x236E;
    16
}
/// $0639BE: move.w #$ffff, $88(a6)
pub fn i_0639be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0639C4: jsr $5e9b6.l
pub fn i_0639c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x639CA);
    cpu.pc = 0x5E9B6;
    16
}
/// $0639CA: andi.w #$f, d0
pub fn i_0639ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639CE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0639CE: move.w d0, $70(a6)
pub fn i_0639ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639D2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Mortar_Shell_0639d2  ($0639D2, 74 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0639D2: move.b $86(a6), d0
pub fn i_0639d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639D6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0639D6: andi.w #$3, d0
pub fn i_0639d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639DA;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0639DA: movea.l #$2c5698, a0
pub fn i_0639da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639E0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C5698;
    cpu.a[0] = v1;
    4
}
/// $0639E0: lsl.w #$2, d0
pub fn i_0639e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639E2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0639E2: movea.l (a0, d0.w), a0
pub fn i_0639e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639E6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0639E6: cmpa.l #$ffffffff, a0
pub fn i_0639e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639EC;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0639EC: beq.w $639f6
pub fn i_0639ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x639F6; return 10; }
    8
}
/// $0639F0: jsr $28cd4.l
pub fn i_0639f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x639F6);
    cpu.pc = 0x28CD4;
    16
}
/// $0639F6: lea.l $639fc(pc), a1
pub fn i_0639f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x639FC;
    16
}
/// $0639FA: move.l a1, (a6)
pub fn i_0639fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x639FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0639FC: jsr $63cd4(pc)
pub fn i_0639fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A00;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63A00);
    cpu.pc = 0x63CD4;
    16
}
/// $063A00: jsr $63cf0(pc)
pub fn i_063a00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63A04);
    cpu.pc = 0x63CF0;
    16
}
/// $063A04: jsr $28d70.l
pub fn i_063a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63A0A);
    cpu.pc = 0x28D70;
    16
}
/// $063A0A: jsr $63cb4(pc)
pub fn i_063a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63A0E);
    cpu.pc = 0x63CB4;
    16
}
/// $063A0E: bcc.w $63a18
pub fn i_063a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63A18; return 10; }
    8
}
/// $063A12: lea.l $63a4a(pc), a1
pub fn i_063a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63A4A;
    16
}
/// $063A16: move.l a1, (a6)
pub fn i_063a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063A18: bra.w $63b38
pub fn i_063a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x63B38;
    10
}
// ===== Mortar_ShellB_063a1c  ($063A1C, 46 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063A1C: lea.l $29bf34.l, a0
pub fn i_063a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BF34;
    20
}
/// $063A22: jsr $28cd4.l
pub fn i_063a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63A28);
    cpu.pc = 0x28CD4;
    16
}
/// $063A28: lea.l $63a2e(pc), a1
pub fn i_063a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63A2E;
    16
}
/// $063A2C: move.l a1, (a6)
pub fn i_063a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063A2E: jsr $63cd4(pc)
pub fn i_063a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63A32);
    cpu.pc = 0x63CD4;
    16
}
/// $063A32: jsr $63cf0(pc)
pub fn i_063a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A36;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63A36);
    cpu.pc = 0x63CF0;
    16
}
/// $063A36: jsr $28d70.l
pub fn i_063a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63A3C);
    cpu.pc = 0x28D70;
    16
}
/// $063A3C: bcc.w $63a46
pub fn i_063a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63A46; return 10; }
    8
}
/// $063A40: lea.l $639d2(pc), a1
pub fn i_063a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x639D2;
    16
}
/// $063A44: move.l a1, (a6)
pub fn i_063a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063A46: bra.w $63b38
pub fn i_063a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A4A;
    cpu.ir = 0x6000;
    cpu.pc = 0x63B38;
    10
}
// ===== Mortar_ShellAim_063a4a  ($063A4A, 120 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063A4A: jsr $5e0d4.l
pub fn i_063a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63A50);
    cpu.pc = 0x5E0D4;
    16
}
/// $063A50: bcs.w $63a72
pub fn i_063a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A54;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x63A72; return 10; }
    8
}
/// $063A54: move.w $22(a0), d0
pub fn i_063a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A58;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063A58: clr.w d1
pub fn i_063a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A5A;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $063A5A: cmp.w $22(a6), d0
pub fn i_063a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A5E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $063A5E: blt.w $63a6c
pub fn i_063a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A62;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x63A6C; return 10; }
    8
}
/// $063A62: btst.l d1, $3a(a6)
pub fn i_063a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A66;
    cpu.ir = 0x32E;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063A66: beq.b $63a1c
pub fn i_063a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A68;
    cpu.ir = 0x67B4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63A1C; return 10; }
    8
}
/// $063A68: bra.w $63a72
pub fn i_063a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x63A72;
    10
}
/// $063A6C: btst.l d1, $3a(a6)
pub fn i_063a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A70;
    cpu.ir = 0x32E;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063A70: bne.b $63a1c
pub fn i_063a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A72;
    cpu.ir = 0x66AA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x63A1C; return 10; }
    8
}
/// $063A72: lea.l $2b8818.l, a0
pub fn i_063a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8818;
    20
}
/// $063A78: jsr $799de.l
pub fn i_063a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63A7E);
    cpu.pc = 0x799DE;
    16
}
/// $063A7E: move.w d0, $70(a6)
pub fn i_063a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A82;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063A82: lea.l $29ba70.l, a0
pub fn i_063a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BA70;
    20
}
/// $063A88: jsr $28cd4.l
pub fn i_063a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63A8E);
    cpu.pc = 0x28CD4;
    16
}
/// $063A8E: jsr $5e9b6.l
pub fn i_063a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63A94);
    cpu.pc = 0x5E9B6;
    16
}
/// $063A94: andi.b #$3, d0
pub fn i_063a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A98;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $063A98: addi.b #$14, d0
pub fn i_063a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63A9C;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x14, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $063A9C: move.b d0, $5c(a6)
pub fn i_063a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AA0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $063AA0: lea.l $63aa6(pc), a1
pub fn i_063aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63AA6;
    16
}
/// $063AA4: move.l a1, (a6)
pub fn i_063aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063AA6: jsr $63cd4(pc)
pub fn i_063aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63AAA);
    cpu.pc = 0x63CD4;
    16
}
/// $063AAA: jsr $63cf0(pc)
pub fn i_063aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63AAE);
    cpu.pc = 0x63CF0;
    16
}
/// $063AAE: jsr $28d70.l
pub fn i_063aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63AB4);
    cpu.pc = 0x28D70;
    16
}
/// $063AB4: bcc.w $63abe
pub fn i_063ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63ABE; return 10; }
    8
}
/// $063AB8: lea.l $639d2(pc), a1
pub fn i_063ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63ABC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x639D2;
    16
}
/// $063ABC: move.l a1, (a6)
pub fn i_063abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63ABE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063ABE: bra.w $63b38
pub fn i_063abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AC2;
    cpu.ir = 0x6000;
    cpu.pc = 0x63B38;
    10
}
// ===== Mortar_Crew_063ac2  ($063AC2, 110 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063AC2: jsr $4a0d4.l
pub fn i_063ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63AC8);
    cpu.pc = 0x4A0D4;
    16
}
/// $063AC8: move.w #$0, $70(a6)
pub fn i_063ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63ACE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063ACE: lea.l $4b136.l, a0
pub fn i_063ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x4B136;
    20
}
/// $063AD4: jsr $28cd4.l
pub fn i_063ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63ADA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63ADA);
    cpu.pc = 0x28CD4;
    16
}
/// $063ADA: lea.l $63ae0(pc), a1
pub fn i_063ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63ADE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63AE0;
    16
}
/// $063ADE: move.l a1, (a6)
pub fn i_063ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063AE0: jsr $63cd4(pc)
pub fn i_063ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63AE4);
    cpu.pc = 0x63CD4;
    16
}
/// $063AE4: jsr $63cf0(pc)
pub fn i_063ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63AE8);
    cpu.pc = 0x63CF0;
    16
}
/// $063AE8: addq.w #$1, $70(a6)
pub fn i_063ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AEC;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063AEC: move.w $70(a6), d0
pub fn i_063aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AF0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063AF0: cmpi.w #$a, d0
pub fn i_063af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AF4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $063AF4: blt.w $63b10
pub fn i_063af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AF8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x63B10; return 10; }
    8
}
/// $063AF8: btst.b #$0, d0
pub fn i_063af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63AFC;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $063AFC: beq.w $63b20
pub fn i_063afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63B20; return 10; }
    8
}
/// $063B00: cmpi.w #$1e, $70(a6)
pub fn i_063b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B06;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1E, Size::Word, false);
    16
}
/// $063B06: blt.w $63b10
pub fn i_063b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B0A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x63B10; return 10; }
    8
}
/// $063B0A: lea.l $63952(pc), a1
pub fn i_063b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63952;
    16
}
/// $063B0E: move.l a1, (a6)
pub fn i_063b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063B10: jsr $28d70.l
pub fn i_063b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63B16);
    cpu.pc = 0x28D70;
    16
}
/// $063B16: bcs.w $63b20
pub fn i_063b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B1A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x63B20; return 10; }
    8
}
/// $063B1A: move.w #$0, $70(a6)
pub fn i_063b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063B20: movea.l #$ffffffff, a0
pub fn i_063b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B26;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $063B26: jsr $5dd5c.l
pub fn i_063b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63B2C);
    cpu.pc = 0x5DD5C;
    16
}
/// $063B2C: bcc.w $63b36
pub fn i_063b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B30;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63B36; return 10; }
    8
}
// ===== SetTaskHandler_063b30  ($063B30, 8 B, kernel/task_handlers.c)
/// $063B30: lea.l $63952(pc), a1
pub fn i_063b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63952;
    16
}
/// $063B34: move.l a1, (a6)
pub fn i_063b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063B36: rts
pub fn i_063b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_CrewHit_063b38  ($063B38, 118 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063B38: movea.l $c(a6), a0
pub fn i_063b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B3C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $063B3C: cmpi.b #$ff, $20(a0)
pub fn i_063b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B42;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $063B42: bne.w $63b88
pub fn i_063b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x63B88; return 10; }
    8
}
/// $063B46: lea.l $2c5718.l, a0
pub fn i_063b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B4C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5718;
    20
}
/// $063B4C: move.b $86(a6), d0
pub fn i_063b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B50;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $063B50: andi.w #$3, d0
pub fn i_063b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B54;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063B54: move.w d0, d1
pub fn i_063b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B56;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063B56: add.w d1, d1
pub fn i_063b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B58;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $063B58: add.w d1, d0
pub fn i_063b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B5A;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $063B5A: add.w d0, d0
pub fn i_063b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B5C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $063B5C: move.w (a0, d0.w), $28(a6)
pub fn i_063b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B62;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    22
}
/// $063B62: btst.b #$0, $3a(a6)
pub fn i_063b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B68;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063B68: beq.w $63b70
pub fn i_063b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B6C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63B70; return 10; }
    8
}
/// $063B6C: neg.w $28(a6)
pub fn i_063b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B70;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063B70: move.w #$0, $2c(a6)
pub fn i_063b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B76;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063B76: move.w $2(a0, d0.w), $2e(a6)
pub fn i_063b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B7C;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $063B7C: move.w $4(a0, d0.w), $2a(a6)
pub fn i_063b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B82;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $063B82: lea.l $63bb6(pc), a1
pub fn i_063b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63BB6;
    16
}
/// $063B86: move.l a1, (a6)
pub fn i_063b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063B88: jsr $2870a.l
pub fn i_063b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63B8E);
    cpu.pc = 0x2870A;
    16
}
/// $063B8E: bcc.w $63b9e
pub fn i_063b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63B9E; return 10; }
    8
}
/// $063B92: lea.l $63ac2(pc), a1
pub fn i_063b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63AC2;
    16
}
/// $063B96: move.l a1, (a6)
pub fn i_063b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063B98: jsr $519be.l
pub fn i_063b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63B9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63B9E);
    cpu.pc = 0x519BE;
    16
}
/// $063B9E: movea.l #$ffffffff, a0
pub fn i_063b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BA4;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $063BA4: jsr $5dd5c.l
pub fn i_063ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63BAA);
    cpu.pc = 0x5DD5C;
    16
}
/// $063BAA: bcc.w $63bb4
pub fn i_063baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63BB4; return 10; }
    8
}
// ===== SetTaskHandler_063bae  ($063BAE, 8 B, kernel/task_handlers.c)
/// $063BAE: lea.l $63952(pc), a1
pub fn i_063bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63952;
    16
}
/// $063BB2: move.l a1, (a6)
pub fn i_063bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063BB4: rts
pub fn i_063bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_CrewToSoldier_063bb6  ($063BB6, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063BB6: jsr $4a0d4.l
pub fn i_063bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63BBC);
    cpu.pc = 0x4A0D4;
    16
}
/// $063BBC: jmp $57226.l
pub fn i_063bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BC2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x57226;
    8
}
// ===== Mortar_Child_063bc2  ($063BC2, 66 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063BC2: move.l #$2c563a, $60(a6)
pub fn i_063bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BCA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C563A;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $063BCA: lea.l $63bd0(pc), a1
pub fn i_063bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63BD0;
    16
}
/// $063BCE: move.l a1, (a6)
pub fn i_063bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063BD0: jsr $5e506.l
pub fn i_063bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63BD6);
    cpu.pc = 0x5E506;
    16
}
/// $063BD6: jsr $28998.l
pub fn i_063bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63BDC);
    cpu.pc = 0x28998;
    16
}
/// $063BDC: jsr $63d2a(pc)
pub fn i_063bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63BE0);
    cpu.pc = 0x63D2A;
    16
}
/// $063BE0: jsr $28d70.l
pub fn i_063be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63BE6);
    cpu.pc = 0x28D70;
    16
}
/// $063BE6: movea.l $c(a6), a0
pub fn i_063be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BEA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $063BEA: btst.b #$0, $13(a0)
pub fn i_063bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BF0;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063BF0: beq.w $63bfa
pub fn i_063bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BF4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63BFA; return 10; }
    8
}
/// $063BF4: lea.l $63952(pc), a1
pub fn i_063bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63952;
    16
}
/// $063BF8: move.l a1, (a6)
pub fn i_063bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63BFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063BFA: jsr $5e45a.l
pub fn i_063bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63C00);
    cpu.pc = 0x5E45A;
    16
}
/// $063C00: bcc.w $63c0a
pub fn i_063c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C04;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63C0A; return 10; }
    8
}
// ===== SetTaskHandler_063c04  ($063C04, 8 B, kernel/task_handlers.c)
/// $063C04: lea.l $63952(pc), a1
pub fn i_063c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C08;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63952;
    16
}
/// $063C08: move.l a1, (a6)
pub fn i_063c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C0A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063C0A: rts
pub fn i_063c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_Stub_063c0c  ($063C0C, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063C0C: cmpi.b #$1, $3a(a6)
pub fn i_063c0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C12;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $063C12: beq.w $63c1e
pub fn i_063c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63C1E; return 10; }
    8
}
/// $063C16: neg.w d0
pub fn i_063c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C18;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_063c18  ($063C18, 6 B, thunks/ccr_helpers.c)
/// $063C18: ori.b #$11, ccr
pub fn i_063c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C1C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $063C1C: rts
pub fn i_063c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_063c1e  ($063C1E, 6 B, thunks/ccr_helpers.c)
/// $063C1E: andi.b #$ee, ccr
pub fn i_063c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C22;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $063C22: rts
pub fn i_063c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_StubB_063c24  ($063C24, 8 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063C24: cmpi.b #$1, $72(a6)
pub fn i_063c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C2A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $063C2A: bra.b $63c12
pub fn i_063c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C2C;
    cpu.ir = 0x60E6;
    cpu.pc = 0x63C12;
    10
}
// ===== Mortar_StubC_063c2c  ($063C2C, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063C2C: move.b $3a(a6), d0
pub fn i_063c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C30;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $063C30: cmp.b $72(a6), d0
pub fn i_063c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C34;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $063C34: bne.w $63c3e
pub fn i_063c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C38;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x63C3E; return 10; }
    8
}
// ===== ClearXN_063c38  ($063C38, 6 B, thunks/ccr_helpers.c)
/// $063C38: andi.b #$ee, ccr
pub fn i_063c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C3C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $063C3C: rts
pub fn i_063c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_063c3e  ($063C3E, 6 B, thunks/ccr_helpers.c)
/// $063C3E: ori.b #$11, ccr
pub fn i_063c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C42;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $063C42: rts
pub fn i_063c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_HPStage_063c44  ($063C44, 38 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063C44: move.w $22(a6), d1
pub fn i_063c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C48;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063C48: cmpi.b #$0, $72(a6)
pub fn i_063c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C4E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $063C4E: bne.w $63c60
pub fn i_063c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C52;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x63C60; return 10; }
    8
}
/// $063C52: move.w $74(a6), d0
pub fn i_063c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C56;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063C56: cmp.w d0, d1
pub fn i_063c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C58;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $063C58: bgt.w $63c6a
pub fn i_063c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C5C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x63C6A; return 10; }
    8
}
/// $063C5C: bra.w $63c70
pub fn i_063c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C60;
    cpu.ir = 0x6000;
    cpu.pc = 0x63C70;
    10
}
/// $063C60: move.w $74(a6), d0
pub fn i_063c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063C64: cmp.w d0, d1
pub fn i_063c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C66;
    cpu.ir = 0xB240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $063C66: bge.w $63c70
pub fn i_063c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C6A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x63C70; return 10; }
    8
}
// ===== ClearXN_063c6a  ($063C6A, 6 B, thunks/ccr_helpers.c)
/// $063C6A: andi.b #$ee, ccr
pub fn i_063c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C6E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $063C6E: rts
pub fn i_063c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_063c70  ($063C70, 6 B, thunks/ccr_helpers.c)
/// $063C70: ori.b #$11, ccr
pub fn i_063c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C74;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $063C74: rts
pub fn i_063c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_SpawnByTable_063c76  ($063C76, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063C76: clr.b d6
pub fn i_063c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C78;
    cpu.ir = 0x4206;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $063C78: movea.l a0, a2
pub fn i_063c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C7A;
    cpu.ir = 0x2448;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $063C7A: move.l (a2), d0
pub fn i_063c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C7C;
    cpu.ir = 0x2012;
    let v2 = bus.read32(cpu.a[2]);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063C7C: cmpi.l #$ffffffff, d0
pub fn i_063c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C82;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0xFFFFFFFF, Size::Long, false);
    8
}
/// $063C82: beq.w $63cb2
pub fn i_063c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C86;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63CB2; return 10; }
    8
}
/// $063C86: movem.l d6/a2, -(a7)
pub fn i_063c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C8A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $063C8A: movea.l d0, a1
pub fn i_063c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C8C;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $063C8C: jsr $4ae.l
pub fn i_063c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63C92);
    cpu.pc = 0x4AE;
    16
}
/// $063C92: jsr $5dd02.l
pub fn i_063c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63C98);
    cpu.pc = 0x5DD02;
    16
}
/// $063C98: movem.l (a7)+, d6/a2
pub fn i_063c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63C9C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $063C9C: move.b d6, $86(a0)
pub fn i_063c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CA0;
    cpu.ir = 0x1146;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $063CA0: addq.b #$1, d6
pub fn i_063ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CA2;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $063CA2: addq.l #$4, a2
pub fn i_063ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CA4;
    cpu.ir = 0x588A;
    cpu.a[2] = cpu.a[2].wrapping_add(4);
    8
}
/// $063CA4: move.w (a2), $80(a0)
pub fn i_063ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CA8;
    cpu.ir = 0x3152;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x80);
    bus.write16(ad3, v2 as u16);
    16
}
/// $063CA8: addq.l #$2, a2
pub fn i_063ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CAA;
    cpu.ir = 0x548A;
    cpu.a[2] = cpu.a[2].wrapping_add(2);
    8
}
/// $063CAA: move.w (a2), $82(a0)
pub fn i_063caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CAE;
    cpu.ir = 0x3152;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[0].wrapping_add(0x82);
    bus.write16(ad3, v2 as u16);
    16
}
/// $063CAE: addq.l #$2, a2
pub fn i_063cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CB0;
    cpu.ir = 0x548A;
    cpu.a[2] = cpu.a[2].wrapping_add(2);
    8
}
/// $063CB0: bra.b $63c7a
pub fn i_063cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CB2;
    cpu.ir = 0x60C8;
    cpu.pc = 0x63C7A;
    10
}
/// $063CB2: rts
pub fn i_063cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_Timer_063cb4  ($063CB4, 20 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063CB4: subq.w #$1, $70(a6)
pub fn i_063cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CB8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063CB8: cmpi.w #$0, $70(a6)
pub fn i_063cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CBE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $063CBE: bgt.w $63cce
pub fn i_063cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CC2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x63CCE; return 10; }
    8
}
/// $063CC2: move.w #$0, $70(a6)
pub fn i_063cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_063cc8  ($063CC8, 6 B, thunks/ccr_helpers.c)
/// $063CC8: ori.b #$11, ccr
pub fn i_063cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CCC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $063CCC: rts
pub fn i_063ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_063cce  ($063CCE, 6 B, thunks/ccr_helpers.c)
/// $063CCE: andi.b #$ee, ccr
pub fn i_063cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CD2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $063CD2: rts
pub fn i_063cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_FollowParent_063cd4  ($063CD4, 28 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063CD4: move.b $3a(a6), $87(a6)
pub fn i_063cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CDA;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad4, v3 as u8);
    20
}
/// $063CDA: jsr $5e506.l
pub fn i_063cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63CE0);
    cpu.pc = 0x5E506;
    16
}
/// $063CE0: move.b $87(a6), $3a(a6)
pub fn i_063ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CE6;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $063CE6: move.w $88(a6), d0
pub fn i_063ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CEA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063CEA: add.w d0, $38(a6)
pub fn i_063cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CEE;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063CEE: rts
pub fn i_063cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_AnimByFacing_063cf0  ($063CF0, 58 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063CF0: movea.l $c(a6), a0
pub fn i_063cf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CF4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $063CF4: move.w $80(a6), d0
pub fn i_063cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CF8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063CF8: move.w $82(a6), d1
pub fn i_063cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63CFC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063CFC: cmpi.b #$1, $3a(a0)
pub fn i_063cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D02;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $063D02: beq.w $63d08
pub fn i_063d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D06;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63D08; return 10; }
    8
}
/// $063D06: neg.w d0
pub fn i_063d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D08;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $063D08: add.w d0, $22(a6)
pub fn i_063d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D0C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063D0C: add.w d1, $24(a6)
pub fn i_063d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D10;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063D10: move.b $3b(a0), d0
pub fn i_063d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D14;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $063D14: andi.w #$3, d0
pub fn i_063d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D18;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063D18: add.w d0, d0
pub fn i_063d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D1A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $063D1A: lea.l $2c5820.l, a1
pub fn i_063d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C5820;
    20
}
/// $063D20: move.w (a1, d0.w), d0
pub fn i_063d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D24;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $063D24: add.w d0, $24(a6)
pub fn i_063d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D28;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063D28: rts
pub fn i_063d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_Points_063d2a  ($063D2A, 60 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063D2A: move.w #$ffe4, d0
pub fn i_063d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D2E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFE4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063D2E: btst.b #$0, $3a(a6)
pub fn i_063d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D34;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063D34: beq.w $63d3c
pub fn i_063d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63D3C; return 10; }
    8
}
/// $063D38: move.w #$0, d0
pub fn i_063d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D3C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063D3C: add.w $22(a6), d0
pub fn i_063d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D40;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $063D40: move.w $24(a6), d1
pub fn i_063d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D44;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063D44: addi.w #$2c, d1
pub fn i_063d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D48;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2C, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063D48: move.w #$1c, d2
pub fn i_063d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D4C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063D4C: jsr $99812.l
pub fn i_063d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63D52);
    cpu.pc = 0x99812;
    16
}
/// $063D52: move.w #$ffd0, d0
pub fn i_063d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D56;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFFD0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063D56: add.w $22(a6), d0
pub fn i_063d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D5A;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $063D5A: move.w $24(a6), d1
pub fn i_063d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D5E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063D5E: addi.w #$1c, d1
pub fn i_063d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D62;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1C, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063D62: move.w #$60, d2
pub fn i_063d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D66;
    cpu.ir = 0x343C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_063d66  ($063D66, 8 B, thunks/jsr_abs_thunks.c)
/// $063D66: jsr $99812.l
pub fn i_063d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63D6C);
    cpu.pc = 0x99812;
    16
}
/// $063D6C: rts
pub fn i_063d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_Offset_063d6e  ($063D6E, 20 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063D6E: tst.b $91(a6)
pub fn i_063d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D72;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x91);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $063D72: beq.w $63d7c
pub fn i_063d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63D7C; return 10; }
    8
}
/// $063D76: bset.b #$0, $5a(a6)
pub fn i_063d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D7C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $063D7C: clr.b $91(a6)
pub fn i_063d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D80;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $063D80: rts
pub fn i_063d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_SpawnSmoke_063d82  ($063D82, 48 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063D82: jsr $6e412.l
pub fn i_063d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63D88);
    cpu.pc = 0x6E412;
    16
}
/// $063D88: move.w #$c, d0
pub fn i_063d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063D8C: movem.w d0, -(a7)
pub fn i_063d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D90;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $063D90: lea.l $62536.l, a1
pub fn i_063d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D96;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x62536;
    20
}
/// $063D96: jsr $4ae.l
pub fn i_063d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63D9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63D9C);
    cpu.pc = 0x4AE;
    16
}
/// $063D9C: jsr $5dd02.l
pub fn i_063d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63DA2);
    cpu.pc = 0x5DD02;
    16
}
/// $063DA2: addi.w #$18, $24(a0)
pub fn i_063da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DA8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $063DA8: movem.w (a7)+, d0
pub fn i_063da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DAC;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $063DAC: subq.w #$1, d0
pub fn i_063dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DAE;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $063DAE: bcc.b $63d8c
pub fn i_063dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DB0;
    cpu.ir = 0x64DC;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63D8C; return 10; }
    8
}
/// $063DB0: rts
pub fn i_063db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_SpawnDebris_063db2  ($063DB2, 64 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063DB2: move.w #$1033, d0
pub fn i_063db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DB6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063DB6: jsr $2352.l
pub fn i_063db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63DBC);
    cpu.pc = 0x2352;
    16
}
/// $063DBC: move.w $38(a6), $5c(a6)
pub fn i_063dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DC2;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $063DC2: move.w #$4000, d0
pub fn i_063dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063DC6: jsr $28134.l
pub fn i_063dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63DCC);
    cpu.pc = 0x28134;
    16
}
/// $063DCC: andi.w #$ffe3, $38(a6)
pub fn i_063dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DD2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063DD2: ori.w #$8, $38(a6)
pub fn i_063dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DD8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063DD8: lea.l $77fd6.l, a1
pub fn i_063dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DDE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $063DDE: jsr $4ae.l
pub fn i_063dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63DE4);
    cpu.pc = 0x4AE;
    16
}
/// $063DE4: jsr $5dd02.l
pub fn i_063de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63DEA);
    cpu.pc = 0x5DD02;
    16
}
/// $063DEA: move.w $5c(a6), $38(a6)
pub fn i_063dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DF0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $063DF0: rts
pub fn i_063df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_SpawnDebrisB_063df2  ($063DF2, 58 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063DF2: move.w #$1023, d0
pub fn i_063df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DF6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063DF6: jsr $2352.l
pub fn i_063df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63DFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63DFC);
    cpu.pc = 0x2352;
    16
}
/// $063DFC: move.w $38(a6), $5c(a6)
pub fn i_063dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E02;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $063E02: move.w #$4000, d0
pub fn i_063e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E06;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063E06: jsr $28134.l
pub fn i_063e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63E0C);
    cpu.pc = 0x28134;
    16
}
/// $063E0C: andi.w #$ffe3, $38(a6)
pub fn i_063e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E12;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063E12: ori.w #$8, $38(a6)
pub fn i_063e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E18;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063E18: lea.l $2c5686.l, a1
pub fn i_063e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E1E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C5686;
    20
}
/// $063E1E: jsr $77c7e.l
pub fn i_063e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63E24);
    cpu.pc = 0x77C7E;
    16
}
/// $063E24: move.w $5c(a6), $38(a6)
pub fn i_063e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E2A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $063E2A: rts
pub fn i_063e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E2C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_Register_063e2c  ($063E2C, 94 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063E2C: move.b #$ff, $20(a6)
pub fn i_063e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $063E32: tst.b $9b(a6)
pub fn i_063e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E36;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $063E36: beq.w $63e3c
pub fn i_063e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E3A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63E3C; return 10; }
    8
}
/// $063E3A: rts
pub fn i_063e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E3C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $063E3C: move.l #$2c5542, $4c(a6)
pub fn i_063e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E44;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C5542;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $063E44: jsr $283ca.l
pub fn i_063e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63E4A);
    cpu.pc = 0x283CA;
    16
}
/// $063E4A: jsr $283d8.l
pub fn i_063e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63E50);
    cpu.pc = 0x283D8;
    16
}
/// $063E50: move.l #$ffffffff, $4c(a6)
pub fn i_063e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E58;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $063E58: lea.l $2c5730.l, a1
pub fn i_063e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E5E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C5730;
    20
}
/// $063E5E: tst.b $78(a6)
pub fn i_063e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E62;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $063E62: beq.w $63e6c
pub fn i_063e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E66;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63E6C; return 10; }
    8
}
/// $063E66: lea.l $2c57a8.l, a1
pub fn i_063e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E6C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C57A8;
    20
}
/// $063E6C: btst.b #$0, $3a(a6)
pub fn i_063e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E72;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063E72: beq.w $63e8a
pub fn i_063e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63E8A; return 10; }
    8
}
/// $063E76: lea.l $2c576c.l, a1
pub fn i_063e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E7C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C576C;
    20
}
/// $063E7C: tst.b $78(a6)
pub fn i_063e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E80;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $063E80: beq.w $63e8a
pub fn i_063e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E84;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x63E8A; return 10; }
    8
}
/// $063E84: lea.l $2c57e4.l, a1
pub fn i_063e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E8A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C57E4;
    20
}
// ===== JsrAbsThunk_063e8a  ($063E8A, 8 B, thunks/jsr_abs_thunks.c)
/// $063E8A: jsr $43fac.l
pub fn i_063e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63E90);
    cpu.pc = 0x43FAC;
    16
}
/// $063E90: rts
pub fn i_063e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_SpawnChild_063e92  ($063E92, 24 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063E92: lea.l $77fd6.l, a1
pub fn i_063e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E98;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $063E98: jsr $4ae.l
pub fn i_063e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63E9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63E9E);
    cpu.pc = 0x4AE;
    16
}
/// $063E9E: jsr $5dd02.l
pub fn i_063e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63EA4);
    cpu.pc = 0x5DD02;
    16
}
/// $063EA4: subq.w #$1, $38(a0)
pub fn i_063ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EA8;
    cpu.ir = 0x5368;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $063EA8: rts
pub fn i_063ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Mortar_StubD_063eaa  ($063EAA, 16 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063EAA: movea.l $8(a6), a1
pub fn i_063eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EAE;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $063EAE: move.b $10(a6), d0
pub fn i_063eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EB2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $063EB2: cmp.b $10(a1), d0
pub fn i_063eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EB6;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $063EB6: bcs.w $63ec0
pub fn i_063eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EBA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x63EC0; return 10; }
    8
}
// ===== ClearXN_063eba  ($063EBA, 6 B, thunks/ccr_helpers.c)
/// $063EBA: andi.b #$ee, ccr
pub fn i_063eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EBE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $063EBE: rts
pub fn i_063ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_063ec0  ($063EC0, 6 B, thunks/ccr_helpers.c)
/// $063EC0: ori.b #$11, ccr
pub fn i_063ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EC4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $063EC4: rts
pub fn i_063ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EC6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_Tmpl3E_063ec6  ($063EC6, 216 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063EC6: bset.b #$3, $13(a6)
pub fn i_063ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63ECC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $063ECC: move.l #$2c5958, $48(a6)
pub fn i_063ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63ED4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C5958;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $063ED4: bra.w $63ee6
pub fn i_063ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63ED8;
    cpu.ir = 0x6000;
    cpu.pc = 0x63EE6;
    10
}
/// $063EE6: move.w #$83, d1
pub fn i_063ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EEA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063EEA: jsr $236e.l
pub fn i_063eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63EF0);
    cpu.pc = 0x236E;
    16
}
/// $063EF0: move.w #$19, $1c(a6)
pub fn i_063ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EF6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063EF6: jsr $138fe.l
pub fn i_063ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63EFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63EFC);
    cpu.pc = 0x138FE;
    16
}
/// $063EFC: lea.l $2b8c68.l, a0
pub fn i_063efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8C68;
    20
}
/// $063F02: jsr $799de.l
pub fn i_063f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F08);
    cpu.pc = 0x799DE;
    16
}
/// $063F08: move.w d0, $72(a6)
pub fn i_063f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F0C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063F0C: lea.l $2b8b64.l, a0
pub fn i_063f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8B64;
    20
}
/// $063F12: jsr $799de.l
pub fn i_063f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F18);
    cpu.pc = 0x799DE;
    16
}
/// $063F18: move.w d0, $66(a6)
pub fn i_063f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F1C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063F1C: asr.w #$1, d0
pub fn i_063f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F1E;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $063F1E: move.w d0, $80(a6)
pub fn i_063f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F22;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063F22: clr.b $83(a6)
pub fn i_063f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F26;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x83);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $063F26: clr.b $82(a6)
pub fn i_063f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F2A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $063F2A: move.w #$100, d0
pub fn i_063f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F2E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $063F2E: jsr $28134.l
pub fn i_063f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F34);
    cpu.pc = 0x28134;
    16
}
/// $063F34: andi.w #$ffe3, $38(a6)
pub fn i_063f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F3A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063F3A: ori.w #$14, $38(a6)
pub fn i_063f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F40;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $063F40: jsr $267e2.l
pub fn i_063f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F46);
    cpu.pc = 0x267E2;
    16
}
/// $063F46: lea.l $723d2.l, a1
pub fn i_063f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F4C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x723D2;
    20
}
/// $063F4C: jsr $4ae.l
pub fn i_063f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F52);
    cpu.pc = 0x4AE;
    16
}
/// $063F52: jsr $5dd02.l
pub fn i_063f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F58);
    cpu.pc = 0x5DD02;
    16
}
/// $063F58: clr.w $98(a0)
pub fn i_063f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F5C;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $063F5C: move.b $98(a6), d0
pub fn i_063f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F60;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $063F60: andi.w #$1f, d0
pub fn i_063f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F64;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $063F64: lsl.w #$4, d0
pub fn i_063f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F66;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $063F66: move.w d0, $86(a6)
pub fn i_063f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F6A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write16(ad2, v1 as u16);
    12
}
/// $063F6A: lea.l $2c5ab2.l, a0
pub fn i_063f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5AB2;
    20
}
/// $063F70: jsr $28cd4.l
pub fn i_063f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F76);
    cpu.pc = 0x28CD4;
    16
}
/// $063F76: lea.l $63f7c(pc), a1
pub fn i_063f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63F7C;
    16
}
/// $063F7A: move.l a1, (a6)
pub fn i_063f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F7C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063F7C: jsr $2783a.l
pub fn i_063f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F82);
    cpu.pc = 0x2783A;
    16
}
/// $063F82: jsr $28d70.l
pub fn i_063f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63F88);
    cpu.pc = 0x28D70;
    16
}
/// $063F88: move.w $22(a6), d0
pub fn i_063f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F8C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $063F8C: cmp.w $86(a6), d0
pub fn i_063f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F90;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $063F90: bgt.w $63f9a
pub fn i_063f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F94;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x63F9A; return 10; }
    8
}
/// $063F94: lea.l $63f9e(pc), a1
pub fn i_063f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63F9E;
    16
}
/// $063F98: move.l a1, (a6)
pub fn i_063f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063F9A: bra.w $641d2
pub fn i_063f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63F9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x641D2;
    10
}
// ===== Cannon_Idle_063f9e  ($063F9E, 50 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063F9E: bclr.b #$3, $13(a6)
pub fn i_063f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FA4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $063FA4: lea.l $2c5ac6.l, a0
pub fn i_063fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5AC6;
    20
}
/// $063FAA: jsr $28cd4.l
pub fn i_063faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63FB0);
    cpu.pc = 0x28CD4;
    16
}
/// $063FB0: lea.l $63fb6(pc), a1
pub fn i_063fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63FB6;
    16
}
/// $063FB4: move.l a1, (a6)
pub fn i_063fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063FB6: jsr $2783a.l
pub fn i_063fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63FBC);
    cpu.pc = 0x2783A;
    16
}
/// $063FBC: jsr $28d70.l
pub fn i_063fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63FC2);
    cpu.pc = 0x28D70;
    16
}
/// $063FC2: bcc.w $63fcc
pub fn i_063fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FC6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x63FCC; return 10; }
    8
}
/// $063FC6: lea.l $63fd0(pc), a1
pub fn i_063fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63FD0;
    16
}
/// $063FCA: move.l a1, (a6)
pub fn i_063fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $063FCC: bra.w $641d2
pub fn i_063fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FD0;
    cpu.ir = 0x6000;
    cpu.pc = 0x641D2;
    10
}
// ===== Cannon_Fire_063fd0  ($063FD0, 156 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $063FD0: btst.b #$0, $83(a6)
pub fn i_063fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FD6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $063FD6: bne.w $64002
pub fn i_063fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FDA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64002; return 10; }
    8
}
/// $063FDA: jsr $64422(pc)
pub fn i_063fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FDE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x63FDE);
    cpu.pc = 0x64422;
    16
}
/// $063FDE: bcs.w $63ff2
pub fn i_063fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FE2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x63FF2; return 10; }
    8
}
/// $063FE2: lea.l $2c5b90.l, a0
pub fn i_063fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5B90;
    20
}
/// $063FE8: jsr $28cd4.l
pub fn i_063fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63FEE);
    cpu.pc = 0x28CD4;
    16
}
/// $063FEE: bra.w $63ffe
pub fn i_063fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x63FFE;
    10
}
/// $063FF2: lea.l $2c5bb8.l, a0
pub fn i_063ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FF8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5BB8;
    20
}
/// $063FF8: jsr $28cd4.l
pub fn i_063ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x63FFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x63FFE);
    cpu.pc = 0x28CD4;
    16
}
/// $063FFE: bra.w $64026
pub fn i_063ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64002;
    cpu.ir = 0x6000;
    cpu.pc = 0x64026;
    10
}
/// $064002: jsr $64422(pc)
pub fn i_064002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64006;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64006);
    cpu.pc = 0x64422;
    16
}
/// $064006: bcs.w $6401a
pub fn i_064006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6400A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6401A; return 10; }
    8
}
/// $06400A: lea.l $2c5b7c.l, a0
pub fn i_06400a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64010;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5B7C;
    20
}
/// $064010: jsr $28cd4.l
pub fn i_064010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64016;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64016);
    cpu.pc = 0x28CD4;
    16
}
/// $064016: bra.w $64026
pub fn i_064016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6401A;
    cpu.ir = 0x6000;
    cpu.pc = 0x64026;
    10
}
/// $06401A: lea.l $2c5ba4.l, a0
pub fn i_06401a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64020;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5BA4;
    20
}
/// $064020: jsr $28cd4.l
pub fn i_064020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64026);
    cpu.pc = 0x28CD4;
    16
}
/// $064026: lea.l $6402c(pc), a1
pub fn i_064026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6402A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6402C;
    16
}
/// $06402A: move.l a1, (a6)
pub fn i_06402a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6402C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06402C: jsr $2783a.l
pub fn i_06402c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64032;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64032);
    cpu.pc = 0x2783A;
    16
}
/// $064032: jsr $28d70.l
pub fn i_064032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64038;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64038);
    cpu.pc = 0x28D70;
    16
}
/// $064038: jsr $6443c(pc)
pub fn i_064038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6403C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6403C);
    cpu.pc = 0x6443C;
    16
}
/// $06403C: bcc.w $64046
pub fn i_06403c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64040;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64046; return 10; }
    8
}
/// $064040: lea.l $63fd0(pc), a1
pub fn i_064040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64044;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63FD0;
    16
}
/// $064044: move.l a1, (a6)
pub fn i_064044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64046;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064046: cmpi.w #$20, $22(a6)
pub fn i_064046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6404C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $06404C: blt.w $64068
pub fn i_06404c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64050;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x64068; return 10; }
    8
}
/// $064050: cmpi.w #$120, $22(a6)
pub fn i_064050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64056;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $064056: bgt.w $64068
pub fn i_064056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6405A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x64068; return 10; }
    8
}
/// $06405A: jsr $643da(pc)
pub fn i_06405a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6405E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6405E);
    cpu.pc = 0x643DA;
    16
}
/// $06405E: bcc.w $64068
pub fn i_06405e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64062;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64068; return 10; }
    8
}
/// $064062: lea.l $6406c(pc), a1
pub fn i_064062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64066;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6406C;
    16
}
/// $064066: move.l a1, (a6)
pub fn i_064066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64068;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064068: bra.w $641d2
pub fn i_064068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6406C;
    cpu.ir = 0x6000;
    cpu.pc = 0x641D2;
    10
}
// ===== Cannon_FireB_06406c  ($06406C, 132 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06406C: jsr $643f4(pc)
pub fn i_06406c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64070;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64070);
    cpu.pc = 0x643F4;
    16
}
/// $064070: bcs.w $640f0
pub fn i_064070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64074;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x640F0; return 10; }
    8
}
/// $064074: btst.b #$0, $83(a6)
pub fn i_064074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6407A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06407A: bne.w $640a6
pub fn i_06407a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6407E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x640A6; return 10; }
    8
}
/// $06407E: jsr $64422(pc)
pub fn i_06407e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64082;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64082);
    cpu.pc = 0x64422;
    16
}
/// $064082: bcs.w $64096
pub fn i_064082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64086;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x64096; return 10; }
    8
}
/// $064086: lea.l $2c5bcc.l, a0
pub fn i_064086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6408C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5BCC;
    20
}
/// $06408C: jsr $28cd4.l
pub fn i_06408c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64092);
    cpu.pc = 0x28CD4;
    16
}
/// $064092: bra.w $640a2
pub fn i_064092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64096;
    cpu.ir = 0x6000;
    cpu.pc = 0x640A2;
    10
}
/// $064096: lea.l $2c5c84.l, a0
pub fn i_064096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6409C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5C84;
    20
}
/// $06409C: jsr $28cd4.l
pub fn i_06409c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x640A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0640A2: bra.w $640ca
pub fn i_0640a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640A6;
    cpu.ir = 0x6000;
    cpu.pc = 0x640CA;
    10
}
/// $0640A6: jsr $64422(pc)
pub fn i_0640a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x640AA);
    cpu.pc = 0x64422;
    16
}
/// $0640AA: bcs.w $640be
pub fn i_0640aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640AE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x640BE; return 10; }
    8
}
/// $0640AE: lea.l $2c5c28.l, a0
pub fn i_0640ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5C28;
    20
}
/// $0640B4: jsr $28cd4.l
pub fn i_0640b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x640BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0640BA: bra.w $640ca
pub fn i_0640ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x640CA;
    10
}
/// $0640BE: lea.l $2c5ce0.l, a0
pub fn i_0640be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5CE0;
    20
}
/// $0640C4: jsr $28cd4.l
pub fn i_0640c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x640CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0640CA: eori.b #$1, $83(a6)
pub fn i_0640ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640D0;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x83);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0640D0: lea.l $640d6(pc), a1
pub fn i_0640d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x640D6;
    16
}
/// $0640D4: move.l a1, (a6)
pub fn i_0640d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0640D6: jsr $2783a.l
pub fn i_0640d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x640DC);
    cpu.pc = 0x2783A;
    16
}
/// $0640DC: jsr $28d70.l
pub fn i_0640dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x640E2);
    cpu.pc = 0x28D70;
    16
}
/// $0640E2: bcc.w $640ec
pub fn i_0640e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x640EC; return 10; }
    8
}
/// $0640E6: lea.l $640f0(pc), a1
pub fn i_0640e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x640F0;
    16
}
/// $0640EA: move.l a1, (a6)
pub fn i_0640ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0640EC: bra.w $641d2
pub fn i_0640ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x641D2;
    10
}
// ===== Cannon_Rearm_0640f0  ($0640F0, 20 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0640F0: clr.w $76(a6)
pub fn i_0640f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640F4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0640F4: lea.l $2b8be6.l, a0
pub fn i_0640f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x640FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8BE6;
    20
}
/// $0640FA: jsr $799de.l
pub fn i_0640fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64100;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64100);
    cpu.pc = 0x799DE;
    16
}
/// $064100: move.w d0, $74(a6)
pub fn i_064100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64104;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== Cannon_FireC_064104  ($064104, 174 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064104: lea.l $2b8cea.l, a0
pub fn i_064104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6410A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8CEA;
    20
}
/// $06410A: jsr $799de.l
pub fn i_06410a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64110;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64110);
    cpu.pc = 0x799DE;
    16
}
/// $064110: move.w d0, $72(a6)
pub fn i_064110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64114;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064114: btst.b #$0, $83(a6)
pub fn i_064114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6411A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06411A: bne.w $64146
pub fn i_06411a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6411E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64146; return 10; }
    8
}
/// $06411E: jsr $64422(pc)
pub fn i_06411e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64122;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64122);
    cpu.pc = 0x64422;
    16
}
/// $064122: bcs.w $64136
pub fn i_064122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64126;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x64136; return 10; }
    8
}
/// $064126: lea.l $2c5d8c.l, a0
pub fn i_064126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6412C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5D8C;
    20
}
/// $06412C: jsr $28cd4.l
pub fn i_06412c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64132;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64132);
    cpu.pc = 0x28CD4;
    16
}
/// $064132: bra.w $64142
pub fn i_064132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64136;
    cpu.ir = 0x6000;
    cpu.pc = 0x64142;
    10
}
/// $064136: lea.l $2c5e2c.l, a0
pub fn i_064136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6413C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5E2C;
    20
}
/// $06413C: jsr $28cd4.l
pub fn i_06413c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64142;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64142);
    cpu.pc = 0x28CD4;
    16
}
/// $064142: bra.w $6416a
pub fn i_064142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64146;
    cpu.ir = 0x6000;
    cpu.pc = 0x6416A;
    10
}
/// $064146: jsr $64422(pc)
pub fn i_064146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6414A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6414A);
    cpu.pc = 0x64422;
    16
}
/// $06414A: bcs.w $6415e
pub fn i_06414a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6414E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6415E; return 10; }
    8
}
/// $06414E: lea.l $2c5d3c.l, a0
pub fn i_06414e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64154;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5D3C;
    20
}
/// $064154: jsr $28cd4.l
pub fn i_064154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6415A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6415A);
    cpu.pc = 0x28CD4;
    16
}
/// $06415A: bra.w $6416a
pub fn i_06415a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6415E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6416A;
    10
}
/// $06415E: lea.l $2c5ddc.l, a0
pub fn i_06415e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64164;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5DDC;
    20
}
/// $064164: jsr $28cd4.l
pub fn i_064164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6416A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6416A);
    cpu.pc = 0x28CD4;
    16
}
/// $06416A: lea.l $64170(pc), a1
pub fn i_06416a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6416E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64170;
    16
}
/// $06416E: move.l a1, (a6)
pub fn i_06416e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64170;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064170: jsr $2783a.l
pub fn i_064170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64176;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64176);
    cpu.pc = 0x2783A;
    16
}
/// $064176: jsr $28d70.l
pub fn i_064176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6417C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6417C);
    cpu.pc = 0x28D70;
    16
}
/// $06417C: bcc.w $641ae
pub fn i_06417c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64180;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x641AE; return 10; }
    8
}
/// $064180: lea.l $64104(pc), a1
pub fn i_064180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64184;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64104;
    16
}
/// $064184: move.l a1, (a6)
pub fn i_064184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64186;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064186: addq.w #$1, $76(a6)
pub fn i_064186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6418A;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06418A: subq.w #$1, $74(a6)
pub fn i_06418a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6418E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06418E: cmpi.w #$0, $74(a6)
pub fn i_06418e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64194;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $064194: bgt.w $641ae
pub fn i_064194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64198;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x641AE; return 10; }
    8
}
/// $064198: lea.l $63fd0(pc), a1
pub fn i_064198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6419C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x63FD0;
    16
}
/// $06419C: move.l a1, (a6)
pub fn i_06419c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6419E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06419E: lea.l $2b8c68.l, a0
pub fn i_06419e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B8C68;
    20
}
/// $0641A4: jsr $799de.l
pub fn i_0641a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x641AA);
    cpu.pc = 0x799DE;
    16
}
/// $0641AA: move.w d0, $72(a6)
pub fn i_0641aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641AE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0641AE: bra.w $641d2
pub fn i_0641ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x641D2;
    10
}
// ===== SetTaskHandler_0641ca  ($0641CA, 8 B, kernel/task_handlers.c)
/// $0641CA: lea.l $64222(pc), a1
pub fn i_0641ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64222;
    16
}
/// $0641CE: move.l a1, (a6)
pub fn i_0641ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641D0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0641D0: rts
pub fn i_0641d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_HitCheck_0641d2  ($0641D2, 72 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0641D2: jsr $2870a.l
pub fn i_0641d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x641D8);
    cpu.pc = 0x2870A;
    16
}
/// $0641D8: bcc.w $641f4
pub fn i_0641d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x641F4; return 10; }
    8
}
/// $0641DC: jsr $519be.l
pub fn i_0641dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x641E2);
    cpu.pc = 0x519BE;
    16
}
/// $0641E2: lea.l $5e766.l, a0
pub fn i_0641e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0641E8: jsr $5e770.l
pub fn i_0641e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x641EE);
    cpu.pc = 0x5E770;
    16
}
/// $0641EE: bclr.b #$3, $13(a6)
pub fn i_0641ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0641F4: jsr $28758.l
pub fn i_0641f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x641FA);
    cpu.pc = 0x28758;
    16
}
/// $0641FA: bcc.w $64204
pub fn i_0641fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x641FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64204; return 10; }
    8
}
/// $0641FE: lea.l $641b2(pc), a1
pub fn i_0641fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64202;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x641B2;
    16
}
/// $064202: move.l a1, (a6)
pub fn i_064202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64204;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064204: movea.l #$ffffffff, a0
pub fn i_064204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6420A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06420A: lea.l $2c5a54.l, a0
pub fn i_06420a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64210;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5A54;
    20
}
/// $064210: jsr $5dd5c.l
pub fn i_064210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64216;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64216);
    cpu.pc = 0x5DD5C;
    16
}
/// $064216: bcc.w $64220
pub fn i_064216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6421A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64220; return 10; }
    8
}
// ===== SetTaskHandler_06421a  ($06421A, 8 B, kernel/task_handlers.c)
/// $06421A: lea.l $64222(pc), a1
pub fn i_06421a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6421E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64222;
    16
}
/// $06421E: move.l a1, (a6)
pub fn i_06421e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64220;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064220: rts
pub fn i_064220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64222;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_064222  ($064222, 8 B, kernel/jmp_to_scheduler.c)
/// $064222: jmp $518.l
pub fn i_064222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64228;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Jsr5B6ThenJmpScheduler_06422a  ($06422A, 14 B, kernel/jsr5b6_jmp518.c)
/// $06422A: jsr $5b6.l
pub fn i_06422a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64230;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64230);
    cpu.pc = 0x5B6;
    16
}
/// $064230: jmp $518.l
pub fn i_064230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64236;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Cannon_Explode_064238  ($064238, 36 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064238: move.w #$4000, d0
pub fn i_064238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6423C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06423C: jsr $28134.l
pub fn i_06423c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64242);
    cpu.pc = 0x28134;
    16
}
/// $064242: andi.w #$ffe3, $38(a6)
pub fn i_064242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64248;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $064248: ori.w #$0, $38(a6)
pub fn i_064248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6424E;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06424E: move.l #$ffffffff, $48(a6)
pub fn i_06424e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64256;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $064256: jmp $77efe.l
pub fn i_064256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6425C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77EFE;
    8
}
// ===== Cannon_Shell_06425c  ($06425C, 278 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06425C: move.w $76(a6), d0
pub fn i_06425c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64260;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $064260: andi.w #$3, d0
pub fn i_064260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64264;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $064264: move.w d0, d1
pub fn i_064264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64266;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064266: add.w d0, d0
pub fn i_064266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64268;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $064268: add.w d1, d0
pub fn i_064268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6426A;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06426A: add.w d0, d0
pub fn i_06426a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6426C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06426C: lea.l $2c5a9a.l, a0
pub fn i_06426c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64272;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5A9A;
    20
}
/// $064272: move.w $2(a0, d0.w), $2e(a6)
pub fn i_064272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64278;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $064278: move.w $4(a0, d0.w), $2a(a6)
pub fn i_064278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6427E;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06427E: move.w (a0, d0.w), d0
pub fn i_06427e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64282;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $064282: jsr $64486(pc)
pub fn i_064282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64286;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64286);
    cpu.pc = 0x64486;
    16
}
/// $064286: move.w d0, $28(a6)
pub fn i_064286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6428A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06428A: bset.b #$4, $6b(a6)
pub fn i_06428a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64290;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $064290: move.l #$2c5a00, $4c(a6)
pub fn i_064290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64298;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C5A00;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $064298: move.w #$167, d1
pub fn i_064298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6429C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x167;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06429C: jsr $236e.l
pub fn i_06429c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x642A2);
    cpu.pc = 0x236E;
    16
}
/// $0642A2: move.w #$1d2, d1
pub fn i_0642a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642A6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0642A6: jsr $236e.l
pub fn i_0642a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x642AC);
    cpu.pc = 0x236E;
    16
}
/// $0642AC: jsr $283ca.l
pub fn i_0642ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x642B2);
    cpu.pc = 0x283CA;
    16
}
/// $0642B2: move.w #$d000, d0
pub fn i_0642b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0642B6: jsr $28134.l
pub fn i_0642b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x642BC);
    cpu.pc = 0x28134;
    16
}
/// $0642BC: andi.w #$ffe3, $38(a6)
pub fn i_0642bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642C2;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0642C2: ori.w #$14, $38(a6)
pub fn i_0642c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642C8;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0642C8: lea.l $2c5e92.l, a0
pub fn i_0642c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5E92;
    20
}
/// $0642CE: jsr $28cd4.l
pub fn i_0642ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x642D4);
    cpu.pc = 0x28CD4;
    16
}
/// $0642D4: lea.l $642da(pc), a1
pub fn i_0642d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x642DA;
    16
}
/// $0642D8: move.l a1, (a6)
pub fn i_0642d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0642DA: jsr $27cee.l
pub fn i_0642da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x642E0);
    cpu.pc = 0x27CEE;
    16
}
/// $0642E0: bcc.w $642ea
pub fn i_0642e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642E4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x642EA; return 10; }
    8
}
/// $0642E4: lea.l $64238(pc), a1
pub fn i_0642e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64238;
    16
}
/// $0642E8: move.l a1, (a6)
pub fn i_0642e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0642EA: move.w $28(a6), d0
pub fn i_0642ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642EE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0642EE: move.w $2a(a6), d1
pub fn i_0642ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642F2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0642F2: cmpi.w #$0, d0
pub fn i_0642f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642F6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0642F6: blt.w $642fc
pub fn i_0642f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642FA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x642FC; return 10; }
    8
}
/// $0642FA: neg.w d0
pub fn i_0642fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642FC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0642FC: asr.w #$4, d0
pub fn i_0642fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x642FE;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $0642FE: asr.w #$4, d1
pub fn i_0642fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64300;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $064300: jsr $5e018.l
pub fn i_064300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64306;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64306);
    cpu.pc = 0x5E018;
    16
}
/// $064306: cmpi.w #$60, d0
pub fn i_064306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6430A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $06430A: bge.w $64312
pub fn i_06430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6430E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x64312; return 10; }
    8
}
/// $06430E: move.w #$60, d0
pub fn i_06430e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64312;
    cpu.ir = 0x303C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064312: cmpi.w #$c0, d0
pub fn i_064312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64316;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xC0, Size::Word, false);
    8
}
/// $064316: ble.w $6431e
pub fn i_064316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6431A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6431E; return 10; }
    8
}
/// $06431A: move.w #$c0, d0
pub fn i_06431a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6431E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06431E: subi.w #$60, d0
pub fn i_06431e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64322;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x60, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $064322: lsr.w #$2, d0
pub fn i_064322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64324;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $064324: move.w d0, $34(a6)
pub fn i_064324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64328;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064328: move.w $16(a6), d0
pub fn i_064328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6432C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06432C: btst.b #$0, $106f28.l
pub fn i_06432c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64334;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $064334: beq.w $6433c
pub fn i_064334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64338;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6433C; return 10; }
    8
}
/// $064338: move.w $18(a6), d0
pub fn i_064338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6433C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06433C: move.w d0, $14(a6)
pub fn i_06433c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64340;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064340: jsr $28d70.l
pub fn i_064340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64346);
    cpu.pc = 0x28D70;
    16
}
/// $064346: jsr $283d8.l
pub fn i_064346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6434C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6434C);
    cpu.pc = 0x283D8;
    16
}
/// $06434C: btst.b #$1, $13(a6)
pub fn i_06434c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64352;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $064352: beq.w $6435c
pub fn i_064352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64356;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6435C; return 10; }
    8
}
/// $064356: lea.l $64238(pc), a1
pub fn i_064356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6435A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64238;
    16
}
/// $06435A: move.l a1, (a6)
pub fn i_06435a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6435C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06435C: movea.l #$ffffffff, a0
pub fn i_06435c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64362;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $064362: lea.l $2c5a5c.l, a0
pub fn i_064362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64368;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C5A5C;
    20
}
/// $064368: jsr $5dd56.l
pub fn i_064368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6436E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6436E);
    cpu.pc = 0x5DD56;
    16
}
/// $06436E: bcc.w $64378
pub fn i_06436e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64372;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64378; return 10; }
    8
}
// ===== SetTaskHandler_064372  ($064372, 8 B, kernel/task_handlers.c)
/// $064372: lea.l $6422a(pc), a1
pub fn i_064372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64376;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6422A;
    16
}
/// $064376: move.l a1, (a6)
pub fn i_064376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64378;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064378: rts
pub fn i_064378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6437A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_Flash_06437a  ($06437A, 88 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06437A: move.w #$83, d1
pub fn i_06437a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6437E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x83;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06437E: jsr $236e.l
pub fn i_06437e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64384;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64384);
    cpu.pc = 0x236E;
    16
}
/// $064384: move.w #$d000, d0
pub fn i_064384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64388;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064388: jsr $28134.l
pub fn i_064388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6438E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6438E);
    cpu.pc = 0x28134;
    16
}
/// $06438E: andi.w #$ffe3, $38(a6)
pub fn i_06438e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64394;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $064394: ori.w #$14, $38(a6)
pub fn i_064394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6439A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06439A: lea.l $2c605e.l, a0
pub fn i_06439a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C605E;
    20
}
/// $0643A0: jsr $28cd4.l
pub fn i_0643a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x643A6);
    cpu.pc = 0x28CD4;
    16
}
/// $0643A6: lea.l $643ac(pc), a1
pub fn i_0643a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x643AC;
    16
}
/// $0643AA: move.l a1, (a6)
pub fn i_0643aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0643AC: jsr $2783a.l
pub fn i_0643ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x643B2);
    cpu.pc = 0x2783A;
    16
}
/// $0643B2: jsr $28d70.l
pub fn i_0643b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x643B8);
    cpu.pc = 0x28D70;
    16
}
/// $0643B8: bcc.w $643c2
pub fn i_0643b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643BC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x643C2; return 10; }
    8
}
/// $0643BC: lea.l $6422a(pc), a1
pub fn i_0643bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6422A;
    16
}
/// $0643C0: move.l a1, (a6)
pub fn i_0643c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0643C2: movea.l #$ffffffff, a0
pub fn i_0643c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643C8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0643C8: jsr $5dd56.l
pub fn i_0643c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x643CE);
    cpu.pc = 0x5DD56;
    16
}
/// $0643CE: bcc.w $643d8
pub fn i_0643ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643D2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x643D8; return 10; }
    8
}
// ===== SetTaskHandler_0643d2  ($0643D2, 8 B, kernel/task_handlers.c)
/// $0643D2: lea.l $6422a(pc), a1
pub fn i_0643d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6422A;
    16
}
/// $0643D6: move.l a1, (a6)
pub fn i_0643d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0643D8: rts
pub fn i_0643d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_Timer_0643da  ($0643DA, 14 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0643DA: subq.w #$1, $72(a6)
pub fn i_0643da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643DE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0643DE: cmpi.w #$0, $72(a6)
pub fn i_0643de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643E4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0643E4: bgt.w $643ee
pub fn i_0643e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643E8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x643EE; return 10; }
    8
}
// ===== SetXN_0643e8  ($0643E8, 6 B, thunks/ccr_helpers.c)
/// $0643E8: ori.b #$11, ccr
pub fn i_0643e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643EC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0643EC: rts
pub fn i_0643ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_0643ee  ($0643EE, 6 B, thunks/ccr_helpers.c)
/// $0643EE: andi.b #$ee, ccr
pub fn i_0643ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643F2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0643F2: rts
pub fn i_0643f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_Acquire_0643f4  ($0643F4, 28 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0643F4: move.b $3a(a6), $84(a6)
pub fn i_0643f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x643FA;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0643FA: move.b $83(a6), $3a(a6)
pub fn i_0643fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64400;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $064400: jsr $5e5a8.l
pub fn i_064400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64406;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64406);
    cpu.pc = 0x5E5A8;
    16
}
/// $064406: bcc.w $64416
pub fn i_064406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6440A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64416; return 10; }
    8
}
/// $06440A: move.b $84(a6), $3a(a6)
pub fn i_06440a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64410;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== ClearXN_064410  ($064410, 6 B, thunks/ccr_helpers.c)
/// $064410: andi.b #$ee, ccr
pub fn i_064410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64414;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $064414: rts
pub fn i_064414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64416;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_Stub_064416  ($064416, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064416: move.b $84(a6), $3a(a6)
pub fn i_064416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6441C;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetXN_06441c  ($06441C, 6 B, thunks/ccr_helpers.c)
/// $06441C: ori.b #$11, ccr
pub fn i_06441c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64420;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $064420: rts
pub fn i_064420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64422;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_HitCheckB_064422  ($064422, 14 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064422: jsr $6443c(pc)
pub fn i_064422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64426;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64426);
    cpu.pc = 0x6443C;
    16
}
/// $064426: btst.b #$0, $82(a6)
pub fn i_064426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6442C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06442C: bne.w $64436
pub fn i_06442c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64430;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64436; return 10; }
    8
}
// ===== ClearXN_064430  ($064430, 6 B, thunks/ccr_helpers.c)
/// $064430: andi.b #$ee, ccr
pub fn i_064430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64434;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $064434: rts
pub fn i_064434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64436;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_064436  ($064436, 6 B, thunks/ccr_helpers.c)
/// $064436: ori.b #$11, ccr
pub fn i_064436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6443A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06443A: rts
pub fn i_06443a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6443C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_Die_06443c  ($06443C, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06443C: btst.b #$0, $82(a6)
pub fn i_06443c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64442;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $064442: bne.w $64480
pub fn i_064442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64446;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64480; return 10; }
    8
}
/// $064446: move.w $66(a6), d0
pub fn i_064446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6444A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06444A: cmp.w $80(a6), d0
pub fn i_06444a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6444E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06444E: bgt.w $64480
pub fn i_06444e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64452;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x64480; return 10; }
    8
}
/// $064452: move.w #$1027, d0
pub fn i_064452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64456;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064456: jsr $2352.l
pub fn i_064456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6445C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6445C);
    cpu.pc = 0x2352;
    16
}
/// $06445C: ori.b #$1, $82(a6)
pub fn i_06445c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64462;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $064462: lea.l $2c5a76.l, a1
pub fn i_064462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64468;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C5A76;
    20
}
/// $064468: jsr $77c7e.l
pub fn i_064468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6446E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6446E);
    cpu.pc = 0x77C7E;
    16
}
/// $06446E: lea.l $2c5a88.l, a1
pub fn i_06446e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64474;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C5A88;
    20
}
/// $064474: jsr $77c7e.l
pub fn i_064474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6447A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6447A);
    cpu.pc = 0x77C7E;
    16
}
// ===== SetXN_06447a  ($06447A, 6 B, thunks/ccr_helpers.c)
/// $06447A: ori.b #$11, ccr
pub fn i_06447a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6447E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06447E: rts
pub fn i_06447e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64480;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_064480  ($064480, 6 B, thunks/ccr_helpers.c)
/// $064480: andi.b #$ee, ccr
pub fn i_064480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64484;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $064484: rts
pub fn i_064484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64486;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_Offset_064486  ($064486, 12 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064486: btst.b #$0, $3a(a6)
pub fn i_064486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6448C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06448C: beq.w $64498
pub fn i_06448c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64490;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64498; return 10; }
    8
}
/// $064490: neg.w d0
pub fn i_064490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64492;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetXN_064492  ($064492, 6 B, thunks/ccr_helpers.c)
/// $064492: ori.b #$11, ccr
pub fn i_064492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64496;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $064496: rts
pub fn i_064496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64498;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_064498  ($064498, 6 B, thunks/ccr_helpers.c)
/// $064498: andi.b #$ee, ccr
pub fn i_064498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6449C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06449C: rts
pub fn i_06449c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6449E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_SpawnPair_06449e  ($06449E, 74 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06449E: move.w #$1066, d0
pub fn i_06449e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644A2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1066;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0644A2: jsr $2352.l
pub fn i_0644a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x644A8);
    cpu.pc = 0x2352;
    16
}
/// $0644A8: lea.l $6437a(pc), a1
pub fn i_0644a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6437A;
    16
}
/// $0644AC: jsr $4ae.l
pub fn i_0644ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x644B2);
    cpu.pc = 0x4AE;
    16
}
/// $0644B2: jsr $5dd02.l
pub fn i_0644b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x644B8);
    cpu.pc = 0x5DD02;
    16
}
/// $0644B8: eori.b #$1, $3a(a0)
pub fn i_0644b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644BE;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0644BE: lea.l $6425c(pc), a1
pub fn i_0644be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6425C;
    16
}
/// $0644C2: jsr $4ae.l
pub fn i_0644c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x644C8);
    cpu.pc = 0x4AE;
    16
}
/// $0644C8: jsr $5dd02.l
pub fn i_0644c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x644CE);
    cpu.pc = 0x5DD02;
    16
}
/// $0644CE: addi.w #$3c, $24(a0)
pub fn i_0644ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644D4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x3C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0644D4: addi.w #$10, $22(a0)
pub fn i_0644d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644DA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0644DA: eori.b #$1, $3a(a0)
pub fn i_0644da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644E0;
    cpu.ir = 0xA28;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0644E0: move.w $76(a6), $76(a0)
pub fn i_0644e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644E6;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0644E6: rts
pub fn i_0644e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_SpawnPairB_0644e8  ($0644E8, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0644E8: move.w #$1066, d0
pub fn i_0644e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644EC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1066;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0644EC: jsr $2352.l
pub fn i_0644ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x644F2);
    cpu.pc = 0x2352;
    16
}
/// $0644F2: lea.l $6437a(pc), a1
pub fn i_0644f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6437A;
    16
}
/// $0644F6: jsr $4ae.l
pub fn i_0644f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x644FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x644FC);
    cpu.pc = 0x4AE;
    16
}
/// $0644FC: jsr $5dd02.l
pub fn i_0644fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64502);
    cpu.pc = 0x5DD02;
    16
}
/// $064502: lea.l $6425c(pc), a1
pub fn i_064502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64506;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6425C;
    16
}
/// $064506: jsr $4ae.l
pub fn i_064506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6450C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6450C);
    cpu.pc = 0x4AE;
    16
}
/// $06450C: jsr $5dd02.l
pub fn i_06450c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64512;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64512);
    cpu.pc = 0x5DD02;
    16
}
/// $064512: addi.w #$3c, $24(a0)
pub fn i_064512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64518;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x3C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $064518: addi.w #$fff0, $22(a0)
pub fn i_064518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6451E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xFFF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06451E: move.w $76(a6), $76(a0)
pub fn i_06451e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64524;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $064524: rts
pub fn i_064524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64526;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_Register_064526  ($064526, 6 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064526: lea.l $2c5a64.l, a1
pub fn i_064526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6452C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C5A64;
    20
}
// ===== JsrAbsThunk_06452c  ($06452C, 8 B, thunks/jsr_abs_thunks.c)
/// $06452C: jsr $77c7e.l
pub fn i_06452c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64532);
    cpu.pc = 0x77C7E;
    16
}
/// $064532: rts
pub fn i_064532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64534;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Cannon_StubB_064534  ($064534, 16 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064534: movea.l $8(a6), a1
pub fn i_064534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64538;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $064538: move.b $10(a6), d0
pub fn i_064538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6453C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06453C: cmp.b $10(a1), d0
pub fn i_06453c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64540;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $064540: bcs.w $6454a
pub fn i_064540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64544;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6454A; return 10; }
    8
}
// ===== ClearXN_064544  ($064544, 6 B, thunks/ccr_helpers.c)
/// $064544: andi.b #$ee, ccr
pub fn i_064544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64548;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $064548: rts
pub fn i_064548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6454A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06454a  ($06454A, 6 B, thunks/ccr_helpers.c)
/// $06454A: ori.b #$11, ccr
pub fn i_06454a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6454E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06454E: rts
pub fn i_06454e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64550;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_ResetState_064550  ($064550, 50 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064550: bclr.b #$1, $12(a6)
pub fn i_064550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64556;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $064556: clr.b $10e27a.l
pub fn i_064556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6455C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E27A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E27A, r2 as u8);
    16
}
/// $06455C: clr.b $10e27b.l
pub fn i_06455c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64562;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E27B) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E27B, r2 as u8);
    16
}
/// $064562: lea.l $64568(pc), a1
pub fn i_064562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64566;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64568;
    16
}
/// $064566: move.l a1, (a6)
pub fn i_064566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64568;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064568: move.b $10e277.l, $10e276.l
pub fn i_064568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64572;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E277) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E276, v2 as u8);
    28
}
/// $064572: move.b $10e279.l, $10e278.l
pub fn i_064572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6457C;
    cpu.ir = 0x13F9;
    let v2 = bus.read8(0x10E279) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E278, v2 as u8);
    28
}
/// $06457C: clr.b $10e277.l
pub fn i_06457c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64582;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E277) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E277, r2 as u8);
    16
}
// ===== ClrRamWord_064582  ($064582, 8 B, thunks/clr_ram_word.c)
/// $064582: clr.b $10e279.l
pub fn i_064582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64588;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E279) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E279, r2 as u8);
    16
}
/// $064588: rts
pub fn i_064588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6458A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_SetFlags3435_06458a  ($06458A, 16 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06458A: move.b #$34, $10e27a.l
pub fn i_06458a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64592;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x34;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E27A, v1 as u8);
    16
}
/// $064592: move.b #$35, $10e27b.l
pub fn i_064592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6459A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x35;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E27B, v1 as u8);
    16
}
// ===== Jsr5B6ThenJmpScheduler_06459a  ($06459A, 14 B, kernel/jsr5b6_jmp518.c)
/// $06459A: jsr $5b6.l
pub fn i_06459a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x645A0);
    cpu.pc = 0x5B6;
    16
}
/// $0645A0: jmp $518.l
pub fn i_0645a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645A6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Hostage_Tmpl4C_0645a8  ($0645A8, 152 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0645A8: ori.b #$1, $3a(a6)
pub fn i_0645a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645AE;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0645AE: move.b #$1, $79(a6)
pub fn i_0645ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645B4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0645B4: move.w #$44, d1
pub fn i_0645b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x44;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0645B8: jsr $236e.l
pub fn i_0645b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x645BE);
    cpu.pc = 0x236E;
    16
}
/// $0645BE: jsr $5e7c0.l
pub fn i_0645be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x645C4);
    cpu.pc = 0x5E7C0;
    16
}
/// $0645C4: lea.l $2b958c.l, a0
pub fn i_0645c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B958C;
    20
}
/// $0645CA: jsr $799de.l
pub fn i_0645ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x645D0);
    cpu.pc = 0x799DE;
    16
}
/// $0645D0: move.w d0, $66(a6)
pub fn i_0645d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645D4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0645D4: move.b #$0, $78(a6)
pub fn i_0645d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0645DA: move.w #$4000, d0
pub fn i_0645da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0645DE: jsr $28134.l
pub fn i_0645de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x645E4);
    cpu.pc = 0x28134;
    16
}
/// $0645E4: andi.w #$ffe3, $38(a6)
pub fn i_0645e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645EA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0645EA: ori.w #$14, $38(a6)
pub fn i_0645ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645F0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0645F0: clr.b $21(a6)
pub fn i_0645f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645F4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0645F4: bra.w $64682
pub fn i_0645f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x645F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x64682;
    10
}
// ===== Hostage_Init_064640  ($064640, 184 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064640: move.w #$146, d1
pub fn i_064640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64644;
    cpu.ir = 0x323C;
    let v1: u32 = 0x146;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064644: jsr $236e.l
pub fn i_064644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6464A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6464A);
    cpu.pc = 0x236E;
    16
}
/// $06464A: jsr $5e7c0.l
pub fn i_06464a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64650;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64650);
    cpu.pc = 0x5E7C0;
    16
}
/// $064650: lea.l $2b999c.l, a0
pub fn i_064650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64656;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B999C;
    20
}
/// $064656: jsr $799de.l
pub fn i_064656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6465C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6465C);
    cpu.pc = 0x799DE;
    16
}
/// $06465C: move.w d0, $66(a6)
pub fn i_06465c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64660;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064660: move.b #$1, $78(a6)
pub fn i_064660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64666;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $064666: move.w #$8000, d0
pub fn i_064666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6466A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06466A: jsr $28134.l
pub fn i_06466a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64670;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64670);
    cpu.pc = 0x28134;
    16
}
/// $064670: andi.w #$ffe3, $38(a6)
pub fn i_064670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64676;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $064676: ori.w #$14, $38(a6)
pub fn i_064676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6467C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06467C: clr.b $21(a6)
pub fn i_06467c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64680;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $064680: rts
pub fn i_064680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64682;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $064682: move.b $9a(a6), d0
pub fn i_064682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64686;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $064686: andi.w #$1f, d0
pub fn i_064686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6468A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06468A: lsl.w #$4, d0
pub fn i_06468a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6468C;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06468C: move.w d0, $80(a6)
pub fn i_06468c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64690;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064690: move.b $99(a6), d0
pub fn i_064690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64694;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $064694: andi.w #$ff, d0
pub fn i_064694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64698;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $064698: lsl.w #$8, d0
pub fn i_064698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6469A;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $06469A: move.w d0, $84(a6)
pub fn i_06469a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6469E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06469E: move.w #$0, $8e(a6)
pub fn i_06469e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0646A4: lea.l $646aa(pc), a1
pub fn i_0646a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x646AA;
    16
}
/// $0646A8: move.l a1, (a6)
pub fn i_0646a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0646AA: cmpi.b #$0, $98(a6)
pub fn i_0646aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646B0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0646B0: bne.w $646ba
pub fn i_0646b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646B4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x646BA; return 10; }
    8
}
/// $0646B4: lea.l $64700(pc), a1
pub fn i_0646b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64700;
    16
}
/// $0646B8: move.l a1, (a6)
pub fn i_0646b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0646BA: jsr $65d4a(pc)
pub fn i_0646ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x646BE);
    cpu.pc = 0x65D4A;
    16
}
/// $0646BE: cmp.b $98(a6), d0
pub fn i_0646be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646C2;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0646C2: bge.w $646da
pub fn i_0646c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646C6;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x646DA; return 10; }
    8
}
/// $0646C6: lea.l $64700(pc), a1
pub fn i_0646c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64700;
    16
}
/// $0646CA: move.l a1, (a6)
pub fn i_0646ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0646CC: jsr $65d7a(pc)
pub fn i_0646cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x646D0);
    cpu.pc = 0x65D7A;
    16
}
/// $0646D0: move.w #$105d, d0
pub fn i_0646d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646D4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x105D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0646D4: jsr $2352.l
pub fn i_0646d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x646DA);
    cpu.pc = 0x2352;
    16
}
/// $0646DA: cmpi.b #$34, $10e27a.l
pub fn i_0646da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646E2;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E27A) as u32;
    sub_with_flags(cpu, v2, 0x34, Size::Byte, false);
    20
}
/// $0646E2: bne.w $646ec
pub fn i_0646e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x646EC; return 10; }
    8
}
/// $0646E6: lea.l $64d7a(pc), a1
pub fn i_0646e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $0646EA: move.l a1, (a6)
pub fn i_0646ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0646EC: cmpi.b #$35, $10e27b.l
pub fn i_0646ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646F4;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E27B) as u32;
    sub_with_flags(cpu, v2, 0x35, Size::Byte, false);
    20
}
/// $0646F4: bne.w $646fe
pub fn i_0646f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x646FE; return 10; }
    8
}
// ===== SetTaskHandler_0646f8  ($0646F8, 8 B, kernel/task_handlers.c)
/// $0646F8: lea.l $64d7a(pc), a1
pub fn i_0646f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $0646FC: move.l a1, (a6)
pub fn i_0646fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x646FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0646FE: rts
pub fn i_0646fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64700;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_Walk_064700  ($064700, 192 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064700: cmpi.b #$1, $79(a6)
pub fn i_064700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64706;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x79);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $064706: bne.w $6471a
pub fn i_064706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6470A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6471A; return 10; }
    8
}
/// $06470A: lea.l $64d98(pc), a1
pub fn i_06470a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6470E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D98;
    16
}
/// $06470E: jsr $4ae.l
pub fn i_06470e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64714);
    cpu.pc = 0x4AE;
    16
}
/// $064714: jsr $5dd02.l
pub fn i_064714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6471A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6471A);
    cpu.pc = 0x5DD02;
    16
}
/// $06471A: move.l #$2c60c4, d0
pub fn i_06471a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64720;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2C60C4;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $064720: tst.b $78(a6)
pub fn i_064720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64724;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $064724: beq.w $6472e
pub fn i_064724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64728;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6472E; return 10; }
    8
}
/// $064728: move.l #$2c6118, d0
pub fn i_064728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6472E;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2C6118;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $06472E: move.l d0, $48(a6)
pub fn i_06472e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64732;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $064732: cmpi.b #$1, $78(a6)
pub fn i_064732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64738;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $064738: beq.w $6498e
pub fn i_064738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6473C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6498E; return 10; }
    8
}
/// $06473C: move.w #$1, $8e(a6)
pub fn i_06473c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64742;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064742: lea.l $2b960e.l, a0
pub fn i_064742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64748;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B960E;
    20
}
/// $064748: jsr $799de.l
pub fn i_064748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6474E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6474E);
    cpu.pc = 0x799DE;
    16
}
/// $06474E: jsr $65d32(pc)
pub fn i_06474e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64752;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64752);
    cpu.pc = 0x65D32;
    16
}
/// $064752: neg.w d0
pub fn i_064752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64754;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $064754: move.w d0, $28(a6)
pub fn i_064754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64758;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064758: move.w d0, $82(a6)
pub fn i_064758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6475C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06475C: lea.l $2c661c.l, a0
pub fn i_06475c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64762;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C661C;
    20
}
/// $064762: jsr $28cd4.l
pub fn i_064762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64768;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64768);
    cpu.pc = 0x28CD4;
    16
}
/// $064768: cmpi.b #$1, $78(a6)
pub fn i_064768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6476E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06476E: bne.w $6477e
pub fn i_06476e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64772;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6477E; return 10; }
    8
}
/// $064772: lea.l $2c679a.l, a0
pub fn i_064772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64778;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C679A;
    20
}
/// $064778: jsr $28cd4.l
pub fn i_064778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6477E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6477E);
    cpu.pc = 0x28CD4;
    16
}
/// $06477E: lea.l $64784(pc), a1
pub fn i_06477e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64782;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64784;
    16
}
/// $064782: move.l a1, (a6)
pub fn i_064782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64784;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064784: jsr $27afc.l
pub fn i_064784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6478A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6478A);
    cpu.pc = 0x27AFC;
    16
}
/// $06478A: bcc.w $64794
pub fn i_06478a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6478E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64794; return 10; }
    8
}
/// $06478E: lea.l $647c0(pc), a1
pub fn i_06478e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64792;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x647C0;
    16
}
/// $064792: move.l a1, (a6)
pub fn i_064792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64794;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064794: jsr $28d70.l
pub fn i_064794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6479A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6479A);
    cpu.pc = 0x28D70;
    16
}
/// $06479A: jsr $65c02(pc)
pub fn i_06479a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6479E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6479E);
    cpu.pc = 0x65C02;
    16
}
/// $06479E: bcc.w $647a8
pub fn i_06479e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x647A8; return 10; }
    8
}
/// $0647A2: lea.l $647c0(pc), a1
pub fn i_0647a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x647C0;
    16
}
/// $0647A6: move.l a1, (a6)
pub fn i_0647a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0647A8: subq.w #$1, $84(a6)
pub fn i_0647a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647AC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0647AC: cmpi.w #$0, $84(a6)
pub fn i_0647ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647B2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0647B2: bgt.w $647bc
pub fn i_0647b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647B6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x647BC; return 10; }
    8
}
/// $0647B6: lea.l $647c0(pc), a1
pub fn i_0647b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x647C0;
    16
}
/// $0647BA: move.l a1, (a6)
pub fn i_0647ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0647BC: bra.w $64a80
pub fn i_0647bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x64A80;
    10
}
// ===== Hostage_Pose_0647c0  ($0647C0, 186 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0647C0: move.w #$1060, d0
pub fn i_0647c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647C4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1060;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0647C4: jsr $2352.l
pub fn i_0647c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x647CA);
    cpu.pc = 0x2352;
    16
}
/// $0647CA: move.w #$0, $8e(a6)
pub fn i_0647ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0647D0: cmpi.b #$1, $78(a6)
pub fn i_0647d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647D6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0647D6: beq.w $647f6
pub fn i_0647d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x647F6; return 10; }
    8
}
/// $0647DA: move.w $28(a6), d0
pub fn i_0647da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647DE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0647DE: asr.w #$5, d0
pub fn i_0647de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647E0;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0647E0: neg.w d0
pub fn i_0647e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647E2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0647E2: move.w d0, $2c(a6)
pub fn i_0647e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647E6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0647E6: lea.l $2c6714.l, a0
pub fn i_0647e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6714;
    20
}
/// $0647EC: jsr $28cd4.l
pub fn i_0647ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x647F2);
    cpu.pc = 0x28CD4;
    16
}
/// $0647F2: bra.w $6480e
pub fn i_0647f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647F6;
    cpu.ir = 0x6000;
    cpu.pc = 0x6480E;
    10
}
/// $0647F6: move.w $28(a6), d0
pub fn i_0647f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647FA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0647FA: asr.w #$5, d0
pub fn i_0647fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647FC;
    cpu.ir = 0xEA40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $0647FC: neg.w d0
pub fn i_0647fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x647FE;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0647FE: move.w d0, $2c(a6)
pub fn i_0647fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64802;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064802: lea.l $2c6860.l, a0
pub fn i_064802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64808;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6860;
    20
}
/// $064808: jsr $28cd4.l
pub fn i_064808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6480E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6480E);
    cpu.pc = 0x28CD4;
    16
}
/// $06480E: lea.l $64814(pc), a1
pub fn i_06480e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64812;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64814;
    16
}
/// $064812: move.l a1, (a6)
pub fn i_064812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64814;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064814: jsr $27afc.l
pub fn i_064814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6481A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6481A);
    cpu.pc = 0x27AFC;
    16
}
/// $06481A: cmpi.w #$0, $28(a6)
pub fn i_06481a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64820;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $064820: bne.w $64828
pub fn i_064820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64824;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64828; return 10; }
    8
}
/// $064824: clr.w $2c(a6)
pub fn i_064824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64828;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $064828: move.b $106f28.l, d0
pub fn i_064828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6482E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $06482E: andi.b #$7, d0
pub fn i_06482e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64832;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $064832: bne.w $6485c
pub fn i_064832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64836;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6485C; return 10; }
    8
}
/// $064836: lea.l $65aa4(pc), a1
pub fn i_064836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6483A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65AA4;
    16
}
/// $06483A: jsr $4ae.l
pub fn i_06483a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64840);
    cpu.pc = 0x4AE;
    16
}
/// $064840: jsr $5dd02.l
pub fn i_064840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64846);
    cpu.pc = 0x5DD02;
    16
}
/// $064846: move.w #$18, d0
pub fn i_064846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6484A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06484A: jsr $65d32(pc)
pub fn i_06484a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6484E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6484E);
    cpu.pc = 0x65D32;
    16
}
/// $06484E: add.w d0, $22(a0)
pub fn i_06484e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64852;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $064852: move.w $2c(a6), $28(a0)
pub fn i_064852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64858;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $064858: addq.w #$1, $38(a0)
pub fn i_064858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6485C;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06485C: jsr $28d70.l
pub fn i_06485c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64862;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64862);
    cpu.pc = 0x28D70;
    16
}
/// $064862: bcc.w $64876
pub fn i_064862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64866;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64876; return 10; }
    8
}
/// $064866: cmpi.w #$0, $28(a6)
pub fn i_064866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6486C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06486C: bne.w $64876
pub fn i_06486c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64870;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64876; return 10; }
    8
}
/// $064870: lea.l $6487a(pc), a1
pub fn i_064870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6487A;
    16
}
/// $064874: move.l a1, (a6)
pub fn i_064874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64876;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064876: bra.w $64a80
pub fn i_064876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6487A;
    cpu.ir = 0x6000;
    cpu.pc = 0x64A80;
    10
}
// ===== Hostage_Kneel_06487a  ($06487A, 190 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06487A: move.w #$0, $8e(a6)
pub fn i_06487a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64880;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064880: jsr $267e2.l
pub fn i_064880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64886;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64886);
    cpu.pc = 0x267E2;
    16
}
/// $064886: cmpi.b #$1, $78(a6)
pub fn i_064886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6488C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06488C: beq.w $648f2
pub fn i_06488c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64890;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x648F2; return 10; }
    8
}
/// $064890: move.w #$a, $70(a6)
pub fn i_064890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64896;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064896: lea.l $2c674c.l, a0
pub fn i_064896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6489C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C674C;
    20
}
/// $06489C: jsr $28cd4.l
pub fn i_06489c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x648A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0648A2: lea.l $648a8(pc), a1
pub fn i_0648a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x648A8;
    16
}
/// $0648A6: move.l a1, (a6)
pub fn i_0648a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0648A8: subq.w #$1, $70(a6)
pub fn i_0648a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648AC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0648AC: cmpi.w #$0, $70(a6)
pub fn i_0648ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648B2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0648B2: bgt.w $648c2
pub fn i_0648b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648B6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x648C2; return 10; }
    8
}
/// $0648B6: move.b #$8, $20(a6)
pub fn i_0648b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0648BC: lea.l $648c2(pc), a1
pub fn i_0648bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x648C2;
    16
}
/// $0648C0: move.l a1, (a6)
pub fn i_0648c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0648C2: jsr $2783a.l
pub fn i_0648c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x648C8);
    cpu.pc = 0x2783A;
    16
}
/// $0648C8: jsr $28d70.l
pub fn i_0648c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x648CE);
    cpu.pc = 0x28D70;
    16
}
/// $0648CE: cmpi.b #$4, $21(a6)
pub fn i_0648ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $0648D4: bne.w $648de
pub fn i_0648d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x648DE; return 10; }
    8
}
/// $0648D8: lea.l $64938(pc), a1
pub fn i_0648d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64938;
    16
}
/// $0648DC: move.l a1, (a6)
pub fn i_0648dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0648DE: cmpi.w #$0, $22(a6)
pub fn i_0648de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648E4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0648E4: bgt.w $648ee
pub fn i_0648e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648E8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x648EE; return 10; }
    8
}
/// $0648E8: lea.l $64d7a(pc), a1
pub fn i_0648e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $0648EC: move.l a1, (a6)
pub fn i_0648ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0648EE: bra.w $64a80
pub fn i_0648ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x64A80;
    10
}
/// $0648F2: lea.l $2b9aa0.l, a0
pub fn i_0648f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9AA0;
    20
}
/// $0648F8: jsr $799de.l
pub fn i_0648f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x648FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x648FE);
    cpu.pc = 0x799DE;
    16
}
/// $0648FE: move.w d0, $70(a6)
pub fn i_0648fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64902;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064902: lea.l $2c68e8.l, a0
pub fn i_064902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64908;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C68E8;
    20
}
/// $064908: jsr $28cd4.l
pub fn i_064908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6490E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6490E);
    cpu.pc = 0x28CD4;
    16
}
/// $06490E: lea.l $64914(pc), a1
pub fn i_06490e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64912;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64914;
    16
}
/// $064912: move.l a1, (a6)
pub fn i_064912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64914;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064914: jsr $2783a.l
pub fn i_064914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6491A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6491A);
    cpu.pc = 0x2783A;
    16
}
/// $06491A: jsr $28d70.l
pub fn i_06491a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64920;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64920);
    cpu.pc = 0x28D70;
    16
}
/// $064920: subq.w #$1, $70(a6)
pub fn i_064920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64924;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $064924: cmpi.w #$0, $70(a6)
pub fn i_064924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6492A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06492A: bgt.w $64934
pub fn i_06492a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6492E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x64934; return 10; }
    8
}
/// $06492E: lea.l $64938(pc), a1
pub fn i_06492e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64932;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64938;
    16
}
/// $064932: move.l a1, (a6)
pub fn i_064932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64934;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064934: bra.w $64a80
pub fn i_064934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64938;
    cpu.ir = 0x6000;
    cpu.pc = 0x64A80;
    10
}
// ===== Hostage_Stand_064938  ($064938, 86 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064938: move.w #$105e, d0
pub fn i_064938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6493C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x105E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06493C: jsr $2352.l
pub fn i_06493c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64942);
    cpu.pc = 0x2352;
    16
}
/// $064942: move.w #$0, $8e(a6)
pub fn i_064942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64948;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064948: cmpi.b #$1, $78(a6)
pub fn i_064948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6494E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06494E: beq.w $64962
pub fn i_06494e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64952;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64962; return 10; }
    8
}
/// $064952: lea.l $2c675c.l, a0
pub fn i_064952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64958;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C675C;
    20
}
/// $064958: jsr $28cd4.l
pub fn i_064958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6495E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6495E);
    cpu.pc = 0x28CD4;
    16
}
/// $06495E: bra.w $6496e
pub fn i_06495e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64962;
    cpu.ir = 0x6000;
    cpu.pc = 0x6496E;
    10
}
/// $064962: lea.l $2c6906.l, a0
pub fn i_064962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64968;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6906;
    20
}
/// $064968: jsr $28cd4.l
pub fn i_064968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6496E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6496E);
    cpu.pc = 0x28CD4;
    16
}
/// $06496E: lea.l $64974(pc), a1
pub fn i_06496e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64972;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64974;
    16
}
/// $064972: move.l a1, (a6)
pub fn i_064972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64974;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064974: jsr $27afc.l
pub fn i_064974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6497A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6497A);
    cpu.pc = 0x27AFC;
    16
}
/// $06497A: jsr $28d70.l
pub fn i_06497a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64980;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64980);
    cpu.pc = 0x28D70;
    16
}
/// $064980: bcc.w $6498a
pub fn i_064980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64984;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6498A; return 10; }
    8
}
/// $064984: lea.l $6498e(pc), a1
pub fn i_064984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64988;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6498E;
    16
}
/// $064988: move.l a1, (a6)
pub fn i_064988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6498A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06498A: bra.w $64a80
pub fn i_06498a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6498E;
    cpu.ir = 0x6000;
    cpu.pc = 0x64A80;
    10
}
// ===== Hostage_Shoot_06498e  ($06498E, 150 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06498E: lea.l $650fe(pc), a1
pub fn i_06498e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64992;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x650FE;
    16
}
/// $064992: jsr $4ae.l
pub fn i_064992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64998;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64998);
    cpu.pc = 0x4AE;
    16
}
/// $064998: jsr $5dd02.l
pub fn i_064998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6499E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6499E);
    cpu.pc = 0x5DD02;
    16
}
/// $06499E: lea.l $2b9a1e.l, a0
pub fn i_06499e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649A4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9A1E;
    20
}
/// $0649A4: jsr $799de.l
pub fn i_0649a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x649AA);
    cpu.pc = 0x799DE;
    16
}
/// $0649AA: jsr $65d32(pc)
pub fn i_0649aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x649AE);
    cpu.pc = 0x65D32;
    16
}
/// $0649AE: neg.w d0
pub fn i_0649ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649B0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0649B0: move.w d0, $82(a6)
pub fn i_0649b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649B4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0649B4: move.l #$2c6310, $4c(a6)
pub fn i_0649b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649BC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C6310;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0649BC: move.w #$1, $8e(a6)
pub fn i_0649bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0649C2: lea.l $2c664e.l, a0
pub fn i_0649c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649C8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C664E;
    20
}
/// $0649C8: jsr $28cd4.l
pub fn i_0649c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x649CE);
    cpu.pc = 0x28CD4;
    16
}
/// $0649CE: cmpi.b #$1, $78(a6)
pub fn i_0649ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649D4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0649D4: bne.w $649e4
pub fn i_0649d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649D8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x649E4; return 10; }
    8
}
/// $0649D8: lea.l $2c67cc.l, a0
pub fn i_0649d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C67CC;
    20
}
/// $0649DE: jsr $28cd4.l
pub fn i_0649de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x649E4);
    cpu.pc = 0x28CD4;
    16
}
/// $0649E4: lea.l $649ea(pc), a1
pub fn i_0649e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x649EA;
    16
}
/// $0649E8: move.l a1, (a6)
pub fn i_0649e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0649EA: jsr $27afc.l
pub fn i_0649ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x649F0);
    cpu.pc = 0x27AFC;
    16
}
/// $0649F0: bcc.w $649fa
pub fn i_0649f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x649FA; return 10; }
    8
}
/// $0649F4: lea.l $64a24(pc), a1
pub fn i_0649f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649F8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64A24;
    16
}
/// $0649F8: move.l a1, (a6)
pub fn i_0649f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649FA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0649FA: jsr $65c30(pc)
pub fn i_0649fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x649FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x649FE);
    cpu.pc = 0x65C30;
    16
}
/// $0649FE: jsr $28d70.l
pub fn i_0649fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A04);
    cpu.pc = 0x28D70;
    16
}
/// $064A04: jsr $283ca.l
pub fn i_064a04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A0A);
    cpu.pc = 0x283CA;
    16
}
/// $064A0A: jsr $283d8.l
pub fn i_064a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A10);
    cpu.pc = 0x283D8;
    16
}
/// $064A10: btst.b #$1, $13(a6)
pub fn i_064a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A16;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $064A16: beq.w $64a20
pub fn i_064a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64A20; return 10; }
    8
}
/// $064A1A: lea.l $64a24(pc), a1
pub fn i_064a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A1E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64A24;
    16
}
/// $064A1E: move.l a1, (a6)
pub fn i_064a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A20;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064A20: bra.w $64a80
pub fn i_064a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A24;
    cpu.ir = 0x6000;
    cpu.pc = 0x64A80;
    10
}
// ===== Hostage_Die_064a24  ($064A24, 84 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064A24: move.w #$1021, d0
pub fn i_064a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A28;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064A28: jsr $2352.l
pub fn i_064a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A2E);
    cpu.pc = 0x2352;
    16
}
/// $064A2E: move.w #$0, $8e(a6)
pub fn i_064a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A34;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064A34: bclr.b #$1, $12(a6)
pub fn i_064a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A3A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $064A3A: lea.l $2c66e2.l, a0
pub fn i_064a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C66E2;
    20
}
/// $064A40: jsr $28cd4.l
pub fn i_064a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A46);
    cpu.pc = 0x28CD4;
    16
}
/// $064A46: lea.l $64a4c(pc), a1
pub fn i_064a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64A4C;
    16
}
/// $064A4A: move.l a1, (a6)
pub fn i_064a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064A4C: jsr $2783a.l
pub fn i_064a4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A52);
    cpu.pc = 0x2783A;
    16
}
/// $064A52: jsr $28d70.l
pub fn i_064a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A58);
    cpu.pc = 0x28D70;
    16
}
/// $064A58: bcc.w $64a62
pub fn i_064a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64A62; return 10; }
    8
}
/// $064A5C: lea.l $64d7a(pc), a1
pub fn i_064a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $064A60: move.l a1, (a6)
pub fn i_064a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064A62: movea.l #$ffffffff, a0
pub fn i_064a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A68;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $064A68: lea.l $2c63b8.l, a0
pub fn i_064a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C63B8;
    20
}
/// $064A6E: jsr $5dd5c.l
pub fn i_064a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A74);
    cpu.pc = 0x5DD5C;
    16
}
/// $064A74: bcc.w $64a7e
pub fn i_064a74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64A7E; return 10; }
    8
}
// ===== SetTaskHandler_064a78  ($064A78, 8 B, kernel/task_handlers.c)
/// $064A78: lea.l $64d7a(pc), a1
pub fn i_064a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $064A7C: move.l a1, (a6)
pub fn i_064a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064A7E: rts
pub fn i_064a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A80;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_HitCheck_064a80  ($064A80, 80 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064A80: jsr $27eba.l
pub fn i_064a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64A86);
    cpu.pc = 0x27EBA;
    16
}
/// $064A86: bcc.w $64a90
pub fn i_064a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A8A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64A90; return 10; }
    8
}
/// $064A8A: lea.l $64a24(pc), a1
pub fn i_064a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64A24;
    16
}
/// $064A8E: move.l a1, (a6)
pub fn i_064a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064A90: clr.b $21(a6)
pub fn i_064a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A94;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $064A94: jsr $65d7a(pc)
pub fn i_064a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64A98);
    cpu.pc = 0x65D7A;
    16
}
/// $064A98: lea.l $5e766.l, a0
pub fn i_064a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64A9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $064A9E: jsr $5e770.l
pub fn i_064a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64AA4);
    cpu.pc = 0x5E770;
    16
}
/// $064AA4: jsr $28758.l
pub fn i_064aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64AAA);
    cpu.pc = 0x28758;
    16
}
/// $064AAA: bcc.w $64ab4
pub fn i_064aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64AB4; return 10; }
    8
}
/// $064AAE: lea.l $64a24(pc), a1
pub fn i_064aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64A24;
    16
}
/// $064AB2: move.l a1, (a6)
pub fn i_064ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064AB4: bclr.b #$3, $13(a6)
pub fn i_064ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64ABA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $064ABA: movea.l #$ffffffff, a0
pub fn i_064aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AC0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $064AC0: lea.l $2c63b8.l, a0
pub fn i_064ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AC6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C63B8;
    20
}
/// $064AC6: jsr $5dd5c.l
pub fn i_064ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64ACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64ACC);
    cpu.pc = 0x5DD5C;
    16
}
/// $064ACC: bcc.w $64ad6
pub fn i_064acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AD0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64AD6; return 10; }
    8
}
// ===== SetTaskHandler_064ad0  ($064AD0, 8 B, kernel/task_handlers.c)
/// $064AD0: lea.l $64d7a(pc), a1
pub fn i_064ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AD4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $064AD4: move.l a1, (a6)
pub fn i_064ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AD6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064AD6: rts
pub fn i_064ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_Counter_064ad8  ($064AD8, 56 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064AD8: lea.l $64ade(pc), a1
pub fn i_064ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64ADC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64ADE;
    16
}
/// $064ADC: move.l a1, (a6)
pub fn i_064adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64ADE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064ADE: cmpi.b #$0, $98(a6)
pub fn i_064ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AE4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $064AE4: bne.w $64aee
pub fn i_064ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64AEE; return 10; }
    8
}
/// $064AE8: lea.l $64b18(pc), a1
pub fn i_064ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64B18;
    16
}
/// $064AEC: move.l a1, (a6)
pub fn i_064aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064AEE: jsr $65d4a(pc)
pub fn i_064aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64AF2);
    cpu.pc = 0x65D4A;
    16
}
/// $064AF2: cmp.b $98(a6), d0
pub fn i_064af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AF6;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $064AF6: bge.w $64b04
pub fn i_064af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AFA;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x64B04; return 10; }
    8
}
/// $064AFA: lea.l $64b18(pc), a1
pub fn i_064afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64AFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64B18;
    16
}
/// $064AFE: move.l a1, (a6)
pub fn i_064afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B00;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064B00: jsr $65d7a(pc)
pub fn i_064b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64B04);
    cpu.pc = 0x65D7A;
    16
}
/// $064B04: cmpi.b #$35, $10e27b.l
pub fn i_064b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B0C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x10E27B) as u32;
    sub_with_flags(cpu, v2, 0x35, Size::Byte, false);
    20
}
/// $064B0C: bne.w $64b16
pub fn i_064b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64B16; return 10; }
    8
}
// ===== SetTaskHandler_064b10  ($064B10, 8 B, kernel/task_handlers.c)
/// $064B10: lea.l $64d7a(pc), a1
pub fn i_064b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $064B14: move.l a1, (a6)
pub fn i_064b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064B16: rts
pub fn i_064b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_Run_064b18  ($064B18, 174 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064B18: move.l #$2c6118, $48(a6)
pub fn i_064b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B20;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C6118;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $064B20: move.w #$105e, d0
pub fn i_064b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B24;
    cpu.ir = 0x303C;
    let v1: u32 = 0x105E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064B24: jsr $2352.l
pub fn i_064b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64B2A);
    cpu.pc = 0x2352;
    16
}
/// $064B2A: move.b $9a(a6), d0
pub fn i_064b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B2E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $064B2E: andi.w #$1f, d0
pub fn i_064b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B32;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $064B32: lsl.w #$4, d0
pub fn i_064b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B34;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $064B34: move.w d0, $80(a6)
pub fn i_064b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B38;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064B38: lea.l $2b960e.l, a0
pub fn i_064b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B3E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B960E;
    20
}
/// $064B3E: jsr $799de.l
pub fn i_064b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64B44);
    cpu.pc = 0x799DE;
    16
}
/// $064B44: jsr $65d32(pc)
pub fn i_064b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64B48);
    cpu.pc = 0x65D32;
    16
}
/// $064B48: neg.w d0
pub fn i_064b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B4A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $064B4A: move.w d0, $28(a6)
pub fn i_064b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B4E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064B4E: move.w d0, $82(a6)
pub fn i_064b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B52;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064B52: lea.l $2c6962.l, a0
pub fn i_064b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B58;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6962;
    20
}
/// $064B58: jsr $28cd4.l
pub fn i_064b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64B5E);
    cpu.pc = 0x28CD4;
    16
}
/// $064B5E: lea.l $64b64(pc), a1
pub fn i_064b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64B64;
    16
}
/// $064B62: move.l a1, (a6)
pub fn i_064b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064B64: jsr $27afc.l
pub fn i_064b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64B6A);
    cpu.pc = 0x27AFC;
    16
}
/// $064B6A: bcc.w $64b74
pub fn i_064b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B6E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64B74; return 10; }
    8
}
/// $064B6E: lea.l $64bc6(pc), a1
pub fn i_064b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64BC6;
    16
}
/// $064B72: move.l a1, (a6)
pub fn i_064b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064B74: jsr $65c30(pc)
pub fn i_064b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B78;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64B78);
    cpu.pc = 0x65C30;
    16
}
/// $064B78: jsr $28d70.l
pub fn i_064b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64B7E);
    cpu.pc = 0x28D70;
    16
}
/// $064B7E: move.w $22(a6), d0
pub fn i_064b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B82;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $064B82: btst.b #$0, $3a(a6)
pub fn i_064b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B88;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $064B88: beq.w $64b98
pub fn i_064b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64B98; return 10; }
    8
}
/// $064B8C: cmp.w $80(a6), d0
pub fn i_064b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B90;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $064B90: blt.w $64ba6
pub fn i_064b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B94;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x64BA6; return 10; }
    8
}
/// $064B94: bra.w $64ba0
pub fn i_064b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B98;
    cpu.ir = 0x6000;
    cpu.pc = 0x64BA0;
    10
}
/// $064B98: cmp.w $80(a6), d0
pub fn i_064b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64B9C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $064B9C: bgt.w $64ba6
pub fn i_064b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BA0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x64BA6; return 10; }
    8
}
/// $064BA0: lea.l $64bc6(pc), a1
pub fn i_064ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64BC6;
    16
}
/// $064BA4: move.l a1, (a6)
pub fn i_064ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064BA6: jsr $283ca.l
pub fn i_064ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64BAC);
    cpu.pc = 0x283CA;
    16
}
/// $064BAC: jsr $283d8.l
pub fn i_064bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64BB2);
    cpu.pc = 0x283D8;
    16
}
/// $064BB2: btst.b #$1, $13(a6)
pub fn i_064bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BB8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $064BB8: beq.w $64bc2
pub fn i_064bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BBC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64BC2; return 10; }
    8
}
/// $064BBC: lea.l $64d10(pc), a1
pub fn i_064bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D10;
    16
}
/// $064BC0: move.l a1, (a6)
pub fn i_064bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064BC2: bra.w $64a90
pub fn i_064bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x64A90;
    10
}
// ===== Hostage_Fall_064bc6  ($064BC6, 54 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064BC6: move.l #$2c6364, $4c(a6)
pub fn i_064bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BCE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2C6364;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $064BCE: lea.l $2c6990.l, a0
pub fn i_064bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6990;
    20
}
/// $064BD4: jsr $28cd4.l
pub fn i_064bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64BDA);
    cpu.pc = 0x28CD4;
    16
}
/// $064BDA: lea.l $64be0(pc), a1
pub fn i_064bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64BE0;
    16
}
/// $064BDE: move.l a1, (a6)
pub fn i_064bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064BE0: jsr $27cee.l
pub fn i_064be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64BE6);
    cpu.pc = 0x27CEE;
    16
}
/// $064BE6: jsr $65c30(pc)
pub fn i_064be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BEA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64BEA);
    cpu.pc = 0x65C30;
    16
}
/// $064BEA: jsr $28d70.l
pub fn i_064bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64BF0);
    cpu.pc = 0x28D70;
    16
}
/// $064BF0: bcc.w $64bfa
pub fn i_064bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BF4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64BFA; return 10; }
    8
}
/// $064BF4: lea.l $64bfc(pc), a1
pub fn i_064bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BF8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64BFC;
    16
}
/// $064BF8: move.l a1, (a6)
pub fn i_064bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BFA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064BFA: bra.b $64ba6
pub fn i_064bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64BFC;
    cpu.ir = 0x60AA;
    cpu.pc = 0x64BA6;
    10
}
// ===== Hostage_Thrown_064bfc  ($064BFC, 234 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064BFC: move.w #$105c, d0
pub fn i_064bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C00;
    cpu.ir = 0x303C;
    let v1: u32 = 0x105C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064C00: jsr $2352.l
pub fn i_064c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64C06);
    cpu.pc = 0x2352;
    16
}
/// $064C06: lea.l $651be(pc), a1
pub fn i_064c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x651BE;
    16
}
/// $064C0A: jsr $4ae.l
pub fn i_064c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64C10);
    cpu.pc = 0x4AE;
    16
}
/// $064C10: jsr $5dd02.l
pub fn i_064c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64C16);
    cpu.pc = 0x5DD02;
    16
}
/// $064C16: move.w #$18, d0
pub fn i_064c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C1A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064C1A: btst.b #$0, $3a(a6)
pub fn i_064c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C20;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $064C20: beq.w $64c26
pub fn i_064c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64C26; return 10; }
    8
}
/// $064C24: neg.w d0
pub fn i_064c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C26;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $064C26: add.w d0, $22(a0)
pub fn i_064c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C2A;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $064C2A: addq.w #$8, $24(a0)
pub fn i_064c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C2E;
    cpu.ir = 0x5068;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $064C2E: move.w $28(a6), d0
pub fn i_064c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C32;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $064C32: asr.w #$1, d0
pub fn i_064c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C34;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $064C34: move.w d0, $28(a0)
pub fn i_064c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C38;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064C38: move.w #$aa7, $2a(a6)
pub fn i_064c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xAA7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064C3E: move.w #$fed1, $2e(a6)
pub fn i_064c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C44;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFED1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064C44: move.w $28(a6), d0
pub fn i_064c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C48;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $064C48: asr.w #$2, d0
pub fn i_064c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C4A;
    cpu.ir = 0xE440;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $064C4A: move.w d0, $28(a6)
pub fn i_064c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C4E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064C4E: addi.w #$20, $24(a6)
pub fn i_064c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C54;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $064C54: move.w #$d000, $38(a6)
pub fn i_064c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C5A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064C5A: lea.l $2c69ee.l, a0
pub fn i_064c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C69EE;
    20
}
/// $064C60: jsr $28cd4.l
pub fn i_064c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64C66);
    cpu.pc = 0x28CD4;
    16
}
/// $064C66: lea.l $64c6c(pc), a1
pub fn i_064c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64C6C;
    16
}
/// $064C6A: move.l a1, (a6)
pub fn i_064c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064C6C: jsr $27d50.l
pub fn i_064c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64C72);
    cpu.pc = 0x27D50;
    16
}
/// $064C72: bcc.w $64c7c
pub fn i_064c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C76;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64C7C; return 10; }
    8
}
/// $064C76: lea.l $64cee(pc), a1
pub fn i_064c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64CEE;
    16
}
/// $064C7A: move.l a1, (a6)
pub fn i_064c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C7C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064C7C: jsr $28d70.l
pub fn i_064c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64C82);
    cpu.pc = 0x28D70;
    16
}
/// $064C82: cmpi.w #$120, $24(a6)
pub fn i_064c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $064C88: bgt.w $64c92
pub fn i_064c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C8C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x64C92; return 10; }
    8
}
/// $064C8C: lea.l $64cee(pc), a1
pub fn i_064c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64CEE;
    16
}
/// $064C90: move.l a1, (a6)
pub fn i_064c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064C92: jsr $283ca.l
pub fn i_064c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64C98);
    cpu.pc = 0x283CA;
    16
}
/// $064C98: jsr $283d8.l
pub fn i_064c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64C9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64C9E);
    cpu.pc = 0x283D8;
    16
}
/// $064C9E: btst.b #$1, $13(a6)
pub fn i_064c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CA4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $064CA4: beq.w $64cae
pub fn i_064ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64CAE; return 10; }
    8
}
/// $064CA8: lea.l $64d1e(pc), a1
pub fn i_064ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D1E;
    16
}
/// $064CAC: move.l a1, (a6)
pub fn i_064cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064CAE: bclr.b #$3, $13(a6)
pub fn i_064cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CB4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $064CB4: lea.l $5e766.l, a0
pub fn i_064cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CBA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $064CBA: jsr $5e770.l
pub fn i_064cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64CC0);
    cpu.pc = 0x5E770;
    16
}
/// $064CC0: jsr $28758.l
pub fn i_064cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64CC6);
    cpu.pc = 0x28758;
    16
}
/// $064CC6: bcc.w $64cd0
pub fn i_064cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64CD0; return 10; }
    8
}
/// $064CCA: lea.l $64d1e(pc), a1
pub fn i_064cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D1E;
    16
}
/// $064CCE: move.l a1, (a6)
pub fn i_064cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064CD0: movea.l #$ffffffff, a0
pub fn i_064cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CD6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $064CD6: lea.l $2c63b8.l, a0
pub fn i_064cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C63B8;
    20
}
/// $064CDC: jsr $5dd5c.l
pub fn i_064cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64CE2);
    cpu.pc = 0x5DD5C;
    16
}
/// $064CE2: bcc.w $64cec
pub fn i_064ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64CEC; return 10; }
    8
}
// ===== SetTaskHandler_064ce6  ($064CE6, 8 B, kernel/task_handlers.c)
/// $064CE6: lea.l $64d7a(pc), a1
pub fn i_064ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $064CEA: move.l a1, (a6)
pub fn i_064cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064CEC: rts
pub fn i_064cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_Explode_064cee  ($064CEE, 34 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064CEE: move.w #$120, $24(a6)
pub fn i_064cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CF4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064CF4: move.w #$f000, d0
pub fn i_064cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064CF8: jsr $28134.l
pub fn i_064cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64CFE);
    cpu.pc = 0x28134;
    16
}
/// $064CFE: andi.w #$ffe3, $38(a6)
pub fn i_064cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D04;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $064D04: ori.w #$1c, $38(a6)
pub fn i_064d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D0A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $064D0A: jmp $78890.l
pub fn i_064d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D10;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x78890;
    8
}
// ===== Hostage_Music_064d10  ($064D10, 14 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064D10: move.w #$1023, d0
pub fn i_064d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D14;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064D14: jsr $2352.l
pub fn i_064d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64D1A);
    cpu.pc = 0x2352;
    16
}
/// $064D1A: bra.w $64a2e
pub fn i_064d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D1E;
    cpu.ir = 0x6000;
    cpu.pc = 0x64A2E;
    10
}
// ===== Hostage_Wreck_064d1e  ($064D1E, 84 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064D1E: move.w #$1023, d0
pub fn i_064d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D22;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1023;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064D22: jsr $2352.l
pub fn i_064d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64D28);
    cpu.pc = 0x2352;
    16
}
/// $064D28: move.w #$0, $8e(a6)
pub fn i_064d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D2E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064D2E: bclr.b #$1, $12(a6)
pub fn i_064d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D34;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $064D34: lea.l $2c66fe.l, a0
pub fn i_064d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D3A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C66FE;
    20
}
/// $064D3A: jsr $28cd4.l
pub fn i_064d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64D40);
    cpu.pc = 0x28CD4;
    16
}
/// $064D40: lea.l $64d46(pc), a1
pub fn i_064d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D46;
    16
}
/// $064D44: move.l a1, (a6)
pub fn i_064d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064D46: jsr $2783a.l
pub fn i_064d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64D4C);
    cpu.pc = 0x2783A;
    16
}
/// $064D4C: jsr $28d70.l
pub fn i_064d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64D52);
    cpu.pc = 0x28D70;
    16
}
/// $064D52: bcc.w $64d5c
pub fn i_064d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D56;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64D5C; return 10; }
    8
}
/// $064D56: lea.l $64d7a(pc), a1
pub fn i_064d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $064D5A: move.l a1, (a6)
pub fn i_064d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064D5C: movea.l #$ffffffff, a0
pub fn i_064d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D62;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $064D62: lea.l $2c63b8.l, a0
pub fn i_064d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D68;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C63B8;
    20
}
/// $064D68: jsr $5dd5c.l
pub fn i_064d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64D6E);
    cpu.pc = 0x5DD5C;
    16
}
/// $064D6E: bcc.w $64d78
pub fn i_064d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64D78; return 10; }
    8
}
// ===== SetTaskHandler_064d72  ($064D72, 8 B, kernel/task_handlers.c)
/// $064D72: lea.l $64d7a(pc), a1
pub fn i_064d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $064D76: move.l a1, (a6)
pub fn i_064d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064D78: rts
pub fn i_064d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hostage_Free_064d7a  ($064D7A, 14 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064D7A: move.l #$ffffffff, $48(a6)
pub fn i_064d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D82;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $064D82: jmp $518.l
pub fn i_064d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D88;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_064d88  ($064D88, 2 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064D88: rts
pub fn i_064d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D8A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_064d8a  ($064D8A, 14 B, kernel/jsr5b6_jmp518.c)
/// $064D8A: jsr $5b6.l
pub fn i_064d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64D90);
    cpu.pc = 0x5B6;
    16
}
/// $064D90: jmp $518.l
pub fn i_064d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D96;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Patrol_Init_064d98  ($064D98, 38 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064D98: lea.l $2b9816.l, a0
pub fn i_064d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64D9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9816;
    20
}
/// $064D9E: jsr $799de.l
pub fn i_064d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64DA4);
    cpu.pc = 0x799DE;
    16
}
/// $064DA4: move.w d0, $74(a6)
pub fn i_064da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DA8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064DA8: move.w #$8, $72(a6)
pub fn i_064da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DAE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064DAE: move.b #$0, $20(a6)
pub fn i_064dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DB4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $064DB4: move.w #$45, d1
pub fn i_064db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DB8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x45;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $064DB8: jsr $236e.l
pub fn i_064db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64DBE);
    cpu.pc = 0x236E;
    16
}
// ===== Patrol_Idle_064dbe  ($064DBE, 92 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064DBE: move.b $20(a6), d0
pub fn i_064dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DC2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $064DC2: andi.w #$f, d0
pub fn i_064dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DC6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $064DC6: movea.l #$2c6510, a0
pub fn i_064dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DCC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C6510;
    cpu.a[0] = v1;
    4
}
/// $064DCC: lsl.w #$2, d0
pub fn i_064dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DCE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $064DCE: movea.l (a0, d0.w), a0
pub fn i_064dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DD2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $064DD2: cmpa.l #$ffffffff, a0
pub fn i_064dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DD8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $064DD8: beq.w $64de2
pub fn i_064dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DDC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64DE2; return 10; }
    8
}
/// $064DDC: jsr $28cd4.l
pub fn i_064ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64DE2);
    cpu.pc = 0x28CD4;
    16
}
/// $064DE2: move.b #$1, $21(a6)
pub fn i_064de2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DE8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $064DE8: lea.l $64dee(pc), a1
pub fn i_064de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64DEE;
    16
}
/// $064DEC: move.l a1, (a6)
pub fn i_064dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064DEE: jsr $65bb0(pc)
pub fn i_064dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DF2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64DF2);
    cpu.pc = 0x65BB0;
    16
}
/// $064DF2: cmpi.w #$8, d0
pub fn i_064df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DF6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x8, Size::Word, false);
    8
}
/// $064DF6: beq.w $64e8e
pub fn i_064df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DFA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64E8E; return 10; }
    8
}
/// $064DFA: cmpi.w #$9, d0
pub fn i_064dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64DFE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9, Size::Word, false);
    8
}
/// $064DFE: beq.w $65020
pub fn i_064dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65020; return 10; }
    8
}
/// $064E02: jsr $28d70.l
pub fn i_064e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64E08);
    cpu.pc = 0x28D70;
    16
}
/// $064E08: jsr $65af4(pc)
pub fn i_064e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E0C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64E0C);
    cpu.pc = 0x65AF4;
    16
}
/// $064E0C: bcc.w $64e16
pub fn i_064e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64E16; return 10; }
    8
}
/// $064E10: lea.l $64e1a(pc), a1
pub fn i_064e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64E1A;
    16
}
/// $064E14: move.l a1, (a6)
pub fn i_064e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064E16: bra.w $6503c
pub fn i_064e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6503C;
    10
}
// ===== Patrol_Walk_064e1a  ($064E1A, 178 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064E1A: subq.w #$1, $74(a6)
pub fn i_064e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E1E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $064E1E: cmpi.w #$0, $74(a6)
pub fn i_064e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $064E24: bgt.w $64e4c
pub fn i_064e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E28;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x64E4C; return 10; }
    8
}
/// $064E28: lea.l $2b9816.l, a0
pub fn i_064e28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9816;
    20
}
/// $064E2E: jsr $799de.l
pub fn i_064e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64E34);
    cpu.pc = 0x799DE;
    16
}
/// $064E34: move.w d0, $74(a6)
pub fn i_064e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E38;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064E38: lea.l $2b9898.l, a0
pub fn i_064e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E3E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9898;
    20
}
/// $064E3E: jsr $799de.l
pub fn i_064e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64E44);
    cpu.pc = 0x799DE;
    16
}
/// $064E44: move.w d0, $72(a6)
pub fn i_064e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E48;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064E48: bra.w $64e5c
pub fn i_064e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x64E5C;
    10
}
/// $064E4C: lea.l $2b991a.l, a0
pub fn i_064e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E52;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B991A;
    20
}
/// $064E52: jsr $799de.l
pub fn i_064e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64E58);
    cpu.pc = 0x799DE;
    16
}
/// $064E58: move.w d0, $72(a6)
pub fn i_064e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E5C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064E5C: lea.l $2c6c58.l, a0
pub fn i_064e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6C58;
    20
}
/// $064E62: jsr $28cd4.l
pub fn i_064e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64E68);
    cpu.pc = 0x28CD4;
    16
}
/// $064E68: lea.l $64e6e(pc), a1
pub fn i_064e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64E6E;
    16
}
/// $064E6C: move.l a1, (a6)
pub fn i_064e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064E6E: jsr $65bb0(pc)
pub fn i_064e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E72;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64E72);
    cpu.pc = 0x65BB0;
    16
}
/// $064E72: cmpi.w #$9, d0
pub fn i_064e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E76;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9, Size::Word, false);
    8
}
/// $064E76: beq.w $65020
pub fn i_064e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E7A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65020; return 10; }
    8
}
/// $064E7A: jsr $28d70.l
pub fn i_064e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64E80);
    cpu.pc = 0x28D70;
    16
}
/// $064E80: bcc.w $64e8a
pub fn i_064e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64E8A; return 10; }
    8
}
/// $064E84: lea.l $64dbe(pc), a1
pub fn i_064e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64DBE;
    16
}
/// $064E88: move.l a1, (a6)
pub fn i_064e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064E8A: bra.w $6503c
pub fn i_064e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6503C;
    10
}
/// $064E8E: move.w #$0, $88(a6)
pub fn i_064e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E94;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064E94: move.b #$2, $21(a6)
pub fn i_064e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64E9A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $064E9A: lea.l $2c6c88.l, a0
pub fn i_064e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EA0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6C88;
    20
}
/// $064EA0: jsr $28cd4.l
pub fn i_064ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64EA6);
    cpu.pc = 0x28CD4;
    16
}
/// $064EA6: lea.l $64eac(pc), a1
pub fn i_064ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64EAC;
    16
}
/// $064EAA: move.l a1, (a6)
pub fn i_064eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064EAC: jsr $65bb0(pc)
pub fn i_064eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64EB0);
    cpu.pc = 0x65BB0;
    16
}
/// $064EB0: cmpi.w #$9, d0
pub fn i_064eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EB4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9, Size::Word, false);
    8
}
/// $064EB4: beq.w $65020
pub fn i_064eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EB8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65020; return 10; }
    8
}
/// $064EB8: jsr $28d70.l
pub fn i_064eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64EBE);
    cpu.pc = 0x28D70;
    16
}
/// $064EBE: bcc.w $64ec8
pub fn i_064ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64EC8; return 10; }
    8
}
/// $064EC2: lea.l $64ecc(pc), a1
pub fn i_064ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64ECC;
    16
}
/// $064EC6: move.l a1, (a6)
pub fn i_064ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EC8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064EC8: bra.w $6503c
pub fn i_064ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64ECC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6503C;
    10
}
// ===== Patrol_Rearm_064ecc  ($064ECC, 50 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064ECC: move.w #$6, $72(a6)
pub fn i_064ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64ED2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064ED2: bra.w $64ee6
pub fn i_064ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64ED6;
    cpu.ir = 0x6000;
    cpu.pc = 0x64EE6;
    10
}
/// $064ED6: lea.l $2b9712.l, a0
pub fn i_064ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9712;
    20
}
/// $064EDC: jsr $799de.l
pub fn i_064edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64EE2);
    cpu.pc = 0x799DE;
    16
}
/// $064EE2: move.w d0, $72(a6)
pub fn i_064ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EE6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064EE6: lea.l $2b9690.l, a0
pub fn i_064ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9690;
    20
}
/// $064EEC: jsr $799de.l
pub fn i_064eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64EF2);
    cpu.pc = 0x799DE;
    16
}
/// $064EF2: move.w d0, $74(a6)
pub fn i_064ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EF6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064EF6: clr.w $84(a6)
pub fn i_064ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EFA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $064EFA: bra.w $64f1e
pub fn i_064efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64EFE;
    cpu.ir = 0x6000;
    cpu.pc = 0x64F1E;
    10
}
// ===== Patrol_Fire_064efe  ($064EFE, 144 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064EFE: addq.w #$1, $84(a6)
pub fn i_064efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F02;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $064F02: subq.w #$1, $74(a6)
pub fn i_064f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F06;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $064F06: cmpi.w #$0, $74(a6)
pub fn i_064f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F0C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $064F0C: ble.b $64ed6
pub fn i_064f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F0E;
    cpu.ir = 0x6FC8;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x64ED6; return 10; }
    8
}
/// $064F0E: lea.l $2b9794.l, a0
pub fn i_064f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F14;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9794;
    20
}
/// $064F14: jsr $799de.l
pub fn i_064f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64F1A);
    cpu.pc = 0x799DE;
    16
}
/// $064F1A: move.w d0, $72(a6)
pub fn i_064f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $064F1E: move.b #$3, $21(a6)
pub fn i_064f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F24;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $064F24: lea.l $2c6c9e.l, a0
pub fn i_064f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C6C9E;
    20
}
/// $064F2A: jsr $28cd4.l
pub fn i_064f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64F30);
    cpu.pc = 0x28CD4;
    16
}
/// $064F30: lea.l $64f36(pc), a1
pub fn i_064f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64F36;
    16
}
/// $064F34: move.l a1, (a6)
pub fn i_064f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064F36: jsr $65bb0(pc)
pub fn i_064f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64F3A);
    cpu.pc = 0x65BB0;
    16
}
/// $064F3A: cmpi.w #$9, d0
pub fn i_064f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F3E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9, Size::Word, false);
    8
}
/// $064F3E: beq.w $65020
pub fn i_064f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F42;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65020; return 10; }
    8
}
/// $064F42: jsr $65b36(pc)
pub fn i_064f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64F46);
    cpu.pc = 0x65B36;
    16
}
/// $064F46: bcc.w $64f6c
pub fn i_064f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F4A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64F6C; return 10; }
    8
}
/// $064F4A: subq.w #$1, $72(a6)
pub fn i_064f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F4E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $064F4E: cmpi.w #$0, $72(a6)
pub fn i_064f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F54;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $064F54: bgt.w $64f6c
pub fn i_064f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F58;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x64F6C; return 10; }
    8
}
/// $064F58: clr.w $72(a6)
pub fn i_064f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F5C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $064F5C: cmpi.w #$10, $22(a6)
pub fn i_064f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F62;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $064F62: blt.w $64f6c
pub fn i_064f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F66;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x64F6C; return 10; }
    8
}
/// $064F66: lea.l $64f8e(pc), a1
pub fn i_064f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64F8E;
    16
}
/// $064F6A: move.l a1, (a6)
pub fn i_064f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064F6C: cmpi.w #$8, $88(a6)
pub fn i_064f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F72;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $064F72: bcs.w $64f84
pub fn i_064f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F76;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x64F84; return 10; }
    8
}
/// $064F76: nop
pub fn i_064f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F78;
    cpu.ir = 0x4E71;
    4
}
/// $064F78: nop
pub fn i_064f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F7A;
    cpu.ir = 0x4E71;
    4
}
/// $064F7A: cmpi.w #$8, $88(a6)
pub fn i_064f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F80;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x8, Size::Word, false);
    16
}
/// $064F80: nop
pub fn i_064f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F82;
    cpu.ir = 0x4E71;
    4
}
/// $064F82: trap #$f
pub fn i_064f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F84;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $064F84: jsr $28d70.l
pub fn i_064f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64F8A);
    cpu.pc = 0x28D70;
    16
}
/// $064F8A: bra.w $6503c
pub fn i_064f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6503C;
    10
}
// ===== Patrol_Turn_064f8e  ($064F8E, 166 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $064F8E: move.w $88(a6), d0
pub fn i_064f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F92;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $064F92: lea.l $2c65ac.l, a0
pub fn i_064f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C65AC;
    20
}
/// $064F98: andi.w #$7, d0
pub fn i_064f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F9C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $064F9C: add.w d0, d0
pub fn i_064f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64F9E;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $064F9E: move.w (a0, d0.w), d0
pub fn i_064f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FA2;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $064FA2: cmpi.w #$ffff, d0
pub fn i_064fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FA6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $064FA6: bne.w $64fb6
pub fn i_064fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FAA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x64FB6; return 10; }
    8
}
/// $064FAA: nop
pub fn i_064faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FAC;
    cpu.ir = 0x4E71;
    4
}
/// $064FAC: nop
pub fn i_064fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FAE;
    cpu.ir = 0x4E71;
    4
}
/// $064FAE: cmpi.w #$ffff, d0
pub fn i_064fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FB2;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $064FB2: nop
pub fn i_064fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FB4;
    cpu.ir = 0x4E71;
    4
}
/// $064FB4: trap #$f
pub fn i_064fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FB6;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $064FB6: movea.l #$2c6550, a0
pub fn i_064fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FBC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2C6550;
    cpu.a[0] = v1;
    4
}
/// $064FBC: lsl.w #$2, d0
pub fn i_064fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FBE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $064FBE: movea.l (a0, d0.w), a0
pub fn i_064fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FC2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $064FC2: cmpa.l #$ffffffff, a0
pub fn i_064fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FC8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $064FC8: beq.w $64fd2
pub fn i_064fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FCC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x64FD2; return 10; }
    8
}
/// $064FCC: jsr $28cd4.l
pub fn i_064fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64FD2);
    cpu.pc = 0x28CD4;
    16
}
/// $064FD2: lea.l $64fd8(pc), a1
pub fn i_064fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64FD8;
    16
}
/// $064FD6: move.l a1, (a6)
pub fn i_064fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FD8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064FD8: jsr $65bb0(pc)
pub fn i_064fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FDC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x64FDC);
    cpu.pc = 0x65BB0;
    16
}
/// $064FDC: cmpi.w #$9, d0
pub fn i_064fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FE0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9, Size::Word, false);
    8
}
/// $064FE0: beq.w $65020
pub fn i_064fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FE4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x65020; return 10; }
    8
}
/// $064FE4: jsr $28d70.l
pub fn i_064fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x64FEA);
    cpu.pc = 0x28D70;
    16
}
/// $064FEA: bcc.w $64ff4
pub fn i_064fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FEE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x64FF4; return 10; }
    8
}
/// $064FEE: lea.l $64efe(pc), a1
pub fn i_064fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64EFE;
    16
}
/// $064FF2: move.l a1, (a6)
pub fn i_064ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $064FF4: bra.w $6503c
pub fn i_064ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x64FF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6503C;
    10
}
/// $065020: jsr $65bb0(pc)
pub fn i_065020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65024;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x65024);
    cpu.pc = 0x65BB0;
    16
}
/// $065024: lea.l $6546a(pc), a1
pub fn i_065024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65028;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6546A;
    16
}
/// $065028: jsr $4ae.l
pub fn i_065028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6502E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6502E);
    cpu.pc = 0x4AE;
    16
}
/// $06502E: jsr $5dd02.l
pub fn i_06502e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65034);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetTaskHandler_065034  ($065034, 8 B, kernel/task_handlers.c)
/// $065034: lea.l $64d7a(pc), a1
pub fn i_065034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65038;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $065038: move.l a1, (a6)
pub fn i_065038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6503A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06503A: rts
pub fn i_06503a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6503C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_ParentCheck_06503c  ($06503C, 10 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06503C: jsr $5e45a.l
pub fn i_06503c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65042;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65042);
    cpu.pc = 0x5E45A;
    16
}
/// $065042: bcc.w $6504c
pub fn i_065042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65046;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6504C; return 10; }
    8
}
// ===== SetTaskHandler_065046  ($065046, 8 B, kernel/task_handlers.c)
/// $065046: lea.l $64d7a(pc), a1
pub fn i_065046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6504A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D7A;
    16
}
/// $06504A: move.l a1, (a6)
pub fn i_06504a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6504C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06504C: rts
pub fn i_06504c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6504E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Die_06504e  ($06504E, 98 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $06504E: move.w #$10, d1
pub fn i_06504e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65052;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065052: jsr $236e.l
pub fn i_065052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65058;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65058);
    cpu.pc = 0x236E;
    16
}
/// $065058: lea.l $2c7514.l, a0
pub fn i_065058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6505E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7514;
    20
}
/// $06505E: jsr $28cd4.l
pub fn i_06505e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65064;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65064);
    cpu.pc = 0x28CD4;
    16
}
/// $065064: lea.l $6506a(pc), a1
pub fn i_065064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65068;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6506A;
    16
}
/// $065068: move.l a1, (a6)
pub fn i_065068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6506A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06506A: jsr $5e45a.l
pub fn i_06506a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65070;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65070);
    cpu.pc = 0x5E45A;
    16
}
/// $065070: bcc.w $6507a
pub fn i_065070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65074;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6507A; return 10; }
    8
}
/// $065074: lea.l $64d8a(pc), a1
pub fn i_065074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65078;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $065078: move.l a1, (a6)
pub fn i_065078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6507A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06507A: jsr $5e506.l
pub fn i_06507a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65080;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65080);
    cpu.pc = 0x5E506;
    16
}
/// $065080: movea.l $c(a6), a0
pub fn i_065080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65084;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $065084: cmpi.b #$9, $20(a0)
pub fn i_065084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6508A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x9, Size::Byte, false);
    16
}
/// $06508A: bne.w $65094
pub fn i_06508a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6508E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x65094; return 10; }
    8
}
/// $06508E: lea.l $650b8(pc), a1
pub fn i_06508e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65092;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x650B8;
    16
}
/// $065092: move.l a1, (a6)
pub fn i_065092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65094;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065094: jsr $28d70.l
pub fn i_065094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6509A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6509A);
    cpu.pc = 0x28D70;
    16
}
/// $06509A: movea.l #$ffffffff, a0
pub fn i_06509a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650A0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0650A0: lea.l $2c63b8.l, a0
pub fn i_0650a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C63B8;
    20
}
/// $0650A6: jsr $5dd5c.l
pub fn i_0650a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x650AC);
    cpu.pc = 0x5DD5C;
    16
}
/// $0650AC: bcc.w $650b6
pub fn i_0650ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x650B6; return 10; }
    8
}
// ===== SetTaskHandler_0650b0  ($0650B0, 8 B, kernel/task_handlers.c)
/// $0650B0: lea.l $64d8a(pc), a1
pub fn i_0650b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0650B4: move.l a1, (a6)
pub fn i_0650b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0650B6: rts
pub fn i_0650b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_Wreck_0650b8  ($0650B8, 62 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0650B8: lea.l $2c7542.l, a0
pub fn i_0650b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C7542;
    20
}
/// $0650BE: jsr $28cd4.l
pub fn i_0650be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x650C4);
    cpu.pc = 0x28CD4;
    16
}
/// $0650C4: lea.l $650ca(pc), a1
pub fn i_0650c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x650CA;
    16
}
/// $0650C8: move.l a1, (a6)
pub fn i_0650c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0650CA: jsr $2783a.l
pub fn i_0650ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x650D0);
    cpu.pc = 0x2783A;
    16
}
/// $0650D0: jsr $28d70.l
pub fn i_0650d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x650D6);
    cpu.pc = 0x28D70;
    16
}
/// $0650D6: bcc.w $650e0
pub fn i_0650d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x650E0; return 10; }
    8
}
/// $0650DA: lea.l $64d8a(pc), a1
pub fn i_0650da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0650DE: move.l a1, (a6)
pub fn i_0650de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0650E0: movea.l #$ffffffff, a0
pub fn i_0650e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650E6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0650E6: lea.l $2c63b8.l, a0
pub fn i_0650e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C63B8;
    20
}
/// $0650EC: jsr $5dd5c.l
pub fn i_0650ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x650F2);
    cpu.pc = 0x5DD5C;
    16
}
/// $0650F2: bcc.w $650fc
pub fn i_0650f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x650FC; return 10; }
    8
}
// ===== SetTaskHandler_0650f6  ($0650F6, 8 B, kernel/task_handlers.c)
/// $0650F6: lea.l $64d8a(pc), a1
pub fn i_0650f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x64D8A;
    16
}
/// $0650FA: move.l a1, (a6)
pub fn i_0650fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0650FC: rts
pub fn i_0650fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x650FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Patrol_SpawnChild_0650fe  ($0650FE, 88 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0650FE: move.w #$14, d0
pub fn i_0650fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65102;
    cpu.ir = 0x303C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $065102: btst.b #$0, $3a(a6)
pub fn i_065102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65108;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $065108: beq.w $6510e
pub fn i_065108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6510C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6510E; return 10; }
    8
}
/// $06510C: neg.w d0
pub fn i_06510c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6510E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06510E: move.w d0, $90(a6)
pub fn i_06510e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65112;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $065112: lea.l $65118(pc), a1
pub fn i_065112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65116;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x65118;
    16
}
/// $065116: move.l a1, (a6)
pub fn i_065116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65118;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $065118: move.b $106f28.l, d0
pub fn i_065118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6511E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $06511E: andi.b #$3, d0
pub fn i_06511e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65122;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $065122: bne.w $6514c
pub fn i_065122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65126;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6514C; return 10; }
    8
}
/// $065126: jsr $5e506.l
pub fn i_065126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6512C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6512C);
    cpu.pc = 0x5E506;
    16
}
/// $06512C: move.w $90(a6), d0
pub fn i_06512c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65130;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $065130: add.w d0, $22(a6)
pub fn i_065130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65134;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $065134: lea.l $77f46.l, a1
pub fn i_065134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6513A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F46;
    20
}
/// $06513A: jsr $4ae.l
pub fn i_06513a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65140);
    cpu.pc = 0x4AE;
    16
}
/// $065140: jsr $5dd02.l
pub fn i_065140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65146);
    cpu.pc = 0x5DD02;
    16
}
/// $065146: move.w #$4000, $38(a0)
pub fn i_065146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6514C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06514C: jsr $5e45a.l
pub fn i_06514c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65152);
    cpu.pc = 0x5E45A;
    16
}
/// $065152: bcc.w $6515c
pub fn i_065152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65156;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6515C; return 10; }
    8
}
// ===== SetTaskHandler_065156  ($065156, 8 B, kernel/task_handlers.c)
/// $065156: lea.l $6515e(pc), a1
pub fn i_065156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6515A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6515E;
    16
}
/// $06515A: move.l a1, (a6)
pub fn i_06515a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6515C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06515C: rts
pub fn i_06515c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6515E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06515e  ($06515E, 8 B, kernel/jmp_to_scheduler.c)
/// $06515E: jmp $518.l
pub fn i_06515e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65164;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Patrol_Explode_065166  ($065166, 88 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $065166: move.w #$1022, d0
pub fn i_065166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6516A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1022;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06516A: jsr $2352.l
pub fn i_06516a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65170;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x65170);
    cpu.pc = 0x2352;
    16
}
/// $065170: bra.w $65194
pub fn i_065170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x65174;
    cpu.ir = 0x6000;
    cpu.pc = 0x65194;
    10
}
/// $065194: jsr $13600.l
pub fn i_065194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6519A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6519A);
    cpu.pc = 0x13600;
    16
}
/// $06519A: move.w #$4000, d0
pub fn i_06519a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6519E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06519E: jsr $28134.l
pub fn i_06519e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x651A4);
    cpu.pc = 0x28134;
    16
}
/// $0651A4: andi.w #$ffe3, $38(a6)
pub fn i_0651a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651AA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0651AA: ori.w #$1c, $38(a6)
pub fn i_0651aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651B0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0651B0: move.l #$ffffffff, $48(a6)
pub fn i_0651b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651B8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0651B8: jmp $77f6a.l
pub fn i_0651b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651BE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Patrol_ToSoldierFlee_0651be  ($0651BE, 40 B, enemies/soldiers/sniper_camper_mortar_062008.s)
/// $0651BE: move.w #$32a, $2a(a6)
pub fn i_0651be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x32A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0651C4: move.w #$ffaf, $2e(a6)
pub fn i_0651c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0651CA: neg.w $28(a6)
pub fn i_0651ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651CE;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0651CE: move.w #$e, d1
pub fn i_0651ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651D2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0651D2: jsr $236e.l
pub fn i_0651d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x651D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x651D8);
    cpu.pc = 0x236E;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0636AA, i_0636aa),
    (0x0636AE, i_0636ae),
    (0x0636B4, i_0636b4),
    (0x0636B8, i_0636b8),
    (0x0636BC, i_0636bc),
    (0x0636C0, i_0636c0),
    (0x0636C4, i_0636c4),
    (0x0636C8, i_0636c8),
    (0x0636CA, i_0636ca),
    (0x0636CE, i_0636ce),
    (0x0636D4, i_0636d4),
    (0x0636D8, i_0636d8),
    (0x0636DC, i_0636dc),
    (0x0636DE, i_0636de),
    (0x0636E2, i_0636e2),
    (0x0636E6, i_0636e6),
    (0x0636EA, i_0636ea),
    (0x0636F2, i_0636f2),
    (0x0636FA, i_0636fa),
    (0x063700, i_063700),
    (0x063706, i_063706),
    (0x06370C, i_06370c),
    (0x063710, i_063710),
    (0x063714, i_063714),
    (0x063718, i_063718),
    (0x06371C, i_06371c),
    (0x063722, i_063722),
    (0x063726, i_063726),
    (0x06372C, i_06372c),
    (0x063732, i_063732),
    (0x063738, i_063738),
    (0x06373C, i_06373c),
    (0x063742, i_063742),
    (0x063746, i_063746),
    (0x063748, i_063748),
    (0x06374E, i_06374e),
    (0x063752, i_063752),
    (0x063754, i_063754),
    (0x063758, i_063758),
    (0x06375E, i_06375e),
    (0x063764, i_063764),
    (0x06376A, i_06376a),
    (0x06376E, i_06376e),
    (0x063776, i_063776),
    (0x06377A, i_06377a),
    (0x063780, i_063780),
    (0x063784, i_063784),
    (0x063788, i_063788),
    (0x06378C, i_06378c),
    (0x063792, i_063792),
    (0x063798, i_063798),
    (0x06379C, i_06379c),
    (0x0637A2, i_0637a2),
    (0x0637A6, i_0637a6),
    (0x0637A8, i_0637a8),
    (0x0637AE, i_0637ae),
    (0x0637B2, i_0637b2),
    (0x0637B4, i_0637b4),
    (0x0637B8, i_0637b8),
    (0x0637BE, i_0637be),
    (0x0637C4, i_0637c4),
    (0x0637CA, i_0637ca),
    (0x0637CE, i_0637ce),
    (0x0637D2, i_0637d2),
    (0x0637D6, i_0637d6),
    (0x0637D8, i_0637d8),
    (0x0637DE, i_0637de),
    (0x0637E4, i_0637e4),
    (0x0637E8, i_0637e8),
    (0x0637EE, i_0637ee),
    (0x0637F4, i_0637f4),
    (0x0637FA, i_0637fa),
    (0x0637FE, i_0637fe),
    (0x063804, i_063804),
    (0x06380A, i_06380a),
    (0x06380E, i_06380e),
    (0x063812, i_063812),
    (0x063818, i_063818),
    (0x06381C, i_06381c),
    (0x063820, i_063820),
    (0x063824, i_063824),
    (0x063828, i_063828),
    (0x06382E, i_06382e),
    (0x063832, i_063832),
    (0x063838, i_063838),
    (0x06383C, i_06383c),
    (0x063842, i_063842),
    (0x063846, i_063846),
    (0x06384A, i_06384a),
    (0x06384C, i_06384c),
    (0x063852, i_063852),
    (0x063856, i_063856),
    (0x06385A, i_06385a),
    (0x06385C, i_06385c),
    (0x063862, i_063862),
    (0x063866, i_063866),
    (0x06386A, i_06386a),
    (0x06386C, i_06386c),
    (0x063872, i_063872),
    (0x063878, i_063878),
    (0x06387E, i_06387e),
    (0x063882, i_063882),
    (0x063886, i_063886),
    (0x063888, i_063888),
    (0x06388A, i_06388a),
    (0x063890, i_063890),
    (0x063894, i_063894),
    (0x06389A, i_06389a),
    (0x0638A0, i_0638a0),
    (0x0638A6, i_0638a6),
    (0x0638AC, i_0638ac),
    (0x0638B2, i_0638b2),
    (0x0638B6, i_0638b6),
    (0x0638B8, i_0638b8),
    (0x0638BE, i_0638be),
    (0x0638C4, i_0638c4),
    (0x0638C8, i_0638c8),
    (0x0638CC, i_0638cc),
    (0x0638D0, i_0638d0),
    (0x0638D4, i_0638d4),
    (0x0638D6, i_0638d6),
    (0x0638DC, i_0638dc),
    (0x0638E0, i_0638e0),
    (0x0638E4, i_0638e4),
    (0x0638E6, i_0638e6),
    (0x0638EC, i_0638ec),
    (0x0638F2, i_0638f2),
    (0x0638F8, i_0638f8),
    (0x0638FC, i_0638fc),
    (0x063900, i_063900),
    (0x063902, i_063902),
    (0x063904, i_063904),
    (0x063908, i_063908),
    (0x06390E, i_06390e),
    (0x063914, i_063914),
    (0x06391A, i_06391a),
    (0x063920, i_063920),
    (0x063926, i_063926),
    (0x06392C, i_06392c),
    (0x063932, i_063932),
    (0x063938, i_063938),
    (0x06393E, i_06393e),
    (0x063942, i_063942),
    (0x063948, i_063948),
    (0x06394A, i_06394a),
    (0x063952, i_063952),
    (0x063958, i_063958),
    (0x063960, i_063960),
    (0x063964, i_063964),
    (0x06396A, i_06396a),
    (0x06396E, i_06396e),
    (0x063972, i_063972),
    (0x063976, i_063976),
    (0x06397A, i_06397a),
    (0x063980, i_063980),
    (0x063984, i_063984),
    (0x06398A, i_06398a),
    (0x063990, i_063990),
    (0x063994, i_063994),
    (0x06399A, i_06399a),
    (0x0639A0, i_0639a0),
    (0x0639A6, i_0639a6),
    (0x0639AE, i_0639ae),
    (0x0639B4, i_0639b4),
    (0x0639B8, i_0639b8),
    (0x0639BE, i_0639be),
    (0x0639C4, i_0639c4),
    (0x0639CA, i_0639ca),
    (0x0639CE, i_0639ce),
    (0x0639D2, i_0639d2),
    (0x0639D6, i_0639d6),
    (0x0639DA, i_0639da),
    (0x0639E0, i_0639e0),
    (0x0639E2, i_0639e2),
    (0x0639E6, i_0639e6),
    (0x0639EC, i_0639ec),
    (0x0639F0, i_0639f0),
    (0x0639F6, i_0639f6),
    (0x0639FA, i_0639fa),
    (0x0639FC, i_0639fc),
    (0x063A00, i_063a00),
    (0x063A04, i_063a04),
    (0x063A0A, i_063a0a),
    (0x063A0E, i_063a0e),
    (0x063A12, i_063a12),
    (0x063A16, i_063a16),
    (0x063A18, i_063a18),
    (0x063A1C, i_063a1c),
    (0x063A22, i_063a22),
    (0x063A28, i_063a28),
    (0x063A2C, i_063a2c),
    (0x063A2E, i_063a2e),
    (0x063A32, i_063a32),
    (0x063A36, i_063a36),
    (0x063A3C, i_063a3c),
    (0x063A40, i_063a40),
    (0x063A44, i_063a44),
    (0x063A46, i_063a46),
    (0x063A4A, i_063a4a),
    (0x063A50, i_063a50),
    (0x063A54, i_063a54),
    (0x063A58, i_063a58),
    (0x063A5A, i_063a5a),
    (0x063A5E, i_063a5e),
    (0x063A62, i_063a62),
    (0x063A66, i_063a66),
    (0x063A68, i_063a68),
    (0x063A6C, i_063a6c),
    (0x063A70, i_063a70),
    (0x063A72, i_063a72),
    (0x063A78, i_063a78),
    (0x063A7E, i_063a7e),
    (0x063A82, i_063a82),
    (0x063A88, i_063a88),
    (0x063A8E, i_063a8e),
    (0x063A94, i_063a94),
    (0x063A98, i_063a98),
    (0x063A9C, i_063a9c),
    (0x063AA0, i_063aa0),
    (0x063AA4, i_063aa4),
    (0x063AA6, i_063aa6),
    (0x063AAA, i_063aaa),
    (0x063AAE, i_063aae),
    (0x063AB4, i_063ab4),
    (0x063AB8, i_063ab8),
    (0x063ABC, i_063abc),
    (0x063ABE, i_063abe),
    (0x063AC2, i_063ac2),
    (0x063AC8, i_063ac8),
    (0x063ACE, i_063ace),
    (0x063AD4, i_063ad4),
    (0x063ADA, i_063ada),
    (0x063ADE, i_063ade),
    (0x063AE0, i_063ae0),
    (0x063AE4, i_063ae4),
    (0x063AE8, i_063ae8),
    (0x063AEC, i_063aec),
    (0x063AF0, i_063af0),
    (0x063AF4, i_063af4),
    (0x063AF8, i_063af8),
    (0x063AFC, i_063afc),
    (0x063B00, i_063b00),
    (0x063B06, i_063b06),
    (0x063B0A, i_063b0a),
    (0x063B0E, i_063b0e),
    (0x063B10, i_063b10),
    (0x063B16, i_063b16),
    (0x063B1A, i_063b1a),
    (0x063B20, i_063b20),
    (0x063B26, i_063b26),
    (0x063B2C, i_063b2c),
    (0x063B30, i_063b30),
    (0x063B34, i_063b34),
    (0x063B36, i_063b36),
    (0x063B38, i_063b38),
    (0x063B3C, i_063b3c),
    (0x063B42, i_063b42),
    (0x063B46, i_063b46),
    (0x063B4C, i_063b4c),
    (0x063B50, i_063b50),
    (0x063B54, i_063b54),
    (0x063B56, i_063b56),
    (0x063B58, i_063b58),
    (0x063B5A, i_063b5a),
    (0x063B5C, i_063b5c),
    (0x063B62, i_063b62),
    (0x063B68, i_063b68),
    (0x063B6C, i_063b6c),
    (0x063B70, i_063b70),
    (0x063B76, i_063b76),
    (0x063B7C, i_063b7c),
    (0x063B82, i_063b82),
    (0x063B86, i_063b86),
    (0x063B88, i_063b88),
    (0x063B8E, i_063b8e),
    (0x063B92, i_063b92),
    (0x063B96, i_063b96),
    (0x063B98, i_063b98),
    (0x063B9E, i_063b9e),
    (0x063BA4, i_063ba4),
    (0x063BAA, i_063baa),
    (0x063BAE, i_063bae),
    (0x063BB2, i_063bb2),
    (0x063BB4, i_063bb4),
    (0x063BB6, i_063bb6),
    (0x063BBC, i_063bbc),
    (0x063BC2, i_063bc2),
    (0x063BCA, i_063bca),
    (0x063BCE, i_063bce),
    (0x063BD0, i_063bd0),
    (0x063BD6, i_063bd6),
    (0x063BDC, i_063bdc),
    (0x063BE0, i_063be0),
    (0x063BE6, i_063be6),
    (0x063BEA, i_063bea),
    (0x063BF0, i_063bf0),
    (0x063BF4, i_063bf4),
    (0x063BF8, i_063bf8),
    (0x063BFA, i_063bfa),
    (0x063C00, i_063c00),
    (0x063C04, i_063c04),
    (0x063C08, i_063c08),
    (0x063C0A, i_063c0a),
    (0x063C0C, i_063c0c),
    (0x063C12, i_063c12),
    (0x063C16, i_063c16),
    (0x063C18, i_063c18),
    (0x063C1C, i_063c1c),
    (0x063C1E, i_063c1e),
    (0x063C22, i_063c22),
    (0x063C24, i_063c24),
    (0x063C2A, i_063c2a),
    (0x063C2C, i_063c2c),
    (0x063C30, i_063c30),
    (0x063C34, i_063c34),
    (0x063C38, i_063c38),
    (0x063C3C, i_063c3c),
    (0x063C3E, i_063c3e),
    (0x063C42, i_063c42),
    (0x063C44, i_063c44),
    (0x063C48, i_063c48),
    (0x063C4E, i_063c4e),
    (0x063C52, i_063c52),
    (0x063C56, i_063c56),
    (0x063C58, i_063c58),
    (0x063C5C, i_063c5c),
    (0x063C60, i_063c60),
    (0x063C64, i_063c64),
    (0x063C66, i_063c66),
    (0x063C6A, i_063c6a),
    (0x063C6E, i_063c6e),
    (0x063C70, i_063c70),
    (0x063C74, i_063c74),
    (0x063C76, i_063c76),
    (0x063C78, i_063c78),
    (0x063C7A, i_063c7a),
    (0x063C7C, i_063c7c),
    (0x063C82, i_063c82),
    (0x063C86, i_063c86),
    (0x063C8A, i_063c8a),
    (0x063C8C, i_063c8c),
    (0x063C92, i_063c92),
    (0x063C98, i_063c98),
    (0x063C9C, i_063c9c),
    (0x063CA0, i_063ca0),
    (0x063CA2, i_063ca2),
    (0x063CA4, i_063ca4),
    (0x063CA8, i_063ca8),
    (0x063CAA, i_063caa),
    (0x063CAE, i_063cae),
    (0x063CB0, i_063cb0),
    (0x063CB2, i_063cb2),
    (0x063CB4, i_063cb4),
    (0x063CB8, i_063cb8),
    (0x063CBE, i_063cbe),
    (0x063CC2, i_063cc2),
    (0x063CC8, i_063cc8),
    (0x063CCC, i_063ccc),
    (0x063CCE, i_063cce),
    (0x063CD2, i_063cd2),
    (0x063CD4, i_063cd4),
    (0x063CDA, i_063cda),
    (0x063CE0, i_063ce0),
    (0x063CE6, i_063ce6),
    (0x063CEA, i_063cea),
    (0x063CEE, i_063cee),
    (0x063CF0, i_063cf0),
    (0x063CF4, i_063cf4),
    (0x063CF8, i_063cf8),
    (0x063CFC, i_063cfc),
    (0x063D02, i_063d02),
    (0x063D06, i_063d06),
    (0x063D08, i_063d08),
    (0x063D0C, i_063d0c),
    (0x063D10, i_063d10),
    (0x063D14, i_063d14),
    (0x063D18, i_063d18),
    (0x063D1A, i_063d1a),
    (0x063D20, i_063d20),
    (0x063D24, i_063d24),
    (0x063D28, i_063d28),
    (0x063D2A, i_063d2a),
    (0x063D2E, i_063d2e),
    (0x063D34, i_063d34),
    (0x063D38, i_063d38),
    (0x063D3C, i_063d3c),
    (0x063D40, i_063d40),
    (0x063D44, i_063d44),
    (0x063D48, i_063d48),
    (0x063D4C, i_063d4c),
    (0x063D52, i_063d52),
    (0x063D56, i_063d56),
    (0x063D5A, i_063d5a),
    (0x063D5E, i_063d5e),
    (0x063D62, i_063d62),
    (0x063D66, i_063d66),
    (0x063D6C, i_063d6c),
    (0x063D6E, i_063d6e),
    (0x063D72, i_063d72),
    (0x063D76, i_063d76),
    (0x063D7C, i_063d7c),
    (0x063D80, i_063d80),
    (0x063D82, i_063d82),
    (0x063D88, i_063d88),
    (0x063D8C, i_063d8c),
    (0x063D90, i_063d90),
    (0x063D96, i_063d96),
    (0x063D9C, i_063d9c),
    (0x063DA2, i_063da2),
    (0x063DA8, i_063da8),
    (0x063DAC, i_063dac),
    (0x063DAE, i_063dae),
    (0x063DB0, i_063db0),
    (0x063DB2, i_063db2),
    (0x063DB6, i_063db6),
    (0x063DBC, i_063dbc),
    (0x063DC2, i_063dc2),
    (0x063DC6, i_063dc6),
    (0x063DCC, i_063dcc),
    (0x063DD2, i_063dd2),
    (0x063DD8, i_063dd8),
    (0x063DDE, i_063dde),
    (0x063DE4, i_063de4),
    (0x063DEA, i_063dea),
    (0x063DF0, i_063df0),
    (0x063DF2, i_063df2),
    (0x063DF6, i_063df6),
    (0x063DFC, i_063dfc),
    (0x063E02, i_063e02),
    (0x063E06, i_063e06),
    (0x063E0C, i_063e0c),
    (0x063E12, i_063e12),
    (0x063E18, i_063e18),
    (0x063E1E, i_063e1e),
    (0x063E24, i_063e24),
    (0x063E2A, i_063e2a),
    (0x063E2C, i_063e2c),
    (0x063E32, i_063e32),
    (0x063E36, i_063e36),
    (0x063E3A, i_063e3a),
    (0x063E3C, i_063e3c),
    (0x063E44, i_063e44),
    (0x063E4A, i_063e4a),
    (0x063E50, i_063e50),
    (0x063E58, i_063e58),
    (0x063E5E, i_063e5e),
    (0x063E62, i_063e62),
    (0x063E66, i_063e66),
    (0x063E6C, i_063e6c),
    (0x063E72, i_063e72),
    (0x063E76, i_063e76),
    (0x063E7C, i_063e7c),
    (0x063E80, i_063e80),
    (0x063E84, i_063e84),
    (0x063E8A, i_063e8a),
    (0x063E90, i_063e90),
    (0x063E92, i_063e92),
    (0x063E98, i_063e98),
    (0x063E9E, i_063e9e),
    (0x063EA4, i_063ea4),
    (0x063EA8, i_063ea8),
    (0x063EAA, i_063eaa),
    (0x063EAE, i_063eae),
    (0x063EB2, i_063eb2),
    (0x063EB6, i_063eb6),
    (0x063EBA, i_063eba),
    (0x063EBE, i_063ebe),
    (0x063EC0, i_063ec0),
    (0x063EC4, i_063ec4),
    (0x063EC6, i_063ec6),
    (0x063ECC, i_063ecc),
    (0x063ED4, i_063ed4),
    (0x063EE6, i_063ee6),
    (0x063EEA, i_063eea),
    (0x063EF0, i_063ef0),
    (0x063EF6, i_063ef6),
    (0x063EFC, i_063efc),
    (0x063F02, i_063f02),
    (0x063F08, i_063f08),
    (0x063F0C, i_063f0c),
    (0x063F12, i_063f12),
    (0x063F18, i_063f18),
    (0x063F1C, i_063f1c),
    (0x063F1E, i_063f1e),
    (0x063F22, i_063f22),
    (0x063F26, i_063f26),
    (0x063F2A, i_063f2a),
    (0x063F2E, i_063f2e),
    (0x063F34, i_063f34),
    (0x063F3A, i_063f3a),
    (0x063F40, i_063f40),
    (0x063F46, i_063f46),
    (0x063F4C, i_063f4c),
    (0x063F52, i_063f52),
    (0x063F58, i_063f58),
    (0x063F5C, i_063f5c),
    (0x063F60, i_063f60),
    (0x063F64, i_063f64),
    (0x063F66, i_063f66),
    (0x063F6A, i_063f6a),
    (0x063F70, i_063f70),
    (0x063F76, i_063f76),
    (0x063F7A, i_063f7a),
    (0x063F7C, i_063f7c),
    (0x063F82, i_063f82),
    (0x063F88, i_063f88),
    (0x063F8C, i_063f8c),
    (0x063F90, i_063f90),
    (0x063F94, i_063f94),
    (0x063F98, i_063f98),
    (0x063F9A, i_063f9a),
    (0x063F9E, i_063f9e),
    (0x063FA4, i_063fa4),
    (0x063FAA, i_063faa),
    (0x063FB0, i_063fb0),
    (0x063FB4, i_063fb4),
    (0x063FB6, i_063fb6),
    (0x063FBC, i_063fbc),
    (0x063FC2, i_063fc2),
    (0x063FC6, i_063fc6),
    (0x063FCA, i_063fca),
    (0x063FCC, i_063fcc),
    (0x063FD0, i_063fd0),
    (0x063FD6, i_063fd6),
    (0x063FDA, i_063fda),
    (0x063FDE, i_063fde),
    (0x063FE2, i_063fe2),
    (0x063FE8, i_063fe8),
    (0x063FEE, i_063fee),
    (0x063FF2, i_063ff2),
    (0x063FF8, i_063ff8),
    (0x063FFE, i_063ffe),
    (0x064002, i_064002),
    (0x064006, i_064006),
    (0x06400A, i_06400a),
    (0x064010, i_064010),
    (0x064016, i_064016),
    (0x06401A, i_06401a),
    (0x064020, i_064020),
    (0x064026, i_064026),
    (0x06402A, i_06402a),
    (0x06402C, i_06402c),
    (0x064032, i_064032),
    (0x064038, i_064038),
    (0x06403C, i_06403c),
    (0x064040, i_064040),
    (0x064044, i_064044),
    (0x064046, i_064046),
    (0x06404C, i_06404c),
    (0x064050, i_064050),
    (0x064056, i_064056),
    (0x06405A, i_06405a),
    (0x06405E, i_06405e),
    (0x064062, i_064062),
    (0x064066, i_064066),
    (0x064068, i_064068),
    (0x06406C, i_06406c),
    (0x064070, i_064070),
    (0x064074, i_064074),
    (0x06407A, i_06407a),
    (0x06407E, i_06407e),
    (0x064082, i_064082),
    (0x064086, i_064086),
    (0x06408C, i_06408c),
    (0x064092, i_064092),
    (0x064096, i_064096),
    (0x06409C, i_06409c),
    (0x0640A2, i_0640a2),
    (0x0640A6, i_0640a6),
    (0x0640AA, i_0640aa),
    (0x0640AE, i_0640ae),
    (0x0640B4, i_0640b4),
    (0x0640BA, i_0640ba),
    (0x0640BE, i_0640be),
    (0x0640C4, i_0640c4),
    (0x0640CA, i_0640ca),
    (0x0640D0, i_0640d0),
    (0x0640D4, i_0640d4),
    (0x0640D6, i_0640d6),
    (0x0640DC, i_0640dc),
    (0x0640E2, i_0640e2),
    (0x0640E6, i_0640e6),
    (0x0640EA, i_0640ea),
    (0x0640EC, i_0640ec),
    (0x0640F0, i_0640f0),
    (0x0640F4, i_0640f4),
    (0x0640FA, i_0640fa),
    (0x064100, i_064100),
    (0x064104, i_064104),
    (0x06410A, i_06410a),
    (0x064110, i_064110),
    (0x064114, i_064114),
    (0x06411A, i_06411a),
    (0x06411E, i_06411e),
    (0x064122, i_064122),
    (0x064126, i_064126),
    (0x06412C, i_06412c),
    (0x064132, i_064132),
    (0x064136, i_064136),
    (0x06413C, i_06413c),
    (0x064142, i_064142),
    (0x064146, i_064146),
    (0x06414A, i_06414a),
    (0x06414E, i_06414e),
    (0x064154, i_064154),
    (0x06415A, i_06415a),
    (0x06415E, i_06415e),
    (0x064164, i_064164),
    (0x06416A, i_06416a),
    (0x06416E, i_06416e),
    (0x064170, i_064170),
    (0x064176, i_064176),
    (0x06417C, i_06417c),
    (0x064180, i_064180),
    (0x064184, i_064184),
    (0x064186, i_064186),
    (0x06418A, i_06418a),
    (0x06418E, i_06418e),
    (0x064194, i_064194),
    (0x064198, i_064198),
    (0x06419C, i_06419c),
    (0x06419E, i_06419e),
    (0x0641A4, i_0641a4),
    (0x0641AA, i_0641aa),
    (0x0641AE, i_0641ae),
    (0x0641CA, i_0641ca),
    (0x0641CE, i_0641ce),
    (0x0641D0, i_0641d0),
    (0x0641D2, i_0641d2),
    (0x0641D8, i_0641d8),
    (0x0641DC, i_0641dc),
    (0x0641E2, i_0641e2),
    (0x0641E8, i_0641e8),
    (0x0641EE, i_0641ee),
    (0x0641F4, i_0641f4),
    (0x0641FA, i_0641fa),
    (0x0641FE, i_0641fe),
    (0x064202, i_064202),
    (0x064204, i_064204),
    (0x06420A, i_06420a),
    (0x064210, i_064210),
    (0x064216, i_064216),
    (0x06421A, i_06421a),
    (0x06421E, i_06421e),
    (0x064220, i_064220),
    (0x064222, i_064222),
    (0x06422A, i_06422a),
    (0x064230, i_064230),
    (0x064238, i_064238),
    (0x06423C, i_06423c),
    (0x064242, i_064242),
    (0x064248, i_064248),
    (0x06424E, i_06424e),
    (0x064256, i_064256),
    (0x06425C, i_06425c),
    (0x064260, i_064260),
    (0x064264, i_064264),
    (0x064266, i_064266),
    (0x064268, i_064268),
    (0x06426A, i_06426a),
    (0x06426C, i_06426c),
    (0x064272, i_064272),
    (0x064278, i_064278),
    (0x06427E, i_06427e),
    (0x064282, i_064282),
    (0x064286, i_064286),
    (0x06428A, i_06428a),
    (0x064290, i_064290),
    (0x064298, i_064298),
    (0x06429C, i_06429c),
    (0x0642A2, i_0642a2),
    (0x0642A6, i_0642a6),
    (0x0642AC, i_0642ac),
    (0x0642B2, i_0642b2),
    (0x0642B6, i_0642b6),
    (0x0642BC, i_0642bc),
    (0x0642C2, i_0642c2),
    (0x0642C8, i_0642c8),
    (0x0642CE, i_0642ce),
    (0x0642D4, i_0642d4),
    (0x0642D8, i_0642d8),
    (0x0642DA, i_0642da),
    (0x0642E0, i_0642e0),
    (0x0642E4, i_0642e4),
    (0x0642E8, i_0642e8),
    (0x0642EA, i_0642ea),
    (0x0642EE, i_0642ee),
    (0x0642F2, i_0642f2),
    (0x0642F6, i_0642f6),
    (0x0642FA, i_0642fa),
    (0x0642FC, i_0642fc),
    (0x0642FE, i_0642fe),
    (0x064300, i_064300),
    (0x064306, i_064306),
    (0x06430A, i_06430a),
    (0x06430E, i_06430e),
    (0x064312, i_064312),
    (0x064316, i_064316),
    (0x06431A, i_06431a),
    (0x06431E, i_06431e),
    (0x064322, i_064322),
    (0x064324, i_064324),
    (0x064328, i_064328),
    (0x06432C, i_06432c),
    (0x064334, i_064334),
    (0x064338, i_064338),
    (0x06433C, i_06433c),
    (0x064340, i_064340),
    (0x064346, i_064346),
    (0x06434C, i_06434c),
    (0x064352, i_064352),
    (0x064356, i_064356),
    (0x06435A, i_06435a),
    (0x06435C, i_06435c),
    (0x064362, i_064362),
    (0x064368, i_064368),
    (0x06436E, i_06436e),
    (0x064372, i_064372),
    (0x064376, i_064376),
    (0x064378, i_064378),
    (0x06437A, i_06437a),
    (0x06437E, i_06437e),
    (0x064384, i_064384),
    (0x064388, i_064388),
    (0x06438E, i_06438e),
    (0x064394, i_064394),
    (0x06439A, i_06439a),
    (0x0643A0, i_0643a0),
    (0x0643A6, i_0643a6),
    (0x0643AA, i_0643aa),
    (0x0643AC, i_0643ac),
    (0x0643B2, i_0643b2),
    (0x0643B8, i_0643b8),
    (0x0643BC, i_0643bc),
    (0x0643C0, i_0643c0),
    (0x0643C2, i_0643c2),
    (0x0643C8, i_0643c8),
    (0x0643CE, i_0643ce),
    (0x0643D2, i_0643d2),
    (0x0643D6, i_0643d6),
    (0x0643D8, i_0643d8),
    (0x0643DA, i_0643da),
    (0x0643DE, i_0643de),
    (0x0643E4, i_0643e4),
    (0x0643E8, i_0643e8),
    (0x0643EC, i_0643ec),
    (0x0643EE, i_0643ee),
    (0x0643F2, i_0643f2),
    (0x0643F4, i_0643f4),
    (0x0643FA, i_0643fa),
    (0x064400, i_064400),
    (0x064406, i_064406),
    (0x06440A, i_06440a),
    (0x064410, i_064410),
    (0x064414, i_064414),
    (0x064416, i_064416),
    (0x06441C, i_06441c),
    (0x064420, i_064420),
    (0x064422, i_064422),
    (0x064426, i_064426),
    (0x06442C, i_06442c),
    (0x064430, i_064430),
    (0x064434, i_064434),
    (0x064436, i_064436),
    (0x06443A, i_06443a),
    (0x06443C, i_06443c),
    (0x064442, i_064442),
    (0x064446, i_064446),
    (0x06444A, i_06444a),
    (0x06444E, i_06444e),
    (0x064452, i_064452),
    (0x064456, i_064456),
    (0x06445C, i_06445c),
    (0x064462, i_064462),
    (0x064468, i_064468),
    (0x06446E, i_06446e),
    (0x064474, i_064474),
    (0x06447A, i_06447a),
    (0x06447E, i_06447e),
    (0x064480, i_064480),
    (0x064484, i_064484),
    (0x064486, i_064486),
    (0x06448C, i_06448c),
    (0x064490, i_064490),
    (0x064492, i_064492),
    (0x064496, i_064496),
    (0x064498, i_064498),
    (0x06449C, i_06449c),
    (0x06449E, i_06449e),
    (0x0644A2, i_0644a2),
    (0x0644A8, i_0644a8),
    (0x0644AC, i_0644ac),
    (0x0644B2, i_0644b2),
    (0x0644B8, i_0644b8),
    (0x0644BE, i_0644be),
    (0x0644C2, i_0644c2),
    (0x0644C8, i_0644c8),
    (0x0644CE, i_0644ce),
    (0x0644D4, i_0644d4),
    (0x0644DA, i_0644da),
    (0x0644E0, i_0644e0),
    (0x0644E6, i_0644e6),
    (0x0644E8, i_0644e8),
    (0x0644EC, i_0644ec),
    (0x0644F2, i_0644f2),
    (0x0644F6, i_0644f6),
    (0x0644FC, i_0644fc),
    (0x064502, i_064502),
    (0x064506, i_064506),
    (0x06450C, i_06450c),
    (0x064512, i_064512),
    (0x064518, i_064518),
    (0x06451E, i_06451e),
    (0x064524, i_064524),
    (0x064526, i_064526),
    (0x06452C, i_06452c),
    (0x064532, i_064532),
    (0x064534, i_064534),
    (0x064538, i_064538),
    (0x06453C, i_06453c),
    (0x064540, i_064540),
    (0x064544, i_064544),
    (0x064548, i_064548),
    (0x06454A, i_06454a),
    (0x06454E, i_06454e),
    (0x064550, i_064550),
    (0x064556, i_064556),
    (0x06455C, i_06455c),
    (0x064562, i_064562),
    (0x064566, i_064566),
    (0x064568, i_064568),
    (0x064572, i_064572),
    (0x06457C, i_06457c),
    (0x064582, i_064582),
    (0x064588, i_064588),
    (0x06458A, i_06458a),
    (0x064592, i_064592),
    (0x06459A, i_06459a),
    (0x0645A0, i_0645a0),
    (0x0645A8, i_0645a8),
    (0x0645AE, i_0645ae),
    (0x0645B4, i_0645b4),
    (0x0645B8, i_0645b8),
    (0x0645BE, i_0645be),
    (0x0645C4, i_0645c4),
    (0x0645CA, i_0645ca),
    (0x0645D0, i_0645d0),
    (0x0645D4, i_0645d4),
    (0x0645DA, i_0645da),
    (0x0645DE, i_0645de),
    (0x0645E4, i_0645e4),
    (0x0645EA, i_0645ea),
    (0x0645F0, i_0645f0),
    (0x0645F4, i_0645f4),
    (0x064640, i_064640),
    (0x064644, i_064644),
    (0x06464A, i_06464a),
    (0x064650, i_064650),
    (0x064656, i_064656),
    (0x06465C, i_06465c),
    (0x064660, i_064660),
    (0x064666, i_064666),
    (0x06466A, i_06466a),
    (0x064670, i_064670),
    (0x064676, i_064676),
    (0x06467C, i_06467c),
    (0x064680, i_064680),
    (0x064682, i_064682),
    (0x064686, i_064686),
    (0x06468A, i_06468a),
    (0x06468C, i_06468c),
    (0x064690, i_064690),
    (0x064694, i_064694),
    (0x064698, i_064698),
    (0x06469A, i_06469a),
    (0x06469E, i_06469e),
    (0x0646A4, i_0646a4),
    (0x0646A8, i_0646a8),
    (0x0646AA, i_0646aa),
    (0x0646B0, i_0646b0),
    (0x0646B4, i_0646b4),
    (0x0646B8, i_0646b8),
    (0x0646BA, i_0646ba),
    (0x0646BE, i_0646be),
    (0x0646C2, i_0646c2),
    (0x0646C6, i_0646c6),
    (0x0646CA, i_0646ca),
    (0x0646CC, i_0646cc),
    (0x0646D0, i_0646d0),
    (0x0646D4, i_0646d4),
    (0x0646DA, i_0646da),
    (0x0646E2, i_0646e2),
    (0x0646E6, i_0646e6),
    (0x0646EA, i_0646ea),
    (0x0646EC, i_0646ec),
    (0x0646F4, i_0646f4),
    (0x0646F8, i_0646f8),
    (0x0646FC, i_0646fc),
    (0x0646FE, i_0646fe),
    (0x064700, i_064700),
    (0x064706, i_064706),
    (0x06470A, i_06470a),
    (0x06470E, i_06470e),
    (0x064714, i_064714),
    (0x06471A, i_06471a),
    (0x064720, i_064720),
    (0x064724, i_064724),
    (0x064728, i_064728),
    (0x06472E, i_06472e),
    (0x064732, i_064732),
    (0x064738, i_064738),
    (0x06473C, i_06473c),
    (0x064742, i_064742),
    (0x064748, i_064748),
    (0x06474E, i_06474e),
    (0x064752, i_064752),
    (0x064754, i_064754),
    (0x064758, i_064758),
    (0x06475C, i_06475c),
    (0x064762, i_064762),
    (0x064768, i_064768),
    (0x06476E, i_06476e),
    (0x064772, i_064772),
    (0x064778, i_064778),
    (0x06477E, i_06477e),
    (0x064782, i_064782),
    (0x064784, i_064784),
    (0x06478A, i_06478a),
    (0x06478E, i_06478e),
    (0x064792, i_064792),
    (0x064794, i_064794),
    (0x06479A, i_06479a),
    (0x06479E, i_06479e),
    (0x0647A2, i_0647a2),
    (0x0647A6, i_0647a6),
    (0x0647A8, i_0647a8),
    (0x0647AC, i_0647ac),
    (0x0647B2, i_0647b2),
    (0x0647B6, i_0647b6),
    (0x0647BA, i_0647ba),
    (0x0647BC, i_0647bc),
    (0x0647C0, i_0647c0),
    (0x0647C4, i_0647c4),
    (0x0647CA, i_0647ca),
    (0x0647D0, i_0647d0),
    (0x0647D6, i_0647d6),
    (0x0647DA, i_0647da),
    (0x0647DE, i_0647de),
    (0x0647E0, i_0647e0),
    (0x0647E2, i_0647e2),
    (0x0647E6, i_0647e6),
    (0x0647EC, i_0647ec),
    (0x0647F2, i_0647f2),
    (0x0647F6, i_0647f6),
    (0x0647FA, i_0647fa),
    (0x0647FC, i_0647fc),
    (0x0647FE, i_0647fe),
    (0x064802, i_064802),
    (0x064808, i_064808),
    (0x06480E, i_06480e),
    (0x064812, i_064812),
    (0x064814, i_064814),
    (0x06481A, i_06481a),
    (0x064820, i_064820),
    (0x064824, i_064824),
    (0x064828, i_064828),
    (0x06482E, i_06482e),
    (0x064832, i_064832),
    (0x064836, i_064836),
    (0x06483A, i_06483a),
    (0x064840, i_064840),
    (0x064846, i_064846),
    (0x06484A, i_06484a),
    (0x06484E, i_06484e),
    (0x064852, i_064852),
    (0x064858, i_064858),
    (0x06485C, i_06485c),
    (0x064862, i_064862),
    (0x064866, i_064866),
    (0x06486C, i_06486c),
    (0x064870, i_064870),
    (0x064874, i_064874),
    (0x064876, i_064876),
    (0x06487A, i_06487a),
    (0x064880, i_064880),
    (0x064886, i_064886),
    (0x06488C, i_06488c),
    (0x064890, i_064890),
    (0x064896, i_064896),
    (0x06489C, i_06489c),
    (0x0648A2, i_0648a2),
    (0x0648A6, i_0648a6),
    (0x0648A8, i_0648a8),
    (0x0648AC, i_0648ac),
    (0x0648B2, i_0648b2),
    (0x0648B6, i_0648b6),
    (0x0648BC, i_0648bc),
    (0x0648C0, i_0648c0),
    (0x0648C2, i_0648c2),
    (0x0648C8, i_0648c8),
    (0x0648CE, i_0648ce),
    (0x0648D4, i_0648d4),
    (0x0648D8, i_0648d8),
    (0x0648DC, i_0648dc),
    (0x0648DE, i_0648de),
    (0x0648E4, i_0648e4),
    (0x0648E8, i_0648e8),
    (0x0648EC, i_0648ec),
    (0x0648EE, i_0648ee),
    (0x0648F2, i_0648f2),
    (0x0648F8, i_0648f8),
    (0x0648FE, i_0648fe),
    (0x064902, i_064902),
    (0x064908, i_064908),
    (0x06490E, i_06490e),
    (0x064912, i_064912),
    (0x064914, i_064914),
    (0x06491A, i_06491a),
    (0x064920, i_064920),
    (0x064924, i_064924),
    (0x06492A, i_06492a),
    (0x06492E, i_06492e),
    (0x064932, i_064932),
    (0x064934, i_064934),
    (0x064938, i_064938),
    (0x06493C, i_06493c),
    (0x064942, i_064942),
    (0x064948, i_064948),
    (0x06494E, i_06494e),
    (0x064952, i_064952),
    (0x064958, i_064958),
    (0x06495E, i_06495e),
    (0x064962, i_064962),
    (0x064968, i_064968),
    (0x06496E, i_06496e),
    (0x064972, i_064972),
    (0x064974, i_064974),
    (0x06497A, i_06497a),
    (0x064980, i_064980),
    (0x064984, i_064984),
    (0x064988, i_064988),
    (0x06498A, i_06498a),
    (0x06498E, i_06498e),
    (0x064992, i_064992),
    (0x064998, i_064998),
    (0x06499E, i_06499e),
    (0x0649A4, i_0649a4),
    (0x0649AA, i_0649aa),
    (0x0649AE, i_0649ae),
    (0x0649B0, i_0649b0),
    (0x0649B4, i_0649b4),
    (0x0649BC, i_0649bc),
    (0x0649C2, i_0649c2),
    (0x0649C8, i_0649c8),
    (0x0649CE, i_0649ce),
    (0x0649D4, i_0649d4),
    (0x0649D8, i_0649d8),
    (0x0649DE, i_0649de),
    (0x0649E4, i_0649e4),
    (0x0649E8, i_0649e8),
    (0x0649EA, i_0649ea),
    (0x0649F0, i_0649f0),
    (0x0649F4, i_0649f4),
    (0x0649F8, i_0649f8),
    (0x0649FA, i_0649fa),
    (0x0649FE, i_0649fe),
    (0x064A04, i_064a04),
    (0x064A0A, i_064a0a),
    (0x064A10, i_064a10),
    (0x064A16, i_064a16),
    (0x064A1A, i_064a1a),
    (0x064A1E, i_064a1e),
    (0x064A20, i_064a20),
    (0x064A24, i_064a24),
    (0x064A28, i_064a28),
    (0x064A2E, i_064a2e),
    (0x064A34, i_064a34),
    (0x064A3A, i_064a3a),
    (0x064A40, i_064a40),
    (0x064A46, i_064a46),
    (0x064A4A, i_064a4a),
    (0x064A4C, i_064a4c),
    (0x064A52, i_064a52),
    (0x064A58, i_064a58),
    (0x064A5C, i_064a5c),
    (0x064A60, i_064a60),
    (0x064A62, i_064a62),
    (0x064A68, i_064a68),
    (0x064A6E, i_064a6e),
    (0x064A74, i_064a74),
    (0x064A78, i_064a78),
    (0x064A7C, i_064a7c),
    (0x064A7E, i_064a7e),
    (0x064A80, i_064a80),
    (0x064A86, i_064a86),
    (0x064A8A, i_064a8a),
    (0x064A8E, i_064a8e),
    (0x064A90, i_064a90),
    (0x064A94, i_064a94),
    (0x064A98, i_064a98),
    (0x064A9E, i_064a9e),
    (0x064AA4, i_064aa4),
    (0x064AAA, i_064aaa),
    (0x064AAE, i_064aae),
    (0x064AB2, i_064ab2),
    (0x064AB4, i_064ab4),
    (0x064ABA, i_064aba),
    (0x064AC0, i_064ac0),
    (0x064AC6, i_064ac6),
    (0x064ACC, i_064acc),
    (0x064AD0, i_064ad0),
    (0x064AD4, i_064ad4),
    (0x064AD6, i_064ad6),
    (0x064AD8, i_064ad8),
    (0x064ADC, i_064adc),
    (0x064ADE, i_064ade),
    (0x064AE4, i_064ae4),
    (0x064AE8, i_064ae8),
    (0x064AEC, i_064aec),
    (0x064AEE, i_064aee),
    (0x064AF2, i_064af2),
    (0x064AF6, i_064af6),
    (0x064AFA, i_064afa),
    (0x064AFE, i_064afe),
    (0x064B00, i_064b00),
    (0x064B04, i_064b04),
    (0x064B0C, i_064b0c),
    (0x064B10, i_064b10),
    (0x064B14, i_064b14),
    (0x064B16, i_064b16),
    (0x064B18, i_064b18),
    (0x064B20, i_064b20),
    (0x064B24, i_064b24),
    (0x064B2A, i_064b2a),
    (0x064B2E, i_064b2e),
    (0x064B32, i_064b32),
    (0x064B34, i_064b34),
    (0x064B38, i_064b38),
    (0x064B3E, i_064b3e),
    (0x064B44, i_064b44),
    (0x064B48, i_064b48),
    (0x064B4A, i_064b4a),
    (0x064B4E, i_064b4e),
    (0x064B52, i_064b52),
    (0x064B58, i_064b58),
    (0x064B5E, i_064b5e),
    (0x064B62, i_064b62),
    (0x064B64, i_064b64),
    (0x064B6A, i_064b6a),
    (0x064B6E, i_064b6e),
    (0x064B72, i_064b72),
    (0x064B74, i_064b74),
    (0x064B78, i_064b78),
    (0x064B7E, i_064b7e),
    (0x064B82, i_064b82),
    (0x064B88, i_064b88),
    (0x064B8C, i_064b8c),
    (0x064B90, i_064b90),
    (0x064B94, i_064b94),
    (0x064B98, i_064b98),
    (0x064B9C, i_064b9c),
    (0x064BA0, i_064ba0),
    (0x064BA4, i_064ba4),
    (0x064BA6, i_064ba6),
    (0x064BAC, i_064bac),
    (0x064BB2, i_064bb2),
    (0x064BB8, i_064bb8),
    (0x064BBC, i_064bbc),
    (0x064BC0, i_064bc0),
    (0x064BC2, i_064bc2),
    (0x064BC6, i_064bc6),
    (0x064BCE, i_064bce),
    (0x064BD4, i_064bd4),
    (0x064BDA, i_064bda),
    (0x064BDE, i_064bde),
    (0x064BE0, i_064be0),
    (0x064BE6, i_064be6),
    (0x064BEA, i_064bea),
    (0x064BF0, i_064bf0),
    (0x064BF4, i_064bf4),
    (0x064BF8, i_064bf8),
    (0x064BFA, i_064bfa),
    (0x064BFC, i_064bfc),
    (0x064C00, i_064c00),
    (0x064C06, i_064c06),
    (0x064C0A, i_064c0a),
    (0x064C10, i_064c10),
    (0x064C16, i_064c16),
    (0x064C1A, i_064c1a),
    (0x064C20, i_064c20),
    (0x064C24, i_064c24),
    (0x064C26, i_064c26),
    (0x064C2A, i_064c2a),
    (0x064C2E, i_064c2e),
    (0x064C32, i_064c32),
    (0x064C34, i_064c34),
    (0x064C38, i_064c38),
    (0x064C3E, i_064c3e),
    (0x064C44, i_064c44),
    (0x064C48, i_064c48),
    (0x064C4A, i_064c4a),
    (0x064C4E, i_064c4e),
    (0x064C54, i_064c54),
    (0x064C5A, i_064c5a),
    (0x064C60, i_064c60),
    (0x064C66, i_064c66),
    (0x064C6A, i_064c6a),
    (0x064C6C, i_064c6c),
    (0x064C72, i_064c72),
    (0x064C76, i_064c76),
    (0x064C7A, i_064c7a),
    (0x064C7C, i_064c7c),
    (0x064C82, i_064c82),
    (0x064C88, i_064c88),
    (0x064C8C, i_064c8c),
    (0x064C90, i_064c90),
    (0x064C92, i_064c92),
    (0x064C98, i_064c98),
    (0x064C9E, i_064c9e),
    (0x064CA4, i_064ca4),
    (0x064CA8, i_064ca8),
    (0x064CAC, i_064cac),
    (0x064CAE, i_064cae),
    (0x064CB4, i_064cb4),
    (0x064CBA, i_064cba),
    (0x064CC0, i_064cc0),
    (0x064CC6, i_064cc6),
    (0x064CCA, i_064cca),
    (0x064CCE, i_064cce),
    (0x064CD0, i_064cd0),
    (0x064CD6, i_064cd6),
    (0x064CDC, i_064cdc),
    (0x064CE2, i_064ce2),
    (0x064CE6, i_064ce6),
    (0x064CEA, i_064cea),
    (0x064CEC, i_064cec),
    (0x064CEE, i_064cee),
    (0x064CF4, i_064cf4),
    (0x064CF8, i_064cf8),
    (0x064CFE, i_064cfe),
    (0x064D04, i_064d04),
    (0x064D0A, i_064d0a),
    (0x064D10, i_064d10),
    (0x064D14, i_064d14),
    (0x064D1A, i_064d1a),
    (0x064D1E, i_064d1e),
    (0x064D22, i_064d22),
    (0x064D28, i_064d28),
    (0x064D2E, i_064d2e),
    (0x064D34, i_064d34),
    (0x064D3A, i_064d3a),
    (0x064D40, i_064d40),
    (0x064D44, i_064d44),
    (0x064D46, i_064d46),
    (0x064D4C, i_064d4c),
    (0x064D52, i_064d52),
    (0x064D56, i_064d56),
    (0x064D5A, i_064d5a),
    (0x064D5C, i_064d5c),
    (0x064D62, i_064d62),
    (0x064D68, i_064d68),
    (0x064D6E, i_064d6e),
    (0x064D72, i_064d72),
    (0x064D76, i_064d76),
    (0x064D78, i_064d78),
    (0x064D7A, i_064d7a),
    (0x064D82, i_064d82),
    (0x064D88, i_064d88),
    (0x064D8A, i_064d8a),
    (0x064D90, i_064d90),
    (0x064D98, i_064d98),
    (0x064D9E, i_064d9e),
    (0x064DA4, i_064da4),
    (0x064DA8, i_064da8),
    (0x064DAE, i_064dae),
    (0x064DB4, i_064db4),
    (0x064DB8, i_064db8),
    (0x064DBE, i_064dbe),
    (0x064DC2, i_064dc2),
    (0x064DC6, i_064dc6),
    (0x064DCC, i_064dcc),
    (0x064DCE, i_064dce),
    (0x064DD2, i_064dd2),
    (0x064DD8, i_064dd8),
    (0x064DDC, i_064ddc),
    (0x064DE2, i_064de2),
    (0x064DE8, i_064de8),
    (0x064DEC, i_064dec),
    (0x064DEE, i_064dee),
    (0x064DF2, i_064df2),
    (0x064DF6, i_064df6),
    (0x064DFA, i_064dfa),
    (0x064DFE, i_064dfe),
    (0x064E02, i_064e02),
    (0x064E08, i_064e08),
    (0x064E0C, i_064e0c),
    (0x064E10, i_064e10),
    (0x064E14, i_064e14),
    (0x064E16, i_064e16),
    (0x064E1A, i_064e1a),
    (0x064E1E, i_064e1e),
    (0x064E24, i_064e24),
    (0x064E28, i_064e28),
    (0x064E2E, i_064e2e),
    (0x064E34, i_064e34),
    (0x064E38, i_064e38),
    (0x064E3E, i_064e3e),
    (0x064E44, i_064e44),
    (0x064E48, i_064e48),
    (0x064E4C, i_064e4c),
    (0x064E52, i_064e52),
    (0x064E58, i_064e58),
    (0x064E5C, i_064e5c),
    (0x064E62, i_064e62),
    (0x064E68, i_064e68),
    (0x064E6C, i_064e6c),
    (0x064E6E, i_064e6e),
    (0x064E72, i_064e72),
    (0x064E76, i_064e76),
    (0x064E7A, i_064e7a),
    (0x064E80, i_064e80),
    (0x064E84, i_064e84),
    (0x064E88, i_064e88),
    (0x064E8A, i_064e8a),
    (0x064E8E, i_064e8e),
    (0x064E94, i_064e94),
    (0x064E9A, i_064e9a),
    (0x064EA0, i_064ea0),
    (0x064EA6, i_064ea6),
    (0x064EAA, i_064eaa),
    (0x064EAC, i_064eac),
    (0x064EB0, i_064eb0),
    (0x064EB4, i_064eb4),
    (0x064EB8, i_064eb8),
    (0x064EBE, i_064ebe),
    (0x064EC2, i_064ec2),
    (0x064EC6, i_064ec6),
    (0x064EC8, i_064ec8),
    (0x064ECC, i_064ecc),
    (0x064ED2, i_064ed2),
    (0x064ED6, i_064ed6),
    (0x064EDC, i_064edc),
    (0x064EE2, i_064ee2),
    (0x064EE6, i_064ee6),
    (0x064EEC, i_064eec),
    (0x064EF2, i_064ef2),
    (0x064EF6, i_064ef6),
    (0x064EFA, i_064efa),
    (0x064EFE, i_064efe),
    (0x064F02, i_064f02),
    (0x064F06, i_064f06),
    (0x064F0C, i_064f0c),
    (0x064F0E, i_064f0e),
    (0x064F14, i_064f14),
    (0x064F1A, i_064f1a),
    (0x064F1E, i_064f1e),
    (0x064F24, i_064f24),
    (0x064F2A, i_064f2a),
    (0x064F30, i_064f30),
    (0x064F34, i_064f34),
    (0x064F36, i_064f36),
    (0x064F3A, i_064f3a),
    (0x064F3E, i_064f3e),
    (0x064F42, i_064f42),
    (0x064F46, i_064f46),
    (0x064F4A, i_064f4a),
    (0x064F4E, i_064f4e),
    (0x064F54, i_064f54),
    (0x064F58, i_064f58),
    (0x064F5C, i_064f5c),
    (0x064F62, i_064f62),
    (0x064F66, i_064f66),
    (0x064F6A, i_064f6a),
    (0x064F6C, i_064f6c),
    (0x064F72, i_064f72),
    (0x064F76, i_064f76),
    (0x064F78, i_064f78),
    (0x064F7A, i_064f7a),
    (0x064F80, i_064f80),
    (0x064F82, i_064f82),
    (0x064F84, i_064f84),
    (0x064F8A, i_064f8a),
    (0x064F8E, i_064f8e),
    (0x064F92, i_064f92),
    (0x064F98, i_064f98),
    (0x064F9C, i_064f9c),
    (0x064F9E, i_064f9e),
    (0x064FA2, i_064fa2),
    (0x064FA6, i_064fa6),
    (0x064FAA, i_064faa),
    (0x064FAC, i_064fac),
    (0x064FAE, i_064fae),
    (0x064FB2, i_064fb2),
    (0x064FB4, i_064fb4),
    (0x064FB6, i_064fb6),
    (0x064FBC, i_064fbc),
    (0x064FBE, i_064fbe),
    (0x064FC2, i_064fc2),
    (0x064FC8, i_064fc8),
    (0x064FCC, i_064fcc),
    (0x064FD2, i_064fd2),
    (0x064FD6, i_064fd6),
    (0x064FD8, i_064fd8),
    (0x064FDC, i_064fdc),
    (0x064FE0, i_064fe0),
    (0x064FE4, i_064fe4),
    (0x064FEA, i_064fea),
    (0x064FEE, i_064fee),
    (0x064FF2, i_064ff2),
    (0x064FF4, i_064ff4),
    (0x065020, i_065020),
    (0x065024, i_065024),
    (0x065028, i_065028),
    (0x06502E, i_06502e),
    (0x065034, i_065034),
    (0x065038, i_065038),
    (0x06503A, i_06503a),
    (0x06503C, i_06503c),
    (0x065042, i_065042),
    (0x065046, i_065046),
    (0x06504A, i_06504a),
    (0x06504C, i_06504c),
    (0x06504E, i_06504e),
    (0x065052, i_065052),
    (0x065058, i_065058),
    (0x06505E, i_06505e),
    (0x065064, i_065064),
    (0x065068, i_065068),
    (0x06506A, i_06506a),
    (0x065070, i_065070),
    (0x065074, i_065074),
    (0x065078, i_065078),
    (0x06507A, i_06507a),
    (0x065080, i_065080),
    (0x065084, i_065084),
    (0x06508A, i_06508a),
    (0x06508E, i_06508e),
    (0x065092, i_065092),
    (0x065094, i_065094),
    (0x06509A, i_06509a),
    (0x0650A0, i_0650a0),
    (0x0650A6, i_0650a6),
    (0x0650AC, i_0650ac),
    (0x0650B0, i_0650b0),
    (0x0650B4, i_0650b4),
    (0x0650B6, i_0650b6),
    (0x0650B8, i_0650b8),
    (0x0650BE, i_0650be),
    (0x0650C4, i_0650c4),
    (0x0650C8, i_0650c8),
    (0x0650CA, i_0650ca),
    (0x0650D0, i_0650d0),
    (0x0650D6, i_0650d6),
    (0x0650DA, i_0650da),
    (0x0650DE, i_0650de),
    (0x0650E0, i_0650e0),
    (0x0650E6, i_0650e6),
    (0x0650EC, i_0650ec),
    (0x0650F2, i_0650f2),
    (0x0650F6, i_0650f6),
    (0x0650FA, i_0650fa),
    (0x0650FC, i_0650fc),
    (0x0650FE, i_0650fe),
    (0x065102, i_065102),
    (0x065108, i_065108),
    (0x06510C, i_06510c),
    (0x06510E, i_06510e),
    (0x065112, i_065112),
    (0x065116, i_065116),
    (0x065118, i_065118),
    (0x06511E, i_06511e),
    (0x065122, i_065122),
    (0x065126, i_065126),
    (0x06512C, i_06512c),
    (0x065130, i_065130),
    (0x065134, i_065134),
    (0x06513A, i_06513a),
    (0x065140, i_065140),
    (0x065146, i_065146),
    (0x06514C, i_06514c),
    (0x065152, i_065152),
    (0x065156, i_065156),
    (0x06515A, i_06515a),
    (0x06515C, i_06515c),
    (0x06515E, i_06515e),
    (0x065166, i_065166),
    (0x06516A, i_06516a),
    (0x065170, i_065170),
    (0x065194, i_065194),
    (0x06519A, i_06519a),
    (0x06519E, i_06519e),
    (0x0651A4, i_0651a4),
    (0x0651AA, i_0651aa),
    (0x0651B0, i_0651b0),
    (0x0651B8, i_0651b8),
    (0x0651BE, i_0651be),
    (0x0651C4, i_0651c4),
    (0x0651CA, i_0651ca),
    (0x0651CE, i_0651ce),
    (0x0651D2, i_0651d2),
];
