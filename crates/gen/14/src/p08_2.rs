// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $08473E: beq.w $84748
pub fn i_08473e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84742;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84748; return 10; }
    8
}
/// $084742: jsr $28cd4.l
pub fn i_084742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84748;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84748);
    cpu.pc = 0x28CD4;
    16
}
/// $084748: jsr $5e9b6.l
pub fn i_084748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8474E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8474E);
    cpu.pc = 0x5E9B6;
    16
}
/// $08474E: andi.l #$f, d0
pub fn i_08474e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84754;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $084754: asl.l #$2, d0
pub fn i_084754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84756;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $084756: lea.l $2eb02c.l, a0
pub fn i_084756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8475C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB02C;
    20
}
/// $08475C: movea.l (a0, d0.w), a1
pub fn i_08475c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84760;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $084760: jsr $4ae.l
pub fn i_084760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84766;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84766);
    cpu.pc = 0x4AE;
    16
}
/// $084766: jsr $5dd22.l
pub fn i_084766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8476C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8476C);
    cpu.pc = 0x5DD22;
    16
}
/// $08476C: addi.w #$4, $22(a0)
pub fn i_08476c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84772;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084772: addi.w #$1, $24(a0)
pub fn i_084772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84778;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084778: andi.b #$1, d0
pub fn i_084778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8477C;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08477C: add.b d0, $98(a0)
pub fn i_08477c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84780;
    cpu.ir = 0xD128;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $084780: addi.b #$7f, $99(a0)
pub fn i_084780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84786;
    cpu.ir = 0x628;
    let ad1 = cpu.a[0].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x7F, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $084786: move.b #$1, $11(a0)
pub fn i_084786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8478C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08478C: move.w #$8000, $38(a6)
pub fn i_08478c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84792;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084792: jsr $267e2.l
pub fn i_084792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84798;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84798);
    cpu.pc = 0x267E2;
    16
}
/// $084798: jsr $27cee.l
pub fn i_084798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8479E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8479E);
    cpu.pc = 0x27CEE;
    16
}
/// $08479E: lea.l $847a4(pc), a1
pub fn i_08479e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x847A4;
    16
}
/// $0847A2: move.l a1, (a6)
pub fn i_0847a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0847A4: jsr $2783a.l
pub fn i_0847a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847AA);
    cpu.pc = 0x2783A;
    16
}
/// $0847AA: cmpi.w #$150, $22(a6)
pub fn i_0847aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847B0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $0847B0: bgt.w $84834
pub fn i_0847b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847B4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x84834; return 10; }
    8
}
/// $0847B4: jsr $28d70.l
pub fn i_0847b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847BA);
    cpu.pc = 0x28D70;
    16
}
/// $0847BA: movea.l $c(a6), a0
pub fn i_0847ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847BE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0847BE: cmpi.b #$ff, $21(a0)
pub fn i_0847be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847C4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0847C4: beq.w $84828
pub fn i_0847c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84828; return 10; }
    8
}
/// $0847C8: cmpi.l #$ffffffff, $3c(a6)
pub fn i_0847c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847D0;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0847D0: beq.w $8481e
pub fn i_0847d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8481E; return 10; }
    8
}
/// $0847D4: jsr $2870a.l
pub fn i_0847d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847DA);
    cpu.pc = 0x2870A;
    16
}
/// $0847DA: bcc.w $847f0
pub fn i_0847da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x847F0; return 10; }
    8
}
/// $0847DE: lea.l $5e766.l, a0
pub fn i_0847de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0847E4: jsr $5e770.l
pub fn i_0847e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847EA);
    cpu.pc = 0x5E770;
    16
}
/// $0847EA: bclr.b #$3, $13(a6)
pub fn i_0847ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847F0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0847F0: jsr $28758.l
pub fn i_0847f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x847F6);
    cpu.pc = 0x28758;
    16
}
/// $0847F6: bcc.w $8481e
pub fn i_0847f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8481E; return 10; }
    8
}
/// $0847FA: move.w #$102e, d0
pub fn i_0847fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x847FE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0847FE: jsr $2352.l
pub fn i_0847fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84804);
    cpu.pc = 0x2352;
    16
}
/// $084804: lea.l $ffff.w, a0
pub fn i_084804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84808;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084808: jsr $28cd4.l
pub fn i_084808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8480E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8480E);
    cpu.pc = 0x28CD4;
    16
}
/// $08480E: lea.l $2e9adc.l, a1
pub fn i_08480e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84814;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9ADC;
    20
}
/// $084814: jsr $77c7e.l
pub fn i_084814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8481A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8481A);
    cpu.pc = 0x77C7E;
    16
}
/// $08481A: bra.w $84834
pub fn i_08481a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8481E;
    cpu.ir = 0x6000;
    cpu.pc = 0x84834;
    10
}
/// $08481E: jsr $4fa70.l
pub fn i_08481e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84824;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84824);
    cpu.pc = 0x4FA70;
    16
}
/// $084824: bcc.w $84834
pub fn i_084824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84828;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84834; return 10; }
    8
}
// ===== Jsr5B6ThenJmpScheduler_084828  ($084828, 14 B, kernel/jsr5b6_jmp518.c)
/// $084828: jsr $5b6.l
pub fn i_084828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8482E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8482E);
    cpu.pc = 0x5B6;
    16
}
/// $08482E: jmp $518.l
pub fn i_08482e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84834;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084834: rts
pub fn i_084834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84836;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084836  ($084836, 92 B, enemies/squad/rescue_squad_084836.s)
/// $084836: move.w #$a9, d1
pub fn i_084836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8483A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08483A: jsr $236e.l
pub fn i_08483a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84840;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84840);
    cpu.pc = 0x236E;
    16
}
/// $084840: move.b #$ff, $32(a6)
pub fn i_084840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84846;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084846: move.b #$ff, $33(a6)
pub fn i_084846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8484C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08484C: clr.l d0
pub fn i_08484c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8484E;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08484E: move.b $21(a6), d0
pub fn i_08484e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84852;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084852: asl.l #$1, d0
pub fn i_084852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84854;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $084854: movea.l #$2e77ca, a0
pub fn i_084854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8485A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E77CA;
    cpu.a[0] = v1;
    4
}
/// $08485A: lsl.w #$2, d0
pub fn i_08485a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8485C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08485C: movea.l (a0, d0.w), a0
pub fn i_08485c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84860;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $084860: cmpa.l #$ffffffff, a0
pub fn i_084860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84866;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $084866: beq.w $84870
pub fn i_084866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8486A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84870; return 10; }
    8
}
/// $08486A: jsr $28cd4.l
pub fn i_08486a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84870);
    cpu.pc = 0x28CD4;
    16
}
/// $084870: lea.l $84876(pc), a1
pub fn i_084870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84874;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84876;
    16
}
/// $084874: move.l a1, (a6)
pub fn i_084874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84876;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084876: jsr $2783a.l
pub fn i_084876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8487C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8487C);
    cpu.pc = 0x2783A;
    16
}
/// $08487C: jsr $28d70.l
pub fn i_08487c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84882;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84882);
    cpu.pc = 0x28D70;
    16
}
/// $084882: movea.l $c(a6), a0
pub fn i_084882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84886;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084886: move.b $21(a6), d0
pub fn i_084886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8488A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08488A: cmp.b $21(a0), d0
pub fn i_08488a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8488E;
    cpu.ir = 0xB028;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08488E: bne.w $84898
pub fn i_08488e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84892;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84898; return 10; }
    8
}
// ===== SetTaskHandler_084892  ($084892, 8 B, kernel/task_handlers.c)
/// $084892: lea.l $8489a(pc), a1
pub fn i_084892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84896;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8489A;
    16
}
/// $084896: move.l a1, (a6)
pub fn i_084896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84898;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084898: rts
pub fn i_084898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8489A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08489a  ($08489A, 60 B, enemies/squad/rescue_squad_084836.s)
/// $08489A: clr.l d0
pub fn i_08489a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8489C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08489C: move.b $21(a6), d0
pub fn i_08489c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848A0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0848A0: asl.l #$1, d0
pub fn i_0848a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848A2;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $0848A2: addq.l #$1, d0
pub fn i_0848a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848A4;
    cpu.ir = 0x5280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Long, true);
    cpu.d[0] = r2;
    4
}
/// $0848A4: movea.l #$2e77ca, a0
pub fn i_0848a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848AA;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E77CA;
    cpu.a[0] = v1;
    4
}
/// $0848AA: lsl.w #$2, d0
pub fn i_0848aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848AC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0848AC: movea.l (a0, d0.w), a0
pub fn i_0848ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848B0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0848B0: cmpa.l #$ffffffff, a0
pub fn i_0848b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848B6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0848B6: beq.w $848c0
pub fn i_0848b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x848C0; return 10; }
    8
}
/// $0848BA: jsr $28cd4.l
pub fn i_0848ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x848C0);
    cpu.pc = 0x28CD4;
    16
}
/// $0848C0: lea.l $848c6(pc), a1
pub fn i_0848c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x848C6;
    16
}
/// $0848C4: move.l a1, (a6)
pub fn i_0848c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0848C6: jsr $2783a.l
pub fn i_0848c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x848CC);
    cpu.pc = 0x2783A;
    16
}
/// $0848CC: jsr $28d70.l
pub fn i_0848cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x848D2);
    cpu.pc = 0x28D70;
    16
}
/// $0848D2: bcc.w $848dc
pub fn i_0848d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x848DC; return 10; }
    8
}
// ===== SetTaskHandler_0848d6  ($0848D6, 8 B, kernel/task_handlers.c)
/// $0848D6: lea.l $848de(pc), a1
pub fn i_0848d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x848DE;
    16
}
/// $0848DA: move.l a1, (a6)
pub fn i_0848da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0848DC: rts
pub fn i_0848dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848DE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0848de  ($0848DE, 76 B, enemies/squad/rescue_squad_084836.s)
/// $0848DE: movea.l $c(a6), a0
pub fn i_0848de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848E2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0848E2: addq.b #$1, $21(a0)
pub fn i_0848e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848E6;
    cpu.ir = 0x5228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0848E6: jsr $267e2.l
pub fn i_0848e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x848EC);
    cpu.pc = 0x267E2;
    16
}
/// $0848EC: move.w #$fff0, $2e(a6)
pub fn i_0848ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0848F2: lea.l $848f8(pc), a1
pub fn i_0848f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x848F8;
    16
}
/// $0848F6: move.l a1, (a6)
pub fn i_0848f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0848F8: jsr $863a0(pc)
pub fn i_0848f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x848FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x848FC);
    cpu.pc = 0x863A0;
    16
}
/// $0848FC: jsr $27cee.l
pub fn i_0848fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84902;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84902);
    cpu.pc = 0x27CEE;
    16
}
/// $084902: bcc.w $8492a
pub fn i_084902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84906;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8492A; return 10; }
    8
}
/// $084906: jsr $434ce.l
pub fn i_084906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8490C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8490C);
    cpu.pc = 0x434CE;
    16
}
/// $08490C: lea.l $2e993c.l, a1
pub fn i_08490c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84912;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E993C;
    20
}
/// $084912: jsr $77c7e.l
pub fn i_084912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84918);
    cpu.pc = 0x77C7E;
    16
}
/// $084918: bset.b #$6, $12(a0)
pub fn i_084918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8491E;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08491E: subi.w #$40, $24(a0)
pub fn i_08491e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84924;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084924: lea.l $84932(pc), a1
pub fn i_084924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84928;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84932;
    16
}
/// $084928: move.l a1, (a6)
pub fn i_084928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8492A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08492a  ($08492A, 8 B, thunks/jsr_abs_thunks.c)
/// $08492A: jsr $28d70.l
pub fn i_08492a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84930;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84930);
    cpu.pc = 0x28D70;
    16
}
/// $084930: rts
pub fn i_084930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84932;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084932  ($084932, 44 B, enemies/squad/rescue_squad_084836.s)
/// $084932: jsr $8601c(pc)
pub fn i_084932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84936;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84936);
    cpu.pc = 0x8601C;
    16
}
/// $084936: lea.l $8493c(pc), a1
pub fn i_084936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8493A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8493C;
    16
}
/// $08493A: move.l a1, (a6)
pub fn i_08493a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8493C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08493C: jsr $863a0(pc)
pub fn i_08493c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84940;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84940);
    cpu.pc = 0x863A0;
    16
}
/// $084940: jsr $27cee.l
pub fn i_084940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84946);
    cpu.pc = 0x27CEE;
    16
}
/// $084946: jsr $28d70.l
pub fn i_084946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8494C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8494C);
    cpu.pc = 0x28D70;
    16
}
/// $08494C: cmpi.w #$40, $24(a6)
pub fn i_08494c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84952;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x40, Size::Word, false);
    16
}
/// $084952: bgt.w $8495c
pub fn i_084952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84956;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8495C; return 10; }
    8
}
/// $084956: jmp $518.l
pub fn i_084956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8495C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08495C: rts
pub fn i_08495c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8495E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008495E  ($08495E, 92 B, enemies/squad/rescue_squad_084836.s)
/// $08495E: move.w #$85, d1
pub fn i_08495e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84962;
    cpu.ir = 0x323C;
    let v1: u32 = 0x85;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084962: jsr $236e.l
pub fn i_084962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84968;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84968);
    cpu.pc = 0x236E;
    16
}
/// $084968: move.b #$ff, $32(a6)
pub fn i_084968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8496E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08496E: move.b #$ff, $33(a6)
pub fn i_08496e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84974;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084974: move.b #$0, $3a(a6)
pub fn i_084974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8497A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08497A: lea.l $2c0218.l, a0
pub fn i_08497a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84980;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0218;
    20
}
/// $084980: jsr $799de.l
pub fn i_084980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84986;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84986);
    cpu.pc = 0x799DE;
    16
}
/// $084986: move.w d0, $66(a6)
pub fn i_084986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8498A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08498A: move.w #$0, $38(a6)
pub fn i_08498a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84990;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084990: addi.w #$98, $22(a6)
pub fn i_084990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84996;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x98, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084996: subi.w #$8, $24(a6)
pub fn i_084996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8499C;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08499C: move.b #$0, $20(a6)
pub fn i_08499c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849A2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0849A2: lea.l $849a8(pc), a1
pub fn i_0849a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x849A8;
    16
}
/// $0849A6: move.l a1, (a6)
pub fn i_0849a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0849A8: jsr $2783a.l
pub fn i_0849a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849AE);
    cpu.pc = 0x2783A;
    16
}
/// $0849AE: cmpi.w #$150, $22(a6)
pub fn i_0849ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849B4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $0849B4: ble.w $849ba
pub fn i_0849b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849B8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x849BA; return 10; }
    8
}
/// $0849B8: rts
pub fn i_0849b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0849ba  ($0849BA, 240 B, enemies/squad/rescue_squad_084836.s)
/// $0849BA: lea.l $2e7566.l, a0
pub fn i_0849ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7566;
    20
}
/// $0849C0: jsr $28cd4.l
pub fn i_0849c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849C6);
    cpu.pc = 0x28CD4;
    16
}
/// $0849C6: lea.l $ffff.w, a0
pub fn i_0849c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849CA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0849CA: move.l a0, $48(a6)
pub fn i_0849ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849CE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0849CE: lea.l $84aaa(pc), a1
pub fn i_0849ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849D2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84AAA;
    16
}
/// $0849D2: jsr $4ae.l
pub fn i_0849d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849D8);
    cpu.pc = 0x4AE;
    16
}
/// $0849D8: jsr $5dd22.l
pub fn i_0849d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849DE);
    cpu.pc = 0x5DD22;
    16
}
/// $0849DE: lea.l $6aa14.l, a1
pub fn i_0849de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x6AA14;
    20
}
/// $0849E4: jsr $4ae.l
pub fn i_0849e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849EA);
    cpu.pc = 0x4AE;
    16
}
/// $0849EA: jsr $5dd22.l
pub fn i_0849ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x849F0);
    cpu.pc = 0x5DD22;
    16
}
/// $0849F0: addi.w #$65, $22(a0)
pub fn i_0849f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849F6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x65, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0849F6: move.b #$1, $3a(a0)
pub fn i_0849f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x849FC;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0849FC: move.b #$12, $98(a0)
pub fn i_0849fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A02;
    cpu.ir = 0x117C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084A02: move.l #$2eb06c, $60(a6)
pub fn i_084a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A0A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2EB06C;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084A0A: lea.l $84a10(pc), a1
pub fn i_084a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84A10;
    16
}
/// $084A0E: move.l a1, (a6)
pub fn i_084a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084A10: jsr $28998.l
pub fn i_084a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A16);
    cpu.pc = 0x28998;
    16
}
/// $084A16: jsr $2783a.l
pub fn i_084a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A1C);
    cpu.pc = 0x2783A;
    16
}
/// $084A1C: jsr $28d70.l
pub fn i_084a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A22);
    cpu.pc = 0x28D70;
    16
}
/// $084A22: jsr $2870a.l
pub fn i_084a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A28);
    cpu.pc = 0x2870A;
    16
}
/// $084A28: bcc.w $84a3e
pub fn i_084a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A2C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84A3E; return 10; }
    8
}
/// $084A2C: lea.l $5e766.l, a0
pub fn i_084a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084A32: jsr $5e770.l
pub fn i_084a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A38);
    cpu.pc = 0x5E770;
    16
}
/// $084A38: bclr.b #$3, $13(a6)
pub fn i_084a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A3E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084A3E: move.l $106f50.l, d0
pub fn i_084a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A44;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $084A44: swap d0
pub fn i_084a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A46;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084A46: cmpi.w #$4f0, d0
pub fn i_084a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A4A;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x4F0, Size::Word, false);
    8
}
/// $084A4A: blt.w $84a58
pub fn i_084a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A4E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x84A58; return 10; }
    8
}
/// $084A4E: lea.l $2e915a.l, a0
pub fn i_084a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E915A;
    20
}
/// $084A54: move.l a0, $48(a6)
pub fn i_084a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A58;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084A58: jsr $28758.l
pub fn i_084a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A5E);
    cpu.pc = 0x28758;
    16
}
/// $084A5E: bcc.w $84aa8
pub fn i_084a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84AA8; return 10; }
    8
}
/// $084A62: move.b #$ff, $20(a6)
pub fn i_084a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084A68: move.w #$1028, d0
pub fn i_084a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A6C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084A6C: jsr $2352.l
pub fn i_084a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A72);
    cpu.pc = 0x2352;
    16
}
/// $084A72: lea.l $2e98a2.l, a1
pub fn i_084a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A78;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E98A2;
    20
}
/// $084A78: jsr $77c7e.l
pub fn i_084a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A7E);
    cpu.pc = 0x77C7E;
    16
}
/// $084A7E: lea.l $2e98b4.l, a1
pub fn i_084a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A84;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E98B4;
    20
}
/// $084A84: jsr $77c7e.l
pub fn i_084a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A8A);
    cpu.pc = 0x77C7E;
    16
}
/// $084A8A: lea.l $2e9aee.l, a1
pub fn i_084a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A90;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9AEE;
    20
}
/// $084A90: jsr $77c7e.l
pub fn i_084a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84A96);
    cpu.pc = 0x77C7E;
    16
}
/// $084A96: lea.l $2eacc4.l, a1
pub fn i_084a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84A9C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EACC4;
    20
}
/// $084A9C: jsr $43fac.l
pub fn i_084a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AA2);
    cpu.pc = 0x43FAC;
    16
}
/// $084AA2: jmp $518.l
pub fn i_084aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AA8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084AA8: rts
pub fn i_084aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084aaa  ($084AAA, 114 B, enemies/squad/rescue_squad_084836.s)
/// $084AAA: move.w #$e, d1
pub fn i_084aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AAE;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084AAE: jsr $236e.l
pub fn i_084aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AB4);
    cpu.pc = 0x236E;
    16
}
/// $084AB4: move.b #$ff, $32(a6)
pub fn i_084ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ABA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084ABA: move.b #$ff, $33(a6)
pub fn i_084aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AC0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084AC0: move.b #$1, $3a(a6)
pub fn i_084ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084AC6: addi.w #$17, $22(a6)
pub fn i_084ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ACC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x17, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084ACC: addi.w #$28, $24(a6)
pub fn i_084acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AD2;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $084AD2: move.w #$0, $38(a6)
pub fn i_084ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AD8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084AD8: lea.l $2c10d8.l, a0
pub fn i_084ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ADE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C10D8;
    20
}
/// $084ADE: jsr $28cd4.l
pub fn i_084ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AE4);
    cpu.pc = 0x28CD4;
    16
}
/// $084AE4: lea.l $ffff.w, a0
pub fn i_084ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AE8;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084AE8: move.l a0, $48(a6)
pub fn i_084ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AEC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084AEC: lea.l $84af2(pc), a1
pub fn i_084aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84AF2;
    16
}
/// $084AF0: move.l a1, (a6)
pub fn i_084af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084AF2: jsr $2783a.l
pub fn i_084af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AF8);
    cpu.pc = 0x2783A;
    16
}
/// $084AF8: jsr $28d70.l
pub fn i_084af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84AFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84AFE);
    cpu.pc = 0x28D70;
    16
}
/// $084AFE: movea.l $c(a6), a0
pub fn i_084afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B02;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084B02: cmpi.b #$ff, $20(a0)
pub fn i_084b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B08;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $084B08: bne.w $84b22
pub fn i_084b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84B22; return 10; }
    8
}
/// $084B0C: lea.l $84c5e(pc), a1
pub fn i_084b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84C5E;
    16
}
/// $084B10: jsr $4ae.l
pub fn i_084b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B16);
    cpu.pc = 0x4AE;
    16
}
/// $084B16: jsr $5dd22.l
pub fn i_084b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B1C);
    cpu.pc = 0x5DD22;
    16
}
// ===== SetTaskHandler_084b1c  ($084B1C, 8 B, kernel/task_handlers.c)
/// $084B1C: lea.l $84b24(pc), a1
pub fn i_084b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B24;
    16
}
/// $084B20: move.l a1, (a6)
pub fn i_084b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084B22: rts
pub fn i_084b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084b24  ($084B24, 54 B, enemies/squad/rescue_squad_084836.s)
/// $084B24: jsr $267e2.l
pub fn i_084b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B2A);
    cpu.pc = 0x267E2;
    16
}
/// $084B2A: move.w #$ffe0, $2e(a6)
pub fn i_084b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084B30: lea.l $2b5b92.l, a0
pub fn i_084b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5B92;
    20
}
/// $084B36: jsr $28cd4.l
pub fn i_084b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B3C);
    cpu.pc = 0x28CD4;
    16
}
/// $084B3C: lea.l $ffff.w, a0
pub fn i_084b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B40;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084B40: move.l a0, $48(a6)
pub fn i_084b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B44;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084B44: lea.l $84b4a(pc), a1
pub fn i_084b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B4A;
    16
}
/// $084B48: move.l a1, (a6)
pub fn i_084b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084B4A: jsr $27cee.l
pub fn i_084b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B50);
    cpu.pc = 0x27CEE;
    16
}
/// $084B50: bcc.w $84b5a
pub fn i_084b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84B5A; return 10; }
    8
}
/// $084B54: lea.l $84b62(pc), a1
pub fn i_084b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B62;
    16
}
/// $084B58: move.l a1, (a6)
pub fn i_084b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_084b5a  ($084B5A, 8 B, thunks/jsr_abs_thunks.c)
/// $084B5A: jsr $28d70.l
pub fn i_084b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B60);
    cpu.pc = 0x28D70;
    16
}
/// $084B60: rts
pub fn i_084b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084b62  ($084B62, 48 B, enemies/squad/rescue_squad_084836.s)
/// $084B62: jsr $267e2.l
pub fn i_084b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B68);
    cpu.pc = 0x267E2;
    16
}
/// $084B68: lea.l $2b5c22.l, a0
pub fn i_084b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5C22;
    20
}
/// $084B6E: jsr $28cd4.l
pub fn i_084b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B74);
    cpu.pc = 0x28CD4;
    16
}
/// $084B74: lea.l $ffff.w, a0
pub fn i_084b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B78;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084B78: move.l a0, $48(a6)
pub fn i_084b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B7C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084B7C: lea.l $84b82(pc), a1
pub fn i_084b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B82;
    16
}
/// $084B80: move.l a1, (a6)
pub fn i_084b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084B82: jsr $2783a.l
pub fn i_084b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B88);
    cpu.pc = 0x2783A;
    16
}
/// $084B88: jsr $28d70.l
pub fn i_084b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84B8E);
    cpu.pc = 0x28D70;
    16
}
/// $084B8E: bcc.w $84b98
pub fn i_084b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84B98; return 10; }
    8
}
// ===== SetTaskHandler_084b92  ($084B92, 8 B, kernel/task_handlers.c)
/// $084B92: lea.l $84b9a(pc), a1
pub fn i_084b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B96;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84B9A;
    16
}
/// $084B96: move.l a1, (a6)
pub fn i_084b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B98;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084B98: rts
pub fn i_084b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84B9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084b9a  ($084B9A, 48 B, enemies/squad/rescue_squad_084836.s)
/// $084B9A: jsr $267e2.l
pub fn i_084b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BA0);
    cpu.pc = 0x267E2;
    16
}
/// $084BA0: lea.l $2b5cfa.l, a0
pub fn i_084ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B5CFA;
    20
}
/// $084BA6: jsr $28cd4.l
pub fn i_084ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BAC);
    cpu.pc = 0x28CD4;
    16
}
/// $084BAC: lea.l $ffff.w, a0
pub fn i_084bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BB0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084BB0: move.l a0, $48(a6)
pub fn i_084bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BB4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084BB4: lea.l $84bba(pc), a1
pub fn i_084bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BB8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84BBA;
    16
}
/// $084BB8: move.l a1, (a6)
pub fn i_084bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BBA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084BBA: jsr $2783a.l
pub fn i_084bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BC0);
    cpu.pc = 0x2783A;
    16
}
/// $084BC0: jsr $28d70.l
pub fn i_084bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BC6);
    cpu.pc = 0x28D70;
    16
}
/// $084BC6: bcc.w $84bd0
pub fn i_084bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84BD0; return 10; }
    8
}
// ===== SetTaskHandler_084bca  ($084BCA, 8 B, kernel/task_handlers.c)
/// $084BCA: lea.l $84bd2(pc), a1
pub fn i_084bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84BD2;
    16
}
/// $084BCE: move.l a1, (a6)
pub fn i_084bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084BD0: rts
pub fn i_084bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084bd2  ($084BD2, 76 B, enemies/squad/rescue_squad_084836.s)
/// $084BD2: jsr $267e2.l
pub fn i_084bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BD8);
    cpu.pc = 0x267E2;
    16
}
/// $084BD8: move.w #$fe80, $28(a6)
pub fn i_084bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFE80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084BDE: move.w #$27, $2a(a6)
pub fn i_084bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x27;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084BE4: lea.l $2b604a.l, a0
pub fn i_084be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B604A;
    20
}
/// $084BEA: jsr $28cd4.l
pub fn i_084bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84BF0);
    cpu.pc = 0x28CD4;
    16
}
/// $084BF0: lea.l $ffff.w, a0
pub fn i_084bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BF4;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084BF4: move.l a0, $48(a6)
pub fn i_084bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BF8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084BF8: lea.l $84bfe(pc), a1
pub fn i_084bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84BFE;
    16
}
/// $084BFC: move.l a1, (a6)
pub fn i_084bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84BFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084BFE: jsr $27cee.l
pub fn i_084bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C04);
    cpu.pc = 0x27CEE;
    16
}
/// $084C04: jsr $28d70.l
pub fn i_084c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C0A);
    cpu.pc = 0x28D70;
    16
}
/// $084C0A: cmpi.w #$84, $22(a6)
pub fn i_084c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C10;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x84, Size::Word, false);
    16
}
/// $084C10: bgt.w $84c24
pub fn i_084c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C14;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x84C24; return 10; }
    8
}
/// $084C14: move.w #$1040, d0
pub fn i_084c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C18;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1040;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084C18: jsr $2352.l
pub fn i_084c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C1E);
    cpu.pc = 0x2352;
    16
}
// ===== SetTaskHandler_084c1e  ($084C1E, 8 B, kernel/task_handlers.c)
/// $084C1E: lea.l $84c26(pc), a1
pub fn i_084c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84C26;
    16
}
/// $084C22: move.l a1, (a6)
pub fn i_084c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084C24: rts
pub fn i_084c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084c26  ($084C26, 42 B, enemies/squad/rescue_squad_084836.s)
/// $084C26: lea.l $84c2c(pc), a1
pub fn i_084c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84C2C;
    16
}
/// $084C2A: move.l a1, (a6)
pub fn i_084c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084C2C: jsr $27cee.l
pub fn i_084c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C32);
    cpu.pc = 0x27CEE;
    16
}
/// $084C32: jsr $28d70.l
pub fn i_084c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C38);
    cpu.pc = 0x28D70;
    16
}
/// $084C38: cmpi.w #$64, $22(a6)
pub fn i_084c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C3E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x64, Size::Word, false);
    16
}
/// $084C3E: bgt.w $84c5c
pub fn i_084c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C42;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x84C5C; return 10; }
    8
}
/// $084C42: movea.l $c(a6), a0
pub fn i_084c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C46;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084C46: movea.l $c(a0), a0
pub fn i_084c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C4A;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084C4A: move.b #$ff, $21(a0)
pub fn i_084c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C50;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== Jsr5B6ThenJmpScheduler_084c50  ($084C50, 14 B, kernel/jsr5b6_jmp518.c)
/// $084C50: jsr $5b6.l
pub fn i_084c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C56);
    cpu.pc = 0x5B6;
    16
}
/// $084C56: jmp $518.l
pub fn i_084c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C5C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084C5C: rts
pub fn i_084c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084c5e  ($084C5E, 62 B, enemies/squad/rescue_squad_084836.s)
/// $084C5E: move.w #$4, d1
pub fn i_084c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C62;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084C62: jsr $236e.l
pub fn i_084c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C68);
    cpu.pc = 0x236E;
    16
}
/// $084C68: move.b #$ff, $32(a6)
pub fn i_084c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084C6E: move.b #$ff, $33(a6)
pub fn i_084c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C74;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084C74: lea.l $2e75c0.l, a0
pub fn i_084c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C7A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E75C0;
    20
}
/// $084C7A: jsr $28cd4.l
pub fn i_084c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84C80);
    cpu.pc = 0x28CD4;
    16
}
/// $084C80: lea.l $84c86(pc), a1
pub fn i_084c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84C86;
    16
}
/// $084C84: move.l a1, (a6)
pub fn i_084c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084C86: movea.l $c(a6), a0
pub fn i_084c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C8A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084C8A: move.w $22(a0), $22(a6)
pub fn i_084c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C90;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $084C90: move.w $24(a0), $24(a6)
pub fn i_084c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C96;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $084C96: addi.w #$1e, $24(a6)
pub fn i_084c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84C9C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1E, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== JsrAbsThunk_084c9c  ($084C9C, 8 B, thunks/jsr_abs_thunks.c)
/// $084C9C: jsr $28d70.l
pub fn i_084c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CA2);
    cpu.pc = 0x28D70;
    16
}
/// $084CA2: rts
pub fn i_084ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084ca4  ($084CA4, 158 B, enemies/squad/rescue_squad_084836.s)
/// $084CA4: movea.l $3c(a6), a1
pub fn i_084ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CA8;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084CA8: jsr $2942a.l
pub fn i_084ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CAE);
    cpu.pc = 0x2942A;
    16
}
/// $084CAE: move.w #$20, $70(a6)
pub fn i_084cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084CB4: move.w #$a, $66(a6)
pub fn i_084cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CBA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084CBA: move.w $38(a6), d0
pub fn i_084cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CBE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $084CBE: move.b d0, $21(a6)
pub fn i_084cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CC2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084CC2: lea.l $2e9202.l, a0
pub fn i_084cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9202;
    20
}
/// $084CC8: move.l a0, $48(a6)
pub fn i_084cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CCC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084CCC: lea.l $84cd2(pc), a1
pub fn i_084ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84CD2;
    16
}
/// $084CD0: move.l a1, (a6)
pub fn i_084cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084CD2: jsr $2783a.l
pub fn i_084cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CD8);
    cpu.pc = 0x2783A;
    16
}
/// $084CD8: jsr $2870a.l
pub fn i_084cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CDE);
    cpu.pc = 0x2870A;
    16
}
/// $084CDE: bcc.w $84cf4
pub fn i_084cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CE2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84CF4; return 10; }
    8
}
/// $084CE2: lea.l $5e766.l, a0
pub fn i_084ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CE8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084CE8: jsr $5e770.l
pub fn i_084ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CEE);
    cpu.pc = 0x5E770;
    16
}
/// $084CEE: bclr.b #$3, $13(a6)
pub fn i_084cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CF4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084CF4: jsr $28758.l
pub fn i_084cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84CFA);
    cpu.pc = 0x28758;
    16
}
/// $084CFA: bcc.w $84d30
pub fn i_084cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84CFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84D30; return 10; }
    8
}
/// $084CFE: move.l #$2000, d0
pub fn i_084cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D04;
    cpu.ir = 0x203C;
    let v1: u32 = 0x2000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084D04: jsr $51a28.l
pub fn i_084d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D0A);
    cpu.pc = 0x51A28;
    16
}
/// $084D0A: move.w #$1027, d0
pub fn i_084d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084D0E: jsr $2352.l
pub fn i_084d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D14);
    cpu.pc = 0x2352;
    16
}
/// $084D14: lea.l $ffff.w, a0
pub fn i_084d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D18;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $084D18: move.l a0, $48(a6)
pub fn i_084d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D1C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084D1C: jsr $86504(pc)
pub fn i_084d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84D20);
    cpu.pc = 0x86504;
    16
}
/// $084D20: lea.l $2e9ab8.l, a1
pub fn i_084d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D26;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9AB8;
    20
}
/// $084D26: jsr $77c7e.l
pub fn i_084d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D2C);
    cpu.pc = 0x77C7E;
    16
}
/// $084D2C: bra.w $84d3a
pub fn i_084d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D30;
    cpu.ir = 0x6000;
    cpu.pc = 0x84D3A;
    10
}
/// $084D30: jsr $4fa70.l
pub fn i_084d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D36);
    cpu.pc = 0x4FA70;
    16
}
/// $084D36: bcc.w $84d40
pub fn i_084d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D3A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84D40; return 10; }
    8
}
/// $084D3A: jmp $518.l
pub fn i_084d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D40;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084D40: rts
pub fn i_084d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D42;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084d42  ($084D42, 110 B, enemies/squad/rescue_squad_084836.s)
/// $084D42: movea.l $3c(a6), a1
pub fn i_084d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D46;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084D46: jsr $2942a.l
pub fn i_084d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D4C);
    cpu.pc = 0x2942A;
    16
}
/// $084D4C: move.w #$a8, d1
pub fn i_084d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D50;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084D50: jsr $236e.l
pub fn i_084d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D56);
    cpu.pc = 0x236E;
    16
}
/// $084D56: move.w #$40, $70(a6)
pub fn i_084d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D5C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084D5C: move.b #$ff, $32(a6)
pub fn i_084d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D62;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084D62: move.b #$ff, $33(a6)
pub fn i_084d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D68;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084D68: move.b #$0, $3a(a6)
pub fn i_084d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D6E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084D6E: lea.l $2e77ba.l, a0
pub fn i_084d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E77BA;
    20
}
/// $084D74: jsr $28cd4.l
pub fn i_084d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D7A);
    cpu.pc = 0x28CD4;
    16
}
/// $084D7A: move.w #$8000, $38(a6)
pub fn i_084d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084D80: jsr $267e2.l
pub fn i_084d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D86);
    cpu.pc = 0x267E2;
    16
}
/// $084D86: jsr $27cee.l
pub fn i_084d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D8C);
    cpu.pc = 0x27CEE;
    16
}
/// $084D8C: lea.l $84d92(pc), a1
pub fn i_084d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84D92;
    16
}
/// $084D90: move.l a1, (a6)
pub fn i_084d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084D92: jsr $2783a.l
pub fn i_084d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D98);
    cpu.pc = 0x2783A;
    16
}
/// $084D98: jsr $28d70.l
pub fn i_084d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84D9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84D9E);
    cpu.pc = 0x28D70;
    16
}
/// $084D9E: jsr $4fa70.l
pub fn i_084d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84DA4);
    cpu.pc = 0x4FA70;
    16
}
/// $084DA4: bcc.w $84dae
pub fn i_084da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84DAE; return 10; }
    8
}
/// $084DA8: jmp $518.l
pub fn i_084da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DAE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084DAE: rts
pub fn i_084dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084db0  ($084DB0, 274 B, enemies/squad/rescue_squad_084836.s)
/// $084DB0: move.b #$ff, $32(a6)
pub fn i_084db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DB6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084DB6: move.b #$ff, $33(a6)
pub fn i_084db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084DBC: move.b #$0, $3a(a6)
pub fn i_084dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084DC2: cmpi.b #$0, $98(a6)
pub fn i_084dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DC8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $084DC8: bne.w $84dee
pub fn i_084dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DCC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84DEE; return 10; }
    8
}
/// $084DCC: lea.l $2e92aa.l, a0
pub fn i_084dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DD2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E92AA;
    20
}
/// $084DD2: move.l a0, $48(a6)
pub fn i_084dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DD6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084DD6: lea.l $2e9c12.l, a0
pub fn i_084dd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DDC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9C12;
    20
}
/// $084DDC: move.l a0, $70(a6)
pub fn i_084ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DE0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084DE0: lea.l $2e9838.l, a0
pub fn i_084de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DE6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9838;
    20
}
/// $084DE6: move.l a0, $74(a6)
pub fn i_084de6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DEA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084DEA: bra.w $84e38
pub fn i_084dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x84E38;
    10
}
/// $084DEE: cmpi.b #$1, $98(a6)
pub fn i_084dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DF4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $084DF4: bne.w $84e1a
pub fn i_084df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x84E1A; return 10; }
    8
}
/// $084DF8: lea.l $2e92fe.l, a0
pub fn i_084df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84DFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E92FE;
    20
}
/// $084DFE: move.l a0, $48(a6)
pub fn i_084dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E02;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E02: lea.l $2e9c24.l, a0
pub fn i_084e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E08;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9C24;
    20
}
/// $084E08: move.l a0, $70(a6)
pub fn i_084e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E0C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E0C: lea.l $2e9842.l, a0
pub fn i_084e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9842;
    20
}
/// $084E12: move.l a0, $74(a6)
pub fn i_084e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E16;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E16: bra.w $84e38
pub fn i_084e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x84E38;
    10
}
/// $084E1A: lea.l $2e9352.l, a0
pub fn i_084e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E20;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9352;
    20
}
/// $084E20: move.l a0, $48(a6)
pub fn i_084e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E24;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E24: lea.l $2e9c36.l, a0
pub fn i_084e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9C36;
    20
}
/// $084E2A: move.l a0, $70(a6)
pub fn i_084e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E2E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E2E: lea.l $2e984c.l, a0
pub fn i_084e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E984C;
    20
}
/// $084E34: move.l a0, $74(a6)
pub fn i_084e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E38;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $084E38: clr.w d0
pub fn i_084e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E3A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $084E3A: move.b $9a(a6), d0
pub fn i_084e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E3E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084E3E: move.w d0, $66(a6)
pub fn i_084e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E42;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084E42: jsr $267e2.l
pub fn i_084e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E48);
    cpu.pc = 0x267E2;
    16
}
/// $084E48: lea.l $2e6bf0.l, a0
pub fn i_084e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6BF0;
    20
}
/// $084E4E: jsr $28cd4.l
pub fn i_084e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E54);
    cpu.pc = 0x28CD4;
    16
}
/// $084E54: lea.l $84e5a(pc), a1
pub fn i_084e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84E5A;
    16
}
/// $084E58: move.l a1, (a6)
pub fn i_084e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084E5A: jsr $2783a.l
pub fn i_084e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E60);
    cpu.pc = 0x2783A;
    16
}
/// $084E60: jsr $28d70.l
pub fn i_084e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E66);
    cpu.pc = 0x28D70;
    16
}
/// $084E66: jsr $2870a.l
pub fn i_084e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E6C);
    cpu.pc = 0x2870A;
    16
}
/// $084E6C: bcc.w $84e82
pub fn i_084e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84E82; return 10; }
    8
}
/// $084E70: lea.l $5e766.l, a0
pub fn i_084e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E76;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $084E76: jsr $5e770.l
pub fn i_084e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E7C);
    cpu.pc = 0x5E770;
    16
}
/// $084E7C: bclr.b #$3, $13(a6)
pub fn i_084e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E82;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084E82: jsr $28758.l
pub fn i_084e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84E88);
    cpu.pc = 0x28758;
    16
}
/// $084E88: bcc.w $84eac
pub fn i_084e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E8C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84EAC; return 10; }
    8
}
/// $084E8C: cmpi.b #$0, $99(a6)
pub fn i_084e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E92;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $084E92: beq.w $84eba
pub fn i_084e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84EBA; return 10; }
    8
}
/// $084E96: movea.l $70(a6), a1
pub fn i_084e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84E9A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084E9A: jsr $77c7e.l
pub fn i_084e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84EA0);
    cpu.pc = 0x77C7E;
    16
}
/// $084EA0: subi.b #$1, $99(a6)
pub fn i_084ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EA6;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x99);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $084EA6: beq.w $84eba
pub fn i_084ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x84EBA; return 10; }
    8
}
/// $084EAA: bra.b $84e96
pub fn i_084eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EAC;
    cpu.ir = 0x60EA;
    cpu.pc = 0x84E96;
    10
}
/// $084EAC: movea.l $74(a6), a0
pub fn i_084eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EB0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $084EB0: jsr $5dd5c.l
pub fn i_084eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84EB6);
    cpu.pc = 0x5DD5C;
    16
}
/// $084EB6: bcc.w $84ec0
pub fn i_084eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84EC0; return 10; }
    8
}
/// $084EBA: jmp $518.l
pub fn i_084eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EC0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084EC0: rts
pub fn i_084ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084ec2  ($084EC2, 100 B, enemies/squad/rescue_squad_084836.s)
/// $084EC2: movea.l $3c(a6), a1
pub fn i_084ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EC6;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $084EC6: jsr $2942a.l
pub fn i_084ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ECC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84ECC);
    cpu.pc = 0x2942A;
    16
}
/// $084ECC: lea.l $2e7812.l, a0
pub fn i_084ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ED2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7812;
    20
}
/// $084ED2: jsr $28cd4.l
pub fn i_084ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84ED8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84ED8);
    cpu.pc = 0x28CD4;
    16
}
/// $084ED8: move.b #$ff, $32(a6)
pub fn i_084ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EDE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084EDE: move.b #$ff, $33(a6)
pub fn i_084ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EE4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084EE4: move.b #$0, $3a(a6)
pub fn i_084ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $084EEA: lea.l $84ef0(pc), a1
pub fn i_084eea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84EF0;
    16
}
/// $084EEE: move.l a1, (a6)
pub fn i_084eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084EF0: jsr $2783a.l
pub fn i_084ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84EF6);
    cpu.pc = 0x2783A;
    16
}
/// $084EF6: jsr $28d70.l
pub fn i_084ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84EFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84EFC);
    cpu.pc = 0x28D70;
    16
}
/// $084EFC: bclr.b #$3, $13(a6)
pub fn i_084efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F02;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084F02: bclr.b #$0, $13(a6)
pub fn i_084f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F08;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $084F08: move.w #$7fff, $66(a6)
pub fn i_084f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F0E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084F0E: lea.l $2e9856.l, a0
pub fn i_084f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F14;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9856;
    20
}
/// $084F14: jsr $5dd5c.l
pub fn i_084f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F1A);
    cpu.pc = 0x5DD5C;
    16
}
/// $084F1A: bcc.w $84f24
pub fn i_084f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84F24; return 10; }
    8
}
/// $084F1E: jmp $518.l
pub fn i_084f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F24;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084F24: rts
pub fn i_084f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084f26  ($084F26, 40 B, enemies/squad/rescue_squad_084836.s)
/// $084F26: move.w #$1b1, d1
pub fn i_084f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F2A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084F2A: jsr $236e.l
pub fn i_084f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F30);
    cpu.pc = 0x236E;
    16
}
/// $084F30: move.w #$0, $72(a6)
pub fn i_084f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F36;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084F36: move.w #$2000, $38(a6)
pub fn i_084f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F3C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084F3C: jsr $267e2.l
pub fn i_084f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F42);
    cpu.pc = 0x267E2;
    16
}
/// $084F42: jsr $85f44(pc)
pub fn i_084f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F46);
    cpu.pc = 0x85F44;
    16
}
/// $084F46: jsr $85f60(pc)
pub fn i_084f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F4A);
    cpu.pc = 0x85F60;
    16
}
/// $084F4A: bcc.w $84f56
pub fn i_084f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84F56; return 10; }
    8
}
// ===== SetTaskHandler_084f4e  ($084F4E, 8 B, kernel/task_handlers.c)
/// $084F4E: lea.l $84fca(pc), a1
pub fn i_084f4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84FCA;
    16
}
/// $084F52: move.l a1, (a6)
pub fn i_084f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084F54: rts
pub fn i_084f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_084f56  ($084F56, 8 B, kernel/task_handlers.c)
/// $084F56: lea.l $84f5e(pc), a1
pub fn i_084f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F5A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84F5E;
    16
}
/// $084F5A: move.l a1, (a6)
pub fn i_084f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F5C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084F5C: rts
pub fn i_084f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084f5e  ($084F5E, 108 B, enemies/squad/rescue_squad_084836.s)
/// $084F5E: move.w #$1074, d0
pub fn i_084f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F62;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1074;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084F62: jsr $2352.l
pub fn i_084f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F68);
    cpu.pc = 0x2352;
    16
}
/// $084F68: lea.l $2e7828.l, a0
pub fn i_084f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F6E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7828;
    20
}
/// $084F6E: jsr $28cd4.l
pub fn i_084f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F74);
    cpu.pc = 0x28CD4;
    16
}
/// $084F74: clr.w d0
pub fn i_084f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F76;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $084F76: move.b $9a(a6), d0
pub fn i_084f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F7A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084F7A: asl.w #$3, d0
pub fn i_084f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F7C;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $084F7C: move.w d0, $2a(a6)
pub fn i_084f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F80;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084F80: lea.l $84f86(pc), a1
pub fn i_084f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84F86;
    16
}
/// $084F84: move.l a1, (a6)
pub fn i_084f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084F86: jsr $27cee.l
pub fn i_084f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F8C);
    cpu.pc = 0x27CEE;
    16
}
/// $084F8C: jsr $85ee8(pc)
pub fn i_084f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F90;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F90);
    cpu.pc = 0x85EE8;
    16
}
/// $084F90: jsr $85f08(pc)
pub fn i_084f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F94);
    cpu.pc = 0x85F08;
    16
}
/// $084F94: jsr $28d70.l
pub fn i_084f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84F9A);
    cpu.pc = 0x28D70;
    16
}
/// $084F9A: jsr $85f44(pc)
pub fn i_084f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84F9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84F9E);
    cpu.pc = 0x85F44;
    16
}
/// $084F9E: jsr $85f60(pc)
pub fn i_084f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FA2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x84FA2);
    cpu.pc = 0x85F60;
    16
}
/// $084FA2: bcc.w $84fac
pub fn i_084fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84FAC; return 10; }
    8
}
/// $084FA6: lea.l $85062(pc), a1
pub fn i_084fa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FAA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85062;
    16
}
/// $084FAA: move.l a1, (a6)
pub fn i_084faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FAC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $084FAC: movea.l #$ffffffff, a0
pub fn i_084fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FB2;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $084FB2: lea.l $2e9860.l, a0
pub fn i_084fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9860;
    20
}
/// $084FB8: jsr $5dd5c.l
pub fn i_084fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84FBE);
    cpu.pc = 0x5DD5C;
    16
}
/// $084FBE: bcc.w $84fc8
pub fn i_084fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x84FC8; return 10; }
    8
}
/// $084FC2: jmp $518.l
pub fn i_084fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FC8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $084FC8: rts
pub fn i_084fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_084fca  ($084FCA, 152 B, enemies/squad/rescue_squad_084836.s)
/// $084FCA: move.w #$1074, d0
pub fn i_084fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FCE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1074;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $084FCE: jsr $2352.l
pub fn i_084fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84FD4);
    cpu.pc = 0x2352;
    16
}
/// $084FD4: lea.l $2e7866.l, a0
pub fn i_084fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7866;
    20
}
/// $084FDA: jsr $28cd4.l
pub fn i_084fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x84FE0);
    cpu.pc = 0x28CD4;
    16
}
/// $084FE0: clr.w d0
pub fn i_084fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FE2;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $084FE2: move.b $9a(a6), d0
pub fn i_084fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FE6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $084FE6: asl.w #$3, d0
pub fn i_084fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FE8;
    cpu.ir = 0xE740;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $084FE8: neg.w d0
pub fn i_084fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FEA;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $084FEA: move.w d0, $2a(a6)
pub fn i_084fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FEE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $084FEE: move.l $106f5c.l, d0
pub fn i_084fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FF4;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $084FF4: swap d0
pub fn i_084ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FF6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $084FF6: cmpi.w #$108, d0
pub fn i_084ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FFA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x108, Size::Word, false);
    8
}
/// $084FFA: bgt.w $85018
pub fn i_084ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x84FFE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x85018; return 10; }
    8
}
/// $084FFE: jsr $6f0.l
pub fn i_084ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85004;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85004);
    cpu.pc = 0x6F0;
    16
}
/// $085004: bcs.w $85018
pub fn i_085004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85008;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85018; return 10; }
    8
}
/// $085008: lea.l $8512c(pc), a1
pub fn i_085008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8500C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8512C;
    16
}
/// $08500C: jsr $4ae.l
pub fn i_08500c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85012;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85012);
    cpu.pc = 0x4AE;
    16
}
/// $085012: jsr $5dd02.l
pub fn i_085012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85018);
    cpu.pc = 0x5DD02;
    16
}
/// $085018: lea.l $8501e(pc), a1
pub fn i_085018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8501C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8501E;
    16
}
/// $08501C: move.l a1, (a6)
pub fn i_08501c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8501E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08501E: jsr $27cee.l
pub fn i_08501e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85024;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85024);
    cpu.pc = 0x27CEE;
    16
}
/// $085024: jsr $85ee8(pc)
pub fn i_085024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85028;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85028);
    cpu.pc = 0x85EE8;
    16
}
/// $085028: jsr $85f08(pc)
pub fn i_085028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8502C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8502C);
    cpu.pc = 0x85F08;
    16
}
/// $08502C: jsr $28d70.l
pub fn i_08502c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85032;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85032);
    cpu.pc = 0x28D70;
    16
}
/// $085032: jsr $85f44(pc)
pub fn i_085032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85036;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85036);
    cpu.pc = 0x85F44;
    16
}
/// $085036: jsr $85f60(pc)
pub fn i_085036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8503A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8503A);
    cpu.pc = 0x85F60;
    16
}
/// $08503A: bcs.w $85044
pub fn i_08503a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8503E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85044; return 10; }
    8
}
/// $08503E: lea.l $85062(pc), a1
pub fn i_08503e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85042;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85062;
    16
}
/// $085042: move.l a1, (a6)
pub fn i_085042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85044;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085044: movea.l #$ffffffff, a0
pub fn i_085044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8504A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08504A: lea.l $2e9860.l, a0
pub fn i_08504a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85050;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9860;
    20
}
/// $085050: jsr $5dd5c.l
pub fn i_085050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85056;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85056);
    cpu.pc = 0x5DD5C;
    16
}
/// $085056: bcc.w $85060
pub fn i_085056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8505A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85060; return 10; }
    8
}
/// $08505A: jmp $518.l
pub fn i_08505a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85060;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085060: rts
pub fn i_085060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85062;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_085062  ($085062, 100 B, enemies/squad/rescue_squad_084836.s)
/// $085062: move.w #$1075, d0
pub fn i_085062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85066;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085066: jsr $2352.l
pub fn i_085066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8506C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8506C);
    cpu.pc = 0x2352;
    16
}
/// $08506C: lea.l $2e78a4.l, a0
pub fn i_08506c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85072;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78A4;
    20
}
/// $085072: jsr $28cd4.l
pub fn i_085072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85078);
    cpu.pc = 0x28CD4;
    16
}
/// $085078: move.w #$a, $72(a6)
pub fn i_085078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8507E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08507E: lea.l $85084(pc), a1
pub fn i_08507e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85082;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85084;
    16
}
/// $085082: move.l a1, (a6)
pub fn i_085082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85084;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085084: jsr $2783a.l
pub fn i_085084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8508A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8508A);
    cpu.pc = 0x2783A;
    16
}
/// $08508A: jsr $85f08(pc)
pub fn i_08508a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8508E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8508E);
    cpu.pc = 0x85F08;
    16
}
/// $08508E: jsr $28d70.l
pub fn i_08508e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85094;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85094);
    cpu.pc = 0x28D70;
    16
}
/// $085094: jsr $85f44(pc)
pub fn i_085094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85098;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85098);
    cpu.pc = 0x85F44;
    16
}
/// $085098: cmpi.w #$0, $72(a6)
pub fn i_085098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8509E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08509E: bgt.w $850a8
pub fn i_08509e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850A2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x850A8; return 10; }
    8
}
/// $0850A2: lea.l $850c6(pc), a1
pub fn i_0850a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x850C6;
    16
}
/// $0850A6: move.l a1, (a6)
pub fn i_0850a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0850A8: movea.l #$ffffffff, a0
pub fn i_0850a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850AE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0850AE: lea.l $2e9860.l, a0
pub fn i_0850ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9860;
    20
}
/// $0850B4: jsr $5dd5c.l
pub fn i_0850b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850BA);
    cpu.pc = 0x5DD5C;
    16
}
/// $0850BA: bcc.w $850c4
pub fn i_0850ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850BE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x850C4; return 10; }
    8
}
/// $0850BE: jmp $518.l
pub fn i_0850be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850C4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0850C4: rts
pub fn i_0850c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0850c6  ($0850C6, 102 B, enemies/squad/rescue_squad_084836.s)
/// $0850C6: lea.l $2e78a4.l, a0
pub fn i_0850c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78A4;
    20
}
/// $0850CC: jsr $28cd4.l
pub fn i_0850cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850D2);
    cpu.pc = 0x28CD4;
    16
}
/// $0850D2: move.w #$14, $72(a6)
pub fn i_0850d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0850D8: lea.l $850de(pc), a1
pub fn i_0850d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x850DE;
    16
}
/// $0850DC: move.l a1, (a6)
pub fn i_0850dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0850DE: jsr $2783a.l
pub fn i_0850de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850E4);
    cpu.pc = 0x2783A;
    16
}
/// $0850E4: jsr $28d70.l
pub fn i_0850e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x850EA);
    cpu.pc = 0x28D70;
    16
}
/// $0850EA: jsr $85f44(pc)
pub fn i_0850ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x850EE);
    cpu.pc = 0x85F44;
    16
}
/// $0850EE: subq.w #$1, $72(a6)
pub fn i_0850ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850F2;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0850F2: bpl.w $8510e
pub fn i_0850f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850F6;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8510E; return 10; }
    8
}
/// $0850F6: jsr $85f60(pc)
pub fn i_0850f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x850FA);
    cpu.pc = 0x85F60;
    16
}
/// $0850FA: bcc.w $85108
pub fn i_0850fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x850FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85108; return 10; }
    8
}
/// $0850FE: lea.l $84fca(pc), a1
pub fn i_0850fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85102;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84FCA;
    16
}
/// $085102: move.l a1, (a6)
pub fn i_085102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85104;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085104: bra.w $8510e
pub fn i_085104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85108;
    cpu.ir = 0x6000;
    cpu.pc = 0x8510E;
    10
}
/// $085108: lea.l $84f5e(pc), a1
pub fn i_085108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8510C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84F5E;
    16
}
/// $08510C: move.l a1, (a6)
pub fn i_08510c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8510E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08510E: movea.l #$ffffffff, a0
pub fn i_08510e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85114;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $085114: lea.l $2e9860.l, a0
pub fn i_085114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8511A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9860;
    20
}
/// $08511A: jsr $5dd5c.l
pub fn i_08511a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85120);
    cpu.pc = 0x5DD5C;
    16
}
/// $085120: bcc.w $8512a
pub fn i_085120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85124;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8512A; return 10; }
    8
}
/// $085124: jmp $518.l
pub fn i_085124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8512A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08512A: rts
pub fn i_08512a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8512C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_08512c  ($08512C, 8 B, kernel/task_handlers.c)
/// $08512C: lea.l $85134(pc), a1
pub fn i_08512c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85130;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85134;
    16
}
/// $085130: move.l a1, (a6)
pub fn i_085130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85132;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085132: rts
pub fn i_085132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85134;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_PlatformSpawn_085134  ($085134, 90 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085134: lea.l $676ba.l, a1
pub fn i_085134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8513A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x676BA;
    20
}
/// $08513A: jsr $4ae.l
pub fn i_08513a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85140);
    cpu.pc = 0x4AE;
    16
}
/// $085140: jsr $5dd02.l
pub fn i_085140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85146);
    cpu.pc = 0x5DD02;
    16
}
/// $085146: move.w #$160, $22(a0)
pub fn i_085146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8514C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08514C: movem.l a0, -(a7)
pub fn i_08514c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85150;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $085150: move.w #$0, d0
pub fn i_085150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85154;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085154: move.w #$320, d1
pub fn i_085154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85158;
    cpu.ir = 0x323C;
    let v1: u32 = 0x320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085158: jsr $44022.l
pub fn i_085158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8515E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8515E);
    cpu.pc = 0x44022;
    16
}
/// $08515E: movem.l (a7)+, a0
pub fn i_08515e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85162;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $085162: move.w d1, $24(a0)
pub fn i_085162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85166;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085166: move.l #$0, $98(a0)
pub fn i_085166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8516E;
    cpu.ir = 0x217C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08516E: move.l #$0, $9c(a0)
pub fn i_08516e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85176;
    cpu.ir = 0x217C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085176: move.b #$0, $98(a0)
pub fn i_085176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8517C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08517C: move.b #$10, $99(a0)
pub fn i_08517c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85182;
    cpu.ir = 0x117C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085182: move.b #$1, $11(a0)
pub fn i_085182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85188;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085188: jmp $518.l
pub fn i_085188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8518E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08518e  ($08518E, 2 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08518E: rts
pub fn i_08518e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Carrier_Init_085190  ($085190, 88 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085190: move.w #$1c5, d1
pub fn i_085190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85194;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085194: jsr $236e.l
pub fn i_085194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8519A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8519A);
    cpu.pc = 0x236E;
    16
}
/// $08519A: lea.l $2c029a.l, a0
pub fn i_08519a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C029A;
    20
}
/// $0851A0: jsr $799de.l
pub fn i_0851a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851A6);
    cpu.pc = 0x799DE;
    16
}
/// $0851A6: move.w d0, $66(a6)
pub fn i_0851a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851AA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0851AA: lea.l $85394(pc), a1
pub fn i_0851aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85394;
    16
}
/// $0851AE: jsr $4ae.l
pub fn i_0851ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851B4);
    cpu.pc = 0x4AE;
    16
}
/// $0851B4: jsr $5dd22.l
pub fn i_0851b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851BA);
    cpu.pc = 0x5DD22;
    16
}
/// $0851BA: move.b #$0, $20(a6)
pub fn i_0851ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851C0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0851C0: move.b #$0, $21(a6)
pub fn i_0851c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851C6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0851C6: move.b #$0, $89(a6)
pub fn i_0851c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0851CC: lea.l $851d2(pc), a1
pub fn i_0851cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x851D2;
    16
}
/// $0851D0: move.l a1, (a6)
pub fn i_0851d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0851D2: move.l $106f5c.l, d0
pub fn i_0851d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851D8;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0851D8: swap d0
pub fn i_0851d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851DA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0851DA: cmpi.w #$1f0, d0
pub fn i_0851da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851DE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1F0, Size::Word, false);
    8
}
/// $0851DE: bgt.w $851f0
pub fn i_0851de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851E2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x851F0; return 10; }
    8
}
/// $0851E2: clr.b $10e39a.l
pub fn i_0851e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851E8;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
// ===== JsrAbsThunk_0851e8  ($0851E8, 8 B, thunks/jsr_abs_thunks.c)
/// $0851E8: jsr $2783a.l
pub fn i_0851e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851EE);
    cpu.pc = 0x2783A;
    16
}
/// $0851EE: rts
pub fn i_0851ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Carrier_Approach_0851f0  ($0851F0, 120 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0851F0: lea.l $85608(pc), a1
pub fn i_0851f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85608;
    16
}
/// $0851F4: jsr $4ae.l
pub fn i_0851f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x851FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x851FA);
    cpu.pc = 0x4AE;
    16
}
/// $0851FA: lea.l $2e93fa.l, a0
pub fn i_0851fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85200;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E93FA;
    20
}
/// $085200: move.l a0, $48(a6)
pub fn i_085200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85204;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085204: lea.l $2e78ce.l, a0
pub fn i_085204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8520A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78CE;
    20
}
/// $08520A: jsr $28cd4.l
pub fn i_08520a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85210;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85210);
    cpu.pc = 0x28CD4;
    16
}
/// $085210: lea.l $85216(pc), a1
pub fn i_085210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85214;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85216;
    16
}
/// $085214: move.l a1, (a6)
pub fn i_085214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85216;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085216: clr.b $10e39a.l
pub fn i_085216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8521C;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08521C: jsr $2783a.l
pub fn i_08521c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85222;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85222);
    cpu.pc = 0x2783A;
    16
}
/// $085222: move.b #$0, $89(a6)
pub fn i_085222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85228;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085228: jsr $2870a.l
pub fn i_085228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8522E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8522E);
    cpu.pc = 0x2870A;
    16
}
/// $08522E: bcc.w $85268
pub fn i_08522e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85232;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85268; return 10; }
    8
}
/// $085232: move.b #$ff, $89(a6)
pub fn i_085232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85238;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085238: lea.l $2e78b4.l, a0
pub fn i_085238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8523E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78B4;
    20
}
/// $08523E: jsr $28cd4.l
pub fn i_08523e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85244;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85244);
    cpu.pc = 0x28CD4;
    16
}
/// $085244: lea.l $5e766.l, a0
pub fn i_085244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8524A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08524A: jsr $5e798.l
pub fn i_08524a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85250;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85250);
    cpu.pc = 0x5E798;
    16
}
/// $085250: bclr.b #$3, $13(a6)
pub fn i_085250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85256;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $085256: lea.l $2e994e.l, a1
pub fn i_085256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8525C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E994E;
    20
}
/// $08525C: jsr $77c7e.l
pub fn i_08525c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85262;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85262);
    cpu.pc = 0x77C7E;
    16
}
/// $085262: lea.l $85270(pc), a1
pub fn i_085262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85266;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85270;
    16
}
/// $085266: move.l a1, (a6)
pub fn i_085266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85268;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_085268  ($085268, 8 B, thunks/jsr_abs_thunks.c)
/// $085268: jsr $28d70.l
pub fn i_085268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8526E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8526E);
    cpu.pc = 0x28D70;
    16
}
/// $08526E: rts
pub fn i_08526e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Carrier_Fight_085270  ($085270, 192 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085270: move.w #$102d, d0
pub fn i_085270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85274;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085274: jsr $2352.l
pub fn i_085274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8527A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8527A);
    cpu.pc = 0x2352;
    16
}
/// $08527A: lea.l $2ea858.l, a2
pub fn i_08527a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85280;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA858;
    20
}
/// $085280: jsr $5022a.l
pub fn i_085280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85286;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85286);
    cpu.pc = 0x5022A;
    16
}
/// $085286: lea.l $8528c(pc), a1
pub fn i_085286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8528A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8528C;
    16
}
/// $08528A: move.l a1, (a6)
pub fn i_08528a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8528C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08528C: clr.b $10e39a.l
pub fn i_08528c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85292;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $085292: jsr $2783a.l
pub fn i_085292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85298);
    cpu.pc = 0x2783A;
    16
}
/// $085298: move.b #$0, $89(a6)
pub fn i_085298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8529E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08529E: jsr $2870a.l
pub fn i_08529e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852A4);
    cpu.pc = 0x2870A;
    16
}
/// $0852A4: bcc.w $852cc
pub fn i_0852a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852A8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x852CC; return 10; }
    8
}
/// $0852A8: move.b #$ff, $89(a6)
pub fn i_0852a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x89);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0852AE: lea.l $2e78de.l, a0
pub fn i_0852ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E78DE;
    20
}
/// $0852B4: jsr $28cd4.l
pub fn i_0852b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0852BA: lea.l $5e766.l, a0
pub fn i_0852ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $0852C0: jsr $5e798.l
pub fn i_0852c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852C6);
    cpu.pc = 0x5E798;
    16
}
/// $0852C6: bclr.b #$3, $13(a6)
pub fn i_0852c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852CC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0852CC: jsr $28758.l
pub fn i_0852cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852D2);
    cpu.pc = 0x28758;
    16
}
/// $0852D2: bcc.w $85330
pub fn i_0852d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852D6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85330; return 10; }
    8
}
/// $0852D6: lea.l $ffff.w, a0
pub fn i_0852d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852DA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0852DA: move.l a0, $48(a6)
pub fn i_0852da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852DE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0852DE: move.b #$ff, $20(a6)
pub fn i_0852de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0852E4: lea.l $2eaf02.l, a1
pub fn i_0852e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAF02;
    20
}
/// $0852EA: jsr $43fac.l
pub fn i_0852ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852F0);
    cpu.pc = 0x43FAC;
    16
}
/// $0852F0: lea.l $2e9960.l, a1
pub fn i_0852f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9960;
    20
}
/// $0852F6: jsr $77c7e.l
pub fn i_0852f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x852FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x852FC);
    cpu.pc = 0x77C7E;
    16
}
/// $0852FC: lea.l $2e9972.l, a1
pub fn i_0852fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85302;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9972;
    20
}
/// $085302: jsr $77c7e.l
pub fn i_085302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85308);
    cpu.pc = 0x77C7E;
    16
}
/// $085308: move.b #$ff, $74(a6)
pub fn i_085308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8530E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08530E: lea.l $2e9790.l, a0
pub fn i_08530e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85314;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9790;
    20
}
/// $085314: move.l a0, $4c(a6)
pub fn i_085314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85318;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085318: jsr $283ca.l
pub fn i_085318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8531E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8531E);
    cpu.pc = 0x283CA;
    16
}
/// $08531E: jsr $283ca.l
pub fn i_08531e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85324);
    cpu.pc = 0x283CA;
    16
}
/// $085324: jsr $283d8.l
pub fn i_085324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8532A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8532A);
    cpu.pc = 0x283D8;
    16
}
/// $08532A: lea.l $85338(pc), a1
pub fn i_08532a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8532E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85338;
    16
}
/// $08532E: move.l a1, (a6)
pub fn i_08532e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85330;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_085330  ($085330, 8 B, thunks/jsr_abs_thunks.c)
/// $085330: jsr $28d70.l
pub fn i_085330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85336);
    cpu.pc = 0x28D70;
    16
}
/// $085336: rts
pub fn i_085336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Carrier_Wreck_085338  ($085338, 92 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085338: lea.l $ffff.w, a0
pub fn i_085338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8533C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08533C: move.l a0, $4c(a6)
pub fn i_08533c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85340;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085340: jsr $283ca.l
pub fn i_085340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85346;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85346);
    cpu.pc = 0x283CA;
    16
}
/// $085346: move.w #$1028, d0
pub fn i_085346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8534A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1028;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08534A: jsr $2352.l
pub fn i_08534a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85350;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85350);
    cpu.pc = 0x2352;
    16
}
/// $085350: jsr $85b7e(pc)
pub fn i_085350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85354;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85354);
    cpu.pc = 0x85B7E;
    16
}
/// $085354: move.w #$74, d0
pub fn i_085354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85358;
    cpu.ir = 0x303C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085358: move.b #$0, $74(a6)
pub fn i_085358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8535E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08535E: lea.l $ec6c8.l, a1
pub fn i_08535e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85364;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xEC6C8;
    20
}
/// $085364: jsr $4429e.l
pub fn i_085364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8536A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8536A);
    cpu.pc = 0x4429E;
    16
}
/// $08536A: lea.l $2ea86c.l, a2
pub fn i_08536a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85370;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EA86C;
    20
}
/// $085370: jsr $5022a.l
pub fn i_085370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85376);
    cpu.pc = 0x5022A;
    16
}
/// $085376: lea.l $8537c(pc), a1
pub fn i_085376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8537A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8537C;
    16
}
/// $08537A: move.l a1, (a6)
pub fn i_08537a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8537C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08537C: clr.b $10e39a.l
pub fn i_08537c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85382;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $085382: cmpi.b #$ff, $21(a6)
pub fn i_085382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85388;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $085388: bne.w $85392
pub fn i_085388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8538C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85392; return 10; }
    8
}
/// $08538C: jmp $518.l
pub fn i_08538c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85392;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085392: rts
pub fn i_085392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85394;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Init_085394  ($085394, 118 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085394: lea.l $856aa(pc), a1
pub fn i_085394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85398;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x856AA;
    16
}
/// $085398: jsr $4ae.l
pub fn i_085398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8539E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8539E);
    cpu.pc = 0x4AE;
    16
}
/// $08539E: movea.l $c(a6), a1
pub fn i_08539e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853A2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0853A2: move.w $22(a1), $22(a0)
pub fn i_0853a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853A8;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0853A8: move.w $24(a1), $24(a0)
pub fn i_0853a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853AE;
    cpu.ir = 0x3169;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0853AE: move.w #$100, d0
pub fn i_0853ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853B2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0853B2: move.w #$2f6, d1
pub fn i_0853b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2F6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0853B6: jsr $44022.l
pub fn i_0853b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x853BC);
    cpu.pc = 0x44022;
    16
}
/// $0853BC: move.w d0, $22(a6)
pub fn i_0853bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0853C0: move.w d1, $24(a6)
pub fn i_0853c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853C4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0853C4: move.w #$1c8, d1
pub fn i_0853c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0853C8: jsr $236e.l
pub fn i_0853c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x853CE);
    cpu.pc = 0x236E;
    16
}
/// $0853CE: lea.l $2e7908.l, a0
pub fn i_0853ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7908;
    20
}
/// $0853D4: jsr $28cd4.l
pub fn i_0853d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x853DA);
    cpu.pc = 0x28CD4;
    16
}
/// $0853DA: move.w #$100, $38(a6)
pub fn i_0853da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0853E0: move.l #$2eb0b8, $60(a6)
pub fn i_0853e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853E8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2EB0B8;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0853E8: lea.l $853ee(pc), a1
pub fn i_0853e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x853EE;
    16
}
/// $0853EC: move.l a1, (a6)
pub fn i_0853ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0853EE: move.l $106f5c.l, d0
pub fn i_0853ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853F4;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F5C);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $0853F4: swap d0
pub fn i_0853f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853F6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0853F6: cmpi.w #$1f0, d0
pub fn i_0853f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853FA;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1F0, Size::Word, false);
    8
}
/// $0853FA: bgt.w $85412
pub fn i_0853fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x853FE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x85412; return 10; }
    8
}
/// $0853FE: jsr $28998.l
pub fn i_0853fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85404;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85404);
    cpu.pc = 0x28998;
    16
}
/// $085404: jsr $2783a.l
pub fn i_085404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8540A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8540A);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08540a  ($08540A, 8 B, thunks/jsr_abs_thunks.c)
/// $08540A: jsr $28d70.l
pub fn i_08540a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85410);
    cpu.pc = 0x28D70;
    16
}
/// $085410: rts
pub fn i_085410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85412;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Active_085412  ($085412, 106 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085412: jsr $267e2.l
pub fn i_085412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85418;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85418);
    cpu.pc = 0x267E2;
    16
}
/// $085418: bclr.b #$1, $12(a6)
pub fn i_085418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8541E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08541E: lea.l $ffff.w, a0
pub fn i_08541e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85422;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $085422: move.l a0, $4c(a6)
pub fn i_085422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85426;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085426: jsr $283ca.l
pub fn i_085426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8542C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8542C);
    cpu.pc = 0x283CA;
    16
}
/// $08542C: lea.l $85526(pc), a1
pub fn i_08542c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85430;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85526;
    16
}
/// $085430: jsr $4ae.l
pub fn i_085430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85436);
    cpu.pc = 0x4AE;
    16
}
/// $085436: lea.l $8543c(pc), a1
pub fn i_085436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8543A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8543C;
    16
}
/// $08543A: move.l a1, (a6)
pub fn i_08543a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8543C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08543C: jsr $28998.l
pub fn i_08543c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85442;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85442);
    cpu.pc = 0x28998;
    16
}
/// $085442: jsr $27cee.l
pub fn i_085442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85448;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85448);
    cpu.pc = 0x27CEE;
    16
}
/// $085448: jsr $85d32(pc)
pub fn i_085448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8544C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8544C);
    cpu.pc = 0x85D32;
    16
}
/// $08544C: jsr $85ebc(pc)
pub fn i_08544c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85450;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85450);
    cpu.pc = 0x85EBC;
    16
}
/// $085450: jsr $28d70.l
pub fn i_085450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85456;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85456);
    cpu.pc = 0x28D70;
    16
}
/// $085456: jsr $85da8(pc)
pub fn i_085456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8545A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8545A);
    cpu.pc = 0x85DA8;
    16
}
/// $08545A: movea.l $c(a6), a0
pub fn i_08545a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8545E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08545E: cmpi.b #$ff, $20(a0)
pub fn i_08545e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85464;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $085464: bne.w $85482
pub fn i_085464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85468;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85482; return 10; }
    8
}
/// $085468: lea.l $ffff.w, a0
pub fn i_085468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8546C;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08546C: move.l a0, $48(a6)
pub fn i_08546c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85470;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085470: lea.l $2e9984.l, a1
pub fn i_085470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85476;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9984;
    20
}
/// $085476: jsr $77c7e.l
pub fn i_085476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8547C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8547C);
    cpu.pc = 0x77C7E;
    16
}
// ===== SetTaskHandler_08547c  ($08547C, 8 B, kernel/task_handlers.c)
/// $08547C: lea.l $85484(pc), a1
pub fn i_08547c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85480;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85484;
    16
}
/// $085480: move.l a1, (a6)
pub fn i_085480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85482;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085482: rts
pub fn i_085482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85484;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Death_085484  ($085484, 100 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085484: lea.l $ffff.w, a0
pub fn i_085484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85488;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $085488: move.l a0, $48(a6)
pub fn i_085488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8548C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08548C: move.w #$1033, d0
pub fn i_08548c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85490;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085490: jsr $2352.l
pub fn i_085490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85496;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85496);
    cpu.pc = 0x2352;
    16
}
/// $085496: lea.l $2e9996.l, a1
pub fn i_085496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8549C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E9996;
    20
}
/// $08549C: jsr $77c7e.l
pub fn i_08549c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854A2);
    cpu.pc = 0x77C7E;
    16
}
/// $0854A2: jsr $267e2.l
pub fn i_0854a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854A8);
    cpu.pc = 0x267E2;
    16
}
/// $0854A8: move.w #$ffe0, $2e(a6)
pub fn i_0854a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0854AE: lea.l $2e7940.l, a0
pub fn i_0854ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7940;
    20
}
/// $0854B4: jsr $28cd4.l
pub fn i_0854b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0854BA: lea.l $854c0(pc), a1
pub fn i_0854ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x854C0;
    16
}
/// $0854BE: move.l a1, (a6)
pub fn i_0854be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0854C0: jsr $27cee.l
pub fn i_0854c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854C6);
    cpu.pc = 0x27CEE;
    16
}
/// $0854C6: jsr $28d70.l
pub fn i_0854c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854CC);
    cpu.pc = 0x28D70;
    16
}
/// $0854CC: cmpi.w #$20, $24(a6)
pub fn i_0854cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $0854D2: bgt.w $854e6
pub fn i_0854d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854D6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x854E6; return 10; }
    8
}
/// $0854D6: movea.l $c(a6), a0
pub fn i_0854d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854DA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0854DA: move.b #$ff, $21(a0)
pub fn i_0854da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854E0;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0854E0: jmp $518.l
pub fn i_0854e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854E6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0854E6: rts
pub fn i_0854e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SmokePuff_0854e8  ($0854E8, 62 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0854E8: move.w #$2, $72(a6)
pub fn i_0854e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0854EE: lea.l $854f4(pc), a1
pub fn i_0854ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x854F4;
    16
}
/// $0854F2: move.l a1, (a6)
pub fn i_0854f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0854F4: jsr $2783a.l
pub fn i_0854f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x854FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x854FA);
    cpu.pc = 0x2783A;
    16
}
/// $0854FA: lea.l $2e96ec.l, a0
pub fn i_0854fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85500;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E96EC;
    20
}
/// $085500: move.l a0, $4c(a6)
pub fn i_085500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85504;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085504: jsr $283ca.l
pub fn i_085504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8550A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8550A);
    cpu.pc = 0x283CA;
    16
}
/// $08550A: jsr $283ca.l
pub fn i_08550a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85510);
    cpu.pc = 0x283CA;
    16
}
/// $085510: jsr $283d8.l
pub fn i_085510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85516;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85516);
    cpu.pc = 0x283D8;
    16
}
/// $085516: subq.w #$1, $72(a6)
pub fn i_085516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8551A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08551A: bpl.w $85524
pub fn i_08551a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8551E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x85524; return 10; }
    8
}
/// $08551E: jmp $518.l
pub fn i_08551e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85524;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085524: rts
pub fn i_085524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85526;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_TurretMount_085526  ($085526, 94 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085526: lea.l $2e7a10.l, a0
pub fn i_085526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8552C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7A10;
    20
}
/// $08552C: jsr $28cd4.l
pub fn i_08552c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85532;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85532);
    cpu.pc = 0x28CD4;
    16
}
/// $085532: lea.l $85538(pc), a1
pub fn i_085532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85536;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85538;
    16
}
/// $085536: move.l a1, (a6)
pub fn i_085536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85538;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085538: movea.l $c(a6), a0
pub fn i_085538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8553C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08553C: move.w $22(a0), $22(a6)
pub fn i_08553c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85542;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $085542: move.w $24(a0), $24(a6)
pub fn i_085542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85548;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $085548: jsr $28d70.l
pub fn i_085548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8554E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8554E);
    cpu.pc = 0x28D70;
    16
}
/// $08554E: jsr $2870a.l
pub fn i_08554e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85554;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85554);
    cpu.pc = 0x2870A;
    16
}
/// $085554: bcc.w $8556a
pub fn i_085554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85558;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8556A; return 10; }
    8
}
/// $085558: lea.l $5e766.l, a0
pub fn i_085558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8555E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08555E: jsr $5e798.l
pub fn i_08555e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85564;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85564);
    cpu.pc = 0x5E798;
    16
}
/// $085564: bclr.b #$3, $13(a6)
pub fn i_085564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8556A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08556A: movea.l $c(a6), a0
pub fn i_08556a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8556E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08556E: movea.l $c(a0), a0
pub fn i_08556e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85572;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085572: cmpi.b #$ff, $21(a0)
pub fn i_085572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85578;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $085578: bne.w $85582
pub fn i_085578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8557C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85582; return 10; }
    8
}
/// $08557C: jmp $518.l
pub fn i_08557c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85582;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085582: rts
pub fn i_085582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85584;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SoldierDropper_085584  ($085584, 126 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085584: move.w #$180, d0
pub fn i_085584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85588;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085588: move.w #$258, d1
pub fn i_085588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8558C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08558C: jsr $44022.l
pub fn i_08558c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85592;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85592);
    cpu.pc = 0x44022;
    16
}
/// $085592: move.w d0, $22(a6)
pub fn i_085592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85596;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085596: move.w d1, $24(a6)
pub fn i_085596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8559A;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08559A: move.w #$0, $72(a6)
pub fn i_08559a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0855A0: lea.l $855a6(pc), a1
pub fn i_0855a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x855A6;
    16
}
/// $0855A4: move.l a1, (a6)
pub fn i_0855a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0855A6: jsr $2783a.l
pub fn i_0855a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x855AC);
    cpu.pc = 0x2783A;
    16
}
/// $0855AC: movea.l $c(a6), a0
pub fn i_0855ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855B0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0855B0: cmpi.b #$ff, $20(a0)
pub fn i_0855b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855B6;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $0855B6: bne.w $855c2
pub fn i_0855b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855BA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x855C2; return 10; }
    8
}
/// $0855BA: jmp $518.l
pub fn i_0855ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855C0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0855C2: jsr $6f0.l
pub fn i_0855c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x855C8);
    cpu.pc = 0x6F0;
    16
}
/// $0855C8: bcs.w $85606
pub fn i_0855c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855CC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85606; return 10; }
    8
}
/// $0855CC: cmpi.w #$0, $72(a6)
pub fn i_0855cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855D2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0855D2: ble.w $855de
pub fn i_0855d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855D6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x855DE; return 10; }
    8
}
/// $0855D6: subq.w #$1, $72(a6)
pub fn i_0855d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855DA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0855DA: bra.w $85606
pub fn i_0855da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x85606;
    10
}
/// $0855DE: lea.l $483e2.l, a1
pub fn i_0855de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x483E2;
    20
}
/// $0855E4: jsr $4ae.l
pub fn i_0855e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x855EA);
    cpu.pc = 0x4AE;
    16
}
/// $0855EA: jsr $5dd22.l
pub fn i_0855ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x855F0);
    cpu.pc = 0x5DD22;
    16
}
/// $0855F0: move.b #$0, $11(a0)
pub fn i_0855f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855F6;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0855F6: lea.l $2c0420.l, a0
pub fn i_0855f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x855FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0420;
    20
}
/// $0855FC: jsr $799de.l
pub fn i_0855fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85602;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85602);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_085602  ($085602, 6 B, kernel/task_setters_w.c)
/// $085602: move.w d0, $72(a6)
pub fn i_085602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85606;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085606: rts
pub fn i_085606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85608;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SoldierDropperB_085608  ($085608, 156 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085608: move.w #$180, d0
pub fn i_085608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8560C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08560C: move.w #$2ff, d1
pub fn i_08560c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85610;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085610: jsr $44022.l
pub fn i_085610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85616;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85616);
    cpu.pc = 0x44022;
    16
}
/// $085616: move.w d0, $22(a6)
pub fn i_085616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8561A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08561A: move.w d1, $24(a6)
pub fn i_08561a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8561E;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08561E: move.w #$0, $72(a6)
pub fn i_08561e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85624;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085624: lea.l $8562a(pc), a1
pub fn i_085624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85628;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8562A;
    16
}
/// $085628: move.l a1, (a6)
pub fn i_085628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8562A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08562A: jsr $2783a.l
pub fn i_08562a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85630);
    cpu.pc = 0x2783A;
    16
}
/// $085630: movea.l $c(a6), a0
pub fn i_085630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85634;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085634: cmpi.b #$ff, $20(a0)
pub fn i_085634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8563A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08563A: bne.w $85646
pub fn i_08563a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8563E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85646; return 10; }
    8
}
/// $08563E: jmp $518.l
pub fn i_08563e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85644;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085646: jsr $6f0.l
pub fn i_085646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8564C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8564C);
    cpu.pc = 0x6F0;
    16
}
/// $08564C: bcs.w $856a8
pub fn i_08564c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85650;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x856A8; return 10; }
    8
}
/// $085650: cmpi.w #$0, $72(a6)
pub fn i_085650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85656;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $085656: ble.w $85662
pub fn i_085656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8565A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85662; return 10; }
    8
}
/// $08565A: subq.w #$1, $72(a6)
pub fn i_08565a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8565E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08565E: bra.w $856a8
pub fn i_08565e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85662;
    cpu.ir = 0x6000;
    cpu.pc = 0x856A8;
    10
}
/// $085662: lea.l $483e2.l, a1
pub fn i_085662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85668;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x483E2;
    20
}
/// $085668: jsr $4ae.l
pub fn i_085668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8566E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8566E);
    cpu.pc = 0x4AE;
    16
}
/// $08566E: jsr $5dd22.l
pub fn i_08566e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85674;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85674);
    cpu.pc = 0x5DD22;
    16
}
/// $085674: move.b #$0, $11(a0)
pub fn i_085674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8567A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08567A: move.b #$f, $98(a0)
pub fn i_08567a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85680;
    cpu.ir = 0x117C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085680: move.b #$0, $99(a0)
pub fn i_085680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85686;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085686: move.b #$0, $9a(a0)
pub fn i_085686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8568C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08568C: move.b #$1, $9b(a0)
pub fn i_08568c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85692;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085692: move.b #$1e, $9c(a0)
pub fn i_085692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85698;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x9C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085698: lea.l $2c0420.l, a0
pub fn i_085698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8569E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C0420;
    20
}
/// $08569E: jsr $799de.l
pub fn i_08569e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856A4);
    cpu.pc = 0x799DE;
    16
}
// ===== SetTaskW_0856a4  ($0856A4, 6 B, kernel/task_setters_w.c)
/// $0856A4: move.w d0, $72(a6)
pub fn i_0856a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856A8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0856A8: rts
pub fn i_0856a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Wheel_0856aa  ($0856AA, 144 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0856AA: move.w #$1d0, d1
pub fn i_0856aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856AE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0856AE: jsr $236e.l
pub fn i_0856ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856B4);
    cpu.pc = 0x236E;
    16
}
/// $0856B4: move.w #$1d1, d1
pub fn i_0856b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856B8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1D1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0856B8: jsr $236e.l
pub fn i_0856b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856BE);
    cpu.pc = 0x236E;
    16
}
/// $0856BE: addi.w #$30, $22(a6)
pub fn i_0856be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856C4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0856C4: addi.w #$61, $24(a6)
pub fn i_0856c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856CA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x61, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0856CA: clr.l d0
pub fn i_0856ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856CC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0856CC: move.b d0, $88(a6)
pub fn i_0856cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856D0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0856D0: movea.l #$2e7bde, a0
pub fn i_0856d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856D6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7BDE;
    cpu.a[0] = v1;
    4
}
/// $0856D6: lsl.w #$2, d0
pub fn i_0856d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856D8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0856D8: movea.l (a0, d0.w), a0
pub fn i_0856d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856DC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0856DC: cmpa.l #$ffffffff, a0
pub fn i_0856dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856E2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0856E2: beq.w $856ec
pub fn i_0856e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x856EC; return 10; }
    8
}
/// $0856E6: jsr $28cd4.l
pub fn i_0856e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856EC);
    cpu.pc = 0x28CD4;
    16
}
/// $0856EC: move.w #$0, $38(a6)
pub fn i_0856ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856F2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0856F2: lea.l $856f8(pc), a1
pub fn i_0856f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x856F8;
    16
}
/// $0856F6: move.l a1, (a6)
pub fn i_0856f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0856F8: jsr $2783a.l
pub fn i_0856f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x856FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x856FE);
    cpu.pc = 0x2783A;
    16
}
/// $0856FE: move.w $16(a6), $14(a6)
pub fn i_0856fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85704;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $085704: movea.l $c(a6), a0
pub fn i_085704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85708;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085708: movea.l $c(a0), a0
pub fn i_085708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8570C;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08570C: cmpi.b #$ff, $89(a0)
pub fn i_08570c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85712;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x89);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $085712: bne.w $8571c
pub fn i_085712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85716;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8571C; return 10; }
    8
}
/// $085716: move.w $18(a6), $14(a6)
pub fn i_085716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8571C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08571C: jsr $85c32(pc)
pub fn i_08571c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85720;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85720);
    cpu.pc = 0x85C32;
    16
}
/// $085720: movea.l $c(a6), a0
pub fn i_085720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85724;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085724: movea.l $c(a0), a0
pub fn i_085724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85728;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085728: cmpi.b #$ff, $20(a0)
pub fn i_085728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8572E;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08572E: bne.w $85738
pub fn i_08572e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85732;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85738; return 10; }
    8
}
/// $085732: jmp $518.l
pub fn i_085732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85738;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085738: rts
pub fn i_085738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8573A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SplashFx_08573a  ($08573A, 64 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08573A: move.w #$1b3, d1
pub fn i_08573a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8573E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08573E: jsr $236e.l
pub fn i_08573e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85744;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85744);
    cpu.pc = 0x236E;
    16
}
/// $085744: lea.l $2e7950.l, a0
pub fn i_085744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8574A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7950;
    20
}
/// $08574A: jsr $28cd4.l
pub fn i_08574a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85750;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85750);
    cpu.pc = 0x28CD4;
    16
}
/// $085750: move.w #$0, $38(a6)
pub fn i_085750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85756;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085756: jsr $267e2.l
pub fn i_085756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8575C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8575C);
    cpu.pc = 0x267E2;
    16
}
/// $08575C: lea.l $85762(pc), a1
pub fn i_08575c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85760;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85762;
    16
}
/// $085760: move.l a1, (a6)
pub fn i_085760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85762;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085762: jsr $2783a.l
pub fn i_085762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85768;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85768);
    cpu.pc = 0x2783A;
    16
}
/// $085768: jsr $28d70.l
pub fn i_085768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8576E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8576E);
    cpu.pc = 0x28D70;
    16
}
/// $08576E: bcc.w $85778
pub fn i_08576e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85772;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85778; return 10; }
    8
}
/// $085772: jmp $518.l
pub fn i_085772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85778;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085778: rts
pub fn i_085778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8577A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Rail_SpawnRow_08577a  ($08577A, 56 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08577A: clr.b d0
pub fn i_08577a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8577C;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $08577C: movem.w d0, -(a7)
pub fn i_08577c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85780;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $085780: lea.l $857b4(pc), a1
pub fn i_085780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85784;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x857B4;
    16
}
/// $085784: jsr $4ae.l
pub fn i_085784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8578A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8578A);
    cpu.pc = 0x4AE;
    16
}
/// $08578A: jsr $5dd22.l
pub fn i_08578a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85790;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85790);
    cpu.pc = 0x5DD22;
    16
}
/// $085790: movem.w (a7)+, d0
pub fn i_085790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85794;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $085794: move.b d0, $21(a0)
pub fn i_085794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85798;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085798: addi.w #$10, $22(a6)
pub fn i_085798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8579E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08579E: addq.b #$1, d0
pub fn i_08579e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857A0;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0857A0: cmpi.b #$14, d0
pub fn i_0857a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857A4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x14, Size::Byte, false);
    8
}
/// $0857A4: blt.b $8577c
pub fn i_0857a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857A6;
    cpu.ir = 0x6DD6;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8577C; return 10; }
    8
}
/// $0857A6: lea.l $857ac(pc), a1
pub fn i_0857a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x857AC;
    16
}
/// $0857AA: move.l a1, (a6)
pub fn i_0857aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0857AC: jmp $518.l
pub fn i_0857ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857B2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_0857b2  ($0857B2, 2 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0857B2: rts
pub fn i_0857b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Rail_Segment_0857b4  ($0857B4, 62 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0857B4: lea.l $857ba(pc), a1
pub fn i_0857b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x857BA;
    16
}
/// $0857B8: move.l a1, (a6)
pub fn i_0857b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0857BA: jsr $2783a.l
pub fn i_0857ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x857C0);
    cpu.pc = 0x2783A;
    16
}
/// $0857C0: movea.l #$ffffffff, a0
pub fn i_0857c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857C6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0857C6: lea.l $2e986a.l, a0
pub fn i_0857c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E986A;
    20
}
/// $0857CC: jsr $5dd5c.l
pub fn i_0857cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x857D2);
    cpu.pc = 0x5DD5C;
    16
}
/// $0857D2: bcs.w $857ea
pub fn i_0857d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857D6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x857EA; return 10; }
    8
}
/// $0857D6: lea.l $2eb104.l, a0
pub fn i_0857d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB104;
    20
}
/// $0857DC: jsr $5e086.l
pub fn i_0857dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x857E2);
    cpu.pc = 0x5E086;
    16
}
/// $0857E2: bcs.w $857f0
pub fn i_0857e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857E6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x857F0; return 10; }
    8
}
/// $0857E6: jsr $85d04(pc)
pub fn i_0857e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x857EA);
    cpu.pc = 0x85D04;
    16
}
/// $0857EA: jmp $518.l
pub fn i_0857ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0857F0: rts
pub fn i_0857f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Water_Spawn2_0857f2  ($0857F2, 44 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0857F2: lea.l $85820(pc), a1
pub fn i_0857f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85820;
    16
}
/// $0857F6: jsr $4ae.l
pub fn i_0857f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x857FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x857FC);
    cpu.pc = 0x4AE;
    16
}
/// $0857FC: jsr $5dd22.l
pub fn i_0857fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85802;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85802);
    cpu.pc = 0x5DD22;
    16
}
/// $085802: lea.l $8582a(pc), a1
pub fn i_085802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85806;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8582A;
    16
}
/// $085806: jsr $4ae.l
pub fn i_085806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8580C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8580C);
    cpu.pc = 0x4AE;
    16
}
/// $08580C: jsr $5dd22.l
pub fn i_08580c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85812;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85812);
    cpu.pc = 0x5DD22;
    16
}
/// $085812: lea.l $85818(pc), a1
pub fn i_085812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85818;
    16
}
/// $085816: move.l a1, (a6)
pub fn i_085816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85818;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085818: jmp $518.l
pub fn i_085818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8581E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08581e  ($08581E, 2 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08581E: rts
pub fn i_08581e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85820;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Water_InitP1_085820  ($085820, 10 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085820: move.b #$0, $20(a6)
pub fn i_085820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85826;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085826: bra.w $85830
pub fn i_085826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8582A;
    cpu.ir = 0x6000;
    cpu.pc = 0x85830;
    10
}
// ===== M4_Water_InitP2_08582a  ($08582A, 166 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08582A: move.b #$1, $20(a6)
pub fn i_08582a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85830;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085830: lea.l $2e79fa.l, a0
pub fn i_085830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85836;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E79FA;
    20
}
/// $085836: jsr $28cd4.l
pub fn i_085836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8583C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8583C);
    cpu.pc = 0x28CD4;
    16
}
/// $08583C: lea.l $85842(pc), a1
pub fn i_08583c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85840;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85842;
    16
}
/// $085840: move.l a1, (a6)
pub fn i_085840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85842;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085842: jsr $2783a.l
pub fn i_085842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85848;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85848);
    cpu.pc = 0x2783A;
    16
}
/// $085848: jsr $28d70.l
pub fn i_085848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8584E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8584E);
    cpu.pc = 0x28D70;
    16
}
/// $08584E: movea.l #$ffffffff, a0
pub fn i_08584e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85854;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $085854: lea.l $2e986a.l, a0
pub fn i_085854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8585A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E986A;
    20
}
/// $08585A: jsr $5dd5c.l
pub fn i_08585a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85860);
    cpu.pc = 0x5DD5C;
    16
}
/// $085860: bcc.w $8586c
pub fn i_085860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85864;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8586C; return 10; }
    8
}
/// $085864: jmp $518.l
pub fn i_085864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8586A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08586C: jsr $6f0.l
pub fn i_08586c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85872;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85872);
    cpu.pc = 0x6F0;
    16
}
/// $085872: bcs.w $858ce
pub fn i_085872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85876;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x858CE; return 10; }
    8
}
/// $085876: lea.l $100440.l, a0
pub fn i_085876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8587C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $08587C: movea.l a0, a4
pub fn i_08587c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8587E;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $08587E: cmpi.b #$0, $20(a6)
pub fn i_08587e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85884;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $085884: beq.w $85890
pub fn i_085884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85888;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85890; return 10; }
    8
}
/// $085888: lea.l $1004e0.l, a0
pub fn i_085888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8588E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $08588E: movea.l a0, a4
pub fn i_08588e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85890;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $085890: lea.l $2eb10e.l, a1
pub fn i_085890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85896;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EB10E;
    20
}
/// $085896: jsr $5e260.l
pub fn i_085896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8589C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8589C);
    cpu.pc = 0x5E260;
    16
}
/// $08589C: bcs.w $858ce
pub fn i_08589c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858A0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x858CE; return 10; }
    8
}
/// $0858A0: cmpi.w #$0, $28(a4)
pub fn i_0858a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858A6;
    cpu.ir = 0xC6C;
    let ad2 = cpu.a[4].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0858A6: beq.w $858ce
pub fn i_0858a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x858CE; return 10; }
    8
}
/// $0858AA: movem.l a4, -(a7)
pub fn i_0858aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858AE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0858AE: lea.l $8573a(pc), a1
pub fn i_0858ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8573A;
    16
}
/// $0858B2: jsr $4ae.l
pub fn i_0858b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858B8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x858B8);
    cpu.pc = 0x4AE;
    16
}
/// $0858B8: movem.l (a7)+, a4
pub fn i_0858b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858BC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0858BC: move.w $22(a4), $22(a0)
pub fn i_0858bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858C2;
    cpu.ir = 0x316C;
    let ad2 = cpu.a[4].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0858C2: move.w $24(a4), d0
pub fn i_0858c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858C6;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0858C6: subi.w #$8, d0
pub fn i_0858c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858CA;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0858CA: move.w d0, $24(a0)
pub fn i_0858ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858CE;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0858CE: rts
pub fn i_0858ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Debris_Init_0858d0  ($0858D0, 34 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0858D0: move.w #$1c9, d1
pub fn i_0858d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1C9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0858D4: jsr $236e.l
pub fn i_0858d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x858DA);
    cpu.pc = 0x236E;
    16
}
/// $0858DA: move.w #$0, $38(a6)
pub fn i_0858da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0858E0: jsr $267e2.l
pub fn i_0858e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x858E6);
    cpu.pc = 0x267E2;
    16
}
/// $0858E6: move.b $99(a6), d0
pub fn i_0858e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858EA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0858EA: andi.b #$1, d0
pub fn i_0858ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858EE;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0858EE: move.b d0, $3a(a6)
pub fn i_0858ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858F2;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== M4_Debris_Idle_0858f2  ($0858F2, 108 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0858F2: clr.l d0
pub fn i_0858f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858F4;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $0858F4: move.b $98(a6), d0
pub fn i_0858f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858F8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0858F8: andi.b #$3, d0
pub fn i_0858f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x858FC;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0858FC: movea.l #$2e7b2e, a0
pub fn i_0858fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85902;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7B2E;
    cpu.a[0] = v1;
    4
}
/// $085902: lsl.w #$2, d0
pub fn i_085902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85904;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085904: movea.l (a0, d0.w), a0
pub fn i_085904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85908;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085908: cmpa.l #$ffffffff, a0
pub fn i_085908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8590E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08590E: beq.w $85918
pub fn i_08590e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85912;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85918; return 10; }
    8
}
/// $085912: jsr $28cd4.l
pub fn i_085912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85918);
    cpu.pc = 0x28CD4;
    16
}
/// $085918: lea.l $8591e(pc), a1
pub fn i_085918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8591C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8591E;
    16
}
/// $08591C: move.l a1, (a6)
pub fn i_08591c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8591E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08591E: jsr $2783a.l
pub fn i_08591e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85924;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85924);
    cpu.pc = 0x2783A;
    16
}
/// $085924: jsr $28d70.l
pub fn i_085924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8592A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8592A);
    cpu.pc = 0x28D70;
    16
}
/// $08592A: lea.l $2eb118.l, a0
pub fn i_08592a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85930;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB118;
    20
}
/// $085930: jsr $5e086.l
pub fn i_085930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85936);
    cpu.pc = 0x5E086;
    16
}
/// $085936: bcs.w $85940
pub fn i_085936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8593A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85940; return 10; }
    8
}
/// $08593A: lea.l $8595e(pc), a1
pub fn i_08593a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8593E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8595E;
    16
}
/// $08593E: move.l a1, (a6)
pub fn i_08593e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85940;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085940: movea.l #$ffffffff, a0
pub fn i_085940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85946;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $085946: lea.l $2e9874.l, a0
pub fn i_085946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8594C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9874;
    20
}
/// $08594C: jsr $5dd5c.l
pub fn i_08594c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85952);
    cpu.pc = 0x5DD5C;
    16
}
/// $085952: bcc.w $8595c
pub fn i_085952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85956;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8595C; return 10; }
    8
}
/// $085956: jmp $518.l
pub fn i_085956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8595C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08595C: rts
pub fn i_08595c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8595E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Debris_Hit_08595e  ($08595E, 118 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08595E: clr.l d0
pub fn i_08595e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85960;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085960: move.b $98(a6), d0
pub fn i_085960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85964;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085964: andi.b #$3, d0
pub fn i_085964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85968;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085968: movea.l #$2e7b3e, a0
pub fn i_085968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8596E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7B3E;
    cpu.a[0] = v1;
    4
}
/// $08596E: lsl.w #$2, d0
pub fn i_08596e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85970;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085970: movea.l (a0, d0.w), a0
pub fn i_085970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85974;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085974: cmpa.l #$ffffffff, a0
pub fn i_085974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8597A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08597A: beq.w $85984
pub fn i_08597a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8597E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85984; return 10; }
    8
}
/// $08597E: jsr $28cd4.l
pub fn i_08597e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85984);
    cpu.pc = 0x28CD4;
    16
}
/// $085984: lea.l $8598a(pc), a1
pub fn i_085984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85988;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8598A;
    16
}
/// $085988: move.l a1, (a6)
pub fn i_085988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8598A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08598A: jsr $2783a.l
pub fn i_08598a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85990;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85990);
    cpu.pc = 0x2783A;
    16
}
/// $085990: jsr $28d70.l
pub fn i_085990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85996);
    cpu.pc = 0x28D70;
    16
}
/// $085996: bcc.w $859b6
pub fn i_085996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8599A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x859B6; return 10; }
    8
}
/// $08599A: lea.l $8595e(pc), a1
pub fn i_08599a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8599E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8595E;
    16
}
/// $08599E: move.l a1, (a6)
pub fn i_08599e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0859A0: lea.l $2eb118.l, a0
pub fn i_0859a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859A6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EB118;
    20
}
/// $0859A6: jsr $5e086.l
pub fn i_0859a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x859AC);
    cpu.pc = 0x5E086;
    16
}
/// $0859AC: bcc.w $859b6
pub fn i_0859ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x859B6; return 10; }
    8
}
/// $0859B0: lea.l $858f2(pc), a1
pub fn i_0859b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x858F2;
    16
}
/// $0859B4: move.l a1, (a6)
pub fn i_0859b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859B6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0859B6: movea.l #$ffffffff, a0
pub fn i_0859b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859BC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0859BC: lea.l $2e9874.l, a0
pub fn i_0859bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E9874;
    20
}
/// $0859C2: jsr $5dd5c.l
pub fn i_0859c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x859C8);
    cpu.pc = 0x5DD5C;
    16
}
/// $0859C8: bcc.w $859d2
pub fn i_0859c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x859D2; return 10; }
    8
}
/// $0859CC: jmp $518.l
pub fn i_0859cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859D2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0859D2: rts
pub fn i_0859d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_CamFloor_Init_0859d4  ($0859D4, 52 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0859D4: move.b #$0, $20(a6)
pub fn i_0859d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859DA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0859DA: move.w #$430, $10816a.l
pub fn i_0859da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859E2;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x430;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v1 as u16);
    16
}
/// $0859E2: move.w #$430, $10816e.l
pub fn i_0859e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859EA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x430;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v1 as u16);
    16
}
/// $0859EA: move.w #$a0, $22(a6)
pub fn i_0859ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0859F0: btst.b #$0, $100001.l
pub fn i_0859f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859F8;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $0859F8: beq.w $85a08
pub fn i_0859f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x859FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85A08; return 10; }
    8
}
/// $0859FC: lea.l $2e7bfe.l, a0
pub fn i_0859fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E7BFE;
    20
}
/// $085A02: jsr $28cd4.l
pub fn i_085a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A08);
    cpu.pc = 0x28CD4;
    16
}
// ===== M4_CamFloor_Step_085a08  ($085A08, 190 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085A08: clr.l d0
pub fn i_085a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A0A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085A0A: move.b $20(a6), d0
pub fn i_085a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A0E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085A0E: asl.l #$1, d0
pub fn i_085a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A10;
    cpu.ir = 0xE380;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Long, 0, true);
    cpu.d[0] = r;
    8
}
/// $085A10: lea.l $ec882.l, a0
pub fn i_085a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0xEC882;
    20
}
/// $085A16: move.w (a0, d0.w), d1
pub fn i_085a16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A1A;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $085A1A: cmpi.w #$ffff, d1
pub fn i_085a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A1E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $085A1E: bne.w $85a2a
pub fn i_085a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A22;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85A2A; return 10; }
    8
}
/// $085A22: jmp $518.l
pub fn i_085a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A28;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $085A2A: jsr $44022.l
pub fn i_085a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A30);
    cpu.pc = 0x44022;
    16
}
/// $085A30: move.w d1, $24(a6)
pub fn i_085a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A34;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A34: lea.l $85a3a(pc), a1
pub fn i_085a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85A3A;
    16
}
/// $085A38: move.l a1, (a6)
pub fn i_085a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085A3A: move.w $22(a6), d0
pub fn i_085a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A3E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085A3E: move.w $24(a6), d1
pub fn i_085a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A42;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085A42: movem.w d0, -(a7)
pub fn i_085a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A46;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $085A46: movem.w d1, -(a7)
pub fn i_085a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A4A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[1] as u16);
    cpu.a[7] = a;
    20
}
/// $085A4A: move.w #$0, $22(a6)
pub fn i_085a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A50: move.w #$fc, $24(a6)
pub fn i_085a50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A56;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A56: lea.l $2e97e4.l, a0
pub fn i_085a56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A5C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E97E4;
    20
}
/// $085A5C: move.l a0, $4c(a6)
pub fn i_085a5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A60;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085A60: jsr $283ca.l
pub fn i_085a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A66);
    cpu.pc = 0x283CA;
    16
}
/// $085A66: jsr $283ca.l
pub fn i_085a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A6C);
    cpu.pc = 0x283CA;
    16
}
/// $085A6C: jsr $283d8.l
pub fn i_085a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A72);
    cpu.pc = 0x283D8;
    16
}
/// $085A72: movem.w (a7)+, d1
pub fn i_085a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A76;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $085A76: movem.w (a7)+, d0
pub fn i_085a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A7A;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $085A7A: move.w d0, $22(a6)
pub fn i_085a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A7E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A7E: move.w d1, $24(a6)
pub fn i_085a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A82;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085A82: jsr $2783a.l
pub fn i_085a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A88);
    cpu.pc = 0x2783A;
    16
}
/// $085A88: btst.b #$0, $100001.l
pub fn i_085a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A90;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((0) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $085A90: beq.w $85a9a
pub fn i_085a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85A9A; return 10; }
    8
}
/// $085A94: jsr $28d70.l
pub fn i_085a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85A9A);
    cpu.pc = 0x28D70;
    16
}
/// $085A9A: jsr $85ace(pc)
pub fn i_085a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85A9E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85A9E);
    cpu.pc = 0x85ACE;
    16
}
/// $085A9E: bcc.w $85acc
pub fn i_085a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AA2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85ACC; return 10; }
    8
}
/// $085AA2: move.w $24(a6), d1
pub fn i_085aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AA6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085AA6: jsr $4400e.l
pub fn i_085aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85AAC);
    cpu.pc = 0x4400E;
    16
}
/// $085AAC: subi.w #$100, d1
pub fn i_085aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AB0;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x100, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085AB0: bpl.w $85ab6
pub fn i_085ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AB4;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x85AB6; return 10; }
    8
}
/// $085AB4: clr.w d1
pub fn i_085ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AB6;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085AB6: move.w d1, $10816a.l
pub fn i_085ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ABC;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816A, v1 as u16);
    16
}
/// $085ABC: move.w d1, $10816e.l
pub fn i_085abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AC2;
    cpu.ir = 0x33C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10816E, v1 as u16);
    16
}
/// $085AC2: addq.b #$1, $20(a6)
pub fn i_085ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AC6;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetTaskHandler_085ac6  ($085AC6, 8 B, kernel/task_handlers.c)
/// $085AC6: lea.l $85a08(pc), a1
pub fn i_085ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ACA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x85A08;
    16
}
/// $085ACA: move.l a1, (a6)
pub fn i_085aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ACC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $085ACC: rts
pub fn i_085acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ACE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_PlayersBelowY_085ace  ($085ACE, 142 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085ACE: jsr $5e55c.l
pub fn i_085ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85AD4);
    cpu.pc = 0x5E55C;
    16
}
/// $085AD4: cmpi.b #$0, d0
pub fn i_085ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AD8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $085AD8: beq.w $85b62
pub fn i_085ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ADC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085ADC: cmpi.b #$1, d0
pub fn i_085adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AE0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $085AE0: bne.w $85b02
pub fn i_085ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85B02; return 10; }
    8
}
/// $085AE4: lea.l $100440.l, a0
pub fn i_085ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $085AEA: jsr $85b68(pc)
pub fn i_085aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85AEE);
    cpu.pc = 0x85B68;
    16
}
/// $085AEE: bcc.w $85b62
pub fn i_085aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085AF2: move.w $24(a0), d1
pub fn i_085af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AF6;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085AF6: cmp.w $24(a6), d1
pub fn i_085af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AFA;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $085AFA: blt.w $85b62
pub fn i_085afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85AFE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085AFE: bra.w $85b5c
pub fn i_085afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B02;
    cpu.ir = 0x6000;
    cpu.pc = 0x85B5C;
    10
}
/// $085B02: cmpi.b #$2, d0
pub fn i_085b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B06;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $085B06: bne.w $85b28
pub fn i_085b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B0A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85B28; return 10; }
    8
}
/// $085B0A: lea.l $1004e0.l, a0
pub fn i_085b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $085B10: jsr $85b68(pc)
pub fn i_085b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85B14);
    cpu.pc = 0x85B68;
    16
}
/// $085B14: bcc.w $85b62
pub fn i_085b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B18: move.w $24(a0), d1
pub fn i_085b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B1C;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085B1C: cmp.w $24(a6), d1
pub fn i_085b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B20;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $085B20: blt.w $85b62
pub fn i_085b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B24;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B24: bra.w $85b5c
pub fn i_085b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B28;
    cpu.ir = 0x6000;
    cpu.pc = 0x85B5C;
    10
}
/// $085B28: lea.l $100440.l, a0
pub fn i_085b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B2E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $085B2E: jsr $85b68(pc)
pub fn i_085b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85B32);
    cpu.pc = 0x85B68;
    16
}
/// $085B32: bcc.w $85b62
pub fn i_085b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B36;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B36: move.w $24(a0), d1
pub fn i_085b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B3A;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085B3A: cmp.w $24(a6), d1
pub fn i_085b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B3E;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $085B3E: blt.w $85b62
pub fn i_085b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B42;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B42: lea.l $1004e0.l, a0
pub fn i_085b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B48;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $085B48: jsr $85b68(pc)
pub fn i_085b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x85B4C);
    cpu.pc = 0x85B68;
    16
}
/// $085B4C: bcc.w $85b62
pub fn i_085b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85B62; return 10; }
    8
}
/// $085B50: move.w $24(a0), d1
pub fn i_085b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B54;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085B54: cmp.w $24(a6), d1
pub fn i_085b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B58;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $085B58: blt.w $85b62
pub fn i_085b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B5C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85B62; return 10; }
    8
}
// ===== SetXN_085b5c  ($085B5C, 6 B, thunks/ccr_helpers.c)
/// $085B5C: ori.b #$11, ccr
pub fn i_085b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B60;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085B60: rts
pub fn i_085b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B62;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085b62  ($085B62, 6 B, thunks/ccr_helpers.c)
/// $085B62: andi.b #$ee, ccr
pub fn i_085b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B66;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085B66: rts
pub fn i_085b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SlotAlive_085b68  ($085B68, 10 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085B68: jsr $32e08.l
pub fn i_085b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85B6E);
    cpu.pc = 0x32E08;
    16
}
/// $085B6E: bcs.w $85b78
pub fn i_085b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B72;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85B78; return 10; }
    8
}
// ===== SetXN_085b72  ($085B72, 6 B, thunks/ccr_helpers.c)
/// $085B72: ori.b #$11, ccr
pub fn i_085b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B76;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085B76: rts
pub fn i_085b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085b78  ($085B78, 6 B, thunks/ccr_helpers.c)
/// $085B78: andi.b #$ee, ccr
pub fn i_085b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B7C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085B7C: rts
pub fn i_085b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_SetupSprites4_085b7e  ($085B7E, 82 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085B7E: move.w #$4e, d1
pub fn i_085b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B82;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B82: move.w #$1f8, d2
pub fn i_085b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B86;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B86: move.w #$ffff, d3
pub fn i_085b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B8A;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B8A: move.w #$1, d4
pub fn i_085b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B8E;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B8E: jsr $2c26.l
pub fn i_085b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85B94);
    cpu.pc = 0x2C26;
    16
}
/// $085B94: move.w #$4f, d1
pub fn i_085b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B98;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B98: move.w #$1f9, d2
pub fn i_085b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85B9C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1F9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085B9C: move.w #$ffff, d3
pub fn i_085b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BA0;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BA0: move.w #$1, d4
pub fn i_085ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BA4;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BA4: jsr $2c26.l
pub fn i_085ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85BAA);
    cpu.pc = 0x2C26;
    16
}
/// $085BAA: move.w #$50, d1
pub fn i_085baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BAE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BAE: move.w #$1fa, d2
pub fn i_085bae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BB2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1FA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BB2: move.w #$ffff, d3
pub fn i_085bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BB6;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BB6: move.w #$1, d4
pub fn i_085bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BBA;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BBA: jsr $2c26.l
pub fn i_085bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85BC0);
    cpu.pc = 0x2C26;
    16
}
/// $085BC0: move.w #$51, d1
pub fn i_085bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BC4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x51;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BC4: move.w #$1fb, d2
pub fn i_085bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BC8;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1FB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BC8: move.w #$ffff, d3
pub fn i_085bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BCC;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085BCC: move.w #$1, d4
pub fn i_085bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BD0;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_085bd0  ($085BD0, 8 B, thunks/jsr_abs_thunks.c)
/// $085BD0: jsr $2c26.l
pub fn i_085bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85BD6);
    cpu.pc = 0x2C26;
    16
}
/// $085BD6: rts
pub fn i_085bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85BD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_085c2a  ($085C2A, 8 B, thunks/jsr_abs_thunks.c)
/// $085C2A: jsr $2c26.l
pub fn i_085c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85C30);
    cpu.pc = 0x2C26;
    16
}
/// $085C30: rts
pub fn i_085c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Wheel_Anim_085c32  ($085C32, 202 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085C32: movea.l $c(a6), a0
pub fn i_085c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C36;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $085C36: cmpi.w #$0, $2a(a0)
pub fn i_085c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C3C;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $085C3C: beq.w $85cd6
pub fn i_085c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85CD6; return 10; }
    8
}
/// $085C40: clr.l d0
pub fn i_085c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C42;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085C42: move.w $2a(a0), d0
pub fn i_085c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C46;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085C46: btst.b #$f, d0
pub fn i_085c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C4A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085C4A: bne.w $85c86
pub fn i_085c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85C86; return 10; }
    8
}
/// $085C4E: move.b $88(a6), d0
pub fn i_085c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C52;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085C52: subq.b #$1, d0
pub fn i_085c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C54;
    cpu.ir = 0x5300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $085C54: andi.b #$3, d0
pub fn i_085c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C58;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085C58: move.b d0, $88(a6)
pub fn i_085c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C5C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085C5C: clr.l d0
pub fn i_085c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C5E;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085C5E: move.b $88(a6), d0
pub fn i_085c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C62;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085C62: andi.b #$3, d0
pub fn i_085c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C66;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085C66: movea.l #$2e7bee, a0
pub fn i_085c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C6C;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7BEE;
    cpu.a[0] = v1;
    4
}
/// $085C6C: lsl.w #$2, d0
pub fn i_085c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C6E;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085C6E: movea.l (a0, d0.w), a0
pub fn i_085c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C72;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085C72: cmpa.l #$ffffffff, a0
pub fn i_085c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C78;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $085C78: beq.w $85c82
pub fn i_085c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C7C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85C82; return 10; }
    8
}
/// $085C7C: jsr $28cd4.l
pub fn i_085c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85C82);
    cpu.pc = 0x28CD4;
    16
}
/// $085C82: bra.w $85cfc
pub fn i_085c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C86;
    cpu.ir = 0x6000;
    cpu.pc = 0x85CFC;
    10
}
/// $085C86: jsr $28d70.l
pub fn i_085c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85C8C);
    cpu.pc = 0x28D70;
    16
}
/// $085C8C: bcc.w $85d02
pub fn i_085c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C90;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85D02; return 10; }
    8
}
/// $085C90: move.b $88(a6), d0
pub fn i_085c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C94;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085C94: addq.b #$1, d0
pub fn i_085c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C96;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $085C96: andi.b #$3, d0
pub fn i_085c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C9A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085C9A: move.b d0, $88(a6)
pub fn i_085c9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85C9E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085C9E: clr.l d0
pub fn i_085c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CA0;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085CA0: move.b $88(a6), d0
pub fn i_085ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CA4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085CA4: andi.b #$3, d0
pub fn i_085ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CA8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085CA8: movea.l #$2e7bde, a0
pub fn i_085ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CAE;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7BDE;
    cpu.a[0] = v1;
    4
}
/// $085CAE: lsl.w #$2, d0
pub fn i_085cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CB0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085CB0: movea.l (a0, d0.w), a0
pub fn i_085cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CB4;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085CB4: cmpa.l #$ffffffff, a0
pub fn i_085cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CBA;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $085CBA: beq.w $85cc4
pub fn i_085cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CBE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85CC4; return 10; }
    8
}
/// $085CBE: jsr $28cd4.l
pub fn i_085cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85CC4);
    cpu.pc = 0x28CD4;
    16
}
/// $085CC4: bra.w $85d02
pub fn i_085cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x85D02;
    10
}
/// $085CD6: clr.l d0
pub fn i_085cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CD8;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085CD8: move.b $88(a6), d0
pub fn i_085cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CDC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085CDC: andi.b #$3, d0
pub fn i_085cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CE0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085CE0: movea.l #$2e7bde, a0
pub fn i_085ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CE6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E7BDE;
    cpu.a[0] = v1;
    4
}
/// $085CE6: lsl.w #$2, d0
pub fn i_085ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CE8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $085CE8: movea.l (a0, d0.w), a0
pub fn i_085ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CEC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $085CEC: cmpa.l #$ffffffff, a0
pub fn i_085cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CF2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $085CF2: beq.w $85cfc
pub fn i_085cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CF6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85CFC; return 10; }
    8
}
/// $085CF6: jsr $28cd4.l
pub fn i_085cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85CFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85CFC);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_085cfc  ($085CFC, 8 B, thunks/jsr_abs_thunks.c)
/// $085CFC: jsr $28d70.l
pub fn i_085cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D02);
    cpu.pc = 0x28D70;
    16
}
/// $085D02: rts
pub fn i_085d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Rail_Blit_085d04  ($085D04, 30 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085D04: lea.l $2eaa10.l, a0
pub fn i_085d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D0A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EAA10;
    20
}
/// $085D0A: clr.l d0
pub fn i_085d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D0C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085D0C: move.b $21(a6), d0
pub fn i_085d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D10;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085D10: cmpi.b #$14, d0
pub fn i_085d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D14;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x14, Size::Byte, false);
    8
}
/// $085D14: blt.w $85d1c
pub fn i_085d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D18;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85D1C; return 10; }
    8
}
/// $085D18: move.b #$0, d0
pub fn i_085d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D1C;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $085D1C: asl.l #$2, d0
pub fn i_085d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D1E;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $085D1E: movea.l (a0, d0.w), a2
pub fn i_085d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D22;
    cpu.ir = 0x2470;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_085d22  ($085D22, 8 B, thunks/jsr_abs_thunks.c)
/// $085D22: jsr $5022a.l
pub fn i_085d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D28);
    cpu.pc = 0x5022A;
    16
}
/// $085D28: rts
pub fn i_085d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrAbsThunk_085d2a  ($085D2A, 8 B, thunks/jsr_abs_thunks.c)
/// $085D2A: jsr $8b558.l
pub fn i_085d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D30);
    cpu.pc = 0x8B558;
    16
}
/// $085D30: rts
pub fn i_085d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D32;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_EdgeCheck_085d32  ($085D32, 106 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085D32: move.w $22(a6), d0
pub fn i_085d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D36;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085D36: move.w $24(a6), d1
pub fn i_085d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D3A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085D3A: jsr $4400e.l
pub fn i_085d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D40);
    cpu.pc = 0x4400E;
    16
}
/// $085D40: clr.l d0
pub fn i_085d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D42;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $085D42: move.w $2a(a6), d0
pub fn i_085d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D46;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085D46: btst.b #$f, d0
pub fn i_085d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D4A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085D4A: bne.w $85d5a
pub fn i_085d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85D5A; return 10; }
    8
}
/// $085D4E: cmpi.w #$2d0, d1
pub fn i_085d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D52;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2D0, Size::Word, false);
    8
}
/// $085D52: bgt.w $85da2
pub fn i_085d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D56;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x85DA2; return 10; }
    8
}
/// $085D56: bra.w $85d9c
pub fn i_085d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D5A;
    cpu.ir = 0x6000;
    cpu.pc = 0x85D9C;
    10
}
/// $085D5A: cmpi.w #$310, d1
pub fn i_085d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D5E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x310, Size::Word, false);
    8
}
/// $085D5E: blt.w $85da2
pub fn i_085d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D62;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x85DA2; return 10; }
    8
}
/// $085D62: lea.l $2e99a8.l, a1
pub fn i_085d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E99A8;
    20
}
/// $085D68: jsr $77c7e.l
pub fn i_085d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D6E);
    cpu.pc = 0x77C7E;
    16
}
/// $085D6E: subi.w #$8, $24(a0)
pub fn i_085d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D74;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $085D74: lea.l $2e99ba.l, a1
pub fn i_085d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D7A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E99BA;
    20
}
/// $085D7A: jsr $77c7e.l
pub fn i_085d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D80);
    cpu.pc = 0x77C7E;
    16
}
/// $085D80: subi.w #$8, $38(a0)
pub fn i_085d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D86;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $085D86: move.b #$a, $75(a6)
pub fn i_085d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D8C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085D8C: lea.l $854e8(pc), a1
pub fn i_085d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x854E8;
    16
}
/// $085D90: jsr $4ae.l
pub fn i_085d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D96;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D96);
    cpu.pc = 0x4AE;
    16
}
/// $085D96: jsr $5dd02.l
pub fn i_085d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85D9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85D9C);
    cpu.pc = 0x5DD02;
    16
}
// ===== SetXN_085d9c  ($085D9C, 6 B, thunks/ccr_helpers.c)
/// $085D9C: ori.b #$11, ccr
pub fn i_085d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DA0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085DA0: rts
pub fn i_085da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DA2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085da2  ($085DA2, 6 B, thunks/ccr_helpers.c)
/// $085DA2: andi.b #$ee, ccr
pub fn i_085da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DA6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085DA6: rts
pub fn i_085da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Knockback_085da8  ($085DA8, 102 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085DA8: lea.l $2e94a2.l, a0
pub fn i_085da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DAE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E94A2;
    20
}
/// $085DAE: move.l a0, $48(a6)
pub fn i_085dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DB2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $085DB2: move.w #$7fff, $66(a6)
pub fn i_085db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DB8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085DB8: jsr $2870a.l
pub fn i_085db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DBE);
    cpu.pc = 0x2870A;
    16
}
/// $085DBE: bcc.w $85e14
pub fn i_085dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x85E14; return 10; }
    8
}
/// $085DC2: lea.l $2c031c.l, a0
pub fn i_085dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C031C;
    20
}
/// $085DC8: jsr $799de.l
pub fn i_085dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DCE);
    cpu.pc = 0x799DE;
    16
}
/// $085DCE: move.w d0, $2a(a6)
pub fn i_085dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DD2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085DD2: lea.l $2e791e.l, a0
pub fn i_085dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DD8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E791E;
    20
}
/// $085DD8: jsr $28cd4.l
pub fn i_085dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DDE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DDE);
    cpu.pc = 0x28CD4;
    16
}
/// $085DDE: bclr.b #$3, $13(a6)
pub fn i_085dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DE4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $085DE4: move.w #$108d, d0
pub fn i_085de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DE8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085DE8: jsr $2352.l
pub fn i_085de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DEE);
    cpu.pc = 0x2352;
    16
}
/// $085DEE: move.w $22(a6), d0
pub fn i_085dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DF2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085DF2: move.w $24(a6), d1
pub fn i_085df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DF6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085DF6: jsr $4400e.l
pub fn i_085df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85DFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85DFC);
    cpu.pc = 0x4400E;
    16
}
/// $085DFC: cmpi.w #$2d0, d1
pub fn i_085dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E00;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2D0, Size::Word, false);
    8
}
/// $085E00: ble.w $85e2a
pub fn i_085e00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E04;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85E2A; return 10; }
    8
}
/// $085E04: move.b $78(a6), d0
pub fn i_085e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E08;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085E08: asl.b #$1, d0
pub fn i_085e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E0A;
    cpu.ir = 0xE300;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $085E0A: bclr.b #$0, d0
pub fn i_085e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E0E;
    cpu.ir = 0x880;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
// ===== SetTaskB_085e0e  ($085E0E, 6 B, kernel/task_setters_b.c)
/// $085E0E: move.b d0, $78(a6)
pub fn i_085e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E12;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085E12: rts
pub fn i_085e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Recoil_085e14  ($085E14, 52 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085E14: move.w $22(a6), d0
pub fn i_085e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E18;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085E18: move.w $24(a6), d1
pub fn i_085e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E1C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085E1C: jsr $4400e.l
pub fn i_085e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85E22);
    cpu.pc = 0x4400E;
    16
}
/// $085E22: cmpi.w #$310, d1
pub fn i_085e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E26;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x310, Size::Word, false);
    8
}
/// $085E26: bge.w $85e4e
pub fn i_085e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E2A;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x85E4E; return 10; }
    8
}
/// $085E2A: lea.l $2c039e.l, a0
pub fn i_085e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C039E;
    20
}
/// $085E30: jsr $799de.l
pub fn i_085e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85E36);
    cpu.pc = 0x799DE;
    16
}
/// $085E36: move.w d0, d0
pub fn i_085e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E38;
    cpu.ir = 0x3000;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085E38: neg.w d0
pub fn i_085e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E3A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085E3A: move.w d0, $2a(a6)
pub fn i_085e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E3E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085E3E: move.b $78(a6), d0
pub fn i_085e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E42;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085E42: asl.b #$1, d0
pub fn i_085e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E44;
    cpu.ir = 0xE300;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $085E44: bclr.b #$0, d0
pub fn i_085e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E48;
    cpu.ir = 0x880;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
// ===== SetTaskB_085e48  ($085E48, 6 B, kernel/task_setters_b.c)
/// $085E48: move.b d0, $78(a6)
pub fn i_085e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E4C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085E4C: rts
pub fn i_085e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E4E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_RecoilStop_085e4e  ($085E4E, 22 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085E4E: move.b $78(a6), d0
pub fn i_085e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E52;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085E52: asl.b #$1, d0
pub fn i_085e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E54;
    cpu.ir = 0xE300;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Byte, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $085E54: bset.b #$0, d0
pub fn i_085e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E58;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $085E58: move.b d0, $78(a6)
pub fn i_085e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E5C;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write8(ad2, v1 as u8);
    12
}
/// $085E5C: move.w #$0, $2a(a6)
pub fn i_085e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E62;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085E62: rts
pub fn i_085e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_PickTargetSlot_085e64  ($085E64, 76 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085E64: jsr $5e1aa.l
pub fn i_085e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85E6A);
    cpu.pc = 0x5E1AA;
    16
}
/// $085E6A: bcs.w $85eb6
pub fn i_085e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E6E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x85EB6; return 10; }
    8
}
/// $085E6E: cmpi.w #$1, d0
pub fn i_085e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E72;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $085E72: bne.w $85e80
pub fn i_085e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85E80; return 10; }
    8
}
/// $085E76: lea.l $100440.l, a0
pub fn i_085e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E7C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $085E7C: bra.w $85eb0
pub fn i_085e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E80;
    cpu.ir = 0x6000;
    cpu.pc = 0x85EB0;
    10
}
/// $085E80: cmpi.w #$2, d0
pub fn i_085e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E84;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $085E84: bne.w $85e92
pub fn i_085e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E88;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85E92; return 10; }
    8
}
/// $085E88: lea.l $1004e0.l, a0
pub fn i_085e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E8E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $085E8E: bra.w $85eb0
pub fn i_085e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E92;
    cpu.ir = 0x6000;
    cpu.pc = 0x85EB0;
    10
}
/// $085E92: move.b $106f28.l, d0
pub fn i_085e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E98;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $085E98: btst.b #$7, d0
pub fn i_085e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85E9C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085E9C: bne.w $85eaa
pub fn i_085e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85EAA; return 10; }
    8
}
/// $085EA0: lea.l $100440.l, a0
pub fn i_085ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $085EA6: bra.w $85eb0
pub fn i_085ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EAA;
    cpu.ir = 0x6000;
    cpu.pc = 0x85EB0;
    10
}
/// $085EAA: lea.l $1004e0.l, a0
pub fn i_085eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
// ===== SetXN_085eb0  ($085EB0, 6 B, thunks/ccr_helpers.c)
/// $085EB0: ori.b #$11, ccr
pub fn i_085eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EB4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085EB4: rts
pub fn i_085eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085eb6  ($085EB6, 6 B, thunks/ccr_helpers.c)
/// $085EB6: andi.b #$ee, ccr
pub fn i_085eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EBA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085EBA: rts
pub fn i_085eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M4_Turret_Shake_085ebc  ($085EBC, 44 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085EBC: cmpi.b #$0, $75(a6)
pub fn i_085ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EC2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $085EC2: ble.w $85ee6
pub fn i_085ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EC6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85EE6; return 10; }
    8
}
/// $085EC6: subq.b #$1, $75(a6)
pub fn i_085ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ECA;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x75);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $085ECA: move.w #$1, d1
pub fn i_085eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ECE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085ECE: move.b $75(a6), d0
pub fn i_085ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ED2;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x75);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085ED2: btst.b #$0, d0
pub fn i_085ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85ED6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085ED6: bne.w $85ee2
pub fn i_085ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EDA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85EE2; return 10; }
    8
}
/// $085EDA: add.w d1, $24(a6)
pub fn i_085eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EDE;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $085EDE: bra.w $85ee6
pub fn i_085ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x85EE6;
    10
}
/// $085EE2: sub.w d1, $24(a6)
pub fn i_085ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EE6;
    cpu.ir = 0x936E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $085EE6: rts
pub fn i_085ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_WobbleArm_085ee8  ($085EE8, 32 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085EE8: cmpi.w #$0, $72(a6)
pub fn i_085ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EEE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $085EEE: bgt.w $85f06
pub fn i_085eee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EF2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x85F06; return 10; }
    8
}
/// $085EF2: jsr $5e9b6.l
pub fn i_085ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85EF8);
    cpu.pc = 0x5E9B6;
    16
}
/// $085EF8: andi.w #$1f, d0
pub fn i_085ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85EFC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085EFC: bne.w $85f06
pub fn i_085efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F00;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85F06; return 10; }
    8
}
/// $085F00: move.w #$2, $72(a6)
pub fn i_085f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085F06: rts
pub fn i_085f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_WobbleStep_085f08  ($085F08, 60 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085F08: cmpi.w #$0, $72(a6)
pub fn i_085f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F0E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $085F0E: ble.w $85f42
pub fn i_085f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F12;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85F42; return 10; }
    8
}
/// $085F12: subq.w #$1, $72(a6)
pub fn i_085f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F16;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $085F16: move.w #$100, d1
pub fn i_085f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F1A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085F1A: move.w $72(a6), d0
pub fn i_085f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F1E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F1E: btst.b #$0, d0
pub fn i_085f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F22;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085F22: bne.w $85f28
pub fn i_085f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x85F28; return 10; }
    8
}
/// $085F26: neg.w d1
pub fn i_085f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F28;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085F28: move.w $2a(a6), d2
pub fn i_085f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F2C;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F2C: movem.w d2, -(a7)
pub fn i_085f2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F30;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    cpu.a[7] = a;
    20
}
/// $085F30: move.w d1, $2a(a6)
pub fn i_085f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F34;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085F34: jsr $27cee.l
pub fn i_085f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85F3A);
    cpu.pc = 0x27CEE;
    16
}
/// $085F3A: movem.w (a7)+, d2
pub fn i_085f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F3E;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $085F3E: move.w d2, $2a(a6)
pub fn i_085f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F42;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085F42: rts
pub fn i_085f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F44;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_HitboxParams_085f44  ($085F44, 20 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085F44: move.w $22(a6), d0
pub fn i_085f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F48;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F48: subi.w #$60, d0
pub fn i_085f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F4C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x60, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085F4C: move.w $24(a6), d1
pub fn i_085f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F50;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F50: subi.w #$0, d1
pub fn i_085f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F54;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x0, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085F54: move.w #$c0, d2
pub fn i_085f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F58;
    cpu.ir = 0x343C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_085f58  ($085F58, 8 B, thunks/jsr_abs_thunks.c)
/// $085F58: jsr $99812.l
pub fn i_085f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85F5E);
    cpu.pc = 0x99812;
    16
}
/// $085F5E: rts
pub fn i_085f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_AltitudeCheck_085f60  ($085F60, 50 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085F60: move.w $22(a6), d0
pub fn i_085f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F64;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F64: move.w $24(a6), d1
pub fn i_085f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F68;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F68: jsr $44022.l
pub fn i_085f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85F6E);
    cpu.pc = 0x44022;
    16
}
/// $085F6E: clr.w d2
pub fn i_085f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F70;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085F70: clr.w d3
pub fn i_085f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F72;
    cpu.ir = 0x4243;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $085F72: move.b $98(a6), d2
pub fn i_085f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F76;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085F76: move.b $99(a6), d3
pub fn i_085f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F7A;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $085F7A: asl.w #$4, d2
pub fn i_085f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F7C;
    cpu.ir = 0xE942;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $085F7C: asl.w #$4, d3
pub fn i_085f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F7E;
    cpu.ir = 0xE943;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $085F7E: clr.l d4
pub fn i_085f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F80;
    cpu.ir = 0x4284;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[4] = r2;
    4
}
/// $085F80: move.w $2a(a6), d4
pub fn i_085f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F84;
    cpu.ir = 0x382E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $085F84: btst.b #$f, d4
pub fn i_085f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F88;
    cpu.ir = 0x804;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $085F88: beq.w $85f9e
pub fn i_085f88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F8C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x85F9E; return 10; }
    8
}
/// $085F8C: cmp.w d1, d3
pub fn i_085f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F8E;
    cpu.ir = 0xB641;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $085F8E: ble.w $85f98
pub fn i_085f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F92;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x85F98; return 10; }
    8
}
// ===== SetXN_085f92  ($085F92, 6 B, thunks/ccr_helpers.c)
/// $085F92: ori.b #$11, ccr
pub fn i_085f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F96;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085F96: rts
pub fn i_085f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F98;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_085f98  ($085F98, 6 B, thunks/ccr_helpers.c)
/// $085F98: andi.b #$ee, ccr
pub fn i_085f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F9C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085F9C: rts
pub fn i_085f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85F9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Flight_AltitudeCheckUp_085f9e  ($085F9E, 6 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085F9E: cmp.w d1, d2
pub fn i_085f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FA0;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $085FA0: bge.w $85faa
pub fn i_085fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FA4;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x85FAA; return 10; }
    8
}
// ===== ClearXN_085fa4  ($085FA4, 6 B, thunks/ccr_helpers.c)
/// $085FA4: andi.b #$ee, ccr
pub fn i_085fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FA8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $085FA8: rts
pub fn i_085fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FAA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_085faa  ($085FAA, 6 B, thunks/ccr_helpers.c)
/// $085FAA: ori.b #$11, ccr
pub fn i_085faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FAE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $085FAE: rts
pub fn i_085fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_RandomDropSpawn_085fb0  ($085FB0, 108 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $085FB0: jsr $5e9b6.l
pub fn i_085fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FB6);
    cpu.pc = 0x5E9B6;
    16
}
/// $085FB6: andi.b #$7, d0
pub fn i_085fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FBA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $085FBA: bne.w $8601a
pub fn i_085fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FBE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8601A; return 10; }
    8
}
/// $085FBE: move.w #$2, d0
pub fn i_085fbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FC2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085FC2: jsr $5e9e4.l
pub fn i_085fc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FC8);
    cpu.pc = 0x5E9E4;
    16
}
/// $085FC8: addi.w #$1, d0
pub fn i_085fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FCC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $085FCC: cmpi.w #$0, d0
pub fn i_085fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FD0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $085FD0: beq.w $8601a
pub fn i_085fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FD4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8601A; return 10; }
    8
}
/// $085FD4: movem.w d0, -(a7)
pub fn i_085fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FD8;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $085FD8: lea.l $3fec6.l, a1
pub fn i_085fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FDE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3FEC6;
    20
}
/// $085FDE: jsr $4ae.l
pub fn i_085fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FE4);
    cpu.pc = 0x4AE;
    16
}
/// $085FE4: jsr $5dd22.l
pub fn i_085fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FEA);
    cpu.pc = 0x5DD22;
    16
}
/// $085FEA: movem.l a0, -(a7)
pub fn i_085fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FEE;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $085FEE: move.w #$200, $24(a0)
pub fn i_085fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FF4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $085FF4: move.w #$30, d0
pub fn i_085ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $085FF8: jsr $5e9e4.l
pub fn i_085ff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x85FFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x85FFE);
    cpu.pc = 0x5E9E4;
    16
}
/// $085FFE: btst.b #$0, d0
pub fn i_085ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86002;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $086002: beq.w $86008
pub fn i_086002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86006;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86008; return 10; }
    8
}
/// $086006: neg.w d0
pub fn i_086006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86008;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $086008: movem.l (a7)+, a0
pub fn i_086008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8600C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08600C: add.w d0, $22(a0)
pub fn i_08600c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86010;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $086010: movem.w (a7)+, d0
pub fn i_086010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86014;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $086014: subi.w #$1, d0
pub fn i_086014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86018;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $086018: bra.b $85fcc
pub fn i_086018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8601A;
    cpu.ir = 0x60B2;
    cpu.pc = 0x85FCC;
    10
}
/// $08601A: rts
pub fn i_08601a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8601C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_PhaseJingle_08601c  ($08601C, 24 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08601C: move.b $21(a6), d0
pub fn i_08601c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86020;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $086020: cmpi.b #$1, d0
pub fn i_086020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86024;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $086024: beq.w $86030
pub fn i_086024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86028;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x86030; return 10; }
    8
}
/// $086028: cmpi.b #$5, d0
pub fn i_086028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8602C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $08602C: bne.w $8603c
pub fn i_08602c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86030;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8603C; return 10; }
    8
}
/// $086030: move.w #$1026, d0
pub fn i_086030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86034;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1026;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_086034  ($086034, 8 B, thunks/jsr_abs_thunks.c)
/// $086034: jsr $2352.l
pub fn i_086034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8603A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8603A);
    cpu.pc = 0x2352;
    16
}
/// $08603A: rts
pub fn i_08603a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8603C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_PhaseJingle8_08603c  ($08603C, 12 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08603C: cmpi.b #$8, d0
pub fn i_08603c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86040;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $086040: bne.w $8604e
pub fn i_086040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86044;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8604E; return 10; }
    8
}
/// $086044: move.w #$1032, d0
pub fn i_086044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86048;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1032;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrAbsThunk_086048  ($086048, 8 B, thunks/jsr_abs_thunks.c)
/// $086048: jsr $2352.l
pub fn i_086048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8604E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8604E);
    cpu.pc = 0x2352;
    16
}
/// $08604E: rts
pub fn i_08604e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86050;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SineBob_086050  ($086050, 30 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086050: addi.w #$8, $34(a6)
pub fn i_086050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86056;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086056: andi.w #$ff, $34(a6)
pub fn i_086056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8605C;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08605C: move.w $34(a6), d0
pub fn i_08605c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86060;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $086060: move.w #$80, d1
pub fn i_086060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86064;
    cpu.ir = 0x323C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $086064: jsr $13c0e.l
pub fn i_086064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8606A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8606A);
    cpu.pc = 0x13C0E;
    16
}
/// $08606A: move.w d2, $2a(a6)
pub fn i_08606a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8606E;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_08606e  ($08606E, 8 B, thunks/jsr_abs_thunks.c)
/// $08606E: jsr $27cee.l
pub fn i_08606e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86074);
    cpu.pc = 0x27CEE;
    16
}
/// $086074: rts
pub fn i_086074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86076;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnGuardList_086076  ($086076, 110 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086076: lea.l $2eaf1c.l, a1
pub fn i_086076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8607C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAF1C;
    20
}
/// $08607C: cmpi.b #$b, $106ece.l
pub fn i_08607c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86084;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xB, Size::Byte, false);
    20
}
/// $086084: bne.w $8608e
pub fn i_086084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86088;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8608E; return 10; }
    8
}
/// $086088: lea.l $2eaf7e.l, a1
pub fn i_086088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8608E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAF7E;
    20
}
/// $08608E: clr.l d0
pub fn i_08608e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86090;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086090: movem.l a1, -(a7)
pub fn i_086090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86094;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086094: movem.l d0, -(a7)
pub fn i_086094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86098;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086098: lea.l $83c02(pc), a1
pub fn i_086098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8609C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83C02;
    16
}
/// $08609C: jsr $6fe.l
pub fn i_08609c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x860A2);
    cpu.pc = 0x6FE;
    16
}
/// $0860A2: jsr $5dd02.l
pub fn i_0860a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x860A8);
    cpu.pc = 0x5DD02;
    16
}
/// $0860A8: movem.l (a7)+, d0
pub fn i_0860a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860AC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0860AC: movem.l (a7)+, a1
pub fn i_0860ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860B0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0860B0: move.w (a1, d0.w), d1
pub fn i_0860b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860B4;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0860B4: add.w d1, $22(a0)
pub fn i_0860b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860B8;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0860B8: move.w $2(a1, d0.w), d1
pub fn i_0860b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860BC;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0860BC: add.w d1, $24(a0)
pub fn i_0860bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860C0;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0860C0: move.w $4(a1, d0.w), $38(a0)
pub fn i_0860c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860C6;
    cpu.ir = 0x3171;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0860C6: move.b $6(a1, d0.w), $21(a0)
pub fn i_0860c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860CC;
    cpu.ir = 0x1171;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    22
}
/// $0860CC: move.b $7(a1, d0.w), d1
pub fn i_0860cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860D0;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x7).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0860D0: or.b d1, $3a(a0)
pub fn i_0860d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860D4;
    cpu.ir = 0x8328;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0860D4: addi.l #$8, d0
pub fn i_0860d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860DA;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $0860DA: cmpi.w #$ffff, (a1, d0.w)
pub fn i_0860da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860E0;
    cpu.ir = 0xC71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $0860E0: bne.b $86090
pub fn i_0860e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860E2;
    cpu.ir = 0x66AE;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86090; return 10; }
    8
}
/// $0860E2: rts
pub fn i_0860e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_Spawn45Children_0860e4  ($0860E4, 40 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $0860E4: clr.b d0
pub fn i_0860e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860E6;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0860E6: movem.w d0, -(a7)
pub fn i_0860e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860EA;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $0860EA: lea.l $845ec(pc), a1
pub fn i_0860ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x845EC;
    16
}
/// $0860EE: jsr $6fe.l
pub fn i_0860ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x860F4);
    cpu.pc = 0x6FE;
    16
}
/// $0860F4: jsr $5dd02.l
pub fn i_0860f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x860FA);
    cpu.pc = 0x5DD02;
    16
}
/// $0860FA: movem.w (a7)+, d0
pub fn i_0860fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x860FE;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $0860FE: move.b d0, $21(a0)
pub fn i_0860fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86102;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $086102: addq.b #$1, d0
pub fn i_086102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86104;
    cpu.ir = 0x5200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $086104: cmpi.b #$2d, d0
pub fn i_086104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86108;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2D, Size::Byte, false);
    8
}
/// $086108: blt.b $860e6
pub fn i_086108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8610A;
    cpu.ir = 0x6DDC;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x860E6; return 10; }
    8
}
/// $08610A: rts
pub fn i_08610a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8610C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnStepList_08610c  ($08610C, 138 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $08610C: lea.l $2eafe2.l, a1
pub fn i_08610c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86112;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EAFE2;
    20
}
/// $086112: clr.l d0
pub fn i_086112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86114;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $086114: cmpi.l #$38, d0
pub fn i_086114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8611A;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x38, Size::Long, false);
    8
}
/// $08611A: bge.w $86142
pub fn i_08611a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8611E;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x86142; return 10; }
    8
}
/// $08611E: movem.l a1, -(a7)
pub fn i_08611e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86122;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086122: movem.l d0, -(a7)
pub fn i_086122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86126;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086126: lea.l $84836(pc), a1
pub fn i_086126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8612A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84836;
    16
}
/// $08612A: jsr $6fe.l
pub fn i_08612a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86130;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86130);
    cpu.pc = 0x6FE;
    16
}
/// $086130: jsr $5dd02.l
pub fn i_086130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86136);
    cpu.pc = 0x5DD02;
    16
}
/// $086136: movem.l (a7)+, d0
pub fn i_086136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8613A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08613A: movem.l (a7)+, a1
pub fn i_08613a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8613E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08613E: bra.w $86162
pub fn i_08613e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86142;
    cpu.ir = 0x6000;
    cpu.pc = 0x86162;
    10
}
/// $086142: movem.l a1, -(a7)
pub fn i_086142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86146;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $086146: movem.l d0, -(a7)
pub fn i_086146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8614A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $08614A: lea.l $84836(pc), a1
pub fn i_08614a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8614E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x84836;
    16
}
/// $08614E: jsr $4ae.l
pub fn i_08614e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86154;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86154);
    cpu.pc = 0x4AE;
    16
}
/// $086154: jsr $5dd22.l
pub fn i_086154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8615A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8615A);
    cpu.pc = 0x5DD22;
    16
}
/// $08615A: movem.l (a7)+, d0
pub fn i_08615a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8615E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $08615E: movem.l (a7)+, a1
pub fn i_08615e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86162;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $086162: move.w (a1, d0.w), d1
pub fn i_086162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86166;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $086166: add.w d1, $22(a0)
pub fn i_086166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8616A;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08616A: move.w $2(a1, d0.w), d1
pub fn i_08616a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8616E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08616E: add.w d1, $24(a0)
pub fn i_08616e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86172;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $086172: move.w $4(a1, d0.w), $38(a0)
pub fn i_086172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86178;
    cpu.ir = 0x3171;
    let ad2 = cpu.a[1].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    22
}
/// $086178: move.b $6(a1, d0.w), $21(a0)
pub fn i_086178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8617E;
    cpu.ir = 0x1171;
    let ad2 = cpu.a[1].wrapping_add(0x6).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad4, v3 as u8);
    22
}
/// $08617E: move.b $7(a1, d0.w), d1
pub fn i_08617e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86182;
    cpu.ir = 0x1231;
    let ad2 = cpu.a[1].wrapping_add(0x7).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $086182: or.b d1, $3a(a0)
pub fn i_086182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86186;
    cpu.ir = 0x8328;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $086186: addi.l #$8, d0
pub fn i_086186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8618C;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08618C: cmpi.w #$ffff, (a1, d0.w)
pub fn i_08618c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86192;
    cpu.ir = 0xC71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    18
}
/// $086192: bne.b $86114
pub fn i_086192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86194;
    cpu.ir = 0x6680;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x86114; return 10; }
    8
}
/// $086194: rts
pub fn i_086194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86196;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Boss_SpawnTenEscorts_086196  ($086196, 362 B, enemies/bosses/m4_carrier_boss_helpers_085134.s)
/// $086196: lea.l $77fd6.l, a1
pub fn i_086196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8619C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $08619C: jsr $4ae.l
pub fn i_08619c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861A2);
    cpu.pc = 0x4AE;
    16
}
/// $0861A2: jsr $5dd22.l
pub fn i_0861a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861A8);
    cpu.pc = 0x5DD22;
    16
}
/// $0861A8: addi.w #$30, $22(a0)
pub fn i_0861a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861AE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861AE: addi.w #$20, $24(a0)
pub fn i_0861ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861B4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861B4: move.w #$c000, $38(a0)
pub fn i_0861b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861BA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0861BA: lea.l $77fd6.l, a1
pub fn i_0861ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861C0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0861C0: jsr $4ae.l
pub fn i_0861c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861C6);
    cpu.pc = 0x4AE;
    16
}
/// $0861C6: jsr $5dd22.l
pub fn i_0861c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861CC);
    cpu.pc = 0x5DD22;
    16
}
/// $0861CC: addi.w #$20, $22(a0)
pub fn i_0861cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861D2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861D2: addi.w #$50, $24(a0)
pub fn i_0861d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861D8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861D8: move.w #$c000, $38(a0)
pub fn i_0861d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861DE;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0861DE: lea.l $77fd6.l, a1
pub fn i_0861de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861E4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0861E4: jsr $4ae.l
pub fn i_0861e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861EA);
    cpu.pc = 0x4AE;
    16
}
/// $0861EA: jsr $5dd22.l
pub fn i_0861ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x861F0);
    cpu.pc = 0x5DD22;
    16
}
/// $0861F0: addi.w #$40, $22(a0)
pub fn i_0861f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861F6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861F6: addi.w #$70, $24(a0)
pub fn i_0861f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x861FC;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0861FC: move.w #$c000, $38(a0)
pub fn i_0861fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86202;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086202: lea.l $77fd6.l, a1
pub fn i_086202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86208;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $086208: jsr $4ae.l
pub fn i_086208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8620E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8620E);
    cpu.pc = 0x4AE;
    16
}
/// $08620E: jsr $5dd22.l
pub fn i_08620e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86214);
    cpu.pc = 0x5DD22;
    16
}
/// $086214: addi.w #$60, $22(a0)
pub fn i_086214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8621A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08621A: addi.w #$10, $24(a0)
pub fn i_08621a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86220;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086220: move.w #$c000, $38(a0)
pub fn i_086220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86226;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086226: lea.l $77fd6.l, a1
pub fn i_086226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8622C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $08622C: jsr $4ae.l
pub fn i_08622c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86232);
    cpu.pc = 0x4AE;
    16
}
/// $086232: jsr $5dd22.l
pub fn i_086232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86238);
    cpu.pc = 0x5DD22;
    16
}
/// $086238: addi.w #$80, $22(a0)
pub fn i_086238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8623E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08623E: addi.w #$40, $24(a0)
pub fn i_08623e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86244;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086244: move.w #$c000, $38(a0)
pub fn i_086244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8624A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08624A: lea.l $77fd6.l, a1
pub fn i_08624a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86250;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $086250: jsr $4ae.l
pub fn i_086250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86256;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86256);
    cpu.pc = 0x4AE;
    16
}
/// $086256: jsr $5dd22.l
pub fn i_086256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8625C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8625C);
    cpu.pc = 0x5DD22;
    16
}
/// $08625C: addi.w #$90, $22(a0)
pub fn i_08625c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86262;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086262: addi.w #$70, $24(a0)
pub fn i_086262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86268;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086268: move.w #$c000, $38(a0)
pub fn i_086268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8626E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08626E: lea.l $77fd6.l, a1
pub fn i_08626e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86274;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $086274: jsr $4ae.l
pub fn i_086274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8627A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8627A);
    cpu.pc = 0x4AE;
    16
}
/// $08627A: jsr $5dd22.l
pub fn i_08627a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x86280);
    cpu.pc = 0x5DD22;
    16
}
/// $086280: addi.w #$b0, $22(a0)
pub fn i_086280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86286;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xB0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $086286: addi.w #$20, $24(a0)
pub fn i_086286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8628C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08628C: move.w #$c000, $38(a0)
pub fn i_08628c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86292;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $086292: lea.l $77fd6.l, a1
pub fn i_086292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x86298;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $086298: jsr $4ae.l
pub fn i_086298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8629E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8629E);
    cpu.pc = 0x4AE;
    16
}
/// $08629E: jsr $5dd22.l
pub fn i_08629e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862A4);
    cpu.pc = 0x5DD22;
    16
}
/// $0862A4: addi.w #$d0, $22(a0)
pub fn i_0862a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862AA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xD0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862AA: addi.w #$60, $24(a0)
pub fn i_0862aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862B0;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862B0: move.w #$c000, $38(a0)
pub fn i_0862b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862B6;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0862B6: lea.l $77fd6.l, a1
pub fn i_0862b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862BC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0862BC: jsr $4ae.l
pub fn i_0862bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862C2);
    cpu.pc = 0x4AE;
    16
}
/// $0862C2: jsr $5dd22.l
pub fn i_0862c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862C8);
    cpu.pc = 0x5DD22;
    16
}
/// $0862C8: addi.w #$e0, $22(a0)
pub fn i_0862c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862CE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xE0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862CE: addi.w #$20, $24(a0)
pub fn i_0862ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862D4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862D4: move.w #$c000, $38(a0)
pub fn i_0862d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862DA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0862DA: lea.l $77fd6.l, a1
pub fn i_0862da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862E0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0862E0: jsr $4ae.l
pub fn i_0862e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862E6);
    cpu.pc = 0x4AE;
    16
}
/// $0862E6: jsr $5dd22.l
pub fn i_0862e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x862EC);
    cpu.pc = 0x5DD22;
    16
}
/// $0862EC: addi.w #$f0, $22(a0)
pub fn i_0862ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862F2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xF0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0862F2: addi.w #$70, $24(a0)
pub fn i_0862f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x862F8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x08473E, i_08473e),
    (0x084742, i_084742),
    (0x084748, i_084748),
    (0x08474E, i_08474e),
    (0x084754, i_084754),
    (0x084756, i_084756),
    (0x08475C, i_08475c),
    (0x084760, i_084760),
    (0x084766, i_084766),
    (0x08476C, i_08476c),
    (0x084772, i_084772),
    (0x084778, i_084778),
    (0x08477C, i_08477c),
    (0x084780, i_084780),
    (0x084786, i_084786),
    (0x08478C, i_08478c),
    (0x084792, i_084792),
    (0x084798, i_084798),
    (0x08479E, i_08479e),
    (0x0847A2, i_0847a2),
    (0x0847A4, i_0847a4),
    (0x0847AA, i_0847aa),
    (0x0847B0, i_0847b0),
    (0x0847B4, i_0847b4),
    (0x0847BA, i_0847ba),
    (0x0847BE, i_0847be),
    (0x0847C4, i_0847c4),
    (0x0847C8, i_0847c8),
    (0x0847D0, i_0847d0),
    (0x0847D4, i_0847d4),
    (0x0847DA, i_0847da),
    (0x0847DE, i_0847de),
    (0x0847E4, i_0847e4),
    (0x0847EA, i_0847ea),
    (0x0847F0, i_0847f0),
    (0x0847F6, i_0847f6),
    (0x0847FA, i_0847fa),
    (0x0847FE, i_0847fe),
    (0x084804, i_084804),
    (0x084808, i_084808),
    (0x08480E, i_08480e),
    (0x084814, i_084814),
    (0x08481A, i_08481a),
    (0x08481E, i_08481e),
    (0x084824, i_084824),
    (0x084828, i_084828),
    (0x08482E, i_08482e),
    (0x084834, i_084834),
    (0x084836, i_084836),
    (0x08483A, i_08483a),
    (0x084840, i_084840),
    (0x084846, i_084846),
    (0x08484C, i_08484c),
    (0x08484E, i_08484e),
    (0x084852, i_084852),
    (0x084854, i_084854),
    (0x08485A, i_08485a),
    (0x08485C, i_08485c),
    (0x084860, i_084860),
    (0x084866, i_084866),
    (0x08486A, i_08486a),
    (0x084870, i_084870),
    (0x084874, i_084874),
    (0x084876, i_084876),
    (0x08487C, i_08487c),
    (0x084882, i_084882),
    (0x084886, i_084886),
    (0x08488A, i_08488a),
    (0x08488E, i_08488e),
    (0x084892, i_084892),
    (0x084896, i_084896),
    (0x084898, i_084898),
    (0x08489A, i_08489a),
    (0x08489C, i_08489c),
    (0x0848A0, i_0848a0),
    (0x0848A2, i_0848a2),
    (0x0848A4, i_0848a4),
    (0x0848AA, i_0848aa),
    (0x0848AC, i_0848ac),
    (0x0848B0, i_0848b0),
    (0x0848B6, i_0848b6),
    (0x0848BA, i_0848ba),
    (0x0848C0, i_0848c0),
    (0x0848C4, i_0848c4),
    (0x0848C6, i_0848c6),
    (0x0848CC, i_0848cc),
    (0x0848D2, i_0848d2),
    (0x0848D6, i_0848d6),
    (0x0848DA, i_0848da),
    (0x0848DC, i_0848dc),
    (0x0848DE, i_0848de),
    (0x0848E2, i_0848e2),
    (0x0848E6, i_0848e6),
    (0x0848EC, i_0848ec),
    (0x0848F2, i_0848f2),
    (0x0848F6, i_0848f6),
    (0x0848F8, i_0848f8),
    (0x0848FC, i_0848fc),
    (0x084902, i_084902),
    (0x084906, i_084906),
    (0x08490C, i_08490c),
    (0x084912, i_084912),
    (0x084918, i_084918),
    (0x08491E, i_08491e),
    (0x084924, i_084924),
    (0x084928, i_084928),
    (0x08492A, i_08492a),
    (0x084930, i_084930),
    (0x084932, i_084932),
    (0x084936, i_084936),
    (0x08493A, i_08493a),
    (0x08493C, i_08493c),
    (0x084940, i_084940),
    (0x084946, i_084946),
    (0x08494C, i_08494c),
    (0x084952, i_084952),
    (0x084956, i_084956),
    (0x08495C, i_08495c),
    (0x08495E, i_08495e),
    (0x084962, i_084962),
    (0x084968, i_084968),
    (0x08496E, i_08496e),
    (0x084974, i_084974),
    (0x08497A, i_08497a),
    (0x084980, i_084980),
    (0x084986, i_084986),
    (0x08498A, i_08498a),
    (0x084990, i_084990),
    (0x084996, i_084996),
    (0x08499C, i_08499c),
    (0x0849A2, i_0849a2),
    (0x0849A6, i_0849a6),
    (0x0849A8, i_0849a8),
    (0x0849AE, i_0849ae),
    (0x0849B4, i_0849b4),
    (0x0849B8, i_0849b8),
    (0x0849BA, i_0849ba),
    (0x0849C0, i_0849c0),
    (0x0849C6, i_0849c6),
    (0x0849CA, i_0849ca),
    (0x0849CE, i_0849ce),
    (0x0849D2, i_0849d2),
    (0x0849D8, i_0849d8),
    (0x0849DE, i_0849de),
    (0x0849E4, i_0849e4),
    (0x0849EA, i_0849ea),
    (0x0849F0, i_0849f0),
    (0x0849F6, i_0849f6),
    (0x0849FC, i_0849fc),
    (0x084A02, i_084a02),
    (0x084A0A, i_084a0a),
    (0x084A0E, i_084a0e),
    (0x084A10, i_084a10),
    (0x084A16, i_084a16),
    (0x084A1C, i_084a1c),
    (0x084A22, i_084a22),
    (0x084A28, i_084a28),
    (0x084A2C, i_084a2c),
    (0x084A32, i_084a32),
    (0x084A38, i_084a38),
    (0x084A3E, i_084a3e),
    (0x084A44, i_084a44),
    (0x084A46, i_084a46),
    (0x084A4A, i_084a4a),
    (0x084A4E, i_084a4e),
    (0x084A54, i_084a54),
    (0x084A58, i_084a58),
    (0x084A5E, i_084a5e),
    (0x084A62, i_084a62),
    (0x084A68, i_084a68),
    (0x084A6C, i_084a6c),
    (0x084A72, i_084a72),
    (0x084A78, i_084a78),
    (0x084A7E, i_084a7e),
    (0x084A84, i_084a84),
    (0x084A8A, i_084a8a),
    (0x084A90, i_084a90),
    (0x084A96, i_084a96),
    (0x084A9C, i_084a9c),
    (0x084AA2, i_084aa2),
    (0x084AA8, i_084aa8),
    (0x084AAA, i_084aaa),
    (0x084AAE, i_084aae),
    (0x084AB4, i_084ab4),
    (0x084ABA, i_084aba),
    (0x084AC0, i_084ac0),
    (0x084AC6, i_084ac6),
    (0x084ACC, i_084acc),
    (0x084AD2, i_084ad2),
    (0x084AD8, i_084ad8),
    (0x084ADE, i_084ade),
    (0x084AE4, i_084ae4),
    (0x084AE8, i_084ae8),
    (0x084AEC, i_084aec),
    (0x084AF0, i_084af0),
    (0x084AF2, i_084af2),
    (0x084AF8, i_084af8),
    (0x084AFE, i_084afe),
    (0x084B02, i_084b02),
    (0x084B08, i_084b08),
    (0x084B0C, i_084b0c),
    (0x084B10, i_084b10),
    (0x084B16, i_084b16),
    (0x084B1C, i_084b1c),
    (0x084B20, i_084b20),
    (0x084B22, i_084b22),
    (0x084B24, i_084b24),
    (0x084B2A, i_084b2a),
    (0x084B30, i_084b30),
    (0x084B36, i_084b36),
    (0x084B3C, i_084b3c),
    (0x084B40, i_084b40),
    (0x084B44, i_084b44),
    (0x084B48, i_084b48),
    (0x084B4A, i_084b4a),
    (0x084B50, i_084b50),
    (0x084B54, i_084b54),
    (0x084B58, i_084b58),
    (0x084B5A, i_084b5a),
    (0x084B60, i_084b60),
    (0x084B62, i_084b62),
    (0x084B68, i_084b68),
    (0x084B6E, i_084b6e),
    (0x084B74, i_084b74),
    (0x084B78, i_084b78),
    (0x084B7C, i_084b7c),
    (0x084B80, i_084b80),
    (0x084B82, i_084b82),
    (0x084B88, i_084b88),
    (0x084B8E, i_084b8e),
    (0x084B92, i_084b92),
    (0x084B96, i_084b96),
    (0x084B98, i_084b98),
    (0x084B9A, i_084b9a),
    (0x084BA0, i_084ba0),
    (0x084BA6, i_084ba6),
    (0x084BAC, i_084bac),
    (0x084BB0, i_084bb0),
    (0x084BB4, i_084bb4),
    (0x084BB8, i_084bb8),
    (0x084BBA, i_084bba),
    (0x084BC0, i_084bc0),
    (0x084BC6, i_084bc6),
    (0x084BCA, i_084bca),
    (0x084BCE, i_084bce),
    (0x084BD0, i_084bd0),
    (0x084BD2, i_084bd2),
    (0x084BD8, i_084bd8),
    (0x084BDE, i_084bde),
    (0x084BE4, i_084be4),
    (0x084BEA, i_084bea),
    (0x084BF0, i_084bf0),
    (0x084BF4, i_084bf4),
    (0x084BF8, i_084bf8),
    (0x084BFC, i_084bfc),
    (0x084BFE, i_084bfe),
    (0x084C04, i_084c04),
    (0x084C0A, i_084c0a),
    (0x084C10, i_084c10),
    (0x084C14, i_084c14),
    (0x084C18, i_084c18),
    (0x084C1E, i_084c1e),
    (0x084C22, i_084c22),
    (0x084C24, i_084c24),
    (0x084C26, i_084c26),
    (0x084C2A, i_084c2a),
    (0x084C2C, i_084c2c),
    (0x084C32, i_084c32),
    (0x084C38, i_084c38),
    (0x084C3E, i_084c3e),
    (0x084C42, i_084c42),
    (0x084C46, i_084c46),
    (0x084C4A, i_084c4a),
    (0x084C50, i_084c50),
    (0x084C56, i_084c56),
    (0x084C5C, i_084c5c),
    (0x084C5E, i_084c5e),
    (0x084C62, i_084c62),
    (0x084C68, i_084c68),
    (0x084C6E, i_084c6e),
    (0x084C74, i_084c74),
    (0x084C7A, i_084c7a),
    (0x084C80, i_084c80),
    (0x084C84, i_084c84),
    (0x084C86, i_084c86),
    (0x084C8A, i_084c8a),
    (0x084C90, i_084c90),
    (0x084C96, i_084c96),
    (0x084C9C, i_084c9c),
    (0x084CA2, i_084ca2),
    (0x084CA4, i_084ca4),
    (0x084CA8, i_084ca8),
    (0x084CAE, i_084cae),
    (0x084CB4, i_084cb4),
    (0x084CBA, i_084cba),
    (0x084CBE, i_084cbe),
    (0x084CC2, i_084cc2),
    (0x084CC8, i_084cc8),
    (0x084CCC, i_084ccc),
    (0x084CD0, i_084cd0),
    (0x084CD2, i_084cd2),
    (0x084CD8, i_084cd8),
    (0x084CDE, i_084cde),
    (0x084CE2, i_084ce2),
    (0x084CE8, i_084ce8),
    (0x084CEE, i_084cee),
    (0x084CF4, i_084cf4),
    (0x084CFA, i_084cfa),
    (0x084CFE, i_084cfe),
    (0x084D04, i_084d04),
    (0x084D0A, i_084d0a),
    (0x084D0E, i_084d0e),
    (0x084D14, i_084d14),
    (0x084D18, i_084d18),
    (0x084D1C, i_084d1c),
    (0x084D20, i_084d20),
    (0x084D26, i_084d26),
    (0x084D2C, i_084d2c),
    (0x084D30, i_084d30),
    (0x084D36, i_084d36),
    (0x084D3A, i_084d3a),
    (0x084D40, i_084d40),
    (0x084D42, i_084d42),
    (0x084D46, i_084d46),
    (0x084D4C, i_084d4c),
    (0x084D50, i_084d50),
    (0x084D56, i_084d56),
    (0x084D5C, i_084d5c),
    (0x084D62, i_084d62),
    (0x084D68, i_084d68),
    (0x084D6E, i_084d6e),
    (0x084D74, i_084d74),
    (0x084D7A, i_084d7a),
    (0x084D80, i_084d80),
    (0x084D86, i_084d86),
    (0x084D8C, i_084d8c),
    (0x084D90, i_084d90),
    (0x084D92, i_084d92),
    (0x084D98, i_084d98),
    (0x084D9E, i_084d9e),
    (0x084DA4, i_084da4),
    (0x084DA8, i_084da8),
    (0x084DAE, i_084dae),
    (0x084DB0, i_084db0),
    (0x084DB6, i_084db6),
    (0x084DBC, i_084dbc),
    (0x084DC2, i_084dc2),
    (0x084DC8, i_084dc8),
    (0x084DCC, i_084dcc),
    (0x084DD2, i_084dd2),
    (0x084DD6, i_084dd6),
    (0x084DDC, i_084ddc),
    (0x084DE0, i_084de0),
    (0x084DE6, i_084de6),
    (0x084DEA, i_084dea),
    (0x084DEE, i_084dee),
    (0x084DF4, i_084df4),
    (0x084DF8, i_084df8),
    (0x084DFE, i_084dfe),
    (0x084E02, i_084e02),
    (0x084E08, i_084e08),
    (0x084E0C, i_084e0c),
    (0x084E12, i_084e12),
    (0x084E16, i_084e16),
    (0x084E1A, i_084e1a),
    (0x084E20, i_084e20),
    (0x084E24, i_084e24),
    (0x084E2A, i_084e2a),
    (0x084E2E, i_084e2e),
    (0x084E34, i_084e34),
    (0x084E38, i_084e38),
    (0x084E3A, i_084e3a),
    (0x084E3E, i_084e3e),
    (0x084E42, i_084e42),
    (0x084E48, i_084e48),
    (0x084E4E, i_084e4e),
    (0x084E54, i_084e54),
    (0x084E58, i_084e58),
    (0x084E5A, i_084e5a),
    (0x084E60, i_084e60),
    (0x084E66, i_084e66),
    (0x084E6C, i_084e6c),
    (0x084E70, i_084e70),
    (0x084E76, i_084e76),
    (0x084E7C, i_084e7c),
    (0x084E82, i_084e82),
    (0x084E88, i_084e88),
    (0x084E8C, i_084e8c),
    (0x084E92, i_084e92),
    (0x084E96, i_084e96),
    (0x084E9A, i_084e9a),
    (0x084EA0, i_084ea0),
    (0x084EA6, i_084ea6),
    (0x084EAA, i_084eaa),
    (0x084EAC, i_084eac),
    (0x084EB0, i_084eb0),
    (0x084EB6, i_084eb6),
    (0x084EBA, i_084eba),
    (0x084EC0, i_084ec0),
    (0x084EC2, i_084ec2),
    (0x084EC6, i_084ec6),
    (0x084ECC, i_084ecc),
    (0x084ED2, i_084ed2),
    (0x084ED8, i_084ed8),
    (0x084EDE, i_084ede),
    (0x084EE4, i_084ee4),
    (0x084EEA, i_084eea),
    (0x084EEE, i_084eee),
    (0x084EF0, i_084ef0),
    (0x084EF6, i_084ef6),
    (0x084EFC, i_084efc),
    (0x084F02, i_084f02),
    (0x084F08, i_084f08),
    (0x084F0E, i_084f0e),
    (0x084F14, i_084f14),
    (0x084F1A, i_084f1a),
    (0x084F1E, i_084f1e),
    (0x084F24, i_084f24),
    (0x084F26, i_084f26),
    (0x084F2A, i_084f2a),
    (0x084F30, i_084f30),
    (0x084F36, i_084f36),
    (0x084F3C, i_084f3c),
    (0x084F42, i_084f42),
    (0x084F46, i_084f46),
    (0x084F4A, i_084f4a),
    (0x084F4E, i_084f4e),
    (0x084F52, i_084f52),
    (0x084F54, i_084f54),
    (0x084F56, i_084f56),
    (0x084F5A, i_084f5a),
    (0x084F5C, i_084f5c),
    (0x084F5E, i_084f5e),
    (0x084F62, i_084f62),
    (0x084F68, i_084f68),
    (0x084F6E, i_084f6e),
    (0x084F74, i_084f74),
    (0x084F76, i_084f76),
    (0x084F7A, i_084f7a),
    (0x084F7C, i_084f7c),
    (0x084F80, i_084f80),
    (0x084F84, i_084f84),
    (0x084F86, i_084f86),
    (0x084F8C, i_084f8c),
    (0x084F90, i_084f90),
    (0x084F94, i_084f94),
    (0x084F9A, i_084f9a),
    (0x084F9E, i_084f9e),
    (0x084FA2, i_084fa2),
    (0x084FA6, i_084fa6),
    (0x084FAA, i_084faa),
    (0x084FAC, i_084fac),
    (0x084FB2, i_084fb2),
    (0x084FB8, i_084fb8),
    (0x084FBE, i_084fbe),
    (0x084FC2, i_084fc2),
    (0x084FC8, i_084fc8),
    (0x084FCA, i_084fca),
    (0x084FCE, i_084fce),
    (0x084FD4, i_084fd4),
    (0x084FDA, i_084fda),
    (0x084FE0, i_084fe0),
    (0x084FE2, i_084fe2),
    (0x084FE6, i_084fe6),
    (0x084FE8, i_084fe8),
    (0x084FEA, i_084fea),
    (0x084FEE, i_084fee),
    (0x084FF4, i_084ff4),
    (0x084FF6, i_084ff6),
    (0x084FFA, i_084ffa),
    (0x084FFE, i_084ffe),
    (0x085004, i_085004),
    (0x085008, i_085008),
    (0x08500C, i_08500c),
    (0x085012, i_085012),
    (0x085018, i_085018),
    (0x08501C, i_08501c),
    (0x08501E, i_08501e),
    (0x085024, i_085024),
    (0x085028, i_085028),
    (0x08502C, i_08502c),
    (0x085032, i_085032),
    (0x085036, i_085036),
    (0x08503A, i_08503a),
    (0x08503E, i_08503e),
    (0x085042, i_085042),
    (0x085044, i_085044),
    (0x08504A, i_08504a),
    (0x085050, i_085050),
    (0x085056, i_085056),
    (0x08505A, i_08505a),
    (0x085060, i_085060),
    (0x085062, i_085062),
    (0x085066, i_085066),
    (0x08506C, i_08506c),
    (0x085072, i_085072),
    (0x085078, i_085078),
    (0x08507E, i_08507e),
    (0x085082, i_085082),
    (0x085084, i_085084),
    (0x08508A, i_08508a),
    (0x08508E, i_08508e),
    (0x085094, i_085094),
    (0x085098, i_085098),
    (0x08509E, i_08509e),
    (0x0850A2, i_0850a2),
    (0x0850A6, i_0850a6),
    (0x0850A8, i_0850a8),
    (0x0850AE, i_0850ae),
    (0x0850B4, i_0850b4),
    (0x0850BA, i_0850ba),
    (0x0850BE, i_0850be),
    (0x0850C4, i_0850c4),
    (0x0850C6, i_0850c6),
    (0x0850CC, i_0850cc),
    (0x0850D2, i_0850d2),
    (0x0850D8, i_0850d8),
    (0x0850DC, i_0850dc),
    (0x0850DE, i_0850de),
    (0x0850E4, i_0850e4),
    (0x0850EA, i_0850ea),
    (0x0850EE, i_0850ee),
    (0x0850F2, i_0850f2),
    (0x0850F6, i_0850f6),
    (0x0850FA, i_0850fa),
    (0x0850FE, i_0850fe),
    (0x085102, i_085102),
    (0x085104, i_085104),
    (0x085108, i_085108),
    (0x08510C, i_08510c),
    (0x08510E, i_08510e),
    (0x085114, i_085114),
    (0x08511A, i_08511a),
    (0x085120, i_085120),
    (0x085124, i_085124),
    (0x08512A, i_08512a),
    (0x08512C, i_08512c),
    (0x085130, i_085130),
    (0x085132, i_085132),
    (0x085134, i_085134),
    (0x08513A, i_08513a),
    (0x085140, i_085140),
    (0x085146, i_085146),
    (0x08514C, i_08514c),
    (0x085150, i_085150),
    (0x085154, i_085154),
    (0x085158, i_085158),
    (0x08515E, i_08515e),
    (0x085162, i_085162),
    (0x085166, i_085166),
    (0x08516E, i_08516e),
    (0x085176, i_085176),
    (0x08517C, i_08517c),
    (0x085182, i_085182),
    (0x085188, i_085188),
    (0x08518E, i_08518e),
    (0x085190, i_085190),
    (0x085194, i_085194),
    (0x08519A, i_08519a),
    (0x0851A0, i_0851a0),
    (0x0851A6, i_0851a6),
    (0x0851AA, i_0851aa),
    (0x0851AE, i_0851ae),
    (0x0851B4, i_0851b4),
    (0x0851BA, i_0851ba),
    (0x0851C0, i_0851c0),
    (0x0851C6, i_0851c6),
    (0x0851CC, i_0851cc),
    (0x0851D0, i_0851d0),
    (0x0851D2, i_0851d2),
    (0x0851D8, i_0851d8),
    (0x0851DA, i_0851da),
    (0x0851DE, i_0851de),
    (0x0851E2, i_0851e2),
    (0x0851E8, i_0851e8),
    (0x0851EE, i_0851ee),
    (0x0851F0, i_0851f0),
    (0x0851F4, i_0851f4),
    (0x0851FA, i_0851fa),
    (0x085200, i_085200),
    (0x085204, i_085204),
    (0x08520A, i_08520a),
    (0x085210, i_085210),
    (0x085214, i_085214),
    (0x085216, i_085216),
    (0x08521C, i_08521c),
    (0x085222, i_085222),
    (0x085228, i_085228),
    (0x08522E, i_08522e),
    (0x085232, i_085232),
    (0x085238, i_085238),
    (0x08523E, i_08523e),
    (0x085244, i_085244),
    (0x08524A, i_08524a),
    (0x085250, i_085250),
    (0x085256, i_085256),
    (0x08525C, i_08525c),
    (0x085262, i_085262),
    (0x085266, i_085266),
    (0x085268, i_085268),
    (0x08526E, i_08526e),
    (0x085270, i_085270),
    (0x085274, i_085274),
    (0x08527A, i_08527a),
    (0x085280, i_085280),
    (0x085286, i_085286),
    (0x08528A, i_08528a),
    (0x08528C, i_08528c),
    (0x085292, i_085292),
    (0x085298, i_085298),
    (0x08529E, i_08529e),
    (0x0852A4, i_0852a4),
    (0x0852A8, i_0852a8),
    (0x0852AE, i_0852ae),
    (0x0852B4, i_0852b4),
    (0x0852BA, i_0852ba),
    (0x0852C0, i_0852c0),
    (0x0852C6, i_0852c6),
    (0x0852CC, i_0852cc),
    (0x0852D2, i_0852d2),
    (0x0852D6, i_0852d6),
    (0x0852DA, i_0852da),
    (0x0852DE, i_0852de),
    (0x0852E4, i_0852e4),
    (0x0852EA, i_0852ea),
    (0x0852F0, i_0852f0),
    (0x0852F6, i_0852f6),
    (0x0852FC, i_0852fc),
    (0x085302, i_085302),
    (0x085308, i_085308),
    (0x08530E, i_08530e),
    (0x085314, i_085314),
    (0x085318, i_085318),
    (0x08531E, i_08531e),
    (0x085324, i_085324),
    (0x08532A, i_08532a),
    (0x08532E, i_08532e),
    (0x085330, i_085330),
    (0x085336, i_085336),
    (0x085338, i_085338),
    (0x08533C, i_08533c),
    (0x085340, i_085340),
    (0x085346, i_085346),
    (0x08534A, i_08534a),
    (0x085350, i_085350),
    (0x085354, i_085354),
    (0x085358, i_085358),
    (0x08535E, i_08535e),
    (0x085364, i_085364),
    (0x08536A, i_08536a),
    (0x085370, i_085370),
    (0x085376, i_085376),
    (0x08537A, i_08537a),
    (0x08537C, i_08537c),
    (0x085382, i_085382),
    (0x085388, i_085388),
    (0x08538C, i_08538c),
    (0x085392, i_085392),
    (0x085394, i_085394),
    (0x085398, i_085398),
    (0x08539E, i_08539e),
    (0x0853A2, i_0853a2),
    (0x0853A8, i_0853a8),
    (0x0853AE, i_0853ae),
    (0x0853B2, i_0853b2),
    (0x0853B6, i_0853b6),
    (0x0853BC, i_0853bc),
    (0x0853C0, i_0853c0),
    (0x0853C4, i_0853c4),
    (0x0853C8, i_0853c8),
    (0x0853CE, i_0853ce),
    (0x0853D4, i_0853d4),
    (0x0853DA, i_0853da),
    (0x0853E0, i_0853e0),
    (0x0853E8, i_0853e8),
    (0x0853EC, i_0853ec),
    (0x0853EE, i_0853ee),
    (0x0853F4, i_0853f4),
    (0x0853F6, i_0853f6),
    (0x0853FA, i_0853fa),
    (0x0853FE, i_0853fe),
    (0x085404, i_085404),
    (0x08540A, i_08540a),
    (0x085410, i_085410),
    (0x085412, i_085412),
    (0x085418, i_085418),
    (0x08541E, i_08541e),
    (0x085422, i_085422),
    (0x085426, i_085426),
    (0x08542C, i_08542c),
    (0x085430, i_085430),
    (0x085436, i_085436),
    (0x08543A, i_08543a),
    (0x08543C, i_08543c),
    (0x085442, i_085442),
    (0x085448, i_085448),
    (0x08544C, i_08544c),
    (0x085450, i_085450),
    (0x085456, i_085456),
    (0x08545A, i_08545a),
    (0x08545E, i_08545e),
    (0x085464, i_085464),
    (0x085468, i_085468),
    (0x08546C, i_08546c),
    (0x085470, i_085470),
    (0x085476, i_085476),
    (0x08547C, i_08547c),
    (0x085480, i_085480),
    (0x085482, i_085482),
    (0x085484, i_085484),
    (0x085488, i_085488),
    (0x08548C, i_08548c),
    (0x085490, i_085490),
    (0x085496, i_085496),
    (0x08549C, i_08549c),
    (0x0854A2, i_0854a2),
    (0x0854A8, i_0854a8),
    (0x0854AE, i_0854ae),
    (0x0854B4, i_0854b4),
    (0x0854BA, i_0854ba),
    (0x0854BE, i_0854be),
    (0x0854C0, i_0854c0),
    (0x0854C6, i_0854c6),
    (0x0854CC, i_0854cc),
    (0x0854D2, i_0854d2),
    (0x0854D6, i_0854d6),
    (0x0854DA, i_0854da),
    (0x0854E0, i_0854e0),
    (0x0854E6, i_0854e6),
    (0x0854E8, i_0854e8),
    (0x0854EE, i_0854ee),
    (0x0854F2, i_0854f2),
    (0x0854F4, i_0854f4),
    (0x0854FA, i_0854fa),
    (0x085500, i_085500),
    (0x085504, i_085504),
    (0x08550A, i_08550a),
    (0x085510, i_085510),
    (0x085516, i_085516),
    (0x08551A, i_08551a),
    (0x08551E, i_08551e),
    (0x085524, i_085524),
    (0x085526, i_085526),
    (0x08552C, i_08552c),
    (0x085532, i_085532),
    (0x085536, i_085536),
    (0x085538, i_085538),
    (0x08553C, i_08553c),
    (0x085542, i_085542),
    (0x085548, i_085548),
    (0x08554E, i_08554e),
    (0x085554, i_085554),
    (0x085558, i_085558),
    (0x08555E, i_08555e),
    (0x085564, i_085564),
    (0x08556A, i_08556a),
    (0x08556E, i_08556e),
    (0x085572, i_085572),
    (0x085578, i_085578),
    (0x08557C, i_08557c),
    (0x085582, i_085582),
    (0x085584, i_085584),
    (0x085588, i_085588),
    (0x08558C, i_08558c),
    (0x085592, i_085592),
    (0x085596, i_085596),
    (0x08559A, i_08559a),
    (0x0855A0, i_0855a0),
    (0x0855A4, i_0855a4),
    (0x0855A6, i_0855a6),
    (0x0855AC, i_0855ac),
    (0x0855B0, i_0855b0),
    (0x0855B6, i_0855b6),
    (0x0855BA, i_0855ba),
    (0x0855C2, i_0855c2),
    (0x0855C8, i_0855c8),
    (0x0855CC, i_0855cc),
    (0x0855D2, i_0855d2),
    (0x0855D6, i_0855d6),
    (0x0855DA, i_0855da),
    (0x0855DE, i_0855de),
    (0x0855E4, i_0855e4),
    (0x0855EA, i_0855ea),
    (0x0855F0, i_0855f0),
    (0x0855F6, i_0855f6),
    (0x0855FC, i_0855fc),
    (0x085602, i_085602),
    (0x085606, i_085606),
    (0x085608, i_085608),
    (0x08560C, i_08560c),
    (0x085610, i_085610),
    (0x085616, i_085616),
    (0x08561A, i_08561a),
    (0x08561E, i_08561e),
    (0x085624, i_085624),
    (0x085628, i_085628),
    (0x08562A, i_08562a),
    (0x085630, i_085630),
    (0x085634, i_085634),
    (0x08563A, i_08563a),
    (0x08563E, i_08563e),
    (0x085646, i_085646),
    (0x08564C, i_08564c),
    (0x085650, i_085650),
    (0x085656, i_085656),
    (0x08565A, i_08565a),
    (0x08565E, i_08565e),
    (0x085662, i_085662),
    (0x085668, i_085668),
    (0x08566E, i_08566e),
    (0x085674, i_085674),
    (0x08567A, i_08567a),
    (0x085680, i_085680),
    (0x085686, i_085686),
    (0x08568C, i_08568c),
    (0x085692, i_085692),
    (0x085698, i_085698),
    (0x08569E, i_08569e),
    (0x0856A4, i_0856a4),
    (0x0856A8, i_0856a8),
    (0x0856AA, i_0856aa),
    (0x0856AE, i_0856ae),
    (0x0856B4, i_0856b4),
    (0x0856B8, i_0856b8),
    (0x0856BE, i_0856be),
    (0x0856C4, i_0856c4),
    (0x0856CA, i_0856ca),
    (0x0856CC, i_0856cc),
    (0x0856D0, i_0856d0),
    (0x0856D6, i_0856d6),
    (0x0856D8, i_0856d8),
    (0x0856DC, i_0856dc),
    (0x0856E2, i_0856e2),
    (0x0856E6, i_0856e6),
    (0x0856EC, i_0856ec),
    (0x0856F2, i_0856f2),
    (0x0856F6, i_0856f6),
    (0x0856F8, i_0856f8),
    (0x0856FE, i_0856fe),
    (0x085704, i_085704),
    (0x085708, i_085708),
    (0x08570C, i_08570c),
    (0x085712, i_085712),
    (0x085716, i_085716),
    (0x08571C, i_08571c),
    (0x085720, i_085720),
    (0x085724, i_085724),
    (0x085728, i_085728),
    (0x08572E, i_08572e),
    (0x085732, i_085732),
    (0x085738, i_085738),
    (0x08573A, i_08573a),
    (0x08573E, i_08573e),
    (0x085744, i_085744),
    (0x08574A, i_08574a),
    (0x085750, i_085750),
    (0x085756, i_085756),
    (0x08575C, i_08575c),
    (0x085760, i_085760),
    (0x085762, i_085762),
    (0x085768, i_085768),
    (0x08576E, i_08576e),
    (0x085772, i_085772),
    (0x085778, i_085778),
    (0x08577A, i_08577a),
    (0x08577C, i_08577c),
    (0x085780, i_085780),
    (0x085784, i_085784),
    (0x08578A, i_08578a),
    (0x085790, i_085790),
    (0x085794, i_085794),
    (0x085798, i_085798),
    (0x08579E, i_08579e),
    (0x0857A0, i_0857a0),
    (0x0857A4, i_0857a4),
    (0x0857A6, i_0857a6),
    (0x0857AA, i_0857aa),
    (0x0857AC, i_0857ac),
    (0x0857B2, i_0857b2),
    (0x0857B4, i_0857b4),
    (0x0857B8, i_0857b8),
    (0x0857BA, i_0857ba),
    (0x0857C0, i_0857c0),
    (0x0857C6, i_0857c6),
    (0x0857CC, i_0857cc),
    (0x0857D2, i_0857d2),
    (0x0857D6, i_0857d6),
    (0x0857DC, i_0857dc),
    (0x0857E2, i_0857e2),
    (0x0857E6, i_0857e6),
    (0x0857EA, i_0857ea),
    (0x0857F0, i_0857f0),
    (0x0857F2, i_0857f2),
    (0x0857F6, i_0857f6),
    (0x0857FC, i_0857fc),
    (0x085802, i_085802),
    (0x085806, i_085806),
    (0x08580C, i_08580c),
    (0x085812, i_085812),
    (0x085816, i_085816),
    (0x085818, i_085818),
    (0x08581E, i_08581e),
    (0x085820, i_085820),
    (0x085826, i_085826),
    (0x08582A, i_08582a),
    (0x085830, i_085830),
    (0x085836, i_085836),
    (0x08583C, i_08583c),
    (0x085840, i_085840),
    (0x085842, i_085842),
    (0x085848, i_085848),
    (0x08584E, i_08584e),
    (0x085854, i_085854),
    (0x08585A, i_08585a),
    (0x085860, i_085860),
    (0x085864, i_085864),
    (0x08586C, i_08586c),
    (0x085872, i_085872),
    (0x085876, i_085876),
    (0x08587C, i_08587c),
    (0x08587E, i_08587e),
    (0x085884, i_085884),
    (0x085888, i_085888),
    (0x08588E, i_08588e),
    (0x085890, i_085890),
    (0x085896, i_085896),
    (0x08589C, i_08589c),
    (0x0858A0, i_0858a0),
    (0x0858A6, i_0858a6),
    (0x0858AA, i_0858aa),
    (0x0858AE, i_0858ae),
    (0x0858B2, i_0858b2),
    (0x0858B8, i_0858b8),
    (0x0858BC, i_0858bc),
    (0x0858C2, i_0858c2),
    (0x0858C6, i_0858c6),
    (0x0858CA, i_0858ca),
    (0x0858CE, i_0858ce),
    (0x0858D0, i_0858d0),
    (0x0858D4, i_0858d4),
    (0x0858DA, i_0858da),
    (0x0858E0, i_0858e0),
    (0x0858E6, i_0858e6),
    (0x0858EA, i_0858ea),
    (0x0858EE, i_0858ee),
    (0x0858F2, i_0858f2),
    (0x0858F4, i_0858f4),
    (0x0858F8, i_0858f8),
    (0x0858FC, i_0858fc),
    (0x085902, i_085902),
    (0x085904, i_085904),
    (0x085908, i_085908),
    (0x08590E, i_08590e),
    (0x085912, i_085912),
    (0x085918, i_085918),
    (0x08591C, i_08591c),
    (0x08591E, i_08591e),
    (0x085924, i_085924),
    (0x08592A, i_08592a),
    (0x085930, i_085930),
    (0x085936, i_085936),
    (0x08593A, i_08593a),
    (0x08593E, i_08593e),
    (0x085940, i_085940),
    (0x085946, i_085946),
    (0x08594C, i_08594c),
    (0x085952, i_085952),
    (0x085956, i_085956),
    (0x08595C, i_08595c),
    (0x08595E, i_08595e),
    (0x085960, i_085960),
    (0x085964, i_085964),
    (0x085968, i_085968),
    (0x08596E, i_08596e),
    (0x085970, i_085970),
    (0x085974, i_085974),
    (0x08597A, i_08597a),
    (0x08597E, i_08597e),
    (0x085984, i_085984),
    (0x085988, i_085988),
    (0x08598A, i_08598a),
    (0x085990, i_085990),
    (0x085996, i_085996),
    (0x08599A, i_08599a),
    (0x08599E, i_08599e),
    (0x0859A0, i_0859a0),
    (0x0859A6, i_0859a6),
    (0x0859AC, i_0859ac),
    (0x0859B0, i_0859b0),
    (0x0859B4, i_0859b4),
    (0x0859B6, i_0859b6),
    (0x0859BC, i_0859bc),
    (0x0859C2, i_0859c2),
    (0x0859C8, i_0859c8),
    (0x0859CC, i_0859cc),
    (0x0859D2, i_0859d2),
    (0x0859D4, i_0859d4),
    (0x0859DA, i_0859da),
    (0x0859E2, i_0859e2),
    (0x0859EA, i_0859ea),
    (0x0859F0, i_0859f0),
    (0x0859F8, i_0859f8),
    (0x0859FC, i_0859fc),
    (0x085A02, i_085a02),
    (0x085A08, i_085a08),
    (0x085A0A, i_085a0a),
    (0x085A0E, i_085a0e),
    (0x085A10, i_085a10),
    (0x085A16, i_085a16),
    (0x085A1A, i_085a1a),
    (0x085A1E, i_085a1e),
    (0x085A22, i_085a22),
    (0x085A2A, i_085a2a),
    (0x085A30, i_085a30),
    (0x085A34, i_085a34),
    (0x085A38, i_085a38),
    (0x085A3A, i_085a3a),
    (0x085A3E, i_085a3e),
    (0x085A42, i_085a42),
    (0x085A46, i_085a46),
    (0x085A4A, i_085a4a),
    (0x085A50, i_085a50),
    (0x085A56, i_085a56),
    (0x085A5C, i_085a5c),
    (0x085A60, i_085a60),
    (0x085A66, i_085a66),
    (0x085A6C, i_085a6c),
    (0x085A72, i_085a72),
    (0x085A76, i_085a76),
    (0x085A7A, i_085a7a),
    (0x085A7E, i_085a7e),
    (0x085A82, i_085a82),
    (0x085A88, i_085a88),
    (0x085A90, i_085a90),
    (0x085A94, i_085a94),
    (0x085A9A, i_085a9a),
    (0x085A9E, i_085a9e),
    (0x085AA2, i_085aa2),
    (0x085AA6, i_085aa6),
    (0x085AAC, i_085aac),
    (0x085AB0, i_085ab0),
    (0x085AB4, i_085ab4),
    (0x085AB6, i_085ab6),
    (0x085ABC, i_085abc),
    (0x085AC2, i_085ac2),
    (0x085AC6, i_085ac6),
    (0x085ACA, i_085aca),
    (0x085ACC, i_085acc),
    (0x085ACE, i_085ace),
    (0x085AD4, i_085ad4),
    (0x085AD8, i_085ad8),
    (0x085ADC, i_085adc),
    (0x085AE0, i_085ae0),
    (0x085AE4, i_085ae4),
    (0x085AEA, i_085aea),
    (0x085AEE, i_085aee),
    (0x085AF2, i_085af2),
    (0x085AF6, i_085af6),
    (0x085AFA, i_085afa),
    (0x085AFE, i_085afe),
    (0x085B02, i_085b02),
    (0x085B06, i_085b06),
    (0x085B0A, i_085b0a),
    (0x085B10, i_085b10),
    (0x085B14, i_085b14),
    (0x085B18, i_085b18),
    (0x085B1C, i_085b1c),
    (0x085B20, i_085b20),
    (0x085B24, i_085b24),
    (0x085B28, i_085b28),
    (0x085B2E, i_085b2e),
    (0x085B32, i_085b32),
    (0x085B36, i_085b36),
    (0x085B3A, i_085b3a),
    (0x085B3E, i_085b3e),
    (0x085B42, i_085b42),
    (0x085B48, i_085b48),
    (0x085B4C, i_085b4c),
    (0x085B50, i_085b50),
    (0x085B54, i_085b54),
    (0x085B58, i_085b58),
    (0x085B5C, i_085b5c),
    (0x085B60, i_085b60),
    (0x085B62, i_085b62),
    (0x085B66, i_085b66),
    (0x085B68, i_085b68),
    (0x085B6E, i_085b6e),
    (0x085B72, i_085b72),
    (0x085B76, i_085b76),
    (0x085B78, i_085b78),
    (0x085B7C, i_085b7c),
    (0x085B7E, i_085b7e),
    (0x085B82, i_085b82),
    (0x085B86, i_085b86),
    (0x085B8A, i_085b8a),
    (0x085B8E, i_085b8e),
    (0x085B94, i_085b94),
    (0x085B98, i_085b98),
    (0x085B9C, i_085b9c),
    (0x085BA0, i_085ba0),
    (0x085BA4, i_085ba4),
    (0x085BAA, i_085baa),
    (0x085BAE, i_085bae),
    (0x085BB2, i_085bb2),
    (0x085BB6, i_085bb6),
    (0x085BBA, i_085bba),
    (0x085BC0, i_085bc0),
    (0x085BC4, i_085bc4),
    (0x085BC8, i_085bc8),
    (0x085BCC, i_085bcc),
    (0x085BD0, i_085bd0),
    (0x085BD6, i_085bd6),
    (0x085C2A, i_085c2a),
    (0x085C30, i_085c30),
    (0x085C32, i_085c32),
    (0x085C36, i_085c36),
    (0x085C3C, i_085c3c),
    (0x085C40, i_085c40),
    (0x085C42, i_085c42),
    (0x085C46, i_085c46),
    (0x085C4A, i_085c4a),
    (0x085C4E, i_085c4e),
    (0x085C52, i_085c52),
    (0x085C54, i_085c54),
    (0x085C58, i_085c58),
    (0x085C5C, i_085c5c),
    (0x085C5E, i_085c5e),
    (0x085C62, i_085c62),
    (0x085C66, i_085c66),
    (0x085C6C, i_085c6c),
    (0x085C6E, i_085c6e),
    (0x085C72, i_085c72),
    (0x085C78, i_085c78),
    (0x085C7C, i_085c7c),
    (0x085C82, i_085c82),
    (0x085C86, i_085c86),
    (0x085C8C, i_085c8c),
    (0x085C90, i_085c90),
    (0x085C94, i_085c94),
    (0x085C96, i_085c96),
    (0x085C9A, i_085c9a),
    (0x085C9E, i_085c9e),
    (0x085CA0, i_085ca0),
    (0x085CA4, i_085ca4),
    (0x085CA8, i_085ca8),
    (0x085CAE, i_085cae),
    (0x085CB0, i_085cb0),
    (0x085CB4, i_085cb4),
    (0x085CBA, i_085cba),
    (0x085CBE, i_085cbe),
    (0x085CC4, i_085cc4),
    (0x085CD6, i_085cd6),
    (0x085CD8, i_085cd8),
    (0x085CDC, i_085cdc),
    (0x085CE0, i_085ce0),
    (0x085CE6, i_085ce6),
    (0x085CE8, i_085ce8),
    (0x085CEC, i_085cec),
    (0x085CF2, i_085cf2),
    (0x085CF6, i_085cf6),
    (0x085CFC, i_085cfc),
    (0x085D02, i_085d02),
    (0x085D04, i_085d04),
    (0x085D0A, i_085d0a),
    (0x085D0C, i_085d0c),
    (0x085D10, i_085d10),
    (0x085D14, i_085d14),
    (0x085D18, i_085d18),
    (0x085D1C, i_085d1c),
    (0x085D1E, i_085d1e),
    (0x085D22, i_085d22),
    (0x085D28, i_085d28),
    (0x085D2A, i_085d2a),
    (0x085D30, i_085d30),
    (0x085D32, i_085d32),
    (0x085D36, i_085d36),
    (0x085D3A, i_085d3a),
    (0x085D40, i_085d40),
    (0x085D42, i_085d42),
    (0x085D46, i_085d46),
    (0x085D4A, i_085d4a),
    (0x085D4E, i_085d4e),
    (0x085D52, i_085d52),
    (0x085D56, i_085d56),
    (0x085D5A, i_085d5a),
    (0x085D5E, i_085d5e),
    (0x085D62, i_085d62),
    (0x085D68, i_085d68),
    (0x085D6E, i_085d6e),
    (0x085D74, i_085d74),
    (0x085D7A, i_085d7a),
    (0x085D80, i_085d80),
    (0x085D86, i_085d86),
    (0x085D8C, i_085d8c),
    (0x085D90, i_085d90),
    (0x085D96, i_085d96),
    (0x085D9C, i_085d9c),
    (0x085DA0, i_085da0),
    (0x085DA2, i_085da2),
    (0x085DA6, i_085da6),
    (0x085DA8, i_085da8),
    (0x085DAE, i_085dae),
    (0x085DB2, i_085db2),
    (0x085DB8, i_085db8),
    (0x085DBE, i_085dbe),
    (0x085DC2, i_085dc2),
    (0x085DC8, i_085dc8),
    (0x085DCE, i_085dce),
    (0x085DD2, i_085dd2),
    (0x085DD8, i_085dd8),
    (0x085DDE, i_085dde),
    (0x085DE4, i_085de4),
    (0x085DE8, i_085de8),
    (0x085DEE, i_085dee),
    (0x085DF2, i_085df2),
    (0x085DF6, i_085df6),
    (0x085DFC, i_085dfc),
    (0x085E00, i_085e00),
    (0x085E04, i_085e04),
    (0x085E08, i_085e08),
    (0x085E0A, i_085e0a),
    (0x085E0E, i_085e0e),
    (0x085E12, i_085e12),
    (0x085E14, i_085e14),
    (0x085E18, i_085e18),
    (0x085E1C, i_085e1c),
    (0x085E22, i_085e22),
    (0x085E26, i_085e26),
    (0x085E2A, i_085e2a),
    (0x085E30, i_085e30),
    (0x085E36, i_085e36),
    (0x085E38, i_085e38),
    (0x085E3A, i_085e3a),
    (0x085E3E, i_085e3e),
    (0x085E42, i_085e42),
    (0x085E44, i_085e44),
    (0x085E48, i_085e48),
    (0x085E4C, i_085e4c),
    (0x085E4E, i_085e4e),
    (0x085E52, i_085e52),
    (0x085E54, i_085e54),
    (0x085E58, i_085e58),
    (0x085E5C, i_085e5c),
    (0x085E62, i_085e62),
    (0x085E64, i_085e64),
    (0x085E6A, i_085e6a),
    (0x085E6E, i_085e6e),
    (0x085E72, i_085e72),
    (0x085E76, i_085e76),
    (0x085E7C, i_085e7c),
    (0x085E80, i_085e80),
    (0x085E84, i_085e84),
    (0x085E88, i_085e88),
    (0x085E8E, i_085e8e),
    (0x085E92, i_085e92),
    (0x085E98, i_085e98),
    (0x085E9C, i_085e9c),
    (0x085EA0, i_085ea0),
    (0x085EA6, i_085ea6),
    (0x085EAA, i_085eaa),
    (0x085EB0, i_085eb0),
    (0x085EB4, i_085eb4),
    (0x085EB6, i_085eb6),
    (0x085EBA, i_085eba),
    (0x085EBC, i_085ebc),
    (0x085EC2, i_085ec2),
    (0x085EC6, i_085ec6),
    (0x085ECA, i_085eca),
    (0x085ECE, i_085ece),
    (0x085ED2, i_085ed2),
    (0x085ED6, i_085ed6),
    (0x085EDA, i_085eda),
    (0x085EDE, i_085ede),
    (0x085EE2, i_085ee2),
    (0x085EE6, i_085ee6),
    (0x085EE8, i_085ee8),
    (0x085EEE, i_085eee),
    (0x085EF2, i_085ef2),
    (0x085EF8, i_085ef8),
    (0x085EFC, i_085efc),
    (0x085F00, i_085f00),
    (0x085F06, i_085f06),
    (0x085F08, i_085f08),
    (0x085F0E, i_085f0e),
    (0x085F12, i_085f12),
    (0x085F16, i_085f16),
    (0x085F1A, i_085f1a),
    (0x085F1E, i_085f1e),
    (0x085F22, i_085f22),
    (0x085F26, i_085f26),
    (0x085F28, i_085f28),
    (0x085F2C, i_085f2c),
    (0x085F30, i_085f30),
    (0x085F34, i_085f34),
    (0x085F3A, i_085f3a),
    (0x085F3E, i_085f3e),
    (0x085F42, i_085f42),
    (0x085F44, i_085f44),
    (0x085F48, i_085f48),
    (0x085F4C, i_085f4c),
    (0x085F50, i_085f50),
    (0x085F54, i_085f54),
    (0x085F58, i_085f58),
    (0x085F5E, i_085f5e),
    (0x085F60, i_085f60),
    (0x085F64, i_085f64),
    (0x085F68, i_085f68),
    (0x085F6E, i_085f6e),
    (0x085F70, i_085f70),
    (0x085F72, i_085f72),
    (0x085F76, i_085f76),
    (0x085F7A, i_085f7a),
    (0x085F7C, i_085f7c),
    (0x085F7E, i_085f7e),
    (0x085F80, i_085f80),
    (0x085F84, i_085f84),
    (0x085F88, i_085f88),
    (0x085F8C, i_085f8c),
    (0x085F8E, i_085f8e),
    (0x085F92, i_085f92),
    (0x085F96, i_085f96),
    (0x085F98, i_085f98),
    (0x085F9C, i_085f9c),
    (0x085F9E, i_085f9e),
    (0x085FA0, i_085fa0),
    (0x085FA4, i_085fa4),
    (0x085FA8, i_085fa8),
    (0x085FAA, i_085faa),
    (0x085FAE, i_085fae),
    (0x085FB0, i_085fb0),
    (0x085FB6, i_085fb6),
    (0x085FBA, i_085fba),
    (0x085FBE, i_085fbe),
    (0x085FC2, i_085fc2),
    (0x085FC8, i_085fc8),
    (0x085FCC, i_085fcc),
    (0x085FD0, i_085fd0),
    (0x085FD4, i_085fd4),
    (0x085FD8, i_085fd8),
    (0x085FDE, i_085fde),
    (0x085FE4, i_085fe4),
    (0x085FEA, i_085fea),
    (0x085FEE, i_085fee),
    (0x085FF4, i_085ff4),
    (0x085FF8, i_085ff8),
    (0x085FFE, i_085ffe),
    (0x086002, i_086002),
    (0x086006, i_086006),
    (0x086008, i_086008),
    (0x08600C, i_08600c),
    (0x086010, i_086010),
    (0x086014, i_086014),
    (0x086018, i_086018),
    (0x08601A, i_08601a),
    (0x08601C, i_08601c),
    (0x086020, i_086020),
    (0x086024, i_086024),
    (0x086028, i_086028),
    (0x08602C, i_08602c),
    (0x086030, i_086030),
    (0x086034, i_086034),
    (0x08603A, i_08603a),
    (0x08603C, i_08603c),
    (0x086040, i_086040),
    (0x086044, i_086044),
    (0x086048, i_086048),
    (0x08604E, i_08604e),
    (0x086050, i_086050),
    (0x086056, i_086056),
    (0x08605C, i_08605c),
    (0x086060, i_086060),
    (0x086064, i_086064),
    (0x08606A, i_08606a),
    (0x08606E, i_08606e),
    (0x086074, i_086074),
    (0x086076, i_086076),
    (0x08607C, i_08607c),
    (0x086084, i_086084),
    (0x086088, i_086088),
    (0x08608E, i_08608e),
    (0x086090, i_086090),
    (0x086094, i_086094),
    (0x086098, i_086098),
    (0x08609C, i_08609c),
    (0x0860A2, i_0860a2),
    (0x0860A8, i_0860a8),
    (0x0860AC, i_0860ac),
    (0x0860B0, i_0860b0),
    (0x0860B4, i_0860b4),
    (0x0860B8, i_0860b8),
    (0x0860BC, i_0860bc),
    (0x0860C0, i_0860c0),
    (0x0860C6, i_0860c6),
    (0x0860CC, i_0860cc),
    (0x0860D0, i_0860d0),
    (0x0860D4, i_0860d4),
    (0x0860DA, i_0860da),
    (0x0860E0, i_0860e0),
    (0x0860E2, i_0860e2),
    (0x0860E4, i_0860e4),
    (0x0860E6, i_0860e6),
    (0x0860EA, i_0860ea),
    (0x0860EE, i_0860ee),
    (0x0860F4, i_0860f4),
    (0x0860FA, i_0860fa),
    (0x0860FE, i_0860fe),
    (0x086102, i_086102),
    (0x086104, i_086104),
    (0x086108, i_086108),
    (0x08610A, i_08610a),
    (0x08610C, i_08610c),
    (0x086112, i_086112),
    (0x086114, i_086114),
    (0x08611A, i_08611a),
    (0x08611E, i_08611e),
    (0x086122, i_086122),
    (0x086126, i_086126),
    (0x08612A, i_08612a),
    (0x086130, i_086130),
    (0x086136, i_086136),
    (0x08613A, i_08613a),
    (0x08613E, i_08613e),
    (0x086142, i_086142),
    (0x086146, i_086146),
    (0x08614A, i_08614a),
    (0x08614E, i_08614e),
    (0x086154, i_086154),
    (0x08615A, i_08615a),
    (0x08615E, i_08615e),
    (0x086162, i_086162),
    (0x086166, i_086166),
    (0x08616A, i_08616a),
    (0x08616E, i_08616e),
    (0x086172, i_086172),
    (0x086178, i_086178),
    (0x08617E, i_08617e),
    (0x086182, i_086182),
    (0x086186, i_086186),
    (0x08618C, i_08618c),
    (0x086192, i_086192),
    (0x086194, i_086194),
    (0x086196, i_086196),
    (0x08619C, i_08619c),
    (0x0861A2, i_0861a2),
    (0x0861A8, i_0861a8),
    (0x0861AE, i_0861ae),
    (0x0861B4, i_0861b4),
    (0x0861BA, i_0861ba),
    (0x0861C0, i_0861c0),
    (0x0861C6, i_0861c6),
    (0x0861CC, i_0861cc),
    (0x0861D2, i_0861d2),
    (0x0861D8, i_0861d8),
    (0x0861DE, i_0861de),
    (0x0861E4, i_0861e4),
    (0x0861EA, i_0861ea),
    (0x0861F0, i_0861f0),
    (0x0861F6, i_0861f6),
    (0x0861FC, i_0861fc),
    (0x086202, i_086202),
    (0x086208, i_086208),
    (0x08620E, i_08620e),
    (0x086214, i_086214),
    (0x08621A, i_08621a),
    (0x086220, i_086220),
    (0x086226, i_086226),
    (0x08622C, i_08622c),
    (0x086232, i_086232),
    (0x086238, i_086238),
    (0x08623E, i_08623e),
    (0x086244, i_086244),
    (0x08624A, i_08624a),
    (0x086250, i_086250),
    (0x086256, i_086256),
    (0x08625C, i_08625c),
    (0x086262, i_086262),
    (0x086268, i_086268),
    (0x08626E, i_08626e),
    (0x086274, i_086274),
    (0x08627A, i_08627a),
    (0x086280, i_086280),
    (0x086286, i_086286),
    (0x08628C, i_08628c),
    (0x086292, i_086292),
    (0x086298, i_086298),
    (0x08629E, i_08629e),
    (0x0862A4, i_0862a4),
    (0x0862AA, i_0862aa),
    (0x0862B0, i_0862b0),
    (0x0862B6, i_0862b6),
    (0x0862BC, i_0862bc),
    (0x0862C2, i_0862c2),
    (0x0862C8, i_0862c8),
    (0x0862CE, i_0862ce),
    (0x0862D4, i_0862d4),
    (0x0862DA, i_0862da),
    (0x0862E0, i_0862e0),
    (0x0862E6, i_0862e6),
    (0x0862EC, i_0862ec),
    (0x0862F2, i_0862f2),
];
