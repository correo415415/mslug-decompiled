// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región bios_5) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $C141A0: bne.b $c14192
pub fn i_c141a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141A2;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC14192; return 10; }
    8
}
/// $C141A2: jsr $c12a0e(pc)
pub fn i_c141a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC141A6);
    cpu.pc = 0xC12A0E;
    16
}
/// $C141A6: jsr $c14838(pc)
pub fn i_c141a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC141AA);
    cpu.pc = 0xC14838;
    16
}
/// $C141AA: lea.l $c14dd2(pc), a2
pub fn i_c141aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141AE;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14DD2;
    16
}
/// $C141AE: jsr $c1488c(pc)
pub fn i_c141ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC141B2);
    cpu.pc = 0xC1488C;
    16
}
/// $C141B2: jsr $c127f6(pc)
pub fn i_c141b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC141B6);
    cpu.pc = 0xC127F6;
    16
}
/// $C141B6: jsr $c14918(pc)
pub fn i_c141b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC141BA);
    cpu.pc = 0xC14918;
    16
}
/// $C141BA: btst.b #$5, d0
pub fn i_c141ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141BE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C141BE: beq.b $c141c2
pub fn i_c141be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141C0;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC141C2; return 10; }
    8
}
/// $C141C0: rts
pub fn i_c141c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C141C2: btst.b #$6, d0
pub fn i_c141c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141C6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((6) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C141C6: bne.w $c14316
pub fn i_c141c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141CA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC14316; return 10; }
    8
}
/// $C141CA: btst.b #$4, d0
pub fn i_c141ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141CE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C141CE: bne.w $c14218
pub fn i_c141ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC14218; return 10; }
    8
}
/// $C141D2: swap d0
pub fn i_c141d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141D4;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C141D4: andi.b #$3, d0
pub fn i_c141d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141D8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C141D8: beq.b $c141b6
pub fn i_c141d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141DA;
    cpu.ir = 0x67DC;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC141B6; return 10; }
    8
}
/// $C141DA: moveq #$1, d1
pub fn i_c141da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141DC;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C141DC: andi.b #$2, d0
pub fn i_c141dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141E0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C141E0: bne.b $c141e4
pub fn i_c141e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141E2;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC141E4; return 10; }
    8
}
/// $C141E2: moveq #$ff, d1
pub fn i_c141e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141E4;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C141E4: move.w d1, d0
pub fn i_c141e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141E6;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C141E6: add.w $bb6(a5), d0
pub fn i_c141e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141EA;
    cpu.ir = 0xD06D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C141EA: cmp.w $bb0(a5), d0
pub fn i_c141ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141EE;
    cpu.ir = 0xB06D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C141EE: bhi.b $c141b6
pub fn i_c141ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141F0;
    cpu.ir = 0x62C6;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC141B6; return 10; }
    8
}
/// $C141F0: move.w d0, $bb8(a5)
pub fn i_c141f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141F4;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C141F4: add.w $bb4(a5), d1
pub fn i_c141f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141F8;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C141F8: cmpi.w #$6, d1
pub fn i_c141f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141FC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $C141FC: bcs.b $c141b2
pub fn i_c141fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC141FE;
    cpu.ir = 0x65B4;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC141B2; return 10; }
    8
}
/// $C141FE: tst.w d1
pub fn i_c141fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14200;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C14200: bpl.b $c141aa
pub fn i_c14200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14202;
    cpu.ir = 0x6AA8;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC141AA; return 10; }
    8
}
/// $C14202: andi.l #$ffff, d0
pub fn i_c14202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14208;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C14208: move.w d0, d2
pub fn i_c14208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1420A;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1420A: divu.w #$6, d0
pub fn i_c1420a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1420E;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x6;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C1420E: swap d0
pub fn i_c1420e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14210;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14210: sub.w d0, d2
pub fn i_c14210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14212;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C14212: move.w d2, $bb2(a5)
pub fn i_c14212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14216;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14216: bra.b $c141aa
pub fn i_c14216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14218;
    cpu.ir = 0x6092;
    cpu.pc = 0xC141AA;
    10
}
/// $C14218: jsr $c12a0e(pc)
pub fn i_c14218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1421C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1421C);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1421C: lea.l $c14e24(pc), a2
pub fn i_c1421c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14220;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14E24;
    16
}
/// $C14220: jsr $c14958(pc)
pub fn i_c14220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14224;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14224);
    cpu.pc = 0xC14958;
    16
}
/// $C14224: moveq #$0, d0
pub fn i_c14224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14226;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14226: move.w $bb6(a5), d0
pub fn i_c14226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1422A;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1422A: mulu.w #$18, d0
pub fn i_c1422a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1422E;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x18;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C1422E: movea.l #$10c000, a1
pub fn i_c1422e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14234;
    cpu.ir = 0x227C;
    let v1: u32 = 0x10C000;
    cpu.a[1] = v1;
    4
}
/// $C14234: adda.l d0, a1
pub fn i_c14234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14236;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C14236: jsr $c147be(pc)
pub fn i_c14236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1423A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1423A);
    cpu.pc = 0xC147BE;
    16
}
/// $C1423A: nop
pub fn i_c1423a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1423C;
    cpu.ir = 0x4E71;
    4
}
/// $C1423C: move.w (a1)+, $ace(a5)
pub fn i_c1423c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14240;
    cpu.ir = 0x3B59;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACE);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C14240: move.b (a1), $ad0(a5)
pub fn i_c14240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14244;
    cpu.ir = 0x1B51;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xAD0);
    bus.write8(ad3, v2 as u8);
    16
}
/// $C14244: move.l #$100000, $ac8(a5)
pub fn i_c14244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1424C;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x100000;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1424C: move.w #$4000, $acc(a5)
pub fn i_c1424c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14252;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14252: move.b #$2, $ac4(a5)
pub fn i_c14252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14258;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C14258: jsr $c1231e(pc)
pub fn i_c14258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1425C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1425C);
    cpu.pc = 0xC1231E;
    16
}
/// $C1425C: tst.b $ac6(a5)
pub fn i_c1425c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14260;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C14260: bne.w $c141c0
pub fn i_c14260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14264;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC141C0; return 10; }
    8
}
/// $C14264: move.w $aca(a5), $acc(a5)
pub fn i_c14264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1426A;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xACA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACC);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1426A: jsr $c14918(pc)
pub fn i_c1426a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1426E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1426E);
    cpu.pc = 0xC14918;
    16
}
/// $C1426E: btst.b #$5, d0
pub fn i_c1426e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14272;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C14272: bne.w $c141c0
pub fn i_c14272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14276;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC141C0; return 10; }
    8
}
/// $C14276: btst.b #$4, d0
pub fn i_c14276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1427A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1427A: beq.b $c1426a
pub fn i_c1427a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1427C;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1426A; return 10; }
    8
}
/// $C1427C: jsr $c1494c(pc)
pub fn i_c1427c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14280;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14280);
    cpu.pc = 0xC1494C;
    16
}
/// $C14280: bne.b $c1426a
pub fn i_c14280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14282;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1426A; return 10; }
    8
}
/// $C14282: move.w $ad0(a5), $b9a(a5)
pub fn i_c14282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14288;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xB9A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C14288: move.b #$1, $ac4(a5)
pub fn i_c14288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1428E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1428E: jsr $c1231e(pc)
pub fn i_c1428e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14292;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14292);
    cpu.pc = 0xC1231E;
    16
}
/// $C14292: move.b $ac6(a5), d0
pub fn i_c14292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14296;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14296: cmpi.b #$81, d0
pub fn i_c14296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1429A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x81, Size::Byte, false);
    8
}
/// $C1429A: beq.w $c147fe
pub fn i_c1429a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1429E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC147FE; return 10; }
    8
}
/// $C1429E: cmpi.b #$85, d0
pub fn i_c1429e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142A2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x85, Size::Byte, false);
    8
}
/// $C142A2: beq.w $c147fe
pub fn i_c142a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC147FE; return 10; }
    8
}
/// $C142A6: move.w $ad0(a5), d0
pub fn i_c142a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142AA;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C142AA: moveq #$0, d1
pub fn i_c142aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142AC;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C142AC: move.b $b9a(a5), d1
pub fn i_c142ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142B0;
    cpu.ir = 0x122D;
    let ad2 = cpu.a[5].wrapping_add(0xB9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C142B0: move.b d1, $ad0(a5)
pub fn i_c142b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142B4;
    cpu.ir = 0x1B41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C142B4: btst.l d1, d0
pub fn i_c142b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142B6;
    cpu.ir = 0x300;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C142B6: beq.w $c142f0
pub fn i_c142b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC142F0; return 10; }
    8
}
/// $C142BA: jsr $c12a0e(pc)
pub fn i_c142ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC142BE);
    cpu.pc = 0xC12A0E;
    16
}
/// $C142BE: lea.l $c14e6a(pc), a2
pub fn i_c142be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142C2;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14E6A;
    16
}
/// $C142C2: jsr $c14958(pc)
pub fn i_c142c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC142C6);
    cpu.pc = 0xC14958;
    16
}
/// $C142C6: moveq #$0, d0
pub fn i_c142c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142C8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C142C8: move.w $bb6(a5), d0
pub fn i_c142c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142CC;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C142CC: mulu.w #$18, d0
pub fn i_c142cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142D0;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x18;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C142D0: movea.l #$10c000, a1
pub fn i_c142d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142D6;
    cpu.ir = 0x227C;
    let v1: u32 = 0x10C000;
    cpu.a[1] = v1;
    4
}
/// $C142D6: adda.l d0, a1
pub fn i_c142d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142D8;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C142D8: jsr $c147be(pc)
pub fn i_c142d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142DC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC142DC);
    cpu.pc = 0xC147BE;
    16
}
/// $C142DC: nop
pub fn i_c142dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142DE;
    cpu.ir = 0x4E71;
    4
}
/// $C142DE: jsr $c14918(pc)
pub fn i_c142de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC142E2);
    cpu.pc = 0xC14918;
    16
}
/// $C142E2: btst.b #$5, d0
pub fn i_c142e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142E6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C142E6: bne.w $c141c0
pub fn i_c142e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC141C0; return 10; }
    8
}
/// $C142EA: btst.b #$4, d0
pub fn i_c142ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142EE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C142EE: beq.b $c142de
pub fn i_c142ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142F0;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC142DE; return 10; }
    8
}
/// $C142F0: move.l #$100000, $ac8(a5)
pub fn i_c142f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142F8;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x100000;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C142F8: move.b #$3, $ac4(a5)
pub fn i_c142f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC142FE;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C142FE: jsr $c1231e(pc)
pub fn i_c142fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14302;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14302);
    cpu.pc = 0xC1231E;
    16
}
/// $C14302: move.b $ac6(a5), d0
pub fn i_c14302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14306;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14306: beq.w $c1410c
pub fn i_c14306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1430A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1410C; return 10; }
    8
}
/// $C1430A: cmpi.b #$80, d0
pub fn i_c1430a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1430E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x80, Size::Byte, false);
    8
}
/// $C1430E: bne.w $c147fe
pub fn i_c1430e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14312;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC147FE; return 10; }
    8
}
/// $C14312: bra.w $c141c0
pub fn i_c14312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14316;
    cpu.ir = 0x6000;
    cpu.pc = 0xC141C0;
    10
}
/// $C14316: jsr $c12a0e(pc)
pub fn i_c14316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1431A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1431A);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1431A: lea.l $c14ed4(pc), a2
pub fn i_c1431a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1431E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14ED4;
    16
}
/// $C1431E: jsr $c14958(pc)
pub fn i_c1431e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14322;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14322);
    cpu.pc = 0xC14958;
    16
}
/// $C14322: lea.l $100000.l, a0
pub fn i_c14322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14328;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100000;
    20
}
/// $C14328: movea.l #$10c000, a1
pub fn i_c14328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1432E;
    cpu.ir = 0x227C;
    let v1: u32 = 0x10C000;
    cpu.a[1] = v1;
    4
}
/// $C1432E: move.w $bb0(a5), d7
pub fn i_c1432e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14332;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C14332: lea.l $2(a0), a2
pub fn i_c14332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14336;
    cpu.ir = 0x45E8;
    let ad1 = cpu.a[0].wrapping_add(0x2);
    cpu.a[2] = ad1;
    16
}
/// $C14336: move.l a2, $ac8(a5)
pub fn i_c14336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1433A;
    cpu.ir = 0x2B4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1433A: move.w $14(a1), $ace(a5)
pub fn i_c1433a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14340;
    cpu.ir = 0x3B69;
    let ad2 = cpu.a[1].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACE);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C14340: move.b $16(a1), $ad0(a5)
pub fn i_c14340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14346;
    cpu.ir = 0x1B69;
    let ad2 = cpu.a[1].wrapping_add(0x16);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAD0);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C14346: adda.l #$18, a1
pub fn i_c14346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1434C;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x18;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C1434C: move.w #$4000, $acc(a5)
pub fn i_c1434c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14352;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14352: move.b #$2, $ac4(a5)
pub fn i_c14352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14358;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C14358: movem.l d7/a0-a2, -(a7)
pub fn i_c14358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1435C;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C1435C: jsr $c1231e(pc)
pub fn i_c1435c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14360;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14360);
    cpu.pc = 0xC1231E;
    16
}
/// $C14360: movem.l (a7)+, d7/a0-a2
pub fn i_c14360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14364;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C14364: move.b $ac6(a5), d0
pub fn i_c14364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14368;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14368: bne.w $c147fe
pub fn i_c14368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1436C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC147FE; return 10; }
    8
}
/// $C1436C: movea.l $ac8(a5), a3
pub fn i_c1436c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14370;
    cpu.ir = 0x266D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $C14370: movea.l a3, a4
pub fn i_c14370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14372;
    cpu.ir = 0x284B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $C14372: suba.l a2, a4
pub fn i_c14372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14374;
    cpu.ir = 0x99CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[4] = cpu.a[4].wrapping_sub(v1);
    8
}
/// $C14374: move.w a4, (a0)
pub fn i_c14374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14376;
    cpu.ir = 0x308C;
    let v1 = cpu.a[4] as i16 as i32 as u32;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[0], v1 as u16);
    8
}
/// $C14376: movea.l a3, a0
pub fn i_c14376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14378;
    cpu.ir = 0x204B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C14378: dbra d7, $c14332
pub fn i_c14378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1437C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1437C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1437C; return 14; }
    cpu.pc = 0xC14332;
    10
}
/// $C1437C: move.l a0, $bb2(a5)
pub fn i_c1437c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14380;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14380: move.l a0, $ac8(a5)
pub fn i_c14380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14384;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14384: move.b #$7, $ac4(a5)
pub fn i_c14384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1438A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1438A: jsr $c1231e(pc)
pub fn i_c1438a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1438E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1438E);
    cpu.pc = 0xC1231E;
    16
}
/// $C1438E: move.b $ac6(a5), $bb6(a5)
pub fn i_c1438e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14394;
    cpu.ir = 0x1B6D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xBB6);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C14394: jsr $c14918(pc)
pub fn i_c14394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14398;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14398);
    cpu.pc = 0xC14918;
    16
}
/// $C14398: btst.b #$5, d0
pub fn i_c14398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1439C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1439C: bne.w $c141c0
pub fn i_c1439c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC141C0; return 10; }
    8
}
/// $C143A0: btst.b #$4, d0
pub fn i_c143a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143A4;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C143A4: beq.b $c14394
pub fn i_c143a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143A6;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14394; return 10; }
    8
}
/// $C143A6: jsr $c1494c(pc)
pub fn i_c143a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC143AA);
    cpu.pc = 0xC1494C;
    16
}
/// $C143AA: bne.b $c14394
pub fn i_c143aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143AC;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC14394; return 10; }
    8
}
/// $C143AC: move.l #$10fe9c, $ac8(a5)
pub fn i_c143ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143B4;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10FE9C;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C143B4: clr.w $acc(a5)
pub fn i_c143b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143B8;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xACC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C143B8: move.b #$5, $ac4(a5)
pub fn i_c143b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143BE;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C143BE: jsr $c1231e(pc)
pub fn i_c143be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC143C2);
    cpu.pc = 0xC1231E;
    16
}
/// $C143C2: tst.b $ac6(a5)
pub fn i_c143c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143C6;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C143C6: bne.b $c143e6
pub fn i_c143c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143C8;
    cpu.ir = 0x661E;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC143E6; return 10; }
    8
}
/// $C143C8: jsr $c12a0e(pc)
pub fn i_c143c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC143CC);
    cpu.pc = 0xC12A0E;
    16
}
/// $C143CC: lea.l $c14d2c(pc), a2
pub fn i_c143cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143D0;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14D2C;
    16
}
/// $C143D0: jsr $c14958(pc)
pub fn i_c143d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC143D4);
    cpu.pc = 0xC14958;
    16
}
/// $C143D4: jsr $c14918(pc)
pub fn i_c143d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC143D8);
    cpu.pc = 0xC14918;
    16
}
/// $C143D8: btst.b #$5, d0
pub fn i_c143d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143DC;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C143DC: bne.w $c1410a
pub fn i_c143dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143E0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1410A; return 10; }
    8
}
/// $C143E0: btst.b #$4, d0
pub fn i_c143e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143E4;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C143E4: beq.b $c143d4
pub fn i_c143e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143E6;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC143D4; return 10; }
    8
}
/// $C143E6: move.b #$0, $ac4(a5)
pub fn i_c143e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143EC;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C143EC: jsr $c1231e(pc)
pub fn i_c143ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC143F0);
    cpu.pc = 0xC1231E;
    16
}
/// $C143F0: move.b $ac6(a5), d0
pub fn i_c143f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143F4;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C143F4: bne.w $c147fe
pub fn i_c143f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC147FE; return 10; }
    8
}
/// $C143F8: tst.b $bb6(a5)
pub fn i_c143f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143FC;
    cpu.ir = 0x4A2D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C143FC: bne.b $c1440e
pub fn i_c143fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC143FE;
    cpu.ir = 0x6610;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1440E; return 10; }
    8
}
/// $C143FE: move.l $bb2(a5), $ac8(a5)
pub fn i_c143fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14404;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C14404: move.b #$6, $ac4(a5)
pub fn i_c14404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1440A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1440A: jsr $c1231e(pc)
pub fn i_c1440a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1440E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1440E);
    cpu.pc = 0xC1231E;
    16
}
/// $C1440E: lea.l $100000.l, a0
pub fn i_c1440e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14414;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100000;
    20
}
/// $C14414: movea.l #$10c000, a1
pub fn i_c14414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1441A;
    cpu.ir = 0x227C;
    let v1: u32 = 0x10C000;
    cpu.a[1] = v1;
    4
}
/// $C1441A: move.w $bb0(a5), d7
pub fn i_c1441a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1441E;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1441E: move.w (a0)+, $acc(a5)
pub fn i_c1441e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14422;
    cpu.ir = 0x3B58;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACC);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C14422: move.l a0, $ac8(a5)
pub fn i_c14422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14426;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14426: move.w $14(a1), $ace(a5)
pub fn i_c14426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1442C;
    cpu.ir = 0x3B69;
    let ad2 = cpu.a[1].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACE);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1442C: move.b $16(a1), $ad0(a5)
pub fn i_c1442c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14432;
    cpu.ir = 0x1B69;
    let ad2 = cpu.a[1].wrapping_add(0x16);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xAD0);
    bus.write8(ad4, v3 as u8);
    20
}
/// $C14432: adda.l #$18, a1
pub fn i_c14432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14438;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x18;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C14438: move.b #$3, $ac4(a5)
pub fn i_c14438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1443E;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1443E: movem.l d7/a0-a1, -(a7)
pub fn i_c1443e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14442;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[7]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $C14442: jsr $c1231e(pc)
pub fn i_c14442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14446;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14446);
    cpu.pc = 0xC1231E;
    16
}
/// $C14446: movem.l (a7)+, d7/a0-a1
pub fn i_c14446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1444A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $C1444A: move.b $ac6(a5), d0
pub fn i_c1444a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1444E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1444E: bne.w $c147fe
pub fn i_c1444e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14452;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC147FE; return 10; }
    8
}
/// $C14452: moveq #$0, d0
pub fn i_c14452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14454;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14454: move.w -$2(a0), d0
pub fn i_c14454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14458;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C14458: adda.l d0, a0
pub fn i_c14458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1445A;
    cpu.ir = 0xD1C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C1445A: dbra d7, $c1441e
pub fn i_c1445a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1445E;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1445E; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1445E; return 14; }
    cpu.pc = 0xC1441E;
    10
}
/// $C1445E: jmp $c1410c(pc)
pub fn i_c1445e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14462;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC1410C;
    8
}
/// $C14462: jsr $c1494c(pc)
pub fn i_c14462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14466;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14466);
    cpu.pc = 0xC1494C;
    16
}
/// $C14466: beq.w $c14486
pub fn i_c14466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1446A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14486; return 10; }
    8
}
/// $C1446A: lea.l $c150b4(pc), a2
pub fn i_c1446a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1446E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC150B4;
    16
}
/// $C1446E: jsr $c14958(pc)
pub fn i_c1446e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14472;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14472);
    cpu.pc = 0xC14958;
    16
}
/// $C14472: jsr $c14918(pc)
pub fn i_c14472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14476;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14476);
    cpu.pc = 0xC14918;
    16
}
/// $C14476: btst.b #$5, d0
pub fn i_c14476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1447A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1447A: bne.b $c144a0
pub fn i_c1447a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1447C;
    cpu.ir = 0x6624;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC144A0; return 10; }
    8
}
/// $C1447C: jsr $c1494c(pc)
pub fn i_c1447c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14480;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14480);
    cpu.pc = 0xC1494C;
    16
}
/// $C14480: bne.b $c14472
pub fn i_c14480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14482;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC14472; return 10; }
    8
}
/// $C14482: jsr $c12a0e(pc)
pub fn i_c14482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14486;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14486);
    cpu.pc = 0xC12A0E;
    16
}
/// $C14486: jsr $c14838(pc)
pub fn i_c14486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1448A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1448A);
    cpu.pc = 0xC14838;
    16
}
/// $C1448A: lea.l $c14f2e(pc), a2
pub fn i_c1448a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1448E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14F2E;
    16
}
/// $C1448E: jsr $c1488c(pc)
pub fn i_c1448e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14492;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14492);
    cpu.pc = 0xC1488C;
    16
}
/// $C14492: jsr $c127f6(pc)
pub fn i_c14492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14496;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14496);
    cpu.pc = 0xC127F6;
    16
}
/// $C14496: jsr $c14918(pc)
pub fn i_c14496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1449A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1449A);
    cpu.pc = 0xC14918;
    16
}
/// $C1449A: btst.b #$5, d0
pub fn i_c1449a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1449E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1449E: beq.b $c144a2
pub fn i_c1449e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144A0;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC144A2; return 10; }
    8
}
/// $C144A0: rts
pub fn i_c144a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C144A2: btst.b #$4, d0
pub fn i_c144a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144A6;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C144A6: bne.w $c144f0
pub fn i_c144a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144AA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC144F0; return 10; }
    8
}
/// $C144AA: swap d0
pub fn i_c144aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144AC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C144AC: andi.b #$3, d0
pub fn i_c144ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144B0;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C144B0: beq.b $c14496
pub fn i_c144b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144B2;
    cpu.ir = 0x67E4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14496; return 10; }
    8
}
/// $C144B2: moveq #$1, d1
pub fn i_c144b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144B4;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C144B4: andi.b #$2, d0
pub fn i_c144b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144B8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C144B8: bne.b $c144bc
pub fn i_c144b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144BA;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC144BC; return 10; }
    8
}
/// $C144BA: moveq #$ff, d1
pub fn i_c144ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144BC;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C144BC: move.w d1, d0
pub fn i_c144bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144BE;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C144BE: add.w $bb6(a5), d0
pub fn i_c144be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144C2;
    cpu.ir = 0xD06D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C144C2: cmp.w $bb0(a5), d0
pub fn i_c144c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144C6;
    cpu.ir = 0xB06D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C144C6: bhi.b $c14496
pub fn i_c144c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144C8;
    cpu.ir = 0x62CE;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0xC14496; return 10; }
    8
}
/// $C144C8: move.w d0, $bb8(a5)
pub fn i_c144c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144CC;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C144CC: add.w $bb4(a5), d1
pub fn i_c144cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144D0;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C144D0: cmpi.w #$6, d1
pub fn i_c144d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144D4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $C144D4: bcs.b $c14492
pub fn i_c144d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144D6;
    cpu.ir = 0x65BC;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC14492; return 10; }
    8
}
/// $C144D6: tst.w d1
pub fn i_c144d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144D8;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C144D8: bpl.b $c1448a
pub fn i_c144d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144DA;
    cpu.ir = 0x6AB0;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC1448A; return 10; }
    8
}
/// $C144DA: andi.l #$ffff, d0
pub fn i_c144da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144E0;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C144E0: move.w d0, d2
pub fn i_c144e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144E2;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C144E2: divu.w #$6, d0
pub fn i_c144e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144E6;
    cpu.ir = 0x80FC;
    let v1: u32 = 0x6;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $C144E6: swap d0
pub fn i_c144e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144E8;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C144E8: sub.w d0, d2
pub fn i_c144e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144EA;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C144EA: move.w d2, $bb2(a5)
pub fn i_c144ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144EE;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C144EE: bra.b $c1448a
pub fn i_c144ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144F0;
    cpu.ir = 0x609A;
    cpu.pc = 0xC1448A;
    10
}
/// $C144F0: jsr $c12a0e(pc)
pub fn i_c144f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC144F4);
    cpu.pc = 0xC12A0E;
    16
}
/// $C144F4: lea.l $c14f6a(pc), a2
pub fn i_c144f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144F8;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14F6A;
    16
}
/// $C144F8: jsr $c14958(pc)
pub fn i_c144f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC144FC);
    cpu.pc = 0xC14958;
    16
}
/// $C144FC: moveq #$0, d0
pub fn i_c144fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC144FE;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C144FE: move.w $bb6(a5), d0
pub fn i_c144fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14502;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C14502: mulu.w #$18, d0
pub fn i_c14502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14506;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x18;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C14506: movea.l #$10c000, a1
pub fn i_c14506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1450C;
    cpu.ir = 0x227C;
    let v1: u32 = 0x10C000;
    cpu.a[1] = v1;
    4
}
/// $C1450C: adda.l d0, a1
pub fn i_c1450c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1450E;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C1450E: jsr $c147be(pc)
pub fn i_c1450e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14512;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14512);
    cpu.pc = 0xC147BE;
    16
}
/// $C14512: nop
pub fn i_c14512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14514;
    cpu.ir = 0x4E71;
    4
}
/// $C14514: move.w (a1)+, $ace(a5)
pub fn i_c14514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14518;
    cpu.ir = 0x3B59;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xACE);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C14518: move.b (a1), $ad0(a5)
pub fn i_c14518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1451C;
    cpu.ir = 0x1B51;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xAD0);
    bus.write8(ad3, v2 as u8);
    16
}
/// $C1451C: jsr $c14918(pc)
pub fn i_c1451c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14520;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14520);
    cpu.pc = 0xC14918;
    16
}
/// $C14520: btst.b #$5, d0
pub fn i_c14520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14524;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C14524: bne.w $c144a0
pub fn i_c14524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14528;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC144A0; return 10; }
    8
}
/// $C14528: btst.b #$4, d0
pub fn i_c14528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1452C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1452C: beq.b $c1451c
pub fn i_c1452c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1452E;
    cpu.ir = 0x67EE;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1451C; return 10; }
    8
}
/// $C1452E: jsr $c1494c(pc)
pub fn i_c1452e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14532;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14532);
    cpu.pc = 0xC1494C;
    16
}
/// $C14532: bne.b $c1451c
pub fn i_c14532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14534;
    cpu.ir = 0x66E8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1451C; return 10; }
    8
}
/// $C14534: move.b #$4, $ac4(a5)
pub fn i_c14534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1453A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1453A: jsr $c1231e(pc)
pub fn i_c1453a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1453E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1453E);
    cpu.pc = 0xC1231E;
    16
}
/// $C1453E: move.b $ac6(a5), d0
pub fn i_c1453e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14542;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14542: bne.w $c147fe
pub fn i_c14542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14546;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC147FE; return 10; }
    8
}
/// $C14546: jmp $c1410c(pc)
pub fn i_c14546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1454A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC1410C;
    8
}
/// $C1454A: jsr $c1494c(pc)
pub fn i_c1454a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1454E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1454E);
    cpu.pc = 0xC1494C;
    16
}
/// $C1454E: beq.w $c1456e
pub fn i_c1454e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14552;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1456E; return 10; }
    8
}
/// $C14552: lea.l $c150b4(pc), a2
pub fn i_c14552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14556;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC150B4;
    16
}
/// $C14556: jsr $c14958(pc)
pub fn i_c14556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1455A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1455A);
    cpu.pc = 0xC14958;
    16
}
/// $C1455A: jsr $c14918(pc)
pub fn i_c1455a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1455E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1455E);
    cpu.pc = 0xC14918;
    16
}
/// $C1455E: btst.b #$5, d0
pub fn i_c1455e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14562;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C14562: bne.b $c145cc
pub fn i_c14562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14564;
    cpu.ir = 0x6668;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC145CC; return 10; }
    8
}
/// $C14564: jsr $c1494c(pc)
pub fn i_c14564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14568;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14568);
    cpu.pc = 0xC1494C;
    16
}
/// $C14568: bne.b $c1455a
pub fn i_c14568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1456A;
    cpu.ir = 0x66F0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1455A; return 10; }
    8
}
/// $C1456A: jsr $c12a0e(pc)
pub fn i_c1456a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1456E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1456E);
    cpu.pc = 0xC12A0E;
    16
}
/// $C1456E: move.l #$100000, $ac8(a5)
pub fn i_c1456e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14576;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x100000;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14576: move.b #$7, $ac4(a5)
pub fn i_c14576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1457C;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1457C: jsr $c1231e(pc)
pub fn i_c1457c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14580;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14580);
    cpu.pc = 0xC1231E;
    16
}
/// $C14580: move.b $ac6(a5), d0
pub fn i_c14580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14584;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14584: beq.b $c14596
pub fn i_c14584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14586;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14596; return 10; }
    8
}
/// $C14586: lea.l $100000.l, a0
pub fn i_c14586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1458C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100000;
    20
}
/// $C1458C: moveq #$f, d7
pub fn i_c1458c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1458E;
    cpu.ir = 0x7E0F;
    cpu.d[7] = 0xF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1458E: move.b #$20, (a0)+
pub fn i_c1458e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14592;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C14592: dbra d7, $c1458e
pub fn i_c14592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14596;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC14596; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC14596; return 14; }
    cpu.pc = 0xC1458E;
    10
}
/// $C14596: clr.l $bb0(a5)
pub fn i_c14596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1459A;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0xBB0);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C1459A: move.l #$50005, $bb4(a5)
pub fn i_c1459a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145A2;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x50005;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C145A2: clr.w $bb8(a5)
pub fn i_c145a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145A6;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xBB8);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C145A6: jsr $c12a0e(pc)
pub fn i_c145a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145AA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC145AA);
    cpu.pc = 0xC12A0E;
    16
}
/// $C145AA: lea.l $c14faa(pc), a2
pub fn i_c145aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145AE;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC14FAA;
    16
}
/// $C145AE: jsr $c14958(pc)
pub fn i_c145ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC145B2);
    cpu.pc = 0xC14958;
    16
}
/// $C145B2: jsr $c14748(pc)
pub fn i_c145b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC145B6);
    cpu.pc = 0xC14748;
    16
}
/// $C145B6: jsr $c1477a(pc)
pub fn i_c145b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC145BA);
    cpu.pc = 0xC1477A;
    16
}
/// $C145BA: jsr $c17290(pc)
pub fn i_c145ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC145BE);
    cpu.pc = 0xC17290;
    16
}
/// $C145BE: jsr $c146d0(pc)
pub fn i_c145be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC145C2);
    cpu.pc = 0xC146D0;
    16
}
/// $C145C2: jsr $c14918(pc)
pub fn i_c145c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC145C6);
    cpu.pc = 0xC14918;
    16
}
/// $C145C6: btst.b #$5, d0
pub fn i_c145c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145CA;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((5) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C145CA: beq.b $c145ce
pub fn i_c145ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145CC;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC145CE; return 10; }
    8
}
/// $C145CC: rts
pub fn i_c145cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C145CE: btst.b #$4, d0
pub fn i_c145ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145D2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C145D2: beq.w $c146aa
pub fn i_c145d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC146AA; return 10; }
    8
}
/// $C145D6: move.w $bb4(a5), d1
pub fn i_c145d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145DA;
    cpu.ir = 0x322D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C145DA: cmpi.w #$5, d1
pub fn i_c145da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145DE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $C145DE: bcs.w $c1460a
pub fn i_c145de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145E2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1460A; return 10; }
    8
}
/// $C145E2: lea.l $100000.l, a0
pub fn i_c145e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100000;
    20
}
/// $C145E8: move.w $bb0(a5), d3
pub fn i_c145e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145EC;
    cpu.ir = 0x362D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C145EC: lea.l $c14bbe(pc), a1
pub fn i_c145ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC14BBE;
    16
}
/// $C145F0: move.b -$5(a1, d1.w), (a0, d3.w)
pub fn i_c145f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145F6;
    cpu.ir = 0x11B1;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFB).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    24
}
/// $C145F6: addq.w #$1, d3
pub fn i_c145f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145F8;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C145F8: andi.w #$f, d3
pub fn i_c145f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC145FC;
    cpu.ir = 0x243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C145FC: move.w d3, $bb2(a5)
pub fn i_c145fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14600;
    cpu.ir = 0x3B43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14600: jsr $c14748(pc)
pub fn i_c14600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14604;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14604);
    cpu.pc = 0xC14748;
    16
}
/// $C14604: jsr $c1477a(pc)
pub fn i_c14604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14608;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14608);
    cpu.pc = 0xC1477A;
    16
}
/// $C14608: bra.b $c145c2
pub fn i_c14608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1460A;
    cpu.ir = 0x60B8;
    cpu.pc = 0xC145C2;
    10
}
/// $C1460A: add.w d1, d1
pub fn i_c1460a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1460C;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1460C: add.w d1, d1
pub fn i_c1460c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1460E;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1460E: movea.l $c14614(pc, d1.w), a0
pub fn i_c1460e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14612;
    cpu.ir = 0x207B;
    let ad2 = 0xC14614_u32.wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C14612: jmp (a0)
pub fn i_c14612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14614;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C14614: ?
pub fn i_c14614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14616;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14616: not.b $c1(a0)
pub fn i_c14616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1461A;
    cpu.ir = 0x4628;
    let ad1 = cpu.a[0].wrapping_add(0xC1);
    let v2 = bus.read8(ad1) as u32;
    let r3 = !v2 & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1461A: not.w d4
pub fn i_c1461a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1461C;
    cpu.ir = 0x4644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1461C: ?
pub fn i_c1461c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1461E;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1461E: ?
pub fn i_c1461e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14620;
    cpu.ir = 0x464C;
    let v1 = cpu.a[4] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.a[4] = r2 as i16 as i32 as u32;
    4
}
/// $C14620: ?
pub fn i_c14620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14622;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14622: not.w -(a2)
pub fn i_c14622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14624;
    cpu.ir = 0x4662;
    let ad1 = cpu.a[2].wrapping_sub(2); cpu.a[2] = ad1;
    let v2 = bus.read16(ad1) as u32;
    let r3 = !v2 & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    10
}
/// $C14624: ?
pub fn i_c14624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14626;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14626: not.l d2
pub fn i_c14626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14628;
    cpu.ir = 0x4682;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = !v1 & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    4
}
/// $C14628: move.l #$100000, $ac8(a5)
pub fn i_c14628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14630;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x100000;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14630: move.b #$6, $ac4(a5)
pub fn i_c14630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14636;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C14636: jsr $c1231e(pc)
pub fn i_c14636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1463A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1463A);
    cpu.pc = 0xC1231E;
    16
}
/// $C1463A: move.b $ac6(a5), d0
pub fn i_c1463a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1463E;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1463E: bne.w $c147fe
pub fn i_c1463e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14642;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC147FE; return 10; }
    8
}
/// $C14642: rts
pub fn i_c14642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14644;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C14644: move.w $bb0(a5), d0
pub fn i_c14644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14648;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C14648: subq.w #$1, d0
pub fn i_c14648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1464A;
    cpu.ir = 0x5340;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1464A: bra.b $c14652
pub fn i_c1464a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1464C;
    cpu.ir = 0x6006;
    cpu.pc = 0xC14652;
    10
}
/// $C1464C: move.w $bb0(a5), d0
pub fn i_c1464c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14650;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C14650: addq.w #$1, d0
pub fn i_c14650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14652;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C14652: andi.w #$f, d0
pub fn i_c14652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14656;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C14656: move.w d0, $bb2(a5)
pub fn i_c14656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1465A;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1465A: jsr $c1477a(pc)
pub fn i_c1465a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1465E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1465E);
    cpu.pc = 0xC1477A;
    16
}
/// $C1465E: bra.w $c145c2
pub fn i_c1465e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14662;
    cpu.ir = 0x6000;
    cpu.pc = 0xC145C2;
    10
}
/// $C14662: move.w #$5, $bb6(a5)
pub fn i_c14662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14668;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14668: cmpi.w #$1, $bb8(a5)
pub fn i_c14668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1466E;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB8);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $C1466E: bcc.b $c1467a
pub fn i_c1466e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14670;
    cpu.ir = 0x640A;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1467A; return 10; }
    8
}
/// $C14670: move.w #$1, $bb8(a5)
pub fn i_c14670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14676;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14676: bra.w $c145a6
pub fn i_c14676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1467A;
    cpu.ir = 0x6000;
    cpu.pc = 0xC145A6;
    10
}
/// $C1467A: clr.w $bb8(a5)
pub fn i_c1467a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1467E;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xBB8);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1467E: bra.w $c145a6
pub fn i_c1467e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14682;
    cpu.ir = 0x6000;
    cpu.pc = 0xC145A6;
    10
}
/// $C14682: move.w #$5, $bb6(a5)
pub fn i_c14682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14688;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14688: cmpi.w #$2, $bb8(a5)
pub fn i_c14688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1468E;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB8);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $C1468E: beq.b $c14670
pub fn i_c1468e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14690;
    cpu.ir = 0x67E0;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14670; return 10; }
    8
}
/// $C14690: move.w #$2, $bb8(a5)
pub fn i_c14690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14696;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB8);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14696: bra.w $c145a6
pub fn i_c14696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1469A;
    cpu.ir = 0x6000;
    cpu.pc = 0xC145A6;
    10
}
/// $C1469A: ori.b #$0, d0
pub fn i_c1469a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1469E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1469E: btst.l d1, d3
pub fn i_c1469e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146A0;
    cpu.ir = 0x303;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C146A0: btst.l d1, d0
pub fn i_c146a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146A2;
    cpu.ir = 0x300;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C146A2: andi.b #$0, d2
pub fn i_c146a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146A6;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C146A6: ori.b #$0, d0
pub fn i_c146a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146AA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C146AA: swap d0
pub fn i_c146aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146AC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C146AC: andi.w #$f, d0
pub fn i_c146ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146B0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C146B0: beq.w $c145c2
pub fn i_c146b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC145C2; return 10; }
    8
}
/// $C146B4: move.b $c1469a(pc, d0.w), d1
pub fn i_c146b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146B8;
    cpu.ir = 0x123B;
    let ad2 = 0xC1469A_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $C146B8: lea.l $c14a6a(pc), a0
pub fn i_c146b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146BC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0xC14A6A;
    16
}
/// $C146BC: move.w $bb4(a5), d0
pub fn i_c146bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146C0;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C146C0: add.w d0, d0
pub fn i_c146c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146C2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C146C2: add.w d0, d0
pub fn i_c146c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146C4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C146C4: add.b d1, d0
pub fn i_c146c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146C6;
    cpu.ir = 0xD001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $C146C6: move.b (a0, d0.w), $bb7(a5)
pub fn i_c146c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146CC;
    cpu.ir = 0x1B70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xBB7);
    bus.write8(ad4, v3 as u8);
    22
}
/// $C146CC: bra.w $c145be
pub fn i_c146cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146D0;
    cpu.ir = 0x6000;
    cpu.pc = 0xC145BE;
    10
}
/// $C146D0: addq.b #$2, $ac2(a5)
pub fn i_c146d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146D4;
    cpu.ir = 0x542D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C146D4: lea.l $c149c0(pc), a0
pub fn i_c146d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146D8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0xC149C0;
    16
}
/// $C146D8: lea.l $3c0002.l, a2
pub fn i_c146d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x3C0002;
    20
}
/// $C146DE: moveq #$0, d3
pub fn i_c146de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146E0;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C146E0: jsr $c146ee(pc)
pub fn i_c146e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC146E4);
    cpu.pc = 0xC146EE;
    16
}
/// $C146E4: move.w $bb6(a5), $bb4(a5)
pub fn i_c146e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146EA;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xBB4);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C146EA: move.w #$1000, d3
pub fn i_c146ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146EE;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C146EE: move.w $bb4(a5), d0
pub fn i_c146ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146F2;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C146F2: add.w d0, d0
pub fn i_c146f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146F4;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C146F4: move.w (a0, d0.w), -$2(a2)
pub fn i_c146f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146FA;
    cpu.ir = 0x3570;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[2].wrapping_add(0xFFFFFFFE);
    bus.write16(ad4, v3 as u16);
    22
}
/// $C146FA: nop
pub fn i_c146fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146FC;
    cpu.ir = 0x4E71;
    4
}
/// $C146FC: nop
pub fn i_c146fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC146FE;
    cpu.ir = 0x4E71;
    4
}
/// $C146FE: nop
pub fn i_c146fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14700;
    cpu.ir = 0x4E71;
    4
}
/// $C14700: move.w #$1, $2(a2)
pub fn i_c14700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14706;
    cpu.ir = 0x357C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14706: move.w (a2), d0
pub fn i_c14706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14708;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C14708: cmpi.b #$20, d0
pub fn i_c14708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1470C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $C1470C: beq.w $c14742
pub fn i_c1470c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14710;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14742; return 10; }
    8
}
/// $C14710: andi.w #$fff, d0
pub fn i_c14710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14714;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C14714: or.w d3, d0
pub fn i_c14714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14716;
    cpu.ir = 0x8043;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C14716: move.w d0, (a2)
pub fn i_c14716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14718;
    cpu.ir = 0x3480;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $C14718: nop
pub fn i_c14718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1471A;
    cpu.ir = 0x4E71;
    4
}
/// $C1471A: nop
pub fn i_c1471a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1471C;
    cpu.ir = 0x4E71;
    4
}
/// $C1471C: nop
pub fn i_c1471c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1471E;
    cpu.ir = 0x4E71;
    4
}
/// $C1471E: nop
pub fn i_c1471e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14720;
    cpu.ir = 0x4E71;
    4
}
/// $C14720: move.w (a2), d0
pub fn i_c14720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14722;
    cpu.ir = 0x3012;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C14722: nop
pub fn i_c14722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14724;
    cpu.ir = 0x4E71;
    4
}
/// $C14724: nop
pub fn i_c14724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14726;
    cpu.ir = 0x4E71;
    4
}
/// $C14726: nop
pub fn i_c14726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14728;
    cpu.ir = 0x4E71;
    4
}
/// $C14728: nop
pub fn i_c14728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1472A;
    cpu.ir = 0x4E71;
    4
}
/// $C1472A: move.w #$1f, $2(a2)
pub fn i_c1472a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14730;
    cpu.ir = 0x357C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C14730: andi.w #$fff, d0
pub fn i_c14730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14734;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C14734: or.w d3, d0
pub fn i_c14734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14736;
    cpu.ir = 0x8043;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $C14736: move.w d0, (a2)
pub fn i_c14736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14738;
    cpu.ir = 0x3480;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $C14738: nop
pub fn i_c14738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1473A;
    cpu.ir = 0x4E71;
    4
}
/// $C1473A: nop
pub fn i_c1473a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1473C;
    cpu.ir = 0x4E71;
    4
}
/// $C1473C: nop
pub fn i_c1473c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1473E;
    cpu.ir = 0x4E71;
    4
}
/// $C1473E: nop
pub fn i_c1473e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14740;
    cpu.ir = 0x4E71;
    4
}
/// $C14740: bra.b $c14700
pub fn i_c14740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14742;
    cpu.ir = 0x60BE;
    cpu.pc = 0xC14700;
    10
}
/// $C14742: subq.b #$1, $ac2(a5)
pub fn i_c14742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14746;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C14746: rts
pub fn i_c14746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14748;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C14748: addq.b #$1, $ac2(a5)
pub fn i_c14748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1474C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1474C: movea.l $abe(a5), a0
pub fn i_c1474c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14750;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C14750: clr.l (a0)+
pub fn i_c14750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14752;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C14752: move.l #$371a6, (a0)+
pub fn i_c14752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14758;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x371A6;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C14758: move.w #$108, (a0)+
pub fn i_c14758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1475C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1475C: lea.l $100000.l, a1
pub fn i_c1475c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14762;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100000;
    20
}
/// $C14762: moveq #$3, d7
pub fn i_c14762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14764;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14764: move.l (a1)+, (a0)+
pub fn i_c14764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14766;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C14766: dbra d7, $c14764
pub fn i_c14766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1476A;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1476A; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1476A; return 14; }
    cpu.pc = 0xC14764;
    10
}
/// $C1476A: move.l #$ff000000, (a0)+
pub fn i_c1476a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14770;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFF000000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C14770: move.l a0, $abe(a5)
pub fn i_c14770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14774;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14774: subq.b #$1, $ac2(a5)
pub fn i_c14774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14778;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C14778: rts
pub fn i_c14778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1477A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1477A: addq.b #$1, $ac2(a5)
pub fn i_c1477a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1477E;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1477E: movea.l $abe(a5), a0
pub fn i_c1477e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14782;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C14782: clr.l (a0)+
pub fn i_c14782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14784;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C14784: moveq #$20, d3
pub fn i_c14784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14786;
    cpu.ir = 0x7620;
    cpu.d[3] = 0x20;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14786: jsr $c147a2(pc)
pub fn i_c14786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1478A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1478A);
    cpu.pc = 0xC147A2;
    16
}
/// $C1478A: move.w $bb2(a5), $bb0(a5)
pub fn i_c1478a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14790;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xBB0);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C14790: moveq #$12, d3
pub fn i_c14790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14792;
    cpu.ir = 0x7612;
    cpu.d[3] = 0x12;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14792: jsr $c147a2(pc)
pub fn i_c14792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14796;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14796);
    cpu.pc = 0xC147A2;
    16
}
/// $C14796: clr.w (a0)+
pub fn i_c14796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14798;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14798: move.l a0, $abe(a5)
pub fn i_c14798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1479C;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1479C: subq.b #$1, $ac2(a5)
pub fn i_c1479c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147A0;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C147A0: rts
pub fn i_c147a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C147A2: move.w #$3, (a0)+
pub fn i_c147a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147A6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C147A6: move.w $bb0(a5), d0
pub fn i_c147a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147AA;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C147AA: lsl.w #$5, d0
pub fn i_c147aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147AC;
    cpu.ir = 0xEB48;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $C147AC: addi.w #$71a8, d0
pub fn i_c147ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147B0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x71A8, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C147B0: move.w d0, (a0)+
pub fn i_c147b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147B2;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C147B2: move.w #$1108, (a0)+
pub fn i_c147b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147B6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C147B6: move.b d3, (a0)+
pub fn i_c147b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147B8;
    cpu.ir = 0x10C3;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C147B8: move.b #$ff, (a0)+
pub fn i_c147b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147BC;
    cpu.ir = 0x10FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C147BC: rts
pub fn i_c147bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C147BE: addq.b #$1, $ac2(a5)
pub fn i_c147be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147C2;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C147C2: movea.l $abe(a5), a0
pub fn i_c147c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147C6;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C147C6: clr.l (a0)+
pub fn i_c147c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147C8;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C147C8: move.l #$3710c, (a0)+
pub fn i_c147c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147CE;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x3710C;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C147CE: move.w #$108, (a0)+
pub fn i_c147ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147D2;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C147D2: moveq #$4, d7
pub fn i_c147d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147D4;
    cpu.ir = 0x7E04;
    cpu.d[7] = 0x4;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C147D4: move.l (a1)+, (a0)+
pub fn i_c147d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147D6;
    cpu.ir = 0x20D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C147D6: dbra d7, $c147d4
pub fn i_c147d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147DA;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC147DA; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC147DA; return 14; }
    cpu.pc = 0xC147D4;
    10
}
/// $C147DA: move.l #$ff000000, (a0)+
pub fn i_c147da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147E0;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFF000000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C147E0: move.l a0, $abe(a5)
pub fn i_c147e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147E4;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C147E4: subq.b #$1, $ac2(a5)
pub fn i_c147e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147E8;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C147E8: rts
pub fn i_c147e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C147EA: ?
pub fn i_c147ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147EC;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C147EC: dbt d6, $c148af
pub fn i_c147ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147F0;
    cpu.ir = 0x50CE;
    if cpu.sr.evaluate(Condition::True) { cpu.pc = 0xC147F0; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC147F0; return 14; }
    cpu.pc = 0xC148AF;
    10
}
/// $C147F0: st.b -$3f(a2, d0.w)
pub fn i_c147f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147F4;
    cpu.ir = 0x50F2;
    let c1: u32 = if cpu.sr.evaluate(Condition::True) { 0xFF } else { 0 };
    let ad2 = cpu.a[2].wrapping_add(0xFFFFFFC1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    14
}
/// $C147F4: ?
pub fn i_c147f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147F6;
    cpu.ir = 0x510C;
    cpu.a[4] = cpu.a[4].wrapping_sub(8);
    8
}
/// $C147F6: ?
pub fn i_c147f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147F8;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C147F8: subq.b #$8, $c1(a2)
pub fn i_c147f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147FC;
    cpu.ir = 0x512A;
    let ad1 = cpu.a[2].wrapping_add(0xC1);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C147FC: subq.w #$8, d4
pub fn i_c147fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC147FE;
    cpu.ir = 0x5144;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C147FE: subi.b #$81, d0
pub fn i_c147fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14802;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x81, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C14802: bmi.b $c14836
pub fn i_c14802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14804;
    cpu.ir = 0x6B32;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC14836; return 10; }
    8
}
/// $C14804: andi.w #$ff, d0
pub fn i_c14804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14808;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C14808: add.w d0, d0
pub fn i_c14808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1480A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1480A: add.w d0, d0
pub fn i_c1480a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1480C;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C1480C: movea.l $c147ea(pc, d0.w), a2
pub fn i_c1480c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14810;
    cpu.ir = 0x247B;
    let ad2 = 0xC147EA_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
/// $C14810: jsr $c12a0e(pc)
pub fn i_c14810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14814;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14814);
    cpu.pc = 0xC12A0E;
    16
}
/// $C14814: addq.b #$1, $ac2(a5)
pub fn i_c14814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14818;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C14818: movea.l $abe(a5), a0
pub fn i_c14818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1481C;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1481C: move.l a2, (a0)+
pub fn i_c1481c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1481E;
    cpu.ir = 0x20CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1481E: move.l #$c124b8, (a0)+
pub fn i_c1481e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14824;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC124B8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C14824: move.l a0, $abe(a5)
pub fn i_c14824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14828;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14828: subq.b #$1, $ac2(a5)
pub fn i_c14828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1482C;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1482C: jsr $c14918(pc)
pub fn i_c1482c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14830;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14830);
    cpu.pc = 0xC14918;
    16
}
/// $C14830: andi.b #$f0, d0
pub fn i_c14830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14834;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C14834: beq.b $c1482c
pub fn i_c14834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14836;
    cpu.ir = 0x67F6;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1482C; return 10; }
    8
}
/// $C14836: rts
pub fn i_c14836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14838;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C14838: clr.w $acc(a5)
pub fn i_c14838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1483C;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xACC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1483C: move.l #$10c000, $ac8(a5)
pub fn i_c1483c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14844;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x10C000;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14844: move.b #$5, $ac4(a5)
pub fn i_c14844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1484A;
    cpu.ir = 0x1B7C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xAC4);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1484A: jsr $c1231e(pc)
pub fn i_c1484a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1484E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1484E);
    cpu.pc = 0xC1231E;
    16
}
/// $C1484E: move.b $ac6(a5), d0
pub fn i_c1484e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14852;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAC6);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14852: bne.b $c1486c
pub fn i_c14852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14854;
    cpu.ir = 0x6618;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1486C; return 10; }
    8
}
/// $C14854: movea.l $ac8(a5), a0
pub fn i_c14854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14858;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C14858: adda.l #$14, a0
pub fn i_c14858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1485E;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0x14;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $C1485E: move.w $ace(a5), (a0)+
pub fn i_c1485e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14862;
    cpu.ir = 0x30ED;
    let ad2 = cpu.a[5].wrapping_add(0xACE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C14862: move.w $ad0(a5), (a0)+
pub fn i_c14862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14866;
    cpu.ir = 0x30ED;
    let ad2 = cpu.a[5].wrapping_add(0xAD0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C14866: move.l a0, $ac8(a5)
pub fn i_c14866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1486A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xAC8);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1486A: bra.b $c14844
pub fn i_c1486a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1486C;
    cpu.ir = 0x60D8;
    cpu.pc = 0xC14844;
    10
}
/// $C1486C: cmpi.b #$82, d0
pub fn i_c1486c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14870;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x82, Size::Byte, false);
    8
}
/// $C14870: beq.b $c14876
pub fn i_c14870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14872;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14876; return 10; }
    8
}
/// $C14872: addq.l #$4, a7
pub fn i_c14872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14874;
    cpu.ir = 0x588F;
    cpu.a[7] = cpu.a[7].wrapping_add(4);
    8
}
/// $C14874: bra.b $c147fe
pub fn i_c14874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14876;
    cpu.ir = 0x6088;
    cpu.pc = 0xC147FE;
    10
}
/// $C14876: move.w $acc(a5), $bb0(a5)
pub fn i_c14876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1487C;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0xACC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xBB0);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1487C: beq.b $c14872
pub fn i_c1487c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1487E;
    cpu.ir = 0x67F4;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14872; return 10; }
    8
}
/// $C1487E: subq.w #$1, $bb0(a5)
pub fn i_c1487e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14882;
    cpu.ir = 0x536D;
    let ad1 = cpu.a[5].wrapping_add(0xBB0);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C14882: clr.w $bb2(a5)
pub fn i_c14882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14886;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xBB2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C14886: clr.l $bb6(a5)
pub fn i_c14886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1488A;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0xBB6);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C1488A: rts
pub fn i_c1488a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1488C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1488C: clr.w $bb4(a5)
pub fn i_c1488c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14890;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xBB4);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C14890: jsr $c12a0e(pc)
pub fn i_c14890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14894;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14894);
    cpu.pc = 0xC12A0E;
    16
}
/// $C14894: addq.b #$1, $ac2(a5)
pub fn i_c14894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14898;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C14898: movea.l $abe(a5), a0
pub fn i_c14898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1489C;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1489C: move.l a2, (a0)+
pub fn i_c1489c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1489E;
    cpu.ir = 0x20CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1489E: move.l #$c14d8e, (a0)+
pub fn i_c1489e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148A4;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC14D8E;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C148A4: tst.w $bb2(a5)
pub fn i_c148a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148A8;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C148A8: beq.b $c148b0
pub fn i_c148a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148AA;
    cpu.ir = 0x6706;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC148B0; return 10; }
    8
}
/// $C148AA: move.l #$c128de, (a0)+
pub fn i_c148aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148B0;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC128DE;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C148B0: move.w $bb2(a5), d0
pub fn i_c148b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148B4;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C148B4: mulu.w #$18, d0
pub fn i_c148b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148B8;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0x18;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $C148B8: movea.l #$10c000, a1
pub fn i_c148b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148BE;
    cpu.ir = 0x227C;
    let v1: u32 = 0x10C000;
    cpu.a[1] = v1;
    4
}
/// $C148BE: adda.l d0, a1
pub fn i_c148be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148C0;
    cpu.ir = 0xD3C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $C148C0: clr.l (a0)+
pub fn i_c148c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148C2;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C148C2: move.w #$3, (a0)+
pub fn i_c148c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148C6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C148C6: move.w $bb4(a5), d0
pub fn i_c148c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148CA;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C148CA: add.w d0, d0
pub fn i_c148ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148CC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C148CC: addi.w #$716f, d0
pub fn i_c148cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148D0;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x716F, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C148D0: move.w d0, (a0)+
pub fn i_c148d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148D2;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C148D2: move.w #$108, (a0)+
pub fn i_c148d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148D6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C148D6: moveq #$9, d7
pub fn i_c148d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148D8;
    cpu.ir = 0x7E09;
    cpu.d[7] = 0x9;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C148D8: move.w (a1)+, (a0)+
pub fn i_c148d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148DA;
    cpu.ir = 0x30D9;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $C148DA: dbra d7, $c148d8
pub fn i_c148da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148DE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC148DE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC148DE; return 14; }
    cpu.pc = 0xC148D8;
    10
}
/// $C148DE: move.w #$ff00, (a0)+
pub fn i_c148de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148E2;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C148E2: addq.l #$4, a1
pub fn i_c148e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148E4;
    cpu.ir = 0x5889;
    cpu.a[1] = cpu.a[1].wrapping_add(4);
    8
}
/// $C148E4: move.w $bb2(a5), d0
pub fn i_c148e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148E8;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C148E8: addq.w #$1, d0
pub fn i_c148e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148EA;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C148EA: move.w d0, $bb2(a5)
pub fn i_c148ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148EE;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xBB2);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C148EE: cmp.w $bb0(a5), d0
pub fn i_c148ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148F2;
    cpu.ir = 0xB06D;
    let ad2 = cpu.a[5].wrapping_add(0xBB0);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $C148F2: bls.b $c148fa
pub fn i_c148f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148F4;
    cpu.ir = 0x6306;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0xC148FA; return 10; }
    8
}
/// $C148F4: clr.w (a0)+
pub fn i_c148f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148F6;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C148F6: bra.w $c1490e
pub fn i_c148f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148FA;
    cpu.ir = 0x6000;
    cpu.pc = 0xC1490E;
    10
}
/// $C148FA: addq.w #$1, $bb4(a5)
pub fn i_c148fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC148FE;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0xBB4);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C148FE: move.w $bb4(a5), d0
pub fn i_c148fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14902;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xBB4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C14902: subq.w #$6, d0
pub fn i_c14902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14904;
    cpu.ir = 0x5D40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C14904: bne.b $c148c2
pub fn i_c14904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14906;
    cpu.ir = 0x66BC;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC148C2; return 10; }
    8
}
/// $C14906: clr.w (a0)+
pub fn i_c14906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14908;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14908: move.l #$c128fc, (a0)+
pub fn i_c14908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1490E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC128FC;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1490E: move.l a0, $abe(a5)
pub fn i_c1490e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14912;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C14912: subq.b #$1, $ac2(a5)
pub fn i_c14912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14916;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C14916: rts
pub fn i_c14916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14918;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C14918: jsr $c1727e(pc)
pub fn i_c14918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1491C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1491C);
    cpu.pc = 0xC1727E;
    16
}
/// $C1491C: move.b $a98(a5), d0
pub fn i_c1491c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14920;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14920: swap d0
pub fn i_c14920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14922;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14922: move.b $a97(a5), d0
pub fn i_c14922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14926;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xA97);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14926: cmpi.b #$3, $a94(a5)
pub fn i_c14926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1492C;
    cpu.ir = 0xC2D;
    let ad2 = cpu.a[5].wrapping_add(0xA94);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $C1492C: bne.b $c1494a
pub fn i_c1492c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1492E;
    cpu.ir = 0x661C;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1494A; return 10; }
    8
}
/// $C1492E: move.b $aa1(a5), d0
pub fn i_c1492e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14932;
    cpu.ir = 0x102D;
    let ad2 = cpu.a[5].wrapping_add(0xAA1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C14932: bclr.b #$7, d0
pub fn i_c14932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14936;
    cpu.ir = 0x880;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 & !mk;
    cpu.d[0] = r2;
    8
}
/// $C14936: btst.b #$0, $aa2(a5)
pub fn i_c14936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1493C;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xAA2);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C1493C: beq.b $c14942
pub fn i_c1493c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1493E;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC14942; return 10; }
    8
}
/// $C1493E: bset.b #$7, d0
pub fn i_c1493e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14942;
    cpu.ir = 0x8C0;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    let r2 = v1 | mk;
    cpu.d[0] = r2;
    8
}
/// $C14942: ror.b #$4, d0
pub fn i_c14942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14944;
    cpu.ir = 0xE818;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 3, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $C14944: move.b d0, d1
pub fn i_c14944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14946;
    cpu.ir = 0x1200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C14946: swap d0
pub fn i_c14946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14948;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14948: move.b d1, d0
pub fn i_c14948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1494A;
    cpu.ir = 0x1001;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1494A: rts
pub fn i_c1494a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1494C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1494C: move.b $380000.l, d0
pub fn i_c1494c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14952;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x380000) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $C14952: andi.b #$30, d0
pub fn i_c14952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14956;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C14956: rts
pub fn i_c14956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14958;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C14958: addq.b #$1, $ac2(a5)
pub fn i_c14958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1495C;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1495C: movea.l $abe(a5), a0
pub fn i_c1495c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14960;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C14960: move.l a2, (a0)+
pub fn i_c14960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14962;
    cpu.ir = 0x20CA;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C14962: move.l #$c14cb2, (a0)+
pub fn i_c14962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14968;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC14CB2;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C14968: move.l a0, $abe(a5)
pub fn i_c14968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1496C;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1496C: subq.b #$1, $ac2(a5)
pub fn i_c1496c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14970;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C14970: rts
pub fn i_c14970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14972;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C14972: addq.b #$1, $ac2(a5)
pub fn i_c14972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14976;
    cpu.ir = 0x522D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C14976: movea.l $abe(a5), a0
pub fn i_c14976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1497A;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1497A: moveq #$0, d0
pub fn i_c1497a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1497C;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1497C: move.l d0, (a0)+
pub fn i_c1497c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1497E;
    cpu.ir = 0x20C0;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1497E: move.w #$20ff, d1
pub fn i_c1497e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14982;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C14982: jsr $c149a2(pc)
pub fn i_c14982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14986;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14986);
    cpu.pc = 0xC149A2;
    16
}
/// $C14986: moveq #$0, d0
pub fn i_c14986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14988;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14988: move.w $9de(a5), $9d8(a5)
pub fn i_c14988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1498E;
    cpu.ir = 0x3B6D;
    let ad2 = cpu.a[5].wrapping_add(0x9DE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x9D8);
    bus.write16(ad4, v3 as u16);
    20
}
/// $C1498E: move.w #$11ff, d1
pub fn i_c1498e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14992;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C14992: jsr $c149a2(pc)
pub fn i_c14992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14996;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC14996);
    cpu.pc = 0xC149A2;
    16
}
/// $C14996: clr.w (a0)+
pub fn i_c14996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14998;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14998: move.l a0, $abe(a5)
pub fn i_c14998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1499C;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1499C: subq.b #$1, $ac2(a5)
pub fn i_c1499c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149A0;
    cpu.ir = 0x532D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C149A0: rts
pub fn i_c149a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C149A2: move.w #$3, (a0)+
pub fn i_c149a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149A6;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C149A6: move.w $9d8(a5), d0
pub fn i_c149a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149AA;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0x9D8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C149AA: add.w d0, d0
pub fn i_c149aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149AC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C149AC: add.w $10fcd8.l, d0
pub fn i_c149ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149B2;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x10FCD8) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $C149B2: addi.w #$70c9, d0
pub fn i_c149b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149B6;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x70C9, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C149B6: move.w d0, (a0)+
pub fn i_c149b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149B8;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C149B8: move.w #$1108, (a0)+
pub fn i_c149b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149BC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x1108;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C149BC: move.w d1, (a0)+
pub fn i_c149bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149BE;
    cpu.ir = 0x30C1;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C149BE: rts
pub fn i_c149be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149C0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C149C0: moveq #$56, d1
pub fn i_c149c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC149C2;
    cpu.ir = 0x7256;
    cpu.d[1] = 0x56;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14C0E: ori.b #$65, d3
pub fn i_c14c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C12;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x65) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C14C12: movep.w $4d45(a0), d0
pub fn i_c14c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C16;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4D45);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C14C16: ?
pub fn i_c14c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C18;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14C18: addq.w #$1, (a1)+
pub fn i_c14c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C1A;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14C1A: movea.l d3, a0
pub fn i_c14c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C1C;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C14C1C: ?
pub fn i_c14c1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C1E;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14C1E: neg.b -(a0)
pub fn i_c14c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C20;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C14C20: subq.w #$2, (a4)
pub fn i_c14c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C22;
    cpu.ir = 0x5554;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14C22: ?
pub fn i_c14c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C24;
    cpu.ir = 0x494C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14C24: ?
pub fn i_c14c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14C26;
    cpu.ir = 0x4954;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14CB2: ori.b #$9c, d3
pub fn i_c14cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CB6;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x9C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C14CB6: movep.w $4220(a0), d0
pub fn i_c14cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CBA;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4220);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C14CBA: clr.w (a5)
pub fn i_c14cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CBC;
    cpu.ir = 0x4255;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14CBC: addq.w #$2, (a4)
pub fn i_c14cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CBE;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14CBE: ?
pub fn i_c14cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CC0;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14CC0: ?
pub fn i_c14cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CC2;
    cpu.ir = 0x203D;
    let v1: u32 = 0x0;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14CC2: movea.l (a2), a0
pub fn i_c14cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CC4;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $C14CC4: ?
pub fn i_c14cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CC6;
    cpu.ir = 0x4554;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14CC6: subq.w #$2, (a2)
pub fn i_c14cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CC8;
    cpu.ir = 0x5552;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14CC8: ?
pub fn i_c14cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CCA;
    cpu.ir = 0x4E20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14CCA: addq.w #$2, a7
pub fn i_c14cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CCC;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $C14CCC: movea.l a5, a0
pub fn i_c14ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CCE;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C14CCE: ?
pub fn i_c14cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CD0;
    cpu.ir = 0x454E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14CD0: subq.b #$2, -(a0)
pub fn i_c14cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CD2;
    cpu.ir = 0x5520;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C14CD2: subq.w #$1, d3
pub fn i_c14cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CD4;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C14CD4: addq.w #$1, d5
pub fn i_c14cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CD6;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C14CD6: ?
pub fn i_c14cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CD8;
    cpu.ir = 0x454E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14CD8: dc.w $ffff
pub fn i_c14cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CDA;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C14CDA: ori.b #$3, d0
pub fn i_c14cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14CDE;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C14D8E: ori.b #$8a, d3
pub fn i_c14d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14D92;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C14D92: movep.w $4220(a0), d0
pub fn i_c14d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14D96;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4220);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C14D96: clr.w (a5)
pub fn i_c14d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14D98;
    cpu.ir = 0x4255;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14D98: addq.w #$2, (a4)
pub fn i_c14d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14D9A;
    cpu.ir = 0x5454;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14D9A: ?
pub fn i_c14d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14D9C;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14D9C: ?
pub fn i_c14d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14D9E;
    cpu.ir = 0x203D;
    let v1: u32 = 0x0;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C14D9E: movea.l (a2), a0
pub fn i_c14d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DA0;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $C14DA0: ?
pub fn i_c14da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DA2;
    cpu.ir = 0x4554;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14DA2: subq.w #$2, (a2)
pub fn i_c14da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DA4;
    cpu.ir = 0x5552;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C14DA4: ?
pub fn i_c14da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DA6;
    cpu.ir = 0x4E20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14DA6: addq.w #$2, a7
pub fn i_c14da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DA8;
    cpu.ir = 0x544F;
    cpu.a[7] = cpu.a[7].wrapping_add(2);
    8
}
/// $C14DA8: movea.l a5, a0
pub fn i_c14da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DAA;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C14DAA: ?
pub fn i_c14daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DAC;
    cpu.ir = 0x454E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14DAC: subq.b #$2, -(a0)
pub fn i_c14dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DAE;
    cpu.ir = 0x5520;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C14DAE: subq.w #$1, d3
pub fn i_c14dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DB0;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C14DB0: addq.w #$1, d5
pub fn i_c14db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DB2;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C14DB2: ?
pub fn i_c14db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DB4;
    cpu.ir = 0x454E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C14DB4: dc.w $ffff
pub fn i_c14db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DB6;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C14DB6: ori.b #$3, d0
pub fn i_c14db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC14DBA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C150CE: ori.b #$eb, d3
pub fn i_c150ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150D2;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xEB) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C150D2: movep.w $4d45(a0), d0
pub fn i_c150d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150D6;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4D45);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C150D6: ?
pub fn i_c150d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150D8;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C150D8: addq.w #$1, (a1)+
pub fn i_c150d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150DA;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C150DA: movea.l d3, a0
pub fn i_c150da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150DC;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C150DC: ?
pub fn i_c150dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150DE;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C150DE: neg.b -(a0)
pub fn i_c150de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150E0;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C150E0: trap #$f
pub fn i_c150e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150E2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $C150E2: addq.b #$2, -(a0)
pub fn i_c150e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150E4;
    cpu.ir = 0x5420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C150E4: ?
pub fn i_c150e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150E6;
    cpu.ir = 0x464F;
    let v1 = cpu.a[7] & 0xFFFF;
    let r2 = !v1 & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.a[7] = r2 as i16 as i32 as u32;
    4
}
/// $C150E6: addq.w #$1, a5
pub fn i_c150e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150E8;
    cpu.ir = 0x524D;
    cpu.a[5] = cpu.a[5].wrapping_add(1);
    8
}
/// $C150E8: ?
pub fn i_c150e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150EA;
    cpu.ir = 0x4154;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C150EA: addq.w #$2, d5
pub fn i_c150ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150EC;
    cpu.ir = 0x5445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C150EC: neg.b -$1(a6)
pub fn i_c150ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150F0;
    cpu.ir = 0x442E;
    let ad1 = cpu.a[6].wrapping_add(0xFFFFFFFF);
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C150F0: ori.b #$3, d0
pub fn i_c150f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150F4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C150F2: ori.b #$2b, d3
pub fn i_c150f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150F6;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x2B) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C150F6: movep.w $5448(a0), d0
pub fn i_c150f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150FA;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x5448);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C150FA: ?
pub fn i_c150fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150FC;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C150FC: dc.w $4520
pub fn i_c150fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC150FE;
    cpu.ir = 0x4520;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C150FE: ?
pub fn i_c150fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15100;
    cpu.ir = 0x4953;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15100: movea.l a6, a0
pub fn i_c15100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15102;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15102: dc.w $4f20
pub fn i_c15102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15104;
    cpu.ir = 0x4F20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15104: neg.w d1
pub fn i_c15104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15106;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15106: addq.w #$2, d1
pub fn i_c15106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15108;
    cpu.ir = 0x5441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15108: ?
pub fn i_c15108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1510A;
    cpu.ir = 0x2EFF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1510A: ori.b #$3, d0
pub fn i_c1510a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1510E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1510C: ori.b #$2b, d3
pub fn i_c1510c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15110;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x2B) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15110: movep.w $4552(a0), d0
pub fn i_c15110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15114;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4552);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C15114: addq.w #$1, a7
pub fn i_c15114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15116;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C15116: addq.b #$1, -(a0)
pub fn i_c15116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15118;
    cpu.ir = 0x5220;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15118: ?
pub fn i_c15118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1511A;
    cpu.ir = 0x4F4E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1511A: movea.l a5, a0
pub fn i_c1511a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1511C;
    cpu.ir = 0x204D;
    let v1 = cpu.a[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C1511C: ?
pub fn i_c1511c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1511E;
    cpu.ir = 0x454D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1511E: ?
pub fn i_c1511e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15120;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15120: subq.b #$4, -(a0)
pub fn i_c15120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15122;
    cpu.ir = 0x5920;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 4, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15122: ?
pub fn i_c15122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15124;
    cpu.ir = 0x4341;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15124: addq.w #$1, d4
pub fn i_c15124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15126;
    cpu.ir = 0x5244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15126: ?
pub fn i_c15126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15128;
    cpu.ir = 0x2EFF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15128: ori.b #$3, d0
pub fn i_c15128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1512C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1512A: ori.b #$2b, d3
pub fn i_c1512a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1512E;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x2B) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1512E: movep.w $4d45(a0), d0
pub fn i_c1512e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15132;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4D45);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C15132: ?
pub fn i_c15132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15134;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15134: addq.w #$1, (a1)+
pub fn i_c15134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15136;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15136: movea.l d3, a0
pub fn i_c15136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15138;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15138: ?
pub fn i_c15138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1513A;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1513A: neg.b -(a0)
pub fn i_c1513a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1513C;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C1513C: not.w (a5)
pub fn i_c1513c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1513E;
    cpu.ir = 0x4655;
    let ad1 = cpu.a[5];
    let v2 = bus.read16(ad1) as u32;
    let r3 = !v2 & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1513E: ?
pub fn i_c1513e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15140;
    cpu.ir = 0x4C4C;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15140: ?
pub fn i_c15140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15142;
    cpu.ir = 0x2EFF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15142: ori.b #$3, d0
pub fn i_c15142(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15146;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15144: ori.b #$eb, d3
pub fn i_c15144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15148;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0xEB) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15146: moveq #$eb, d0
pub fn i_c15146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15148;
    cpu.ir = 0x70EB;
    cpu.d[0] = 0xFFFFFFEB;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15148: movep.w $4d45(a0), d0
pub fn i_c15148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1514C;
    cpu.ir = 0x108;
    let a = cpu.a[0].wrapping_add(0x4D45);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1514C: ?
pub fn i_c1514c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1514E;
    cpu.ir = 0x4D4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1514E: addq.w #$1, (a1)+
pub fn i_c1514e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15150;
    cpu.ir = 0x5259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15150: movea.l d3, a0
pub fn i_c15150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15152;
    cpu.ir = 0x2043;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15152: ?
pub fn i_c15152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15154;
    cpu.ir = 0x4152;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15154: neg.b -(a0)
pub fn i_c15154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15156;
    cpu.ir = 0x4420;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15156: subq.w #$3, (a2)
pub fn i_c15156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15158;
    cpu.ir = 0x5752;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15158: ?
pub fn i_c15158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1515A;
    cpu.ir = 0x4954;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1515A: dc.w $4520
pub fn i_c1515a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1515C;
    cpu.ir = 0x4520;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1515C: addq.w #$8, (a2)
pub fn i_c1515c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1515E;
    cpu.ir = 0x5052;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1515E: ?
pub fn i_c1515e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15160;
    cpu.ir = 0x4F54;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15160: ?
pub fn i_c15160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15162;
    cpu.ir = 0x4543;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15162: addq.w #$2, d5
pub fn i_c15162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15164;
    cpu.ir = 0x5445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15164: neg.b -$1(a6)
pub fn i_c15164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15168;
    cpu.ir = 0x442E;
    let ad1 = cpu.a[6].wrapping_add(0xFFFFFFFF);
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    12
}
/// $C15168: ori.b #$79, d0
pub fn i_c15168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1516C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x79) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1516C: ori.b #$4c, (a0)
pub fn i_c1516c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15170;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x4C) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C15170: bra.b $c1517a
pub fn i_c15170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15172;
    cpu.ir = 0x6008;
    cpu.pc = 0xC1517A;
    10
}
/// $C15172: move.w #$4, $10fd4c.l
pub fn i_c15172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1517A;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FD4C, v1 as u16);
    16
}
/// $C1517A: ori.w #$700, sr
pub fn i_c1517a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1517E;
    cpu.ir = 0x7C;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let h1 = cpu.sr.0 | 0x700;
    write_sr(cpu, h1);
    20
}
/// $C1517E: movem.l d0-d7/a0-a6, $10fd00.l
pub fn i_c1517e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15186;
    cpu.ir = 0x48F9;
    let mut a = 0x10FD00;
    bus.write32(a, cpu.d[0]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[1]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[2]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[3]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[4]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[5]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[6]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[7]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[0]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[1]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[2]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[3]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[4]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[5]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[6]); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $C15186: lea.l $10ff00.l, a0
pub fn i_c15186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1518C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FF00;
    20
}
/// $C1518C: clr.l (a0)+
pub fn i_c1518c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1518E;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C1518E: move.l #$1f0ff, (a0)+
pub fn i_c1518e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15194;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15194: move.w #$2002, (a0)+
pub fn i_c15194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15198;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15198: move.l #$37085, (a0)+
pub fn i_c15198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1519E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x37085;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1519E: move.w #$7, (a0)+
pub fn i_c1519e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151A2;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C151A2: movea.l a7, a1
pub fn i_c151a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151A4;
    cpu.ir = 0x224F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C151A4: jsr $c15a7c(pc)
pub fn i_c151a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151A8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC151A8);
    cpu.pc = 0xC15A7C;
    16
}
/// $C151A8: jsr $c15a7c(pc)
pub fn i_c151a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC151AC);
    cpu.pc = 0xC15A7C;
    16
}
/// $C151AC: move.w #$2020, (a0)+
pub fn i_c151ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151B0;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C151B0: jsr $c15a7c(pc)
pub fn i_c151b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC151B4);
    cpu.pc = 0xC15A7C;
    16
}
/// $C151B4: jsr $c15a7c(pc)
pub fn i_c151b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC151B8);
    cpu.pc = 0xC15A7C;
    16
}
/// $C151B8: jsr $c15a7c(pc)
pub fn i_c151b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC151BC);
    cpu.pc = 0xC15A7C;
    16
}
/// $C151BC: jsr $c15a7c(pc)
pub fn i_c151bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC151C0);
    cpu.pc = 0xC15A7C;
    16
}
/// $C151C0: move.w #$2020, (a0)+
pub fn i_c151c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151C4;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C151C4: jsr $c15a7c(pc)
pub fn i_c151c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC151C8);
    cpu.pc = 0xC15A7C;
    16
}
/// $C151C8: jsr $c15a7c(pc)
pub fn i_c151c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC151CC);
    cpu.pc = 0xC15A7C;
    16
}
/// $C151CC: move.l #$ff000000, (a0)+
pub fn i_c151cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151D2;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFF000000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C151D2: addq.l #$8, a7
pub fn i_c151d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151D4;
    cpu.ir = 0x508F;
    cpu.a[7] = cpu.a[7].wrapping_add(8);
    8
}
/// $C151D4: bra.b $c15222
pub fn i_c151d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151D6;
    cpu.ir = 0x604C;
    cpu.pc = 0xC15222;
    10
}
/// $C151D6: move.w #$8, $10fd4c.l
pub fn i_c151d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151DE;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FD4C, v1 as u16);
    16
}
/// $C151DE: bra.b $c15210
pub fn i_c151de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151E0;
    cpu.ir = 0x6030;
    cpu.pc = 0xC15210;
    10
}
/// $C151E0: move.w #$c, $10fd4c.l
pub fn i_c151e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151E8;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FD4C, v1 as u16);
    16
}
/// $C151E8: bra.b $c15210
pub fn i_c151e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC151EA;
    cpu.ir = 0x6026;
    cpu.pc = 0xC15210;
    10
}
/// $C15210: ori.w #$700, sr
pub fn i_c15210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15214;
    cpu.ir = 0x7C;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let h1 = cpu.sr.0 | 0x700;
    write_sr(cpu, h1);
    20
}
/// $C15214: movem.l d0-d7/a0-a6, $10fd00.l
pub fn i_c15214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1521C;
    cpu.ir = 0x48F9;
    let mut a = 0x10FD00;
    bus.write32(a, cpu.d[0]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[1]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[2]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[3]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[4]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[5]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[6]); a = a.wrapping_add(4);
    bus.write32(a, cpu.d[7]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[0]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[1]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[2]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[3]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[4]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[5]); a = a.wrapping_add(4);
    bus.write32(a, cpu.a[6]); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $C1521C: lea.l $10ff00.l, a0
pub fn i_c1521c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15222;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FF00;
    20
}
/// $C15222: lea.l $10f300.l, a5
pub fn i_c15222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15228;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C15228: move usp, a3
pub fn i_c15228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1522A;
    cpu.ir = 0x4E6B;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.a[3] = cpu.usp;
    4
}
/// $C1522A: move.l a3, $a40(a5)
pub fn i_c1522a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1522E;
    cpu.ir = 0x2B4B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1522E: move.w (a7)+, $a48(a5)
pub fn i_c1522e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15232;
    cpu.ir = 0x3B5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xA48);
    bus.write16(ad4, v3 as u16);
    16
}
/// $C15232: move.l (a7)+, $a44(a5)
pub fn i_c15232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15236;
    cpu.ir = 0x2B5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xA44);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    24
}
/// $C15236: move.l a7, $a3c(a5)
pub fn i_c15236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1523A;
    cpu.ir = 0x2B4F;
    let v1 = cpu.a[7] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1523A: clr.w $a4a(a5)
pub fn i_c1523a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1523E;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xA4A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1523E: move.l $4001e2.l, $a4e(a5)
pub fn i_c1523e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15246;
    cpu.ir = 0x2B79;
    let v2 = bus.read32(0x4001E2);
    let ad3 = cpu.a[5].wrapping_add(0xA4E);
    bus.write32(ad3, v2);
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C15246: move.l #$eee0000, $4001e2.l
pub fn i_c15246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15250;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x4001E2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C15250: move.b d0, $3a0001.l
pub fn i_c15250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15256;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0001, v1 as u8);
    16
}
/// $C15256: clr.b $ac2(a5)
pub fn i_c15256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1525A;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C1525A: lea.l $c15da0(pc), a1
pub fn i_c1525a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1525E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC15DA0;
    16
}
/// $C1525E: move.w $a4c(a5), d0
pub fn i_c1525e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15262;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA4C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C15262: move.l (a1, d0.w), (a0)+
pub fn i_c15262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15266;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C15266: jsr $c15946(pc)
pub fn i_c15266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1526A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1526A);
    cpu.pc = 0xC15946;
    16
}
/// $C1526A: move.b d0, $300001.l
pub fn i_c1526a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15270;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15270: clr.w $a74(a5)
pub fn i_c15270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15274;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xA74);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C15274: move.l #$100000, $a5e(a5)
pub fn i_c15274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1527C;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x100000;
    let ad2 = cpu.a[5].wrapping_add(0xA5E);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1527C: move.l #$100000, $a66(a5)
pub fn i_c1527c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15284;
    cpu.ir = 0x2B7C;
    let v1: u32 = 0x100000;
    let ad2 = cpu.a[5].wrapping_add(0xA66);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15284: clr.l $a62(a5)
pub fn i_c15284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15288;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0xA62);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C15288: clr.l $a6a(a5)
pub fn i_c15288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1528C;
    cpu.ir = 0x42AD;
    let ad1 = cpu.a[5].wrapping_add(0xA6A);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $C1528C: clr.w $a6e(a5)
pub fn i_c1528c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15290;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xA6E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C15290: btst.b #$7, $a48(a5)
pub fn i_c15290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15296;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA48);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C15296: beq.b $c152a0
pub fn i_c15296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15298;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC152A0; return 10; }
    8
}
/// $C15298: move.w #$5, $10fd74.l
pub fn i_c15298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152A0;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10FD74, v1 as u16);
    16
}
/// $C152A0: move.w $a74(a5), d0
pub fn i_c152a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152A4;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C152A4: cmpi.w #$2, d0
pub fn i_c152a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152A8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x2, Size::Word, false);
    8
}
/// $C152A8: bcs.b $c152e2
pub fn i_c152a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152AA;
    cpu.ir = 0x6538;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC152E2; return 10; }
    8
}
/// $C152AA: cmpi.w #$5, d0
pub fn i_c152aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152AE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $C152AE: bcc.b $c152e2
pub fn i_c152ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152B0;
    cpu.ir = 0x6432;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC152E2; return 10; }
    8
}
/// $C152B0: cmpi.w #$3, d0
pub fn i_c152b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152B4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x3, Size::Word, false);
    8
}
/// $C152B4: beq.b $c152ca
pub fn i_c152b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152B6;
    cpu.ir = 0x6714;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC152CA; return 10; }
    8
}
/// $C152B6: bcc.b $c152c2
pub fn i_c152b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152B8;
    cpu.ir = 0x640A;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC152C2; return 10; }
    8
}
/// $C152B8: movea.l $a66(a5), a6
pub fn i_c152b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152BC;
    cpu.ir = 0x2C6D;
    let ad2 = cpu.a[5].wrapping_add(0xA66);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    16
}
/// $C152BC: move.w (a6), $a70(a5)
pub fn i_c152bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152C0;
    cpu.ir = 0x3B56;
    let v2 = bus.read16(cpu.a[6]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $C152C0: bra.b $c152e2
pub fn i_c152c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152C2;
    cpu.ir = 0x6020;
    cpu.pc = 0xC152E2;
    10
}
/// $C152C2: move.l $a00(a5), $a70(a5)
pub fn i_c152c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152C8;
    cpu.ir = 0x2B6D;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xA70);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $C152C8: bra.b $c152e2
pub fn i_c152c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152CA;
    cpu.ir = 0x6018;
    cpu.pc = 0xC152E2;
    10
}
/// $C152CA: move.w $a6c(a5), $3c0000.l
pub fn i_c152ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152D2;
    cpu.ir = 0x33ED;
    let ad2 = cpu.a[5].wrapping_add(0xA6C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v3 as u16);
    24
}
/// $C152D2: nop
pub fn i_c152d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152D4;
    cpu.ir = 0x4E71;
    4
}
/// $C152D4: nop
pub fn i_c152d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152D6;
    cpu.ir = 0x4E71;
    4
}
/// $C152D6: nop
pub fn i_c152d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152D8;
    cpu.ir = 0x4E71;
    4
}
/// $C152D8: nop
pub fn i_c152d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152DA;
    cpu.ir = 0x4E71;
    4
}
/// $C152DA: move.w $3c0002.l, $a70(a5)
pub fn i_c152da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152E2;
    cpu.ir = 0x3B79;
    let v2 = bus.read16(0x3C0002) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA70);
    bus.write16(ad3, v2 as u16);
    24
}
/// $C152E2: clr.w $a76(a5)
pub fn i_c152e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152E6;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xA76);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C152E6: move.b $c15302(pc, d0.w), $a77(a5)
pub fn i_c152e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152EC;
    cpu.ir = 0x1B7B;
    let ad2 = 0xC15302_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xA77);
    bus.write8(ad4, v3 as u8);
    22
}
/// $C152EC: lsl.w #$2, d0
pub fn i_c152ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152EE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C152EE: movea.l $abe(a5), a0
pub fn i_c152ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152F2;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C152F2: move.l #$c15fc6, (a0)+
pub fn i_c152f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152F8;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xC15FC6;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C152F8: move.l $c1530a(pc, d0.w), (a0)+
pub fn i_c152f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC152FC;
    cpu.ir = 0x20FB;
    let ad2 = 0xC1530A_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C152FC: move.l a0, $abe(a5)
pub fn i_c152fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15300;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15300: bra.b $c15326
pub fn i_c15300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15302;
    cpu.ir = 0x6024;
    cpu.pc = 0xC15326;
    10
}
/// $C15302: subi.b #$5, d4
pub fn i_c15302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15306;
    cpu.ir = 0x404;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15306: ori.b #$0, d0
pub fn i_c15306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1530A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1530A: ?
pub fn i_c1530a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1530C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1530C: addq.l #$7, -$3f(a2, d0.w)
pub fn i_c1530c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15310;
    cpu.ir = 0x5EB2;
    let ad1 = cpu.a[2].wrapping_add(0xFFFFFFC1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 7, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C15310: sgt.b (a0)
pub fn i_c15310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15312;
    cpu.ir = 0x5ED0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Gt) { 0xFF } else { 0 };
    let ad2 = cpu.a[0];
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    8
}
/// $C15312: ?
pub fn i_c15312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15314;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15314: sgt.b $c1.w
pub fn i_c15314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15318;
    cpu.ir = 0x5EF8;
    let c1: u32 = if cpu.sr.evaluate(Condition::Gt) { 0xFF } else { 0 };
    let v2 = bus.read8(0xC1) as u32;
    bus.write8(0xC1, c1 as u8);
    12
}
/// $C15318: subq.b #$7, -(a0)
pub fn i_c15318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1531A;
    cpu.ir = 0x5F20;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 7, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C1531A: ?
pub fn i_c1531a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1531C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1531C: subq.w #$7, d6
pub fn i_c1531c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1531E;
    cpu.ir = 0x5F46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 7, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1531E: ?
pub fn i_c1531e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15320;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15320: ?
pub fn i_c15320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15324;
    cpu.ir = 0x5F7A;
    let v2 = bus.read16(0xC153E3) as u32;
    let r3 = sub_with_flags(cpu, v2, 7, Size::Word, true);
    12
}
/// $C15324: subq.l #$7, (invalid.w)
pub fn i_c15324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15328;
    cpu.ir = 0x5FB2;
    let ad1 = cpu.a[2].wrapping_add(0xFFFFFFC0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let r3 = sub_with_flags(cpu, v2, 7, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C15326: move.b d0, $300001.l
pub fn i_c15326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1532C;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15328: ori.b #$1, $7003c(invalid.w)
pub fn i_c15328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1532E;
    cpu.ir = 0x30;
    let ad1 = cpu.a[0].wrapping_add(0xFFFFFFFC).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    18
}
/// $C1532C: move.w #$7, $3c000c.l
pub fn i_c1532c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15334;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C1532E: ori.b #$3c, d7
pub fn i_c1532e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15332;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x3C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15332: ?
pub fn i_c15332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15336;
    cpu.ir = 0xC;
    let v1 = cpu.a[4] & 0xFF;
    let r2 = (v1 | 0x39) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[4] = r2;
    8
}
/// $C15334: move.w $3c0006.l, d0
pub fn i_c15334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1533A;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x3C0006) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $C15336: ori.b #$6, ccr
pub fn i_c15336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1533A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x6);
    20
}
/// $C1533A: andi.w #$7f80, d0
pub fn i_c1533a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1533E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F80) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $C1533E: bne.b $c15334
pub fn i_c1533e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15340;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15334; return 10; }
    8
}
/// $C15340: jsr $c17290(pc)
pub fn i_c15340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15344;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15344);
    cpu.pc = 0xC17290;
    16
}
/// $C15344: lea.l $10f300.l, a5
pub fn i_c15344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1534A;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C1534A: addq.w #$1, $a78(a5)
pub fn i_c1534a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1534E;
    cpu.ir = 0x526D;
    let ad1 = cpu.a[5].wrapping_add(0xA78);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1534E: jsr $c17080(pc)
pub fn i_c1534e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15352;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15352);
    cpu.pc = 0xC17080;
    16
}
/// $C15352: move.w $a74(a5), d0
pub fn i_c15352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15356;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C15356: move.b $a98(a5), d3
pub fn i_c15356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1535A;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1535A: btst.b #$4, d3
pub fn i_c1535a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1535E;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((4) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1535E: beq.b $c15372
pub fn i_c1535e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15360;
    cpu.ir = 0x6712;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15372; return 10; }
    8
}
/// $C15360: addq.w #$1, d0
pub fn i_c15360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15362;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15362: cmpi.w #$7, d0
pub fn i_c15362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15366;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7, Size::Word, false);
    8
}
/// $C15366: bcs.b $c1536a
pub fn i_c15366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15368;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC1536A; return 10; }
    8
}
/// $C15368: moveq #$0, d0
pub fn i_c15368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1536A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1536A: move.w d0, $a74(a5)
pub fn i_c1536a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1536E;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C1536E: bra.w $c152a0
pub fn i_c1536e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15372;
    cpu.ir = 0x6000;
    cpu.pc = 0xC152A0;
    10
}
/// $C15372: pea.l $c15326(pc)
pub fn i_c15372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15376;
    cpu.ir = 0x487A;
    cpu.push32(bus, 0xC15326);
    24
}
/// $C15376: lsl.w #$2, d0
pub fn i_c15376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15378;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C15378: movea.l $c1537e(pc, d0.w), a0
pub fn i_c15378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1537C;
    cpu.ir = 0x207B;
    let ad2 = 0xC1537E_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $C1537C: jmp (a0)
pub fn i_c1537c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1537E;
    cpu.ir = 0x4ED0;
    cpu.jump_to(cpu.a[0]);
    8
}
/// $C1537E: ?
pub fn i_c1537e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15380;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15380: subq.l #$1, (a2)+
pub fn i_c15380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15382;
    cpu.ir = 0x539A;
    let ad1 = cpu.a[2]; cpu.a[2] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    let r3 = sub_with_flags(cpu, v2, 1, Size::Long, true);
    bus.write32(ad1, r3);
    12
}
/// $C15382: ?
pub fn i_c15382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15384;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15384: addq.w #$2, $c1(a2)
pub fn i_c15384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15388;
    cpu.ir = 0x546A;
    let ad1 = cpu.a[2].wrapping_add(0xC1);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C15388: subq.l #$2, d4
pub fn i_c15388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1538A;
    cpu.ir = 0x5584;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Long, true);
    cpu.d[4] = r2;
    4
}
/// $C1538A: ?
pub fn i_c1538a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1538C;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1538C: addq.w #$3, d0
pub fn i_c1538c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1538E;
    cpu.ir = 0x5640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1538E: ?
pub fn i_c1538e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15390;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15390: ?
pub fn i_c15390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15392;
    cpu.ir = 0x573E;
    let v1: u32 = 0x0;
    let r2 = sub_with_flags(cpu, v1, 3, Size::Byte, true);
    4
}
/// $C15392: ?
pub fn i_c15392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15394;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15394: addq.l #$4, -$3f(a4, d0.w)
pub fn i_c15394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15398;
    cpu.ir = 0x58B4;
    let ad1 = cpu.a[4].wrapping_add(0xFFFFFFC1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 4, Size::Long, true);
    bus.write32(ad1, r3);
    18
}
/// $C15398: svc.b $43ed(a6)
pub fn i_c15398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1539C;
    cpu.ir = 0x58EE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Vc) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0x43ED);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $C1539A: lea.l $a5f(a5), a1
pub fn i_c1539a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1539E;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA5F);
    cpu.a[1] = ad1;
    16
}
/// $C1539C: eori.w #$4eba, (a7)+
pub fn i_c1539c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153A0;
    cpu.ir = 0xA5F;
    let ad1 = cpu.a[7]; cpu.a[7] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 ^ 0x4EBA) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1539E: jsr $c15a1a(pc)
pub fn i_c1539e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153A2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC153A2);
    cpu.pc = 0xC15A1A;
    16
}
/// $C153A0: ?
pub fn i_c153a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153A6;
    cpu.ir = 0x67A;
    let v2 = bus.read16(0xC158F8) as u32;
    let r3 = add_with_flags(cpu, v2, 0x4EBA, Size::Word, true);
    16
}
/// $C153A2: jsr $c158f8(pc)
pub fn i_c153a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC153A6);
    cpu.pc = 0xC158F8;
    16
}
/// $C153A6: move.b $a98(a5), d3
pub fn i_c153a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153AA;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C153AA: andi.b #$c, d3
pub fn i_c153aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153AE;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C153AE: beq.w $c153d2
pub fn i_c153ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC153D2; return 10; }
    8
}
/// $C153B2: moveq #$1, d2
pub fn i_c153b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153B4;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C153B4: andi.b #$8, d3
pub fn i_c153b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153B8;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C153B8: bne.b $c153bc
pub fn i_c153b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153BA;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC153BC; return 10; }
    8
}
/// $C153BA: moveq #$ff, d2
pub fn i_c153ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153BC;
    cpu.ir = 0x74FF;
    cpu.d[2] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C153BC: add.w $a76(a5), d2
pub fn i_c153bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153C0;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C153C0: bpl.b $c153c4
pub fn i_c153c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153C2;
    cpu.ir = 0x6A02;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC153C4; return 10; }
    8
}
/// $C153C2: moveq #$4, d2
pub fn i_c153c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153C4;
    cpu.ir = 0x7404;
    cpu.d[2] = 0x4;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C153C4: cmpi.w #$5, d2
pub fn i_c153c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153C8;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $C153C8: bcs.b $c153cc
pub fn i_c153c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153CA;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC153CC; return 10; }
    8
}
/// $C153CA: moveq #$0, d2
pub fn i_c153ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153CC;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C153CC: move.w d2, $a76(a5)
pub fn i_c153cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153D0;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C153D0: rts
pub fn i_c153d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C153D2: moveq #$3, d3
pub fn i_c153d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153D4;
    cpu.ir = 0x7603;
    cpu.d[3] = 0x3;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C153D4: and.b $a98(a5), d3
pub fn i_c153d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153D8;
    cpu.ir = 0xC62D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C153D8: beq.w $c15414
pub fn i_c153d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15414; return 10; }
    8
}
/// $C153DC: move.l $a5e(a5), d0
pub fn i_c153dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153E0;
    cpu.ir = 0x202D;
    let ad2 = cpu.a[5].wrapping_add(0xA5E);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C153E0: move.w $a76(a5), d2
pub fn i_c153e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153E4;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C153E4: lsl.w #$2, d2
pub fn i_c153e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153E6;
    cpu.ir = 0xE54A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C153E6: move.l $c15400(pc, d2.w), d2
pub fn i_c153e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153EA;
    cpu.ir = 0x243B;
    let ad2 = 0xC15400_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C153EA: andi.b #$1, d3
pub fn i_c153ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153EE;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C153EE: beq.b $c153f2
pub fn i_c153ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153F0;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC153F2; return 10; }
    8
}
/// $C153F0: neg.l d2
pub fn i_c153f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153F2;
    cpu.ir = 0x4482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.set(StatusRegister::V, v1 == 0x80000000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = r2;
    4
}
/// $C153F2: add.l d2, d0
pub fn i_c153f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153F4;
    cpu.ir = 0xD082;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $C153F4: andi.l #$ffffff, d0
pub fn i_c153f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153FA;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C153FA: move.l d0, $a5e(a5)
pub fn i_c153fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC153FE;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA5E);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C153FE: rts
pub fn i_c153fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15400;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15400: ori.b #$0, (a0)
pub fn i_c15400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15404;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C15404: ori.b #$0, d1
pub fn i_c15404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15408;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15408: ori.b #$0, d0
pub fn i_c15408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1540C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1540C: ori.b #$0, d0
pub fn i_c1540c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15410;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15410: ori.b #$40, d0
pub fn i_c15410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15414;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15414: btst.b #$6, $a98(a5)
pub fn i_c15414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1541A;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C1541A: bne.b $c1541e
pub fn i_c1541a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1541C;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1541E; return 10; }
    8
}
/// $C1541C: rts
pub fn i_c1541c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1541E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1541E: movea.l $a5e(a5), a1
pub fn i_c1541e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15422;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA5E);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C15422: movea.l $abe(a5), a0
pub fn i_c15422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15426;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C15426: clr.l (a0)+
pub fn i_c15426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15428;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15428: move.l #$1f0ff, (a0)+
pub fn i_c15428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1542E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1542E: move.w #$2002, (a0)+
pub fn i_c1542e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15432;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15432: move.l #$37070, (a0)+
pub fn i_c15432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15438;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x37070;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15438: moveq #$7, d7
pub fn i_c15438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1543A;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1543A: move.w #$7, (a0)+
pub fn i_c1543a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1543E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1543E: moveq #$3, d6
pub fn i_c1543e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15440;
    cpu.ir = 0x7C03;
    cpu.d[6] = 0x3;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15440: jsr $c15a7c(pc)
pub fn i_c15440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15444;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15444);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15444: jsr $c15a7c(pc)
pub fn i_c15444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15448;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15448);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15448: move.b #$20, (a0)+
pub fn i_c15448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1544C;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C1544C: dbra d6, $c15440
pub fn i_c1544c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15450;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15450; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15450; return 14; }
    cpu.pc = 0xC15440;
    10
}
/// $C15450: move.b #$ff, -$1(a0)
pub fn i_c15450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15456;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C15456: move.l #$50001, (a0)+
pub fn i_c15456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1545C;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x50001;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1545C: dbra d7, $c1543a
pub fn i_c1545c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15460;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15460; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15460; return 14; }
    cpu.pc = 0xC1543A;
    10
}
/// $C15460: subq.l #$4, a0
pub fn i_c15460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15462;
    cpu.ir = 0x5988;
    cpu.a[0] = cpu.a[0].wrapping_sub(4);
    8
}
/// $C15462: clr.w (a0)+
pub fn i_c15462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15464;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15464: move.l a0, $abe(a5)
pub fn i_c15464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15468;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15468: rts
pub fn i_c15468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1546A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1546A: lea.l $a63(a5), a1
pub fn i_c1546a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1546E;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA63);
    cpu.a[1] = ad1;
    16
}
/// $C1546E: jsr $c15a1a(pc)
pub fn i_c1546e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15472;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15472);
    cpu.pc = 0xC15A1A;
    16
}
/// $C15472: jsr $c158f8(pc)
pub fn i_c15472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15476;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15476);
    cpu.pc = 0xC158F8;
    16
}
/// $C15476: move.b $a98(a5), d3
pub fn i_c15476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1547A;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1547A: andi.b #$c, d3
pub fn i_c1547a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1547E;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1547E: beq.w $c154a6
pub fn i_c1547e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15482;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC154A6; return 10; }
    8
}
/// $C15482: moveq #$1, d2
pub fn i_c15482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15484;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15484: andi.b #$8, d3
pub fn i_c15484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15488;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15488: bne.b $c1548c
pub fn i_c15488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1548A;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC1548C; return 10; }
    8
}
/// $C1548A: moveq #$ff, d2
pub fn i_c1548a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1548C;
    cpu.ir = 0x74FF;
    cpu.d[2] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1548C: add.w $a76(a5), d2
pub fn i_c1548c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15490;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C15490: cmpi.w #$1, d2
pub fn i_c15490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15494;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1, Size::Word, false);
    8
}
/// $C15494: bne.b $c15498
pub fn i_c15494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15496;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15498; return 10; }
    8
}
/// $C15496: moveq #$4, d2
pub fn i_c15496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15498;
    cpu.ir = 0x7404;
    cpu.d[2] = 0x4;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15498: cmpi.w #$5, d2
pub fn i_c15498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1549C;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x5, Size::Word, false);
    8
}
/// $C1549C: bcs.b $c154a0
pub fn i_c1549c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1549E;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC154A0; return 10; }
    8
}
/// $C1549E: moveq #$2, d2
pub fn i_c1549e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154A0;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C154A0: move.w d2, $a76(a5)
pub fn i_c154a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154A4;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C154A4: rts
pub fn i_c154a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C154A6: moveq #$3, d3
pub fn i_c154a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154A8;
    cpu.ir = 0x7603;
    cpu.d[3] = 0x3;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C154A8: and.b $a98(a5), d3
pub fn i_c154a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154AC;
    cpu.ir = 0xC62D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C154AC: beq.w $c15508
pub fn i_c154ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15508; return 10; }
    8
}
/// $C154B0: move.l $a62(a5), d0
pub fn i_c154b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154B4;
    cpu.ir = 0x202D;
    let ad2 = cpu.a[5].wrapping_add(0xA62);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C154B4: move.w $a76(a5), d2
pub fn i_c154b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154B8;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C154B8: lsl.w #$2, d2
pub fn i_c154b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154BA;
    cpu.ir = 0xE54A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C154BA: move.l $c154f4(pc, d2.w), d2
pub fn i_c154ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154BE;
    cpu.ir = 0x243B;
    let ad2 = 0xC154F4_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C154BE: andi.b #$1, d3
pub fn i_c154be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154C2;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C154C2: beq.b $c154c6
pub fn i_c154c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154C4;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC154C6; return 10; }
    8
}
/// $C154C4: neg.l d2
pub fn i_c154c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154C6;
    cpu.ir = 0x4482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.set(StatusRegister::V, v1 == 0x80000000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = r2;
    4
}
/// $C154C6: add.l d2, d0
pub fn i_c154c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154C8;
    cpu.ir = 0xD082;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $C154C8: bmi.b $c154e8
pub fn i_c154c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154CA;
    cpu.ir = 0x6B1E;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC154E8; return 10; }
    8
}
/// $C154CA: cmpi.l #$8580, d0
pub fn i_c154ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154D0;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x8580, Size::Long, false);
    8
}
/// $C154D0: bcs.b $c154d6
pub fn i_c154d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154D2;
    cpu.ir = 0x6504;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC154D6; return 10; }
    8
}
/// $C154D2: moveq #$0, d0
pub fn i_c154d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154D4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C154D4: bra.b $c154ee
pub fn i_c154d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154D6;
    cpu.ir = 0x6018;
    cpu.pc = 0xC154EE;
    10
}
/// $C154D6: cmpi.l #$8000, d0
pub fn i_c154d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154DC;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x8000, Size::Long, false);
    8
}
/// $C154DC: bcc.b $c154ee
pub fn i_c154dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154DE;
    cpu.ir = 0x6410;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC154EE; return 10; }
    8
}
/// $C154DE: cmpi.l #$73e1, d0
pub fn i_c154de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154E4;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x73E1, Size::Long, false);
    8
}
/// $C154E4: bcc.b $c154c6
pub fn i_c154e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154E6;
    cpu.ir = 0x64E0;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC154C6; return 10; }
    8
}
/// $C154E6: bra.b $c154ee
pub fn i_c154e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154E8;
    cpu.ir = 0x6006;
    cpu.pc = 0xC154EE;
    10
}
/// $C154E8: move.l #$8560, d0
pub fn i_c154e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154EE;
    cpu.ir = 0x203C;
    let v1: u32 = 0x8560;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C154EE: move.l d0, $a62(a5)
pub fn i_c154ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154F2;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA62);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C154F2: rts
pub fn i_c154f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C154F4: ori.b #$0, d0
pub fn i_c154f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154F8;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C154F8: ori.b #$0, d0
pub fn i_c154f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC154FC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C154FC: ori.b #$0, d0
pub fn i_c154fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15500;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15500: ori.b #$0, d0
pub fn i_c15500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15504;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15504: ori.b #$20, d0
pub fn i_c15504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15508;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x20) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15508: btst.b #$6, $a98(a5)
pub fn i_c15508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1550E;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C1550E: bne.b $c15512
pub fn i_c1550e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15510;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15512; return 10; }
    8
}
/// $C15510: rts
pub fn i_c15510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15512;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15512: lea.l $3c0002.l, a1
pub fn i_c15512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15518;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3C0002;
    20
}
/// $C15518: move.w $a64(a5), d3
pub fn i_c15518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1551C;
    cpu.ir = 0x362D;
    let ad2 = cpu.a[5].wrapping_add(0xA64);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1551C: movea.l $abe(a5), a0
pub fn i_c1551c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15520;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C15520: clr.l (a0)+
pub fn i_c15520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15522;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15522: move.l #$1f0ff, (a0)+
pub fn i_c15522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15528;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15528: move.w #$2002, (a0)+
pub fn i_c15528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1552C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1552C: move.l #$37070, (a0)+
pub fn i_c1552c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15532;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x37070;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15532: moveq #$7, d7
pub fn i_c15532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15534;
    cpu.ir = 0x7E07;
    cpu.d[7] = 0x7;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15534: move.w #$7, (a0)+
pub fn i_c15534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15538;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15538: moveq #$3, d6
pub fn i_c15538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1553A;
    cpu.ir = 0x7C03;
    cpu.d[6] = 0x3;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1553A: jsr $c15562(pc)
pub fn i_c1553a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1553E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1553E);
    cpu.pc = 0xC15562;
    16
}
/// $C1553E: addq.w #$1, d3
pub fn i_c1553e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15540;
    cpu.ir = 0x5243;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15540: move.b #$20, (a0)+
pub fn i_c15540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15544;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15544: dbra d6, $c1553a
pub fn i_c15544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15548;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15548; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15548; return 14; }
    cpu.pc = 0xC1553A;
    10
}
/// $C15548: move.b #$ff, -$1(a0)
pub fn i_c15548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1554E;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C1554E: move.l #$50001, (a0)+
pub fn i_c1554e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15554;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x50001;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15554: dbra d7, $c15534
pub fn i_c15554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15558;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15558; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15558; return 14; }
    cpu.pc = 0xC15534;
    10
}
/// $C15558: subq.l #$4, a0
pub fn i_c15558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1555A;
    cpu.ir = 0x5988;
    cpu.a[0] = cpu.a[0].wrapping_sub(4);
    8
}
/// $C1555A: clr.w (a0)+
pub fn i_c1555a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1555C;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C1555C: move.l a0, $abe(a5)
pub fn i_c1555c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15560;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15560: rts
pub fn i_c15560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15562;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15562: move.w d3, -$2(a1)
pub fn i_c15562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15566;
    cpu.ir = 0x3343;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0xFFFFFFFE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C15566: nop
pub fn i_c15566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15568;
    cpu.ir = 0x4E71;
    4
}
/// $C15568: nop
pub fn i_c15568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1556A;
    cpu.ir = 0x4E71;
    4
}
/// $C1556A: nop
pub fn i_c1556a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1556C;
    cpu.ir = 0x4E71;
    4
}
/// $C1556C: nop
pub fn i_c1556c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1556E;
    cpu.ir = 0x4E71;
    4
}
/// $C1556E: move.w (a1), d2
pub fn i_c1556e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15570;
    cpu.ir = 0x3411;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $C15570: move.w d2, d0
pub fn i_c15570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15572;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15572: lsr.w #$8, d0
pub fn i_c15572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15574;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $C15574: jsr $c1557a(pc)
pub fn i_c15574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15578;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15578);
    cpu.pc = 0xC1557A;
    16
}
/// $C15578: move.w d2, d0
pub fn i_c15578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1557A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C1557A: jsr $c16f54(pc)
pub fn i_c1557a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1557E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1557E);
    cpu.pc = 0xC16F54;
    16
}
/// $C1557E: move.b d0, (a0)+
pub fn i_c1557e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15580;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15580: move.b d1, (a0)+
pub fn i_c15580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15582;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15582: rts
pub fn i_c15582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15584;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15584: lea.l $a67(a5), a1
pub fn i_c15584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15588;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA67);
    cpu.a[1] = ad1;
    16
}
/// $C15588: jsr $c15a1a(pc)
pub fn i_c15588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1558C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1558C);
    cpu.pc = 0xC15A1A;
    16
}
/// $C1558C: moveq #$1, d7
pub fn i_c1558c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1558E;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1558E: jsr $c15a4a(pc)
pub fn i_c1558e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15592;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15592);
    cpu.pc = 0xC15A4A;
    16
}
/// $C15592: jsr $c158f8(pc)
pub fn i_c15592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15596;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15596);
    cpu.pc = 0xC158F8;
    16
}
/// $C15596: move.b $a98(a5), d3
pub fn i_c15596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1559A;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1559A: andi.b #$c, d3
pub fn i_c1559a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1559E;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1559E: beq.w $c155c2
pub fn i_c1559e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC155C2; return 10; }
    8
}
/// $C155A2: moveq #$1, d2
pub fn i_c155a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155A4;
    cpu.ir = 0x7401;
    cpu.d[2] = 0x1;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C155A4: andi.b #$8, d3
pub fn i_c155a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155A8;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C155A8: bne.b $c155ac
pub fn i_c155a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155AA;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC155AC; return 10; }
    8
}
/// $C155AA: moveq #$ff, d2
pub fn i_c155aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155AC;
    cpu.ir = 0x74FF;
    cpu.d[2] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C155AC: add.w $a76(a5), d2
pub fn i_c155ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155B0;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C155B0: bpl.b $c155b4
pub fn i_c155b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155B2;
    cpu.ir = 0x6A02;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC155B4; return 10; }
    8
}
/// $C155B2: moveq #$9, d2
pub fn i_c155b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155B4;
    cpu.ir = 0x7409;
    cpu.d[2] = 0x9;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C155B4: cmpi.w #$a, d2
pub fn i_c155b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155B8;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $C155B8: bcs.b $c155bc
pub fn i_c155b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155BA;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC155BC; return 10; }
    8
}
/// $C155BA: moveq #$0, d2
pub fn i_c155ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155BC;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C155BC: move.w d2, $a76(a5)
pub fn i_c155bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155C0;
    cpu.ir = 0x3B42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C155C0: rts
pub fn i_c155c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C155C2: moveq #$3, d3
pub fn i_c155c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155C4;
    cpu.ir = 0x7603;
    cpu.d[3] = 0x3;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C155C4: and.b $a98(a5), d3
pub fn i_c155c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155C8;
    cpu.ir = 0xC62D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C155C8: beq.w $c1562e
pub fn i_c155c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155CC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1562E; return 10; }
    8
}
/// $C155CC: move.w $a76(a5), d2
pub fn i_c155cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155D0;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C155D0: lsl.w #$2, d2
pub fn i_c155d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155D2;
    cpu.ir = 0xE54A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C155D2: move.l $c15600(pc, d2.w), d2
pub fn i_c155d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155D6;
    cpu.ir = 0x243B;
    let ad2 = 0xC15600_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C155D6: andi.b #$1, d3
pub fn i_c155d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155DA;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C155DA: beq.b $c155de
pub fn i_c155da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155DC;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC155DE; return 10; }
    8
}
/// $C155DC: neg.l d2
pub fn i_c155dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155DE;
    cpu.ir = 0x4482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.set(StatusRegister::V, v1 == 0x80000000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = r2;
    4
}
/// $C155DE: cmpi.w #$6, $a76(a5)
pub fn i_c155de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155E4;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Word, false);
    16
}
/// $C155E4: bcc.w $c15628
pub fn i_c155e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC15628; return 10; }
    8
}
/// $C155E8: move.l $a66(a5), d0
pub fn i_c155e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155EC;
    cpu.ir = 0x202D;
    let ad2 = cpu.a[5].wrapping_add(0xA66);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C155EC: add.l d2, d0
pub fn i_c155ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155EE;
    cpu.ir = 0xD082;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $C155EE: andi.l #$ffffff, d0
pub fn i_c155ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155F4;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $C155F4: move.l d0, $a66(a5)
pub fn i_c155f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155F8;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA66);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C155F8: movea.l d0, a1
pub fn i_c155f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155FA;
    cpu.ir = 0x2240;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C155FA: move.w (a1), $a70(a5)
pub fn i_c155fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC155FE;
    cpu.ir = 0x3B51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $C155FE: rts
pub fn i_c155fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15600;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15600: ori.b #$0, (a0)
pub fn i_c15600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15604;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C15604: ori.b #$0, d1
pub fn i_c15604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15608;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15608: ori.b #$0, d0
pub fn i_c15608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1560C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1560C: ori.b #$0, d0
pub fn i_c1560c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15610;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15610: ori.b #$10, d0
pub fn i_c15610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15614;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15614: ori.b #$2, d0
pub fn i_c15614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15618;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15618: ori.b #$0, d0
pub fn i_c15618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1561C;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1561C: ori.b #$0, d0
pub fn i_c1561c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15620;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15620: ori.b #$10, d0
pub fn i_c15620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15624;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15624: ori.b #$1, d0
pub fn i_c15624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15628;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15628: add.w d2, $a70(a5)
pub fn i_c15628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1562C;
    cpu.ir = 0xD56D;
    let ad1 = cpu.a[5].wrapping_add(0xA70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C1562C: rts
pub fn i_c1562c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1562E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1562E: btst.b #$6, $a98(a5)
pub fn i_c1562e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15634;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C15634: beq.b $c1563e
pub fn i_c15634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15636;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1563E; return 10; }
    8
}
/// $C15636: movea.l $a66(a5), a1
pub fn i_c15636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1563A;
    cpu.ir = 0x226D;
    let ad2 = cpu.a[5].wrapping_add(0xA66);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $C1563A: move.w $a70(a5), (a1)
pub fn i_c1563a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1563E;
    cpu.ir = 0x32AD;
    let ad2 = cpu.a[5].wrapping_add(0xA70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v3 as u16);
    16
}
/// $C1563E: rts
pub fn i_c1563e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15640;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15640: lea.l $a6b(a5), a1
pub fn i_c15640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15644;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA6B);
    cpu.a[1] = ad1;
    16
}
/// $C15644: jsr $c15a1a(pc)
pub fn i_c15644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15648;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15648);
    cpu.pc = 0xC15A1A;
    16
}
/// $C15648: moveq #$1, d7
pub fn i_c15648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1564A;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1564A: jsr $c15a4a(pc)
pub fn i_c1564a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1564E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1564E);
    cpu.pc = 0xC15A4A;
    16
}
/// $C1564E: jsr $c158f8(pc)
pub fn i_c1564e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15652;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15652);
    cpu.pc = 0xC158F8;
    16
}
/// $C15652: move.b $a98(a5), d3
pub fn i_c15652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15656;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C15656: andi.b #$c, d3
pub fn i_c15656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1565A;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1565A: beq.w $c15688
pub fn i_c1565a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1565E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15688; return 10; }
    8
}
/// $C1565E: moveq #$0, d2
pub fn i_c1565e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15660;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15660: andi.b #$8, d3
pub fn i_c15660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15664;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15664: bne.b $c15668
pub fn i_c15664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15666;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15668; return 10; }
    8
}
/// $C15666: moveq #$a, d2
pub fn i_c15666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15668;
    cpu.ir = 0x740A;
    cpu.d[2] = 0xA;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15668: add.w $a76(a5), d2
pub fn i_c15668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1566C;
    cpu.ir = 0xD46D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C1566C: move.b $c15674(pc, d2.w), $a77(a5)
pub fn i_c1566c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15672;
    cpu.ir = 0x1B7B;
    let ad2 = 0xC15674_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xA77);
    bus.write8(ad4, v3 as u8);
    22
}
/// $C15672: rts
pub fn i_c15672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15674;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15674: ori.b #$4, d1
pub fn i_c15674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15678;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15678: btst.l d2, d6
pub fn i_c15678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1567A;
    cpu.ir = 0x506;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C1567A: movep.w $902(a0), d3
pub fn i_c1567a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1567E;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x902);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C1567E: ori.b #$2, d1
pub fn i_c1567e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15682;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15682: btst.l d1, d4
pub fn i_c15682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15684;
    cpu.ir = 0x304;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 32);
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C15684: btst.l d2, d6
pub fn i_c15684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15686;
    cpu.ir = 0x506;
    let mk: u32 = 1u32 << ((cpu.d[2]) % 32);
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C15686: movep.w $7603(a0), d3
pub fn i_c15686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1568A;
    cpu.ir = 0x708;
    let a = cpu.a[0].wrapping_add(0x7603);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C15688: moveq #$3, d3
pub fn i_c15688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1568A;
    cpu.ir = 0x7603;
    cpu.d[3] = 0x3;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1568A: and.b $a98(a5), d3
pub fn i_c1568a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1568E;
    cpu.ir = 0xC62D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    let r = (v3 & cpu.d[3]) & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $C1568E: beq.w $c15724
pub fn i_c1568e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15692;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15724; return 10; }
    8
}
/// $C15692: move.w $a76(a5), d2
pub fn i_c15692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15696;
    cpu.ir = 0x342D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C15696: lsl.w #$2, d2
pub fn i_c15696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15698;
    cpu.ir = 0xE54A;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C15698: move.l $c156f6(pc, d2.w), d2
pub fn i_c15698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1569C;
    cpu.ir = 0x243B;
    let ad2 = 0xC156F6_u32.wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C1569C: andi.b #$1, d3
pub fn i_c1569c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156A0;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C156A0: beq.b $c156a4
pub fn i_c156a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156A2;
    cpu.ir = 0x6702;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC156A4; return 10; }
    8
}
/// $C156A2: neg.l d2
pub fn i_c156a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156A4;
    cpu.ir = 0x4482;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.set(StatusRegister::V, v1 == 0x80000000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = r2;
    4
}
/// $C156A4: cmpi.w #$6, $a76(a5)
pub fn i_c156a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156AA;
    cpu.ir = 0xC6D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6, Size::Word, false);
    16
}
/// $C156AA: bcc.w $c1571e
pub fn i_c156aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156AE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC1571E; return 10; }
    8
}
/// $C156AE: move.l $a6a(a5), d0
pub fn i_c156ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156B2;
    cpu.ir = 0x202D;
    let ad2 = cpu.a[5].wrapping_add(0xA6A);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C156B2: add.l d2, d0
pub fn i_c156b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156B4;
    cpu.ir = 0xD082;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $C156B4: bmi.b $c156d4
pub fn i_c156b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156B6;
    cpu.ir = 0x6B1E;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0xC156D4; return 10; }
    8
}
/// $C156B6: cmpi.l #$8580, d0
pub fn i_c156b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156BC;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x8580, Size::Long, false);
    8
}
/// $C156BC: bcs.b $c156c2
pub fn i_c156bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156BE;
    cpu.ir = 0x6504;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC156C2; return 10; }
    8
}
/// $C156BE: moveq #$0, d0
pub fn i_c156be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156C0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C156C0: bra.b $c156da
pub fn i_c156c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156C2;
    cpu.ir = 0x6018;
    cpu.pc = 0xC156DA;
    10
}
/// $C156C2: cmpi.l #$8000, d0
pub fn i_c156c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156C8;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x8000, Size::Long, false);
    8
}
/// $C156C8: bcc.b $c156da
pub fn i_c156c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156CA;
    cpu.ir = 0x6410;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC156DA; return 10; }
    8
}
/// $C156CA: cmpi.l #$7400, d0
pub fn i_c156ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156D0;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x7400, Size::Long, false);
    8
}
/// $C156D0: bcc.b $c156b2
pub fn i_c156d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156D2;
    cpu.ir = 0x64E0;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC156B2; return 10; }
    8
}
/// $C156D2: bra.b $c156da
pub fn i_c156d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156D4;
    cpu.ir = 0x6006;
    cpu.pc = 0xC156DA;
    10
}
/// $C156D4: move.l #$857f, d0
pub fn i_c156d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156DA;
    cpu.ir = 0x203C;
    let v1: u32 = 0x857F;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C156DA: move.l d0, $a6a(a5)
pub fn i_c156da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156DE;
    cpu.ir = 0x2B40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA6A);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C156DE: move.w d0, $3c0000.l
pub fn i_c156de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156E4;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $C156E4: nop
pub fn i_c156e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156E6;
    cpu.ir = 0x4E71;
    4
}
/// $C156E6: nop
pub fn i_c156e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156E8;
    cpu.ir = 0x4E71;
    4
}
/// $C156E8: nop
pub fn i_c156e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156EA;
    cpu.ir = 0x4E71;
    4
}
/// $C156EA: nop
pub fn i_c156ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156EC;
    cpu.ir = 0x4E71;
    4
}
/// $C156EC: move.w $3c0002.l, $a70(a5)
pub fn i_c156ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156F4;
    cpu.ir = 0x3B79;
    let v2 = bus.read16(0x3C0002) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[5].wrapping_add(0xA70);
    bus.write16(ad3, v2 as u16);
    24
}
/// $C156F4: rts
pub fn i_c156f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C156F6: ori.b #$0, (a0)
pub fn i_c156f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156FA;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C156FA: ori.b #$0, d1
pub fn i_c156fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC156FE;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C156FE: ori.b #$0, d0
pub fn i_c156fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15702;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15702: ori.b #$0, d0
pub fn i_c15702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15706;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15706: ori.b #$10, d0
pub fn i_c15706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1570A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1570A: ori.b #$1, d0
pub fn i_c1570a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1570E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1570E: ori.b #$0, d0
pub fn i_c1570e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15712;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15712: ori.b #$0, d0
pub fn i_c15712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15716;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15716: ori.b #$10, d0
pub fn i_c15716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1571A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1571A: ori.b #$1, d0
pub fn i_c1571a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1571E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1571E: add.w d2, $a70(a5)
pub fn i_c1571e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15722;
    cpu.ir = 0xD56D;
    let ad1 = cpu.a[5].wrapping_add(0xA70);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C15722: rts
pub fn i_c15722(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15724;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15724: btst.b #$6, $a98(a5)
pub fn i_c15724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1572A;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C1572A: beq.b $c1573c
pub fn i_c1572a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1572C;
    cpu.ir = 0x6710;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1573C; return 10; }
    8
}
/// $C1572C: move.w $a6c(a5), $3c0000.l
pub fn i_c1572c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15734;
    cpu.ir = 0x33ED;
    let ad2 = cpu.a[5].wrapping_add(0xA6C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v3 as u16);
    24
}
/// $C15734: move.w $a70(a5), $3c0002.l
pub fn i_c15734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1573C;
    cpu.ir = 0x33ED;
    let ad2 = cpu.a[5].wrapping_add(0xA70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v3 as u16);
    24
}
/// $C1573C: rts
pub fn i_c1573c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1573E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1573E: movea.l $abe(a5), a0
pub fn i_c1573e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15742;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C15742: clr.l (a0)+
pub fn i_c15742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15744;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15744: move.l #$1f0ff, (a0)+
pub fn i_c15744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1574A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1574A: move.l #$370ce, (a0)+
pub fn i_c1574a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15750;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x370CE;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15750: move.w #$7, (a0)+
pub fn i_c15750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15754;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15754: move.w $a6e(a5), d0
pub fn i_c15754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15758;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA6E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C15758: btst.b #$4, $a79(a5)
pub fn i_c15758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1575E;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C1575E: beq.b $c15768
pub fn i_c1575e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15760;
    cpu.ir = 0x6708;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15768; return 10; }
    8
}
/// $C15760: tst.w $a76(a5)
pub fn i_c15760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15764;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C15764: bne.b $c15768
pub fn i_c15764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15766;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15768; return 10; }
    8
}
/// $C15766: moveq #$13, d0
pub fn i_c15766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15768;
    cpu.ir = 0x7013;
    cpu.d[0] = 0x13;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15768: lsl.w #$2, d0
pub fn i_c15768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1576A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C1576A: lea.l $c15864(pc), a2
pub fn i_c1576a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1576E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0xC15864;
    16
}
/// $C1576E: nop
pub fn i_c1576e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15770;
    cpu.ir = 0x4E71;
    4
}
/// $C15770: move.l (a2, d0.w), (a0)+
pub fn i_c15770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15774;
    cpu.ir = 0x20F2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C15774: clr.w (a0)+
pub fn i_c15774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15776;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15776: move.l a0, $abe(a5)
pub fn i_c15776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1577A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C1577A: moveq #$3, d7
pub fn i_c1577a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1577C;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1577C: jsr $c15a4a(pc)
pub fn i_c1577c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15780;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15780);
    cpu.pc = 0xC15A4A;
    16
}
/// $C15780: tst.w $a76(a5)
pub fn i_c15780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15784;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $C15784: beq.b $c1578a
pub fn i_c15784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15786;
    cpu.ir = 0x6704;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1578A; return 10; }
    8
}
/// $C15786: jsr $c158f8(pc)
pub fn i_c15786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1578A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1578A);
    cpu.pc = 0xC158F8;
    16
}
/// $C1578A: move.b $a98(a5), d3
pub fn i_c1578a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1578E;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C1578E: andi.b #$c, d3
pub fn i_c1578e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15792;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15792: beq.w $c157c8
pub fn i_c15792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15796;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC157C8; return 10; }
    8
}
/// $C15796: moveq #$0, d0
pub fn i_c15796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15798;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15798: andi.b #$8, d3
pub fn i_c15798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1579C;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1579C: bne.b $c157a0
pub fn i_c1579c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1579E;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC157A0; return 10; }
    8
}
/// $C1579E: moveq #$e, d0
pub fn i_c1579e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157A0;
    cpu.ir = 0x700E;
    cpu.d[0] = 0xE;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C157A0: add.w $a76(a5), d0
pub fn i_c157a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157A4;
    cpu.ir = 0xD06D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C157A4: move.b $c157ac(pc, d0.w), $a77(a5)
pub fn i_c157a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157AA;
    cpu.ir = 0x1B7B;
    let ad2 = 0xC157AC_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0xA77);
    bus.write8(ad4, v3 as u8);
    22
}
/// $C157AA: rts
pub fn i_c157aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C157AC: addi.b #$0, d0
pub fn i_c157ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157B0;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x0, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C157B0: ori.b #$8, d0
pub fn i_c157b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157B4;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C157B4: movep.w $b0c(a2), d4
pub fn i_c157b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157B8;
    cpu.ir = 0x90A;
    let a = cpu.a[2].wrapping_add(0xB0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C157B8: btst.l d6, d0
pub fn i_c157b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157BA;
    cpu.ir = 0xD00;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C157BA: btst.l d6, d0
pub fn i_c157ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157BC;
    cpu.ir = 0xD00;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C157BC: ori.b #$0, d0
pub fn i_c157bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157C0;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C157C0: ori.b #$8, d6
pub fn i_c157c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157C4;
    cpu.ir = 0x6;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C157C4: movep.w $b0c(a2), d4
pub fn i_c157c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157C8;
    cpu.ir = 0x90A;
    let a = cpu.a[2].wrapping_add(0xB0C);
    let b1 = bus.read8(a) as u32; let b0 = bus.read8(a.wrapping_add(2)) as u32;
    cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | (b1 << 8) | b0;
    16
}
/// $C157C8: move.b $a98(a5), d3
pub fn i_c157c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157CC;
    cpu.ir = 0x162D;
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $C157CC: andi.b #$3, d3
pub fn i_c157cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157D0;
    cpu.ir = 0x203;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C157D0: beq.w $c1583e
pub fn i_c157d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC1583E; return 10; }
    8
}
/// $C157D4: move.w $a76(a5), d0
pub fn i_c157d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157D8;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C157D8: bne.w $c15828
pub fn i_c157d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15828; return 10; }
    8
}
/// $C157DC: moveq #$1, d1
pub fn i_c157dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157DE;
    cpu.ir = 0x7201;
    cpu.d[1] = 0x1;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C157DE: btst.b #$0, d3
pub fn i_c157de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157E2;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C157E2: bne.b $c157e6
pub fn i_c157e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157E4;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC157E6; return 10; }
    8
}
/// $C157E4: moveq #$ff, d1
pub fn i_c157e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157E6;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C157E6: add.w $a6e(a5), d1
pub fn i_c157e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157EA;
    cpu.ir = 0xD26D;
    let ad2 = cpu.a[5].wrapping_add(0xA6E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $C157EA: bpl.b $c157ee
pub fn i_c157ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157EC;
    cpu.ir = 0x6A02;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0xC157EE; return 10; }
    8
}
/// $C157EC: moveq #$12, d1
pub fn i_c157ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157EE;
    cpu.ir = 0x7212;
    cpu.d[1] = 0x12;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C157EE: cmpi.w #$13, d1
pub fn i_c157ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157F2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x13, Size::Word, false);
    8
}
/// $C157F2: bcs.b $c157f6
pub fn i_c157f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157F4;
    cpu.ir = 0x6502;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0xC157F6; return 10; }
    8
}
/// $C157F4: moveq #$0, d1
pub fn i_c157f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157F6;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C157F6: move.w d1, $a6e(a5)
pub fn i_c157f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157FA;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA6E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C157FA: lsl.w #$2, d1
pub fn i_c157fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC157FC;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C157FC: lea.l $a00(a5), a4
pub fn i_c157fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15800;
    cpu.ir = 0x49ED;
    let ad1 = cpu.a[5].wrapping_add(0xA00);
    cpu.a[4] = ad1;
    16
}
/// $C15800: move.l (a4, d1.w), $a70(a5)
pub fn i_c15800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15806;
    cpu.ir = 0x2B74;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[5].wrapping_add(0xA70);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $C15806: rts
pub fn i_c15806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15808;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15808: move.b d0, d0
pub fn i_c15808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1580A;
    cpu.ir = 0x1000;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1580A: ori.b #$0, d0
pub fn i_c1580a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1580E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1580E: ori.b #$10, d0
pub fn i_c1580e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15812;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15812: ori.b #$1, d0
pub fn i_c15812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15816;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15816: ori.b #$0, d0
pub fn i_c15816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1581A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C1581A: move.b d0, d0
pub fn i_c1581a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1581C;
    cpu.ir = 0x1000;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $C1581C: ori.b #$0, d0
pub fn i_c1581c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15820;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15820: ori.b #$10, d0
pub fn i_c15820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15824;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15824: ori.b #$1, d0
pub fn i_c15824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15828;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15828: subq.w #$6, d0
pub fn i_c15828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1582A;
    cpu.ir = 0x5D40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C1582A: lsl.w #$2, d0
pub fn i_c1582a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1582C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C1582C: move.l $c15808(pc, d0.w), d1
pub fn i_c1582c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15830;
    cpu.ir = 0x223B;
    let ad2 = 0xC15808_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[1] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C15830: btst.b #$0, d3
pub fn i_c15830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15834;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $C15834: bne.b $c15838
pub fn i_c15834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15836;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15838; return 10; }
    8
}
/// $C15836: neg.l d1
pub fn i_c15836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15838;
    cpu.ir = 0x4481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.set(StatusRegister::V, v1 == 0x80000000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = r2;
    4
}
/// $C15838: add.l d1, $a70(a5)
pub fn i_c15838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1583C;
    cpu.ir = 0xD3AD;
    let ad1 = cpu.a[5].wrapping_add(0xA70);
    let v2 = bus.read32(ad1);
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $C1583C: rts
pub fn i_c1583c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1583E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1583E: btst.b #$6, $a98(a5)
pub fn i_c1583e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15844;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C15844: beq.b $c15862
pub fn i_c15844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15846;
    cpu.ir = 0x671C;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15862; return 10; }
    8
}
/// $C15846: jsr $c17290(pc)
pub fn i_c15846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1584A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1584A);
    cpu.pc = 0xC17290;
    16
}
/// $C1584A: move.w $a6e(a5), d0
pub fn i_c1584a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1584E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA6E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C1584E: lsl.w #$2, d0
pub fn i_c1584e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15850;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C15850: lea.l $a00(a5), a4
pub fn i_c15850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15854;
    cpu.ir = 0x49ED;
    let ad1 = cpu.a[5].wrapping_add(0xA00);
    cpu.a[4] = ad1;
    16
}
/// $C15854: move.l $a70(a5), (a4, d0.w)
pub fn i_c15854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1585A;
    cpu.ir = 0x29AD;
    let ad2 = cpu.a[5].wrapping_add(0xA70);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $C1585A: movea.l $abe(a5), a0
pub fn i_c1585a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1585E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C1585E: jsr $c15946(pc)
pub fn i_c1585e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15862;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15862);
    cpu.pc = 0xC15946;
    16
}
/// $C15862: rts
pub fn i_c15862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15864;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15864: neg.b -$1(a0, d2.w)
pub fn i_c15864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15868;
    cpu.ir = 0x4430;
    let ad1 = cpu.a[0].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C15868: neg.b -$1(a1, d2.w)
pub fn i_c15868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1586C;
    cpu.ir = 0x4431;
    let ad1 = cpu.a[1].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C1586C: neg.b -$1(a2, d2.w)
pub fn i_c1586c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15870;
    cpu.ir = 0x4432;
    let ad1 = cpu.a[2].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C15870: neg.b -$1(a3, d2.w)
pub fn i_c15870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15874;
    cpu.ir = 0x4433;
    let ad1 = cpu.a[3].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C15874: neg.b -$1(a4, d2.w)
pub fn i_c15874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15878;
    cpu.ir = 0x4434;
    let ad1 = cpu.a[4].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C15878: neg.b -$1(a5, d2.w)
pub fn i_c15878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1587C;
    cpu.ir = 0x4435;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C1587C: neg.b -$1(a6, d2.w)
pub fn i_c1587c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15880;
    cpu.ir = 0x4436;
    let ad1 = cpu.a[6].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C15880: neg.b -$1(a7, d2.w)
pub fn i_c15880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15884;
    cpu.ir = 0x4437;
    let ad1 = cpu.a[7].wrapping_add(0xFFFFFFFF).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.set(StatusRegister::V, v2 == 0x80); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C15884: dc.w $4130
pub fn i_c15884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15886;
    cpu.ir = 0x4130;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15886: ?
pub fn i_c15886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15888;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15888: dc.w $4131
pub fn i_c15888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1588A;
    cpu.ir = 0x4131;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1588A: ?
pub fn i_c1588a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1588C;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1588C: dc.w $4132
pub fn i_c1588c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1588E;
    cpu.ir = 0x4132;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1588E: ?
pub fn i_c1588e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15890;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15890: dc.w $4133
pub fn i_c15890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15892;
    cpu.ir = 0x4133;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15892: ?
pub fn i_c15892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15894;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15894: dc.w $4134
pub fn i_c15894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15896;
    cpu.ir = 0x4134;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15896: ?
pub fn i_c15896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15898;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15898: dc.w $4135
pub fn i_c15898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1589A;
    cpu.ir = 0x4135;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1589A: ?
pub fn i_c1589a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1589C;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1589C: dc.w $4136
pub fn i_c1589c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1589E;
    cpu.ir = 0x4136;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C1589E: ?
pub fn i_c1589e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158A0;
    cpu.ir = 0x20FF;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C158A0: subq.w #$1, (a3)
pub fn i_c158a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158A2;
    cpu.ir = 0x5353;
    let ad1 = cpu.a[3];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C158B4: btst.b #$6, $a98(a5)
pub fn i_c158b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158BA;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C158BA: bne.b $c158be
pub fn i_c158ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158BC;
    cpu.ir = 0x6602;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC158BE; return 10; }
    8
}
/// $C158BC: rts
pub fn i_c158bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C158BE: bset.b #$7, $a48(a5)
pub fn i_c158be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158C4;
    cpu.ir = 0x8ED;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[5].wrapping_add(0xA48);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $C158C4: jsr $c16ed0.l
pub fn i_c158c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0xC158CA);
    cpu.pc = 0xC16ED0;
    16
}
/// $C158CA: move.l $a4e(a5), $4001e2.l
pub fn i_c158ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158D2;
    cpu.ir = 0x23ED;
    let ad2 = cpu.a[5].wrapping_add(0xA4E);
    let v3 = bus.read32(ad2);
    bus.write32(0x4001E2, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    32
}
/// $C158D2: movea.l $a40(a5), a3
pub fn i_c158d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158D6;
    cpu.ir = 0x266D;
    let ad2 = cpu.a[5].wrapping_add(0xA40);
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    16
}
/// $C158D6: move a3, usp
pub fn i_c158d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158D8;
    cpu.ir = 0x4E63;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    cpu.usp = cpu.a[3];
    4
}
/// $C158D8: movea.l $a3c(a5), a7
pub fn i_c158d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158DC;
    cpu.ir = 0x2E6D;
    let ad2 = cpu.a[5].wrapping_add(0xA3C);
    let v3 = bus.read32(ad2);
    cpu.a[7] = v3;
    16
}
/// $C158DC: move.l $a44(a5), -(a7)
pub fn i_c158dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158E0;
    cpu.ir = 0x2F2D;
    let ad2 = cpu.a[5].wrapping_add(0xA44);
    let v3 = bus.read32(ad2);
    let o4 = cpu.a[7];
    bus.write16(o4.wrapping_sub(2), v3 as u16);
    cpu.a[7] = o4.wrapping_sub(4);
    bus.write16(o4.wrapping_sub(4), (v3 >> 16) as u16);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C158E0: move.w $a48(a5), -(a7)
pub fn i_c158e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158E4;
    cpu.ir = 0x3F2D;
    let ad2 = cpu.a[5].wrapping_add(0xA48);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $C158E4: movem.l $10fd00.l, d0-d7/a0-a6
pub fn i_c158e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158EC;
    cpu.ir = 0x4CF9;
    let mut a = 0x10FD00;
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[7] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    let _ = a;
    20
}
/// $C158EC: rte
pub fn i_c158ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158EE;
    cpu.ir = 0x4E73;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let s = cpu.pop16(bus); let r = cpu.pop32(bus); write_sr(cpu, s); cpu.jump_to(r);
    20
}
/// $C158EE: btst.b #$6, $a98(a5)
pub fn i_c158ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158F4;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C158F4: bne.b $c158c4
pub fn i_c158f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158F6;
    cpu.ir = 0x66CE;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC158C4; return 10; }
    8
}
/// $C158F6: rts
pub fn i_c158f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C158F8: btst.b #$4, $a79(a5)
pub fn i_c158f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC158FE;
    cpu.ir = 0x82D;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[5].wrapping_add(0xA79);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $C158FE: beq.b $c15928
pub fn i_c158fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15900;
    cpu.ir = 0x6728;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15928; return 10; }
    8
}
/// $C15900: movea.l $abe(a5), a0
pub fn i_c15900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15904;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C15904: clr.l (a0)+
pub fn i_c15904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15906;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15906: move.l #$1f0ff, (a0)+
pub fn i_c15906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1590C;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1590C: move.w #$3, (a0)+
pub fn i_c1590c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15910;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15910: move.w $a76(a5), d0
pub fn i_c15910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15914;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C15914: add.w d0, d0
pub fn i_c15914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15916;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C15916: move.w $c1592a(pc, d0.w), (a0)+
pub fn i_c15916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1591A;
    cpu.ir = 0x30FB;
    let ad2 = 0xC1592A_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    18
}
/// $C1591A: move.w #$7, (a0)+
pub fn i_c1591a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1591E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1591E: move.l #$20ff0000, (a0)+
pub fn i_c1591e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15924;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x20FF0000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15924: move.l a0, $abe(a5)
pub fn i_c15924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15928;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15928: rts
pub fn i_c15928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1592A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C1592A: moveq #$8e, d0
pub fn i_c1592a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1592C;
    cpu.ir = 0x708E;
    cpu.d[0] = 0xFFFFFF8E;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1592C: moveq #$ae, d0
pub fn i_c1592c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1592E;
    cpu.ir = 0x70AE;
    cpu.d[0] = 0xFFFFFFAE;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C1592E: moveq #$ce, d0
pub fn i_c1592e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15930;
    cpu.ir = 0x70CE;
    cpu.d[0] = 0xFFFFFFCE;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15930: moveq #$ee, d0
pub fn i_c15930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15932;
    cpu.ir = 0x70EE;
    cpu.d[0] = 0xFFFFFFEE;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15946: clr.l (a0)+
pub fn i_c15946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15948;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15948: move.l #$1f0ff, (a0)+
pub fn i_c15948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1594E;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C1594E: move.w #$2002, (a0)+
pub fn i_c1594e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15952;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15952: move.l #$37067, (a0)+
pub fn i_c15952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15958;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x37067;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15958: move.w #$7, (a0)+
pub fn i_c15958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1595C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1595C: lea.l $10fd00.l, a1
pub fn i_c1595c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15962;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10FD00;
    20
}
/// $C15962: moveq #$3, d7
pub fn i_c15962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15964;
    cpu.ir = 0x7E03;
    cpu.d[7] = 0x3;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15964: jsr $c15a7c(pc)
pub fn i_c15964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15968;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15968);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15968: jsr $c15a7c(pc)
pub fn i_c15968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1596C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1596C);
    cpu.pc = 0xC15A7C;
    16
}
/// $C1596C: jsr $c15a7c(pc)
pub fn i_c1596c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15970;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15970);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15970: jsr $c15a7c(pc)
pub fn i_c15970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15974;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15974);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15974: move.b #$20, (a0)+
pub fn i_c15974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15978;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15978: dbra d7, $c15964
pub fn i_c15978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1597C;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC1597C; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC1597C; return 14; }
    cpu.pc = 0xC15964;
    10
}
/// $C1597C: move.b #$ff, -$1(a0)
pub fn i_c1597c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15982;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C15982: clr.w (a0)+
pub fn i_c15982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15984;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15984: move.l a0, $abe(a5)
pub fn i_c15984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15988;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15988: jsr $c17290(pc)
pub fn i_c15988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1598C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC1598C);
    cpu.pc = 0xC17290;
    16
}
/// $C1598C: lea.l $c00(a5), a0
pub fn i_c1598c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15990;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C15990: clr.l (a0)+
pub fn i_c15990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15992;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15992: move.l #$1f0ff, (a0)+
pub fn i_c15992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15998;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15998: move.w #$2002, (a0)+
pub fn i_c15998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC1599C;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C1599C: move.l #$37068, (a0)+
pub fn i_c1599c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159A2;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x37068;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C159A2: lea.l $a10(a5), a1
pub fn i_c159a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159A6;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA10);
    cpu.a[1] = ad1;
    16
}
/// $C159A6: moveq #$2, d7
pub fn i_c159a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159A8;
    cpu.ir = 0x7E02;
    cpu.d[7] = 0x2;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C159A8: move.w #$7, (a0)+
pub fn i_c159a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159AC;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C159AC: moveq #$3, d6
pub fn i_c159ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159AE;
    cpu.ir = 0x7C03;
    cpu.d[6] = 0x3;
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C159AE: jsr $c15a7c(pc)
pub fn i_c159ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159B2);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159B2: jsr $c15a7c(pc)
pub fn i_c159b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159B6);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159B6: jsr $c15a7c(pc)
pub fn i_c159b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159BA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159BA);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159BA: jsr $c15a7c(pc)
pub fn i_c159ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159BE);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159BE: move.b #$20, (a0)+
pub fn i_c159be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159C2;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C159C2: dbra d6, $c159ae
pub fn i_c159c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159C6;
    cpu.ir = 0x51CE;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC159C6; return 12; }
    let n = (cpu.d[6] as u16).wrapping_sub(1);
    cpu.d[6] = (cpu.d[6] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC159C6; return 14; }
    cpu.pc = 0xC159AE;
    10
}
/// $C159C6: move.b #$ff, -$1(a0)
pub fn i_c159c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159CC;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFF);
    bus.write8(ad2, v1 as u8);
    12
}
/// $C159CC: move.l #$50001, (a0)+
pub fn i_c159cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159D2;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x50001;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C159D2: dbra d7, $c159a8
pub fn i_c159d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159D6;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC159D6; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC159D6; return 14; }
    cpu.pc = 0xC159A8;
    10
}
/// $C159D6: move.w #$7, (a0)+
pub fn i_c159d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159DA;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C159DA: jsr $c15a7c(pc)
pub fn i_c159da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159DE);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159DE: jsr $c15a7c(pc)
pub fn i_c159de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159E2);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159E2: jsr $c15a7c(pc)
pub fn i_c159e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159E6);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159E6: jsr $c15a7c(pc)
pub fn i_c159e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159EA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159EA);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159EA: move.b #$20, (a0)+
pub fn i_c159ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159EE;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C159EE: jsr $c15a7c(pc)
pub fn i_c159ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159F2);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159F2: jsr $c15a7c(pc)
pub fn i_c159f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159F6);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159F6: jsr $c15a7c(pc)
pub fn i_c159f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159FA);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159FA: jsr $c15a7c(pc)
pub fn i_c159fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC159FE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC159FE);
    cpu.pc = 0xC15A7C;
    16
}
/// $C159FE: move.b #$20, (a0)+
pub fn i_c159fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A02;
    cpu.ir = 0x10FC;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15A02: jsr $c15a7c(pc)
pub fn i_c15a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15A06);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15A06: jsr $c15a7c(pc)
pub fn i_c15a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15A0A);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15A0A: move.w #$ff00, (a0)+
pub fn i_c15a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A0E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0xFF00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15A0E: clr.w (a0)+
pub fn i_c15a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A10;
    cpu.ir = 0x4258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15A10: move.l a0, $abe(a5)
pub fn i_c15a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A14;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15A14: jsr $c17290(pc)
pub fn i_c15a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15A18);
    cpu.pc = 0xC17290;
    16
}
/// $C15A18: rts
pub fn i_c15a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15A1A: movea.l $abe(a5), a0
pub fn i_c15a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A1E;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C15A1E: clr.l (a0)+
pub fn i_c15a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A20;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15A20: move.l #$1f0ff, (a0)+
pub fn i_c15a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A26;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15A26: move.w #$2002, (a0)+
pub fn i_c15a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A2A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15A2A: move.l #$3708e, (a0)+
pub fn i_c15a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A30;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x3708E;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15A30: move.w #$7, (a0)+
pub fn i_c15a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A34;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15A34: moveq #$2, d2
pub fn i_c15a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A36;
    cpu.ir = 0x7402;
    cpu.d[2] = 0x2;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15A36: jsr $c15a7c(pc)
pub fn i_c15a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15A3A);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15A3A: dbra d2, $c15a36
pub fn i_c15a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A3E;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15A3E; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15A3E; return 14; }
    cpu.pc = 0xC15A36;
    10
}
/// $C15A3E: move.l #$ff000000, (a0)+
pub fn i_c15a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A44;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFF000000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15A44: move.l a0, $abe(a5)
pub fn i_c15a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A48;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15A48: rts
pub fn i_c15a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15A4A: lea.l $a70(a5), a1
pub fn i_c15a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A4E;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA70);
    cpu.a[1] = ad1;
    16
}
/// $C15A4E: movea.l $abe(a5), a0
pub fn i_c15a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A52;
    cpu.ir = 0x206D;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $C15A52: clr.l (a0)+
pub fn i_c15a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A54;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15A54: move.l #$1f0ff, (a0)+
pub fn i_c15a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A5A;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15A5A: move.w #$2002, (a0)+
pub fn i_c15a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A5E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15A5E: move.l #$371ce, (a0)+
pub fn i_c15a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A64;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x371CE;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15A64: move.w #$7, (a0)+
pub fn i_c15a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A68;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15A68: jsr $c15a7c(pc)
pub fn i_c15a68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15A6C);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15A6C: dbra d7, $c15a68
pub fn i_c15a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A70;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15A70; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15A70; return 14; }
    cpu.pc = 0xC15A68;
    10
}
/// $C15A70: move.l #$ff000000, (a0)+
pub fn i_c15a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A76;
    cpu.ir = 0x20FC;
    let v1: u32 = 0xFF000000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15A76: move.l a0, $abe(a5)
pub fn i_c15a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A7A;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15A7A: rts
pub fn i_c15a7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15A7C: move.b (a1)+, d0
pub fn i_c15a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A7E;
    cpu.ir = 0x1019;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    8
}
/// $C15A7E: jsr $c16f54(pc)
pub fn i_c15a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A82;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15A82);
    cpu.pc = 0xC16F54;
    16
}
/// $C15A82: move.b d0, (a0)+
pub fn i_c15a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A84;
    cpu.ir = 0x10C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15A84: move.b d1, (a0)+
pub fn i_c15a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A86;
    cpu.ir = 0x10C1;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    bus.write8(ad2, v1 as u8);
    8
}
/// $C15A86: rts
pub fn i_c15a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15A88: ?
pub fn i_c15a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A8A;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15A8A: addq.w #$6, d0
pub fn i_c15a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A8C;
    cpu.ir = 0x5C40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15A8C: ?
pub fn i_c15a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A8E;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15A8E: addq.w #$6, $c1(a0)
pub fn i_c15a8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A92;
    cpu.ir = 0x5C68;
    let ad1 = cpu.a[0].wrapping_add(0xC1);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 6, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $C15A92: addq.l #$6, (a2)
pub fn i_c15a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A94;
    cpu.ir = 0x5C92;
    let ad1 = cpu.a[2];
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 6, Size::Long, true);
    bus.write32(ad1, r3);
    12
}
/// $C15A94: ?
pub fn i_c15a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A96;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15A96: sge.b d0
pub fn i_c15a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A98;
    cpu.ir = 0x5CC0;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ge) { 0xFF } else { 0 };
    let v2 = cpu.d[0] & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (c1 & 0xFF);
    4
}
/// $C15A98: ?
pub fn i_c15a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A9A;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15A9A: sge.b $c1(a6)
pub fn i_c15a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15A9E;
    cpu.ir = 0x5CEE;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ge) { 0xFF } else { 0 };
    let ad2 = cpu.a[6].wrapping_add(0xC1);
    let v3 = bus.read8(ad2) as u32;
    bus.write8(ad2, c1 as u8);
    12
}
/// $C15A9E: subq.b #$6, (a6)
pub fn i_c15a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AA0;
    cpu.ir = 0x5D16;
    let ad1 = cpu.a[6];
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 6, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    8
}
/// $C15AA0: ?
pub fn i_c15aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AA2;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15AA2: subq.b #$6, -$3f(a4, d0.w)
pub fn i_c15aa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AA6;
    cpu.ir = 0x5D34;
    let ad1 = cpu.a[4].wrapping_add(0xFFFFFFC1).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 6, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    14
}
/// $C15AA6: subq.w #$6, (a4)
pub fn i_c15aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AA8;
    cpu.ir = 0x5D54;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 6, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15AA8: ?
pub fn i_c15aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AAA;
    cpu.ir = 0xC1;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15AAA: subq.w #$6, ([$10f300])
pub fn i_c15aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AAE;
    cpu.ir = 0x5D74;
    let ad1 = cpu.a[4].wrapping_add(0xFFFFFFF9).wrapping_add(cpu.d[4]);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 6, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    14
}
/// $C15AAC: lea.l $10f300.l, a5
pub fn i_c15aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AB2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x10F300;
    20
}
/// $C15AAE: ori.b #$0, (a0)
pub fn i_c15aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AB2;
    cpu.ir = 0x10;
    let ad1 = cpu.a[0];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C15AB2: move.w #$7, $3c000c.l
pub fn i_c15ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ABA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C15ABA: tst.b $c00400.l
pub fn i_c15aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AC0;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0xC00400) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $C15AC0: beq.w $c15b9c
pub fn i_c15ac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0xC15B9C; return 10; }
    8
}
/// $C15AC4: move.l a0, $a00(a5)
pub fn i_c15ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AC8;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xA00);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15AC8: move.w d0, $a04(a5)
pub fn i_c15ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ACC;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA04);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C15ACC: move.w d1, $a06(a5)
pub fn i_c15acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AD0;
    cpu.ir = 0x3B41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA06);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C15AD0: move.w d6, $a08(a5)
pub fn i_c15ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AD4;
    cpu.ir = 0x3B46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    bus.write16(ad2, v1 as u16);
    12
}
/// $C15AD4: move.w #$7000, $3c0000.l
pub fn i_c15ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15ADC;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $C15ADC: move.b d0, $300001.l
pub fn i_c15adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AE2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15AE2: move.w #$1, $3c0004.l
pub fn i_c15ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AEA;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0004, v1 as u16);
    16
}
/// $C15AEA: move.w #$f020, d0
pub fn i_c15aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AEE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15AEE: move.w #$4ff, d7
pub fn i_c15aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AF2;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x4FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15AF2: move.w d0, $3c0002.l
pub fn i_c15af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AF8;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $C15AF8: dbra d7, $c15af2
pub fn i_c15af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15AFC;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15AFC; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15AFC; return 14; }
    cpu.pc = 0xC15AF2;
    10
}
/// $C15AFC: move.l #$eee0000, $4001e2.l
pub fn i_c15afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B06;
    cpu.ir = 0x23FC;
    let v1: u32 = 0xEEE0000;
    bus.write32(0x4001E2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $C15B06: lea.l $c00(a5), a0
pub fn i_c15b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B0A;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0xC00);
    cpu.a[0] = ad1;
    16
}
/// $C15B0A: move.w $a08(a5), d0
pub fn i_c15b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B0E;
    cpu.ir = 0x302D;
    let ad2 = cpu.a[5].wrapping_add(0xA08);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $C15B0E: lsl.w #$2, d0
pub fn i_c15b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B10;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $C15B10: lea.l $c15a88(pc), a1
pub fn i_c15b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0xC15A88;
    16
}
/// $C15B14: move.l (a1, d0.w), (a0)+
pub fn i_c15b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B18;
    cpu.ir = 0x20F1;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[0]; cpu.a[0] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    26
}
/// $C15B18: cmpi.w #$14, d0
pub fn i_c15b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B1C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x14, Size::Word, false);
    8
}
/// $C15B1C: bcc.w $c15b68
pub fn i_c15b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0xC15B68; return 10; }
    8
}
/// $C15B20: lea.l $a00(a5), a1
pub fn i_c15b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B24;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0xA00);
    cpu.a[1] = ad1;
    16
}
/// $C15B24: clr.l (a0)+
pub fn i_c15b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B26;
    cpu.ir = 0x4298;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $C15B26: move.l #$1f0ff, (a0)+
pub fn i_c15b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B2C;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x1F0FF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15B2C: move.w #$2002, (a0)+
pub fn i_c15b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B30;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2002;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15B30: move.l #$3708d, (a0)+
pub fn i_c15b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B36;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x3708D;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15B36: move.w #$7, (a0)+
pub fn i_c15b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B3A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15B3A: jsr $c15a7c(pc)
pub fn i_c15b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B3E);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B3E: jsr $c15a7c(pc)
pub fn i_c15b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B42);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B42: jsr $c15a7c(pc)
pub fn i_c15b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B46;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B46);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B46: jsr $c15a7c(pc)
pub fn i_c15b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B4A);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B4A: move.w #$2020, (a0)+
pub fn i_c15b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B4E;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15B4E: jsr $c15a7c(pc)
pub fn i_c15b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B52);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B52: jsr $c15a7c(pc)
pub fn i_c15b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B56);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B56: move.w #$2020, (a0)+
pub fn i_c15b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B5A;
    cpu.ir = 0x30FC;
    let v1: u32 = 0x2020;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $C15B5A: jsr $c15a7c(pc)
pub fn i_c15b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B5E);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B5E: jsr $c15a7c(pc)
pub fn i_c15b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B62);
    cpu.pc = 0xC15A7C;
    16
}
/// $C15B62: move.l #$20ff0000, (a0)+
pub fn i_c15b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B68;
    cpu.ir = 0x20FC;
    let v1: u32 = 0x20FF0000;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15B68: move.l a0, $abe(a5)
pub fn i_c15b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B6C;
    cpu.ir = 0x2B48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[5].wrapping_add(0xABE);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $C15B6C: clr.b $ac2(a5)
pub fn i_c15b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B70;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xAC2);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $C15B70: jsr $c17290(pc)
pub fn i_c15b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0xC15B74);
    cpu.pc = 0xC17290;
    16
}
/// $C15B74: move.w #$7, $3c000c.l
pub fn i_c15b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B7C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x7;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C000C, v1 as u16);
    16
}
/// $C15B7C: move.b d0, $300001.l
pub fn i_c15b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B82;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15B82: bra.b $c15b7c
pub fn i_c15b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B84;
    cpu.ir = 0x60F8;
    cpu.pc = 0xC15B7C;
    10
}
/// $C15B84: ?
pub fn i_c15b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B88;
    cpu.ir = 0xE;
    let v1 = cpu.a[6] & 0xFF;
    let r2 = (v1 | 0xE) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[6] = r2;
    8
}
/// $C15B88: ?
pub fn i_c15b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B8C;
    cpu.ir = 0xE00;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15B8C: ?
pub fn i_c15b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B8E;
    cpu.ir = 0xE0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15B8E: ?
pub fn i_c15b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B90;
    cpu.ir = 0xE0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15B90: ?
pub fn i_c15b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B92;
    cpu.ir = 0xEE0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15B92: ?
pub fn i_c15b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B94;
    cpu.ir = 0xEE0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15B94: ?
pub fn i_c15b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B98;
    cpu.ir = 0xE0E;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15B98: ?
pub fn i_c15b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B9A;
    cpu.ir = 0xEE;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15B9A: ?
pub fn i_c15b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B9C;
    cpu.ir = 0xEE;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15B9C: add.w d6, d6
pub fn i_c15b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15B9E;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C15B9E: add.w d6, d6
pub fn i_c15b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BA0;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $C15BA0: move.l $c15b84(pc, d6.w), d5
pub fn i_c15ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BA4;
    cpu.ir = 0x2A3B;
    let ad2 = 0xC15B84_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.d[5] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $C15BA4: move.b d0, $3a000f.l
pub fn i_c15ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BAA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A000F, v1 as u8);
    16
}
/// $C15BAA: lea.l $400000.l, a0
pub fn i_c15baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x400000;
    20
}
/// $C15BB0: movea.l a0, a1
pub fn i_c15bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BB2;
    cpu.ir = 0x2248;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $C15BB2: move.w #$7ff, d7
pub fn i_c15bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BB6;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15BB6: move.b d0, $300001.l
pub fn i_c15bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BBC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15BBC: move.l d5, (a0)+
pub fn i_c15bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BBE;
    cpu.ir = 0x20C5;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15BBE: dbra d7, $c15bb6
pub fn i_c15bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BC2;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15BC2; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15BC2; return 14; }
    cpu.pc = 0xC15BB6;
    10
}
/// $C15BC2: move.b d0, $3a001f.l
pub fn i_c15bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BC8;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A001F, v1 as u8);
    16
}
/// $C15BC8: move.w #$7ff, d7
pub fn i_c15bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BCC;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x7FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $C15BCC: move.b d0, $300001.l
pub fn i_c15bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BD2;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15BD2: move.l d5, (a1)+
pub fn i_c15bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BD4;
    cpu.ir = 0x22C5;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let ad2 = cpu.a[1]; cpu.a[1] = ad2.wrapping_add(4);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $C15BD4: dbra d7, $c15bcc
pub fn i_c15bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BD8;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0xC15BD8; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0xC15BD8; return 14; }
    cpu.pc = 0xC15BCC;
    10
}
/// $C15BD8: clr.w $400000.l
pub fn i_c15bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BDE;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x400000) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x400000, r2 as u16);
    16
}
/// $C15BDE: bra.b $c15b74
pub fn i_c15bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BE0;
    cpu.ir = 0x6094;
    cpu.pc = 0xC15B74;
    10
}
/// $C15BE0: move.w #$2700, sr
pub fn i_c15be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BE4;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C15BE4: move.b d0, $3a0013.l
pub fn i_c15be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BEA;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C15BEA: movea.l #$0, a0
pub fn i_c15bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BF0;
    cpu.ir = 0x207C;
    let v1: u32 = 0x0;
    cpu.a[0] = v1;
    4
}
/// $C15BF0: movea.l $10a.w, a1
pub fn i_c15bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BF4;
    cpu.ir = 0x2278;
    let v2 = bus.read32(0x10A);
    cpu.a[1] = v2;
    16
}
/// $C15BF4: moveq #$0, d0
pub fn i_c15bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BF6;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15BF6: move.b d0, $300001.l
pub fn i_c15bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BFC;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15BFC: add.b (a0)+, d0
pub fn i_c15bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15BFE;
    cpu.ir = 0xD018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C15BFE: cmpa.l a0, a1
pub fn i_c15bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C00;
    cpu.ir = 0xB3C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $C15C00: bne.b $c15bf6
pub fn i_c15c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C02;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15BF6; return 10; }
    8
}
/// $C15C02: sub.b $186.w, d0
pub fn i_c15c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C06;
    cpu.ir = 0x9038;
    let v2 = bus.read8(0x186) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $C15C06: move.b d0, $186.w
pub fn i_c15c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C0A;
    cpu.ir = 0x11C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x186, v1 as u8);
    12
}
/// $C15C0A: move.b d0, $100000.l
pub fn i_c15c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C10;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x100000, v1 as u8);
    16
}
/// $C15C10: jmp $c15c10(pc)
pub fn i_c15c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C14;
    cpu.ir = 0x4EFA;
    cpu.pc = 0xC15C10;
    8
}
/// $C15C14: move.w #$2700, sr
pub fn i_c15c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C18;
    cpu.ir = 0x46FC;
    if !cpu.sr.supervisor() { cpu.pc = cpu.instr_pc; cpu.enter_exception(bus, Exception::PrivilegeViolation); return 34; }
    let v1: u32 = 0x2700;
    write_sr(cpu, v1 as u16);
    12
}
/// $C15C18: move.b d0, $3a0013.l
pub fn i_c15c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C1E;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x3A0013, v1 as u8);
    16
}
/// $C15C1E: movea.l #$0, a0
pub fn i_c15c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C24;
    cpu.ir = 0x207C;
    let v1: u32 = 0x0;
    cpu.a[0] = v1;
    4
}
/// $C15C24: movea.l $10a.w, a1
pub fn i_c15c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C28;
    cpu.ir = 0x2278;
    let v2 = bus.read32(0x10A);
    cpu.a[1] = v2;
    16
}
/// $C15C28: moveq #$0, d0
pub fn i_c15c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C2A;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15C2A: move.b d0, $300001.l
pub fn i_c15c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C30;
    cpu.ir = 0x13C0;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x300001, v1 as u8);
    16
}
/// $C15C30: add.b (a0)+, d0
pub fn i_c15c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C32;
    cpu.ir = 0xD018;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    8
}
/// $C15C32: cmpa.l a0, a1
pub fn i_c15c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C34;
    cpu.ir = 0xB3C8;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $C15C34: bne.b $c15c2a
pub fn i_c15c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C36;
    cpu.ir = 0x66F4;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0xC15C2A; return 10; }
    8
}
/// $C15C36: sub.b $186.w, d0
pub fn i_c15c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C3A;
    cpu.ir = 0x9038;
    let v2 = bus.read8(0x186) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $C15C3A: cmp.b $186.w, d0
pub fn i_c15c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C3E;
    cpu.ir = 0xB038;
    let v2 = bus.read8(0x186) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v2, Size::Byte, false);
    12
}
/// $C15C3E: rts
pub fn i_c15c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $C15C40: ori.b #$ff, d1
pub fn i_c15c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C44;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C44: move.l d2, d0
pub fn i_c15c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C46;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15C46: ori.b #$8a, d3
pub fn i_c15c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C4A;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C4A: ori.b #$4f, d7
pub fn i_c15c4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C4E;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C4E: addq.w #$1, a3
pub fn i_c15c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C50;
    cpu.ir = 0x524B;
    cpu.a[3] = cpu.a[3].wrapping_add(1);
    8
}
/// $C15C50: movea.l (a2), a0
pub fn i_c15c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C52;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $C15C52: ?
pub fn i_c15c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C54;
    cpu.ir = 0x414D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15C54: movea.l d5, a0
pub fn i_c15c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C56;
    cpu.ir = 0x2045;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15C56: addq.w #$1, (a2)
pub fn i_c15c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C58;
    cpu.ir = 0x5252;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15C58: ?
pub fn i_c15c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C5A;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15C5A: dc.w $ffff
pub fn i_c15c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C5C;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15C5C: ori.b #$8c, d3
pub fn i_c15c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C60;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C60: ori.b #$c1, d4
pub fn i_c15c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C64;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C64: subq.l #$6, a4
pub fn i_c15c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C66;
    cpu.ir = 0x5D8C;
    cpu.a[4] = cpu.a[4].wrapping_sub(6);
    8
}
/// $C15C66: ori.b #$1, d0
pub fn i_c15c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C6A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C68: ori.b #$ff, d1
pub fn i_c15c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C6C;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C6A: dc.w $f0ff
pub fn i_c15c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C6C;
    cpu.ir = 0xF0FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15C6C: move.l d2, d0
pub fn i_c15c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C6E;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15C6E: ori.b #$8a, d3
pub fn i_c15c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C72;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C72: ori.b #$41, d7
pub fn i_c15c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C76;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x41) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C76: ?
pub fn i_c15c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C78;
    cpu.ir = 0x434B;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15C78: subq.w #$2, (a0)
pub fn i_c15c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C7A;
    cpu.ir = 0x5550;
    let ad1 = cpu.a[0];
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15C7A: movea.l (a2), a0
pub fn i_c15c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C7C;
    cpu.ir = 0x2052;
    let v2 = bus.read32(cpu.a[2]);
    cpu.a[0] = v2;
    12
}
/// $C15C7C: ?
pub fn i_c15c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C7E;
    cpu.ir = 0x414D;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15C7E: movea.l d5, a0
pub fn i_c15c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C80;
    cpu.ir = 0x2045;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $C15C80: addq.w #$1, (a2)
pub fn i_c15c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C82;
    cpu.ir = 0x5252;
    let ad1 = cpu.a[2];
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    8
}
/// $C15C82: ?
pub fn i_c15c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C84;
    cpu.ir = 0x4F52;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15C84: dc.w $ffff
pub fn i_c15c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C86;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15C86: ori.b #$8c, d3
pub fn i_c15c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C8A;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8C) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C8A: ori.b #$c1, d4
pub fn i_c15c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C8E;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0xC1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C8E: subq.l #$6, a4
pub fn i_c15c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C90;
    cpu.ir = 0x5D8C;
    cpu.a[4] = cpu.a[4].wrapping_sub(6);
    8
}
/// $C15C90: ori.b #$1, d0
pub fn i_c15c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C94;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C92: ori.b #$ff, d1
pub fn i_c15c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C96;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C94: dc.w $f0ff
pub fn i_c15c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C96;
    cpu.ir = 0xF0FF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $C15C96: move.l d2, d0
pub fn i_c15c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C98;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15C98: ori.b #$8a, d3
pub fn i_c15c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15C9C;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15C9C: ori.b #$4f, d7
pub fn i_c15c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CA0;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15CA0: ?
pub fn i_c15ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CA2;
    cpu.ir = 0x4C4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15CA2: addq.b #$1, -(a0)
pub fn i_c15ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CA4;
    cpu.ir = 0x5220;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15CA4: addq.w #$1, d1
pub fn i_c15ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CA6;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15CA6: dc.w $4d20
pub fn i_c15ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CA8;
    cpu.ir = 0x4D20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15CA8: clr.w d1
pub fn i_c15ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CAA;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15CAA: trap #$b
pub fn i_c15caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CAC;
    cpu.ir = 0x4E4B;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xAC);
    34
}
/// $C15CAC: move.w -(a0), d0
pub fn i_c15cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CAE;
    cpu.ir = 0x3020;
    let ad2 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $C15CAE: ?
pub fn i_c15cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CB0;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15CB0: addq.w #$1, a7
pub fn i_c15cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CB2;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C15CC0: ori.b #$ff, d1
pub fn i_c15cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CC4;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15CC4: move.l d2, d0
pub fn i_c15cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CC6;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15CC6: ori.b #$8a, d3
pub fn i_c15cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CCA;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15CCA: ori.b #$4f, d7
pub fn i_c15cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CCE;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x4F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15CCE: ?
pub fn i_c15cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CD0;
    cpu.ir = 0x4C4F;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15CD0: addq.b #$1, -(a0)
pub fn i_c15cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CD2;
    cpu.ir = 0x5220;
    let ad1 = cpu.a[0].wrapping_sub(1); cpu.a[0] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    10
}
/// $C15CD2: addq.w #$1, d1
pub fn i_c15cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CD4;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15CD4: dc.w $4d20
pub fn i_c15cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CD6;
    cpu.ir = 0x4D20;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15CD6: clr.w d1
pub fn i_c15cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CD8;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $C15CD8: trap #$b
pub fn i_c15cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CDA;
    cpu.ir = 0x4E4B;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xAC);
    34
}
/// $C15CDA: move.w -(a0), -(a0)
pub fn i_c15cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CDC;
    cpu.ir = 0x3120;
    let ad2 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_sub(2); cpu.a[0] = ad4;
    bus.write16(ad4, v3 as u16);
    16
}
/// $C15CDC: ?
pub fn i_c15cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CDE;
    cpu.ir = 0x4552;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::IllegalInstruction);
    return 34;
}
/// $C15CDE: addq.w #$1, a7
pub fn i_c15cde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CE0;
    cpu.ir = 0x524F;
    cpu.a[7] = cpu.a[7].wrapping_add(1);
    8
}
/// $C15CEE: ori.b #$ff, d1
pub fn i_c15cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CF2;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0xFF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15CF2: move.l d2, d0
pub fn i_c15cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CF4;
    cpu.ir = 0x2002;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $C15CF4: ori.b #$8a, d3
pub fn i_c15cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CF8;
    cpu.ir = 0x3;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 | 0x8A) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    8
}
/// $C15CF8: ori.b #$49, d7
pub fn i_c15cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0xC15CFC;
    cpu.ir = 0x7;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = (v1 | 0x49) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0xC141A0, i_c141a0),
    (0xC141A2, i_c141a2),
    (0xC141A6, i_c141a6),
    (0xC141AA, i_c141aa),
    (0xC141AE, i_c141ae),
    (0xC141B2, i_c141b2),
    (0xC141B6, i_c141b6),
    (0xC141BA, i_c141ba),
    (0xC141BE, i_c141be),
    (0xC141C0, i_c141c0),
    (0xC141C2, i_c141c2),
    (0xC141C6, i_c141c6),
    (0xC141CA, i_c141ca),
    (0xC141CE, i_c141ce),
    (0xC141D2, i_c141d2),
    (0xC141D4, i_c141d4),
    (0xC141D8, i_c141d8),
    (0xC141DA, i_c141da),
    (0xC141DC, i_c141dc),
    (0xC141E0, i_c141e0),
    (0xC141E2, i_c141e2),
    (0xC141E4, i_c141e4),
    (0xC141E6, i_c141e6),
    (0xC141EA, i_c141ea),
    (0xC141EE, i_c141ee),
    (0xC141F0, i_c141f0),
    (0xC141F4, i_c141f4),
    (0xC141F8, i_c141f8),
    (0xC141FC, i_c141fc),
    (0xC141FE, i_c141fe),
    (0xC14200, i_c14200),
    (0xC14202, i_c14202),
    (0xC14208, i_c14208),
    (0xC1420A, i_c1420a),
    (0xC1420E, i_c1420e),
    (0xC14210, i_c14210),
    (0xC14212, i_c14212),
    (0xC14216, i_c14216),
    (0xC14218, i_c14218),
    (0xC1421C, i_c1421c),
    (0xC14220, i_c14220),
    (0xC14224, i_c14224),
    (0xC14226, i_c14226),
    (0xC1422A, i_c1422a),
    (0xC1422E, i_c1422e),
    (0xC14234, i_c14234),
    (0xC14236, i_c14236),
    (0xC1423A, i_c1423a),
    (0xC1423C, i_c1423c),
    (0xC14240, i_c14240),
    (0xC14244, i_c14244),
    (0xC1424C, i_c1424c),
    (0xC14252, i_c14252),
    (0xC14258, i_c14258),
    (0xC1425C, i_c1425c),
    (0xC14260, i_c14260),
    (0xC14264, i_c14264),
    (0xC1426A, i_c1426a),
    (0xC1426E, i_c1426e),
    (0xC14272, i_c14272),
    (0xC14276, i_c14276),
    (0xC1427A, i_c1427a),
    (0xC1427C, i_c1427c),
    (0xC14280, i_c14280),
    (0xC14282, i_c14282),
    (0xC14288, i_c14288),
    (0xC1428E, i_c1428e),
    (0xC14292, i_c14292),
    (0xC14296, i_c14296),
    (0xC1429A, i_c1429a),
    (0xC1429E, i_c1429e),
    (0xC142A2, i_c142a2),
    (0xC142A6, i_c142a6),
    (0xC142AA, i_c142aa),
    (0xC142AC, i_c142ac),
    (0xC142B0, i_c142b0),
    (0xC142B4, i_c142b4),
    (0xC142B6, i_c142b6),
    (0xC142BA, i_c142ba),
    (0xC142BE, i_c142be),
    (0xC142C2, i_c142c2),
    (0xC142C6, i_c142c6),
    (0xC142C8, i_c142c8),
    (0xC142CC, i_c142cc),
    (0xC142D0, i_c142d0),
    (0xC142D6, i_c142d6),
    (0xC142D8, i_c142d8),
    (0xC142DC, i_c142dc),
    (0xC142DE, i_c142de),
    (0xC142E2, i_c142e2),
    (0xC142E6, i_c142e6),
    (0xC142EA, i_c142ea),
    (0xC142EE, i_c142ee),
    (0xC142F0, i_c142f0),
    (0xC142F8, i_c142f8),
    (0xC142FE, i_c142fe),
    (0xC14302, i_c14302),
    (0xC14306, i_c14306),
    (0xC1430A, i_c1430a),
    (0xC1430E, i_c1430e),
    (0xC14312, i_c14312),
    (0xC14316, i_c14316),
    (0xC1431A, i_c1431a),
    (0xC1431E, i_c1431e),
    (0xC14322, i_c14322),
    (0xC14328, i_c14328),
    (0xC1432E, i_c1432e),
    (0xC14332, i_c14332),
    (0xC14336, i_c14336),
    (0xC1433A, i_c1433a),
    (0xC14340, i_c14340),
    (0xC14346, i_c14346),
    (0xC1434C, i_c1434c),
    (0xC14352, i_c14352),
    (0xC14358, i_c14358),
    (0xC1435C, i_c1435c),
    (0xC14360, i_c14360),
    (0xC14364, i_c14364),
    (0xC14368, i_c14368),
    (0xC1436C, i_c1436c),
    (0xC14370, i_c14370),
    (0xC14372, i_c14372),
    (0xC14374, i_c14374),
    (0xC14376, i_c14376),
    (0xC14378, i_c14378),
    (0xC1437C, i_c1437c),
    (0xC14380, i_c14380),
    (0xC14384, i_c14384),
    (0xC1438A, i_c1438a),
    (0xC1438E, i_c1438e),
    (0xC14394, i_c14394),
    (0xC14398, i_c14398),
    (0xC1439C, i_c1439c),
    (0xC143A0, i_c143a0),
    (0xC143A4, i_c143a4),
    (0xC143A6, i_c143a6),
    (0xC143AA, i_c143aa),
    (0xC143AC, i_c143ac),
    (0xC143B4, i_c143b4),
    (0xC143B8, i_c143b8),
    (0xC143BE, i_c143be),
    (0xC143C2, i_c143c2),
    (0xC143C6, i_c143c6),
    (0xC143C8, i_c143c8),
    (0xC143CC, i_c143cc),
    (0xC143D0, i_c143d0),
    (0xC143D4, i_c143d4),
    (0xC143D8, i_c143d8),
    (0xC143DC, i_c143dc),
    (0xC143E0, i_c143e0),
    (0xC143E4, i_c143e4),
    (0xC143E6, i_c143e6),
    (0xC143EC, i_c143ec),
    (0xC143F0, i_c143f0),
    (0xC143F4, i_c143f4),
    (0xC143F8, i_c143f8),
    (0xC143FC, i_c143fc),
    (0xC143FE, i_c143fe),
    (0xC14404, i_c14404),
    (0xC1440A, i_c1440a),
    (0xC1440E, i_c1440e),
    (0xC14414, i_c14414),
    (0xC1441A, i_c1441a),
    (0xC1441E, i_c1441e),
    (0xC14422, i_c14422),
    (0xC14426, i_c14426),
    (0xC1442C, i_c1442c),
    (0xC14432, i_c14432),
    (0xC14438, i_c14438),
    (0xC1443E, i_c1443e),
    (0xC14442, i_c14442),
    (0xC14446, i_c14446),
    (0xC1444A, i_c1444a),
    (0xC1444E, i_c1444e),
    (0xC14452, i_c14452),
    (0xC14454, i_c14454),
    (0xC14458, i_c14458),
    (0xC1445A, i_c1445a),
    (0xC1445E, i_c1445e),
    (0xC14462, i_c14462),
    (0xC14466, i_c14466),
    (0xC1446A, i_c1446a),
    (0xC1446E, i_c1446e),
    (0xC14472, i_c14472),
    (0xC14476, i_c14476),
    (0xC1447A, i_c1447a),
    (0xC1447C, i_c1447c),
    (0xC14480, i_c14480),
    (0xC14482, i_c14482),
    (0xC14486, i_c14486),
    (0xC1448A, i_c1448a),
    (0xC1448E, i_c1448e),
    (0xC14492, i_c14492),
    (0xC14496, i_c14496),
    (0xC1449A, i_c1449a),
    (0xC1449E, i_c1449e),
    (0xC144A0, i_c144a0),
    (0xC144A2, i_c144a2),
    (0xC144A6, i_c144a6),
    (0xC144AA, i_c144aa),
    (0xC144AC, i_c144ac),
    (0xC144B0, i_c144b0),
    (0xC144B2, i_c144b2),
    (0xC144B4, i_c144b4),
    (0xC144B8, i_c144b8),
    (0xC144BA, i_c144ba),
    (0xC144BC, i_c144bc),
    (0xC144BE, i_c144be),
    (0xC144C2, i_c144c2),
    (0xC144C6, i_c144c6),
    (0xC144C8, i_c144c8),
    (0xC144CC, i_c144cc),
    (0xC144D0, i_c144d0),
    (0xC144D4, i_c144d4),
    (0xC144D6, i_c144d6),
    (0xC144D8, i_c144d8),
    (0xC144DA, i_c144da),
    (0xC144E0, i_c144e0),
    (0xC144E2, i_c144e2),
    (0xC144E6, i_c144e6),
    (0xC144E8, i_c144e8),
    (0xC144EA, i_c144ea),
    (0xC144EE, i_c144ee),
    (0xC144F0, i_c144f0),
    (0xC144F4, i_c144f4),
    (0xC144F8, i_c144f8),
    (0xC144FC, i_c144fc),
    (0xC144FE, i_c144fe),
    (0xC14502, i_c14502),
    (0xC14506, i_c14506),
    (0xC1450C, i_c1450c),
    (0xC1450E, i_c1450e),
    (0xC14512, i_c14512),
    (0xC14514, i_c14514),
    (0xC14518, i_c14518),
    (0xC1451C, i_c1451c),
    (0xC14520, i_c14520),
    (0xC14524, i_c14524),
    (0xC14528, i_c14528),
    (0xC1452C, i_c1452c),
    (0xC1452E, i_c1452e),
    (0xC14532, i_c14532),
    (0xC14534, i_c14534),
    (0xC1453A, i_c1453a),
    (0xC1453E, i_c1453e),
    (0xC14542, i_c14542),
    (0xC14546, i_c14546),
    (0xC1454A, i_c1454a),
    (0xC1454E, i_c1454e),
    (0xC14552, i_c14552),
    (0xC14556, i_c14556),
    (0xC1455A, i_c1455a),
    (0xC1455E, i_c1455e),
    (0xC14562, i_c14562),
    (0xC14564, i_c14564),
    (0xC14568, i_c14568),
    (0xC1456A, i_c1456a),
    (0xC1456E, i_c1456e),
    (0xC14576, i_c14576),
    (0xC1457C, i_c1457c),
    (0xC14580, i_c14580),
    (0xC14584, i_c14584),
    (0xC14586, i_c14586),
    (0xC1458C, i_c1458c),
    (0xC1458E, i_c1458e),
    (0xC14592, i_c14592),
    (0xC14596, i_c14596),
    (0xC1459A, i_c1459a),
    (0xC145A2, i_c145a2),
    (0xC145A6, i_c145a6),
    (0xC145AA, i_c145aa),
    (0xC145AE, i_c145ae),
    (0xC145B2, i_c145b2),
    (0xC145B6, i_c145b6),
    (0xC145BA, i_c145ba),
    (0xC145BE, i_c145be),
    (0xC145C2, i_c145c2),
    (0xC145C6, i_c145c6),
    (0xC145CA, i_c145ca),
    (0xC145CC, i_c145cc),
    (0xC145CE, i_c145ce),
    (0xC145D2, i_c145d2),
    (0xC145D6, i_c145d6),
    (0xC145DA, i_c145da),
    (0xC145DE, i_c145de),
    (0xC145E2, i_c145e2),
    (0xC145E8, i_c145e8),
    (0xC145EC, i_c145ec),
    (0xC145F0, i_c145f0),
    (0xC145F6, i_c145f6),
    (0xC145F8, i_c145f8),
    (0xC145FC, i_c145fc),
    (0xC14600, i_c14600),
    (0xC14604, i_c14604),
    (0xC14608, i_c14608),
    (0xC1460A, i_c1460a),
    (0xC1460C, i_c1460c),
    (0xC1460E, i_c1460e),
    (0xC14612, i_c14612),
    (0xC14614, i_c14614),
    (0xC14616, i_c14616),
    (0xC1461A, i_c1461a),
    (0xC1461C, i_c1461c),
    (0xC1461E, i_c1461e),
    (0xC14620, i_c14620),
    (0xC14622, i_c14622),
    (0xC14624, i_c14624),
    (0xC14626, i_c14626),
    (0xC14628, i_c14628),
    (0xC14630, i_c14630),
    (0xC14636, i_c14636),
    (0xC1463A, i_c1463a),
    (0xC1463E, i_c1463e),
    (0xC14642, i_c14642),
    (0xC14644, i_c14644),
    (0xC14648, i_c14648),
    (0xC1464A, i_c1464a),
    (0xC1464C, i_c1464c),
    (0xC14650, i_c14650),
    (0xC14652, i_c14652),
    (0xC14656, i_c14656),
    (0xC1465A, i_c1465a),
    (0xC1465E, i_c1465e),
    (0xC14662, i_c14662),
    (0xC14668, i_c14668),
    (0xC1466E, i_c1466e),
    (0xC14670, i_c14670),
    (0xC14676, i_c14676),
    (0xC1467A, i_c1467a),
    (0xC1467E, i_c1467e),
    (0xC14682, i_c14682),
    (0xC14688, i_c14688),
    (0xC1468E, i_c1468e),
    (0xC14690, i_c14690),
    (0xC14696, i_c14696),
    (0xC1469A, i_c1469a),
    (0xC1469E, i_c1469e),
    (0xC146A0, i_c146a0),
    (0xC146A2, i_c146a2),
    (0xC146A6, i_c146a6),
    (0xC146AA, i_c146aa),
    (0xC146AC, i_c146ac),
    (0xC146B0, i_c146b0),
    (0xC146B4, i_c146b4),
    (0xC146B8, i_c146b8),
    (0xC146BC, i_c146bc),
    (0xC146C0, i_c146c0),
    (0xC146C2, i_c146c2),
    (0xC146C4, i_c146c4),
    (0xC146C6, i_c146c6),
    (0xC146CC, i_c146cc),
    (0xC146D0, i_c146d0),
    (0xC146D4, i_c146d4),
    (0xC146D8, i_c146d8),
    (0xC146DE, i_c146de),
    (0xC146E0, i_c146e0),
    (0xC146E4, i_c146e4),
    (0xC146EA, i_c146ea),
    (0xC146EE, i_c146ee),
    (0xC146F2, i_c146f2),
    (0xC146F4, i_c146f4),
    (0xC146FA, i_c146fa),
    (0xC146FC, i_c146fc),
    (0xC146FE, i_c146fe),
    (0xC14700, i_c14700),
    (0xC14706, i_c14706),
    (0xC14708, i_c14708),
    (0xC1470C, i_c1470c),
    (0xC14710, i_c14710),
    (0xC14714, i_c14714),
    (0xC14716, i_c14716),
    (0xC14718, i_c14718),
    (0xC1471A, i_c1471a),
    (0xC1471C, i_c1471c),
    (0xC1471E, i_c1471e),
    (0xC14720, i_c14720),
    (0xC14722, i_c14722),
    (0xC14724, i_c14724),
    (0xC14726, i_c14726),
    (0xC14728, i_c14728),
    (0xC1472A, i_c1472a),
    (0xC14730, i_c14730),
    (0xC14734, i_c14734),
    (0xC14736, i_c14736),
    (0xC14738, i_c14738),
    (0xC1473A, i_c1473a),
    (0xC1473C, i_c1473c),
    (0xC1473E, i_c1473e),
    (0xC14740, i_c14740),
    (0xC14742, i_c14742),
    (0xC14746, i_c14746),
    (0xC14748, i_c14748),
    (0xC1474C, i_c1474c),
    (0xC14750, i_c14750),
    (0xC14752, i_c14752),
    (0xC14758, i_c14758),
    (0xC1475C, i_c1475c),
    (0xC14762, i_c14762),
    (0xC14764, i_c14764),
    (0xC14766, i_c14766),
    (0xC1476A, i_c1476a),
    (0xC14770, i_c14770),
    (0xC14774, i_c14774),
    (0xC14778, i_c14778),
    (0xC1477A, i_c1477a),
    (0xC1477E, i_c1477e),
    (0xC14782, i_c14782),
    (0xC14784, i_c14784),
    (0xC14786, i_c14786),
    (0xC1478A, i_c1478a),
    (0xC14790, i_c14790),
    (0xC14792, i_c14792),
    (0xC14796, i_c14796),
    (0xC14798, i_c14798),
    (0xC1479C, i_c1479c),
    (0xC147A0, i_c147a0),
    (0xC147A2, i_c147a2),
    (0xC147A6, i_c147a6),
    (0xC147AA, i_c147aa),
    (0xC147AC, i_c147ac),
    (0xC147B0, i_c147b0),
    (0xC147B2, i_c147b2),
    (0xC147B6, i_c147b6),
    (0xC147B8, i_c147b8),
    (0xC147BC, i_c147bc),
    (0xC147BE, i_c147be),
    (0xC147C2, i_c147c2),
    (0xC147C6, i_c147c6),
    (0xC147C8, i_c147c8),
    (0xC147CE, i_c147ce),
    (0xC147D2, i_c147d2),
    (0xC147D4, i_c147d4),
    (0xC147D6, i_c147d6),
    (0xC147DA, i_c147da),
    (0xC147E0, i_c147e0),
    (0xC147E4, i_c147e4),
    (0xC147E8, i_c147e8),
    (0xC147EA, i_c147ea),
    (0xC147EC, i_c147ec),
    (0xC147F0, i_c147f0),
    (0xC147F4, i_c147f4),
    (0xC147F6, i_c147f6),
    (0xC147F8, i_c147f8),
    (0xC147FC, i_c147fc),
    (0xC147FE, i_c147fe),
    (0xC14802, i_c14802),
    (0xC14804, i_c14804),
    (0xC14808, i_c14808),
    (0xC1480A, i_c1480a),
    (0xC1480C, i_c1480c),
    (0xC14810, i_c14810),
    (0xC14814, i_c14814),
    (0xC14818, i_c14818),
    (0xC1481C, i_c1481c),
    (0xC1481E, i_c1481e),
    (0xC14824, i_c14824),
    (0xC14828, i_c14828),
    (0xC1482C, i_c1482c),
    (0xC14830, i_c14830),
    (0xC14834, i_c14834),
    (0xC14836, i_c14836),
    (0xC14838, i_c14838),
    (0xC1483C, i_c1483c),
    (0xC14844, i_c14844),
    (0xC1484A, i_c1484a),
    (0xC1484E, i_c1484e),
    (0xC14852, i_c14852),
    (0xC14854, i_c14854),
    (0xC14858, i_c14858),
    (0xC1485E, i_c1485e),
    (0xC14862, i_c14862),
    (0xC14866, i_c14866),
    (0xC1486A, i_c1486a),
    (0xC1486C, i_c1486c),
    (0xC14870, i_c14870),
    (0xC14872, i_c14872),
    (0xC14874, i_c14874),
    (0xC14876, i_c14876),
    (0xC1487C, i_c1487c),
    (0xC1487E, i_c1487e),
    (0xC14882, i_c14882),
    (0xC14886, i_c14886),
    (0xC1488A, i_c1488a),
    (0xC1488C, i_c1488c),
    (0xC14890, i_c14890),
    (0xC14894, i_c14894),
    (0xC14898, i_c14898),
    (0xC1489C, i_c1489c),
    (0xC1489E, i_c1489e),
    (0xC148A4, i_c148a4),
    (0xC148A8, i_c148a8),
    (0xC148AA, i_c148aa),
    (0xC148B0, i_c148b0),
    (0xC148B4, i_c148b4),
    (0xC148B8, i_c148b8),
    (0xC148BE, i_c148be),
    (0xC148C0, i_c148c0),
    (0xC148C2, i_c148c2),
    (0xC148C6, i_c148c6),
    (0xC148CA, i_c148ca),
    (0xC148CC, i_c148cc),
    (0xC148D0, i_c148d0),
    (0xC148D2, i_c148d2),
    (0xC148D6, i_c148d6),
    (0xC148D8, i_c148d8),
    (0xC148DA, i_c148da),
    (0xC148DE, i_c148de),
    (0xC148E2, i_c148e2),
    (0xC148E4, i_c148e4),
    (0xC148E8, i_c148e8),
    (0xC148EA, i_c148ea),
    (0xC148EE, i_c148ee),
    (0xC148F2, i_c148f2),
    (0xC148F4, i_c148f4),
    (0xC148F6, i_c148f6),
    (0xC148FA, i_c148fa),
    (0xC148FE, i_c148fe),
    (0xC14902, i_c14902),
    (0xC14904, i_c14904),
    (0xC14906, i_c14906),
    (0xC14908, i_c14908),
    (0xC1490E, i_c1490e),
    (0xC14912, i_c14912),
    (0xC14916, i_c14916),
    (0xC14918, i_c14918),
    (0xC1491C, i_c1491c),
    (0xC14920, i_c14920),
    (0xC14922, i_c14922),
    (0xC14926, i_c14926),
    (0xC1492C, i_c1492c),
    (0xC1492E, i_c1492e),
    (0xC14932, i_c14932),
    (0xC14936, i_c14936),
    (0xC1493C, i_c1493c),
    (0xC1493E, i_c1493e),
    (0xC14942, i_c14942),
    (0xC14944, i_c14944),
    (0xC14946, i_c14946),
    (0xC14948, i_c14948),
    (0xC1494A, i_c1494a),
    (0xC1494C, i_c1494c),
    (0xC14952, i_c14952),
    (0xC14956, i_c14956),
    (0xC14958, i_c14958),
    (0xC1495C, i_c1495c),
    (0xC14960, i_c14960),
    (0xC14962, i_c14962),
    (0xC14968, i_c14968),
    (0xC1496C, i_c1496c),
    (0xC14970, i_c14970),
    (0xC14972, i_c14972),
    (0xC14976, i_c14976),
    (0xC1497A, i_c1497a),
    (0xC1497C, i_c1497c),
    (0xC1497E, i_c1497e),
    (0xC14982, i_c14982),
    (0xC14986, i_c14986),
    (0xC14988, i_c14988),
    (0xC1498E, i_c1498e),
    (0xC14992, i_c14992),
    (0xC14996, i_c14996),
    (0xC14998, i_c14998),
    (0xC1499C, i_c1499c),
    (0xC149A0, i_c149a0),
    (0xC149A2, i_c149a2),
    (0xC149A6, i_c149a6),
    (0xC149AA, i_c149aa),
    (0xC149AC, i_c149ac),
    (0xC149B2, i_c149b2),
    (0xC149B6, i_c149b6),
    (0xC149B8, i_c149b8),
    (0xC149BC, i_c149bc),
    (0xC149BE, i_c149be),
    (0xC149C0, i_c149c0),
    (0xC14C0E, i_c14c0e),
    (0xC14C12, i_c14c12),
    (0xC14C16, i_c14c16),
    (0xC14C18, i_c14c18),
    (0xC14C1A, i_c14c1a),
    (0xC14C1C, i_c14c1c),
    (0xC14C1E, i_c14c1e),
    (0xC14C20, i_c14c20),
    (0xC14C22, i_c14c22),
    (0xC14C24, i_c14c24),
    (0xC14CB2, i_c14cb2),
    (0xC14CB6, i_c14cb6),
    (0xC14CBA, i_c14cba),
    (0xC14CBC, i_c14cbc),
    (0xC14CBE, i_c14cbe),
    (0xC14CC0, i_c14cc0),
    (0xC14CC2, i_c14cc2),
    (0xC14CC4, i_c14cc4),
    (0xC14CC6, i_c14cc6),
    (0xC14CC8, i_c14cc8),
    (0xC14CCA, i_c14cca),
    (0xC14CCC, i_c14ccc),
    (0xC14CCE, i_c14cce),
    (0xC14CD0, i_c14cd0),
    (0xC14CD2, i_c14cd2),
    (0xC14CD4, i_c14cd4),
    (0xC14CD6, i_c14cd6),
    (0xC14CD8, i_c14cd8),
    (0xC14CDA, i_c14cda),
    (0xC14D8E, i_c14d8e),
    (0xC14D92, i_c14d92),
    (0xC14D96, i_c14d96),
    (0xC14D98, i_c14d98),
    (0xC14D9A, i_c14d9a),
    (0xC14D9C, i_c14d9c),
    (0xC14D9E, i_c14d9e),
    (0xC14DA0, i_c14da0),
    (0xC14DA2, i_c14da2),
    (0xC14DA4, i_c14da4),
    (0xC14DA6, i_c14da6),
    (0xC14DA8, i_c14da8),
    (0xC14DAA, i_c14daa),
    (0xC14DAC, i_c14dac),
    (0xC14DAE, i_c14dae),
    (0xC14DB0, i_c14db0),
    (0xC14DB2, i_c14db2),
    (0xC14DB4, i_c14db4),
    (0xC14DB6, i_c14db6),
    (0xC150CE, i_c150ce),
    (0xC150D2, i_c150d2),
    (0xC150D6, i_c150d6),
    (0xC150D8, i_c150d8),
    (0xC150DA, i_c150da),
    (0xC150DC, i_c150dc),
    (0xC150DE, i_c150de),
    (0xC150E0, i_c150e0),
    (0xC150E2, i_c150e2),
    (0xC150E4, i_c150e4),
    (0xC150E6, i_c150e6),
    (0xC150E8, i_c150e8),
    (0xC150EA, i_c150ea),
    (0xC150EC, i_c150ec),
    (0xC150F0, i_c150f0),
    (0xC150F2, i_c150f2),
    (0xC150F6, i_c150f6),
    (0xC150FA, i_c150fa),
    (0xC150FC, i_c150fc),
    (0xC150FE, i_c150fe),
    (0xC15100, i_c15100),
    (0xC15102, i_c15102),
    (0xC15104, i_c15104),
    (0xC15106, i_c15106),
    (0xC15108, i_c15108),
    (0xC1510A, i_c1510a),
    (0xC1510C, i_c1510c),
    (0xC15110, i_c15110),
    (0xC15114, i_c15114),
    (0xC15116, i_c15116),
    (0xC15118, i_c15118),
    (0xC1511A, i_c1511a),
    (0xC1511C, i_c1511c),
    (0xC1511E, i_c1511e),
    (0xC15120, i_c15120),
    (0xC15122, i_c15122),
    (0xC15124, i_c15124),
    (0xC15126, i_c15126),
    (0xC15128, i_c15128),
    (0xC1512A, i_c1512a),
    (0xC1512E, i_c1512e),
    (0xC15132, i_c15132),
    (0xC15134, i_c15134),
    (0xC15136, i_c15136),
    (0xC15138, i_c15138),
    (0xC1513A, i_c1513a),
    (0xC1513C, i_c1513c),
    (0xC1513E, i_c1513e),
    (0xC15140, i_c15140),
    (0xC15142, i_c15142),
    (0xC15144, i_c15144),
    (0xC15146, i_c15146),
    (0xC15148, i_c15148),
    (0xC1514C, i_c1514c),
    (0xC1514E, i_c1514e),
    (0xC15150, i_c15150),
    (0xC15152, i_c15152),
    (0xC15154, i_c15154),
    (0xC15156, i_c15156),
    (0xC15158, i_c15158),
    (0xC1515A, i_c1515a),
    (0xC1515C, i_c1515c),
    (0xC1515E, i_c1515e),
    (0xC15160, i_c15160),
    (0xC15162, i_c15162),
    (0xC15164, i_c15164),
    (0xC15168, i_c15168),
    (0xC1516C, i_c1516c),
    (0xC15170, i_c15170),
    (0xC15172, i_c15172),
    (0xC1517A, i_c1517a),
    (0xC1517E, i_c1517e),
    (0xC15186, i_c15186),
    (0xC1518C, i_c1518c),
    (0xC1518E, i_c1518e),
    (0xC15194, i_c15194),
    (0xC15198, i_c15198),
    (0xC1519E, i_c1519e),
    (0xC151A2, i_c151a2),
    (0xC151A4, i_c151a4),
    (0xC151A8, i_c151a8),
    (0xC151AC, i_c151ac),
    (0xC151B0, i_c151b0),
    (0xC151B4, i_c151b4),
    (0xC151B8, i_c151b8),
    (0xC151BC, i_c151bc),
    (0xC151C0, i_c151c0),
    (0xC151C4, i_c151c4),
    (0xC151C8, i_c151c8),
    (0xC151CC, i_c151cc),
    (0xC151D2, i_c151d2),
    (0xC151D4, i_c151d4),
    (0xC151D6, i_c151d6),
    (0xC151DE, i_c151de),
    (0xC151E0, i_c151e0),
    (0xC151E8, i_c151e8),
    (0xC15210, i_c15210),
    (0xC15214, i_c15214),
    (0xC1521C, i_c1521c),
    (0xC15222, i_c15222),
    (0xC15228, i_c15228),
    (0xC1522A, i_c1522a),
    (0xC1522E, i_c1522e),
    (0xC15232, i_c15232),
    (0xC15236, i_c15236),
    (0xC1523A, i_c1523a),
    (0xC1523E, i_c1523e),
    (0xC15246, i_c15246),
    (0xC15250, i_c15250),
    (0xC15256, i_c15256),
    (0xC1525A, i_c1525a),
    (0xC1525E, i_c1525e),
    (0xC15262, i_c15262),
    (0xC15266, i_c15266),
    (0xC1526A, i_c1526a),
    (0xC15270, i_c15270),
    (0xC15274, i_c15274),
    (0xC1527C, i_c1527c),
    (0xC15284, i_c15284),
    (0xC15288, i_c15288),
    (0xC1528C, i_c1528c),
    (0xC15290, i_c15290),
    (0xC15296, i_c15296),
    (0xC15298, i_c15298),
    (0xC152A0, i_c152a0),
    (0xC152A4, i_c152a4),
    (0xC152A8, i_c152a8),
    (0xC152AA, i_c152aa),
    (0xC152AE, i_c152ae),
    (0xC152B0, i_c152b0),
    (0xC152B4, i_c152b4),
    (0xC152B6, i_c152b6),
    (0xC152B8, i_c152b8),
    (0xC152BC, i_c152bc),
    (0xC152C0, i_c152c0),
    (0xC152C2, i_c152c2),
    (0xC152C8, i_c152c8),
    (0xC152CA, i_c152ca),
    (0xC152D2, i_c152d2),
    (0xC152D4, i_c152d4),
    (0xC152D6, i_c152d6),
    (0xC152D8, i_c152d8),
    (0xC152DA, i_c152da),
    (0xC152E2, i_c152e2),
    (0xC152E6, i_c152e6),
    (0xC152EC, i_c152ec),
    (0xC152EE, i_c152ee),
    (0xC152F2, i_c152f2),
    (0xC152F8, i_c152f8),
    (0xC152FC, i_c152fc),
    (0xC15300, i_c15300),
    (0xC15302, i_c15302),
    (0xC15306, i_c15306),
    (0xC1530A, i_c1530a),
    (0xC1530C, i_c1530c),
    (0xC15310, i_c15310),
    (0xC15312, i_c15312),
    (0xC15314, i_c15314),
    (0xC15318, i_c15318),
    (0xC1531A, i_c1531a),
    (0xC1531C, i_c1531c),
    (0xC1531E, i_c1531e),
    (0xC15320, i_c15320),
    (0xC15324, i_c15324),
    (0xC15326, i_c15326),
    (0xC15328, i_c15328),
    (0xC1532C, i_c1532c),
    (0xC1532E, i_c1532e),
    (0xC15332, i_c15332),
    (0xC15334, i_c15334),
    (0xC15336, i_c15336),
    (0xC1533A, i_c1533a),
    (0xC1533E, i_c1533e),
    (0xC15340, i_c15340),
    (0xC15344, i_c15344),
    (0xC1534A, i_c1534a),
    (0xC1534E, i_c1534e),
    (0xC15352, i_c15352),
    (0xC15356, i_c15356),
    (0xC1535A, i_c1535a),
    (0xC1535E, i_c1535e),
    (0xC15360, i_c15360),
    (0xC15362, i_c15362),
    (0xC15366, i_c15366),
    (0xC15368, i_c15368),
    (0xC1536A, i_c1536a),
    (0xC1536E, i_c1536e),
    (0xC15372, i_c15372),
    (0xC15376, i_c15376),
    (0xC15378, i_c15378),
    (0xC1537C, i_c1537c),
    (0xC1537E, i_c1537e),
    (0xC15380, i_c15380),
    (0xC15382, i_c15382),
    (0xC15384, i_c15384),
    (0xC15388, i_c15388),
    (0xC1538A, i_c1538a),
    (0xC1538C, i_c1538c),
    (0xC1538E, i_c1538e),
    (0xC15390, i_c15390),
    (0xC15392, i_c15392),
    (0xC15394, i_c15394),
    (0xC15398, i_c15398),
    (0xC1539A, i_c1539a),
    (0xC1539C, i_c1539c),
    (0xC1539E, i_c1539e),
    (0xC153A0, i_c153a0),
    (0xC153A2, i_c153a2),
    (0xC153A6, i_c153a6),
    (0xC153AA, i_c153aa),
    (0xC153AE, i_c153ae),
    (0xC153B2, i_c153b2),
    (0xC153B4, i_c153b4),
    (0xC153B8, i_c153b8),
    (0xC153BA, i_c153ba),
    (0xC153BC, i_c153bc),
    (0xC153C0, i_c153c0),
    (0xC153C2, i_c153c2),
    (0xC153C4, i_c153c4),
    (0xC153C8, i_c153c8),
    (0xC153CA, i_c153ca),
    (0xC153CC, i_c153cc),
    (0xC153D0, i_c153d0),
    (0xC153D2, i_c153d2),
    (0xC153D4, i_c153d4),
    (0xC153D8, i_c153d8),
    (0xC153DC, i_c153dc),
    (0xC153E0, i_c153e0),
    (0xC153E4, i_c153e4),
    (0xC153E6, i_c153e6),
    (0xC153EA, i_c153ea),
    (0xC153EE, i_c153ee),
    (0xC153F0, i_c153f0),
    (0xC153F2, i_c153f2),
    (0xC153F4, i_c153f4),
    (0xC153FA, i_c153fa),
    (0xC153FE, i_c153fe),
    (0xC15400, i_c15400),
    (0xC15404, i_c15404),
    (0xC15408, i_c15408),
    (0xC1540C, i_c1540c),
    (0xC15410, i_c15410),
    (0xC15414, i_c15414),
    (0xC1541A, i_c1541a),
    (0xC1541C, i_c1541c),
    (0xC1541E, i_c1541e),
    (0xC15422, i_c15422),
    (0xC15426, i_c15426),
    (0xC15428, i_c15428),
    (0xC1542E, i_c1542e),
    (0xC15432, i_c15432),
    (0xC15438, i_c15438),
    (0xC1543A, i_c1543a),
    (0xC1543E, i_c1543e),
    (0xC15440, i_c15440),
    (0xC15444, i_c15444),
    (0xC15448, i_c15448),
    (0xC1544C, i_c1544c),
    (0xC15450, i_c15450),
    (0xC15456, i_c15456),
    (0xC1545C, i_c1545c),
    (0xC15460, i_c15460),
    (0xC15462, i_c15462),
    (0xC15464, i_c15464),
    (0xC15468, i_c15468),
    (0xC1546A, i_c1546a),
    (0xC1546E, i_c1546e),
    (0xC15472, i_c15472),
    (0xC15476, i_c15476),
    (0xC1547A, i_c1547a),
    (0xC1547E, i_c1547e),
    (0xC15482, i_c15482),
    (0xC15484, i_c15484),
    (0xC15488, i_c15488),
    (0xC1548A, i_c1548a),
    (0xC1548C, i_c1548c),
    (0xC15490, i_c15490),
    (0xC15494, i_c15494),
    (0xC15496, i_c15496),
    (0xC15498, i_c15498),
    (0xC1549C, i_c1549c),
    (0xC1549E, i_c1549e),
    (0xC154A0, i_c154a0),
    (0xC154A4, i_c154a4),
    (0xC154A6, i_c154a6),
    (0xC154A8, i_c154a8),
    (0xC154AC, i_c154ac),
    (0xC154B0, i_c154b0),
    (0xC154B4, i_c154b4),
    (0xC154B8, i_c154b8),
    (0xC154BA, i_c154ba),
    (0xC154BE, i_c154be),
    (0xC154C2, i_c154c2),
    (0xC154C4, i_c154c4),
    (0xC154C6, i_c154c6),
    (0xC154C8, i_c154c8),
    (0xC154CA, i_c154ca),
    (0xC154D0, i_c154d0),
    (0xC154D2, i_c154d2),
    (0xC154D4, i_c154d4),
    (0xC154D6, i_c154d6),
    (0xC154DC, i_c154dc),
    (0xC154DE, i_c154de),
    (0xC154E4, i_c154e4),
    (0xC154E6, i_c154e6),
    (0xC154E8, i_c154e8),
    (0xC154EE, i_c154ee),
    (0xC154F2, i_c154f2),
    (0xC154F4, i_c154f4),
    (0xC154F8, i_c154f8),
    (0xC154FC, i_c154fc),
    (0xC15500, i_c15500),
    (0xC15504, i_c15504),
    (0xC15508, i_c15508),
    (0xC1550E, i_c1550e),
    (0xC15510, i_c15510),
    (0xC15512, i_c15512),
    (0xC15518, i_c15518),
    (0xC1551C, i_c1551c),
    (0xC15520, i_c15520),
    (0xC15522, i_c15522),
    (0xC15528, i_c15528),
    (0xC1552C, i_c1552c),
    (0xC15532, i_c15532),
    (0xC15534, i_c15534),
    (0xC15538, i_c15538),
    (0xC1553A, i_c1553a),
    (0xC1553E, i_c1553e),
    (0xC15540, i_c15540),
    (0xC15544, i_c15544),
    (0xC15548, i_c15548),
    (0xC1554E, i_c1554e),
    (0xC15554, i_c15554),
    (0xC15558, i_c15558),
    (0xC1555A, i_c1555a),
    (0xC1555C, i_c1555c),
    (0xC15560, i_c15560),
    (0xC15562, i_c15562),
    (0xC15566, i_c15566),
    (0xC15568, i_c15568),
    (0xC1556A, i_c1556a),
    (0xC1556C, i_c1556c),
    (0xC1556E, i_c1556e),
    (0xC15570, i_c15570),
    (0xC15572, i_c15572),
    (0xC15574, i_c15574),
    (0xC15578, i_c15578),
    (0xC1557A, i_c1557a),
    (0xC1557E, i_c1557e),
    (0xC15580, i_c15580),
    (0xC15582, i_c15582),
    (0xC15584, i_c15584),
    (0xC15588, i_c15588),
    (0xC1558C, i_c1558c),
    (0xC1558E, i_c1558e),
    (0xC15592, i_c15592),
    (0xC15596, i_c15596),
    (0xC1559A, i_c1559a),
    (0xC1559E, i_c1559e),
    (0xC155A2, i_c155a2),
    (0xC155A4, i_c155a4),
    (0xC155A8, i_c155a8),
    (0xC155AA, i_c155aa),
    (0xC155AC, i_c155ac),
    (0xC155B0, i_c155b0),
    (0xC155B2, i_c155b2),
    (0xC155B4, i_c155b4),
    (0xC155B8, i_c155b8),
    (0xC155BA, i_c155ba),
    (0xC155BC, i_c155bc),
    (0xC155C0, i_c155c0),
    (0xC155C2, i_c155c2),
    (0xC155C4, i_c155c4),
    (0xC155C8, i_c155c8),
    (0xC155CC, i_c155cc),
    (0xC155D0, i_c155d0),
    (0xC155D2, i_c155d2),
    (0xC155D6, i_c155d6),
    (0xC155DA, i_c155da),
    (0xC155DC, i_c155dc),
    (0xC155DE, i_c155de),
    (0xC155E4, i_c155e4),
    (0xC155E8, i_c155e8),
    (0xC155EC, i_c155ec),
    (0xC155EE, i_c155ee),
    (0xC155F4, i_c155f4),
    (0xC155F8, i_c155f8),
    (0xC155FA, i_c155fa),
    (0xC155FE, i_c155fe),
    (0xC15600, i_c15600),
    (0xC15604, i_c15604),
    (0xC15608, i_c15608),
    (0xC1560C, i_c1560c),
    (0xC15610, i_c15610),
    (0xC15614, i_c15614),
    (0xC15618, i_c15618),
    (0xC1561C, i_c1561c),
    (0xC15620, i_c15620),
    (0xC15624, i_c15624),
    (0xC15628, i_c15628),
    (0xC1562C, i_c1562c),
    (0xC1562E, i_c1562e),
    (0xC15634, i_c15634),
    (0xC15636, i_c15636),
    (0xC1563A, i_c1563a),
    (0xC1563E, i_c1563e),
    (0xC15640, i_c15640),
    (0xC15644, i_c15644),
    (0xC15648, i_c15648),
    (0xC1564A, i_c1564a),
    (0xC1564E, i_c1564e),
    (0xC15652, i_c15652),
    (0xC15656, i_c15656),
    (0xC1565A, i_c1565a),
    (0xC1565E, i_c1565e),
    (0xC15660, i_c15660),
    (0xC15664, i_c15664),
    (0xC15666, i_c15666),
    (0xC15668, i_c15668),
    (0xC1566C, i_c1566c),
    (0xC15672, i_c15672),
    (0xC15674, i_c15674),
    (0xC15678, i_c15678),
    (0xC1567A, i_c1567a),
    (0xC1567E, i_c1567e),
    (0xC15682, i_c15682),
    (0xC15684, i_c15684),
    (0xC15686, i_c15686),
    (0xC15688, i_c15688),
    (0xC1568A, i_c1568a),
    (0xC1568E, i_c1568e),
    (0xC15692, i_c15692),
    (0xC15696, i_c15696),
    (0xC15698, i_c15698),
    (0xC1569C, i_c1569c),
    (0xC156A0, i_c156a0),
    (0xC156A2, i_c156a2),
    (0xC156A4, i_c156a4),
    (0xC156AA, i_c156aa),
    (0xC156AE, i_c156ae),
    (0xC156B2, i_c156b2),
    (0xC156B4, i_c156b4),
    (0xC156B6, i_c156b6),
    (0xC156BC, i_c156bc),
    (0xC156BE, i_c156be),
    (0xC156C0, i_c156c0),
    (0xC156C2, i_c156c2),
    (0xC156C8, i_c156c8),
    (0xC156CA, i_c156ca),
    (0xC156D0, i_c156d0),
    (0xC156D2, i_c156d2),
    (0xC156D4, i_c156d4),
    (0xC156DA, i_c156da),
    (0xC156DE, i_c156de),
    (0xC156E4, i_c156e4),
    (0xC156E6, i_c156e6),
    (0xC156E8, i_c156e8),
    (0xC156EA, i_c156ea),
    (0xC156EC, i_c156ec),
    (0xC156F4, i_c156f4),
    (0xC156F6, i_c156f6),
    (0xC156FA, i_c156fa),
    (0xC156FE, i_c156fe),
    (0xC15702, i_c15702),
    (0xC15706, i_c15706),
    (0xC1570A, i_c1570a),
    (0xC1570E, i_c1570e),
    (0xC15712, i_c15712),
    (0xC15716, i_c15716),
    (0xC1571A, i_c1571a),
    (0xC1571E, i_c1571e),
    (0xC15722, i_c15722),
    (0xC15724, i_c15724),
    (0xC1572A, i_c1572a),
    (0xC1572C, i_c1572c),
    (0xC15734, i_c15734),
    (0xC1573C, i_c1573c),
    (0xC1573E, i_c1573e),
    (0xC15742, i_c15742),
    (0xC15744, i_c15744),
    (0xC1574A, i_c1574a),
    (0xC15750, i_c15750),
    (0xC15754, i_c15754),
    (0xC15758, i_c15758),
    (0xC1575E, i_c1575e),
    (0xC15760, i_c15760),
    (0xC15764, i_c15764),
    (0xC15766, i_c15766),
    (0xC15768, i_c15768),
    (0xC1576A, i_c1576a),
    (0xC1576E, i_c1576e),
    (0xC15770, i_c15770),
    (0xC15774, i_c15774),
    (0xC15776, i_c15776),
    (0xC1577A, i_c1577a),
    (0xC1577C, i_c1577c),
    (0xC15780, i_c15780),
    (0xC15784, i_c15784),
    (0xC15786, i_c15786),
    (0xC1578A, i_c1578a),
    (0xC1578E, i_c1578e),
    (0xC15792, i_c15792),
    (0xC15796, i_c15796),
    (0xC15798, i_c15798),
    (0xC1579C, i_c1579c),
    (0xC1579E, i_c1579e),
    (0xC157A0, i_c157a0),
    (0xC157A4, i_c157a4),
    (0xC157AA, i_c157aa),
    (0xC157AC, i_c157ac),
    (0xC157B0, i_c157b0),
    (0xC157B4, i_c157b4),
    (0xC157B8, i_c157b8),
    (0xC157BA, i_c157ba),
    (0xC157BC, i_c157bc),
    (0xC157C0, i_c157c0),
    (0xC157C4, i_c157c4),
    (0xC157C8, i_c157c8),
    (0xC157CC, i_c157cc),
    (0xC157D0, i_c157d0),
    (0xC157D4, i_c157d4),
    (0xC157D8, i_c157d8),
    (0xC157DC, i_c157dc),
    (0xC157DE, i_c157de),
    (0xC157E2, i_c157e2),
    (0xC157E4, i_c157e4),
    (0xC157E6, i_c157e6),
    (0xC157EA, i_c157ea),
    (0xC157EC, i_c157ec),
    (0xC157EE, i_c157ee),
    (0xC157F2, i_c157f2),
    (0xC157F4, i_c157f4),
    (0xC157F6, i_c157f6),
    (0xC157FA, i_c157fa),
    (0xC157FC, i_c157fc),
    (0xC15800, i_c15800),
    (0xC15806, i_c15806),
    (0xC15808, i_c15808),
    (0xC1580A, i_c1580a),
    (0xC1580E, i_c1580e),
    (0xC15812, i_c15812),
    (0xC15816, i_c15816),
    (0xC1581A, i_c1581a),
    (0xC1581C, i_c1581c),
    (0xC15820, i_c15820),
    (0xC15824, i_c15824),
    (0xC15828, i_c15828),
    (0xC1582A, i_c1582a),
    (0xC1582C, i_c1582c),
    (0xC15830, i_c15830),
    (0xC15834, i_c15834),
    (0xC15836, i_c15836),
    (0xC15838, i_c15838),
    (0xC1583C, i_c1583c),
    (0xC1583E, i_c1583e),
    (0xC15844, i_c15844),
    (0xC15846, i_c15846),
    (0xC1584A, i_c1584a),
    (0xC1584E, i_c1584e),
    (0xC15850, i_c15850),
    (0xC15854, i_c15854),
    (0xC1585A, i_c1585a),
    (0xC1585E, i_c1585e),
    (0xC15862, i_c15862),
    (0xC15864, i_c15864),
    (0xC15868, i_c15868),
    (0xC1586C, i_c1586c),
    (0xC15870, i_c15870),
    (0xC15874, i_c15874),
    (0xC15878, i_c15878),
    (0xC1587C, i_c1587c),
    (0xC15880, i_c15880),
    (0xC15884, i_c15884),
    (0xC15886, i_c15886),
    (0xC15888, i_c15888),
    (0xC1588A, i_c1588a),
    (0xC1588C, i_c1588c),
    (0xC1588E, i_c1588e),
    (0xC15890, i_c15890),
    (0xC15892, i_c15892),
    (0xC15894, i_c15894),
    (0xC15896, i_c15896),
    (0xC15898, i_c15898),
    (0xC1589A, i_c1589a),
    (0xC1589C, i_c1589c),
    (0xC1589E, i_c1589e),
    (0xC158A0, i_c158a0),
    (0xC158B4, i_c158b4),
    (0xC158BA, i_c158ba),
    (0xC158BC, i_c158bc),
    (0xC158BE, i_c158be),
    (0xC158C4, i_c158c4),
    (0xC158CA, i_c158ca),
    (0xC158D2, i_c158d2),
    (0xC158D6, i_c158d6),
    (0xC158D8, i_c158d8),
    (0xC158DC, i_c158dc),
    (0xC158E0, i_c158e0),
    (0xC158E4, i_c158e4),
    (0xC158EC, i_c158ec),
    (0xC158EE, i_c158ee),
    (0xC158F4, i_c158f4),
    (0xC158F6, i_c158f6),
    (0xC158F8, i_c158f8),
    (0xC158FE, i_c158fe),
    (0xC15900, i_c15900),
    (0xC15904, i_c15904),
    (0xC15906, i_c15906),
    (0xC1590C, i_c1590c),
    (0xC15910, i_c15910),
    (0xC15914, i_c15914),
    (0xC15916, i_c15916),
    (0xC1591A, i_c1591a),
    (0xC1591E, i_c1591e),
    (0xC15924, i_c15924),
    (0xC15928, i_c15928),
    (0xC1592A, i_c1592a),
    (0xC1592C, i_c1592c),
    (0xC1592E, i_c1592e),
    (0xC15930, i_c15930),
    (0xC15946, i_c15946),
    (0xC15948, i_c15948),
    (0xC1594E, i_c1594e),
    (0xC15952, i_c15952),
    (0xC15958, i_c15958),
    (0xC1595C, i_c1595c),
    (0xC15962, i_c15962),
    (0xC15964, i_c15964),
    (0xC15968, i_c15968),
    (0xC1596C, i_c1596c),
    (0xC15970, i_c15970),
    (0xC15974, i_c15974),
    (0xC15978, i_c15978),
    (0xC1597C, i_c1597c),
    (0xC15982, i_c15982),
    (0xC15984, i_c15984),
    (0xC15988, i_c15988),
    (0xC1598C, i_c1598c),
    (0xC15990, i_c15990),
    (0xC15992, i_c15992),
    (0xC15998, i_c15998),
    (0xC1599C, i_c1599c),
    (0xC159A2, i_c159a2),
    (0xC159A6, i_c159a6),
    (0xC159A8, i_c159a8),
    (0xC159AC, i_c159ac),
    (0xC159AE, i_c159ae),
    (0xC159B2, i_c159b2),
    (0xC159B6, i_c159b6),
    (0xC159BA, i_c159ba),
    (0xC159BE, i_c159be),
    (0xC159C2, i_c159c2),
    (0xC159C6, i_c159c6),
    (0xC159CC, i_c159cc),
    (0xC159D2, i_c159d2),
    (0xC159D6, i_c159d6),
    (0xC159DA, i_c159da),
    (0xC159DE, i_c159de),
    (0xC159E2, i_c159e2),
    (0xC159E6, i_c159e6),
    (0xC159EA, i_c159ea),
    (0xC159EE, i_c159ee),
    (0xC159F2, i_c159f2),
    (0xC159F6, i_c159f6),
    (0xC159FA, i_c159fa),
    (0xC159FE, i_c159fe),
    (0xC15A02, i_c15a02),
    (0xC15A06, i_c15a06),
    (0xC15A0A, i_c15a0a),
    (0xC15A0E, i_c15a0e),
    (0xC15A10, i_c15a10),
    (0xC15A14, i_c15a14),
    (0xC15A18, i_c15a18),
    (0xC15A1A, i_c15a1a),
    (0xC15A1E, i_c15a1e),
    (0xC15A20, i_c15a20),
    (0xC15A26, i_c15a26),
    (0xC15A2A, i_c15a2a),
    (0xC15A30, i_c15a30),
    (0xC15A34, i_c15a34),
    (0xC15A36, i_c15a36),
    (0xC15A3A, i_c15a3a),
    (0xC15A3E, i_c15a3e),
    (0xC15A44, i_c15a44),
    (0xC15A48, i_c15a48),
    (0xC15A4A, i_c15a4a),
    (0xC15A4E, i_c15a4e),
    (0xC15A52, i_c15a52),
    (0xC15A54, i_c15a54),
    (0xC15A5A, i_c15a5a),
    (0xC15A5E, i_c15a5e),
    (0xC15A64, i_c15a64),
    (0xC15A68, i_c15a68),
    (0xC15A6C, i_c15a6c),
    (0xC15A70, i_c15a70),
    (0xC15A76, i_c15a76),
    (0xC15A7A, i_c15a7a),
    (0xC15A7C, i_c15a7c),
    (0xC15A7E, i_c15a7e),
    (0xC15A82, i_c15a82),
    (0xC15A84, i_c15a84),
    (0xC15A86, i_c15a86),
    (0xC15A88, i_c15a88),
    (0xC15A8A, i_c15a8a),
    (0xC15A8C, i_c15a8c),
    (0xC15A8E, i_c15a8e),
    (0xC15A92, i_c15a92),
    (0xC15A94, i_c15a94),
    (0xC15A96, i_c15a96),
    (0xC15A98, i_c15a98),
    (0xC15A9A, i_c15a9a),
    (0xC15A9E, i_c15a9e),
    (0xC15AA0, i_c15aa0),
    (0xC15AA2, i_c15aa2),
    (0xC15AA6, i_c15aa6),
    (0xC15AA8, i_c15aa8),
    (0xC15AAA, i_c15aaa),
    (0xC15AAC, i_c15aac),
    (0xC15AAE, i_c15aae),
    (0xC15AB2, i_c15ab2),
    (0xC15ABA, i_c15aba),
    (0xC15AC0, i_c15ac0),
    (0xC15AC4, i_c15ac4),
    (0xC15AC8, i_c15ac8),
    (0xC15ACC, i_c15acc),
    (0xC15AD0, i_c15ad0),
    (0xC15AD4, i_c15ad4),
    (0xC15ADC, i_c15adc),
    (0xC15AE2, i_c15ae2),
    (0xC15AEA, i_c15aea),
    (0xC15AEE, i_c15aee),
    (0xC15AF2, i_c15af2),
    (0xC15AF8, i_c15af8),
    (0xC15AFC, i_c15afc),
    (0xC15B06, i_c15b06),
    (0xC15B0A, i_c15b0a),
    (0xC15B0E, i_c15b0e),
    (0xC15B10, i_c15b10),
    (0xC15B14, i_c15b14),
    (0xC15B18, i_c15b18),
    (0xC15B1C, i_c15b1c),
    (0xC15B20, i_c15b20),
    (0xC15B24, i_c15b24),
    (0xC15B26, i_c15b26),
    (0xC15B2C, i_c15b2c),
    (0xC15B30, i_c15b30),
    (0xC15B36, i_c15b36),
    (0xC15B3A, i_c15b3a),
    (0xC15B3E, i_c15b3e),
    (0xC15B42, i_c15b42),
    (0xC15B46, i_c15b46),
    (0xC15B4A, i_c15b4a),
    (0xC15B4E, i_c15b4e),
    (0xC15B52, i_c15b52),
    (0xC15B56, i_c15b56),
    (0xC15B5A, i_c15b5a),
    (0xC15B5E, i_c15b5e),
    (0xC15B62, i_c15b62),
    (0xC15B68, i_c15b68),
    (0xC15B6C, i_c15b6c),
    (0xC15B70, i_c15b70),
    (0xC15B74, i_c15b74),
    (0xC15B7C, i_c15b7c),
    (0xC15B82, i_c15b82),
    (0xC15B84, i_c15b84),
    (0xC15B88, i_c15b88),
    (0xC15B8C, i_c15b8c),
    (0xC15B8E, i_c15b8e),
    (0xC15B90, i_c15b90),
    (0xC15B92, i_c15b92),
    (0xC15B94, i_c15b94),
    (0xC15B98, i_c15b98),
    (0xC15B9A, i_c15b9a),
    (0xC15B9C, i_c15b9c),
    (0xC15B9E, i_c15b9e),
    (0xC15BA0, i_c15ba0),
    (0xC15BA4, i_c15ba4),
    (0xC15BAA, i_c15baa),
    (0xC15BB0, i_c15bb0),
    (0xC15BB2, i_c15bb2),
    (0xC15BB6, i_c15bb6),
    (0xC15BBC, i_c15bbc),
    (0xC15BBE, i_c15bbe),
    (0xC15BC2, i_c15bc2),
    (0xC15BC8, i_c15bc8),
    (0xC15BCC, i_c15bcc),
    (0xC15BD2, i_c15bd2),
    (0xC15BD4, i_c15bd4),
    (0xC15BD8, i_c15bd8),
    (0xC15BDE, i_c15bde),
    (0xC15BE0, i_c15be0),
    (0xC15BE4, i_c15be4),
    (0xC15BEA, i_c15bea),
    (0xC15BF0, i_c15bf0),
    (0xC15BF4, i_c15bf4),
    (0xC15BF6, i_c15bf6),
    (0xC15BFC, i_c15bfc),
    (0xC15BFE, i_c15bfe),
    (0xC15C00, i_c15c00),
    (0xC15C02, i_c15c02),
    (0xC15C06, i_c15c06),
    (0xC15C0A, i_c15c0a),
    (0xC15C10, i_c15c10),
    (0xC15C14, i_c15c14),
    (0xC15C18, i_c15c18),
    (0xC15C1E, i_c15c1e),
    (0xC15C24, i_c15c24),
    (0xC15C28, i_c15c28),
    (0xC15C2A, i_c15c2a),
    (0xC15C30, i_c15c30),
    (0xC15C32, i_c15c32),
    (0xC15C34, i_c15c34),
    (0xC15C36, i_c15c36),
    (0xC15C3A, i_c15c3a),
    (0xC15C3E, i_c15c3e),
    (0xC15C40, i_c15c40),
    (0xC15C44, i_c15c44),
    (0xC15C46, i_c15c46),
    (0xC15C4A, i_c15c4a),
    (0xC15C4E, i_c15c4e),
    (0xC15C50, i_c15c50),
    (0xC15C52, i_c15c52),
    (0xC15C54, i_c15c54),
    (0xC15C56, i_c15c56),
    (0xC15C58, i_c15c58),
    (0xC15C5A, i_c15c5a),
    (0xC15C5C, i_c15c5c),
    (0xC15C60, i_c15c60),
    (0xC15C64, i_c15c64),
    (0xC15C66, i_c15c66),
    (0xC15C68, i_c15c68),
    (0xC15C6A, i_c15c6a),
    (0xC15C6C, i_c15c6c),
    (0xC15C6E, i_c15c6e),
    (0xC15C72, i_c15c72),
    (0xC15C76, i_c15c76),
    (0xC15C78, i_c15c78),
    (0xC15C7A, i_c15c7a),
    (0xC15C7C, i_c15c7c),
    (0xC15C7E, i_c15c7e),
    (0xC15C80, i_c15c80),
    (0xC15C82, i_c15c82),
    (0xC15C84, i_c15c84),
    (0xC15C86, i_c15c86),
    (0xC15C8A, i_c15c8a),
    (0xC15C8E, i_c15c8e),
    (0xC15C90, i_c15c90),
    (0xC15C92, i_c15c92),
    (0xC15C94, i_c15c94),
    (0xC15C96, i_c15c96),
    (0xC15C98, i_c15c98),
    (0xC15C9C, i_c15c9c),
    (0xC15CA0, i_c15ca0),
    (0xC15CA2, i_c15ca2),
    (0xC15CA4, i_c15ca4),
    (0xC15CA6, i_c15ca6),
    (0xC15CA8, i_c15ca8),
    (0xC15CAA, i_c15caa),
    (0xC15CAC, i_c15cac),
    (0xC15CAE, i_c15cae),
    (0xC15CB0, i_c15cb0),
    (0xC15CC0, i_c15cc0),
    (0xC15CC4, i_c15cc4),
    (0xC15CC6, i_c15cc6),
    (0xC15CCA, i_c15cca),
    (0xC15CCE, i_c15cce),
    (0xC15CD0, i_c15cd0),
    (0xC15CD2, i_c15cd2),
    (0xC15CD4, i_c15cd4),
    (0xC15CD6, i_c15cd6),
    (0xC15CD8, i_c15cd8),
    (0xC15CDA, i_c15cda),
    (0xC15CDC, i_c15cdc),
    (0xC15CDE, i_c15cde),
    (0xC15CEE, i_c15cee),
    (0xC15CF2, i_c15cf2),
    (0xC15CF4, i_c15cf4),
    (0xC15CF8, i_c15cf8),
];
