// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $07C538: beq.w $7c56a
pub fn i_07c538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C53C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C56A; return 10; }
    8
}
/// $07C53C: addq.w #$1, $76(a6)
pub fn i_07c53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C540;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C540: cmpi.w #$c8, $76(a6)
pub fn i_07c540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C546;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xC8, Size::Word, false);
    16
}
/// $07C546: blt.w $7c558
pub fn i_07c546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C54A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C558; return 10; }
    8
}
/// $07C54A: clr.w $76(a6)
pub fn i_07c54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C54E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C54E: addq.w #$1, $78(a6)
pub fn i_07c54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C552;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C552: andi.w #$1, $78(a6)
pub fn i_07c552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C558;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C558: lea.l $2df7ce.l, a0
pub fn i_07c558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C55E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF7CE;
    20
}
/// $07C55E: move.w $78(a6), d0
pub fn i_07c55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C562;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C562: add.w d0, d0
pub fn i_07c562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C564;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C564: add.w d0, d0
pub fn i_07c564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C566;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C566: movea.l (a0, d0.w), a0
pub fn i_07c566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C56A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07C56A: tst.w $82(a6)
pub fn i_07c56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C56E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07C56E: beq.w $7c576
pub fn i_07c56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C572;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C576; return 10; }
    8
}
/// $07C572: subq.w #$1, $82(a6)
pub fn i_07c572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C576;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C576: jsr $5e070.l
pub fn i_07c576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C57C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C57C);
    cpu.pc = 0x5E070;
    16
}
/// $07C57C: lea.l $2df818.l, a0
pub fn i_07c57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C582;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DF818;
    20
}
/// $07C582: moveq #$0, d1
pub fn i_07c582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C584;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07C584: moveq #$6, d2
pub fn i_07c584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C586;
    cpu.ir = 0x7406;
    cpu.d[2] = 0x6;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07C586: cmp.w (a0, d1.w), d0
pub fn i_07c586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C58A;
    cpu.ir = 0xB070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $07C58A: blt.w $7c596
pub fn i_07c58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C58E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C596; return 10; }
    8
}
/// $07C58E: cmp.w $2(a0, d1.w), d0
pub fn i_07c58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C592;
    cpu.ir = 0xB070;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    14
}
/// $07C592: blt.w $7c59e
pub fn i_07c592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C596;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C59E; return 10; }
    8
}
/// $07C596: addq.w #$8, d1
pub fn i_07c596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C598;
    cpu.ir = 0x5041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07C598: dbra d2, $7c586
pub fn i_07c598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C59C;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x7C59C; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x7C59C; return 14; }
    cpu.pc = 0x7C586;
    10
}
/// $07C59C: rts
pub fn i_07c59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C59E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07C59E: move.w $6(a0, d1.w), d3
pub fn i_07c59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5A2;
    cpu.ir = 0x3630;
    let ad2 = cpu.a[0].wrapping_add(0x6).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07C5A2: cmp.w $70(a6), d3
pub fn i_07c5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5A6;
    cpu.ir = 0xB66E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07C5A6: beq.w $7c5de
pub fn i_07c5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C5DE; return 10; }
    8
}
/// $07C5AA: blt.w $7c5c8
pub fn i_07c5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5AE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C5C8; return 10; }
    8
}
/// $07C5AE: addq.w #$1, $7a(a6)
pub fn i_07c5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5B2;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C5B2: cmpi.w #$5, $7a(a6)
pub fn i_07c5b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5B8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Word, false);
    16
}
/// $07C5B8: blt.w $7c5de
pub fn i_07c5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5BC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C5DE; return 10; }
    8
}
/// $07C5BC: clr.w $7a(a6)
pub fn i_07c5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5C0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C5C0: addq.w #$1, $70(a6)
pub fn i_07c5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5C4;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C5C4: bra.w $7c5de
pub fn i_07c5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x7C5DE;
    10
}
/// $07C5C8: addq.w #$1, $7a(a6)
pub fn i_07c5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5CC;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C5CC: cmpi.w #$5, $7a(a6)
pub fn i_07c5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Word, false);
    16
}
/// $07C5D2: blt.w $7c5de
pub fn i_07c5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5D6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C5DE; return 10; }
    8
}
/// $07C5D6: clr.w $7a(a6)
pub fn i_07c5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5DA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C5DA: subq.w #$1, $70(a6)
pub fn i_07c5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5DE;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C5DE: movea.l $c(a6), a1
pub fn i_07c5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5E2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07C5E2: clr.w $70(a1)
pub fn i_07c5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5E6;
    cpu.ir = 0x4269;
    let ad1 = cpu.a[1].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C5E6: move.w $4(a0, d1.w), d1
pub fn i_07c5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5EA;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07C5EA: sub.w d0, d1
pub fn i_07c5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5EC;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C5EC: bpl.w $7c604
pub fn i_07c5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5F0;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7C604; return 10; }
    8
}
/// $07C5F0: neg.w d1
pub fn i_07c5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5F2;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07C5F2: cmpi.w #$3, d1
pub fn i_07c5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5F6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $07C5F6: ble.w $7c618
pub fn i_07c5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C5FA;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7C618; return 10; }
    8
}
/// $07C5FA: move.w #$1, $70(a1)
pub fn i_07c5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C600;
    cpu.ir = 0x337C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C600: bra.w $7c612
pub fn i_07c600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C604;
    cpu.ir = 0x6000;
    cpu.pc = 0x7C612;
    10
}
/// $07C604: cmpi.w #$3, d1
pub fn i_07c604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C608;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $07C608: ble.w $7c618
pub fn i_07c608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C60C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7C618; return 10; }
    8
}
/// $07C60C: move.w #$ffff, $70(a1)
pub fn i_07c60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C612;
    cpu.ir = 0x337C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== ClearC_07c612  ($07C612, 6 B, thunks/ccr_helpers.c)
/// $07C612: andi.b #$fe, ccr
pub fn i_07c612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C616;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $07C616: rts
pub fn i_07c616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C618;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_LatchAngle_07c618  ($07C618, 10 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C618: tst.w $82(a6)
pub fn i_07c618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C61C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07C61C: bne.b $7c612
pub fn i_07c61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C61E;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C612; return 10; }
    8
}
/// $07C61E: move.w d0, $72(a6)
pub fn i_07c61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C622;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetC_07c622  ($07C622, 6 B, thunks/ccr_helpers.c)
/// $07C622: ori.b #$1, ccr
pub fn i_07c622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C626;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $07C626: rts
pub fn i_07c626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C628;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_OffWorldBox_07c628  ($07C628, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C628: lea.l $2e011a.l, a0
pub fn i_07c628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C62E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E011A;
    20
}
/// $07C62E: jsr $5dd5c.l
pub fn i_07c62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C634;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C634);
    cpu.pc = 0x5DD5C;
    16
}
/// $07C634: bcc.w $7c63e
pub fn i_07c634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C638;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C63E; return 10; }
    8
}
// ===== SetC_07c638  ($07C638, 6 B, thunks/ccr_helpers.c)
/// $07C638: ori.b #$1, ccr
pub fn i_07c638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C63C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $07C63C: rts
pub fn i_07c63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C63E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_07c63e  ($07C63E, 6 B, thunks/ccr_helpers.c)
/// $07C63E: andi.b #$fe, ccr
pub fn i_07c63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C642;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $07C642: rts
pub fn i_07c642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C644;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Turret_FreeWithParent_07c644  ($07C644, 22 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C644: move.b #$ff, $21(a6)
pub fn i_07c644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C64A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07C64A: movea.l $c(a6), a0
pub fn i_07c64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C64E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C64E: move.b #$ff, $21(a0)
pub fn i_07c64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C654;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07C654: jmp $518.l
pub fn i_07c654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C65A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M2Boss_Rts_07c65a  ($07C65A, 2 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C65A: rts
pub fn i_07c65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C65C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_07c65c  ($07C65C, 8 B, kernel/jmp_to_scheduler.c)
/// $07C65C: jmp $518.l
pub fn i_07c65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C662;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M2Boss_Shell_Init_07c664  ($07C664, 232 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C664: move.w #$122, d1
pub fn i_07c664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C668;
    cpu.ir = 0x323C;
    let v1: u32 = 0x122;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C668: jsr $236e.l
pub fn i_07c668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C66E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C66E);
    cpu.pc = 0x236E;
    16
}
/// $07C66E: lea.l $2bd058.l, a0
pub fn i_07c66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C674;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BD058;
    20
}
/// $07C674: jsr $799de.l
pub fn i_07c674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C67A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C67A);
    cpu.pc = 0x799DE;
    16
}
/// $07C67A: move.w d0, $28(a6)
pub fn i_07c67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C67E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C67E: move.w $28(a6), d1
pub fn i_07c67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C682;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C682: move.w $72(a6), d0
pub fn i_07c682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C686;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C686: jsr $13c0e.l
pub fn i_07c686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C68C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C68C);
    cpu.pc = 0x13C0E;
    16
}
/// $07C68C: move.w d1, $28(a6)
pub fn i_07c68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C690;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C690: move.w d2, $2a(a6)
pub fn i_07c690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C694;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C694: move.w #$1, $66(a6)
pub fn i_07c694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C69A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C69A: move.w #$d000, d0
pub fn i_07c69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C69E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C69E: jsr $28134.l
pub fn i_07c69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C6A4);
    cpu.pc = 0x28134;
    16
}
/// $07C6A4: andi.w #$ffe3, $38(a6)
pub fn i_07c6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6AA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C6AA: ori.w #$10, $38(a6)
pub fn i_07c6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6B0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C6B0: bset.b #$4, $6b(a6)
pub fn i_07c6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6B6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07C6B6: clr.w $70(a6)
pub fn i_07c6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6BA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C6BA: lea.l $2dfcf0.l, a0
pub fn i_07c6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DFCF0;
    20
}
/// $07C6C0: jsr $28cd4.l
pub fn i_07c6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C6C6);
    cpu.pc = 0x28CD4;
    16
}
/// $07C6C6: lea.l $7c6cc(pc), a1
pub fn i_07c6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C6CC;
    16
}
/// $07C6CA: move.l a1, (a6)
pub fn i_07c6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C6CC: jsr $27d50.l
pub fn i_07c6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C6D2);
    cpu.pc = 0x27D50;
    16
}
/// $07C6D2: bcc.w $7c6dc
pub fn i_07c6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C6DC; return 10; }
    8
}
/// $07C6D6: lea.l $7c754(pc), a1
pub fn i_07c6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C754;
    16
}
/// $07C6DA: move.l a1, (a6)
pub fn i_07c6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C6DC: jsr $28d70.l
pub fn i_07c6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C6E2);
    cpu.pc = 0x28D70;
    16
}
/// $07C6E2: jsr $283d8.l
pub fn i_07c6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C6E8);
    cpu.pc = 0x283D8;
    16
}
/// $07C6E8: btst.b #$1, $13(a6)
pub fn i_07c6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6EE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07C6EE: beq.w $7c6f8
pub fn i_07c6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C6F8; return 10; }
    8
}
/// $07C6F2: lea.l $7c808(pc), a1
pub fn i_07c6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C808;
    16
}
/// $07C6F6: move.l a1, (a6)
pub fn i_07c6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C6F8: addq.w #$1, $70(a6)
pub fn i_07c6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C6FC;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C6FC: cmpi.w #$f, $70(a6)
pub fn i_07c6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C702;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF, Size::Word, false);
    16
}
/// $07C702: blt.w $7c71a
pub fn i_07c702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C706;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7C71A; return 10; }
    8
}
/// $07C706: clr.w $70(a6)
pub fn i_07c706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C70A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C70A: lea.l $7c868(pc), a1
pub fn i_07c70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C70E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C868;
    16
}
/// $07C70E: jsr $4ae.l
pub fn i_07c70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C714);
    cpu.pc = 0x4AE;
    16
}
/// $07C714: jsr $5dd02.l
pub fn i_07c714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C71A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C71A);
    cpu.pc = 0x5DD02;
    16
}
/// $07C71A: movea.l $c(a6), a0
pub fn i_07c71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C71E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C71E: movea.l $c(a0), a0
pub fn i_07c71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C722;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C722: cmpi.b #$ff, $20(a0)
pub fn i_07c722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C728;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07C728: bne.w $7c73c
pub fn i_07c728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C72C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C73C; return 10; }
    8
}
/// $07C72C: lea.l $ffff.w, a0
pub fn i_07c72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C730;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07C730: move.l a0, $48(a6)
pub fn i_07c730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C734;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07C734: lea.l $77f6a.l, a1
pub fn i_07c734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C73A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $07C73A: move.l a1, (a6)
pub fn i_07c73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C73C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C73C: movea.l #$ffffffff, a0
pub fn i_07c73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C742;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07C742: jsr $5dd56.l
pub fn i_07c742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C748);
    cpu.pc = 0x5DD56;
    16
}
/// $07C748: bcc.w $7c752
pub fn i_07c748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C74C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C752; return 10; }
    8
}
// ===== SetTaskHandler_07c74c  ($07C74C, 8 B, kernel/task_handlers.c)
/// $07C74C: lea.l $7c65c(pc), a1
pub fn i_07c74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C750;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C750: move.l a1, (a6)
pub fn i_07c750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C752;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C752: rts
pub fn i_07c752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C754;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Shell_Impact_07c754  ($07C754, 192 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C754: move.w #$1021, d0
pub fn i_07c754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C758;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1021;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C758: jsr $2352.l
pub fn i_07c758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C75E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C75E);
    cpu.pc = 0x2352;
    16
}
/// $07C75E: jsr $434dc.l
pub fn i_07c75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C764);
    cpu.pc = 0x434DC;
    16
}
/// $07C764: lea.l $2df7aa.l, a1
pub fn i_07c764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C76A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF7AA;
    20
}
/// $07C76A: jsr $77c7e.l
pub fn i_07c76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C770;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C770);
    cpu.pc = 0x77C7E;
    16
}
/// $07C770: lea.l $7c838(pc), a1
pub fn i_07c770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C774;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C838;
    16
}
/// $07C774: jsr $4ae.l
pub fn i_07c774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C77A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C77A);
    cpu.pc = 0x4AE;
    16
}
/// $07C77A: jsr $5dd02.l
pub fn i_07c77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C780;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C780);
    cpu.pc = 0x5DD02;
    16
}
/// $07C780: lea.l $7c814(pc), a1
pub fn i_07c780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C784;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C814;
    16
}
/// $07C784: jsr $4ae.l
pub fn i_07c784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C78A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C78A);
    cpu.pc = 0x4AE;
    16
}
/// $07C78A: jsr $5dd02.l
pub fn i_07c78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C790);
    cpu.pc = 0x5DD02;
    16
}
/// $07C790: addi.w #$20, $24(a0)
pub fn i_07c790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C796;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C796: move.w #$4, $88(a0)
pub fn i_07c796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C79C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C79C: move.w #$2000, $38(a0)
pub fn i_07c79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7A2;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C7A2: lea.l $7c814(pc), a1
pub fn i_07c7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C814;
    16
}
/// $07C7A6: jsr $4ae.l
pub fn i_07c7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C7AC);
    cpu.pc = 0x4AE;
    16
}
/// $07C7AC: jsr $5dd02.l
pub fn i_07c7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C7B2);
    cpu.pc = 0x5DD02;
    16
}
/// $07C7B2: addi.w #$30, $24(a0)
pub fn i_07c7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7B8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C7B8: move.w #$8, $88(a0)
pub fn i_07c7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7BE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C7BE: move.w #$2000, $38(a0)
pub fn i_07c7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7C4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C7C4: lea.l $7c814(pc), a1
pub fn i_07c7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C814;
    16
}
/// $07C7C8: jsr $4ae.l
pub fn i_07c7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C7CE);
    cpu.pc = 0x4AE;
    16
}
/// $07C7CE: jsr $5dd02.l
pub fn i_07c7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C7D4);
    cpu.pc = 0x5DD02;
    16
}
/// $07C7D4: addi.w #$40, $24(a0)
pub fn i_07c7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7DA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C7DA: move.w #$c, $88(a0)
pub fn i_07c7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7E0;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C7E0: move.w #$2000, $38(a0)
pub fn i_07c7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7E6;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C7E6: lea.l $7c814(pc), a1
pub fn i_07c7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C814;
    16
}
/// $07C7EA: jsr $4ae.l
pub fn i_07c7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C7F0);
    cpu.pc = 0x4AE;
    16
}
/// $07C7F0: jsr $5dd02.l
pub fn i_07c7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C7F6);
    cpu.pc = 0x5DD02;
    16
}
/// $07C7F6: addi.w #$50, $24(a0)
pub fn i_07c7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C7FC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C7FC: move.w #$10, $88(a0)
pub fn i_07c7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C802;
    cpu.ir = 0x317C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x88);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C802: move.w #$2000, $38(a0)
pub fn i_07c802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C808;
    cpu.ir = 0x317C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C808: move.w #$2000, $38(a6)
pub fn i_07c808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C80E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07C80E: jmp $77fd6.l
pub fn i_07c80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C814;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77FD6;
    8
}
// ===== M2Boss_Shell_FlashDelay_07c814  ($07C814, 36 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C814: subq.w #$1, $88(a6)
pub fn i_07c814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C818;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C818: bne.w $7c836
pub fn i_07c818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C81C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C836; return 10; }
    8
}
/// $07C81C: jsr $5e9b6.l
pub fn i_07c81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C822);
    cpu.pc = 0x5E9B6;
    16
}
/// $07C822: andi.w #$2f, d0
pub fn i_07c822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C826;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x2F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C826: subi.w #$1f, d0
pub fn i_07c826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C82A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C82A: add.w d0, $22(a6)
pub fn i_07c82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C82E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C82E: lea.l $77f6a.l, a1
pub fn i_07c82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C834;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F6A;
    20
}
/// $07C834: move.l a1, (a6)
pub fn i_07c834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C836;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C836: rts
pub fn i_07c836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C838;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Shell_Trail_07c838  ($07C838, 40 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C838: lea.l $2dfed6.l, a0
pub fn i_07c838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C83E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DFED6;
    20
}
/// $07C83E: jsr $28cd4.l
pub fn i_07c83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C844;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C844);
    cpu.pc = 0x28CD4;
    16
}
/// $07C844: lea.l $7c84a(pc), a1
pub fn i_07c844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C848;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C84A;
    16
}
/// $07C848: move.l a1, (a6)
pub fn i_07c848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C84A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C84A: jsr $2783a.l
pub fn i_07c84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C850;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C850);
    cpu.pc = 0x2783A;
    16
}
/// $07C850: jsr $28d70.l
pub fn i_07c850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C856);
    cpu.pc = 0x28D70;
    16
}
/// $07C856: bcc.w $7c860
pub fn i_07c856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C85A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C860; return 10; }
    8
}
/// $07C85A: lea.l $7c65c(pc), a1
pub fn i_07c85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C85E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C85E: move.l a1, (a6)
pub fn i_07c85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C860;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_07c860  ($07C860, 8 B, thunks/jsr_abs_thunks.c)
/// $07C860: jsr $283d8.l
pub fn i_07c860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C866;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C866);
    cpu.pc = 0x283D8;
    16
}
/// $07C866: rts
pub fn i_07c866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C868;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Shell_Flash_07c868  ($07C868, 66 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C868: move.w #$122, d1
pub fn i_07c868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C86C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x122;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C86C: jsr $236e.l
pub fn i_07c86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C872;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C872);
    cpu.pc = 0x236E;
    16
}
/// $07C872: move.w #$4000, d0
pub fn i_07c872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C876;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C876: jsr $28134.l
pub fn i_07c876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C87C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C87C);
    cpu.pc = 0x28134;
    16
}
/// $07C87C: andi.w #$ffe3, $38(a6)
pub fn i_07c87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C882;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C882: ori.w #$10, $38(a6)
pub fn i_07c882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C888;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C888: lea.l $2dfd44.l, a0
pub fn i_07c888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C88E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DFD44;
    20
}
/// $07C88E: jsr $28cd4.l
pub fn i_07c88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C894;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C894);
    cpu.pc = 0x28CD4;
    16
}
/// $07C894: lea.l $7c89a(pc), a1
pub fn i_07c894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C898;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C89A;
    16
}
/// $07C898: move.l a1, (a6)
pub fn i_07c898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C89A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C89A: jsr $27cee.l
pub fn i_07c89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C8A0);
    cpu.pc = 0x27CEE;
    16
}
/// $07C8A0: jsr $28d70.l
pub fn i_07c8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C8A6);
    cpu.pc = 0x28D70;
    16
}
/// $07C8A6: bcc.w $7c8b0
pub fn i_07c8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C8B0; return 10; }
    8
}
// ===== SetTaskHandler_07c8aa  ($07C8AA, 8 B, kernel/task_handlers.c)
/// $07C8AA: lea.l $7c65c(pc), a1
pub fn i_07c8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C8AE: move.l a1, (a6)
pub fn i_07c8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C8B0: rts
pub fn i_07c8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8B2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Shell_Spark_07c8b2  ($07C8B2, 94 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C8B2: move.w #$8, d1
pub fn i_07c8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C8B6: jsr $236e.l
pub fn i_07c8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C8BC);
    cpu.pc = 0x236E;
    16
}
/// $07C8BC: move.w #$4000, d0
pub fn i_07c8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C8C0: jsr $28134.l
pub fn i_07c8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C8C6);
    cpu.pc = 0x28134;
    16
}
/// $07C8C6: andi.w #$ffe3, $38(a6)
pub fn i_07c8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8CC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C8CC: ori.w #$14, $38(a6)
pub fn i_07c8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8D2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07C8D2: jsr $267e2.l
pub fn i_07c8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C8D8);
    cpu.pc = 0x267E2;
    16
}
/// $07C8D8: lea.l $29e440.l, a0
pub fn i_07c8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8DE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E440;
    20
}
/// $07C8DE: jsr $28cd4.l
pub fn i_07c8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C8E4);
    cpu.pc = 0x28CD4;
    16
}
/// $07C8E4: lea.l $7c8ea(pc), a1
pub fn i_07c8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8E8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C8EA;
    16
}
/// $07C8E8: move.l a1, (a6)
pub fn i_07c8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8EA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C8EA: jsr $2783a.l
pub fn i_07c8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C8F0);
    cpu.pc = 0x2783A;
    16
}
/// $07C8F0: jsr $28d70.l
pub fn i_07c8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C8F6);
    cpu.pc = 0x28D70;
    16
}
/// $07C8F6: bcc.w $7c900
pub fn i_07c8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C900; return 10; }
    8
}
/// $07C8FA: lea.l $7c65c(pc), a1
pub fn i_07c8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C8FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C8FE: move.l a1, (a6)
pub fn i_07c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C900;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C900: movea.l #$ffffffff, a0
pub fn i_07c900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C906;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07C906: jsr $5dd56.l
pub fn i_07c906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C90C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C90C);
    cpu.pc = 0x5DD56;
    16
}
/// $07C90C: bcc.w $7c916
pub fn i_07c90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C910;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7C916; return 10; }
    8
}
// ===== SetTaskHandler_07c910  ($07C910, 8 B, kernel/task_handlers.c)
/// $07C910: lea.l $7c65c(pc), a1
pub fn i_07c910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C914;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07C914: move.l a1, (a6)
pub fn i_07c914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C916;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07C916: rts
pub fn i_07c916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C918;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Shell_Spawn_07c918  ($07C918, 126 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C918: movea.l $c(a6), a0
pub fn i_07c918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C91C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C91C: movea.l $c(a0), a0
pub fn i_07c91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C920;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07C920: tst.b $20(a0)
pub fn i_07c920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C924;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07C924: bne.w $7c966
pub fn i_07c924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C928;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7C966; return 10; }
    8
}
/// $07C928: move.w #$1065, d0
pub fn i_07c928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C92C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1065;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C92C: jsr $2352.l
pub fn i_07c92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C932;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C932);
    cpu.pc = 0x2352;
    16
}
/// $07C932: lea.l $7c664(pc), a1
pub fn i_07c932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C936;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C664;
    16
}
/// $07C936: jsr $4ae.l
pub fn i_07c936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C93C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C93C);
    cpu.pc = 0x4AE;
    16
}
/// $07C93C: jsr $5dd02.l
pub fn i_07c93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C942);
    cpu.pc = 0x5DD02;
    16
}
/// $07C942: move.w $72(a6), $72(a0)
pub fn i_07c942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C948;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07C948: move.w $70(a6), d1
pub fn i_07c948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C94C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C94C: add.w d1, d1
pub fn i_07c94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C94E;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C94E: add.w d1, d1
pub fn i_07c94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C950;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C950: lea.l $2df888.l, a1
pub fn i_07c950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C956;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF888;
    20
}
/// $07C956: move.w (a1, d1.w), d0
pub fn i_07c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C95A;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07C95A: add.w d0, $22(a0)
pub fn i_07c95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C95E;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C95E: move.w $2(a1, d1.w), d0
pub fn i_07c95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C962;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07C962: add.w d0, $24(a0)
pub fn i_07c962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C966;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C966: lea.l $7c8b2(pc), a1
pub fn i_07c966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C96A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C8B2;
    16
}
/// $07C96A: jsr $4ae.l
pub fn i_07c96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C970);
    cpu.pc = 0x4AE;
    16
}
/// $07C970: jsr $5dd02.l
pub fn i_07c970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C976;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C976);
    cpu.pc = 0x5DD02;
    16
}
/// $07C976: move.w $70(a6), d1
pub fn i_07c976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C97A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07C97A: add.w d1, d1
pub fn i_07c97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C97C;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C97C: add.w d1, d1
pub fn i_07c97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C97E;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C97E: lea.l $2df888.l, a1
pub fn i_07c97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C984;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF888;
    20
}
/// $07C984: move.w (a1, d1.w), d0
pub fn i_07c984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C988;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07C988: add.w d0, $22(a0)
pub fn i_07c988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C98C;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C98C: move.w $2(a1, d1.w), d0
pub fn i_07c98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C990;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07C990: add.w d0, $24(a0)
pub fn i_07c990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C994;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07C994: rts
pub fn i_07c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C996;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Debris_Init_07c996  ($07C996, 194 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07C996: jsr $5e9b6.l
pub fn i_07c996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C99C);
    cpu.pc = 0x5E9B6;
    16
}
/// $07C99C: andi.l #$7, d0
pub fn i_07c99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9A2;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x7) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $07C9A2: movea.l #$2df7e2, a0
pub fn i_07c9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9A8;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2DF7E2;
    cpu.a[0] = v1;
    4
}
/// $07C9A8: lsl.w #$2, d0
pub fn i_07c9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9AA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07C9AA: movea.l (a0, d0.w), a0
pub fn i_07c9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9AE;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07C9AE: cmpa.l #$ffffffff, a0
pub fn i_07c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9B4;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07C9B4: beq.w $7c9be
pub fn i_07c9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9B8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7C9BE; return 10; }
    8
}
/// $07C9B8: jsr $28cd4.l
pub fn i_07c9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C9BE);
    cpu.pc = 0x28CD4;
    16
}
/// $07C9BE: move.w #$a, d1
pub fn i_07c9be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9C2;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C9C2: jsr $236e.l
pub fn i_07c9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C9C8);
    cpu.pc = 0x236E;
    16
}
/// $07C9C8: jsr $5e9b6.l
pub fn i_07c9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7C9CE);
    cpu.pc = 0x5E9B6;
    16
}
/// $07C9CE: move.w d0, d3
pub fn i_07c9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9D0;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07C9D0: andi.l #$3e, d0
pub fn i_07c9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9D6;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0x3E) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $07C9D6: subi.w #$20, d0
pub fn i_07c9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9DA;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C9DA: andi.w #$ff, d0
pub fn i_07c9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9DE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C9DE: add.w d0, d0
pub fn i_07c9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9E0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07C9E0: lea.l $2c072c.l, a1
pub fn i_07c9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $07C9E6: move.w (a1, d0.w), d1
pub fn i_07c9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9EA;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07C9EA: lea.l $2c07ac.l, a1
pub fn i_07c9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $07C9F0: move.w (a1, d0.w), d2
pub fn i_07c9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9F4;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07C9F4: andi.w #$7, d3
pub fn i_07c9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9F8;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07C9F8: addq.w #$1, d3
pub fn i_07c9f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9FA;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07C9FA: muls.w d3, d1
pub fn i_07c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9FC;
    cpu.ir = 0xC3C3;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $07C9FC: asl.w #$2, d2
pub fn i_07c9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7C9FE;
    cpu.ir = 0xE542;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07C9FE: move.w d1, $2a(a6)
pub fn i_07c9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA02;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CA02: move.w d2, $28(a6)
pub fn i_07ca02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA06;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CA06: jsr $5e9b6.l
pub fn i_07ca06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CA0C);
    cpu.pc = 0x5E9B6;
    16
}
/// $07CA0C: andi.w #$3, d0
pub fn i_07ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA10;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07CA10: addq.w #$2, d0
pub fn i_07ca10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA12;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07CA12: move.w $28(a6), d1
pub fn i_07ca12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA16;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07CA16: muls.w d1, d0
pub fn i_07ca16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA18;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $07CA18: asr.w #$1, d0
pub fn i_07ca18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA1A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07CA1A: move.w d0, $28(a6)
pub fn i_07ca1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CA1E: jsr $5e9b6.l
pub fn i_07ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CA24);
    cpu.pc = 0x5E9B6;
    16
}
/// $07CA24: andi.w #$3, d0
pub fn i_07ca24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA28;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07CA28: addq.w #$2, d0
pub fn i_07ca28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA2A;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07CA2A: move.w $2a(a6), d1
pub fn i_07ca2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA2E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07CA2E: muls.w d1, d0
pub fn i_07ca2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA30;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $07CA30: asr.w #$1, d0
pub fn i_07ca30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA32;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07CA32: move.w d0, $2a(a6)
pub fn i_07ca32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA36;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CA36: lea.l $7ca3c(pc), a1
pub fn i_07ca36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA3A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CA3C;
    16
}
/// $07CA3A: move.l a1, (a6)
pub fn i_07ca3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA3C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07CA3C: jsr $27d50.l
pub fn i_07ca3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CA42);
    cpu.pc = 0x27D50;
    16
}
/// $07CA42: jsr $28d70.l
pub fn i_07ca42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CA48);
    cpu.pc = 0x28D70;
    16
}
/// $07CA48: movea.l #$ffffffff, a0
pub fn i_07ca48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA4E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07CA4E: jsr $5dd56.l
pub fn i_07ca4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CA54);
    cpu.pc = 0x5DD56;
    16
}
/// $07CA54: bcc.w $7ca5e
pub fn i_07ca54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7CA5E; return 10; }
    8
}
// ===== SetTaskHandler_07ca58  ($07CA58, 8 B, kernel/task_handlers.c)
/// $07CA58: lea.l $7c65c(pc), a1
pub fn i_07ca58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7C65C;
    16
}
/// $07CA5C: move.l a1, (a6)
pub fn i_07ca5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07CA5E: rts
pub fn i_07ca5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn0_07ca60  ($07CA60, 80 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CA60: lea.l $77fd6.l, a1
pub fn i_07ca60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA66;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07CA66: jsr $4ae.l
pub fn i_07ca66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CA6C);
    cpu.pc = 0x4AE;
    16
}
/// $07CA6C: jsr $5dd02.l
pub fn i_07ca6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CA72);
    cpu.pc = 0x5DD02;
    16
}
/// $07CA72: subi.w #$60, $22(a0)
pub fn i_07ca72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA78;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CA78: subi.w #$8, $24(a0)
pub fn i_07ca78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA7E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CA7E: lea.l $2df73e.l, a1
pub fn i_07ca7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA84;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF73E;
    20
}
/// $07CA84: jsr $77c7e.l
pub fn i_07ca84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CA8A);
    cpu.pc = 0x77C7E;
    16
}
/// $07CA8A: subi.w #$60, $22(a0)
pub fn i_07ca8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA90;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CA90: subi.w #$8, $24(a0)
pub fn i_07ca90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA96;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CA96: lea.l $2df762.l, a1
pub fn i_07ca96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CA9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF762;
    20
}
/// $07CA9C: jsr $77c7e.l
pub fn i_07ca9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CAA2);
    cpu.pc = 0x77C7E;
    16
}
/// $07CAA2: subi.w #$60, $22(a0)
pub fn i_07caa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAA8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CAA8: subi.w #$8, $24(a0)
pub fn i_07caa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAAE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CAAE: rts
pub fn i_07caae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn1_07cab0  ($07CAB0, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CAB0: lea.l $2df786.l, a1
pub fn i_07cab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAB6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF786;
    20
}
/// $07CAB6: jsr $77c7e.l
pub fn i_07cab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CABC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CABC);
    cpu.pc = 0x77C7E;
    16
}
/// $07CABC: subi.w #$40, $22(a0)
pub fn i_07cabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAC2;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CAC2: subi.w #$c, $24(a0)
pub fn i_07cac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAC8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CAC8: lea.l $2df73e.l, a1
pub fn i_07cac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CACE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF73E;
    20
}
/// $07CACE: jsr $77c7e.l
pub fn i_07cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CAD4);
    cpu.pc = 0x77C7E;
    16
}
/// $07CAD4: subi.w #$40, $22(a0)
pub fn i_07cad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CADA;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CADA: subi.w #$c, $24(a0)
pub fn i_07cada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAE0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CAE0: lea.l $2df762.l, a1
pub fn i_07cae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAE6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF762;
    20
}
/// $07CAE6: jsr $77c7e.l
pub fn i_07cae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CAEC);
    cpu.pc = 0x77C7E;
    16
}
/// $07CAEC: subi.w #$40, $22(a0)
pub fn i_07caec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAF2;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CAF2: subi.w #$c, $24(a0)
pub fn i_07caf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAF8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0xC, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CAF8: rts
pub fn i_07caf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CAFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn2_07cafa  ($07CAFA, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CAFA: lea.l $2df786.l, a1
pub fn i_07cafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB00;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF786;
    20
}
/// $07CB00: jsr $77c7e.l
pub fn i_07cb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CB06);
    cpu.pc = 0x77C7E;
    16
}
/// $07CB06: subi.w #$28, $22(a0)
pub fn i_07cb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB0C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB0C: subi.w #$10, $24(a0)
pub fn i_07cb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB12;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB12: lea.l $2df750.l, a1
pub fn i_07cb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB18;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF750;
    20
}
/// $07CB18: jsr $77c7e.l
pub fn i_07cb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CB1E);
    cpu.pc = 0x77C7E;
    16
}
/// $07CB1E: subi.w #$28, $22(a0)
pub fn i_07cb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB24;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB24: subi.w #$10, $24(a0)
pub fn i_07cb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB2A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB2A: lea.l $2df774.l, a1
pub fn i_07cb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB30;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF774;
    20
}
/// $07CB30: jsr $77c7e.l
pub fn i_07cb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CB36);
    cpu.pc = 0x77C7E;
    16
}
/// $07CB36: subi.w #$28, $22(a0)
pub fn i_07cb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB3C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB3C: subi.w #$10, $24(a0)
pub fn i_07cb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB42;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB42: rts
pub fn i_07cb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn3_07cb44  ($07CB44, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CB44: lea.l $2df786.l, a1
pub fn i_07cb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB4A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF786;
    20
}
/// $07CB4A: jsr $77c7e.l
pub fn i_07cb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CB50);
    cpu.pc = 0x77C7E;
    16
}
/// $07CB50: subi.w #$18, $22(a0)
pub fn i_07cb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB56;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB56: subi.w #$10, $24(a0)
pub fn i_07cb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB5C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB5C: lea.l $2df750.l, a1
pub fn i_07cb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB62;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF750;
    20
}
/// $07CB62: jsr $77c7e.l
pub fn i_07cb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CB68);
    cpu.pc = 0x77C7E;
    16
}
/// $07CB68: subi.w #$18, $22(a0)
pub fn i_07cb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB6E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB6E: subi.w #$10, $24(a0)
pub fn i_07cb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB74;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB74: lea.l $2df774.l, a1
pub fn i_07cb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB7A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF774;
    20
}
/// $07CB7A: jsr $77c7e.l
pub fn i_07cb7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CB80);
    cpu.pc = 0x77C7E;
    16
}
/// $07CB80: subi.w #$18, $22(a0)
pub fn i_07cb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB86;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB86: subi.w #$10, $24(a0)
pub fn i_07cb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB8C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CB8C: rts
pub fn i_07cb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn4_07cb8e  ($07CB8E, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CB8E: lea.l $2df786.l, a1
pub fn i_07cb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB94;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF786;
    20
}
/// $07CB94: jsr $77c7e.l
pub fn i_07cb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CB9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CB9A);
    cpu.pc = 0x77C7E;
    16
}
/// $07CB9A: subi.w #$8, $22(a0)
pub fn i_07cb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBA0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CBA0: subi.w #$18, $24(a0)
pub fn i_07cba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBA6;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CBA6: lea.l $2df750.l, a1
pub fn i_07cba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBAC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF750;
    20
}
/// $07CBAC: jsr $77c7e.l
pub fn i_07cbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CBB2);
    cpu.pc = 0x77C7E;
    16
}
/// $07CBB2: subi.w #$8, $22(a0)
pub fn i_07cbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBB8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CBB8: subi.w #$18, $24(a0)
pub fn i_07cbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBBE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CBBE: lea.l $2df774.l, a1
pub fn i_07cbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBC4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF774;
    20
}
/// $07CBC4: jsr $77c7e.l
pub fn i_07cbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CBCA);
    cpu.pc = 0x77C7E;
    16
}
/// $07CBCA: subi.w #$8, $22(a0)
pub fn i_07cbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBD0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CBD0: subi.w #$18, $24(a0)
pub fn i_07cbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBD6;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CBD6: rts
pub fn i_07cbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn5_07cbd8  ($07CBD8, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CBD8: lea.l $2df786.l, a1
pub fn i_07cbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBDE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF786;
    20
}
/// $07CBDE: jsr $77c7e.l
pub fn i_07cbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CBE4);
    cpu.pc = 0x77C7E;
    16
}
/// $07CBE4: addi.w #$10, $22(a0)
pub fn i_07cbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBEA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CBEA: subi.w #$1c, $24(a0)
pub fn i_07cbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBF0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CBF0: lea.l $2df750.l, a1
pub fn i_07cbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBF6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF750;
    20
}
/// $07CBF6: jsr $77c7e.l
pub fn i_07cbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CBFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CBFC);
    cpu.pc = 0x77C7E;
    16
}
/// $07CBFC: addi.w #$10, $22(a0)
pub fn i_07cbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC02;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC02: subi.w #$1c, $24(a0)
pub fn i_07cc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC08;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC08: lea.l $2df774.l, a1
pub fn i_07cc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC0E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF774;
    20
}
/// $07CC0E: jsr $77c7e.l
pub fn i_07cc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CC14);
    cpu.pc = 0x77C7E;
    16
}
/// $07CC14: addi.w #$10, $22(a0)
pub fn i_07cc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC1A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC1A: subi.w #$1c, $24(a0)
pub fn i_07cc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC20;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1C, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC20: rts
pub fn i_07cc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn6_07cc22  ($07CC22, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CC22: lea.l $2df786.l, a1
pub fn i_07cc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC28;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF786;
    20
}
/// $07CC28: jsr $77c7e.l
pub fn i_07cc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CC2E);
    cpu.pc = 0x77C7E;
    16
}
/// $07CC2E: addi.w #$20, $22(a0)
pub fn i_07cc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC34;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC34: subi.w #$20, $24(a0)
pub fn i_07cc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC3A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC3A: lea.l $2df750.l, a1
pub fn i_07cc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC40;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF750;
    20
}
/// $07CC40: jsr $77c7e.l
pub fn i_07cc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CC46);
    cpu.pc = 0x77C7E;
    16
}
/// $07CC46: addi.w #$20, $22(a0)
pub fn i_07cc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC4C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC4C: subi.w #$20, $24(a0)
pub fn i_07cc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC52;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC52: lea.l $2df774.l, a1
pub fn i_07cc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC58;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF774;
    20
}
/// $07CC58: jsr $77c7e.l
pub fn i_07cc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CC5E);
    cpu.pc = 0x77C7E;
    16
}
/// $07CC5E: addi.w #$20, $22(a0)
pub fn i_07cc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC64;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC64: subi.w #$20, $24(a0)
pub fn i_07cc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC6A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC6A: rts
pub fn i_07cc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn7_07cc6c  ($07CC6C, 74 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CC6C: lea.l $2df786.l, a1
pub fn i_07cc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC72;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF786;
    20
}
/// $07CC72: jsr $77c7e.l
pub fn i_07cc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CC78);
    cpu.pc = 0x77C7E;
    16
}
/// $07CC78: addi.w #$30, $22(a0)
pub fn i_07cc78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC7E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC7E: subi.w #$20, $24(a0)
pub fn i_07cc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC84;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC84: lea.l $2df750.l, a1
pub fn i_07cc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC8A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF750;
    20
}
/// $07CC8A: jsr $77c7e.l
pub fn i_07cc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CC90);
    cpu.pc = 0x77C7E;
    16
}
/// $07CC90: addi.w #$30, $22(a0)
pub fn i_07cc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC96;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC96: subi.w #$20, $24(a0)
pub fn i_07cc96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CC9C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CC9C: lea.l $2df774.l, a1
pub fn i_07cc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCA2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF774;
    20
}
/// $07CCA2: jsr $77c7e.l
pub fn i_07cca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CCA8);
    cpu.pc = 0x77C7E;
    16
}
/// $07CCA8: addi.w #$30, $22(a0)
pub fn i_07cca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCAE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CCAE: subi.w #$20, $24(a0)
pub fn i_07ccae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCB4;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CCB4: rts
pub fn i_07ccb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Wreck_Spawn8_07ccb6  ($07CCB6, 80 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CCB6: lea.l $77fd6.l, a1
pub fn i_07ccb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCBC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07CCBC: jsr $4ae.l
pub fn i_07ccbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CCC2);
    cpu.pc = 0x4AE;
    16
}
/// $07CCC2: jsr $5dd02.l
pub fn i_07ccc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CCC8);
    cpu.pc = 0x5DD02;
    16
}
/// $07CCC8: addi.w #$48, $22(a0)
pub fn i_07ccc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCCE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CCCE: subi.w #$20, $24(a0)
pub fn i_07ccce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCD4;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CCD4: lea.l $2df762.l, a1
pub fn i_07ccd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCDA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF762;
    20
}
/// $07CCDA: jsr $77c7e.l
pub fn i_07ccda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CCE0);
    cpu.pc = 0x77C7E;
    16
}
/// $07CCE0: addi.w #$48, $22(a0)
pub fn i_07cce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCE6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CCE6: subi.w #$20, $24(a0)
pub fn i_07cce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCEC;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CCEC: lea.l $2df798.l, a1
pub fn i_07ccec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCF2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2DF798;
    20
}
/// $07CCF2: jsr $77c7e.l
pub fn i_07ccf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CCF8);
    cpu.pc = 0x77C7E;
    16
}
/// $07CCF8: addi.w #$48, $22(a0)
pub fn i_07ccf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CCFE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CCFE: subi.w #$20, $24(a0)
pub fn i_07ccfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD04;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD04: rts
pub fn i_07cd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD06;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn0_07cd06  ($07CD06, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CD06: lea.l $7ce14(pc), a1
pub fn i_07cd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE14;
    16
}
/// $07CD0A: jsr $4ae.l
pub fn i_07cd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD10);
    cpu.pc = 0x4AE;
    16
}
/// $07CD10: jsr $5dd02.l
pub fn i_07cd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD16);
    cpu.pc = 0x5DD02;
    16
}
/// $07CD16: subi.w #$80, $22(a0)
pub fn i_07cd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD1C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD1C: subi.w #$20, $24(a0)
pub fn i_07cd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD22;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD22: rts
pub fn i_07cd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn1_07cd24  ($07CD24, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CD24: lea.l $7ce28(pc), a1
pub fn i_07cd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD28;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE28;
    16
}
/// $07CD28: jsr $4ae.l
pub fn i_07cd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD2E);
    cpu.pc = 0x4AE;
    16
}
/// $07CD2E: jsr $5dd02.l
pub fn i_07cd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD34);
    cpu.pc = 0x5DD02;
    16
}
/// $07CD34: subi.w #$58, $22(a0)
pub fn i_07cd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD3A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x58, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD3A: subi.w #$28, $24(a0)
pub fn i_07cd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD40;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD40: rts
pub fn i_07cd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn2_07cd42  ($07CD42, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CD42: lea.l $7ce28(pc), a1
pub fn i_07cd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE28;
    16
}
/// $07CD46: jsr $4ae.l
pub fn i_07cd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD4C);
    cpu.pc = 0x4AE;
    16
}
/// $07CD4C: jsr $5dd02.l
pub fn i_07cd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD52);
    cpu.pc = 0x5DD02;
    16
}
/// $07CD52: subi.w #$48, $22(a0)
pub fn i_07cd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD58;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD58: subi.w #$38, $24(a0)
pub fn i_07cd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD5E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD5E: rts
pub fn i_07cd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn3_07cd60  ($07CD60, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CD60: lea.l $7ce28(pc), a1
pub fn i_07cd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE28;
    16
}
/// $07CD64: jsr $4ae.l
pub fn i_07cd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD6A);
    cpu.pc = 0x4AE;
    16
}
/// $07CD6A: jsr $5dd02.l
pub fn i_07cd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD70);
    cpu.pc = 0x5DD02;
    16
}
/// $07CD70: subi.w #$38, $22(a0)
pub fn i_07cd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD76;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD76: subi.w #$38, $24(a0)
pub fn i_07cd76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD7C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD7C: rts
pub fn i_07cd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn4_07cd7e  ($07CD7E, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CD7E: lea.l $7ce28(pc), a1
pub fn i_07cd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE28;
    16
}
/// $07CD82: jsr $4ae.l
pub fn i_07cd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD88);
    cpu.pc = 0x4AE;
    16
}
/// $07CD88: jsr $5dd02.l
pub fn i_07cd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CD8E);
    cpu.pc = 0x5DD02;
    16
}
/// $07CD8E: subi.w #$28, $22(a0)
pub fn i_07cd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD94;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD94: subi.w #$38, $24(a0)
pub fn i_07cd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD9A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CD9A: rts
pub fn i_07cd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CD9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn5_07cd9c  ($07CD9C, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CD9C: lea.l $7ce28(pc), a1
pub fn i_07cd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE28;
    16
}
/// $07CDA0: jsr $4ae.l
pub fn i_07cda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CDA6);
    cpu.pc = 0x4AE;
    16
}
/// $07CDA6: jsr $5dd02.l
pub fn i_07cda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CDAC);
    cpu.pc = 0x5DD02;
    16
}
/// $07CDAC: subi.w #$8, $22(a0)
pub fn i_07cdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDB2;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CDB2: subi.w #$38, $24(a0)
pub fn i_07cdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDB8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CDB8: rts
pub fn i_07cdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn6_07cdba  ($07CDBA, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CDBA: lea.l $7ce28(pc), a1
pub fn i_07cdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE28;
    16
}
/// $07CDBE: jsr $4ae.l
pub fn i_07cdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CDC4);
    cpu.pc = 0x4AE;
    16
}
/// $07CDC4: jsr $5dd02.l
pub fn i_07cdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CDCA);
    cpu.pc = 0x5DD02;
    16
}
/// $07CDCA: addi.w #$8, $22(a0)
pub fn i_07cdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDD0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CDD0: subi.w #$38, $24(a0)
pub fn i_07cdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDD6;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CDD6: rts
pub fn i_07cdd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn7_07cdd8  ($07CDD8, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CDD8: lea.l $7ce28(pc), a1
pub fn i_07cdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE28;
    16
}
/// $07CDDC: jsr $4ae.l
pub fn i_07cddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CDE2);
    cpu.pc = 0x4AE;
    16
}
/// $07CDE2: jsr $5dd02.l
pub fn i_07cde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CDE8);
    cpu.pc = 0x5DD02;
    16
}
/// $07CDE8: addi.w #$8, $22(a0)
pub fn i_07cde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDEE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CDEE: subi.w #$38, $24(a0)
pub fn i_07cdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDF4;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CDF4: rts
pub fn i_07cdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_Spawn8_07cdf6  ($07CDF6, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CDF6: lea.l $7ce3c(pc), a1
pub fn i_07cdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CDFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CE3C;
    16
}
/// $07CDFA: jsr $4ae.l
pub fn i_07cdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE00);
    cpu.pc = 0x4AE;
    16
}
/// $07CE00: jsr $5dd02.l
pub fn i_07ce00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE06);
    cpu.pc = 0x5DD02;
    16
}
/// $07CE06: addi.w #$20, $22(a0)
pub fn i_07ce06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE0C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CE0C: subi.w #$38, $24(a0)
pub fn i_07ce0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE12;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CE12: rts
pub fn i_07ce12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M2Boss_Burst_HitboxA_07ce14  ($07CE14, 20 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CE14: lea.l $2e001e.l, a0
pub fn i_07ce14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE1A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E001E;
    20
}
/// $07CE1A: move.l a0, $4c(a6)
pub fn i_07ce1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE1E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07CE1E: jsr $283ca.l
pub fn i_07ce1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE24);
    cpu.pc = 0x283CA;
    16
}
/// $07CE24: bra.w $7ce4c
pub fn i_07ce24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE28;
    cpu.ir = 0x6000;
    cpu.pc = 0x7CE4C;
    10
}
// ===== M2Boss_Burst_HitboxB_07ce28  ($07CE28, 20 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CE28: lea.l $2e0072.l, a0
pub fn i_07ce28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0072;
    20
}
/// $07CE2E: move.l a0, $4c(a6)
pub fn i_07ce2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE32;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07CE32: jsr $283ca.l
pub fn i_07ce32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE38);
    cpu.pc = 0x283CA;
    16
}
/// $07CE38: bra.w $7ce4c
pub fn i_07ce38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7CE4C;
    10
}
// ===== M2Boss_Burst_HitboxC_07ce3c  ($07CE3C, 84 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CE3C: lea.l $2e00c6.l, a0
pub fn i_07ce3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E00C6;
    20
}
/// $07CE42: move.l a0, $4c(a6)
pub fn i_07ce42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE46;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07CE46: jsr $283ca.l
pub fn i_07ce46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE4C);
    cpu.pc = 0x283CA;
    16
}
/// $07CE4C: move.w #$122, d1
pub fn i_07ce4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE50;
    cpu.ir = 0x323C;
    let v1: u32 = 0x122;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07CE50: jsr $236e.l
pub fn i_07ce50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE56);
    cpu.pc = 0x236E;
    16
}
/// $07CE56: jsr $267e2.l
pub fn i_07ce56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE5C);
    cpu.pc = 0x267E2;
    16
}
/// $07CE5C: move.w #$f000, d0
pub fn i_07ce5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE60;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07CE60: jsr $28134.l
pub fn i_07ce60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE66);
    cpu.pc = 0x28134;
    16
}
/// $07CE66: andi.w #$ffe3, $38(a6)
pub fn i_07ce66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE6C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CE6C: ori.w #$10, $38(a6)
pub fn i_07ce6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE72;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CE72: lea.l $2dfef4.l, a0
pub fn i_07ce72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DFEF4;
    20
}
/// $07CE78: jsr $28cd4.l
pub fn i_07ce78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE7E);
    cpu.pc = 0x28CD4;
    16
}
/// $07CE7E: jsr $28d70.l
pub fn i_07ce7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE84);
    cpu.pc = 0x28D70;
    16
}
/// $07CE84: jsr $283d8.l
pub fn i_07ce84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CE8A);
    cpu.pc = 0x283D8;
    16
}
/// $07CE8A: jmp $518.l
pub fn i_07ce8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE90;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== M2Boss_Rts_07ce90  ($07CE90, 2 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CE90: rts
pub fn i_07ce90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CE92;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SDS_07ced8  ($07CED8, 14 B, kernel/state_dispatch_stubs.c)
/// $07CED8: lea.l $2985f8.l, a2
pub fn i_07ced8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEDE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2985F8;
    20
}
/// $07CEDE: jsr $5022a.l
pub fn i_07cede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CEE4);
    cpu.pc = 0x5022A;
    16
}
/// $07CEE4: rts
pub fn i_07cee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_RetGate_07cee6  ($07CEE6, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CEE6: movea.l $8(a6), a1
pub fn i_07cee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEEA;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07CEEA: move.b $10(a6), d0
pub fn i_07ceea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEEE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07CEEE: cmp.b $10(a1), d0
pub fn i_07ceee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEF2;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07CEF2: bcs.w $7cefc
pub fn i_07cef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEF6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7CEFC; return 10; }
    8
}
// ===== ClearXN_07cef6  ($07CEF6, 6 B, thunks/ccr_helpers.c)
/// $07CEF6: andi.b #$ee, ccr
pub fn i_07cef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEFA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07CEFA: rts
pub fn i_07cefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CEFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07cefc  ($07CEFC, 6 B, thunks/ccr_helpers.c)
/// $07CEFC: ori.b #$11, ccr
pub fn i_07cefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF00;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07CF00: rts
pub fn i_07cf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Tmpl138_07cf02  ($07CF02, 10 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CF02: move.w #$0, $70(a6)
pub fn i_07cf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF08;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CF08: bra.w $7cf12
pub fn i_07cf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF0C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7CF12;
    10
}
// ===== Crab_Tmpl139_07cf0c  ($07CF0C, 72 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CF0C: move.w #$1, $70(a6)
pub fn i_07cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF12;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CF12: move.w #$4b, d1
pub fn i_07cf12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF16;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07CF16: jsr $236e.l
pub fn i_07cf16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF1C);
    cpu.pc = 0x236E;
    16
}
/// $07CF1C: move.w #$a, $1c(a6)
pub fn i_07cf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF22;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CF22: jsr $138fe.l
pub fn i_07cf22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF28);
    cpu.pc = 0x138FE;
    16
}
/// $07CF28: moveq #$0, d0
pub fn i_07cf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF2A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07CF2A: move.b d0, $20(a6)
pub fn i_07cf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF2E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07CF2E: move.b d0, $21(a6)
pub fn i_07cf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF32;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07CF32: move.b $9c(a6), $84(a6)
pub fn i_07cf32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF38;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad4, v3 as u8);
    20
}
/// $07CF38: lea.l $2bda80.l, a0
pub fn i_07cf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF3E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDA80;
    20
}
/// $07CF3E: jsr $799de.l
pub fn i_07cf3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF44);
    cpu.pc = 0x799DE;
    16
}
/// $07CF44: move.w d0, $66(a6)
pub fn i_07cf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF48;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CF48: jsr $267e2.l
pub fn i_07cf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF4E);
    cpu.pc = 0x267E2;
    16
}
/// $07CF4E: move.b $9a(a6), $3a(a6)
pub fn i_07cf4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF54;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
// ===== SetTaskHandler_07cf54  ($07CF54, 8 B, kernel/task_handlers.c)
/// $07CF54: lea.l $7cf5c(pc), a1
pub fn i_07cf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CF5C;
    16
}
/// $07CF58: move.l a1, (a6)
pub fn i_07cf58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07CF5A: rts
pub fn i_07cf5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF5C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_SpawnClaws_07cf5c  ($07CF5C, 60 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CF5C: jsr $5e7c0.l
pub fn i_07cf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF62);
    cpu.pc = 0x5E7C0;
    16
}
/// $07CF62: move.w #$4001, d0
pub fn i_07cf62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF66;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4001;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07CF66: jsr $28134.l
pub fn i_07cf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF6C);
    cpu.pc = 0x28134;
    16
}
/// $07CF6C: andi.w #$ffe3, $38(a6)
pub fn i_07cf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF72;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CF72: ori.w #$c, $38(a6)
pub fn i_07cf72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF78;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07CF78: lea.l $7d4aa(pc), a1
pub fn i_07cf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D4AA;
    16
}
/// $07CF7C: jsr $4ae.l
pub fn i_07cf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF82);
    cpu.pc = 0x4AE;
    16
}
/// $07CF82: jsr $5dd02.l
pub fn i_07cf82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF88);
    cpu.pc = 0x5DD02;
    16
}
/// $07CF88: lea.l $7d520(pc), a1
pub fn i_07cf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D520;
    16
}
/// $07CF8C: jsr $4ae.l
pub fn i_07cf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF92);
    cpu.pc = 0x4AE;
    16
}
/// $07CF92: jsr $5dd02.l
pub fn i_07cf92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CF98);
    cpu.pc = 0x5DD02;
    16
}
// ===== Crab_Idle_07cf98  ($07CF98, 182 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07CF98: moveq #$0, d0
pub fn i_07cf98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF9A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07CF9A: move.b d0, $72(a6)
pub fn i_07cf9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CF9E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07CF9E: move.w d0, $74(a6)
pub fn i_07cf9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFA2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07CFA2: lea.l $2e04d6.l, a0
pub fn i_07cfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E04D6;
    20
}
/// $07CFA8: jsr $28cd4.l
pub fn i_07cfa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CFAE);
    cpu.pc = 0x28CD4;
    16
}
/// $07CFAE: lea.l $7cfb4(pc), a1
pub fn i_07cfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7CFB4;
    16
}
/// $07CFB2: move.l a1, (a6)
pub fn i_07cfb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07CFB4: jsr $2783a.l
pub fn i_07cfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CFBA);
    cpu.pc = 0x2783A;
    16
}
/// $07CFBA: jsr $28d70.l
pub fn i_07cfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CFC0);
    cpu.pc = 0x28D70;
    16
}
/// $07CFC0: tst.w $70(a6)
pub fn i_07cfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFC4;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07CFC4: bne.w $7cfd6
pub fn i_07cfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFC8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7CFD6; return 10; }
    8
}
/// $07CFC8: move.w $106f50.l, d0
pub fn i_07cfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFCE;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x106F50) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $07CFCE: cmp.w $98(a6), d0
pub fn i_07cfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFD2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07CFD2: blt.w $7cffc
pub fn i_07cfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFD6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7CFFC; return 10; }
    8
}
/// $07CFD6: jsr $5e0d4.l
pub fn i_07cfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7CFDC);
    cpu.pc = 0x5E0D4;
    16
}
/// $07CFDC: move.w $22(a6), d0
pub fn i_07cfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFE0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07CFE0: sub.w $22(a0), d0
pub fn i_07cfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFE4;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07CFE4: cmp.w $98(a6), d0
pub fn i_07cfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFE8;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07CFE8: bgt.w $7cffc
pub fn i_07cfe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFEC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7CFFC; return 10; }
    8
}
/// $07CFEC: move.b #$1, $20(a6)
pub fn i_07cfec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFF2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07CFF2: clr.w $76(a6)
pub fn i_07cff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFF6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07CFF6: lea.l $7d056(pc), a1
pub fn i_07cff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D056;
    16
}
/// $07CFFA: move.l a1, (a6)
pub fn i_07cffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7CFFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07CFFC: jsr $2870a.l
pub fn i_07cffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D002;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D002);
    cpu.pc = 0x2870A;
    16
}
/// $07D002: bcc.w $7d018
pub fn i_07d002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D006;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D018; return 10; }
    8
}
/// $07D006: lea.l $5e766.l, a0
pub fn i_07d006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D00C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07D00C: jsr $5e770.l
pub fn i_07d00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D012);
    cpu.pc = 0x5E770;
    16
}
/// $07D012: bclr.b #$3, $13(a6)
pub fn i_07d012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D018;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D018: jsr $27eba.l
pub fn i_07d018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D01E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D01E);
    cpu.pc = 0x27EBA;
    16
}
/// $07D01E: bcc.w $7d028
pub fn i_07d01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D022;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D028; return 10; }
    8
}
/// $07D022: lea.l $7d3e2(pc), a1
pub fn i_07d022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D026;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3E2;
    16
}
/// $07D026: move.l a1, (a6)
pub fn i_07d026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D028;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D028: jsr $28758.l
pub fn i_07d028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D02E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D02E);
    cpu.pc = 0x28758;
    16
}
/// $07D02E: bcc.w $7d038
pub fn i_07d02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D032;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D038; return 10; }
    8
}
/// $07D032: lea.l $7d332(pc), a1
pub fn i_07d032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D036;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D332;
    16
}
/// $07D036: move.l a1, (a6)
pub fn i_07d036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D038;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D038: movea.l #$ffffffff, a0
pub fn i_07d038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D03E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07D03E: lea.l $2e0b5c.l, a0
pub fn i_07d03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D044;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0B5C;
    20
}
/// $07D044: jsr $5dd5c.l
pub fn i_07d044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D04A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D04A);
    cpu.pc = 0x5DD5C;
    16
}
/// $07D04A: bcc.w $7d054
pub fn i_07d04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D04E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D054; return 10; }
    8
}
// ===== SetTaskHandler_07d04e  ($07D04E, 8 B, kernel/task_handlers.c)
/// $07D04E: lea.l $7dba2(pc), a1
pub fn i_07d04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D052;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07D052: move.l a1, (a6)
pub fn i_07d052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D054;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D054: rts
pub fn i_07d054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D056;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Charge_07d056  ($07D056, 282 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D056: jsr $267e2.l
pub fn i_07d056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D05C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D05C);
    cpu.pc = 0x267E2;
    16
}
/// $07D05C: move.w #$ffe0, $28(a6)
pub fn i_07d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D062;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D062: btst.b #$0, $3a(a6)
pub fn i_07d062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D068;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D068: beq.w $7d070
pub fn i_07d068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D06C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D070; return 10; }
    8
}
/// $07D06C: neg.w $28(a6)
pub fn i_07d06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D070;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D070: move.w $28(a6), $2c(a6)
pub fn i_07d070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D076;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07D076: moveq #$0, d0
pub fn i_07d076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D078;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07D078: move.w d0, $28(a6)
pub fn i_07d078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D07C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D07C: move.b d0, $72(a6)
pub fn i_07d07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D080;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D080: move.w d0, $74(a6)
pub fn i_07d080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D084;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D084: lea.l $2e050a.l, a0
pub fn i_07d084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D08A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E050A;
    20
}
/// $07D08A: jsr $28cd4.l
pub fn i_07d08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D090;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D090);
    cpu.pc = 0x28CD4;
    16
}
/// $07D090: lea.l $7d096(pc), a1
pub fn i_07d090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D094;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D096;
    16
}
/// $07D094: move.l a1, (a6)
pub fn i_07d094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D096;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D096: jsr $27afc.l
pub fn i_07d096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D09C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D09C);
    cpu.pc = 0x27AFC;
    16
}
/// $07D09C: bcc.w $7d0a6
pub fn i_07d09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0A0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D0A6; return 10; }
    8
}
/// $07D0A0: move.b #$ff, $72(a6)
pub fn i_07d0a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0A6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D0A6: cmpi.w #$fc00, $28(a6)
pub fn i_07d0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0AC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFC00, Size::Word, false);
    16
}
/// $07D0AC: blt.w $7d0ba
pub fn i_07d0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0B0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7D0BA; return 10; }
    8
}
/// $07D0B0: cmpi.w #$400, $28(a6)
pub fn i_07d0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0B6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x400, Size::Word, false);
    16
}
/// $07D0B6: blt.w $7d0d2
pub fn i_07d0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0BA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7D0D2; return 10; }
    8
}
/// $07D0BA: clr.w $2c(a6)
pub fn i_07d0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0BE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D0BE: move.w #$fc00, $28(a6)
pub fn i_07d0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D0C4: btst.b #$0, $3a(a6)
pub fn i_07d0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0CA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D0CA: beq.w $7d0d2
pub fn i_07d0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D0D2; return 10; }
    8
}
/// $07D0CE: neg.w $28(a6)
pub fn i_07d0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0D2;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D0D2: jsr $28d70.l
pub fn i_07d0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D0D8);
    cpu.pc = 0x28D70;
    16
}
/// $07D0D8: tst.b $72(a6)
pub fn i_07d0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0DC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07D0DC: bne.w $7d0fa
pub fn i_07d0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D0FA; return 10; }
    8
}
/// $07D0E0: move.w $76(a6), d0
pub fn i_07d0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0E4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D0E4: add.w $28(a6), d0
pub fn i_07d0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0E8;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07D0E8: move.w d0, $76(a6)
pub fn i_07d0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0EC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D0EC: bpl.w $7d0f2
pub fn i_07d0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0F0;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7D0F2; return 10; }
    8
}
/// $07D0F0: neg.w d0
pub fn i_07d0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0F2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D0F2: cmpi.w #$6000, d0
pub fn i_07d0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0F6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6000, Size::Word, false);
    8
}
/// $07D0F6: blt.w $7d11e
pub fn i_07d0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D0FA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7D11E; return 10; }
    8
}
/// $07D0FA: move.b #$2, $20(a6)
pub fn i_07d0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D100;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D100: clr.b $72(a6)
pub fn i_07d100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D104;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07D104: move.w $76(a6), d0
pub fn i_07d104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D108;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D108: bpl.w $7d10e
pub fn i_07d108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D10C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7D10E; return 10; }
    8
}
/// $07D10C: neg.w d0
pub fn i_07d10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D10E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D10E: move.w d0, $78(a6)
pub fn i_07d10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D112;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D112: move.w #$28, $80(a6)
pub fn i_07d112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D118;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D118: lea.l $7d178(pc), a1
pub fn i_07d118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D11C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D178;
    16
}
/// $07D11C: move.l a1, (a6)
pub fn i_07d11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D11E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D11E: jsr $2870a.l
pub fn i_07d11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D124);
    cpu.pc = 0x2870A;
    16
}
/// $07D124: bcc.w $7d13a
pub fn i_07d124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D128;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D13A; return 10; }
    8
}
/// $07D128: lea.l $5e766.l, a0
pub fn i_07d128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D12E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07D12E: jsr $5e770.l
pub fn i_07d12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D134;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D134);
    cpu.pc = 0x5E770;
    16
}
/// $07D134: bclr.b #$3, $13(a6)
pub fn i_07d134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D13A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D13A: jsr $27eba.l
pub fn i_07d13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D140);
    cpu.pc = 0x27EBA;
    16
}
/// $07D140: bcc.w $7d14a
pub fn i_07d140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D144;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D14A; return 10; }
    8
}
/// $07D144: lea.l $7d3e2(pc), a1
pub fn i_07d144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D148;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3E2;
    16
}
/// $07D148: move.l a1, (a6)
pub fn i_07d148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D14A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D14A: jsr $28758.l
pub fn i_07d14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D150);
    cpu.pc = 0x28758;
    16
}
/// $07D150: bcc.w $7d15a
pub fn i_07d150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D154;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D15A; return 10; }
    8
}
/// $07D154: lea.l $7d332(pc), a1
pub fn i_07d154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D158;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D332;
    16
}
/// $07D158: move.l a1, (a6)
pub fn i_07d158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D15A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D15A: movea.l #$ffffffff, a0
pub fn i_07d15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D160;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07D160: lea.l $2e0b5c.l, a0
pub fn i_07d160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D166;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0B5C;
    20
}
/// $07D166: jsr $5dd5c.l
pub fn i_07d166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D16C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D16C);
    cpu.pc = 0x5DD5C;
    16
}
/// $07D16C: bcc.w $7d176
pub fn i_07d16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D170;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D176; return 10; }
    8
}
// ===== SetTaskHandler_07d170  ($07D170, 8 B, kernel/task_handlers.c)
/// $07D170: lea.l $7dba2(pc), a1
pub fn i_07d170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D174;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07D174: move.l a1, (a6)
pub fn i_07d174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D176;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D176: rts
pub fn i_07d176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D178;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Pause_07d178  ($07D178, 174 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D178: jsr $267e2.l
pub fn i_07d178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D17E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D17E);
    cpu.pc = 0x267E2;
    16
}
/// $07D17E: clr.w $74(a6)
pub fn i_07d17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D182;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D182: move.w #$109e, d0
pub fn i_07d182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D186;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D186: jsr $2222.l
pub fn i_07d186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D18C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D18C);
    cpu.pc = 0x2222;
    16
}
/// $07D18C: lea.l $2e04d6.l, a0
pub fn i_07d18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D192;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E04D6;
    20
}
/// $07D192: jsr $28cd4.l
pub fn i_07d192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D198;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D198);
    cpu.pc = 0x28CD4;
    16
}
/// $07D198: lea.l $7d19e(pc), a1
pub fn i_07d198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D19C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D19E;
    16
}
/// $07D19C: move.l a1, (a6)
pub fn i_07d19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D19E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D19E: jsr $2783a.l
pub fn i_07d19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D1A4);
    cpu.pc = 0x2783A;
    16
}
/// $07D1A4: jsr $28d70.l
pub fn i_07d1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D1AA);
    cpu.pc = 0x28D70;
    16
}
/// $07D1AA: tst.b $9b(a6)
pub fn i_07d1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1AE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07D1AE: beq.w $7d1d4
pub fn i_07d1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D1D4; return 10; }
    8
}
/// $07D1B2: cmpi.b #$4, $20(a6)
pub fn i_07d1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1B8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $07D1B8: beq.w $7d1d4
pub fn i_07d1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D1D4; return 10; }
    8
}
/// $07D1BC: subq.w #$1, $80(a6)
pub fn i_07d1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1C0;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D1C0: bne.w $7d1d4
pub fn i_07d1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1C4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D1D4; return 10; }
    8
}
/// $07D1C4: move.b #$3, $20(a6)
pub fn i_07d1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1CA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D1CA: clr.w $76(a6)
pub fn i_07d1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1CE;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D1CE: lea.l $7d22e(pc), a1
pub fn i_07d1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D22E;
    16
}
/// $07D1D2: move.l a1, (a6)
pub fn i_07d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1D4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D1D4: jsr $2870a.l
pub fn i_07d1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D1DA);
    cpu.pc = 0x2870A;
    16
}
/// $07D1DA: bcc.w $7d1f0
pub fn i_07d1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D1F0; return 10; }
    8
}
/// $07D1DE: lea.l $5e766.l, a0
pub fn i_07d1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07D1E4: jsr $5e770.l
pub fn i_07d1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D1EA);
    cpu.pc = 0x5E770;
    16
}
/// $07D1EA: bclr.b #$3, $13(a6)
pub fn i_07d1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1F0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D1F0: jsr $27eba.l
pub fn i_07d1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D1F6);
    cpu.pc = 0x27EBA;
    16
}
/// $07D1F6: bcc.w $7d200
pub fn i_07d1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D200; return 10; }
    8
}
/// $07D1FA: lea.l $7d3e2(pc), a1
pub fn i_07d1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D1FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3E2;
    16
}
/// $07D1FE: move.l a1, (a6)
pub fn i_07d1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D200;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D200: jsr $28758.l
pub fn i_07d200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D206;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D206);
    cpu.pc = 0x28758;
    16
}
/// $07D206: bcc.w $7d210
pub fn i_07d206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D20A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D210; return 10; }
    8
}
/// $07D20A: lea.l $7d332(pc), a1
pub fn i_07d20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D20E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D332;
    16
}
/// $07D20E: move.l a1, (a6)
pub fn i_07d20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D210;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D210: movea.l #$ffffffff, a0
pub fn i_07d210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D216;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07D216: lea.l $2e0b5c.l, a0
pub fn i_07d216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D21C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0B5C;
    20
}
/// $07D21C: jsr $5dd5c.l
pub fn i_07d21c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D222);
    cpu.pc = 0x5DD5C;
    16
}
/// $07D222: bcc.w $7d22c
pub fn i_07d222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D226;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D22C; return 10; }
    8
}
// ===== SetTaskHandler_07d226  ($07D226, 8 B, kernel/task_handlers.c)
/// $07D226: lea.l $7dba2(pc), a1
pub fn i_07d226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D22A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07D22A: move.l a1, (a6)
pub fn i_07d22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D22C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D22C: rts
pub fn i_07d22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D22E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Retreat_07d22e  ($07D22E, 252 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D22E: jsr $267e2.l
pub fn i_07d22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D234;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D234);
    cpu.pc = 0x267E2;
    16
}
/// $07D234: move.w #$20, $28(a6)
pub fn i_07d234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D23A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D23A: btst.b #$0, $3a(a6)
pub fn i_07d23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D240;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D240: beq.w $7d248
pub fn i_07d240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D244;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D248; return 10; }
    8
}
/// $07D244: neg.w $28(a6)
pub fn i_07d244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D248;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D248: move.w $28(a6), $2c(a6)
pub fn i_07d248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D24E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07D24E: moveq #$0, d0
pub fn i_07d24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D250;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07D250: move.w d0, $28(a6)
pub fn i_07d250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D254;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D254: move.w d0, $74(a6)
pub fn i_07d254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D258;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D258: lea.l $2e057e.l, a0
pub fn i_07d258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D25E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E057E;
    20
}
/// $07D25E: jsr $28cd4.l
pub fn i_07d25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D264;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D264);
    cpu.pc = 0x28CD4;
    16
}
/// $07D264: lea.l $7d26a(pc), a1
pub fn i_07d264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D268;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D26A;
    16
}
/// $07D268: move.l a1, (a6)
pub fn i_07d268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D26A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D26A: jsr $27afc.l
pub fn i_07d26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D270);
    cpu.pc = 0x27AFC;
    16
}
/// $07D270: bcc.w $7d280
pub fn i_07d270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D274;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D280; return 10; }
    8
}
/// $07D274: move.b #$4, $20(a6)
pub fn i_07d274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D27A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D27A: lea.l $7d178(pc), a1
pub fn i_07d27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D27E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D178;
    16
}
/// $07D27E: move.l a1, (a6)
pub fn i_07d27e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D280;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D280: cmpi.w #$fc00, $28(a6)
pub fn i_07d280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D286;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFC00, Size::Word, false);
    16
}
/// $07D286: blt.w $7d294
pub fn i_07d286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D28A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7D294; return 10; }
    8
}
/// $07D28A: cmpi.w #$400, $28(a6)
pub fn i_07d28a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D290;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x400, Size::Word, false);
    16
}
/// $07D290: blt.w $7d2ac
pub fn i_07d290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D294;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7D2AC; return 10; }
    8
}
/// $07D294: clr.w $2c(a6)
pub fn i_07d294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D298;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D298: move.w #$400, $28(a6)
pub fn i_07d298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D29E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D29E: btst.b #$0, $3a(a6)
pub fn i_07d29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2A4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D2A4: beq.w $7d2ac
pub fn i_07d2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2A8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D2AC; return 10; }
    8
}
/// $07D2A8: neg.w $28(a6)
pub fn i_07d2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2AC;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D2AC: jsr $28d70.l
pub fn i_07d2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D2B2);
    cpu.pc = 0x28D70;
    16
}
/// $07D2B2: move.w $76(a6), d0
pub fn i_07d2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2B6;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D2B6: add.w $28(a6), d0
pub fn i_07d2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2BA;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07D2BA: move.w d0, $76(a6)
pub fn i_07d2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2BE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D2BE: bpl.w $7d2c4
pub fn i_07d2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2C2;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7D2C4; return 10; }
    8
}
/// $07D2C2: neg.w d0
pub fn i_07d2c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2C4;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D2C4: cmp.w $78(a6), d0
pub fn i_07d2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2C8;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07D2C8: blt.w $7d2d8
pub fn i_07d2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2CC;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7D2D8; return 10; }
    8
}
/// $07D2CC: move.b #$4, $20(a6)
pub fn i_07d2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D2D2: lea.l $7d178(pc), a1
pub fn i_07d2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D178;
    16
}
/// $07D2D6: move.l a1, (a6)
pub fn i_07d2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D2D8: jsr $2870a.l
pub fn i_07d2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D2DE);
    cpu.pc = 0x2870A;
    16
}
/// $07D2DE: bcc.w $7d2f4
pub fn i_07d2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D2F4; return 10; }
    8
}
/// $07D2E2: lea.l $5e766.l, a0
pub fn i_07d2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07D2E8: jsr $5e770.l
pub fn i_07d2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D2EE);
    cpu.pc = 0x5E770;
    16
}
/// $07D2EE: bclr.b #$3, $13(a6)
pub fn i_07d2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D2F4: jsr $27eba.l
pub fn i_07d2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D2FA);
    cpu.pc = 0x27EBA;
    16
}
/// $07D2FA: bcc.w $7d304
pub fn i_07d2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D2FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D304; return 10; }
    8
}
/// $07D2FE: lea.l $7d3e2(pc), a1
pub fn i_07d2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D302;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3E2;
    16
}
/// $07D302: move.l a1, (a6)
pub fn i_07d302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D304;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D304: jsr $28758.l
pub fn i_07d304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D30A);
    cpu.pc = 0x28758;
    16
}
/// $07D30A: bcc.w $7d314
pub fn i_07d30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D30E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D314; return 10; }
    8
}
/// $07D30E: lea.l $7d332(pc), a1
pub fn i_07d30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D312;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D332;
    16
}
/// $07D312: move.l a1, (a6)
pub fn i_07d312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D314;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D314: movea.l #$ffffffff, a0
pub fn i_07d314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D31A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07D31A: lea.l $2e0b5c.l, a0
pub fn i_07d31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D320;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0B5C;
    20
}
/// $07D320: jsr $5dd5c.l
pub fn i_07d320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D326;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D326);
    cpu.pc = 0x5DD5C;
    16
}
/// $07D326: bcc.w $7d330
pub fn i_07d326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D32A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D330; return 10; }
    8
}
// ===== SetTaskHandler_07d32a  ($07D32A, 8 B, kernel/task_handlers.c)
/// $07D32A: lea.l $7dba2(pc), a1
pub fn i_07d32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D32E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07D32E: move.l a1, (a6)
pub fn i_07d32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D330;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D330: rts
pub fn i_07d330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D332;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Death_07d332  ($07D332, 118 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D332: lea.l $77fd6.l, a1
pub fn i_07d332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D338;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07D338: jsr $4ae.l
pub fn i_07d338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D33E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D33E);
    cpu.pc = 0x4AE;
    16
}
/// $07D33E: jsr $5dd02.l
pub fn i_07d33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D344;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D344);
    cpu.pc = 0x5DD02;
    16
}
/// $07D344: move.b #$ff, $20(a6)
pub fn i_07d344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D34A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D34A: bclr.b #$1, $12(a6)
pub fn i_07d34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D350;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D350: lea.l $2e064a.l, a0
pub fn i_07d350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D356;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E064A;
    20
}
/// $07D356: jsr $28cd4.l
pub fn i_07d356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D35C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D35C);
    cpu.pc = 0x28CD4;
    16
}
/// $07D35C: move.w #$109e, d0
pub fn i_07d35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D360;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D360: jsr $2222.l
pub fn i_07d360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D366;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D366);
    cpu.pc = 0x2222;
    16
}
/// $07D366: lea.l $7d36c(pc), a1
pub fn i_07d366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D36A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D36C;
    16
}
/// $07D36A: move.l a1, (a6)
pub fn i_07d36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D36C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D36C: jsr $2783a.l
pub fn i_07d36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D372;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D372);
    cpu.pc = 0x2783A;
    16
}
/// $07D372: jsr $28d70.l
pub fn i_07d372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D378;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D378);
    cpu.pc = 0x28D70;
    16
}
/// $07D378: bcc.w $7d382
pub fn i_07d378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D37C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D382; return 10; }
    8
}
/// $07D37C: lea.l $7d3b0(pc), a1
pub fn i_07d37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D380;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3B0;
    16
}
/// $07D380: move.l a1, (a6)
pub fn i_07d380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D382;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D382: jsr $27eba.l
pub fn i_07d382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D388;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D388);
    cpu.pc = 0x27EBA;
    16
}
/// $07D388: bcc.w $7d392
pub fn i_07d388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D38C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D392; return 10; }
    8
}
/// $07D38C: lea.l $7d3b0(pc), a1
pub fn i_07d38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D390;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3B0;
    16
}
/// $07D390: move.l a1, (a6)
pub fn i_07d390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D392;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D392: movea.l #$ffffffff, a0
pub fn i_07d392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D398;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07D398: lea.l $2e0b5c.l, a0
pub fn i_07d398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D39E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0B5C;
    20
}
/// $07D39E: jsr $5dd5c.l
pub fn i_07d39e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D3A4);
    cpu.pc = 0x5DD5C;
    16
}
/// $07D3A4: bcc.w $7d3ae
pub fn i_07d3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D3AE; return 10; }
    8
}
// ===== SetTaskHandler_07d3a8  ($07D3A8, 8 B, kernel/task_handlers.c)
/// $07D3A8: lea.l $7dba2(pc), a1
pub fn i_07d3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07D3AC: move.l a1, (a6)
pub fn i_07d3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D3AE: rts
pub fn i_07d3ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Explode_07d3b0  ($07D3B0, 50 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D3B0: move.w #$c000, d0
pub fn i_07d3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3B4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D3B4: jsr $28134.l
pub fn i_07d3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D3BA);
    cpu.pc = 0x28134;
    16
}
/// $07D3BA: andi.w #$ffe3, $38(a6)
pub fn i_07d3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3C0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07D3C0: ori.w #$14, $38(a6)
pub fn i_07d3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3C6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07D3C6: lea.l $2e0348.l, a1
pub fn i_07d3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E0348;
    20
}
/// $07D3CC: jsr $77c7e.l
pub fn i_07d3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D3D2);
    cpu.pc = 0x77C7E;
    16
}
/// $07D3D2: move.w #$1033, d0
pub fn i_07d3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3D6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D3D6: jsr $2352.l
pub fn i_07d3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D3DC);
    cpu.pc = 0x2352;
    16
}
/// $07D3DC: jmp $77f6a.l
pub fn i_07d3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3E2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
// ===== Crab_Knockback_07d3e2  ($07D3E2, 192 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D3E2: move.b #$0, $21(a6)
pub fn i_07d3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D3E8: move.b #$80, $20(a6)
pub fn i_07d3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3EE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D3EE: bclr.b #$1, $12(a6)
pub fn i_07d3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3F4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D3F4: asr.w $28(a6)
pub fn i_07d3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3F8;
    cpu.ir = 0xE0EE;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = do_shift_rotate(cpu, v2, 1, Size::Word, 0, false);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07D3F8: move.w #$0, $2c(a6)
pub fn i_07d3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D3FE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D3FE: move.w #$fff0, $2e(a6)
pub fn i_07d3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D404;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D404: move.w #$c000, d0
pub fn i_07d404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D408;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D408: jsr $28134.l
pub fn i_07d408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D40E);
    cpu.pc = 0x28134;
    16
}
/// $07D40E: andi.w #$ffe3, $38(a6)
pub fn i_07d40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D414;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07D414: ori.w #$1c, $38(a6)
pub fn i_07d414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D41A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07D41A: lea.l $2e075a.l, a0
pub fn i_07d41a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D420;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E075A;
    20
}
/// $07D420: jsr $28cd4.l
pub fn i_07d420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D426;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D426);
    cpu.pc = 0x28CD4;
    16
}
/// $07D426: move.w #$109e, d0
pub fn i_07d426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D42A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D42A: jsr $2222.l
pub fn i_07d42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D430);
    cpu.pc = 0x2222;
    16
}
/// $07D430: lea.l $7d436(pc), a1
pub fn i_07d430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D434;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D436;
    16
}
/// $07D434: move.l a1, (a6)
pub fn i_07d434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D436;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D436: bset.b #$6, $13(a6)
pub fn i_07d436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D43C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D43C: jsr $27cee.l
pub fn i_07d43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D442);
    cpu.pc = 0x27CEE;
    16
}
/// $07D442: bclr.b #$6, $13(a6)
pub fn i_07d442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D448;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D448: jsr $44048.l
pub fn i_07d448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D44E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D44E);
    cpu.pc = 0x44048;
    16
}
/// $07D44E: jsr $28d70.l
pub fn i_07d44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D454);
    cpu.pc = 0x28D70;
    16
}
/// $07D454: cmpi.w #$120, $24(a6)
pub fn i_07d454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D45A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $07D45A: bgt.w $7d48c
pub fn i_07d45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D45E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7D48C; return 10; }
    8
}
/// $07D45E: tst.b $21(a6)
pub fn i_07d45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D462;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07D462: bne.w $7d48c
pub fn i_07d462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D466;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D48C; return 10; }
    8
}
/// $07D466: move.b #$ff, $21(a6)
pub fn i_07d466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D46C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D46C: tst.b $84(a6)
pub fn i_07d46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D470;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07D470: bne.w $7d48c
pub fn i_07d470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D474;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D48C; return 10; }
    8
}
/// $07D474: lea.l $78864.l, a1
pub fn i_07d474(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D47A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78864;
    20
}
/// $07D47A: jsr $4ae.l
pub fn i_07d47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D480;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D480);
    cpu.pc = 0x4AE;
    16
}
/// $07D480: jsr $5dd02.l
pub fn i_07d480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D486;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D486);
    cpu.pc = 0x5DD02;
    16
}
/// $07D486: move.w #$ffff, $38(a0)
pub fn i_07d486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D48C;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D48C: movea.l #$ffffffff, a0
pub fn i_07d48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D492;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07D492: lea.l $2e0340.l, a0
pub fn i_07d492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D498;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0340;
    20
}
/// $07D498: jsr $5dd5c.l
pub fn i_07d498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D49E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D49E);
    cpu.pc = 0x5DD5C;
    16
}
/// $07D49E: bcc.w $7d4a8
pub fn i_07d49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D4A8; return 10; }
    8
}
// ===== SetTaskHandler_07d4a2  ($07D4A2, 8 B, kernel/task_handlers.c)
/// $07D4A2: lea.l $7dba2(pc), a1
pub fn i_07d4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07D4A6: move.l a1, (a6)
pub fn i_07d4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D4A8: rts
pub fn i_07d4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Claw_Init_07d4aa  ($07D4AA, 186 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D4AA: move.w #$4b, d1
pub fn i_07d4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D4AE: jsr $236e.l
pub fn i_07d4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D4B4);
    cpu.pc = 0x236E;
    16
}
/// $07D4B4: move.w #$a, $1c(a6)
pub fn i_07d4b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D4BA: jsr $138fe.l
pub fn i_07d4ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D4C0);
    cpu.pc = 0x138FE;
    16
}
/// $07D4C0: jsr $267e2.l
pub fn i_07d4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D4C6);
    cpu.pc = 0x267E2;
    16
}
/// $07D4C6: moveq #$0, d0
pub fn i_07d4c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4C8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07D4C8: move.b d0, $20(a6)
pub fn i_07d4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4CC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D4CC: move.b d0, $21(a6)
pub fn i_07d4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4D0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D4D0: move.w d0, $72(a6)
pub fn i_07d4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4D4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D4D4: move.w #$1, $82(a6)
pub fn i_07d4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4DA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D4DA: lea.l $7d6a6(pc), a1
pub fn i_07d4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D6A6;
    16
}
/// $07D4DE: jsr $4ae.l
pub fn i_07d4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D4E4);
    cpu.pc = 0x4AE;
    16
}
/// $07D4E4: jsr $5dd02.l
pub fn i_07d4e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D4EA);
    cpu.pc = 0x5DD02;
    16
}
/// $07D4EA: lea.l $7d6b6(pc), a1
pub fn i_07d4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D6B6;
    16
}
/// $07D4EE: jsr $4ae.l
pub fn i_07d4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D4F4);
    cpu.pc = 0x4AE;
    16
}
/// $07D4F4: jsr $5dd02.l
pub fn i_07d4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D4FA);
    cpu.pc = 0x5DD02;
    16
}
/// $07D4FA: lea.l $7d6c6(pc), a1
pub fn i_07d4fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D4FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D6C6;
    16
}
/// $07D4FE: jsr $4ae.l
pub fn i_07d4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D504);
    cpu.pc = 0x4AE;
    16
}
/// $07D504: jsr $5dd02.l
pub fn i_07d504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D50A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D50A);
    cpu.pc = 0x5DD02;
    16
}
/// $07D50A: lea.l $2e0770.l, a0
pub fn i_07d50a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D510;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0770;
    20
}
/// $07D510: jsr $28cd4.l
pub fn i_07d510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D516);
    cpu.pc = 0x28CD4;
    16
}
/// $07D516: lea.l $7d564(pc), a1
pub fn i_07d516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D51A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D564;
    16
}
/// $07D51A: move.l a1, (a6)
pub fn i_07d51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D51C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D51C: bra.w $7d564
pub fn i_07d51c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D520;
    cpu.ir = 0x6000;
    cpu.pc = 0x7D564;
    10
}
// ===== Crab_Claw_Follow_07d564  ($07D564, 84 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D564: jsr $7dc1a(pc)
pub fn i_07d564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D568;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7D568);
    cpu.pc = 0x7DC1A;
    16
}
/// $07D568: movea.l $c(a6), a0
pub fn i_07d568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D56C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D56C: btst.b #$7, $5a(a0)
pub fn i_07d56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D572;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D572: beq.w $7d57c
pub fn i_07d572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D576;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D57C; return 10; }
    8
}
/// $07D576: bset.b #$0, $5a(a6)
pub fn i_07d576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D57C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D57C: jsr $28d70.l
pub fn i_07d57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D582;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D582);
    cpu.pc = 0x28D70;
    16
}
/// $07D582: movea.l $c(a6), a0
pub fn i_07d582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D586;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D586: cmpi.b #$1, $20(a0)
pub fn i_07d586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D58C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07D58C: bne.w $7d596
pub fn i_07d58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D590;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D596; return 10; }
    8
}
/// $07D590: lea.l $7d5c0(pc), a1
pub fn i_07d590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D594;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D5C0;
    16
}
/// $07D594: move.l a1, (a6)
pub fn i_07d594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D596;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D596: movea.l $c(a6), a0
pub fn i_07d596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D59A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D59A: cmpi.b #$ff, $20(a0)
pub fn i_07d59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5A0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07D5A0: beq.w $7d5b8
pub fn i_07d5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D5B8; return 10; }
    8
}
/// $07D5A4: cmpi.b #$80, $20(a0)
pub fn i_07d5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5AA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $07D5AA: beq.w $7d5b8
pub fn i_07d5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D5B8; return 10; }
    8
}
/// $07D5AE: cmpi.b #$40, $20(a0)
pub fn i_07d5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5B4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $07D5B4: bne.w $7d5be
pub fn i_07d5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D5BE; return 10; }
    8
}
// ===== SetTaskHandler_07d5b8  ($07D5B8, 8 B, kernel/task_handlers.c)
/// $07D5B8: lea.l $7dba8(pc), a1
pub fn i_07d5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D5BC: move.l a1, (a6)
pub fn i_07d5bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D5BE: rts
pub fn i_07d5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Claw_Open_07d5c0  ($07D5C0, 112 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D5C0: move.w $72(a6), d0
pub fn i_07d5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5C4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D5C4: movea.l #$2e0456, a0
pub fn i_07d5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5CA;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E0456;
    cpu.a[0] = v1;
    4
}
/// $07D5CA: lsl.w #$2, d0
pub fn i_07d5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5CC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07D5CC: movea.l (a0, d0.w), a0
pub fn i_07d5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5D0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07D5D0: cmpa.l #$ffffffff, a0
pub fn i_07d5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5D6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07D5D6: beq.w $7d5e0
pub fn i_07d5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5DA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D5E0; return 10; }
    8
}
/// $07D5DA: jsr $28cd4.l
pub fn i_07d5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D5E0);
    cpu.pc = 0x28CD4;
    16
}
/// $07D5E0: lea.l $7d5e6(pc), a1
pub fn i_07d5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5E4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D5E6;
    16
}
/// $07D5E4: move.l a1, (a6)
pub fn i_07d5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5E6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D5E6: jsr $7dc1a(pc)
pub fn i_07d5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7D5EA);
    cpu.pc = 0x7DC1A;
    16
}
/// $07D5EA: movea.l $c(a6), a0
pub fn i_07d5ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5EE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D5EE: btst.b #$7, $5a(a0)
pub fn i_07d5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5F4;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D5F4: beq.w $7d5fe
pub fn i_07d5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5F8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D5FE; return 10; }
    8
}
/// $07D5F8: bset.b #$0, $5a(a6)
pub fn i_07d5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D5FE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D5FE: jsr $28d70.l
pub fn i_07d5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D604);
    cpu.pc = 0x28D70;
    16
}
/// $07D604: bcc.w $7d60e
pub fn i_07d604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D608;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D60E; return 10; }
    8
}
/// $07D608: lea.l $7d638(pc), a1
pub fn i_07d608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D60C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D638;
    16
}
/// $07D60C: move.l a1, (a6)
pub fn i_07d60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D60E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D60E: movea.l $c(a6), a0
pub fn i_07d60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D612;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D612: cmpi.b #$ff, $20(a0)
pub fn i_07d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D618;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07D618: beq.w $7d630
pub fn i_07d618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D61C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D630; return 10; }
    8
}
/// $07D61C: cmpi.b #$80, $20(a0)
pub fn i_07d61c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D622;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $07D622: beq.w $7d630
pub fn i_07d622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D626;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D630; return 10; }
    8
}
/// $07D626: cmpi.b #$40, $20(a0)
pub fn i_07d626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D62C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $07D62C: bne.w $7d636
pub fn i_07d62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D630;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D636; return 10; }
    8
}
// ===== SetTaskHandler_07d630  ($07D630, 8 B, kernel/task_handlers.c)
/// $07D630: lea.l $7dba8(pc), a1
pub fn i_07d630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D634;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D634: move.l a1, (a6)
pub fn i_07d634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D636;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D636: rts
pub fn i_07d636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D638;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Claw_Close_07d638  ($07D638, 102 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D638: move.w $72(a6), d0
pub fn i_07d638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D63C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D63C: movea.l #$2e044e, a0
pub fn i_07d63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D642;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E044E;
    cpu.a[0] = v1;
    4
}
/// $07D642: lsl.w #$2, d0
pub fn i_07d642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D644;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07D644: movea.l (a0, d0.w), a0
pub fn i_07d644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D648;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07D648: cmpa.l #$ffffffff, a0
pub fn i_07d648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D64E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07D64E: beq.w $7d658
pub fn i_07d64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D652;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D658; return 10; }
    8
}
/// $07D652: jsr $28cd4.l
pub fn i_07d652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D658;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D658);
    cpu.pc = 0x28CD4;
    16
}
/// $07D658: lea.l $7d65e(pc), a1
pub fn i_07d658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D65C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D65E;
    16
}
/// $07D65C: move.l a1, (a6)
pub fn i_07d65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D65E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D65E: jsr $7dc1a(pc)
pub fn i_07d65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D662;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7D662);
    cpu.pc = 0x7DC1A;
    16
}
/// $07D662: movea.l $c(a6), a0
pub fn i_07d662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D666;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D666: btst.b #$7, $5a(a0)
pub fn i_07d666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D66C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D66C: beq.w $7d676
pub fn i_07d66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D670;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D676; return 10; }
    8
}
/// $07D670: bset.b #$0, $5a(a6)
pub fn i_07d670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D676;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D676: jsr $28d70.l
pub fn i_07d676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D67C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D67C);
    cpu.pc = 0x28D70;
    16
}
/// $07D67C: movea.l $c(a6), a0
pub fn i_07d67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D680;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D680: cmpi.b #$ff, $20(a0)
pub fn i_07d680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D686;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07D686: beq.w $7d69e
pub fn i_07d686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D68A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D69E; return 10; }
    8
}
/// $07D68A: cmpi.b #$80, $20(a0)
pub fn i_07d68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D690;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $07D690: beq.w $7d69e
pub fn i_07d690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D694;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D69E; return 10; }
    8
}
/// $07D694: cmpi.b #$40, $20(a0)
pub fn i_07d694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D69A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $07D69A: bne.w $7d6a4
pub fn i_07d69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D69E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D6A4; return 10; }
    8
}
// ===== SetTaskHandler_07d69e  ($07D69E, 8 B, kernel/task_handlers.c)
/// $07D69E: lea.l $7dba8(pc), a1
pub fn i_07d69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D6A2: move.l a1, (a6)
pub fn i_07d6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D6A4: rts
pub fn i_07d6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Leg_Init_07d6a6  ($07D6A6, 192 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D6A6: move.w #$0, $70(a6)
pub fn i_07d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D6AC: move.w #$26, $74(a6)
pub fn i_07d6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6B2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x26;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D6B2: bra.w $7d6d2
pub fn i_07d6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6B6;
    cpu.ir = 0x6000;
    cpu.pc = 0x7D6D2;
    10
}
/// $07D6D2: move.w #$4c, d1
pub fn i_07d6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6D6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D6D6: jsr $236e.l
pub fn i_07d6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D6DC);
    cpu.pc = 0x236E;
    16
}
/// $07D6DC: moveq #$0, d0
pub fn i_07d6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6DE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07D6DE: move.b d0, $20(a6)
pub fn i_07d6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6E2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D6E2: move.b d0, $21(a6)
pub fn i_07d6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6E6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07D6E6: bset.b #$4, $6b(a6)
pub fn i_07d6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6EC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D6EC: move.w #$1, $66(a6)
pub fn i_07d6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D6F2: jsr $267e2.l
pub fn i_07d6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D6F8);
    cpu.pc = 0x267E2;
    16
}
/// $07D6F8: jsr $5e9b6.l
pub fn i_07d6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D6FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D6FE);
    cpu.pc = 0x5E9B6;
    16
}
/// $07D6FE: andi.w #$7, d0
pub fn i_07d6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D702;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07D702: subq.w #$4, d0
pub fn i_07d702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D704;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D704: move.w d0, $72(a6)
pub fn i_07d704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D708;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D708: lea.l $2e08ac.l, a0
pub fn i_07d708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D70E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E08AC;
    20
}
/// $07D70E: jsr $28cd4.l
pub fn i_07d70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D714);
    cpu.pc = 0x28CD4;
    16
}
/// $07D714: lea.l $7d71a(pc), a1
pub fn i_07d714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D718;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D71A;
    16
}
/// $07D718: move.l a1, (a6)
pub fn i_07d718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D71A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D71A: jsr $7dbda(pc)
pub fn i_07d71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D71E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7D71E);
    cpu.pc = 0x7DBDA;
    16
}
/// $07D71E: jsr $28d70.l
pub fn i_07d71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D724);
    cpu.pc = 0x28D70;
    16
}
/// $07D724: movea.l $c(a6), a0
pub fn i_07d724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D728;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D728: movea.l $c(a0), a0
pub fn i_07d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D72C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D72C: cmpi.b #$1, $20(a0)
pub fn i_07d72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D732;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07D732: bne.w $7d73c
pub fn i_07d732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D736;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D73C; return 10; }
    8
}
/// $07D736: lea.l $7d76e(pc), a1
pub fn i_07d736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D73A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D76E;
    16
}
/// $07D73A: move.l a1, (a6)
pub fn i_07d73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D73C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D73C: cmpi.b #$ff, $20(a0)
pub fn i_07d73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D742;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07D742: bne.w $7d74c
pub fn i_07d742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D746;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D74C; return 10; }
    8
}
/// $07D746: lea.l $7dba8(pc), a1
pub fn i_07d746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D74A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D74A: move.l a1, (a6)
pub fn i_07d74a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D74C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D74C: cmpi.b #$80, $20(a0)
pub fn i_07d74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D752;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $07D752: bne.w $7d75c
pub fn i_07d752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D756;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D75C; return 10; }
    8
}
/// $07D756: lea.l $7dba8(pc), a1
pub fn i_07d756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D75A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D75A: move.l a1, (a6)
pub fn i_07d75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D75C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D75C: cmpi.b #$40, $20(a0)
pub fn i_07d75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D762;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $07D762: bne.w $7d76c
pub fn i_07d762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D766;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D76C; return 10; }
    8
}
// ===== SetTaskHandler_07d766  ($07D766, 8 B, kernel/task_handlers.c)
/// $07D766: lea.l $7dba8(pc), a1
pub fn i_07d766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D76A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D76A: move.l a1, (a6)
pub fn i_07d76a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D76C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D76C: rts
pub fn i_07d76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D76E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Leg_Step_07d76e  ($07D76E, 290 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D76E: jsr $267e2.l
pub fn i_07d76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D774;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D774);
    cpu.pc = 0x267E2;
    16
}
/// $07D774: clr.w $36(a6)
pub fn i_07d774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D778;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07D778: lea.l $2e08cc.l, a0
pub fn i_07d778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D77E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E08CC;
    20
}
/// $07D77E: jsr $28cd4.l
pub fn i_07d77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D784;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D784);
    cpu.pc = 0x28CD4;
    16
}
/// $07D784: lea.l $7d78a(pc), a1
pub fn i_07d784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D788;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D78A;
    16
}
/// $07D788: move.l a1, (a6)
pub fn i_07d788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D78A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D78A: jsr $7dbda(pc)
pub fn i_07d78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D78E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7D78E);
    cpu.pc = 0x7DBDA;
    16
}
/// $07D78E: lea.l $2e045e.l, a0
pub fn i_07d78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D794;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E045E;
    20
}
/// $07D794: move.w $70(a6), d0
pub fn i_07d794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D798;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D798: add.w d0, d0
pub fn i_07d798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D79A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D79A: add.w d0, d0
pub fn i_07d79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D79C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D79C: movea.l (a0, d0.w), a0
pub fn i_07d79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7A0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07D7A0: movea.l $c(a6), a1
pub fn i_07d7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7A4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07D7A4: move.b $20(a1), d0
pub fn i_07d7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7A8;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07D7A8: andi.w #$ff, d0
pub fn i_07d7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7AC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07D7AC: add.w d0, d0
pub fn i_07d7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7AE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D7AE: add.w d0, d0
pub fn i_07d7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7B0;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D7B0: move.w (a0, d0.w), d1
pub fn i_07d7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7B4;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07D7B4: btst.b #$0, $3a(a6)
pub fn i_07d7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7BA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D7BA: beq.w $7d7c0
pub fn i_07d7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D7C0; return 10; }
    8
}
/// $07D7BE: neg.w d1
pub fn i_07d7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7C0;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D7C0: add.w $22(a6), d1
pub fn i_07d7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7C4;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07D7C4: move.w d1, $22(a6)
pub fn i_07d7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7C8;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D7C8: move.w $22(a6), $76(a6)
pub fn i_07d7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7CE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07D7CE: move.w $2(a0, d0.w), d1
pub fn i_07d7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7D2;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07D7D2: add.w $24(a6), d1
pub fn i_07d7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7D6;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07D7D6: move.w d1, $24(a6)
pub fn i_07d7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7DA;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D7DA: lea.l $2e03f8.l, a0
pub fn i_07d7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E03F8;
    20
}
/// $07D7E0: move.w $70(a6), d0
pub fn i_07d7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7E4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D7E4: add.w d0, d0
pub fn i_07d7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7E6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D7E6: add.w d0, d0
pub fn i_07d7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7E8;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D7E8: movea.l (a0, d0.w), a0
pub fn i_07d7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7EC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07D7EC: move.b $20(a1), d0
pub fn i_07d7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7F0;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07D7F0: andi.w #$ff, d0
pub fn i_07d7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7F4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07D7F4: add.w d0, d0
pub fn i_07d7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7F6;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D7F6: move.w (a0, d0.w), d1
pub fn i_07d7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7FA;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07D7FA: add.w $36(a6), d1
pub fn i_07d7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D7FE;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07D7FE: move.w d1, $36(a6)
pub fn i_07d7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D802;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D802: lea.l $2e03e6.l, a0
pub fn i_07d802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D808;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E03E6;
    20
}
/// $07D808: move.w (a0, d0.w), d0
pub fn i_07d808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D80C;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07D80C: btst.b #$0, $3a(a6)
pub fn i_07d80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D812;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D812: beq.w $7d820
pub fn i_07d812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D816;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D820; return 10; }
    8
}
/// $07D816: neg.w d0
pub fn i_07d816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D818;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D818: subi.w #$80, d0
pub fn i_07d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D81C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x80, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07D81C: andi.w #$ff, d0
pub fn i_07d81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D820;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07D820: jsr $13c0e.l
pub fn i_07d820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D826;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D826);
    cpu.pc = 0x13C0E;
    16
}
/// $07D826: move.w d1, $28(a6)
pub fn i_07d826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D82A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D82A: move.w d2, $2a(a6)
pub fn i_07d82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D82E;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D82E: jsr $7dbb0(pc)
pub fn i_07d82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D832;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7D832);
    cpu.pc = 0x7DBB0;
    16
}
/// $07D832: jsr $28d70.l
pub fn i_07d832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D838;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D838);
    cpu.pc = 0x28D70;
    16
}
/// $07D838: bcc.w $7d842
pub fn i_07d838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D83C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D842; return 10; }
    8
}
/// $07D83C: lea.l $7d898(pc), a1
pub fn i_07d83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D840;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D898;
    16
}
/// $07D840: move.l a1, (a6)
pub fn i_07d840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D842;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D842: movea.l $c(a6), a0
pub fn i_07d842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D846;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D846: movea.l $c(a0), a0
pub fn i_07d846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D84A;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D84A: cmpi.b #$ff, $20(a0)
pub fn i_07d84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D850;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07D850: bne.w $7d85a
pub fn i_07d850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D854;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D85A; return 10; }
    8
}
/// $07D854: lea.l $7dba8(pc), a1
pub fn i_07d854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D858;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D858: move.l a1, (a6)
pub fn i_07d858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D85A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D85A: cmpi.b #$80, $20(a0)
pub fn i_07d85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D860;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Byte, false);
    16
}
/// $07D860: bne.w $7d86a
pub fn i_07d860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D864;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D86A; return 10; }
    8
}
/// $07D864: lea.l $7dba8(pc), a1
pub fn i_07d864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D868;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D868: move.l a1, (a6)
pub fn i_07d868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D86A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D86A: cmpi.b #$40, $20(a0)
pub fn i_07d86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D870;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Byte, false);
    16
}
/// $07D870: bne.w $7d87a
pub fn i_07d870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D874;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D87A; return 10; }
    8
}
/// $07D874: lea.l $7dba8(pc), a1
pub fn i_07d874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D878;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07D878: move.l a1, (a6)
pub fn i_07d878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D87A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D87A: move.w $22(a6), d0
pub fn i_07d87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D87E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D87E: sub.w $76(a6), d0
pub fn i_07d87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D882;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07D882: bpl.w $7d888
pub fn i_07d882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D886;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7D888; return 10; }
    8
}
/// $07D886: neg.w d0
pub fn i_07d886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D888;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D888: cmpi.w #$1c, d0
pub fn i_07d888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D88C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1C, Size::Word, false);
    8
}
/// $07D88C: blt.w $7d896
pub fn i_07d88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D890;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7D896; return 10; }
    8
}
// ===== SetTaskHandler_07d890  ($07D890, 8 B, kernel/task_handlers.c)
/// $07D890: lea.l $7d898(pc), a1
pub fn i_07d890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D894;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D898;
    16
}
/// $07D894: move.l a1, (a6)
pub fn i_07d894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D896;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D896: rts
pub fn i_07d896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D898;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Leg_Detach_07d898  ($07D898, 422 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07D898: jsr $5e9b6.l
pub fn i_07d898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D89E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D89E);
    cpu.pc = 0x5E9B6;
    16
}
/// $07D89E: andi.w #$7f, d0
pub fn i_07d89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8A2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07D8A2: move.w $28(a6), d1
pub fn i_07d8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8A6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D8A6: asr.w #$5, d1
pub fn i_07d8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8A8;
    cpu.ir = 0xEA41;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $07D8A8: add.w d1, d0
pub fn i_07d8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8AA;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D8AA: move.w d0, $28(a6)
pub fn i_07d8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8AE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D8AE: move.w $2a(a6), d0
pub fn i_07d8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8B2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D8B2: asr.w #$4, d0
pub fn i_07d8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8B4;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07D8B4: move.w d0, $2a(a6)
pub fn i_07d8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8B8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D8B8: move.w #$0, $2c(a6)
pub fn i_07d8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D8BE: move.w #$ffe0, $2e(a6)
pub fn i_07d8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D8C4: jsr $5e9b6.l
pub fn i_07d8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D8CA);
    cpu.pc = 0x5E9B6;
    16
}
/// $07D8CA: andi.w #$3, d0
pub fn i_07d8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8CE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07D8CE: addq.w #$3, d0
pub fn i_07d8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8D0;
    cpu.ir = 0x5640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D8D0: add.w d0, d0
pub fn i_07d8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8D2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D8D2: add.w d0, d0
pub fn i_07d8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8D4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D8D4: lea.l $2e035a.l, a0
pub fn i_07d8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E035A;
    20
}
/// $07D8DA: move.l (a0, d0.w), $5c(a6)
pub fn i_07d8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8E0;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $07D8E0: move.w #$d000, d0
pub fn i_07d8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D8E4: jsr $28134.l
pub fn i_07d8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D8EA);
    cpu.pc = 0x28134;
    16
}
/// $07D8EA: andi.w #$ffe3, $38(a6)
pub fn i_07d8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8F0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07D8F0: ori.w #$14, $38(a6)
pub fn i_07d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8F6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07D8F6: cmpi.w #$2, $70(a6)
pub fn i_07d8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D8FC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $07D8FC: bne.w $7d90a
pub fn i_07d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D900;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D90A; return 10; }
    8
}
/// $07D900: move.w #$1061, d0
pub fn i_07d900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D904;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1061;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07D904: jsr $2352.l
pub fn i_07d904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D90A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D90A);
    cpu.pc = 0x2352;
    16
}
/// $07D90A: move.w #$0, $70(a6)
pub fn i_07d90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D910;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D910: move.w #$ffff, $78(a6)
pub fn i_07d910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D916;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D916: lea.l $2e094a.l, a0
pub fn i_07d916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D91C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E094A;
    20
}
/// $07D91C: jsr $28cd4.l
pub fn i_07d91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D922;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D922);
    cpu.pc = 0x28CD4;
    16
}
/// $07D922: lea.l $7d928(pc), a1
pub fn i_07d922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D926;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D928;
    16
}
/// $07D926: move.l a1, (a6)
pub fn i_07d926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D928;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07D928: jsr $27cee.l
pub fn i_07d928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D92E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D92E);
    cpu.pc = 0x27CEE;
    16
}
/// $07D92E: jsr $28d70.l
pub fn i_07d92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D934);
    cpu.pc = 0x28D70;
    16
}
/// $07D934: tst.w $78(a6)
pub fn i_07d934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D938;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07D938: beq.w $7d9e8
pub fn i_07d938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D93C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D9E8; return 10; }
    8
}
/// $07D93C: jsr $2870a.l
pub fn i_07d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D942;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D942);
    cpu.pc = 0x2870A;
    16
}
/// $07D942: bcc.w $7d99c
pub fn i_07d942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D946;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7D99C; return 10; }
    8
}
/// $07D946: lea.l $5e766.l, a0
pub fn i_07d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D94C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07D94C: jsr $5e770.l
pub fn i_07d94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D952);
    cpu.pc = 0x5E770;
    16
}
/// $07D952: bclr.b #$3, $13(a6)
pub fn i_07d952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D958;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D958: cmpi.b #$2, $58(a6)
pub fn i_07d958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D95E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $07D95E: beq.w $7d96c
pub fn i_07d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D962;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D96C; return 10; }
    8
}
/// $07D962: cmpi.b #$3, $58(a6)
pub fn i_07d962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D968;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07D968: bne.w $7d99c
pub fn i_07d968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D96C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D99C; return 10; }
    8
}
/// $07D96C: lea.l $2e0ab0.l, a0
pub fn i_07d96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D972;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0AB0;
    20
}
/// $07D972: jsr $28cd4.l
pub fn i_07d972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D978);
    cpu.pc = 0x28CD4;
    16
}
/// $07D978: move.l #$2e0376, $5c(a6)
pub fn i_07d978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D980;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E0376;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07D980: bset.b #$3, $13(a6)
pub fn i_07d980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D986;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07D986: movea.l $50(a6), a0
pub fn i_07d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D98A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07D98A: move.w $28(a0), d0
pub fn i_07d98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D98E;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D98E: add.w $28(a6), d0
pub fn i_07d98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D992;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07D992: move.w d0, $28(a6)
pub fn i_07d992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D996;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D996: move.w #$0, $78(a6)
pub fn i_07d996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D99C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07D99C: tst.w $78(a6)
pub fn i_07d99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9A0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07D9A0: beq.w $7d9e8
pub fn i_07d9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D9E8; return 10; }
    8
}
/// $07D9A4: lea.l $2e141a.l, a1
pub fn i_07d9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9AA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E141A;
    20
}
/// $07D9AA: move.w $70(a6), d1
pub fn i_07d9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9AE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07D9AE: add.w d1, d1
pub fn i_07d9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9B0;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D9B0: add.w d1, d1
pub fn i_07d9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9B2;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07D9B2: movea.l (a1, d1.w), a1
pub fn i_07d9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9B6;
    cpu.ir = 0x2271;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $07D9B6: moveq #$0, d1
pub fn i_07d9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9B8;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07D9B8: movea.l (a1, d1.w), a0
pub fn i_07d9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9BC;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $07D9BC: cmpa.l #$ffffffff, a0
pub fn i_07d9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9C2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $07D9C2: beq.w $7d9e8
pub fn i_07d9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D9E8; return 10; }
    8
}
/// $07D9C6: movem.l d1/a1, -(a7)
pub fn i_07d9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9CA;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $07D9CA: move.l a0, $4c(a6)
pub fn i_07d9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9CE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07D9CE: jsr $283ca.l
pub fn i_07d9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D9D4);
    cpu.pc = 0x283CA;
    16
}
/// $07D9D4: jsr $283d8.l
pub fn i_07d9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7D9DA);
    cpu.pc = 0x283D8;
    16
}
/// $07D9DA: movem.l (a7)+, d1/a1
pub fn i_07d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9DE;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $07D9DE: addq.w #$4, d1
pub fn i_07d9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9E0;
    cpu.ir = 0x5841;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07D9E0: btst.b #$1, $13(a6)
pub fn i_07d9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9E6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07D9E6: beq.b $7d9b8
pub fn i_07d9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9E8;
    cpu.ir = 0x67D0;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7D9B8; return 10; }
    8
}
/// $07D9E8: cmpi.w #$120, $24(a6)
pub fn i_07d9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9EE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $07D9EE: bgt.w $7da28
pub fn i_07d9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9F2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7DA28; return 10; }
    8
}
/// $07D9F2: tst.b $21(a6)
pub fn i_07d9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9F6;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07D9F6: bne.w $7da28
pub fn i_07d9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7D9FA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7DA28; return 10; }
    8
}
/// $07D9FA: move.b #$ff, $21(a6)
pub fn i_07d9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DA00: movea.l $c(a6), a0
pub fn i_07da00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA04;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07DA04: movea.l $c(a0), a0
pub fn i_07da04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA08;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07DA08: tst.b $84(a0)
pub fn i_07da08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA0C;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07DA0C: bne.w $7d48c
pub fn i_07da0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA10;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7D48C; return 10; }
    8
}
/// $07DA10: lea.l $78890.l, a1
pub fn i_07da10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA16;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x78890;
    20
}
/// $07DA16: jsr $4ae.l
pub fn i_07da16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DA1C);
    cpu.pc = 0x4AE;
    16
}
/// $07DA1C: jsr $5dd02.l
pub fn i_07da1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DA22);
    cpu.pc = 0x5DD02;
    16
}
/// $07DA22: move.w #$ffff, $38(a0)
pub fn i_07da22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA28;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DA28: movea.l #$ffffffff, a0
pub fn i_07da28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA2E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07DA2E: lea.l $2e0340.l, a0
pub fn i_07da2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0340;
    20
}
/// $07DA34: jsr $5dd56.l
pub fn i_07da34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DA3A);
    cpu.pc = 0x5DD56;
    16
}
/// $07DA3A: bcc.w $7da44
pub fn i_07da3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DA44; return 10; }
    8
}
// ===== SetTaskHandler_07da3e  ($07DA3E, 8 B, kernel/task_handlers.c)
/// $07DA3E: lea.l $7dba8(pc), a1
pub fn i_07da3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA8;
    16
}
/// $07DA42: move.l a1, (a6)
pub fn i_07da42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DA44: rts
pub fn i_07da44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Shard_Big_07da46  ($07DA46, 174 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DA46: move.w #$0, d0
pub fn i_07da46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA4A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DA4A: jsr $5dca4.l
pub fn i_07da4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DA50);
    cpu.pc = 0x5DCA4;
    16
}
/// $07DA50: move.w d0, $28(a6)
pub fn i_07da50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA54;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DA54: move.w #$190, $2a(a6)
pub fn i_07da54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA5A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DA5A: move.w #$fff0, $2e(a6)
pub fn i_07da5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA60;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DA60: move.w #$0, $2c(a6)
pub fn i_07da60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA66;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DA66: jsr $5e9b6.l
pub fn i_07da66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DA6C);
    cpu.pc = 0x5E9B6;
    16
}
/// $07DA6C: andi.w #$3ff, d0
pub fn i_07da6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA70;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DA70: move.w d0, $28(a6)
pub fn i_07da70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA74;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DA74: jsr $5e9b6.l
pub fn i_07da74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DA7A);
    cpu.pc = 0x5E9B6;
    16
}
/// $07DA7A: andi.w #$3ff, d0
pub fn i_07da7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA7E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DA7E: add.w d0, $2a(a6)
pub fn i_07da7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA82;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DA82: jsr $5e9b6.l
pub fn i_07da82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DA88);
    cpu.pc = 0x5E9B6;
    16
}
/// $07DA88: andi.w #$1, d0
pub fn i_07da88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA8C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DA8C: eor.b d0, $3a(a6)
pub fn i_07da8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA90;
    cpu.ir = 0xB12E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07DA90: btst.b #$0, $3a(a6)
pub fn i_07da90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA96;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07DA96: beq.w $7da9e
pub fn i_07da96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7DA9E; return 10; }
    8
}
/// $07DA9A: neg.w $28(a6)
pub fn i_07da9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DA9E;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DA9E: jsr $5e9b6.l
pub fn i_07da9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DAA4);
    cpu.pc = 0x5E9B6;
    16
}
/// $07DAA4: andi.w #$3, d0
pub fn i_07daa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAA8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DAA8: add.w d0, d0
pub fn i_07daa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAAA;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07DAAA: add.w d0, d0
pub fn i_07daaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAAC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07DAAC: lea.l $2e035a.l, a0
pub fn i_07daac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAB2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E035A;
    20
}
/// $07DAB2: move.l (a0, d0.w), $5c(a6)
pub fn i_07dab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAB8;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $07DAB8: move.w #$d000, d0
pub fn i_07dab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DABC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DABC: jsr $28134.l
pub fn i_07dabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DAC2);
    cpu.pc = 0x28134;
    16
}
/// $07DAC2: andi.w #$ffe3, $38(a6)
pub fn i_07dac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAC8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07DAC8: ori.w #$14, $38(a6)
pub fn i_07dac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DACE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07DACE: moveq #$0, d0
pub fn i_07dace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAD0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07DAD0: move.w d0, $70(a6)
pub fn i_07dad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAD4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DAD4: move.w d0, $78(a6)
pub fn i_07dad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAD8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DAD8: bset.b #$3, $13(a6)
pub fn i_07dad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DADE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07DADE: lea.l $2e0ab0.l, a0
pub fn i_07dade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0AB0;
    20
}
/// $07DAE4: jsr $28cd4.l
pub fn i_07dae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DAEA);
    cpu.pc = 0x28CD4;
    16
}
/// $07DAEA: lea.l $7d928(pc), a1
pub fn i_07daea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D928;
    16
}
/// $07DAEE: move.l a1, (a6)
pub fn i_07daee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DAF0: bra.w $7d928
pub fn i_07daf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x7D928;
    10
}
// ===== Crab_Shard_Small_07daf4  ($07DAF4, 174 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DAF4: move.w #$0, d0
pub fn i_07daf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DAF8: jsr $5dca4.l
pub fn i_07daf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DAFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DAFE);
    cpu.pc = 0x5DCA4;
    16
}
/// $07DAFE: move.w d0, $28(a6)
pub fn i_07dafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB02;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DB02: move.w #$118, $2a(a6)
pub fn i_07db02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB08;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x118;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DB08: move.w #$fff8, $2e(a6)
pub fn i_07db08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB0E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DB0E: move.w #$0, $2c(a6)
pub fn i_07db0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB14;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DB14: jsr $5e9b6.l
pub fn i_07db14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DB1A);
    cpu.pc = 0x5E9B6;
    16
}
/// $07DB1A: andi.w #$3ff, d0
pub fn i_07db1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB1E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DB1E: move.w d0, $28(a6)
pub fn i_07db1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB22;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DB22: jsr $5e9b6.l
pub fn i_07db22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DB28);
    cpu.pc = 0x5E9B6;
    16
}
/// $07DB28: andi.w #$1ff, d0
pub fn i_07db28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB2C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1FF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DB2C: add.w d0, $2a(a6)
pub fn i_07db2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB30;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DB30: jsr $5e9b6.l
pub fn i_07db30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DB36);
    cpu.pc = 0x5E9B6;
    16
}
/// $07DB36: andi.w #$1, d0
pub fn i_07db36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB3A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DB3A: eor.b d0, $3a(a6)
pub fn i_07db3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB3E;
    cpu.ir = 0xB12E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07DB3E: btst.b #$0, $3a(a6)
pub fn i_07db3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB44;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07DB44: beq.w $7db4c
pub fn i_07db44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7DB4C; return 10; }
    8
}
/// $07DB48: neg.w $28(a6)
pub fn i_07db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB4C;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DB4C: jsr $5e9b6.l
pub fn i_07db4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DB52);
    cpu.pc = 0x5E9B6;
    16
}
/// $07DB52: andi.w #$3, d0
pub fn i_07db52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB56;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DB56: add.w d0, d0
pub fn i_07db56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB58;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07DB58: add.w d0, d0
pub fn i_07db58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB5A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07DB5A: lea.l $2e035a.l, a0
pub fn i_07db5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E035A;
    20
}
/// $07DB60: move.l (a0, d0.w), $5c(a6)
pub fn i_07db60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB66;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $07DB66: move.w #$d000, d0
pub fn i_07db66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB6A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DB6A: jsr $28134.l
pub fn i_07db6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DB70);
    cpu.pc = 0x28134;
    16
}
/// $07DB70: andi.w #$ffe3, $38(a6)
pub fn i_07db70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB76;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07DB76: ori.w #$14, $38(a6)
pub fn i_07db76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB7C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07DB7C: moveq #$0, d0
pub fn i_07db7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB7E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07DB7E: move.w d0, $70(a6)
pub fn i_07db7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB82;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DB82: move.w d0, $78(a6)
pub fn i_07db82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB86;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DB86: bset.b #$3, $13(a6)
pub fn i_07db86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB8C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07DB8C: lea.l $2e0ab0.l, a0
pub fn i_07db8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB92;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0AB0;
    20
}
/// $07DB92: jsr $28cd4.l
pub fn i_07db92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DB98);
    cpu.pc = 0x28CD4;
    16
}
/// $07DB98: lea.l $7d928(pc), a1
pub fn i_07db98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D928;
    16
}
/// $07DB9C: move.l a1, (a6)
pub fn i_07db9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DB9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DB9E: bra.w $7d928
pub fn i_07db9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x7D928;
    10
}
// ===== Crab_MarkDetached_07dba2  ($07DBA2, 6 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DBA2: move.b #$40, $20(a6)
pub fn i_07dba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBA8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Crab_Free_07dba8  ($07DBA8, 6 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DBA8: jmp $518.l
pub fn i_07dba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Crab_Rts_07dbae  ($07DBAE, 2 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DBAE: rts
pub fn i_07dbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_IntegrateVelFrac_07dbb0  ($07DBB0, 42 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DBB0: move.w $28(a6), d0
pub fn i_07dbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBB4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DBB4: move.w d0, d1
pub fn i_07dbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBB6;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DBB6: asr.w #$8, d0
pub fn i_07dbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBB8;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07DBB8: add.b d1, $26(a6)
pub fn i_07dbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBBC;
    cpu.ir = 0xD32E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $07DBBC: moveq #$0, d1
pub fn i_07dbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBBE;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07DBBE: addx.w d1, d0
pub fn i_07dbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBC0;
    cpu.ir = 0xD141;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[1] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07DBC0: add.w d0, $22(a6)
pub fn i_07dbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBC4;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DBC4: move.w $2a(a6), d0
pub fn i_07dbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBC8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DBC8: move.w d0, d1
pub fn i_07dbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBCA;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DBCA: asr.w #$8, d0
pub fn i_07dbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBCC;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07DBCC: add.b d1, $27(a6)
pub fn i_07dbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBD0;
    cpu.ir = 0xD32E;
    let ad1 = cpu.a[6].wrapping_add(0x27);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $07DBD0: moveq #$0, d1
pub fn i_07dbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBD2;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07DBD2: addx.w d1, d0
pub fn i_07dbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBD4;
    cpu.ir = 0xD141;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[1] & 0xFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07DBD4: add.w d0, $24(a6)
pub fn i_07dbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBD8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DBD8: rts
pub fn i_07dbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Leg_PosFromParent_07dbda  ($07DBDA, 58 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DBDA: movea.l $c(a6), a0
pub fn i_07dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBDE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07DBDE: move.w $22(a0), d0
pub fn i_07dbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBE2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DBE2: add.w $72(a6), d0
pub fn i_07dbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBE6;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07DBE6: move.w d0, $22(a6)
pub fn i_07dbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBEA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DBEA: move.w $24(a0), d0
pub fn i_07dbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBEE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DBEE: cmpi.w #$2, $70(a6)
pub fn i_07dbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBF4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $07DBF4: bne.w $7dc00
pub fn i_07dbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7DC00; return 10; }
    8
}
/// $07DBF8: movea.l $c(a0), a1
pub fn i_07dbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DBFC;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07DBFC: add.w $74(a1), d0
pub fn i_07dbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC00;
    cpu.ir = 0xD069;
    let ad2 = cpu.a[1].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07DC00: add.w $74(a6), d0
pub fn i_07dc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC04;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07DC04: move.w d0, $24(a6)
pub fn i_07dc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC08;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC08: move.w $38(a0), d0
pub fn i_07dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC0C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DC0C: andi.w #$ffe3, d0
pub fn i_07dc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC10;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DC10: ori.w #$8, d0
pub fn i_07dc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC14;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x8) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== SetTaskW_07dc14  ($07DC14, 6 B, kernel/task_setters_w.c)
/// $07DC14: move.w d0, $38(a6)
pub fn i_07dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC18;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC18: rts
pub fn i_07dc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Claw_PosFromParent_07dc1a  ($07DC1A, 30 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DC1A: movea.l $c(a6), a0
pub fn i_07dc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC1E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07DC1E: move.w $22(a0), $22(a6)
pub fn i_07dc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC24;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07DC24: move.w $24(a0), d0
pub fn i_07dc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC28;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DC28: add.w $74(a0), d0
pub fn i_07dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC2C;
    cpu.ir = 0xD068;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07DC2C: move.w d0, $24(a6)
pub fn i_07dc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC30;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC30: move.w $38(a0), d0
pub fn i_07dc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC34;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DC34: add.w $82(a6), d0
pub fn i_07dc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC38;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
// ===== SetTaskW_07dc38  ($07DC38, 6 B, kernel/task_setters_w.c)
/// $07DC38: move.w d0, $38(a6)
pub fn i_07dc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC3C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC3C: rts
pub fn i_07dc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC3E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Tmpl140_07dc3e  ($07DC3E, 112 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DC3E: move.w #$4b, d1
pub fn i_07dc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC42;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DC42: jsr $236e.l
pub fn i_07dc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DC48);
    cpu.pc = 0x236E;
    16
}
/// $07DC48: move.w #$a, $1c(a6)
pub fn i_07dc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC4E: jsr $138fe.l
pub fn i_07dc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DC54);
    cpu.pc = 0x138FE;
    16
}
/// $07DC54: moveq #$0, d0
pub fn i_07dc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC56;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07DC56: move.b d0, $20(a6)
pub fn i_07dc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC5A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DC5A: move.b d0, $21(a6)
pub fn i_07dc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC5E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DC5E: lea.l $2bda80.l, a0
pub fn i_07dc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDA80;
    20
}
/// $07DC64: jsr $799de.l
pub fn i_07dc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DC6A);
    cpu.pc = 0x799DE;
    16
}
/// $07DC6A: move.w d0, $66(a6)
pub fn i_07dc6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC6E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC6E: lea.l $2bdb02.l, a0
pub fn i_07dc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BDB02;
    20
}
/// $07DC74: jsr $799de.l
pub fn i_07dc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DC7A);
    cpu.pc = 0x799DE;
    16
}
/// $07DC7A: move.w d0, $7e(a6)
pub fn i_07dc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC7E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC7E: jsr $267e2.l
pub fn i_07dc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DC84);
    cpu.pc = 0x267E2;
    16
}
/// $07DC84: move.b $98(a6), d0
pub fn i_07dc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC88;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07DC88: andi.w #$ff, d0
pub fn i_07dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC8C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07DC8C: lsl.w #$4, d0
pub fn i_07dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC8E;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07DC8E: move.w d0, $7c(a6)
pub fn i_07dc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC92;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC92: neg.w d0
pub fn i_07dc92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC94;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07DC94: move.w d0, $7a(a6)
pub fn i_07dc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC98;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DC98: move.b #$0, $3a(a6)
pub fn i_07dc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DC9E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DC9E: cmpi.w #$a0, $22(a6)
pub fn i_07dc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCA4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $07DCA4: blt.w $7dcae
pub fn i_07dca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCA8;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7DCAE; return 10; }
    8
}
/// $07DCA8: move.b #$1, $3a(a6)
pub fn i_07dca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCAE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetTaskHandler_07dcae  ($07DCAE, 8 B, kernel/task_handlers.c)
/// $07DCAE: lea.l $7dcb6(pc), a1
pub fn i_07dcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DCB6;
    16
}
/// $07DCB2: move.l a1, (a6)
pub fn i_07dcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DCB4: rts
pub fn i_07dcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Patrol_Walk_07dcb6  ($07DCB6, 290 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DCB6: jsr $5e7c0.l
pub fn i_07dcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DCBC);
    cpu.pc = 0x5E7C0;
    16
}
/// $07DCBC: move.w #$4001, d0
pub fn i_07dcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCC0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4001;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DCC0: jsr $28134.l
pub fn i_07dcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DCC6);
    cpu.pc = 0x28134;
    16
}
/// $07DCC6: andi.w #$ffe3, $38(a6)
pub fn i_07dcc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCCC;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07DCCC: ori.w #$c, $38(a6)
pub fn i_07dccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCD2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07DCD2: lea.l $7d4aa(pc), a1
pub fn i_07dcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCD6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D4AA;
    16
}
/// $07DCD6: jsr $4ae.l
pub fn i_07dcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DCDC);
    cpu.pc = 0x4AE;
    16
}
/// $07DCDC: jsr $5dd02.l
pub fn i_07dcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DCE2);
    cpu.pc = 0x5DD02;
    16
}
/// $07DCE2: lea.l $7d520(pc), a1
pub fn i_07dce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D520;
    16
}
/// $07DCE6: jsr $4ae.l
pub fn i_07dce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DCEC);
    cpu.pc = 0x4AE;
    16
}
/// $07DCEC: jsr $5dd02.l
pub fn i_07dcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DCF2);
    cpu.pc = 0x5DD02;
    16
}
/// $07DCF2: move.w #$20, $28(a6)
pub fn i_07dcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DCF8: btst.b #$0, $3a(a6)
pub fn i_07dcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DCFE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07DCFE: beq.w $7dd06
pub fn i_07dcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD02;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7DD06; return 10; }
    8
}
/// $07DD02: neg.w $28(a6)
pub fn i_07dd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD06;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DD06: move.w $28(a6), $2c(a6)
pub fn i_07dd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD0C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07DD0C: moveq #$0, d0
pub fn i_07dd0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD0E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07DD0E: move.w d0, $28(a6)
pub fn i_07dd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD12;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DD12: move.b d0, $72(a6)
pub fn i_07dd12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD16;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DD16: move.w d0, $74(a6)
pub fn i_07dd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD1A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DD1A: move.w d0, $76(a6)
pub fn i_07dd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD1E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DD1E: move.b $99(a6), $84(a6)
pub fn i_07dd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD24;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x84);
    bus.write8(ad4, v3 as u8);
    20
}
/// $07DD24: lea.l $2e057e.l, a0
pub fn i_07dd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E057E;
    20
}
/// $07DD2A: jsr $28cd4.l
pub fn i_07dd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DD30);
    cpu.pc = 0x28CD4;
    16
}
/// $07DD30: lea.l $7dd36(pc), a1
pub fn i_07dd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DD36;
    16
}
/// $07DD34: move.l a1, (a6)
pub fn i_07dd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DD36: jsr $27afc.l
pub fn i_07dd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DD3C);
    cpu.pc = 0x27AFC;
    16
}
/// $07DD3C: bcc.w $7dd46
pub fn i_07dd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DD46; return 10; }
    8
}
/// $07DD40: lea.l $7dde0(pc), a1
pub fn i_07dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DDE0;
    16
}
/// $07DD44: move.l a1, (a6)
pub fn i_07dd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DD46: move.w $28(a6), d0
pub fn i_07dd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD4A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DD4A: cmp.w $7c(a6), d0
pub fn i_07dd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD4E;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07DD4E: bgt.w $7dd5a
pub fn i_07dd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD52;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7DD5A; return 10; }
    8
}
/// $07DD52: cmp.w $7a(a6), d0
pub fn i_07dd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD56;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07DD56: bge.w $7dd72
pub fn i_07dd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD5A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7DD72; return 10; }
    8
}
/// $07DD5A: clr.w $2c(a6)
pub fn i_07dd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD5E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DD5E: move.w $7c(a6), $28(a6)
pub fn i_07dd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD64;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07DD64: btst.b #$0, $3a(a6)
pub fn i_07dd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD6A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07DD6A: beq.w $7dd72
pub fn i_07dd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD6E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7DD72; return 10; }
    8
}
/// $07DD6E: neg.w $28(a6)
pub fn i_07dd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD72;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DD72: jsr $28d70.l
pub fn i_07dd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DD78);
    cpu.pc = 0x28D70;
    16
}
/// $07DD78: jsr $2870a.l
pub fn i_07dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DD7E);
    cpu.pc = 0x2870A;
    16
}
/// $07DD7E: bcc.w $7dd94
pub fn i_07dd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD82;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DD94; return 10; }
    8
}
/// $07DD82: lea.l $5e766.l, a0
pub fn i_07dd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD88;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07DD88: jsr $5e770.l
pub fn i_07dd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DD8E);
    cpu.pc = 0x5E770;
    16
}
/// $07DD8E: bclr.b #$3, $13(a6)
pub fn i_07dd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD94;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07DD94: jsr $7df6c(pc)
pub fn i_07dd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD98;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7DD98);
    cpu.pc = 0x7DF6C;
    16
}
/// $07DD98: bcc.w $7dda2
pub fn i_07dd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DD9C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DDA2; return 10; }
    8
}
/// $07DD9C: lea.l $7dde0(pc), a1
pub fn i_07dd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDA0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DDE0;
    16
}
/// $07DDA0: move.l a1, (a6)
pub fn i_07dda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDA2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DDA2: jsr $27eba.l
pub fn i_07dda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DDA8);
    cpu.pc = 0x27EBA;
    16
}
/// $07DDA8: bcc.w $7ddb2
pub fn i_07dda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDAC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DDB2; return 10; }
    8
}
/// $07DDAC: lea.l $7d3e2(pc), a1
pub fn i_07ddac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3E2;
    16
}
/// $07DDB0: move.l a1, (a6)
pub fn i_07ddb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DDB2: jsr $28758.l
pub fn i_07ddb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DDB8);
    cpu.pc = 0x28758;
    16
}
/// $07DDB8: bcc.w $7ddc2
pub fn i_07ddb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DDC2; return 10; }
    8
}
/// $07DDBC: lea.l $7d332(pc), a1
pub fn i_07ddbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D332;
    16
}
/// $07DDC0: move.l a1, (a6)
pub fn i_07ddc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DDC2: movea.l #$ffffffff, a0
pub fn i_07ddc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDC8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07DDC8: lea.l $2e0b5c.l, a0
pub fn i_07ddc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDCE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0B5C;
    20
}
/// $07DDCE: jsr $5dd5c.l
pub fn i_07ddce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DDD4);
    cpu.pc = 0x5DD5C;
    16
}
/// $07DDD4: bcc.w $7ddde
pub fn i_07ddd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDD8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DDDE; return 10; }
    8
}
// ===== SetTaskHandler_07ddd8  ($07DDD8, 8 B, kernel/task_handlers.c)
/// $07DDD8: lea.l $7dba2(pc), a1
pub fn i_07ddd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07DDDC: move.l a1, (a6)
pub fn i_07dddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DDDE: rts
pub fn i_07ddde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Patrol_Pause_07dde0  ($07DDE0, 166 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DDE0: jsr $267e2.l
pub fn i_07dde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DDE6);
    cpu.pc = 0x267E2;
    16
}
/// $07DDE6: clr.w $74(a6)
pub fn i_07dde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDEA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DDEA: move.w #$109e, d0
pub fn i_07ddea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDEE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07DDEE: jsr $2222.l
pub fn i_07ddee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DDF4);
    cpu.pc = 0x2222;
    16
}
/// $07DDF4: lea.l $2e04d6.l, a0
pub fn i_07ddf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DDFA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E04D6;
    20
}
/// $07DDFA: jsr $28cd4.l
pub fn i_07ddfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE00);
    cpu.pc = 0x28CD4;
    16
}
/// $07DE00: lea.l $7de06(pc), a1
pub fn i_07de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DE06;
    16
}
/// $07DE04: move.l a1, (a6)
pub fn i_07de04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DE06: jsr $2783a.l
pub fn i_07de06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE0C);
    cpu.pc = 0x2783A;
    16
}
/// $07DE0C: jsr $28d70.l
pub fn i_07de0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE12);
    cpu.pc = 0x28D70;
    16
}
/// $07DE12: jsr $2870a.l
pub fn i_07de12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE18);
    cpu.pc = 0x2870A;
    16
}
/// $07DE18: bcc.w $7de2e
pub fn i_07de18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DE2E; return 10; }
    8
}
/// $07DE1C: lea.l $5e766.l, a0
pub fn i_07de1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07DE22: jsr $5e770.l
pub fn i_07de22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE28);
    cpu.pc = 0x5E770;
    16
}
/// $07DE28: bclr.b #$3, $13(a6)
pub fn i_07de28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE2E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07DE2E: cmpi.b #$ff, $21(a6)
pub fn i_07de2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE34;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07DE34: beq.w $7de50
pub fn i_07de34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7DE50; return 10; }
    8
}
/// $07DE38: addq.w #$1, $76(a6)
pub fn i_07de38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE3C;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DE3C: cmpi.w #$3c, $76(a6)
pub fn i_07de3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE42;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Word, false);
    16
}
/// $07DE42: blt.w $7de4c
pub fn i_07de42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE46;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7DE4C; return 10; }
    8
}
/// $07DE46: lea.l $7de8e(pc), a1
pub fn i_07de46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DE8E;
    16
}
/// $07DE4A: move.l a1, (a6)
pub fn i_07de4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DE4C: jsr $7df6c(pc)
pub fn i_07de4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7DE50);
    cpu.pc = 0x7DF6C;
    16
}
/// $07DE50: jsr $27eba.l
pub fn i_07de50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE56);
    cpu.pc = 0x27EBA;
    16
}
/// $07DE56: bcc.w $7de60
pub fn i_07de56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE5A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DE60; return 10; }
    8
}
/// $07DE5A: lea.l $7d3e2(pc), a1
pub fn i_07de5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3E2;
    16
}
/// $07DE5E: move.l a1, (a6)
pub fn i_07de5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DE60: jsr $28758.l
pub fn i_07de60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE66);
    cpu.pc = 0x28758;
    16
}
/// $07DE66: bcc.w $7de70
pub fn i_07de66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DE70; return 10; }
    8
}
/// $07DE6A: lea.l $7d332(pc), a1
pub fn i_07de6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D332;
    16
}
/// $07DE6E: move.l a1, (a6)
pub fn i_07de6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DE70: movea.l #$ffffffff, a0
pub fn i_07de70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE76;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07DE76: lea.l $2e0b5c.l, a0
pub fn i_07de76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0B5C;
    20
}
/// $07DE7C: jsr $5dd5c.l
pub fn i_07de7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE82);
    cpu.pc = 0x5DD5C;
    16
}
/// $07DE82: bcc.w $7de8c
pub fn i_07de82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DE8C; return 10; }
    8
}
// ===== SetTaskHandler_07de86  ($07DE86, 8 B, kernel/task_handlers.c)
/// $07DE86: lea.l $7dba2(pc), a1
pub fn i_07de86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE8A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07DE8A: move.l a1, (a6)
pub fn i_07de8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE8C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DE8C: rts
pub fn i_07de8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Patrol_WalkBack_07de8e  ($07DE8E, 214 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DE8E: jsr $267e2.l
pub fn i_07de8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DE94);
    cpu.pc = 0x267E2;
    16
}
/// $07DE94: move.w #$ffe0, $28(a6)
pub fn i_07de94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DE9A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DE9A: btst.b #$0, $3a(a6)
pub fn i_07de9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEA0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07DEA0: beq.w $7dea8
pub fn i_07dea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7DEA8; return 10; }
    8
}
/// $07DEA4: neg.w $28(a6)
pub fn i_07dea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEA8;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DEA8: move.w $28(a6), $2c(a6)
pub fn i_07dea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEAE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07DEAE: moveq #$0, d0
pub fn i_07deae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEB0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07DEB0: move.w d0, $28(a6)
pub fn i_07deb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEB4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DEB4: move.w d0, $74(a6)
pub fn i_07deb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEB8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DEB8: lea.l $2e050a.l, a0
pub fn i_07deb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEBE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E050A;
    20
}
/// $07DEBE: jsr $28cd4.l
pub fn i_07debe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DEC4);
    cpu.pc = 0x28CD4;
    16
}
/// $07DEC4: lea.l $7deca(pc), a1
pub fn i_07dec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DECA;
    16
}
/// $07DEC8: move.l a1, (a6)
pub fn i_07dec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DECA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DECA: jsr $27afc.l
pub fn i_07deca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DED0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DED0);
    cpu.pc = 0x27AFC;
    16
}
/// $07DED0: bcc.w $7dee0
pub fn i_07ded0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DED4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DEE0; return 10; }
    8
}
/// $07DED4: move.b #$ff, $21(a6)
pub fn i_07ded4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEDA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DEDA: lea.l $7dde0(pc), a1
pub fn i_07deda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DDE0;
    16
}
/// $07DEDE: move.l a1, (a6)
pub fn i_07dede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DEE0: move.w $28(a6), d0
pub fn i_07dee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEE4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DEE4: cmp.w $7c(a6), d0
pub fn i_07dee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEE8;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07DEE8: bgt.w $7def4
pub fn i_07dee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEEC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7DEF4; return 10; }
    8
}
/// $07DEEC: cmp.w $7a(a6), d0
pub fn i_07deec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEF0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07DEF0: bge.w $7df0c
pub fn i_07def0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEF4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7DF0C; return 10; }
    8
}
/// $07DEF4: clr.w $2c(a6)
pub fn i_07def4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEF8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DEF8: move.w $7a(a6), $28(a6)
pub fn i_07def8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DEFE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07DEFE: btst.b #$0, $3a(a6)
pub fn i_07defe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF04;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07DF04: beq.w $7df0c
pub fn i_07df04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7DF0C; return 10; }
    8
}
/// $07DF08: neg.w $28(a6)
pub fn i_07df08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF0C;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DF0C: jsr $28d70.l
pub fn i_07df0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DF12);
    cpu.pc = 0x28D70;
    16
}
/// $07DF12: jsr $2870a.l
pub fn i_07df12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DF18);
    cpu.pc = 0x2870A;
    16
}
/// $07DF18: bcc.w $7df2e
pub fn i_07df18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DF2E; return 10; }
    8
}
/// $07DF1C: lea.l $5e766.l, a0
pub fn i_07df1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $07DF22: jsr $5e770.l
pub fn i_07df22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DF28);
    cpu.pc = 0x5E770;
    16
}
/// $07DF28: bclr.b #$3, $13(a6)
pub fn i_07df28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF2E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07DF2E: jsr $27eba.l
pub fn i_07df2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DF34);
    cpu.pc = 0x27EBA;
    16
}
/// $07DF34: bcc.w $7df3e
pub fn i_07df34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DF3E; return 10; }
    8
}
/// $07DF38: lea.l $7d3e2(pc), a1
pub fn i_07df38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D3E2;
    16
}
/// $07DF3C: move.l a1, (a6)
pub fn i_07df3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DF3E: jsr $28758.l
pub fn i_07df3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DF44);
    cpu.pc = 0x28758;
    16
}
/// $07DF44: bcc.w $7df4e
pub fn i_07df44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DF4E; return 10; }
    8
}
/// $07DF48: lea.l $7d332(pc), a1
pub fn i_07df48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7D332;
    16
}
/// $07DF4C: move.l a1, (a6)
pub fn i_07df4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DF4E: movea.l #$ffffffff, a0
pub fn i_07df4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF54;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $07DF54: lea.l $2e0b5c.l, a0
pub fn i_07df54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF5A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E0B5C;
    20
}
/// $07DF5A: jsr $5dd5c.l
pub fn i_07df5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DF60);
    cpu.pc = 0x5DD5C;
    16
}
/// $07DF60: bcc.w $7df6a
pub fn i_07df60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF64;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7DF6A; return 10; }
    8
}
// ===== SetTaskHandler_07df64  ($07DF64, 8 B, kernel/task_handlers.c)
/// $07DF64: lea.l $7dba2(pc), a1
pub fn i_07df64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7DBA2;
    16
}
/// $07DF68: move.l a1, (a6)
pub fn i_07df68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DF6A: rts
pub fn i_07df6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF6C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Crab_Patrol_PlayerNear_07df6c  ($07DF6C, 48 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DF6C: cmpi.b #$1, $20(a6)
pub fn i_07df6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF72;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07DF72: beq.w $7dfa2
pub fn i_07df72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7DFA2; return 10; }
    8
}
/// $07DF76: clr.w $76(a6)
pub fn i_07df76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF7A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $07DF7A: jsr $5e0d4.l
pub fn i_07df7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DF80);
    cpu.pc = 0x5E0D4;
    16
}
/// $07DF80: move.w $22(a6), d0
pub fn i_07df80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF84;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07DF84: sub.w $22(a0), d0
pub fn i_07df84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF88;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07DF88: bpl.w $7df8e
pub fn i_07df88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF8C;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7DF8E; return 10; }
    8
}
/// $07DF8C: neg.w d0
pub fn i_07df8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF8E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07DF8E: cmp.w $7e(a6), d0
pub fn i_07df8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF92;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07DF92: bhi.w $7dfa2
pub fn i_07df92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF96;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x7DFA2; return 10; }
    8
}
/// $07DF96: move.b #$1, $20(a6)
pub fn i_07df96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DF9C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetC_07df9c  ($07DF9C, 6 B, thunks/ccr_helpers.c)
/// $07DF9C: ori.b #$1, ccr
pub fn i_07df9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFA0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $07DFA0: rts
pub fn i_07dfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_07dfa2  ($07DFA2, 6 B, thunks/ccr_helpers.c)
/// $07DFA2: andi.b #$fe, ccr
pub fn i_07dfa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFA6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $07DFA6: rts
pub fn i_07dfa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_RetGate_07dfa8  ($07DFA8, 16 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DFA8: movea.l $8(a6), a1
pub fn i_07dfa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFAC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $07DFAC: move.b $10(a6), d0
pub fn i_07dfac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFB0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07DFB0: cmp.b $10(a1), d0
pub fn i_07dfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFB4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $07DFB4: bcs.w $7dfbe
pub fn i_07dfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFB8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7DFBE; return 10; }
    8
}
// ===== ClearXN_07dfb8  ($07DFB8, 6 B, thunks/ccr_helpers.c)
/// $07DFB8: andi.b #$ee, ccr
pub fn i_07dfb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFBC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $07DFBC: rts
pub fn i_07dfbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_07dfbe  ($07DFBE, 6 B, thunks/ccr_helpers.c)
/// $07DFBE: ori.b #$11, ccr
pub fn i_07dfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFC2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $07DFC2: rts
pub fn i_07dfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Tmpl129_07dfc4  ($07DFC4, 52 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07DFC4: moveq #$0, d0
pub fn i_07dfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFC6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07DFC6: move.b d0, $20(a6)
pub fn i_07dfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFCA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DFCA: move.b d0, $21(a6)
pub fn i_07dfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFCE;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DFCE: move.b d0, $82(a6)
pub fn i_07dfce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFD2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07DFD2: jsr $267e2.l
pub fn i_07dfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DFD8);
    cpu.pc = 0x267E2;
    16
}
/// $07DFD8: move.w #$5000, $38(a6)
pub fn i_07dfd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DFDE: lea.l $2bcb42.l, a0
pub fn i_07dfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCB42;
    20
}
/// $07DFE4: jsr $799de.l
pub fn i_07dfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DFEA);
    cpu.pc = 0x799DE;
    16
}
/// $07DFEA: move.w d0, $66(a6)
pub fn i_07dfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFEE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07DFEE: lea.l $7e2f2(pc), a1
pub fn i_07dfee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E2F2;
    16
}
/// $07DFF2: jsr $4ae.l
pub fn i_07dff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7DFF8);
    cpu.pc = 0x4AE;
    16
}
// ===== SetTaskHandler_07dff8  ($07DFF8, 8 B, kernel/task_handlers.c)
/// $07DFF8: lea.l $7e078(pc), a1
pub fn i_07dff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E078;
    16
}
/// $07DFFC: move.l a1, (a6)
pub fn i_07dffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7DFFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07DFFE: rts
pub fn i_07dffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E000;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Tmpl130_07e000  ($07E000, 52 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E000: moveq #$0, d0
pub fn i_07e000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E002;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07E002: move.b d0, $20(a6)
pub fn i_07e002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E006;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E006: move.b d0, $21(a6)
pub fn i_07e006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E00A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E00A: move.b d0, $82(a6)
pub fn i_07e00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E00E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E00E: jsr $267e2.l
pub fn i_07e00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E014);
    cpu.pc = 0x267E2;
    16
}
/// $07E014: move.w #$5000, $38(a6)
pub fn i_07e014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E01A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E01A: lea.l $2bcb42.l, a0
pub fn i_07e01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E020;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCB42;
    20
}
/// $07E020: jsr $799de.l
pub fn i_07e020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E026);
    cpu.pc = 0x799DE;
    16
}
/// $07E026: move.w d0, $66(a6)
pub fn i_07e026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E02A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E02A: lea.l $7e2f2(pc), a1
pub fn i_07e02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E02E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E2F2;
    16
}
/// $07E02E: jsr $4ae.l
pub fn i_07e02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E034;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E034);
    cpu.pc = 0x4AE;
    16
}
// ===== SetTaskHandler_07e034  ($07E034, 8 B, kernel/task_handlers.c)
/// $07E034: lea.l $7e08c(pc), a1
pub fn i_07e034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E038;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E08C;
    16
}
/// $07E038: move.l a1, (a6)
pub fn i_07e038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E03A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E03A: rts
pub fn i_07e03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E03C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Tmpl131_07e03c  ($07E03C, 52 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E03C: clr.b $20(a6)
pub fn i_07e03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E040;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E040: clr.b $21(a6)
pub fn i_07e040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E044;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $07E044: move.b #$ff, $82(a6)
pub fn i_07e044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E04A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07E04A: jsr $267e2.l
pub fn i_07e04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E050;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E050);
    cpu.pc = 0x267E2;
    16
}
/// $07E050: move.w #$5000, $38(a6)
pub fn i_07e050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E056;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x5000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E056: lea.l $2bcc46.l, a0
pub fn i_07e056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E05C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BCC46;
    20
}
/// $07E05C: jsr $799de.l
pub fn i_07e05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E062;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E062);
    cpu.pc = 0x799DE;
    16
}
/// $07E062: move.w d0, $66(a6)
pub fn i_07e062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E066;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E066: lea.l $7e2f2(pc), a1
pub fn i_07e066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E06A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E2F2;
    16
}
/// $07E06A: jsr $4ae.l
pub fn i_07e06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E070;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E070);
    cpu.pc = 0x4AE;
    16
}
// ===== SetTaskHandler_07e070  ($07E070, 8 B, kernel/task_handlers.c)
/// $07E070: lea.l $7e078(pc), a1
pub fn i_07e070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E074;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7E078;
    16
}
/// $07E074: move.l a1, (a6)
pub fn i_07e074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E076;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07E076: rts
pub fn i_07e076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E078;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_InitWithHatchB_07e078  ($07E078, 20 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E078: lea.l $7eb76(pc), a1
pub fn i_07e078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E07C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EB76;
    16
}
/// $07E07C: jsr $4ae.l
pub fn i_07e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E082;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E082);
    cpu.pc = 0x4AE;
    16
}
/// $07E082: jsr $5dd02.l
pub fn i_07e082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E088;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E088);
    cpu.pc = 0x5DD02;
    16
}
/// $07E088: bra.w $7e09c
pub fn i_07e088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E08C;
    cpu.ir = 0x6000;
    cpu.pc = 0x7E09C;
    10
}
// ===== Carrier_Init_07e08c  ($07E08C, 464 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07E08C: lea.l $7eb7e(pc), a1
pub fn i_07e08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E090;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7EB7E;
    16
}
/// $07E090: jsr $4ae.l
pub fn i_07e090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E096);
    cpu.pc = 0x4AE;
    16
}
/// $07E096: jsr $5dd02.l
pub fn i_07e096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E09C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7E09C);
    cpu.pc = 0x5DD02;
    16
}
/// $07E09C: moveq #$0, d0
pub fn i_07e09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E09E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07E09E: move.w #$24, $70(a6)
pub fn i_07e09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0A4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E0A4: move.w d0, $72(a6)
pub fn i_07e0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0A8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E0A8: move.w d0, $74(a6)
pub fn i_07e0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0AC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E0AC: move.w d0, $76(a6)
pub fn i_07e0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0B0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E0B0: move.w d0, $78(a6)
pub fn i_07e0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0B4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E0B4: move.w #$3c, $7a(a6)
pub fn i_07e0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0BA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E0BA: move.w d0, $7c(a6)
pub fn i_07e0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0BE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07E0BE: move.l d0, $7e(a6)
pub fn i_07e0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7E0C2;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x07C538, i_07c538),
    (0x07C53C, i_07c53c),
    (0x07C540, i_07c540),
    (0x07C546, i_07c546),
    (0x07C54A, i_07c54a),
    (0x07C54E, i_07c54e),
    (0x07C552, i_07c552),
    (0x07C558, i_07c558),
    (0x07C55E, i_07c55e),
    (0x07C562, i_07c562),
    (0x07C564, i_07c564),
    (0x07C566, i_07c566),
    (0x07C56A, i_07c56a),
    (0x07C56E, i_07c56e),
    (0x07C572, i_07c572),
    (0x07C576, i_07c576),
    (0x07C57C, i_07c57c),
    (0x07C582, i_07c582),
    (0x07C584, i_07c584),
    (0x07C586, i_07c586),
    (0x07C58A, i_07c58a),
    (0x07C58E, i_07c58e),
    (0x07C592, i_07c592),
    (0x07C596, i_07c596),
    (0x07C598, i_07c598),
    (0x07C59C, i_07c59c),
    (0x07C59E, i_07c59e),
    (0x07C5A2, i_07c5a2),
    (0x07C5A6, i_07c5a6),
    (0x07C5AA, i_07c5aa),
    (0x07C5AE, i_07c5ae),
    (0x07C5B2, i_07c5b2),
    (0x07C5B8, i_07c5b8),
    (0x07C5BC, i_07c5bc),
    (0x07C5C0, i_07c5c0),
    (0x07C5C4, i_07c5c4),
    (0x07C5C8, i_07c5c8),
    (0x07C5CC, i_07c5cc),
    (0x07C5D2, i_07c5d2),
    (0x07C5D6, i_07c5d6),
    (0x07C5DA, i_07c5da),
    (0x07C5DE, i_07c5de),
    (0x07C5E2, i_07c5e2),
    (0x07C5E6, i_07c5e6),
    (0x07C5EA, i_07c5ea),
    (0x07C5EC, i_07c5ec),
    (0x07C5F0, i_07c5f0),
    (0x07C5F2, i_07c5f2),
    (0x07C5F6, i_07c5f6),
    (0x07C5FA, i_07c5fa),
    (0x07C600, i_07c600),
    (0x07C604, i_07c604),
    (0x07C608, i_07c608),
    (0x07C60C, i_07c60c),
    (0x07C612, i_07c612),
    (0x07C616, i_07c616),
    (0x07C618, i_07c618),
    (0x07C61C, i_07c61c),
    (0x07C61E, i_07c61e),
    (0x07C622, i_07c622),
    (0x07C626, i_07c626),
    (0x07C628, i_07c628),
    (0x07C62E, i_07c62e),
    (0x07C634, i_07c634),
    (0x07C638, i_07c638),
    (0x07C63C, i_07c63c),
    (0x07C63E, i_07c63e),
    (0x07C642, i_07c642),
    (0x07C644, i_07c644),
    (0x07C64A, i_07c64a),
    (0x07C64E, i_07c64e),
    (0x07C654, i_07c654),
    (0x07C65A, i_07c65a),
    (0x07C65C, i_07c65c),
    (0x07C664, i_07c664),
    (0x07C668, i_07c668),
    (0x07C66E, i_07c66e),
    (0x07C674, i_07c674),
    (0x07C67A, i_07c67a),
    (0x07C67E, i_07c67e),
    (0x07C682, i_07c682),
    (0x07C686, i_07c686),
    (0x07C68C, i_07c68c),
    (0x07C690, i_07c690),
    (0x07C694, i_07c694),
    (0x07C69A, i_07c69a),
    (0x07C69E, i_07c69e),
    (0x07C6A4, i_07c6a4),
    (0x07C6AA, i_07c6aa),
    (0x07C6B0, i_07c6b0),
    (0x07C6B6, i_07c6b6),
    (0x07C6BA, i_07c6ba),
    (0x07C6C0, i_07c6c0),
    (0x07C6C6, i_07c6c6),
    (0x07C6CA, i_07c6ca),
    (0x07C6CC, i_07c6cc),
    (0x07C6D2, i_07c6d2),
    (0x07C6D6, i_07c6d6),
    (0x07C6DA, i_07c6da),
    (0x07C6DC, i_07c6dc),
    (0x07C6E2, i_07c6e2),
    (0x07C6E8, i_07c6e8),
    (0x07C6EE, i_07c6ee),
    (0x07C6F2, i_07c6f2),
    (0x07C6F6, i_07c6f6),
    (0x07C6F8, i_07c6f8),
    (0x07C6FC, i_07c6fc),
    (0x07C702, i_07c702),
    (0x07C706, i_07c706),
    (0x07C70A, i_07c70a),
    (0x07C70E, i_07c70e),
    (0x07C714, i_07c714),
    (0x07C71A, i_07c71a),
    (0x07C71E, i_07c71e),
    (0x07C722, i_07c722),
    (0x07C728, i_07c728),
    (0x07C72C, i_07c72c),
    (0x07C730, i_07c730),
    (0x07C734, i_07c734),
    (0x07C73A, i_07c73a),
    (0x07C73C, i_07c73c),
    (0x07C742, i_07c742),
    (0x07C748, i_07c748),
    (0x07C74C, i_07c74c),
    (0x07C750, i_07c750),
    (0x07C752, i_07c752),
    (0x07C754, i_07c754),
    (0x07C758, i_07c758),
    (0x07C75E, i_07c75e),
    (0x07C764, i_07c764),
    (0x07C76A, i_07c76a),
    (0x07C770, i_07c770),
    (0x07C774, i_07c774),
    (0x07C77A, i_07c77a),
    (0x07C780, i_07c780),
    (0x07C784, i_07c784),
    (0x07C78A, i_07c78a),
    (0x07C790, i_07c790),
    (0x07C796, i_07c796),
    (0x07C79C, i_07c79c),
    (0x07C7A2, i_07c7a2),
    (0x07C7A6, i_07c7a6),
    (0x07C7AC, i_07c7ac),
    (0x07C7B2, i_07c7b2),
    (0x07C7B8, i_07c7b8),
    (0x07C7BE, i_07c7be),
    (0x07C7C4, i_07c7c4),
    (0x07C7C8, i_07c7c8),
    (0x07C7CE, i_07c7ce),
    (0x07C7D4, i_07c7d4),
    (0x07C7DA, i_07c7da),
    (0x07C7E0, i_07c7e0),
    (0x07C7E6, i_07c7e6),
    (0x07C7EA, i_07c7ea),
    (0x07C7F0, i_07c7f0),
    (0x07C7F6, i_07c7f6),
    (0x07C7FC, i_07c7fc),
    (0x07C802, i_07c802),
    (0x07C808, i_07c808),
    (0x07C80E, i_07c80e),
    (0x07C814, i_07c814),
    (0x07C818, i_07c818),
    (0x07C81C, i_07c81c),
    (0x07C822, i_07c822),
    (0x07C826, i_07c826),
    (0x07C82A, i_07c82a),
    (0x07C82E, i_07c82e),
    (0x07C834, i_07c834),
    (0x07C836, i_07c836),
    (0x07C838, i_07c838),
    (0x07C83E, i_07c83e),
    (0x07C844, i_07c844),
    (0x07C848, i_07c848),
    (0x07C84A, i_07c84a),
    (0x07C850, i_07c850),
    (0x07C856, i_07c856),
    (0x07C85A, i_07c85a),
    (0x07C85E, i_07c85e),
    (0x07C860, i_07c860),
    (0x07C866, i_07c866),
    (0x07C868, i_07c868),
    (0x07C86C, i_07c86c),
    (0x07C872, i_07c872),
    (0x07C876, i_07c876),
    (0x07C87C, i_07c87c),
    (0x07C882, i_07c882),
    (0x07C888, i_07c888),
    (0x07C88E, i_07c88e),
    (0x07C894, i_07c894),
    (0x07C898, i_07c898),
    (0x07C89A, i_07c89a),
    (0x07C8A0, i_07c8a0),
    (0x07C8A6, i_07c8a6),
    (0x07C8AA, i_07c8aa),
    (0x07C8AE, i_07c8ae),
    (0x07C8B0, i_07c8b0),
    (0x07C8B2, i_07c8b2),
    (0x07C8B6, i_07c8b6),
    (0x07C8BC, i_07c8bc),
    (0x07C8C0, i_07c8c0),
    (0x07C8C6, i_07c8c6),
    (0x07C8CC, i_07c8cc),
    (0x07C8D2, i_07c8d2),
    (0x07C8D8, i_07c8d8),
    (0x07C8DE, i_07c8de),
    (0x07C8E4, i_07c8e4),
    (0x07C8E8, i_07c8e8),
    (0x07C8EA, i_07c8ea),
    (0x07C8F0, i_07c8f0),
    (0x07C8F6, i_07c8f6),
    (0x07C8FA, i_07c8fa),
    (0x07C8FE, i_07c8fe),
    (0x07C900, i_07c900),
    (0x07C906, i_07c906),
    (0x07C90C, i_07c90c),
    (0x07C910, i_07c910),
    (0x07C914, i_07c914),
    (0x07C916, i_07c916),
    (0x07C918, i_07c918),
    (0x07C91C, i_07c91c),
    (0x07C920, i_07c920),
    (0x07C924, i_07c924),
    (0x07C928, i_07c928),
    (0x07C92C, i_07c92c),
    (0x07C932, i_07c932),
    (0x07C936, i_07c936),
    (0x07C93C, i_07c93c),
    (0x07C942, i_07c942),
    (0x07C948, i_07c948),
    (0x07C94C, i_07c94c),
    (0x07C94E, i_07c94e),
    (0x07C950, i_07c950),
    (0x07C956, i_07c956),
    (0x07C95A, i_07c95a),
    (0x07C95E, i_07c95e),
    (0x07C962, i_07c962),
    (0x07C966, i_07c966),
    (0x07C96A, i_07c96a),
    (0x07C970, i_07c970),
    (0x07C976, i_07c976),
    (0x07C97A, i_07c97a),
    (0x07C97C, i_07c97c),
    (0x07C97E, i_07c97e),
    (0x07C984, i_07c984),
    (0x07C988, i_07c988),
    (0x07C98C, i_07c98c),
    (0x07C990, i_07c990),
    (0x07C994, i_07c994),
    (0x07C996, i_07c996),
    (0x07C99C, i_07c99c),
    (0x07C9A2, i_07c9a2),
    (0x07C9A8, i_07c9a8),
    (0x07C9AA, i_07c9aa),
    (0x07C9AE, i_07c9ae),
    (0x07C9B4, i_07c9b4),
    (0x07C9B8, i_07c9b8),
    (0x07C9BE, i_07c9be),
    (0x07C9C2, i_07c9c2),
    (0x07C9C8, i_07c9c8),
    (0x07C9CE, i_07c9ce),
    (0x07C9D0, i_07c9d0),
    (0x07C9D6, i_07c9d6),
    (0x07C9DA, i_07c9da),
    (0x07C9DE, i_07c9de),
    (0x07C9E0, i_07c9e0),
    (0x07C9E6, i_07c9e6),
    (0x07C9EA, i_07c9ea),
    (0x07C9F0, i_07c9f0),
    (0x07C9F4, i_07c9f4),
    (0x07C9F8, i_07c9f8),
    (0x07C9FA, i_07c9fa),
    (0x07C9FC, i_07c9fc),
    (0x07C9FE, i_07c9fe),
    (0x07CA02, i_07ca02),
    (0x07CA06, i_07ca06),
    (0x07CA0C, i_07ca0c),
    (0x07CA10, i_07ca10),
    (0x07CA12, i_07ca12),
    (0x07CA16, i_07ca16),
    (0x07CA18, i_07ca18),
    (0x07CA1A, i_07ca1a),
    (0x07CA1E, i_07ca1e),
    (0x07CA24, i_07ca24),
    (0x07CA28, i_07ca28),
    (0x07CA2A, i_07ca2a),
    (0x07CA2E, i_07ca2e),
    (0x07CA30, i_07ca30),
    (0x07CA32, i_07ca32),
    (0x07CA36, i_07ca36),
    (0x07CA3A, i_07ca3a),
    (0x07CA3C, i_07ca3c),
    (0x07CA42, i_07ca42),
    (0x07CA48, i_07ca48),
    (0x07CA4E, i_07ca4e),
    (0x07CA54, i_07ca54),
    (0x07CA58, i_07ca58),
    (0x07CA5C, i_07ca5c),
    (0x07CA5E, i_07ca5e),
    (0x07CA60, i_07ca60),
    (0x07CA66, i_07ca66),
    (0x07CA6C, i_07ca6c),
    (0x07CA72, i_07ca72),
    (0x07CA78, i_07ca78),
    (0x07CA7E, i_07ca7e),
    (0x07CA84, i_07ca84),
    (0x07CA8A, i_07ca8a),
    (0x07CA90, i_07ca90),
    (0x07CA96, i_07ca96),
    (0x07CA9C, i_07ca9c),
    (0x07CAA2, i_07caa2),
    (0x07CAA8, i_07caa8),
    (0x07CAAE, i_07caae),
    (0x07CAB0, i_07cab0),
    (0x07CAB6, i_07cab6),
    (0x07CABC, i_07cabc),
    (0x07CAC2, i_07cac2),
    (0x07CAC8, i_07cac8),
    (0x07CACE, i_07cace),
    (0x07CAD4, i_07cad4),
    (0x07CADA, i_07cada),
    (0x07CAE0, i_07cae0),
    (0x07CAE6, i_07cae6),
    (0x07CAEC, i_07caec),
    (0x07CAF2, i_07caf2),
    (0x07CAF8, i_07caf8),
    (0x07CAFA, i_07cafa),
    (0x07CB00, i_07cb00),
    (0x07CB06, i_07cb06),
    (0x07CB0C, i_07cb0c),
    (0x07CB12, i_07cb12),
    (0x07CB18, i_07cb18),
    (0x07CB1E, i_07cb1e),
    (0x07CB24, i_07cb24),
    (0x07CB2A, i_07cb2a),
    (0x07CB30, i_07cb30),
    (0x07CB36, i_07cb36),
    (0x07CB3C, i_07cb3c),
    (0x07CB42, i_07cb42),
    (0x07CB44, i_07cb44),
    (0x07CB4A, i_07cb4a),
    (0x07CB50, i_07cb50),
    (0x07CB56, i_07cb56),
    (0x07CB5C, i_07cb5c),
    (0x07CB62, i_07cb62),
    (0x07CB68, i_07cb68),
    (0x07CB6E, i_07cb6e),
    (0x07CB74, i_07cb74),
    (0x07CB7A, i_07cb7a),
    (0x07CB80, i_07cb80),
    (0x07CB86, i_07cb86),
    (0x07CB8C, i_07cb8c),
    (0x07CB8E, i_07cb8e),
    (0x07CB94, i_07cb94),
    (0x07CB9A, i_07cb9a),
    (0x07CBA0, i_07cba0),
    (0x07CBA6, i_07cba6),
    (0x07CBAC, i_07cbac),
    (0x07CBB2, i_07cbb2),
    (0x07CBB8, i_07cbb8),
    (0x07CBBE, i_07cbbe),
    (0x07CBC4, i_07cbc4),
    (0x07CBCA, i_07cbca),
    (0x07CBD0, i_07cbd0),
    (0x07CBD6, i_07cbd6),
    (0x07CBD8, i_07cbd8),
    (0x07CBDE, i_07cbde),
    (0x07CBE4, i_07cbe4),
    (0x07CBEA, i_07cbea),
    (0x07CBF0, i_07cbf0),
    (0x07CBF6, i_07cbf6),
    (0x07CBFC, i_07cbfc),
    (0x07CC02, i_07cc02),
    (0x07CC08, i_07cc08),
    (0x07CC0E, i_07cc0e),
    (0x07CC14, i_07cc14),
    (0x07CC1A, i_07cc1a),
    (0x07CC20, i_07cc20),
    (0x07CC22, i_07cc22),
    (0x07CC28, i_07cc28),
    (0x07CC2E, i_07cc2e),
    (0x07CC34, i_07cc34),
    (0x07CC3A, i_07cc3a),
    (0x07CC40, i_07cc40),
    (0x07CC46, i_07cc46),
    (0x07CC4C, i_07cc4c),
    (0x07CC52, i_07cc52),
    (0x07CC58, i_07cc58),
    (0x07CC5E, i_07cc5e),
    (0x07CC64, i_07cc64),
    (0x07CC6A, i_07cc6a),
    (0x07CC6C, i_07cc6c),
    (0x07CC72, i_07cc72),
    (0x07CC78, i_07cc78),
    (0x07CC7E, i_07cc7e),
    (0x07CC84, i_07cc84),
    (0x07CC8A, i_07cc8a),
    (0x07CC90, i_07cc90),
    (0x07CC96, i_07cc96),
    (0x07CC9C, i_07cc9c),
    (0x07CCA2, i_07cca2),
    (0x07CCA8, i_07cca8),
    (0x07CCAE, i_07ccae),
    (0x07CCB4, i_07ccb4),
    (0x07CCB6, i_07ccb6),
    (0x07CCBC, i_07ccbc),
    (0x07CCC2, i_07ccc2),
    (0x07CCC8, i_07ccc8),
    (0x07CCCE, i_07ccce),
    (0x07CCD4, i_07ccd4),
    (0x07CCDA, i_07ccda),
    (0x07CCE0, i_07cce0),
    (0x07CCE6, i_07cce6),
    (0x07CCEC, i_07ccec),
    (0x07CCF2, i_07ccf2),
    (0x07CCF8, i_07ccf8),
    (0x07CCFE, i_07ccfe),
    (0x07CD04, i_07cd04),
    (0x07CD06, i_07cd06),
    (0x07CD0A, i_07cd0a),
    (0x07CD10, i_07cd10),
    (0x07CD16, i_07cd16),
    (0x07CD1C, i_07cd1c),
    (0x07CD22, i_07cd22),
    (0x07CD24, i_07cd24),
    (0x07CD28, i_07cd28),
    (0x07CD2E, i_07cd2e),
    (0x07CD34, i_07cd34),
    (0x07CD3A, i_07cd3a),
    (0x07CD40, i_07cd40),
    (0x07CD42, i_07cd42),
    (0x07CD46, i_07cd46),
    (0x07CD4C, i_07cd4c),
    (0x07CD52, i_07cd52),
    (0x07CD58, i_07cd58),
    (0x07CD5E, i_07cd5e),
    (0x07CD60, i_07cd60),
    (0x07CD64, i_07cd64),
    (0x07CD6A, i_07cd6a),
    (0x07CD70, i_07cd70),
    (0x07CD76, i_07cd76),
    (0x07CD7C, i_07cd7c),
    (0x07CD7E, i_07cd7e),
    (0x07CD82, i_07cd82),
    (0x07CD88, i_07cd88),
    (0x07CD8E, i_07cd8e),
    (0x07CD94, i_07cd94),
    (0x07CD9A, i_07cd9a),
    (0x07CD9C, i_07cd9c),
    (0x07CDA0, i_07cda0),
    (0x07CDA6, i_07cda6),
    (0x07CDAC, i_07cdac),
    (0x07CDB2, i_07cdb2),
    (0x07CDB8, i_07cdb8),
    (0x07CDBA, i_07cdba),
    (0x07CDBE, i_07cdbe),
    (0x07CDC4, i_07cdc4),
    (0x07CDCA, i_07cdca),
    (0x07CDD0, i_07cdd0),
    (0x07CDD6, i_07cdd6),
    (0x07CDD8, i_07cdd8),
    (0x07CDDC, i_07cddc),
    (0x07CDE2, i_07cde2),
    (0x07CDE8, i_07cde8),
    (0x07CDEE, i_07cdee),
    (0x07CDF4, i_07cdf4),
    (0x07CDF6, i_07cdf6),
    (0x07CDFA, i_07cdfa),
    (0x07CE00, i_07ce00),
    (0x07CE06, i_07ce06),
    (0x07CE0C, i_07ce0c),
    (0x07CE12, i_07ce12),
    (0x07CE14, i_07ce14),
    (0x07CE1A, i_07ce1a),
    (0x07CE1E, i_07ce1e),
    (0x07CE24, i_07ce24),
    (0x07CE28, i_07ce28),
    (0x07CE2E, i_07ce2e),
    (0x07CE32, i_07ce32),
    (0x07CE38, i_07ce38),
    (0x07CE3C, i_07ce3c),
    (0x07CE42, i_07ce42),
    (0x07CE46, i_07ce46),
    (0x07CE4C, i_07ce4c),
    (0x07CE50, i_07ce50),
    (0x07CE56, i_07ce56),
    (0x07CE5C, i_07ce5c),
    (0x07CE60, i_07ce60),
    (0x07CE66, i_07ce66),
    (0x07CE6C, i_07ce6c),
    (0x07CE72, i_07ce72),
    (0x07CE78, i_07ce78),
    (0x07CE7E, i_07ce7e),
    (0x07CE84, i_07ce84),
    (0x07CE8A, i_07ce8a),
    (0x07CE90, i_07ce90),
    (0x07CED8, i_07ced8),
    (0x07CEDE, i_07cede),
    (0x07CEE4, i_07cee4),
    (0x07CEE6, i_07cee6),
    (0x07CEEA, i_07ceea),
    (0x07CEEE, i_07ceee),
    (0x07CEF2, i_07cef2),
    (0x07CEF6, i_07cef6),
    (0x07CEFA, i_07cefa),
    (0x07CEFC, i_07cefc),
    (0x07CF00, i_07cf00),
    (0x07CF02, i_07cf02),
    (0x07CF08, i_07cf08),
    (0x07CF0C, i_07cf0c),
    (0x07CF12, i_07cf12),
    (0x07CF16, i_07cf16),
    (0x07CF1C, i_07cf1c),
    (0x07CF22, i_07cf22),
    (0x07CF28, i_07cf28),
    (0x07CF2A, i_07cf2a),
    (0x07CF2E, i_07cf2e),
    (0x07CF32, i_07cf32),
    (0x07CF38, i_07cf38),
    (0x07CF3E, i_07cf3e),
    (0x07CF44, i_07cf44),
    (0x07CF48, i_07cf48),
    (0x07CF4E, i_07cf4e),
    (0x07CF54, i_07cf54),
    (0x07CF58, i_07cf58),
    (0x07CF5A, i_07cf5a),
    (0x07CF5C, i_07cf5c),
    (0x07CF62, i_07cf62),
    (0x07CF66, i_07cf66),
    (0x07CF6C, i_07cf6c),
    (0x07CF72, i_07cf72),
    (0x07CF78, i_07cf78),
    (0x07CF7C, i_07cf7c),
    (0x07CF82, i_07cf82),
    (0x07CF88, i_07cf88),
    (0x07CF8C, i_07cf8c),
    (0x07CF92, i_07cf92),
    (0x07CF98, i_07cf98),
    (0x07CF9A, i_07cf9a),
    (0x07CF9E, i_07cf9e),
    (0x07CFA2, i_07cfa2),
    (0x07CFA8, i_07cfa8),
    (0x07CFAE, i_07cfae),
    (0x07CFB2, i_07cfb2),
    (0x07CFB4, i_07cfb4),
    (0x07CFBA, i_07cfba),
    (0x07CFC0, i_07cfc0),
    (0x07CFC4, i_07cfc4),
    (0x07CFC8, i_07cfc8),
    (0x07CFCE, i_07cfce),
    (0x07CFD2, i_07cfd2),
    (0x07CFD6, i_07cfd6),
    (0x07CFDC, i_07cfdc),
    (0x07CFE0, i_07cfe0),
    (0x07CFE4, i_07cfe4),
    (0x07CFE8, i_07cfe8),
    (0x07CFEC, i_07cfec),
    (0x07CFF2, i_07cff2),
    (0x07CFF6, i_07cff6),
    (0x07CFFA, i_07cffa),
    (0x07CFFC, i_07cffc),
    (0x07D002, i_07d002),
    (0x07D006, i_07d006),
    (0x07D00C, i_07d00c),
    (0x07D012, i_07d012),
    (0x07D018, i_07d018),
    (0x07D01E, i_07d01e),
    (0x07D022, i_07d022),
    (0x07D026, i_07d026),
    (0x07D028, i_07d028),
    (0x07D02E, i_07d02e),
    (0x07D032, i_07d032),
    (0x07D036, i_07d036),
    (0x07D038, i_07d038),
    (0x07D03E, i_07d03e),
    (0x07D044, i_07d044),
    (0x07D04A, i_07d04a),
    (0x07D04E, i_07d04e),
    (0x07D052, i_07d052),
    (0x07D054, i_07d054),
    (0x07D056, i_07d056),
    (0x07D05C, i_07d05c),
    (0x07D062, i_07d062),
    (0x07D068, i_07d068),
    (0x07D06C, i_07d06c),
    (0x07D070, i_07d070),
    (0x07D076, i_07d076),
    (0x07D078, i_07d078),
    (0x07D07C, i_07d07c),
    (0x07D080, i_07d080),
    (0x07D084, i_07d084),
    (0x07D08A, i_07d08a),
    (0x07D090, i_07d090),
    (0x07D094, i_07d094),
    (0x07D096, i_07d096),
    (0x07D09C, i_07d09c),
    (0x07D0A0, i_07d0a0),
    (0x07D0A6, i_07d0a6),
    (0x07D0AC, i_07d0ac),
    (0x07D0B0, i_07d0b0),
    (0x07D0B6, i_07d0b6),
    (0x07D0BA, i_07d0ba),
    (0x07D0BE, i_07d0be),
    (0x07D0C4, i_07d0c4),
    (0x07D0CA, i_07d0ca),
    (0x07D0CE, i_07d0ce),
    (0x07D0D2, i_07d0d2),
    (0x07D0D8, i_07d0d8),
    (0x07D0DC, i_07d0dc),
    (0x07D0E0, i_07d0e0),
    (0x07D0E4, i_07d0e4),
    (0x07D0E8, i_07d0e8),
    (0x07D0EC, i_07d0ec),
    (0x07D0F0, i_07d0f0),
    (0x07D0F2, i_07d0f2),
    (0x07D0F6, i_07d0f6),
    (0x07D0FA, i_07d0fa),
    (0x07D100, i_07d100),
    (0x07D104, i_07d104),
    (0x07D108, i_07d108),
    (0x07D10C, i_07d10c),
    (0x07D10E, i_07d10e),
    (0x07D112, i_07d112),
    (0x07D118, i_07d118),
    (0x07D11C, i_07d11c),
    (0x07D11E, i_07d11e),
    (0x07D124, i_07d124),
    (0x07D128, i_07d128),
    (0x07D12E, i_07d12e),
    (0x07D134, i_07d134),
    (0x07D13A, i_07d13a),
    (0x07D140, i_07d140),
    (0x07D144, i_07d144),
    (0x07D148, i_07d148),
    (0x07D14A, i_07d14a),
    (0x07D150, i_07d150),
    (0x07D154, i_07d154),
    (0x07D158, i_07d158),
    (0x07D15A, i_07d15a),
    (0x07D160, i_07d160),
    (0x07D166, i_07d166),
    (0x07D16C, i_07d16c),
    (0x07D170, i_07d170),
    (0x07D174, i_07d174),
    (0x07D176, i_07d176),
    (0x07D178, i_07d178),
    (0x07D17E, i_07d17e),
    (0x07D182, i_07d182),
    (0x07D186, i_07d186),
    (0x07D18C, i_07d18c),
    (0x07D192, i_07d192),
    (0x07D198, i_07d198),
    (0x07D19C, i_07d19c),
    (0x07D19E, i_07d19e),
    (0x07D1A4, i_07d1a4),
    (0x07D1AA, i_07d1aa),
    (0x07D1AE, i_07d1ae),
    (0x07D1B2, i_07d1b2),
    (0x07D1B8, i_07d1b8),
    (0x07D1BC, i_07d1bc),
    (0x07D1C0, i_07d1c0),
    (0x07D1C4, i_07d1c4),
    (0x07D1CA, i_07d1ca),
    (0x07D1CE, i_07d1ce),
    (0x07D1D2, i_07d1d2),
    (0x07D1D4, i_07d1d4),
    (0x07D1DA, i_07d1da),
    (0x07D1DE, i_07d1de),
    (0x07D1E4, i_07d1e4),
    (0x07D1EA, i_07d1ea),
    (0x07D1F0, i_07d1f0),
    (0x07D1F6, i_07d1f6),
    (0x07D1FA, i_07d1fa),
    (0x07D1FE, i_07d1fe),
    (0x07D200, i_07d200),
    (0x07D206, i_07d206),
    (0x07D20A, i_07d20a),
    (0x07D20E, i_07d20e),
    (0x07D210, i_07d210),
    (0x07D216, i_07d216),
    (0x07D21C, i_07d21c),
    (0x07D222, i_07d222),
    (0x07D226, i_07d226),
    (0x07D22A, i_07d22a),
    (0x07D22C, i_07d22c),
    (0x07D22E, i_07d22e),
    (0x07D234, i_07d234),
    (0x07D23A, i_07d23a),
    (0x07D240, i_07d240),
    (0x07D244, i_07d244),
    (0x07D248, i_07d248),
    (0x07D24E, i_07d24e),
    (0x07D250, i_07d250),
    (0x07D254, i_07d254),
    (0x07D258, i_07d258),
    (0x07D25E, i_07d25e),
    (0x07D264, i_07d264),
    (0x07D268, i_07d268),
    (0x07D26A, i_07d26a),
    (0x07D270, i_07d270),
    (0x07D274, i_07d274),
    (0x07D27A, i_07d27a),
    (0x07D27E, i_07d27e),
    (0x07D280, i_07d280),
    (0x07D286, i_07d286),
    (0x07D28A, i_07d28a),
    (0x07D290, i_07d290),
    (0x07D294, i_07d294),
    (0x07D298, i_07d298),
    (0x07D29E, i_07d29e),
    (0x07D2A4, i_07d2a4),
    (0x07D2A8, i_07d2a8),
    (0x07D2AC, i_07d2ac),
    (0x07D2B2, i_07d2b2),
    (0x07D2B6, i_07d2b6),
    (0x07D2BA, i_07d2ba),
    (0x07D2BE, i_07d2be),
    (0x07D2C2, i_07d2c2),
    (0x07D2C4, i_07d2c4),
    (0x07D2C8, i_07d2c8),
    (0x07D2CC, i_07d2cc),
    (0x07D2D2, i_07d2d2),
    (0x07D2D6, i_07d2d6),
    (0x07D2D8, i_07d2d8),
    (0x07D2DE, i_07d2de),
    (0x07D2E2, i_07d2e2),
    (0x07D2E8, i_07d2e8),
    (0x07D2EE, i_07d2ee),
    (0x07D2F4, i_07d2f4),
    (0x07D2FA, i_07d2fa),
    (0x07D2FE, i_07d2fe),
    (0x07D302, i_07d302),
    (0x07D304, i_07d304),
    (0x07D30A, i_07d30a),
    (0x07D30E, i_07d30e),
    (0x07D312, i_07d312),
    (0x07D314, i_07d314),
    (0x07D31A, i_07d31a),
    (0x07D320, i_07d320),
    (0x07D326, i_07d326),
    (0x07D32A, i_07d32a),
    (0x07D32E, i_07d32e),
    (0x07D330, i_07d330),
    (0x07D332, i_07d332),
    (0x07D338, i_07d338),
    (0x07D33E, i_07d33e),
    (0x07D344, i_07d344),
    (0x07D34A, i_07d34a),
    (0x07D350, i_07d350),
    (0x07D356, i_07d356),
    (0x07D35C, i_07d35c),
    (0x07D360, i_07d360),
    (0x07D366, i_07d366),
    (0x07D36A, i_07d36a),
    (0x07D36C, i_07d36c),
    (0x07D372, i_07d372),
    (0x07D378, i_07d378),
    (0x07D37C, i_07d37c),
    (0x07D380, i_07d380),
    (0x07D382, i_07d382),
    (0x07D388, i_07d388),
    (0x07D38C, i_07d38c),
    (0x07D390, i_07d390),
    (0x07D392, i_07d392),
    (0x07D398, i_07d398),
    (0x07D39E, i_07d39e),
    (0x07D3A4, i_07d3a4),
    (0x07D3A8, i_07d3a8),
    (0x07D3AC, i_07d3ac),
    (0x07D3AE, i_07d3ae),
    (0x07D3B0, i_07d3b0),
    (0x07D3B4, i_07d3b4),
    (0x07D3BA, i_07d3ba),
    (0x07D3C0, i_07d3c0),
    (0x07D3C6, i_07d3c6),
    (0x07D3CC, i_07d3cc),
    (0x07D3D2, i_07d3d2),
    (0x07D3D6, i_07d3d6),
    (0x07D3DC, i_07d3dc),
    (0x07D3E2, i_07d3e2),
    (0x07D3E8, i_07d3e8),
    (0x07D3EE, i_07d3ee),
    (0x07D3F4, i_07d3f4),
    (0x07D3F8, i_07d3f8),
    (0x07D3FE, i_07d3fe),
    (0x07D404, i_07d404),
    (0x07D408, i_07d408),
    (0x07D40E, i_07d40e),
    (0x07D414, i_07d414),
    (0x07D41A, i_07d41a),
    (0x07D420, i_07d420),
    (0x07D426, i_07d426),
    (0x07D42A, i_07d42a),
    (0x07D430, i_07d430),
    (0x07D434, i_07d434),
    (0x07D436, i_07d436),
    (0x07D43C, i_07d43c),
    (0x07D442, i_07d442),
    (0x07D448, i_07d448),
    (0x07D44E, i_07d44e),
    (0x07D454, i_07d454),
    (0x07D45A, i_07d45a),
    (0x07D45E, i_07d45e),
    (0x07D462, i_07d462),
    (0x07D466, i_07d466),
    (0x07D46C, i_07d46c),
    (0x07D470, i_07d470),
    (0x07D474, i_07d474),
    (0x07D47A, i_07d47a),
    (0x07D480, i_07d480),
    (0x07D486, i_07d486),
    (0x07D48C, i_07d48c),
    (0x07D492, i_07d492),
    (0x07D498, i_07d498),
    (0x07D49E, i_07d49e),
    (0x07D4A2, i_07d4a2),
    (0x07D4A6, i_07d4a6),
    (0x07D4A8, i_07d4a8),
    (0x07D4AA, i_07d4aa),
    (0x07D4AE, i_07d4ae),
    (0x07D4B4, i_07d4b4),
    (0x07D4BA, i_07d4ba),
    (0x07D4C0, i_07d4c0),
    (0x07D4C6, i_07d4c6),
    (0x07D4C8, i_07d4c8),
    (0x07D4CC, i_07d4cc),
    (0x07D4D0, i_07d4d0),
    (0x07D4D4, i_07d4d4),
    (0x07D4DA, i_07d4da),
    (0x07D4DE, i_07d4de),
    (0x07D4E4, i_07d4e4),
    (0x07D4EA, i_07d4ea),
    (0x07D4EE, i_07d4ee),
    (0x07D4F4, i_07d4f4),
    (0x07D4FA, i_07d4fa),
    (0x07D4FE, i_07d4fe),
    (0x07D504, i_07d504),
    (0x07D50A, i_07d50a),
    (0x07D510, i_07d510),
    (0x07D516, i_07d516),
    (0x07D51A, i_07d51a),
    (0x07D51C, i_07d51c),
    (0x07D564, i_07d564),
    (0x07D568, i_07d568),
    (0x07D56C, i_07d56c),
    (0x07D572, i_07d572),
    (0x07D576, i_07d576),
    (0x07D57C, i_07d57c),
    (0x07D582, i_07d582),
    (0x07D586, i_07d586),
    (0x07D58C, i_07d58c),
    (0x07D590, i_07d590),
    (0x07D594, i_07d594),
    (0x07D596, i_07d596),
    (0x07D59A, i_07d59a),
    (0x07D5A0, i_07d5a0),
    (0x07D5A4, i_07d5a4),
    (0x07D5AA, i_07d5aa),
    (0x07D5AE, i_07d5ae),
    (0x07D5B4, i_07d5b4),
    (0x07D5B8, i_07d5b8),
    (0x07D5BC, i_07d5bc),
    (0x07D5BE, i_07d5be),
    (0x07D5C0, i_07d5c0),
    (0x07D5C4, i_07d5c4),
    (0x07D5CA, i_07d5ca),
    (0x07D5CC, i_07d5cc),
    (0x07D5D0, i_07d5d0),
    (0x07D5D6, i_07d5d6),
    (0x07D5DA, i_07d5da),
    (0x07D5E0, i_07d5e0),
    (0x07D5E4, i_07d5e4),
    (0x07D5E6, i_07d5e6),
    (0x07D5EA, i_07d5ea),
    (0x07D5EE, i_07d5ee),
    (0x07D5F4, i_07d5f4),
    (0x07D5F8, i_07d5f8),
    (0x07D5FE, i_07d5fe),
    (0x07D604, i_07d604),
    (0x07D608, i_07d608),
    (0x07D60C, i_07d60c),
    (0x07D60E, i_07d60e),
    (0x07D612, i_07d612),
    (0x07D618, i_07d618),
    (0x07D61C, i_07d61c),
    (0x07D622, i_07d622),
    (0x07D626, i_07d626),
    (0x07D62C, i_07d62c),
    (0x07D630, i_07d630),
    (0x07D634, i_07d634),
    (0x07D636, i_07d636),
    (0x07D638, i_07d638),
    (0x07D63C, i_07d63c),
    (0x07D642, i_07d642),
    (0x07D644, i_07d644),
    (0x07D648, i_07d648),
    (0x07D64E, i_07d64e),
    (0x07D652, i_07d652),
    (0x07D658, i_07d658),
    (0x07D65C, i_07d65c),
    (0x07D65E, i_07d65e),
    (0x07D662, i_07d662),
    (0x07D666, i_07d666),
    (0x07D66C, i_07d66c),
    (0x07D670, i_07d670),
    (0x07D676, i_07d676),
    (0x07D67C, i_07d67c),
    (0x07D680, i_07d680),
    (0x07D686, i_07d686),
    (0x07D68A, i_07d68a),
    (0x07D690, i_07d690),
    (0x07D694, i_07d694),
    (0x07D69A, i_07d69a),
    (0x07D69E, i_07d69e),
    (0x07D6A2, i_07d6a2),
    (0x07D6A4, i_07d6a4),
    (0x07D6A6, i_07d6a6),
    (0x07D6AC, i_07d6ac),
    (0x07D6B2, i_07d6b2),
    (0x07D6D2, i_07d6d2),
    (0x07D6D6, i_07d6d6),
    (0x07D6DC, i_07d6dc),
    (0x07D6DE, i_07d6de),
    (0x07D6E2, i_07d6e2),
    (0x07D6E6, i_07d6e6),
    (0x07D6EC, i_07d6ec),
    (0x07D6F2, i_07d6f2),
    (0x07D6F8, i_07d6f8),
    (0x07D6FE, i_07d6fe),
    (0x07D702, i_07d702),
    (0x07D704, i_07d704),
    (0x07D708, i_07d708),
    (0x07D70E, i_07d70e),
    (0x07D714, i_07d714),
    (0x07D718, i_07d718),
    (0x07D71A, i_07d71a),
    (0x07D71E, i_07d71e),
    (0x07D724, i_07d724),
    (0x07D728, i_07d728),
    (0x07D72C, i_07d72c),
    (0x07D732, i_07d732),
    (0x07D736, i_07d736),
    (0x07D73A, i_07d73a),
    (0x07D73C, i_07d73c),
    (0x07D742, i_07d742),
    (0x07D746, i_07d746),
    (0x07D74A, i_07d74a),
    (0x07D74C, i_07d74c),
    (0x07D752, i_07d752),
    (0x07D756, i_07d756),
    (0x07D75A, i_07d75a),
    (0x07D75C, i_07d75c),
    (0x07D762, i_07d762),
    (0x07D766, i_07d766),
    (0x07D76A, i_07d76a),
    (0x07D76C, i_07d76c),
    (0x07D76E, i_07d76e),
    (0x07D774, i_07d774),
    (0x07D778, i_07d778),
    (0x07D77E, i_07d77e),
    (0x07D784, i_07d784),
    (0x07D788, i_07d788),
    (0x07D78A, i_07d78a),
    (0x07D78E, i_07d78e),
    (0x07D794, i_07d794),
    (0x07D798, i_07d798),
    (0x07D79A, i_07d79a),
    (0x07D79C, i_07d79c),
    (0x07D7A0, i_07d7a0),
    (0x07D7A4, i_07d7a4),
    (0x07D7A8, i_07d7a8),
    (0x07D7AC, i_07d7ac),
    (0x07D7AE, i_07d7ae),
    (0x07D7B0, i_07d7b0),
    (0x07D7B4, i_07d7b4),
    (0x07D7BA, i_07d7ba),
    (0x07D7BE, i_07d7be),
    (0x07D7C0, i_07d7c0),
    (0x07D7C4, i_07d7c4),
    (0x07D7C8, i_07d7c8),
    (0x07D7CE, i_07d7ce),
    (0x07D7D2, i_07d7d2),
    (0x07D7D6, i_07d7d6),
    (0x07D7DA, i_07d7da),
    (0x07D7E0, i_07d7e0),
    (0x07D7E4, i_07d7e4),
    (0x07D7E6, i_07d7e6),
    (0x07D7E8, i_07d7e8),
    (0x07D7EC, i_07d7ec),
    (0x07D7F0, i_07d7f0),
    (0x07D7F4, i_07d7f4),
    (0x07D7F6, i_07d7f6),
    (0x07D7FA, i_07d7fa),
    (0x07D7FE, i_07d7fe),
    (0x07D802, i_07d802),
    (0x07D808, i_07d808),
    (0x07D80C, i_07d80c),
    (0x07D812, i_07d812),
    (0x07D816, i_07d816),
    (0x07D818, i_07d818),
    (0x07D81C, i_07d81c),
    (0x07D820, i_07d820),
    (0x07D826, i_07d826),
    (0x07D82A, i_07d82a),
    (0x07D82E, i_07d82e),
    (0x07D832, i_07d832),
    (0x07D838, i_07d838),
    (0x07D83C, i_07d83c),
    (0x07D840, i_07d840),
    (0x07D842, i_07d842),
    (0x07D846, i_07d846),
    (0x07D84A, i_07d84a),
    (0x07D850, i_07d850),
    (0x07D854, i_07d854),
    (0x07D858, i_07d858),
    (0x07D85A, i_07d85a),
    (0x07D860, i_07d860),
    (0x07D864, i_07d864),
    (0x07D868, i_07d868),
    (0x07D86A, i_07d86a),
    (0x07D870, i_07d870),
    (0x07D874, i_07d874),
    (0x07D878, i_07d878),
    (0x07D87A, i_07d87a),
    (0x07D87E, i_07d87e),
    (0x07D882, i_07d882),
    (0x07D886, i_07d886),
    (0x07D888, i_07d888),
    (0x07D88C, i_07d88c),
    (0x07D890, i_07d890),
    (0x07D894, i_07d894),
    (0x07D896, i_07d896),
    (0x07D898, i_07d898),
    (0x07D89E, i_07d89e),
    (0x07D8A2, i_07d8a2),
    (0x07D8A6, i_07d8a6),
    (0x07D8A8, i_07d8a8),
    (0x07D8AA, i_07d8aa),
    (0x07D8AE, i_07d8ae),
    (0x07D8B2, i_07d8b2),
    (0x07D8B4, i_07d8b4),
    (0x07D8B8, i_07d8b8),
    (0x07D8BE, i_07d8be),
    (0x07D8C4, i_07d8c4),
    (0x07D8CA, i_07d8ca),
    (0x07D8CE, i_07d8ce),
    (0x07D8D0, i_07d8d0),
    (0x07D8D2, i_07d8d2),
    (0x07D8D4, i_07d8d4),
    (0x07D8DA, i_07d8da),
    (0x07D8E0, i_07d8e0),
    (0x07D8E4, i_07d8e4),
    (0x07D8EA, i_07d8ea),
    (0x07D8F0, i_07d8f0),
    (0x07D8F6, i_07d8f6),
    (0x07D8FC, i_07d8fc),
    (0x07D900, i_07d900),
    (0x07D904, i_07d904),
    (0x07D90A, i_07d90a),
    (0x07D910, i_07d910),
    (0x07D916, i_07d916),
    (0x07D91C, i_07d91c),
    (0x07D922, i_07d922),
    (0x07D926, i_07d926),
    (0x07D928, i_07d928),
    (0x07D92E, i_07d92e),
    (0x07D934, i_07d934),
    (0x07D938, i_07d938),
    (0x07D93C, i_07d93c),
    (0x07D942, i_07d942),
    (0x07D946, i_07d946),
    (0x07D94C, i_07d94c),
    (0x07D952, i_07d952),
    (0x07D958, i_07d958),
    (0x07D95E, i_07d95e),
    (0x07D962, i_07d962),
    (0x07D968, i_07d968),
    (0x07D96C, i_07d96c),
    (0x07D972, i_07d972),
    (0x07D978, i_07d978),
    (0x07D980, i_07d980),
    (0x07D986, i_07d986),
    (0x07D98A, i_07d98a),
    (0x07D98E, i_07d98e),
    (0x07D992, i_07d992),
    (0x07D996, i_07d996),
    (0x07D99C, i_07d99c),
    (0x07D9A0, i_07d9a0),
    (0x07D9A4, i_07d9a4),
    (0x07D9AA, i_07d9aa),
    (0x07D9AE, i_07d9ae),
    (0x07D9B0, i_07d9b0),
    (0x07D9B2, i_07d9b2),
    (0x07D9B6, i_07d9b6),
    (0x07D9B8, i_07d9b8),
    (0x07D9BC, i_07d9bc),
    (0x07D9C2, i_07d9c2),
    (0x07D9C6, i_07d9c6),
    (0x07D9CA, i_07d9ca),
    (0x07D9CE, i_07d9ce),
    (0x07D9D4, i_07d9d4),
    (0x07D9DA, i_07d9da),
    (0x07D9DE, i_07d9de),
    (0x07D9E0, i_07d9e0),
    (0x07D9E6, i_07d9e6),
    (0x07D9E8, i_07d9e8),
    (0x07D9EE, i_07d9ee),
    (0x07D9F2, i_07d9f2),
    (0x07D9F6, i_07d9f6),
    (0x07D9FA, i_07d9fa),
    (0x07DA00, i_07da00),
    (0x07DA04, i_07da04),
    (0x07DA08, i_07da08),
    (0x07DA0C, i_07da0c),
    (0x07DA10, i_07da10),
    (0x07DA16, i_07da16),
    (0x07DA1C, i_07da1c),
    (0x07DA22, i_07da22),
    (0x07DA28, i_07da28),
    (0x07DA2E, i_07da2e),
    (0x07DA34, i_07da34),
    (0x07DA3A, i_07da3a),
    (0x07DA3E, i_07da3e),
    (0x07DA42, i_07da42),
    (0x07DA44, i_07da44),
    (0x07DA46, i_07da46),
    (0x07DA4A, i_07da4a),
    (0x07DA50, i_07da50),
    (0x07DA54, i_07da54),
    (0x07DA5A, i_07da5a),
    (0x07DA60, i_07da60),
    (0x07DA66, i_07da66),
    (0x07DA6C, i_07da6c),
    (0x07DA70, i_07da70),
    (0x07DA74, i_07da74),
    (0x07DA7A, i_07da7a),
    (0x07DA7E, i_07da7e),
    (0x07DA82, i_07da82),
    (0x07DA88, i_07da88),
    (0x07DA8C, i_07da8c),
    (0x07DA90, i_07da90),
    (0x07DA96, i_07da96),
    (0x07DA9A, i_07da9a),
    (0x07DA9E, i_07da9e),
    (0x07DAA4, i_07daa4),
    (0x07DAA8, i_07daa8),
    (0x07DAAA, i_07daaa),
    (0x07DAAC, i_07daac),
    (0x07DAB2, i_07dab2),
    (0x07DAB8, i_07dab8),
    (0x07DABC, i_07dabc),
    (0x07DAC2, i_07dac2),
    (0x07DAC8, i_07dac8),
    (0x07DACE, i_07dace),
    (0x07DAD0, i_07dad0),
    (0x07DAD4, i_07dad4),
    (0x07DAD8, i_07dad8),
    (0x07DADE, i_07dade),
    (0x07DAE4, i_07dae4),
    (0x07DAEA, i_07daea),
    (0x07DAEE, i_07daee),
    (0x07DAF0, i_07daf0),
    (0x07DAF4, i_07daf4),
    (0x07DAF8, i_07daf8),
    (0x07DAFE, i_07dafe),
    (0x07DB02, i_07db02),
    (0x07DB08, i_07db08),
    (0x07DB0E, i_07db0e),
    (0x07DB14, i_07db14),
    (0x07DB1A, i_07db1a),
    (0x07DB1E, i_07db1e),
    (0x07DB22, i_07db22),
    (0x07DB28, i_07db28),
    (0x07DB2C, i_07db2c),
    (0x07DB30, i_07db30),
    (0x07DB36, i_07db36),
    (0x07DB3A, i_07db3a),
    (0x07DB3E, i_07db3e),
    (0x07DB44, i_07db44),
    (0x07DB48, i_07db48),
    (0x07DB4C, i_07db4c),
    (0x07DB52, i_07db52),
    (0x07DB56, i_07db56),
    (0x07DB58, i_07db58),
    (0x07DB5A, i_07db5a),
    (0x07DB60, i_07db60),
    (0x07DB66, i_07db66),
    (0x07DB6A, i_07db6a),
    (0x07DB70, i_07db70),
    (0x07DB76, i_07db76),
    (0x07DB7C, i_07db7c),
    (0x07DB7E, i_07db7e),
    (0x07DB82, i_07db82),
    (0x07DB86, i_07db86),
    (0x07DB8C, i_07db8c),
    (0x07DB92, i_07db92),
    (0x07DB98, i_07db98),
    (0x07DB9C, i_07db9c),
    (0x07DB9E, i_07db9e),
    (0x07DBA2, i_07dba2),
    (0x07DBA8, i_07dba8),
    (0x07DBAE, i_07dbae),
    (0x07DBB0, i_07dbb0),
    (0x07DBB4, i_07dbb4),
    (0x07DBB6, i_07dbb6),
    (0x07DBB8, i_07dbb8),
    (0x07DBBC, i_07dbbc),
    (0x07DBBE, i_07dbbe),
    (0x07DBC0, i_07dbc0),
    (0x07DBC4, i_07dbc4),
    (0x07DBC8, i_07dbc8),
    (0x07DBCA, i_07dbca),
    (0x07DBCC, i_07dbcc),
    (0x07DBD0, i_07dbd0),
    (0x07DBD2, i_07dbd2),
    (0x07DBD4, i_07dbd4),
    (0x07DBD8, i_07dbd8),
    (0x07DBDA, i_07dbda),
    (0x07DBDE, i_07dbde),
    (0x07DBE2, i_07dbe2),
    (0x07DBE6, i_07dbe6),
    (0x07DBEA, i_07dbea),
    (0x07DBEE, i_07dbee),
    (0x07DBF4, i_07dbf4),
    (0x07DBF8, i_07dbf8),
    (0x07DBFC, i_07dbfc),
    (0x07DC00, i_07dc00),
    (0x07DC04, i_07dc04),
    (0x07DC08, i_07dc08),
    (0x07DC0C, i_07dc0c),
    (0x07DC10, i_07dc10),
    (0x07DC14, i_07dc14),
    (0x07DC18, i_07dc18),
    (0x07DC1A, i_07dc1a),
    (0x07DC1E, i_07dc1e),
    (0x07DC24, i_07dc24),
    (0x07DC28, i_07dc28),
    (0x07DC2C, i_07dc2c),
    (0x07DC30, i_07dc30),
    (0x07DC34, i_07dc34),
    (0x07DC38, i_07dc38),
    (0x07DC3C, i_07dc3c),
    (0x07DC3E, i_07dc3e),
    (0x07DC42, i_07dc42),
    (0x07DC48, i_07dc48),
    (0x07DC4E, i_07dc4e),
    (0x07DC54, i_07dc54),
    (0x07DC56, i_07dc56),
    (0x07DC5A, i_07dc5a),
    (0x07DC5E, i_07dc5e),
    (0x07DC64, i_07dc64),
    (0x07DC6A, i_07dc6a),
    (0x07DC6E, i_07dc6e),
    (0x07DC74, i_07dc74),
    (0x07DC7A, i_07dc7a),
    (0x07DC7E, i_07dc7e),
    (0x07DC84, i_07dc84),
    (0x07DC88, i_07dc88),
    (0x07DC8C, i_07dc8c),
    (0x07DC8E, i_07dc8e),
    (0x07DC92, i_07dc92),
    (0x07DC94, i_07dc94),
    (0x07DC98, i_07dc98),
    (0x07DC9E, i_07dc9e),
    (0x07DCA4, i_07dca4),
    (0x07DCA8, i_07dca8),
    (0x07DCAE, i_07dcae),
    (0x07DCB2, i_07dcb2),
    (0x07DCB4, i_07dcb4),
    (0x07DCB6, i_07dcb6),
    (0x07DCBC, i_07dcbc),
    (0x07DCC0, i_07dcc0),
    (0x07DCC6, i_07dcc6),
    (0x07DCCC, i_07dccc),
    (0x07DCD2, i_07dcd2),
    (0x07DCD6, i_07dcd6),
    (0x07DCDC, i_07dcdc),
    (0x07DCE2, i_07dce2),
    (0x07DCE6, i_07dce6),
    (0x07DCEC, i_07dcec),
    (0x07DCF2, i_07dcf2),
    (0x07DCF8, i_07dcf8),
    (0x07DCFE, i_07dcfe),
    (0x07DD02, i_07dd02),
    (0x07DD06, i_07dd06),
    (0x07DD0C, i_07dd0c),
    (0x07DD0E, i_07dd0e),
    (0x07DD12, i_07dd12),
    (0x07DD16, i_07dd16),
    (0x07DD1A, i_07dd1a),
    (0x07DD1E, i_07dd1e),
    (0x07DD24, i_07dd24),
    (0x07DD2A, i_07dd2a),
    (0x07DD30, i_07dd30),
    (0x07DD34, i_07dd34),
    (0x07DD36, i_07dd36),
    (0x07DD3C, i_07dd3c),
    (0x07DD40, i_07dd40),
    (0x07DD44, i_07dd44),
    (0x07DD46, i_07dd46),
    (0x07DD4A, i_07dd4a),
    (0x07DD4E, i_07dd4e),
    (0x07DD52, i_07dd52),
    (0x07DD56, i_07dd56),
    (0x07DD5A, i_07dd5a),
    (0x07DD5E, i_07dd5e),
    (0x07DD64, i_07dd64),
    (0x07DD6A, i_07dd6a),
    (0x07DD6E, i_07dd6e),
    (0x07DD72, i_07dd72),
    (0x07DD78, i_07dd78),
    (0x07DD7E, i_07dd7e),
    (0x07DD82, i_07dd82),
    (0x07DD88, i_07dd88),
    (0x07DD8E, i_07dd8e),
    (0x07DD94, i_07dd94),
    (0x07DD98, i_07dd98),
    (0x07DD9C, i_07dd9c),
    (0x07DDA0, i_07dda0),
    (0x07DDA2, i_07dda2),
    (0x07DDA8, i_07dda8),
    (0x07DDAC, i_07ddac),
    (0x07DDB0, i_07ddb0),
    (0x07DDB2, i_07ddb2),
    (0x07DDB8, i_07ddb8),
    (0x07DDBC, i_07ddbc),
    (0x07DDC0, i_07ddc0),
    (0x07DDC2, i_07ddc2),
    (0x07DDC8, i_07ddc8),
    (0x07DDCE, i_07ddce),
    (0x07DDD4, i_07ddd4),
    (0x07DDD8, i_07ddd8),
    (0x07DDDC, i_07dddc),
    (0x07DDDE, i_07ddde),
    (0x07DDE0, i_07dde0),
    (0x07DDE6, i_07dde6),
    (0x07DDEA, i_07ddea),
    (0x07DDEE, i_07ddee),
    (0x07DDF4, i_07ddf4),
    (0x07DDFA, i_07ddfa),
    (0x07DE00, i_07de00),
    (0x07DE04, i_07de04),
    (0x07DE06, i_07de06),
    (0x07DE0C, i_07de0c),
    (0x07DE12, i_07de12),
    (0x07DE18, i_07de18),
    (0x07DE1C, i_07de1c),
    (0x07DE22, i_07de22),
    (0x07DE28, i_07de28),
    (0x07DE2E, i_07de2e),
    (0x07DE34, i_07de34),
    (0x07DE38, i_07de38),
    (0x07DE3C, i_07de3c),
    (0x07DE42, i_07de42),
    (0x07DE46, i_07de46),
    (0x07DE4A, i_07de4a),
    (0x07DE4C, i_07de4c),
    (0x07DE50, i_07de50),
    (0x07DE56, i_07de56),
    (0x07DE5A, i_07de5a),
    (0x07DE5E, i_07de5e),
    (0x07DE60, i_07de60),
    (0x07DE66, i_07de66),
    (0x07DE6A, i_07de6a),
    (0x07DE6E, i_07de6e),
    (0x07DE70, i_07de70),
    (0x07DE76, i_07de76),
    (0x07DE7C, i_07de7c),
    (0x07DE82, i_07de82),
    (0x07DE86, i_07de86),
    (0x07DE8A, i_07de8a),
    (0x07DE8C, i_07de8c),
    (0x07DE8E, i_07de8e),
    (0x07DE94, i_07de94),
    (0x07DE9A, i_07de9a),
    (0x07DEA0, i_07dea0),
    (0x07DEA4, i_07dea4),
    (0x07DEA8, i_07dea8),
    (0x07DEAE, i_07deae),
    (0x07DEB0, i_07deb0),
    (0x07DEB4, i_07deb4),
    (0x07DEB8, i_07deb8),
    (0x07DEBE, i_07debe),
    (0x07DEC4, i_07dec4),
    (0x07DEC8, i_07dec8),
    (0x07DECA, i_07deca),
    (0x07DED0, i_07ded0),
    (0x07DED4, i_07ded4),
    (0x07DEDA, i_07deda),
    (0x07DEDE, i_07dede),
    (0x07DEE0, i_07dee0),
    (0x07DEE4, i_07dee4),
    (0x07DEE8, i_07dee8),
    (0x07DEEC, i_07deec),
    (0x07DEF0, i_07def0),
    (0x07DEF4, i_07def4),
    (0x07DEF8, i_07def8),
    (0x07DEFE, i_07defe),
    (0x07DF04, i_07df04),
    (0x07DF08, i_07df08),
    (0x07DF0C, i_07df0c),
    (0x07DF12, i_07df12),
    (0x07DF18, i_07df18),
    (0x07DF1C, i_07df1c),
    (0x07DF22, i_07df22),
    (0x07DF28, i_07df28),
    (0x07DF2E, i_07df2e),
    (0x07DF34, i_07df34),
    (0x07DF38, i_07df38),
    (0x07DF3C, i_07df3c),
    (0x07DF3E, i_07df3e),
    (0x07DF44, i_07df44),
    (0x07DF48, i_07df48),
    (0x07DF4C, i_07df4c),
    (0x07DF4E, i_07df4e),
    (0x07DF54, i_07df54),
    (0x07DF5A, i_07df5a),
    (0x07DF60, i_07df60),
    (0x07DF64, i_07df64),
    (0x07DF68, i_07df68),
    (0x07DF6A, i_07df6a),
    (0x07DF6C, i_07df6c),
    (0x07DF72, i_07df72),
    (0x07DF76, i_07df76),
    (0x07DF7A, i_07df7a),
    (0x07DF80, i_07df80),
    (0x07DF84, i_07df84),
    (0x07DF88, i_07df88),
    (0x07DF8C, i_07df8c),
    (0x07DF8E, i_07df8e),
    (0x07DF92, i_07df92),
    (0x07DF96, i_07df96),
    (0x07DF9C, i_07df9c),
    (0x07DFA0, i_07dfa0),
    (0x07DFA2, i_07dfa2),
    (0x07DFA6, i_07dfa6),
    (0x07DFA8, i_07dfa8),
    (0x07DFAC, i_07dfac),
    (0x07DFB0, i_07dfb0),
    (0x07DFB4, i_07dfb4),
    (0x07DFB8, i_07dfb8),
    (0x07DFBC, i_07dfbc),
    (0x07DFBE, i_07dfbe),
    (0x07DFC2, i_07dfc2),
    (0x07DFC4, i_07dfc4),
    (0x07DFC6, i_07dfc6),
    (0x07DFCA, i_07dfca),
    (0x07DFCE, i_07dfce),
    (0x07DFD2, i_07dfd2),
    (0x07DFD8, i_07dfd8),
    (0x07DFDE, i_07dfde),
    (0x07DFE4, i_07dfe4),
    (0x07DFEA, i_07dfea),
    (0x07DFEE, i_07dfee),
    (0x07DFF2, i_07dff2),
    (0x07DFF8, i_07dff8),
    (0x07DFFC, i_07dffc),
    (0x07DFFE, i_07dffe),
    (0x07E000, i_07e000),
    (0x07E002, i_07e002),
    (0x07E006, i_07e006),
    (0x07E00A, i_07e00a),
    (0x07E00E, i_07e00e),
    (0x07E014, i_07e014),
    (0x07E01A, i_07e01a),
    (0x07E020, i_07e020),
    (0x07E026, i_07e026),
    (0x07E02A, i_07e02a),
    (0x07E02E, i_07e02e),
    (0x07E034, i_07e034),
    (0x07E038, i_07e038),
    (0x07E03A, i_07e03a),
    (0x07E03C, i_07e03c),
    (0x07E040, i_07e040),
    (0x07E044, i_07e044),
    (0x07E04A, i_07e04a),
    (0x07E050, i_07e050),
    (0x07E056, i_07e056),
    (0x07E05C, i_07e05c),
    (0x07E062, i_07e062),
    (0x07E066, i_07e066),
    (0x07E06A, i_07e06a),
    (0x07E070, i_07e070),
    (0x07E074, i_07e074),
    (0x07E076, i_07e076),
    (0x07E078, i_07e078),
    (0x07E07C, i_07e07c),
    (0x07E082, i_07e082),
    (0x07E088, i_07e088),
    (0x07E08C, i_07e08c),
    (0x07E090, i_07e090),
    (0x07E096, i_07e096),
    (0x07E09C, i_07e09c),
    (0x07E09E, i_07e09e),
    (0x07E0A4, i_07e0a4),
    (0x07E0A8, i_07e0a8),
    (0x07E0AC, i_07e0ac),
    (0x07E0B0, i_07e0b0),
    (0x07E0B4, i_07e0b4),
    (0x07E0BA, i_07e0ba),
    (0x07E0BE, i_07e0be),
];
