// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $05C6A0: move.w #$8401, d6
pub fn i_05c6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6A4;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C6A4: add.w d0, d6
pub fn i_05c6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6A6;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C6A6: swap d6
pub fn i_05c6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6A8;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C6A8: move.w $a(a3), d6
pub fn i_05c6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6AC;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C6AC: move.l #$2000000, d5
pub fn i_05c6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6B2;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C6B2: move.l d6, (a4)
pub fn i_05c6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6B4;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C6B4: sub.l d5, d6
pub fn i_05c6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6B6;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C6B6: move.w $8(a3), d6
pub fn i_05c6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6BA;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C6BA: move.l d6, (a4)
pub fn i_05c6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6BC;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C6BC: sub.l d5, d6
pub fn i_05c6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6BE;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C6BE: move.w $6(a3), d6
pub fn i_05c6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6C2;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C6C2: move.l d6, (a4)
pub fn i_05c6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6C4;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C6C4: rts
pub fn i_05c6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05C6C6: move.w #$40, d7
pub fn i_05c6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6CA;
    cpu.ir = 0x3E3C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C6CA: add.w d1, d7
pub fn i_05c6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6CC;
    cpu.ir = 0xDE41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[7] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C6CC: move.w d7, d5
pub fn i_05c6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6CE;
    cpu.ir = 0x3A07;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C6CE: addq.w #$1, d5
pub fn i_05c6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6D0;
    cpu.ir = 0x5245;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05C6D0: swap d5
pub fn i_05c6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6D2;
    cpu.ir = 0x4845;
    cpu.d[5] = cpu.d[5].rotate_left(16);
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C6D2: move.w $4(a3), d5
pub fn i_05c6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6D6;
    cpu.ir = 0x3A2B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C6D6: swap d7
pub fn i_05c6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6D8;
    cpu.ir = 0x4847;
    cpu.d[7] = cpu.d[7].rotate_left(16);
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C6D8: moveq #$2, d4
pub fn i_05c6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6DA;
    cpu.ir = 0x7802;
    cpu.d[4] = 0x2;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C6DA: swap d4
pub fn i_05c6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6DC;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C6DC: movea.l (a3), a6
pub fn i_05c6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6DE;
    cpu.ir = 0x2C53;
    let v2 = bus.read32(cpu.a[3]);
    cpu.a[6] = v2;
    12
}
/// $05C6DE: move.w $a(a3), d6
pub fn i_05c6de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6E2;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C6E2: subq.b #$1, d6
pub fn i_05c6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6E4;
    cpu.ir = 0x5306;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05C6E4: andi.w #$1f, d6
pub fn i_05c6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6E8;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05C6E8: add.w d6, d6
pub fn i_05c6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6EA;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C6EA: add.w d6, d6
pub fn i_05c6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6EC;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C6EC: jmp $5c6f0(pc, d6.w)
pub fn i_05c6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6F0;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5C6F0_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05C6F0: bra.w $5c8a2
pub fn i_05c6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6F4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C8A2;
    10
}
/// $05C6F4: bra.w $5c898
pub fn i_05c6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C898;
    10
}
/// $05C6F8: bra.w $5c88e
pub fn i_05c6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C6FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C88E;
    10
}
/// $05C6FC: bra.w $5c884
pub fn i_05c6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C700;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C884;
    10
}
/// $05C700: bra.w $5c87a
pub fn i_05c700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C704;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C87A;
    10
}
/// $05C704: bra.w $5c870
pub fn i_05c704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C708;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C870;
    10
}
/// $05C708: bra.w $5c866
pub fn i_05c708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C70C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C866;
    10
}
/// $05C70C: bra.w $5c85c
pub fn i_05c70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C710;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C85C;
    10
}
/// $05C710: bra.w $5c852
pub fn i_05c710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C714;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C852;
    10
}
/// $05C714: bra.w $5c848
pub fn i_05c714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C718;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C848;
    10
}
/// $05C718: bra.w $5c83e
pub fn i_05c718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C71C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C83E;
    10
}
/// $05C71C: bra.w $5c834
pub fn i_05c71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C720;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C834;
    10
}
/// $05C720: bra.w $5c82a
pub fn i_05c720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C724;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C82A;
    10
}
/// $05C724: bra.w $5c820
pub fn i_05c724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C728;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C820;
    10
}
/// $05C728: bra.w $5c816
pub fn i_05c728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C72C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C816;
    10
}
/// $05C72C: bra.w $5c80c
pub fn i_05c72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C730;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C80C;
    10
}
/// $05C730: bra.w $5c802
pub fn i_05c730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C734;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C802;
    10
}
/// $05C734: bra.w $5c7f8
pub fn i_05c734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C738;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7F8;
    10
}
/// $05C738: bra.w $5c7ee
pub fn i_05c738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C73C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7EE;
    10
}
/// $05C73C: bra.w $5c7e4
pub fn i_05c73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C740;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7E4;
    10
}
/// $05C740: bra.w $5c7da
pub fn i_05c740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C744;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7DA;
    10
}
/// $05C744: bra.w $5c7d0
pub fn i_05c744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C748;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7D0;
    10
}
/// $05C748: bra.w $5c7c6
pub fn i_05c748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C74C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7C6;
    10
}
/// $05C74C: bra.w $5c7bc
pub fn i_05c74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C750;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7BC;
    10
}
/// $05C750: bra.w $5c7b2
pub fn i_05c750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C754;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7B2;
    10
}
/// $05C754: bra.w $5c7a8
pub fn i_05c754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C758;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C7A8;
    10
}
/// $05C758: bra.w $5c79e
pub fn i_05c758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C75C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C79E;
    10
}
/// $05C75C: bra.w $5c794
pub fn i_05c75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C760;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C794;
    10
}
/// $05C760: bra.w $5c78a
pub fn i_05c760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C764;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C78A;
    10
}
/// $05C764: bra.w $5c780
pub fn i_05c764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C768;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C780;
    10
}
/// $05C768: bra.w $5c776
pub fn i_05c768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C76C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5C776;
    10
}
/// $05C76C: move.w -(a6), d7
pub fn i_05c76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C76E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C76E: move.l d7, (a4)
pub fn i_05c76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C770;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C770: add.l d4, d7
pub fn i_05c770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C772;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C772: move.l d5, (a4)
pub fn i_05c772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C774;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C774: add.l d4, d5
pub fn i_05c774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C776;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C776: move.w -(a6), d7
pub fn i_05c776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C778;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C778: move.l d7, (a4)
pub fn i_05c778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C77A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C77A: add.l d4, d7
pub fn i_05c77a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C77C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C77C: move.l d5, (a4)
pub fn i_05c77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C77E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C77E: add.l d4, d5
pub fn i_05c77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C780;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C780: move.w -(a6), d7
pub fn i_05c780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C782;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C782: move.l d7, (a4)
pub fn i_05c782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C784;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C784: add.l d4, d7
pub fn i_05c784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C786;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C786: move.l d5, (a4)
pub fn i_05c786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C788;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C788: add.l d4, d5
pub fn i_05c788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C78A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C78A: move.w -(a6), d7
pub fn i_05c78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C78C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C78C: move.l d7, (a4)
pub fn i_05c78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C78E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C78E: add.l d4, d7
pub fn i_05c78e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C790;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C790: move.l d5, (a4)
pub fn i_05c790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C792;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C792: add.l d4, d5
pub fn i_05c792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C794;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C794: move.w -(a6), d7
pub fn i_05c794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C796;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C796: move.l d7, (a4)
pub fn i_05c796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C798;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C798: add.l d4, d7
pub fn i_05c798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C79A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C79A: move.l d5, (a4)
pub fn i_05c79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C79C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C79C: add.l d4, d5
pub fn i_05c79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C79E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C79E: move.w -(a6), d7
pub fn i_05c79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7A0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7A0: move.l d7, (a4)
pub fn i_05c7a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7A2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7A2: add.l d4, d7
pub fn i_05c7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7A4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7A4: move.l d5, (a4)
pub fn i_05c7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7A6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7A6: add.l d4, d5
pub fn i_05c7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7A8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7A8: move.w -(a6), d7
pub fn i_05c7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7AA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7AA: move.l d7, (a4)
pub fn i_05c7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7AC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7AC: add.l d4, d7
pub fn i_05c7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7AE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7AE: move.l d5, (a4)
pub fn i_05c7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7B0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7B0: add.l d4, d5
pub fn i_05c7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7B2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7B2: move.w -(a6), d7
pub fn i_05c7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7B4;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7B4: move.l d7, (a4)
pub fn i_05c7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7B6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7B6: add.l d4, d7
pub fn i_05c7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7B8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7B8: move.l d5, (a4)
pub fn i_05c7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7BA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7BA: add.l d4, d5
pub fn i_05c7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7BC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7BC: move.w -(a6), d7
pub fn i_05c7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7BE;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7BE: move.l d7, (a4)
pub fn i_05c7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7C0;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7C0: add.l d4, d7
pub fn i_05c7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7C2;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7C2: move.l d5, (a4)
pub fn i_05c7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7C4;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7C4: add.l d4, d5
pub fn i_05c7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7C6;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7C6: move.w -(a6), d7
pub fn i_05c7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7C8;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7C8: move.l d7, (a4)
pub fn i_05c7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7CA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7CA: add.l d4, d7
pub fn i_05c7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7CC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7CC: move.l d5, (a4)
pub fn i_05c7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7CE;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7CE: add.l d4, d5
pub fn i_05c7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7D0;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7D0: move.w -(a6), d7
pub fn i_05c7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7D2;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7D2: move.l d7, (a4)
pub fn i_05c7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7D4;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7D4: add.l d4, d7
pub fn i_05c7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7D6;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7D6: move.l d5, (a4)
pub fn i_05c7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7D8;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7D8: add.l d4, d5
pub fn i_05c7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7DA;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7DA: move.w -(a6), d7
pub fn i_05c7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7DC;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7DC: move.l d7, (a4)
pub fn i_05c7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7DE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7DE: add.l d4, d7
pub fn i_05c7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7E0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7E0: move.l d5, (a4)
pub fn i_05c7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7E2;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7E2: add.l d4, d5
pub fn i_05c7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7E4;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7E4: move.w -(a6), d7
pub fn i_05c7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7E6;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7E6: move.l d7, (a4)
pub fn i_05c7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7E8;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7E8: add.l d4, d7
pub fn i_05c7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7EA;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7EA: move.l d5, (a4)
pub fn i_05c7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7EC;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7EC: add.l d4, d5
pub fn i_05c7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7EE;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7EE: move.w -(a6), d7
pub fn i_05c7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7F0;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7F0: move.l d7, (a4)
pub fn i_05c7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7F2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7F2: add.l d4, d7
pub fn i_05c7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7F4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7F4: move.l d5, (a4)
pub fn i_05c7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7F6;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7F6: add.l d4, d5
pub fn i_05c7f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7F8;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C7F8: move.w -(a6), d7
pub fn i_05c7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7FA;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C7FA: move.l d7, (a4)
pub fn i_05c7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7FC;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C7FC: add.l d4, d7
pub fn i_05c7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C7FE;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C7FE: move.l d5, (a4)
pub fn i_05c7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C800;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C800: add.l d4, d5
pub fn i_05c800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C802;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C802: move.w -(a6), d7
pub fn i_05c802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C804;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C804: move.l d7, (a4)
pub fn i_05c804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C806;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C806: add.l d4, d7
pub fn i_05c806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C808;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C808: move.l d5, (a4)
pub fn i_05c808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C80A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C80A: add.l d4, d5
pub fn i_05c80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C80C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C80C: move.w -(a6), d7
pub fn i_05c80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C80E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C80E: move.l d7, (a4)
pub fn i_05c80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C810;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C810: add.l d4, d7
pub fn i_05c810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C812;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C812: move.l d5, (a4)
pub fn i_05c812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C814;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C814: add.l d4, d5
pub fn i_05c814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C816;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C816: move.w -(a6), d7
pub fn i_05c816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C818;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C818: move.l d7, (a4)
pub fn i_05c818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C81A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C81A: add.l d4, d7
pub fn i_05c81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C81C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C81C: move.l d5, (a4)
pub fn i_05c81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C81E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C81E: add.l d4, d5
pub fn i_05c81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C820;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C820: move.w -(a6), d7
pub fn i_05c820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C822;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C822: move.l d7, (a4)
pub fn i_05c822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C824;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C824: add.l d4, d7
pub fn i_05c824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C826;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C826: move.l d5, (a4)
pub fn i_05c826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C828;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C828: add.l d4, d5
pub fn i_05c828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C82A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C82A: move.w -(a6), d7
pub fn i_05c82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C82C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C82C: move.l d7, (a4)
pub fn i_05c82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C82E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C82E: add.l d4, d7
pub fn i_05c82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C830;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C830: move.l d5, (a4)
pub fn i_05c830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C832;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C832: add.l d4, d5
pub fn i_05c832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C834;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C834: move.w -(a6), d7
pub fn i_05c834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C836;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C836: move.l d7, (a4)
pub fn i_05c836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C838;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C838: add.l d4, d7
pub fn i_05c838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C83A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C83A: move.l d5, (a4)
pub fn i_05c83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C83C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C83C: add.l d4, d5
pub fn i_05c83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C83E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C83E: move.w -(a6), d7
pub fn i_05c83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C840;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C840: move.l d7, (a4)
pub fn i_05c840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C842;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C842: add.l d4, d7
pub fn i_05c842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C844;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C844: move.l d5, (a4)
pub fn i_05c844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C846;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C846: add.l d4, d5
pub fn i_05c846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C848;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C848: move.w -(a6), d7
pub fn i_05c848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C84A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C84A: move.l d7, (a4)
pub fn i_05c84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C84C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C84C: add.l d4, d7
pub fn i_05c84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C84E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C84E: move.l d5, (a4)
pub fn i_05c84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C850;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C850: add.l d4, d5
pub fn i_05c850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C852;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C852: move.w -(a6), d7
pub fn i_05c852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C854;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C854: move.l d7, (a4)
pub fn i_05c854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C856;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C856: add.l d4, d7
pub fn i_05c856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C858;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C858: move.l d5, (a4)
pub fn i_05c858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C85A;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C85A: add.l d4, d5
pub fn i_05c85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C85C;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C85C: move.w -(a6), d7
pub fn i_05c85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C85E;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C85E: move.l d7, (a4)
pub fn i_05c85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C860;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C860: add.l d4, d7
pub fn i_05c860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C862;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C862: move.l d5, (a4)
pub fn i_05c862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C864;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C864: add.l d4, d5
pub fn i_05c864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C866;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C866: move.w -(a6), d7
pub fn i_05c866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C868;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C868: move.l d7, (a4)
pub fn i_05c868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C86A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C86A: add.l d4, d7
pub fn i_05c86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C86C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C86C: move.l d5, (a4)
pub fn i_05c86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C86E;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C86E: add.l d4, d5
pub fn i_05c86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C870;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C870: move.w -(a6), d7
pub fn i_05c870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C872;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C872: move.l d7, (a4)
pub fn i_05c872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C874;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C874: add.l d4, d7
pub fn i_05c874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C876;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C876: move.l d5, (a4)
pub fn i_05c876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C878;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C878: add.l d4, d5
pub fn i_05c878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C87A;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C87A: move.w -(a6), d7
pub fn i_05c87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C87C;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C87C: move.l d7, (a4)
pub fn i_05c87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C87E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C87E: add.l d4, d7
pub fn i_05c87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C880;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C880: move.l d5, (a4)
pub fn i_05c880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C882;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C882: add.l d4, d5
pub fn i_05c882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C884;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C884: move.w -(a6), d7
pub fn i_05c884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C886;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C886: move.l d7, (a4)
pub fn i_05c886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C888;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C888: add.l d4, d7
pub fn i_05c888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C88A;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C88A: move.l d5, (a4)
pub fn i_05c88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C88C;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C88C: add.l d4, d5
pub fn i_05c88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C88E;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C88E: move.w -(a6), d7
pub fn i_05c88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C890;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C890: move.l d7, (a4)
pub fn i_05c890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C892;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C892: add.l d4, d7
pub fn i_05c892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C894;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C894: move.l d5, (a4)
pub fn i_05c894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C896;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C896: add.l d4, d5
pub fn i_05c896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C898;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C898: move.w -(a6), d7
pub fn i_05c898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C89A;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C89A: move.l d7, (a4)
pub fn i_05c89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C89C;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C89C: add.l d4, d7
pub fn i_05c89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C89E;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C89E: move.l d5, (a4)
pub fn i_05c89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8A0;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8A0: add.l d4, d5
pub fn i_05c8a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8A2;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C8A2: move.w -(a6), d7
pub fn i_05c8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8A4;
    cpu.ir = 0x3E26;
    let ad2 = cpu.a[6].wrapping_sub(2); cpu.a[6] = ad2;
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    10
}
/// $05C8A4: move.l d7, (a4)
pub fn i_05c8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8A6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8A6: add.l d4, d7
pub fn i_05c8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8A8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8A8: move.l d5, (a4)
pub fn i_05c8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8AA;
    cpu.ir = 0x2885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8AA: add.l d4, d5
pub fn i_05c8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8AC;
    cpu.ir = 0xDA84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[5] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[5] = r;
    4
}
/// $05C8AC: clr.w d7
pub fn i_05c8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8AE;
    cpu.ir = 0x4247;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05C8AE: moveq #$1, d4
pub fn i_05c8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8B0;
    cpu.ir = 0x7801;
    cpu.d[4] = 0x1;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C8B0: swap d4
pub fn i_05c8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8B2;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C8B2: add.w d6, d6
pub fn i_05c8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8B4;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C8B4: jmp $5c8b8(pc, d6.w)
pub fn i_05c8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8B8;
    cpu.ir = 0x4EFB;
    let ad1 = 0x5C8B8_u32.wrapping_add((cpu.d[6] as i16 as i32 as u32));
    cpu.jump_to(ad1);
    8
}
/// $05C8B8: move.l d7, (a4)
pub fn i_05c8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8BA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8BA: add.l d4, d7
pub fn i_05c8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8BC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8BC: move.l d7, (a4)
pub fn i_05c8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8BE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8BE: add.l d4, d7
pub fn i_05c8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8C0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8C0: move.l d7, (a4)
pub fn i_05c8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8C2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8C2: add.l d4, d7
pub fn i_05c8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8C4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8C4: move.l d7, (a4)
pub fn i_05c8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8C6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8C6: add.l d4, d7
pub fn i_05c8c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8C8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8C8: move.l d7, (a4)
pub fn i_05c8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8CA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8CA: add.l d4, d7
pub fn i_05c8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8CC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8CC: move.l d7, (a4)
pub fn i_05c8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8CE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8CE: add.l d4, d7
pub fn i_05c8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8D0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8D0: move.l d7, (a4)
pub fn i_05c8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8D2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8D2: add.l d4, d7
pub fn i_05c8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8D4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8D4: move.l d7, (a4)
pub fn i_05c8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8D6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8D6: add.l d4, d7
pub fn i_05c8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8D8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8D8: move.l d7, (a4)
pub fn i_05c8d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8DA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8DA: add.l d4, d7
pub fn i_05c8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8DC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8DC: move.l d7, (a4)
pub fn i_05c8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8DE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8DE: add.l d4, d7
pub fn i_05c8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8E0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8E0: move.l d7, (a4)
pub fn i_05c8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8E2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8E2: add.l d4, d7
pub fn i_05c8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8E4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8E4: move.l d7, (a4)
pub fn i_05c8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8E6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8E6: add.l d4, d7
pub fn i_05c8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8E8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8E8: move.l d7, (a4)
pub fn i_05c8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8EA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8EA: add.l d4, d7
pub fn i_05c8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8EC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8EC: move.l d7, (a4)
pub fn i_05c8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8EE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8EE: add.l d4, d7
pub fn i_05c8ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8F0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8F0: move.l d7, (a4)
pub fn i_05c8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8F2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8F2: add.l d4, d7
pub fn i_05c8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8F4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8F4: move.l d7, (a4)
pub fn i_05c8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8F6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8F6: add.l d4, d7
pub fn i_05c8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8F8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8F8: move.l d7, (a4)
pub fn i_05c8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8FA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8FA: add.l d4, d7
pub fn i_05c8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8FC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C8FC: move.l d7, (a4)
pub fn i_05c8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C8FE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C8FE: add.l d4, d7
pub fn i_05c8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C900;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C900: move.l d7, (a4)
pub fn i_05c900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C902;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C902: add.l d4, d7
pub fn i_05c902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C904;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C904: move.l d7, (a4)
pub fn i_05c904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C906;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C906: add.l d4, d7
pub fn i_05c906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C908;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C908: move.l d7, (a4)
pub fn i_05c908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C90A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C90A: add.l d4, d7
pub fn i_05c90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C90C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C90C: move.l d7, (a4)
pub fn i_05c90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C90E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C90E: add.l d4, d7
pub fn i_05c90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C910;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C910: move.l d7, (a4)
pub fn i_05c910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C912;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C912: add.l d4, d7
pub fn i_05c912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C914;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C914: move.l d7, (a4)
pub fn i_05c914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C916;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C916: add.l d4, d7
pub fn i_05c916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C918;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C918: move.l d7, (a4)
pub fn i_05c918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C91A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C91A: add.l d4, d7
pub fn i_05c91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C91C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C91C: move.l d7, (a4)
pub fn i_05c91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C91E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C91E: add.l d4, d7
pub fn i_05c91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C920;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C920: move.l d7, (a4)
pub fn i_05c920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C922;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C922: add.l d4, d7
pub fn i_05c922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C924;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C924: move.l d7, (a4)
pub fn i_05c924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C926;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C926: add.l d4, d7
pub fn i_05c926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C928;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C928: move.l d7, (a4)
pub fn i_05c928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C92A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C92A: add.l d4, d7
pub fn i_05c92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C92C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C92C: move.l d7, (a4)
pub fn i_05c92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C92E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C92E: add.l d4, d7
pub fn i_05c92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C930;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C930: move.l d7, (a4)
pub fn i_05c930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C932;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C932: add.l d4, d7
pub fn i_05c932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C934;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C934: move.l d7, (a4)
pub fn i_05c934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C936;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C936: add.l d4, d7
pub fn i_05c936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C938;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C938: move.l d7, (a4)
pub fn i_05c938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C93A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C93A: add.l d4, d7
pub fn i_05c93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C93C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C93C: move.l d7, (a4)
pub fn i_05c93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C93E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C93E: add.l d4, d7
pub fn i_05c93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C940;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C940: move.l d7, (a4)
pub fn i_05c940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C942;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C942: add.l d4, d7
pub fn i_05c942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C944;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C944: move.l d7, (a4)
pub fn i_05c944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C946;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C946: add.l d4, d7
pub fn i_05c946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C948;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C948: move.l d7, (a4)
pub fn i_05c948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C94A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C94A: add.l d4, d7
pub fn i_05c94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C94C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C94C: move.l d7, (a4)
pub fn i_05c94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C94E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C94E: add.l d4, d7
pub fn i_05c94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C950;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C950: move.l d7, (a4)
pub fn i_05c950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C952;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C952: add.l d4, d7
pub fn i_05c952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C954;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C954: move.l d7, (a4)
pub fn i_05c954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C956;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C956: add.l d4, d7
pub fn i_05c956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C958;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C958: move.l d7, (a4)
pub fn i_05c958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C95A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C95A: add.l d4, d7
pub fn i_05c95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C95C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C95C: move.l d7, (a4)
pub fn i_05c95c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C95E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C95E: add.l d4, d7
pub fn i_05c95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C960;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C960: move.l d7, (a4)
pub fn i_05c960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C962;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C962: add.l d4, d7
pub fn i_05c962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C964;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C964: move.l d7, (a4)
pub fn i_05c964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C966;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C966: add.l d4, d7
pub fn i_05c966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C968;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C968: move.l d7, (a4)
pub fn i_05c968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C96A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C96A: add.l d4, d7
pub fn i_05c96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C96C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C96C: move.l d7, (a4)
pub fn i_05c96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C96E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C96E: add.l d4, d7
pub fn i_05c96e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C970;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C970: move.l d7, (a4)
pub fn i_05c970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C972;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C972: add.l d4, d7
pub fn i_05c972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C974;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C974: move.l d7, (a4)
pub fn i_05c974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C976;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C976: add.l d4, d7
pub fn i_05c976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C978;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C978: move.l d7, (a4)
pub fn i_05c978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C97A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C97A: add.l d4, d7
pub fn i_05c97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C97C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C97C: move.l d7, (a4)
pub fn i_05c97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C97E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C97E: add.l d4, d7
pub fn i_05c97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C980;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C980: move.l d7, (a4)
pub fn i_05c980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C982;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C982: add.l d4, d7
pub fn i_05c982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C984;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C984: move.l d7, (a4)
pub fn i_05c984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C986;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C986: add.l d4, d7
pub fn i_05c986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C988;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C988: move.l d7, (a4)
pub fn i_05c988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C98A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C98A: add.l d4, d7
pub fn i_05c98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C98C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C98C: move.l d7, (a4)
pub fn i_05c98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C98E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C98E: add.l d4, d7
pub fn i_05c98e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C990;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C990: move.l d7, (a4)
pub fn i_05c990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C992;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C992: add.l d4, d7
pub fn i_05c992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C994;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C994: move.l d7, (a4)
pub fn i_05c994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C996;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C996: add.l d4, d7
pub fn i_05c996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C998;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C998: move.l d7, (a4)
pub fn i_05c998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C99A;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C99A: add.l d4, d7
pub fn i_05c99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C99C;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C99C: move.l d7, (a4)
pub fn i_05c99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C99E;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C99E: add.l d4, d7
pub fn i_05c99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9A0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C9A0: move.l d7, (a4)
pub fn i_05c9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9A2;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C9A2: add.l d4, d7
pub fn i_05c9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9A4;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C9A4: move.l d7, (a4)
pub fn i_05c9a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9A6;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C9A6: add.l d4, d7
pub fn i_05c9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9A8;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C9A8: move.l d7, (a4)
pub fn i_05c9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9AA;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C9AA: add.l d4, d7
pub fn i_05c9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9AC;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C9AC: move.l d7, (a4)
pub fn i_05c9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9AE;
    cpu.ir = 0x2887;
    let v1 = cpu.d[7] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C9AE: add.l d4, d7
pub fn i_05c9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9B0;
    cpu.ir = 0xDE84;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    let h1 = cpu.d[7] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[7] = r;
    4
}
/// $05C9B0: move.w #$8401, d6
pub fn i_05c9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9B4;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x8401;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05C9B4: add.w d0, d6
pub fn i_05c9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9B6;
    cpu.ir = 0xDC40;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05C9B6: swap d6
pub fn i_05c9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9B8;
    cpu.ir = 0x4846;
    cpu.d[6] = cpu.d[6].rotate_left(16);
    cpu.sr.set_nz(cpu.d[6], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C9B8: move.w $a(a3), d6
pub fn i_05c9b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9BC;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C9BC: move.l #$2000000, d5
pub fn i_05c9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9C2;
    cpu.ir = 0x2A3C;
    let v1: u32 = 0x2000000;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05C9C2: move.l d6, (a4)
pub fn i_05c9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9C4;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C9C4: sub.l d5, d6
pub fn i_05c9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9C6;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C9C6: move.w $8(a3), d6
pub fn i_05c9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9CA;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C9CA: move.l d6, (a4)
pub fn i_05c9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9CC;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C9CC: sub.l d5, d6
pub fn i_05c9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9CE;
    cpu.ir = 0x9C85;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let h1 = cpu.d[6] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[6] = r;
    4
}
/// $05C9CE: move.w $6(a3), d6
pub fn i_05c9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9D2;
    cpu.ir = 0x3C2B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C9D2: move.l d6, (a4)
pub fn i_05c9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9D4;
    cpu.ir = 0x2886;
    let v1 = cpu.d[6] & 0xFFFFFFFF;
    bus.write32(cpu.a[4], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05C9D4: rts
pub fn i_05c9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Vblank_FlushSpriteQueue_05c9d6  ($05C9D6, 84 B, video/sprite_queue_render_05aa96.s)
/// $05C9D6: tst.b $10e1ec.l
pub fn i_05c9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9DC;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E1EC) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $05C9DC: beq.w $5ca28
pub fn i_05c9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9E0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CA28; return 10; }
    8
}
/// $05C9E0: clr.b $10e1ec.l
pub fn i_05c9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9E6;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E1EC) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E1EC, r2 as u8);
    16
}
/// $05C9E6: lea.l $3c0000.l, a4
pub fn i_05c9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9EC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x3C0000;
    20
}
/// $05C9EC: lea.l $108080.l, a5
pub fn i_05c9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9F2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x108080;
    20
}
/// $05C9F2: lea.l $614c(a5), a0
pub fn i_05c9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9F6;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x614C);
    cpu.a[0] = ad1;
    16
}
/// $05C9F6: lea.l $542a(a5), a1
pub fn i_05c9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9FA;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05C9FA: move.w $6148(a5), d7
pub fn i_05c9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5C9FE;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x6148);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05C9FE: movea.l a1, a2
pub fn i_05c9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA00;
    cpu.ir = 0x2449;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[2] = v1;
    4
}
/// $05CA00: adda.w d7, a2
pub fn i_05ca00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA02;
    cpu.ir = 0xD4C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[2] = cpu.a[2].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05CA02: bsr.w $5b400
pub fn i_05ca02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA06;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5CA06);
    cpu.pc = 0x5B400;
    18
}
/// $05CA06: move.w $a(a0), $6168(a5)
pub fn i_05ca06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA0C;
    cpu.ir = 0x3B68;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x6168);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05CA0C: lea.l $6158(a5), a0
pub fn i_05ca0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA10;
    cpu.ir = 0x41ED;
    let ad1 = cpu.a[5].wrapping_add(0x6158);
    cpu.a[0] = ad1;
    16
}
/// $05CA10: lea.l $542a(a5), a1
pub fn i_05ca10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA14;
    cpu.ir = 0x43ED;
    let ad1 = cpu.a[5].wrapping_add(0x542A);
    cpu.a[1] = ad1;
    16
}
/// $05CA14: lea.l $348(a1), a2
pub fn i_05ca14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA18;
    cpu.ir = 0x45E9;
    let ad1 = cpu.a[1].wrapping_add(0x348);
    cpu.a[2] = ad1;
    16
}
/// $05CA18: move.w $614a(a5), d7
pub fn i_05ca18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA1C;
    cpu.ir = 0x3E2D;
    let ad2 = cpu.a[5].wrapping_add(0x614A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05CA1C: adda.w d7, a1
pub fn i_05ca1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA1E;
    cpu.ir = 0xD2C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05CA1E: bsr.w $5b400
pub fn i_05ca1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA22;
    cpu.ir = 0x6100;
    cpu.push32(bus, 0x5CA22);
    cpu.pc = 0x5B400;
    18
}
/// $05CA22: move.w $a(a0), $616a(a5)
pub fn i_05ca22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA28;
    cpu.ir = 0x3B68;
    let ad2 = cpu.a[0].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[5].wrapping_add(0x616A);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05CA28: rts
pub fn i_05ca28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_Dispatch_05CA2A  ($05CA2A, 164 B, video/sprite_setup_dispatch_05ca2a.s)
/// $05CA2A: move.b $3a(a6), d5
pub fn i_05ca2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA2E;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA2E: andi.b #$3, d5
pub fn i_05ca2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA32;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CA32: move.l $22(a6), d0
pub fn i_05ca32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA36;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05CA36: move.w $38(a6), d1
pub fn i_05ca36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA3A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05CA3A: move.w $14(a6), d2
pub fn i_05ca3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA3E;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05CA3E: move.b $33(a6), d3
pub fn i_05ca3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA42;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA42: move.b $32(a6), d4
pub fn i_05ca42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA46;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA46: movea.l $3c(a6), a0
pub fn i_05ca46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA4A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05CA4A: bra.w $5a9e6
pub fn i_05ca4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A9E6;
    10
}
/// $05CA4E: move.b $3a(a6), d5
pub fn i_05ca4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA52;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA52: andi.b #$3, d5
pub fn i_05ca52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA56;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CA56: ori.b #$4, d5
pub fn i_05ca56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA5A;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CA5A: lea.l $5a9d6(pc), a1
pub fn i_05ca5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5A9D6;
    16
}
/// $05CA5E: bra.b $5ca32
pub fn i_05ca5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA60;
    cpu.ir = 0x60D2;
    cpu.pc = 0x5CA32;
    10
}
/// $05CA60: move.b $3a(a6), d5
pub fn i_05ca60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA64;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA64: andi.b #$3, d5
pub fn i_05ca64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA68;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CA68: move.l $22(a6), d0
pub fn i_05ca68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA6C;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05CA6C: move.b $33(a6), d3
pub fn i_05ca6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA70;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA70: move.b $32(a6), d4
pub fn i_05ca70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA74;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CA74: btst.b #$2, $12(a6)
pub fn i_05ca74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA7A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05CA7A: beq.w $5cab0
pub fn i_05ca7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CAB0; return 10; }
    8
}
/// $05CA7E: move.l d0, d1
pub fn i_05ca7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA80;
    cpu.ir = 0x2200;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05CA80: swap d0
pub fn i_05ca80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA82;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05CA82: sub.w $10e1e4.l, d0
pub fn i_05ca82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA88;
    cpu.ir = 0x9079;
    let v2 = bus.read16(0x10E1E4) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05CA88: move.w d4, d6
pub fn i_05ca88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA8A;
    cpu.ir = 0x3C04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CA8A: addq.w #$1, d6
pub fn i_05ca8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA8C;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05CA8C: muls.w d6, d0
pub fn i_05ca8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA8E;
    cpu.ir = 0xC1C6;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05CA8E: asr.l #$8, d0
pub fn i_05ca8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA90;
    cpu.ir = 0xE080;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[0] = r;
    22
}
/// $05CA90: add.w $10e1e4.l, d0
pub fn i_05ca90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA96;
    cpu.ir = 0xD079;
    let v2 = bus.read16(0x10E1E4) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05CA96: sub.w $10e1e6.l, d1
pub fn i_05ca96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA9C;
    cpu.ir = 0x9279;
    let v2 = bus.read16(0x10E1E6) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05CA9C: move.w d3, d6
pub fn i_05ca9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CA9E;
    cpu.ir = 0x3C03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CA9E: addq.w #$1, d6
pub fn i_05ca9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAA0;
    cpu.ir = 0x5246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05CAA0: muls.w d6, d1
pub fn i_05caa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAA2;
    cpu.ir = 0xC3C6;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05CAA2: asr.l #$8, d1
pub fn i_05caa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAA4;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $05CAA4: add.w $10e1e6.l, d1
pub fn i_05caa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAAA;
    cpu.ir = 0xD279;
    let v2 = bus.read16(0x10E1E6) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $05CAAA: swap d0
pub fn i_05caaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAAC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05CAAC: clr.w d0
pub fn i_05caac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAAE;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05CAAE: move.w d1, d0
pub fn i_05caae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAB0;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CAB0: move.w $38(a6), d1
pub fn i_05cab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAB4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05CAB4: move.w $14(a6), d2
pub fn i_05cab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAB8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05CAB8: movea.l $3c(a6), a0
pub fn i_05cab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CABC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05CABC: bra.w $5a9e6
pub fn i_05cabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5A9E6;
    10
}
/// $05CAC0: move.b $3a(a6), d5
pub fn i_05cac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAC4;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CAC4: andi.b #$3, d5
pub fn i_05cac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAC8;
    cpu.ir = 0x205;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CAC8: ori.b #$4, d5
pub fn i_05cac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CACC;
    cpu.ir = 0x5;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CACC: bra.b $5ca68
pub fn i_05cacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CACE;
    cpu.ir = 0x609A;
    cpu.pc = 0x5CA68;
    10
}
// ===== Camera_ResetCenter_05CACE  ($05CACE, 18 B, video/camera_reset_center_05cace.s)
/// $05CACE: move.w #$a0, $10e1e4.l
pub fn i_05cace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAD6;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1E4, v1 as u16);
    16
}
/// $05CAD6: move.w #$178, $10e1e6.l
pub fn i_05cad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CADE;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E1E6, v1 as u16);
    16
}
/// $05CADE: rts
pub fn i_05cade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scheduler_CompareField10_05cae0  ($05CAE0, 16 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CAE0: movea.l $8(a6), a1
pub fn i_05cae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAE4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05CAE4: move.b $10(a6), d0
pub fn i_05cae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAE8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CAE8: cmp.b $10(a1), d0
pub fn i_05cae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAEC;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05CAEC: bcs.w $5caf6
pub fn i_05caec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAF0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5CAF6; return 10; }
    8
}
// ===== ClearXN_05caf0  ($05CAF0, 6 B, thunks/ccr_helpers.c)
/// $05CAF0: andi.b #$ee, ccr
pub fn i_05caf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAF4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05CAF4: rts
pub fn i_05caf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05caf6  ($05CAF6, 6 B, thunks/ccr_helpers.c)
/// $05CAF6: ori.b #$11, ccr
pub fn i_05caf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAFA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05CAFA: rts
pub fn i_05cafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CAFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SpriteBlock20x14_Setup_05cafc  ($05CAFC, 108 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CAFC: move.w #$14, d0
pub fn i_05cafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB00;
    cpu.ir = 0x303C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB00: jsr $13952.l
pub fn i_05cb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5CB06);
    cpu.pc = 0x13952;
    16
}
/// $05CB06: moveq #$0, d2
pub fn i_05cb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB08;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05CB08: move.w #$e, d3
pub fn i_05cb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB0C;
    cpu.ir = 0x363C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB0C: move.w #$e, d4
pub fn i_05cb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB10;
    cpu.ir = 0x383C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB10: swap d4
pub fn i_05cb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB12;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05CB12: move.w #$14, d4
pub fn i_05cb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB16;
    cpu.ir = 0x383C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB16: move.w d0, d5
pub fn i_05cb16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB18;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB18: move.w d0, d6
pub fn i_05cb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB1A;
    cpu.ir = 0x3C00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB1A: addi.w #$13, d6
pub fn i_05cb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB1E;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x13, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05CB1E: move.w #$0, d1
pub fn i_05cb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB22;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB22: move.b #$0, d7
pub fn i_05cb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB26;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CB26: movem.w d0, -(a7)
pub fn i_05cb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB2A;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $05CB2A: jsr $139fe.l
pub fn i_05cb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5CB30);
    cpu.pc = 0x139FE;
    16
}
/// $05CB30: movem.w (a7)+, d0
pub fn i_05cb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB34;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $05CB34: addi.w #$8201, d0
pub fn i_05cb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB38;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8201, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05CB38: move.w d0, $106ee4.l
pub fn i_05cb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB3E;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $05CB3E: move.w d0, $3c0000.l
pub fn i_05cb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB44;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $05CB44: move.w #$f78e, $3c0002.l
pub fn i_05cb44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB4C;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xF78E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $05CB4C: addi.w #$200, d0
pub fn i_05cb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB50;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05CB50: move.w d0, $106ee4.l
pub fn i_05cb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB56;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $05CB56: move.w d0, $3c0000.l
pub fn i_05cb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB5C;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $05CB5C: move.w #$0, $3c0002.l
pub fn i_05cb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB64;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $05CB64: jmp $5cbd8(pc)
pub fn i_05cb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB68;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x5CBD8;
    8
}
// ===== SpriteBlock20x14_SetupDup_05cb68  ($05CB68, 124 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CB68: move.w #$14, d0
pub fn i_05cb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB6C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB6C: jsr $13952.l
pub fn i_05cb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5CB72);
    cpu.pc = 0x13952;
    16
}
/// $05CB72: moveq #$0, d2
pub fn i_05cb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB74;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05CB74: move.w #$e, d3
pub fn i_05cb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB78;
    cpu.ir = 0x363C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB78: move.w #$e, d4
pub fn i_05cb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB7C;
    cpu.ir = 0x383C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB7C: swap d4
pub fn i_05cb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB7E;
    cpu.ir = 0x4844;
    cpu.d[4] = cpu.d[4].rotate_left(16);
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05CB7E: move.w #$14, d4
pub fn i_05cb7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB82;
    cpu.ir = 0x383C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB82: move.w d0, d5
pub fn i_05cb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB84;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB84: move.w d0, d6
pub fn i_05cb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB86;
    cpu.ir = 0x3C00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB86: addi.w #$13, d6
pub fn i_05cb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB8A;
    cpu.ir = 0x646;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x13, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05CB8A: move.w #$0, d1
pub fn i_05cb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB8E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CB8E: move.b #$0, d7
pub fn i_05cb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB92;
    cpu.ir = 0x1E3C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CB92: movem.w d0, -(a7)
pub fn i_05cb92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB96;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[0] as u16);
    cpu.a[7] = a;
    20
}
/// $05CB96: jsr $139fe.l
pub fn i_05cb96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CB9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5CB9C);
    cpu.pc = 0x139FE;
    16
}
/// $05CB9C: movem.w (a7)+, d0
pub fn i_05cb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBA0;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $05CBA0: addi.w #$8201, d0
pub fn i_05cba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBA4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x8201, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05CBA4: move.w d0, $106ee4.l
pub fn i_05cba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBAA;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $05CBAA: move.w d0, $3c0000.l
pub fn i_05cbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBB0;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $05CBB0: move.w #$f78e, $3c0002.l
pub fn i_05cbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBB8;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xF78E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $05CBB8: move.w #$f78e, $3c0002.l
pub fn i_05cbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBC0;
    cpu.ir = 0x33FC;
    let v1: u32 = 0xF78E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $05CBC0: addi.w #$200, d0
pub fn i_05cbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBC4;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x200, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05CBC4: move.w d0, $106ee4.l
pub fn i_05cbc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBCA;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106EE4, v1 as u16);
    16
}
/// $05CBCA: move.w d0, $3c0000.l
pub fn i_05cbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBD0;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0000, v1 as u16);
    16
}
/// $05CBD0: move.w #$0, $3c0002.l
pub fn i_05cbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBD8;
    cpu.ir = 0x33FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x3C0002, v1 as u16);
    16
}
/// $05CBD8: lea.l $2b7460.l, a0
pub fn i_05cbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBDE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B7460;
    20
}
/// $05CBDE: jsr $2a7c.l
pub fn i_05cbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5CBE4);
    cpu.pc = 0x2A7C;
    16
}
// ===== SetTaskHandler_05cbe4  ($05CBE4, 8 B, kernel/task_handlers.c)
/// $05CBE4: lea.l $5cbea(pc), a1
pub fn i_05cbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBE8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5CBEA;
    16
}
/// $05CBE8: move.l a1, (a6)
pub fn i_05cbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBEA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05CBEA: rts
pub fn i_05cbea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBEC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Scheduler_CompareField10_05cbec  ($05CBEC, 16 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CBEC: movea.l $8(a6), a1
pub fn i_05cbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBF0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05CBF0: move.b $10(a6), d0
pub fn i_05cbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBF4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05CBF4: cmp.b $10(a1), d0
pub fn i_05cbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBF8;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05CBF8: bcs.w $5cc02
pub fn i_05cbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CBFC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5CC02; return 10; }
    8
}
// ===== ClearXN_05cbfc  ($05CBFC, 6 B, thunks/ccr_helpers.c)
/// $05CBFC: andi.b #$ee, ccr
pub fn i_05cbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC00;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05CC00: rts
pub fn i_05cc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05cc02  ($05CC02, 6 B, thunks/ccr_helpers.c)
/// $05CC02: ori.b #$11, ccr
pub fn i_05cc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC06;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05CC06: rts
pub fn i_05cc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_RisingEdgeSnapshot_05CC0E  ($05CC0E, 186 B, input/rising_edge_snapshot_05cc0e.s)
/// $05CC0E: clr.w $10e20c.l
pub fn i_05cc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC14;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E20C) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E20C, r2 as u16);
    16
}
/// $05CC14: jmp $24fb8.l
pub fn i_05cc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x24FB8;
    8
}
/// $05CC1A: jsr $25066.l
pub fn i_05cc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5CC20);
    cpu.pc = 0x25066;
    16
}
/// $05CC20: lea.l $106eb0.l, a0
pub fn i_05cc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB0;
    20
}
/// $05CC26: lea.l $10e200.l, a1
pub fn i_05cc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC2C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E200;
    20
}
/// $05CC2C: move.l (a0)+, (a1)+
pub fn i_05cc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC2E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05CC2E: move.w (a0), (a1)
pub fn i_05cc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC30;
    cpu.ir = 0x3290;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v2 as u16);
    12
}
/// $05CC30: lea.l $106eb6.l, a0
pub fn i_05cc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC36;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x106EB6;
    20
}
/// $05CC36: lea.l $10e206.l, a1
pub fn i_05cc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC3C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E206;
    20
}
/// $05CC3C: move.l (a0)+, (a1)+
pub fn i_05cc3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC3E;
    cpu.ir = 0x22D8;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05CC3E: move.w (a0), (a1)
pub fn i_05cc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC40;
    cpu.ir = 0x3290;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[1], v2 as u16);
    12
}
/// $05CC40: lea.l $10fdac.l, a0
pub fn i_05cc40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC46;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDAC;
    20
}
/// $05CC46: lea.l $10e20c.l, a1
pub fn i_05cc46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC4C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E20C;
    20
}
/// $05CC4C: move.b (a1), d1
pub fn i_05cc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC4E;
    cpu.ir = 0x1211;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05CC4E: move.b (a0), d0
pub fn i_05cc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC50;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $05CC50: move.b d0, (a1)
pub fn i_05cc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC52;
    cpu.ir = 0x1280;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(cpu.a[1], v1 as u8);
    8
}
/// $05CC52: eor.b d0, d1
pub fn i_05cc52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC54;
    cpu.ir = 0xB101;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05CC54: and.b d0, d1
pub fn i_05cc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC56;
    cpu.ir = 0xC200;
    let v1 = cpu.d[0] & 0xFF;
    let r = (v1 & cpu.d[1]) & 0xFF;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05CC56: move.b d1, $1(a1)
pub fn i_05cc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC5A;
    cpu.ir = 0x1341;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05CC5A: lea.l $10fdb6.l, a0
pub fn i_05cc5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC60;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $05CC60: cmpi.b #$0, (a0)
pub fn i_05cc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC64;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    12
}
/// $05CC64: beq.w $5cc8e
pub fn i_05cc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CC8E; return 10; }
    8
}
/// $05CC68: cmpi.b #$3, (a0)
pub fn i_05cc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC6C;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    12
}
/// $05CC6C: beq.w $5cc84
pub fn i_05cc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC70;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CC84; return 10; }
    8
}
/// $05CC70: lea.l $10e200.l, a2
pub fn i_05cc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC76;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CC76: lea.l $10e212.l, a1
pub fn i_05cc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC7C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E212;
    20
}
/// $05CC7C: move.l (a2)+, (a1)+
pub fn i_05cc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC7E;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05CC7E: move.w (a2)+, (a1)+
pub fn i_05cc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC80;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $05CC80: bra.w $5cc8e
pub fn i_05cc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC84;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CC8E;
    10
}
/// $05CC84: lea.l $10e212.l, a1
pub fn i_05cc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC8A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E212;
    20
}
/// $05CC8A: clr.l (a1)+
pub fn i_05cc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC8C;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $05CC8C: clr.w (a1)+
pub fn i_05cc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC8E;
    cpu.ir = 0x4259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $05CC8E: lea.l $10fdb6.l, a0
pub fn i_05cc8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10FDB6;
    20
}
/// $05CC94: cmpi.b #$0, $1(a0)
pub fn i_05cc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC9A;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $05CC9A: beq.w $5ccc6
pub fn i_05cc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CC9E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CCC6; return 10; }
    8
}
/// $05CC9E: cmpi.b #$3, $1(a0)
pub fn i_05cc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCA4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $05CCA4: beq.w $5ccbc
pub fn i_05cca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CCBC; return 10; }
    8
}
/// $05CCA8: lea.l $10e206.l, a2
pub fn i_05cca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCAE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CCAE: lea.l $10e218.l, a1
pub fn i_05ccae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCB4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E218;
    20
}
/// $05CCB4: move.l (a2)+, (a1)+
pub fn i_05ccb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCB6;
    cpu.ir = 0x22DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(4);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $05CCB6: move.w (a2)+, (a1)+
pub fn i_05ccb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCB8;
    cpu.ir = 0x32DA;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[1]; cpu.a[1] = ad4.wrapping_add(2);
    bus.write16(ad4, v3 as u16);
    12
}
/// $05CCB8: bra.w $5ccc6
pub fn i_05ccb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CCC6;
    10
}
/// $05CCBC: lea.l $10e218.l, a1
pub fn i_05ccbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCC2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x10E218;
    20
}
/// $05CCC2: clr.l (a1)+
pub fn i_05ccc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCC4;
    cpu.ir = 0x4299;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(4);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    12
}
/// $05CCC4: clr.w (a1)+
pub fn i_05ccc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCC6;
    cpu.ir = 0x4259;
    let ad1 = cpu.a[1]; cpu.a[1] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    8
}
/// $05CCC6: rts
pub fn i_05ccc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyField6D_05ccc8  ($05CCC8, 8 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CCC8: move.b $6d(a6), $6d(a0)
pub fn i_05ccc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCCE;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x6D);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05CCCE: rts
pub fn i_05ccce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvtThunk_05ccd0  ($05CCD0, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CCD0: move.b #$10, d1
pub fn i_05ccd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCD4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CCD4: move.w #$2, d0
pub fn i_05ccd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCD8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CCD8: lea.l $10e200.l, a2
pub fn i_05ccd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCDE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CCDE: bra.w $5cff8
pub fn i_05ccde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCE2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cce2  ($05CCE2, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CCE2: move.b #$20, d1
pub fn i_05cce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCE6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CCE6: move.w #$2, d0
pub fn i_05cce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCEA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CCEA: lea.l $10e200.l, a2
pub fn i_05ccea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCF0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CCF0: bra.w $5cff8
pub fn i_05ccf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ccf4  ($05CCF4, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CCF4: move.b #$40, d1
pub fn i_05ccf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCF8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CCF8: move.w #$2, d0
pub fn i_05ccf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CCFC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CCFC: lea.l $10e200.l, a2
pub fn i_05ccfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD02;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CD02: bra.w $5cff8
pub fn i_05cd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD06;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd06  ($05CD06, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD06: move.b #$80, d1
pub fn i_05cd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD0A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD0A: move.w #$2, d0
pub fn i_05cd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD0E: lea.l $10e200.l, a2
pub fn i_05cd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD14;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CD14: bra.w $5cff8
pub fn i_05cd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD18;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd18  ($05CD18, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD18: move.b #$10, d1
pub fn i_05cd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD1C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD1C: move.w #$2, d0
pub fn i_05cd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD20: lea.l $10e206.l, a2
pub fn i_05cd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD26;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CD26: bra.w $5cff8
pub fn i_05cd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD2A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd2a  ($05CD2A, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD2A: move.b #$20, d1
pub fn i_05cd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD2E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD2E: move.w #$2, d0
pub fn i_05cd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD32;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD32: lea.l $10e206.l, a2
pub fn i_05cd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD38;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CD38: bra.w $5cff8
pub fn i_05cd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd3c  ($05CD3C, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD3C: move.b #$40, d1
pub fn i_05cd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD40;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD40: move.w #$2, d0
pub fn i_05cd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD44;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD44: lea.l $10e206.l, a2
pub fn i_05cd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD4A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CD4A: bra.w $5cff8
pub fn i_05cd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd4e  ($05CD4E, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD4E: move.b #$80, d1
pub fn i_05cd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD52;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD52: move.w #$2, d0
pub fn i_05cd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD56;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD56: lea.l $10e206.l, a2
pub fn i_05cd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD5C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CD5C: bra.w $5cff8
pub fn i_05cd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD60;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cd60  ($05CD60, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD60: move.b #$10, d1
pub fn i_05cd60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD64;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD64: move.w #$2, d0
pub fn i_05cd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD68: bra.w $5cfa8
pub fn i_05cd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cd6c  ($05CD6C, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD6C: move.b #$20, d1
pub fn i_05cd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD70;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD70: move.w #$2, d0
pub fn i_05cd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD74;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD74: bra.w $5cfa8
pub fn i_05cd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD78;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cd78  ($05CD78, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD78: move.b #$40, d1
pub fn i_05cd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD7C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD7C: move.w #$2, d0
pub fn i_05cd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD80;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD80: bra.w $5cfa8
pub fn i_05cd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD84;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cd84  ($05CD84, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD84: move.b #$80, d1
pub fn i_05cd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD88;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD88: move.w #$2, d0
pub fn i_05cd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD8C: bra.w $5cfa8
pub fn i_05cd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD90;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cd90  ($05CD90, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD90: move.b #$f0, d1
pub fn i_05cd90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD94;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CD94: move.w #$2, d0
pub fn i_05cd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CD98: bra.w $5cfa8
pub fn i_05cd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CD9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cd9c  ($05CD9C, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CD9C: move.b #$e0, d1
pub fn i_05cd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDA0;
    cpu.ir = 0x123C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDA0: move.w #$2, d0
pub fn i_05cda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDA4: bra.w $5cfa8
pub fn i_05cda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cda8  ($05CDA8, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDA8: move.b #$10, d1
pub fn i_05cda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDAC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDAC: move.w #$3, d0
pub fn i_05cdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDB0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDB0: bra.w $5cfa8
pub fn i_05cdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDB4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdb4  ($05CDB4, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDB4: move.b #$20, d1
pub fn i_05cdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDB8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDB8: move.w #$3, d0
pub fn i_05cdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDBC: bra.w $5cfa8
pub fn i_05cdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdc0  ($05CDC0, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDC0: move.b #$40, d1
pub fn i_05cdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDC4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDC4: move.w #$3, d0
pub fn i_05cdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDC8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDC8: bra.w $5cfa8
pub fn i_05cdc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDCC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdcc  ($05CDCC, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDCC: move.b #$80, d1
pub fn i_05cdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDD0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDD0: move.w #$3, d0
pub fn i_05cdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDD4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDD4: bra.w $5cfa8
pub fn i_05cdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdd8  ($05CDD8, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDD8: move.b #$30, d1
pub fn i_05cdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDDC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDDC: move.w #$3, d0
pub fn i_05cddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDE0: bra.w $5cfa8
pub fn i_05cde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cde4  ($05CDE4, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDE4: move.b #$50, d1
pub fn i_05cde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDE8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDE8: move.w #$3, d0
pub fn i_05cde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDEC: bra.w $5cfa8
pub fn i_05cdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdf0  ($05CDF0, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05CDF0: move.b #$70, d1
pub fn i_05cdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDF4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CDF4: move.w #$3, d0
pub fn i_05cdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDF8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CDF8: bra.w $5cfa8
pub fn i_05cdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CDFC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cdfc  ($05CDFC, 12 B, input/event_thunks_05cdfc.s)
/// $05CDFC: move.b #$f0, d1
pub fn i_05cdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE00;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE00: move.w #$3, d0
pub fn i_05ce00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE04;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE04: bra.w $5cfa8
pub fn i_05ce04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE08;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05ce08  ($05CE08, 12 B, input/event_thunks_05cdfc.s)
/// $05CE08: move.b #$e0, d1
pub fn i_05ce08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE0C;
    cpu.ir = 0x123C;
    let v1: u32 = 0xE0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE0C: move.w #$3, d0
pub fn i_05ce0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE10;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE10: bra.w $5cfa8
pub fn i_05ce10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE14;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05ce14  ($05CE14, 18 B, input/event_thunks_05cdfc.s)
/// $05CE14: move.b #$10, d1
pub fn i_05ce14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE18;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE18: move.w #$3, d0
pub fn i_05ce18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE1C: lea.l $10e200.l, a2
pub fn i_05ce1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE22;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CE22: bra.w $5cff8
pub fn i_05ce22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE26;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce26  ($05CE26, 18 B, input/event_thunks_05cdfc.s)
/// $05CE26: move.b #$20, d1
pub fn i_05ce26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE2A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE2A: move.w #$3, d0
pub fn i_05ce2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE2E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE2E: lea.l $10e200.l, a2
pub fn i_05ce2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE34;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CE34: bra.w $5cff8
pub fn i_05ce34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE38;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce38  ($05CE38, 18 B, input/event_thunks_05cdfc.s)
/// $05CE38: move.b #$40, d1
pub fn i_05ce38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE3C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE3C: move.w #$3, d0
pub fn i_05ce3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE40;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE40: lea.l $10e200.l, a2
pub fn i_05ce40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE46;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CE46: bra.w $5cff8
pub fn i_05ce46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE4A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce4a  ($05CE4A, 18 B, input/event_thunks_05cdfc.s)
/// $05CE4A: move.b #$80, d1
pub fn i_05ce4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE4E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE4E: move.w #$3, d0
pub fn i_05ce4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE52: lea.l $10e200.l, a2
pub fn i_05ce52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE58;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CE58: bra.w $5cff8
pub fn i_05ce58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce5c  ($05CE5C, 18 B, input/event_thunks_05cdfc.s)
/// $05CE5C: move.b #$10, d1
pub fn i_05ce5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE60;
    cpu.ir = 0x123C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE60: move.w #$3, d0
pub fn i_05ce60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE64;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE64: lea.l $10e206.l, a2
pub fn i_05ce64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE6A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CE6A: bra.w $5cff8
pub fn i_05ce6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce6e  ($05CE6E, 18 B, input/event_thunks_05cdfc.s)
/// $05CE6E: move.b #$20, d1
pub fn i_05ce6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE72;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE72: move.w #$3, d0
pub fn i_05ce72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE76;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE76: lea.l $10e206.l, a2
pub fn i_05ce76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE7C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CE7C: bra.w $5cff8
pub fn i_05ce7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE80;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce80  ($05CE80, 18 B, input/event_thunks_05cdfc.s)
/// $05CE80: move.b #$40, d1
pub fn i_05ce80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE84;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE84: move.w #$3, d0
pub fn i_05ce84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE88;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE88: lea.l $10e206.l, a2
pub fn i_05ce88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE8E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CE8E: bra.w $5cff8
pub fn i_05ce8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE92;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ce92  ($05CE92, 18 B, input/event_thunks_05cdfc.s)
/// $05CE92: move.b #$80, d1
pub fn i_05ce92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE96;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CE96: move.w #$3, d0
pub fn i_05ce96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CE9A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CE9A: lea.l $10e206.l, a2
pub fn i_05ce9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEA0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05CEA0: bra.w $5cff8
pub fn i_05cea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cea4  ($05CEA4, 18 B, input/event_thunks_05cdfc.s)
/// $05CEA4: move.b #$1, d1
pub fn i_05cea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEA8;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEA8: move.w #$2, d0
pub fn i_05cea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEAC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CEAC: lea.l $10e200.l, a2
pub fn i_05ceac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEB2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CEB2: bra.w $5cff8
pub fn i_05ceb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ceb6  ($05CEB6, 18 B, input/event_thunks_05cdfc.s)
/// $05CEB6: move.b #$2, d1
pub fn i_05ceb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEBA;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEBA: move.w #$2, d0
pub fn i_05ceba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEBE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CEBE: lea.l $10e200.l, a2
pub fn i_05cebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEC4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CEC4: bra.w $5cff8
pub fn i_05cec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEC8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05cec8  ($05CEC8, 18 B, input/event_thunks_05cdfc.s)
/// $05CEC8: move.b #$8, d1
pub fn i_05cec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CECC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CECC: move.w #$2, d0
pub fn i_05cecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CED0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CED0: lea.l $10e200.l, a2
pub fn i_05ced0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CED6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CED6: bra.w $5cff8
pub fn i_05ced6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEDA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ceda  ($05CEDA, 18 B, input/event_thunks_05cdfc.s)
/// $05CEDA: move.b #$4, d1
pub fn i_05ceda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEDE;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEDE: move.w #$2, d0
pub fn i_05cede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEE2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CEE2: lea.l $10e200.l, a2
pub fn i_05cee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEE8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05CEE8: bra.w $5cff8
pub fn i_05cee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEEC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
// ===== InputEvtThunk_05ceec  ($05CEEC, 12 B, input/event_thunks_05cdfc.s)
/// $05CEEC: move.b #$1, d1
pub fn i_05ceec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEF0;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEF0: move.w #$2, d0
pub fn i_05cef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEF4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CEF4: bra.w $5cfa8
pub fn i_05cef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEF8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cef8  ($05CEF8, 12 B, input/event_thunks_05cdfc.s)
/// $05CEF8: move.b #$2, d1
pub fn i_05cef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CEFC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CEFC: move.w #$2, d0
pub fn i_05cefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF00;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF00: bra.w $5cfa8
pub fn i_05cf00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF04;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf04  ($05CF04, 12 B, input/event_thunks_05cdfc.s)
/// $05CF04: move.b #$8, d1
pub fn i_05cf04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF08;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF08: move.w #$2, d0
pub fn i_05cf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF0C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF0C: bra.w $5cfa8
pub fn i_05cf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF10;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf10  ($05CF10, 12 B, input/event_thunks_05cdfc.s)
/// $05CF10: move.b #$4, d1
pub fn i_05cf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF14;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF14: move.w #$2, d0
pub fn i_05cf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF18;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF18: bra.w $5cfa8
pub fn i_05cf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF1C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf1c  ($05CF1C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF1C: move.b #$4, d1
pub fn i_05cf1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF20;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF20: ori.b #$8, d1
pub fn i_05cf20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF24;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF24: move.w #$2, d0
pub fn i_05cf24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF28;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF28: bra.w $5cfa8
pub fn i_05cf28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF2C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf2c  ($05CF2C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF2C: move.b #$1, d1
pub fn i_05cf2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF30;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF30: ori.b #$8, d1
pub fn i_05cf30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF34;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF34: move.w #$2, d0
pub fn i_05cf34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF38;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF38: bra.w $5cfa8
pub fn i_05cf38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF3C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf3c  ($05CF3C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF3C: move.b #$2, d1
pub fn i_05cf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF40;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF40: ori.b #$8, d1
pub fn i_05cf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF44;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x8) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF44: move.w #$2, d0
pub fn i_05cf44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF48;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF48: bra.w $5cfa8
pub fn i_05cf48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF4C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf4c  ($05CF4C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF4C: move.b #$1, d1
pub fn i_05cf4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF50;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF50: ori.b #$4, d1
pub fn i_05cf50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF54;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF54: move.w #$2, d0
pub fn i_05cf54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF58;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF58: bra.w $5cfa8
pub fn i_05cf58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF5C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf5c  ($05CF5C, 16 B, input/event_thunks_05cdfc.s)
/// $05CF5C: move.b #$2, d1
pub fn i_05cf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF60;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF60: ori.b #$4, d1
pub fn i_05cf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF64;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05CF64: move.w #$2, d0
pub fn i_05cf64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF68;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF68: bra.w $5cfa8
pub fn i_05cf68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF6C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf6c  ($05CF6C, 12 B, input/event_thunks_05cdfc.s)
/// $05CF6C: move.b #$8, d1
pub fn i_05cf6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF70;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF70: move.w #$3, d0
pub fn i_05cf70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF74;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF74: bra.w $5cfa8
pub fn i_05cf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF78;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf78  ($05CF78, 12 B, input/event_thunks_05cdfc.s)
/// $05CF78: move.b #$4, d1
pub fn i_05cf78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF7C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF7C: move.w #$3, d0
pub fn i_05cf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF80;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF80: bra.w $5cfa8
pub fn i_05cf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF84;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf84  ($05CF84, 12 B, input/event_thunks_05cdfc.s)
/// $05CF84: move.b #$1, d1
pub fn i_05cf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF88;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF88: move.w #$3, d0
pub fn i_05cf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF8C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF8C: bra.w $5cfa8
pub fn i_05cf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF90;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf90  ($05CF90, 12 B, input/event_thunks_05cdfc.s)
/// $05CF90: move.b #$2, d1
pub fn i_05cf90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF94;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CF94: move.w #$3, d0
pub fn i_05cf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF98;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CF98: bra.w $5cfa8
pub fn i_05cf98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CF9C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputEvtThunk_05cf9c  ($05CF9C, 12 B, input/event_thunks_05cdfc.s)
/// $05CF9C: move.b #$c, d1
pub fn i_05cf9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFA0;
    cpu.ir = 0x123C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05CFA0: move.w #$3, d0
pub fn i_05cfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFA4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05CFA4: bra.w $5cfa8
pub fn i_05cfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFA8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFA8;
    10
}
// ===== InputMask_CheckChannelAvail_05cfa8  ($05CFA8, 32 B, input/check_channel_05cfa8.s)
/// $05CFA8: btst.b #$7, $100000.l
pub fn i_05cfa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFB0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05CFB0: bne.w $5cfc8
pub fn i_05cfb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFB4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5CFC8; return 10; }
    8
}
/// $05CFB4: lea.l $1001c0.l, a4
pub fn i_05cfb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFBA;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05CFBA: tst.b $44(a4)
pub fn i_05cfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFBE;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05CFBE: beq.w $5cfc8
pub fn i_05cfbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFC2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5CFC8; return 10; }
    8
}
/// $05CFC2: andi.b #$ee, ccr
pub fn i_05cfc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFC6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05CFC6: rts
pub fn i_05cfc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputMask_ReadCtxSwitchPlayer_05cfc8  ($05CFC8, 48 B, input/ctx_switch_05cfc8.s)
/// $05CFC8: lea.l $5cc08(pc), a2
pub fn i_05cfc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFCC;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05CFCC: cmpi.b #$1, $6d(a6)
pub fn i_05cfcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFD2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05CFD2: bne.w $5cfe0
pub fn i_05cfd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5CFE0; return 10; }
    8
}
/// $05CFD6: lea.l $100300.l, a1
pub fn i_05cfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFDC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05CFDC: movea.l $72(a1), a2
pub fn i_05cfdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFE0;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05CFE0: cmpi.b #$2, $6d(a6)
pub fn i_05cfe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFE6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05CFE6: bne.w $5cff4
pub fn i_05cfe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5CFF4; return 10; }
    8
}
/// $05CFEA: lea.l $1003a0.l, a1
pub fn i_05cfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFF0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05CFF0: movea.l $72(a1), a2
pub fn i_05cff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFF4;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05CFF4: jsr $5d674(pc)
pub fn i_05cff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFF8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5CFF8);
    cpu.pc = 0x5D674;
    16
}
// ===== InputMask_TestChannelBit_05cff8  ($05CFF8, 22 B, input/test_bit_05cff8.s)
/// $05CFF8: move.b (a2, d0.w), d0
pub fn i_05cff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFFC;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05CFFC: and.b d1, d0
pub fn i_05cffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5CFFE;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05CFFE: bne.w $5d008
pub fn i_05cffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D002;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D008; return 10; }
    8
}
/// $05D002: andi.b #$ee, ccr
pub fn i_05d002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D006;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D006: rts
pub fn i_05d006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D008;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D008: ori.b #$11, ccr
pub fn i_05d008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D00C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D00C: rts
pub fn i_05d00c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D00E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvtThunk_05d00e  ($05D00E, 12 B, input/event_thunks_05d00e.s)
/// $05D00E: move.b #$30, d1
pub fn i_05d00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D012;
    cpu.ir = 0x123C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D012: move.w #$3, d0
pub fn i_05d012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D016;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D016: bra.w $5d032
pub fn i_05d016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D01A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D032;
    10
}
// ===== InputEvtThunk_05d01a  ($05D01A, 12 B, input/event_thunks_05d00e.s)
/// $05D01A: move.b #$60, d1
pub fn i_05d01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D01E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D01E: move.w #$3, d0
pub fn i_05d01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D022;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D022: bra.w $5d032
pub fn i_05d022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D026;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D032;
    10
}
// ===== InputEvtThunk_05d026  ($05D026, 12 B, input/event_thunks_05d00e.s)
/// $05D026: move.b #$a0, d1
pub fn i_05d026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D02A;
    cpu.ir = 0x123C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D02A: move.w #$3, d0
pub fn i_05d02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D02E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D02E: bra.w $5d032
pub fn i_05d02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D032;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D032;
    10
}
// ===== InputMask_CheckChannelAvail_05d032  ($05D032, 32 B, input/check_channel_05d032.s)
/// $05D032: btst.b #$7, $100000.l
pub fn i_05d032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D03A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05D03A: bne.w $5d052
pub fn i_05d03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D03E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D052; return 10; }
    8
}
/// $05D03E: lea.l $1001c0.l, a4
pub fn i_05d03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D044;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05D044: tst.b $44(a4)
pub fn i_05d044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D048;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05D048: beq.w $5d052
pub fn i_05d048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D04C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D052; return 10; }
    8
}
/// $05D04C: andi.b #$ee, ccr
pub fn i_05d04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D050;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D050: rts
pub fn i_05d050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D052;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputMask_ReadCtxSwitchPlayer_05d052  ($05D052, 48 B, input/ctx_switch_05d052.s)
/// $05D052: lea.l $5cc08(pc), a2
pub fn i_05d052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D056;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05D056: cmpi.b #$1, $6d(a6)
pub fn i_05d056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D05C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05D05C: bne.w $5d06a
pub fn i_05d05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D060;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D06A; return 10; }
    8
}
/// $05D060: lea.l $100300.l, a1
pub fn i_05d060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D066;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05D066: movea.l $72(a1), a2
pub fn i_05d066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D06A;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D06A: cmpi.b #$2, $6d(a6)
pub fn i_05d06a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D070;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05D070: bne.w $5d07e
pub fn i_05d070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D074;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D07E; return 10; }
    8
}
/// $05D074: lea.l $1003a0.l, a1
pub fn i_05d074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D07A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05D07A: movea.l $72(a1), a2
pub fn i_05d07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D07E;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D07E: jsr $5d674(pc)
pub fn i_05d07e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D082;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D082);
    cpu.pc = 0x5D674;
    16
}
// ===== InputMask_TestChannelAllBits_05d082  ($05D082, 24 B, input/test_all_bits_05d082.s)
/// $05D082: move.b (a2, d0.w), d0
pub fn i_05d082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D086;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D086: and.b d1, d0
pub fn i_05d086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D088;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05D088: cmp.b d1, d0
pub fn i_05d088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D08A;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05D08A: beq.w $5d094
pub fn i_05d08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D08E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D094; return 10; }
    8
}
/// $05D08E: andi.b #$ee, ccr
pub fn i_05d08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D092;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D092: rts
pub fn i_05d092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D094;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D094: ori.b #$11, ccr
pub fn i_05d094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D098;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D098: rts
pub fn i_05d098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D09A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvtThunk_05d09a  ($05D09A, 18 B, input/event_thunks_05d09a.s)
/// $05D09A: move.b #$f0, d1
pub fn i_05d09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D09E;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D09E: move.w #$3, d0
pub fn i_05d09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0A2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D0A2: lea.l $10e200.l, a2
pub fn i_05d0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0A8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D0A8: bra.w $5d0e2
pub fn i_05d0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D0E2;
    10
}
// ===== InputEvtThunk_05d0ac  ($05D0AC, 18 B, input/event_thunks_05d09a.s)
/// $05D0AC: move.b #$f0, d1
pub fn i_05d0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0B0;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D0B0: move.w #$3, d0
pub fn i_05d0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0B4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D0B4: lea.l $10e206.l, a2
pub fn i_05d0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0BA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D0BA: bra.w $5d0e2
pub fn i_05d0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D0E2;
    10
}
// ===== InputEvtThunk_05d0be  ($05D0BE, 18 B, input/event_thunks_05d09a.s)
/// $05D0BE: move.b #$f0, d1
pub fn i_05d0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0C2;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D0C2: move.w #$2, d0
pub fn i_05d0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D0C6: lea.l $10e200.l, a2
pub fn i_05d0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0CC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D0CC: bra.w $5d0e2
pub fn i_05d0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0D0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D0E2;
    10
}
// ===== InputEvtThunk_05d0d0  ($05D0D0, 18 B, input/event_thunks_05d09a.s)
/// $05D0D0: move.b #$f0, d1
pub fn i_05d0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0D4;
    cpu.ir = 0x123C;
    let v1: u32 = 0xF0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D0D4: move.w #$2, d0
pub fn i_05d0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0D8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D0D8: lea.l $10e206.l, a2
pub fn i_05d0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0DE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D0DE: bra.w $5d0e2
pub fn i_05d0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D0E2;
    10
}
// ===== InputMask_TestChannelBit_05d0e2  ($05D0E2, 22 B, input/test_bit_05d0e2.s)
/// $05D0E2: move.b (a2, d0.w), d0
pub fn i_05d0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0E6;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D0E6: and.b d1, d0
pub fn i_05d0e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0E8;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05D0E8: bne.w $5d0f2
pub fn i_05d0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0EC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D0F2; return 10; }
    8
}
/// $05D0EC: andi.b #$ee, ccr
pub fn i_05d0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0F0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D0F0: rts
pub fn i_05d0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D0F2: ori.b #$11, ccr
pub fn i_05d0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0F6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D0F6: rts
pub fn i_05d0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvtThunk_05d0f8  ($05D0F8, 18 B, input/event_thunks_05d09a.s)
/// $05D0F8: move.b #$1, d1
pub fn i_05d0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D0FC;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D0FC: move.w #$2, d0
pub fn i_05d0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D100;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D100: lea.l $10e206.l, a2
pub fn i_05d100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D106;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D106: bra.w $5d1c0
pub fn i_05d106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D10A;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1C0;
    10
}
// ===== InputEvtThunk_05d10a  ($05D10A, 18 B, input/event_thunks_05d09a.s)
/// $05D10A: move.b #$2, d1
pub fn i_05d10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D10E;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D10E: move.w #$2, d0
pub fn i_05d10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D112;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D112: lea.l $10e206.l, a2
pub fn i_05d112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D118;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D118: bra.w $5d1c0
pub fn i_05d118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D11C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1C0;
    10
}
// ===== InputEvtThunk_05d11c  ($05D11C, 18 B, input/event_thunks_05d09a.s)
/// $05D11C: move.b #$8, d1
pub fn i_05d11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D120;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D120: move.w #$2, d0
pub fn i_05d120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D124;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D124: lea.l $10e206.l, a2
pub fn i_05d124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D12A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D12A: bra.w $5d1c0
pub fn i_05d12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D12E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1C0;
    10
}
// ===== InputEvtThunk_05d12e  ($05D12E, 18 B, input/event_thunks_05d09a.s)
/// $05D12E: move.b #$4, d1
pub fn i_05d12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D132;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D132: move.w #$2, d0
pub fn i_05d132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D136;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D136: lea.l $10e206.l, a2
pub fn i_05d136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D13C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D13C: bra.w $5d1c0
pub fn i_05d13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D140;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1C0;
    10
}
// ===== InputEvtThunk_05d140  ($05D140, 12 B, input/event_thunks_05d09a.s)
/// $05D140: move.b #$1, d1
pub fn i_05d140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D144;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D144: move.w #$2, d0
pub fn i_05d144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D148;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D148: bra.w $5d170
pub fn i_05d148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D14C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D170;
    10
}
// ===== InputEvtThunk_05d14c  ($05D14C, 12 B, input/event_thunks_05d09a.s)
/// $05D14C: move.b #$2, d1
pub fn i_05d14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D150;
    cpu.ir = 0x123C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D150: move.w #$2, d0
pub fn i_05d150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D154;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D154: bra.w $5d170
pub fn i_05d154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D158;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D170;
    10
}
// ===== InputEvtThunk_05d158  ($05D158, 12 B, input/event_thunks_05d09a.s)
/// $05D158: move.b #$8, d1
pub fn i_05d158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D15C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D15C: move.w #$2, d0
pub fn i_05d15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D160;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D160: bra.w $5d170
pub fn i_05d160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D164;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D170;
    10
}
// ===== InputEvtThunk_05d164  ($05D164, 12 B, input/event_thunks_05d09a.s)
/// $05D164: move.b #$4, d1
pub fn i_05d164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D168;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D168: move.w #$2, d0
pub fn i_05d168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D16C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D16C: bra.w $5d170
pub fn i_05d16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D170;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D170;
    10
}
// ===== InputMask_CheckChannelAvail_05d170  ($05D170, 32 B, input/check_channel_05d170.s)
/// $05D170: btst.b #$7, $100000.l
pub fn i_05d170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D178;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05D178: bne.w $5d190
pub fn i_05d178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D17C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D190; return 10; }
    8
}
/// $05D17C: lea.l $1001c0.l, a4
pub fn i_05d17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D182;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05D182: tst.b $44(a4)
pub fn i_05d182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D186;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05D186: beq.w $5d190
pub fn i_05d186(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D18A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D190; return 10; }
    8
}
/// $05D18A: andi.b #$ee, ccr
pub fn i_05d18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D18E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D18E: rts
pub fn i_05d18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D190;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputMask_ReadCtxSwitchPlayer_05d190  ($05D190, 48 B, input/ctx_switch_05d190.s)
/// $05D190: lea.l $5cc08(pc), a2
pub fn i_05d190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D194;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05D194: cmpi.b #$1, $6d(a6)
pub fn i_05d194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D19A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05D19A: bne.w $5d1a8
pub fn i_05d19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D19E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D1A8; return 10; }
    8
}
/// $05D19E: lea.l $100300.l, a1
pub fn i_05d19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1A4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05D1A4: movea.l $72(a1), a2
pub fn i_05d1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1A8;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D1A8: cmpi.b #$2, $6d(a6)
pub fn i_05d1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1AE;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05D1AE: bne.w $5d1bc
pub fn i_05d1ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D1BC; return 10; }
    8
}
/// $05D1B2: lea.l $1003a0.l, a1
pub fn i_05d1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1B8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05D1B8: movea.l $72(a1), a2
pub fn i_05d1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1BC;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D1BC: jsr $5d674(pc)
pub fn i_05d1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1C0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D1C0);
    cpu.pc = 0x5D674;
    16
}
// ===== InputMask_TestChannelNibbleXor_05d1c0  ($05D1C0, 26 B, input/test_nibble_xor_05d1c0.s)
/// $05D1C0: move.b (a2, d0.w), d0
pub fn i_05d1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1C4;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D1C4: andi.b #$f, d0
pub fn i_05d1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1C8;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D1C8: eor.b d1, d0
pub fn i_05d1c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1CA;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05D1CA: beq.w $5d1d4
pub fn i_05d1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D1D4; return 10; }
    8
}
/// $05D1CE: andi.b #$ee, ccr
pub fn i_05d1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1D2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D1D2: rts
pub fn i_05d1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D1D4: ori.b #$11, ccr
pub fn i_05d1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1D8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D1D8: rts
pub fn i_05d1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvt_Thunk4aThenMask60_05d1da  ($05D1DA, 42 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D1DA: jsr $5ce4a(pc)
pub fn i_05d1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D1DE);
    cpu.pc = 0x5CE4A;
    16
}
/// $05D1DE: bcc.w $5d204
pub fn i_05d1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5D204; return 10; }
    8
}
/// $05D1E2: move.b #$20, d1
pub fn i_05d1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1E6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D1E6: ori.b #$40, d1
pub fn i_05d1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1EA;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D1EA: move.w #$2, d0
pub fn i_05d1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1EE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D1EE: lea.l $10e206.l, a2
pub fn i_05d1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1F4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D1F4: bra.w $5d1f8
pub fn i_05d1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D1F8;
    10
}
/// $05D1F8: move.b (a2, d0.w), d0
pub fn i_05d1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1FC;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D1FC: and.b d1, d0
pub fn i_05d1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D1FE;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05D1FE: cmp.b d1, d0
pub fn i_05d1fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D200;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05D200: beq.w $5d20a
pub fn i_05d200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D204;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D20A; return 10; }
    8
}
// ===== ClearXN_05d204  ($05D204, 6 B, thunks/ccr_helpers.c)
/// $05D204: andi.b #$ee, ccr
pub fn i_05d204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D208;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D208: rts
pub fn i_05d208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D20A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05d20a  ($05D20A, 6 B, thunks/ccr_helpers.c)
/// $05D20A: ori.b #$11, ccr
pub fn i_05d20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D20E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D20E: rts
pub fn i_05d20e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D210;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvt_Thunk4aThenMask40_05d210  ($05D210, 36 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D210: jsr $5ce4a(pc)
pub fn i_05d210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D214;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D214);
    cpu.pc = 0x5CE4A;
    16
}
/// $05D214: bcc.b $5d204
pub fn i_05d214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D216;
    cpu.ir = 0x64EE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5D204; return 10; }
    8
}
/// $05D216: move.b #$40, d1
pub fn i_05d216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D21A;
    cpu.ir = 0x123C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D21A: move.w #$2, d0
pub fn i_05d21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D21E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D21E: lea.l $10e206.l, a2
pub fn i_05d21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D224;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E206;
    20
}
/// $05D224: bra.w $5d228
pub fn i_05d224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D228;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D228;
    10
}
/// $05D228: move.b (a2, d0.w), d0
pub fn i_05d228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D22C;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D22C: and.b d1, d0
pub fn i_05d22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D22E;
    cpu.ir = 0xC001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 & cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05D22E: cmp.b d1, d0
pub fn i_05d22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D230;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05D230: beq.w $5d23a
pub fn i_05d230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D234;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D23A; return 10; }
    8
}
// ===== ClearXN_05d234  ($05D234, 6 B, thunks/ccr_helpers.c)
/// $05D234: andi.b #$ee, ccr
pub fn i_05d234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D238;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D238: rts
pub fn i_05d238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D23A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05d23a  ($05D23A, 6 B, thunks/ccr_helpers.c)
/// $05D23A: ori.b #$11, ccr
pub fn i_05d23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D23E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05D23E: rts
pub fn i_05d23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D240;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvt_Thunk4aThenToggle_05d240  ($05D240, 72 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D240: jsr $5ce4a(pc)
pub fn i_05d240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D244;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D244);
    cpu.pc = 0x5CE4A;
    16
}
/// $05D244: bcs.w $5d24c
pub fn i_05d244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D248;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D24C; return 10; }
    8
}
/// $05D248: bra.w $5d2ee
pub fn i_05d248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D24C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2EE;
    10
}
/// $05D24C: move.b #$8, d1
pub fn i_05d24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D250;
    cpu.ir = 0x123C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D250: move.w #$0, d0
pub fn i_05d250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D254;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D254: bra.w $5d2de
pub fn i_05d254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D258;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2DE;
    10
}
/// $05D258: jsr $5cd06(pc)
pub fn i_05d258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D25C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D25C);
    cpu.pc = 0x5CD06;
    16
}
/// $05D25C: bcs.w $5d264
pub fn i_05d25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D260;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D264; return 10; }
    8
}
/// $05D260: bra.w $5d2ee
pub fn i_05d260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D264;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2EE;
    10
}
/// $05D264: move.b #$1, d1
pub fn i_05d264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D268;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D268: move.w #$0, d0
pub fn i_05d268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D26C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D26C: bra.w $5d2de
pub fn i_05d26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D270;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2DE;
    10
}
/// $05D270: jsr $5cd06(pc)
pub fn i_05d270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D274;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D274);
    cpu.pc = 0x5CD06;
    16
}
/// $05D274: bcs.w $5d27c
pub fn i_05d274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D278;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D27C; return 10; }
    8
}
/// $05D278: bra.w $5d2ee
pub fn i_05d278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D27C;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2EE;
    10
}
/// $05D27C: move.b #$4, d1
pub fn i_05d27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D280;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D280: move.w #$0, d0
pub fn i_05d280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D284;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D284: bra.w $5d2de
pub fn i_05d284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D288;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2DE;
    10
}
// ===== InputEvt_ToggleChain_05d288  ($05D288, 122 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D288: move.b #$1, d1
pub fn i_05d288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D28C;
    cpu.ir = 0x123C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D28C: move.w #$2, d0
pub fn i_05d28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D290;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D290: lea.l $10e200.l, a2
pub fn i_05d290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D296;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D296: jsr $5cff8(pc)
pub fn i_05d296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D29A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D29A);
    cpu.pc = 0x5CFF8;
    16
}
/// $05D29A: bcs.w $5d2a2
pub fn i_05d29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D29E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D2A2; return 10; }
    8
}
/// $05D29E: bra.w $5d2ee
pub fn i_05d29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2A2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2EE;
    10
}
/// $05D2A2: move.b #$80, d1
pub fn i_05d2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2A6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D2A6: move.w #$3, d0
pub fn i_05d2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2AA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D2AA: lea.l $10e200.l, a2
pub fn i_05d2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2B0;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D2B0: bra.w $5cff8
pub fn i_05d2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
/// $05D2B4: jsr $5cd06(pc)
pub fn i_05d2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D2B8);
    cpu.pc = 0x5CD06;
    16
}
/// $05D2B8: bcs.w $5d2c0
pub fn i_05d2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2BC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D2C0; return 10; }
    8
}
/// $05D2BC: bra.w $5d2ee
pub fn i_05d2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2EE;
    10
}
/// $05D2C0: move.b #$80, d1
pub fn i_05d2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2C4;
    cpu.ir = 0x123C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D2C4: move.w #$3, d0
pub fn i_05d2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2C8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D2C8: lea.l $10e200.l, a2
pub fn i_05d2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2CE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E200;
    20
}
/// $05D2CE: bra.w $5cff8
pub fn i_05d2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5CFF8;
    10
}
/// $05D2D2: move.b #$4, d1
pub fn i_05d2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2D6;
    cpu.ir = 0x123C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D2D6: move.w #$1, d0
pub fn i_05d2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2DA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D2DA: bra.w $5d2de
pub fn i_05d2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2DE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D2DE;
    10
}
/// $05D2DE: lea.l $10e20c.l, a2
pub fn i_05d2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2E4;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E20C;
    20
}
/// $05D2E4: move.b (a2, d0.w), d0
pub fn i_05d2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2E8;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D2E8: eor.b d1, d0
pub fn i_05d2e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2EA;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05D2EA: beq.w $5d2f8
pub fn i_05d2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2EE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D2F8; return 10; }
    8
}
/// $05D2EE: move.b #$1, d0
pub fn i_05d2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2F2;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D2F2: subi.b #$1, d0
pub fn i_05d2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2F6;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D2F6: rts
pub fn i_05d2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D2F8: move.b #$0, d0
pub fn i_05d2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D2FC;
    cpu.ir = 0x103C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05D2FC: subi.b #$1, d0
pub fn i_05d2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D300;
    cpu.ir = 0x400;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D300: rts
pub fn i_05d300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D302;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputEvt_ThunkMaskF0_05d302  ($05D302, 8 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D302: jsr $5cd90(pc)
pub fn i_05d302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D306;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D306);
    cpu.pc = 0x5CD90;
    16
}
/// $05D306: bcc.w $5d310
pub fn i_05d306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D30A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5D310; return 10; }
    8
}
// ===== JsrPcThunk_05d30a  ($05D30A, 6 B, thunks/jsr_pc_thunks.c)
/// $05D30A: jsr $5cf6c(pc)
pub fn i_05d30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D30E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D30E);
    cpu.pc = 0x5CF6C;
    16
}
/// $05D30E: rts
pub fn i_05d30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D310;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_05d310  ($05D310, 6 B, thunks/ccr_helpers.c)
/// $05D310: andi.b #$ee, ccr
pub fn i_05d310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D314;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05D314: rts
pub fn i_05d314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D316;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputLayout_ReadField2_05D5B6  ($05D5B6, 96 B, input/aim_tables_05d316.s)
/// $05D5B6: move.w #$2, d1
pub fn i_05d5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5BA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D5BA: lea.l $5cc08(pc), a2
pub fn i_05d5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5BE;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05D5BE: cmpi.b #$1, $6d(a6)
pub fn i_05d5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05D5C4: bne.w $5d5d2
pub fn i_05d5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5C8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D5D2; return 10; }
    8
}
/// $05D5C8: lea.l $100300.l, a1
pub fn i_05d5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05D5CE: movea.l $72(a1), a2
pub fn i_05d5ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5D2;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D5D2: cmpi.b #$2, $6d(a6)
pub fn i_05d5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5D8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05D5D8: bne.w $5d5e6
pub fn i_05d5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D5E6; return 10; }
    8
}
/// $05D5DC: lea.l $1003a0.l, a1
pub fn i_05d5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05D5E2: movea.l $72(a1), a2
pub fn i_05d5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5E6;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D5E6: moveq #$0, d0
pub fn i_05d5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5E8;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D5E8: btst.b #$7, $100000.l
pub fn i_05d5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5F0;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05D5F0: bne.w $5d606
pub fn i_05d5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D606; return 10; }
    8
}
/// $05D5F4: lea.l $1001c0.l, a4
pub fn i_05d5f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5FA;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05D5FA: tst.b $44(a4)
pub fn i_05d5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D5FE;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05D5FE: beq.w $5d606
pub fn i_05d5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D602;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D606; return 10; }
    8
}
/// $05D602: nop
pub fn i_05d602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D604;
    cpu.ir = 0x4E71;
    4
}
/// $05D604: rts
pub fn i_05d604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D606;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D606: jsr $5d674(pc)
pub fn i_05d606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D60A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D60A);
    cpu.pc = 0x5D674;
    16
}
/// $05D60A: move.b (a2, d1.w), d0
pub fn i_05d60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D60E;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D60E: andi.b #$c, d0
pub fn i_05d60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D612;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xC) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D612: asr.b #$2, d0
pub fn i_05d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D614;
    cpu.ir = 0xE400;
    let h1 = cpu.d[0] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Byte, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    10
}
/// $05D614: rts
pub fn i_05d614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D616;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputLayout_ReadField0_05D616  ($05D616, 94 B, input/aim_tables_05d316.s)
/// $05D616: move.w #$2, d1
pub fn i_05d616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D61A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D61A: lea.l $5cc08(pc), a2
pub fn i_05d61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D61E;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x5CC08;
    16
}
/// $05D61E: cmpi.b #$1, $6d(a6)
pub fn i_05d61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D624;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $05D624: bne.w $5d632
pub fn i_05d624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D628;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D632; return 10; }
    8
}
/// $05D628: movea.l $72(a1), a2
pub fn i_05d628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D62C;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D62C: lea.l $100300.l, a1
pub fn i_05d62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D632;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100300;
    20
}
/// $05D632: cmpi.b #$2, $6d(a6)
pub fn i_05d632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D638;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $05D638: bne.w $5d646
pub fn i_05d638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D63C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D646; return 10; }
    8
}
/// $05D63C: lea.l $1003a0.l, a1
pub fn i_05d63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D642;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1003A0;
    20
}
/// $05D642: movea.l $72(a1), a2
pub fn i_05d642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D646;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x72);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $05D646: moveq #$0, d0
pub fn i_05d646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D648;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D648: btst.b #$7, $100000.l
pub fn i_05d648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D650;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((7) % 8);
    let v2 = bus.read8(0x100000) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $05D650: bne.w $5d666
pub fn i_05d650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D654;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D666; return 10; }
    8
}
/// $05D654: lea.l $1001c0.l, a4
pub fn i_05d654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D65A;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x1001C0;
    20
}
/// $05D65A: tst.b $44(a4)
pub fn i_05d65a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D65E;
    cpu.ir = 0x4A2C;
    let ad2 = cpu.a[4].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $05D65E: beq.w $5d666
pub fn i_05d65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D662;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D666; return 10; }
    8
}
/// $05D662: nop
pub fn i_05d662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D664;
    cpu.ir = 0x4E71;
    4
}
/// $05D664: rts
pub fn i_05d664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D666;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05D666: jsr $5d674(pc)
pub fn i_05d666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D66A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D66A);
    cpu.pc = 0x5D674;
    16
}
/// $05D66A: move.b (a2, d1.w), d0
pub fn i_05d66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D66E;
    cpu.ir = 0x1032;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D66E: andi.b #$3, d0
pub fn i_05d66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D672;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05D672: rts
pub fn i_05d672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D674;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputCtx_DemoOverride_05D674  ($05D674, 44 B, input/aim_tables_05d316.s)
/// $05D674: movem.l d0, -(a7)
pub fn i_05d674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D678;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05D678: tst.b $10e39d.l
pub fn i_05d678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D67E;
    cpu.ir = 0x4A39;
    let v2 = bus.read8(0x10E39D) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    16
}
/// $05D67E: beq.w $5d69a
pub fn i_05d67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D682;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D69A; return 10; }
    8
}
/// $05D682: move.l a2, d0
pub fn i_05d682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D684;
    cpu.ir = 0x200A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D684: lea.l $10e2e2.l, a2
pub fn i_05d684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D68A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E2E2;
    20
}
/// $05D68A: cmpi.l #$10e200, d0
pub fn i_05d68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D690;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x10E200, Size::Long, false);
    8
}
/// $05D690: beq.w $5d69a
pub fn i_05d690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D694;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D69A; return 10; }
    8
}
/// $05D694: lea.l $10e2e8.l, a2
pub fn i_05d694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D69A;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E2E8;
    20
}
/// $05D69A: movem.l (a7)+, d0
pub fn i_05d69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D69E;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05D69E: rts
pub fn i_05d69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6A0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DebugHex_SetupA1_05D6A0  ($05D6A0, 10 B, input/aim_tables_05d316.s)
/// $05D6A0: movea.l #$7063, a1
pub fn i_05d6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6A6;
    cpu.ir = 0x227C;
    let v1: u32 = 0x7063;
    cpu.a[1] = v1;
    4
}
/// $05D6A6: move.w #$300, d1
pub fn i_05d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
// ===== JsrPcThunk_05d6aa  ($05D6AA, 6 B, thunks/jsr_pc_thunks.c)
/// $05D6AA: jsr $5d6c2(pc)
pub fn i_05d6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D6AE);
    cpu.pc = 0x5D6C2;
    16
}
/// $05D6AE: rts
pub fn i_05d6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_HexFormat8_Prologue_05d6b0  ($05D6B0, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D6B0: lea.l $10e21e.l, a2
pub fn i_05d6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6B6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D6B6: moveq #$1c, d2
pub fn i_05d6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6B8;
    cpu.ir = 0x741C;
    cpu.d[2] = 0x1C;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D6B8: move.w #$8, d3
pub fn i_05d6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6BC;
    cpu.ir = 0x363C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D6BC: move.w d3, -(a7)
pub fn i_05d6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6BE;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D6BE: jmp $5d6d0(pc)
pub fn i_05d6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6C2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x5D6D0;
    8
}
// ===== Sprite_HexFormat4_05D6C2  ($05D6C2, 90 B, video/sprite_hex_format4_05d6c2.s)
/// $05D6C2: lea.l $10e21e.l, a2
pub fn i_05d6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6C8;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D6C8: moveq #$c, d2
pub fn i_05d6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6CA;
    cpu.ir = 0x740C;
    cpu.d[2] = 0xC;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D6CA: move.w #$4, d3
pub fn i_05d6ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6CE;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D6CE: move.w d3, -(a7)
pub fn i_05d6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6D0;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D6D0: move.w #$0, d1
pub fn i_05d6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6D4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D6D4: move.l d2, -(a7)
pub fn i_05d6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6D6;
    cpu.ir = 0x2F02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05D6D6: move.l d0, d3
pub fn i_05d6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6D8;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D6D8: lsr.l d2, d3
pub fn i_05d6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6DA;
    cpu.ir = 0xE4AB;
    let cnt = cpu.d[2] & 0x3F;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 1, false);
    cpu.d[3] = r;
    6 + 2 * cnt
}
/// $05D6DA: andi.l #$f, d3
pub fn i_05d6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E0;
    cpu.ir = 0x283;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[3] = r2;
    8
}
/// $05D6E0: move.w d3, (a2)
pub fn i_05d6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E2;
    cpu.ir = 0x3483;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D6E2: addq.l #$2, a2
pub fn i_05d6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E4;
    cpu.ir = 0x548A;
    cpu.a[2] = cpu.a[2].wrapping_add(2);
    8
}
/// $05D6E4: move.l (a7)+, d2
pub fn i_05d6e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E6;
    cpu.ir = 0x241F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05D6E6: subq.l #$4, d2
pub fn i_05d6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6E8;
    cpu.ir = 0x5982;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Long, true);
    cpu.d[2] = r2;
    4
}
/// $05D6E8: bne.b $5d6d0
pub fn i_05d6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6EA;
    cpu.ir = 0x66E6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D6D0; return 10; }
    8
}
/// $05D6EA: andi.l #$f, d0
pub fn i_05d6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6F0;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D6F0: move.w d0, (a2)
pub fn i_05d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6F2;
    cpu.ir = 0x3480;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D6F2: move.w (a7)+, d3
pub fn i_05d6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6F4;
    cpu.ir = 0x361F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D6F4: subq.w #$1, d3
pub fn i_05d6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6F6;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D6F6: lea.l $10e21e.l, a3
pub fn i_05d6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6FC;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10E21E;
    20
}
/// $05D6FC: move.l a1, d0
pub fn i_05d6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D6FE;
    cpu.ir = 0x2009;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D6FE: move.w (a3)+, d4
pub fn i_05d6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D700;
    cpu.ir = 0x381B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D700: move.b $5d71c(pc, d4.w), d2
pub fn i_05d700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D704;
    cpu.ir = 0x143B;
    let ad2 = 0x5D71C_u32.wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D704: andi.w #$ff, d2
pub fn i_05d704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D708;
    cpu.ir = 0x242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D708: add.w d1, d2
pub fn i_05d708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D70A;
    cpu.ir = 0xD441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05D70A: movem.w d0/d2, $3c0000.l
pub fn i_05d70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D712;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05D712: addi.w #$20, d0
pub fn i_05d712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D716;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D716: dbra d3, $5d6fe
pub fn i_05d716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D71A;
    cpu.ir = 0x51CB;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5D71A; return 12; }
    let n = (cpu.d[3] as u16).wrapping_sub(1);
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5D71A; return 14; }
    cpu.pc = 0x5D6FE;
    10
}
/// $05D71A: rts
pub fn i_05d71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D71C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sprite_HexFormat8_05D72C  ($05D72C, 146 B, video/sprite_hex_format8_05d72c.s)
/// $05D72C: move.l -(a0), d0
pub fn i_05d72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D72E;
    cpu.ir = 0x2020;
    let ad2 = cpu.a[0].wrapping_sub(4); cpu.a[0] = ad2;
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05D72E: lea.l $10e21e.l, a2
pub fn i_05d72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D734;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D734: moveq #$1c, d2
pub fn i_05d734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D736;
    cpu.ir = 0x741C;
    cpu.d[2] = 0x1C;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D736: move.w #$8, d3
pub fn i_05d736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D73A;
    cpu.ir = 0x363C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D73A: move.w d3, -(a7)
pub fn i_05d73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D73C;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D73C: bra.w $5d74e
pub fn i_05d73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D740;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D74E;
    10
}
/// $05D740: lea.l $10e21e.l, a2
pub fn i_05d740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D746;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D746: moveq #$c, d2
pub fn i_05d746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D748;
    cpu.ir = 0x740C;
    cpu.d[2] = 0xC;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D748: move.w #$4, d3
pub fn i_05d748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D74C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D74C: move.w d3, -(a7)
pub fn i_05d74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D74E;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D74E: move.w #$0, d1
pub fn i_05d74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D752;
    cpu.ir = 0x323C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D752: move.l d2, -(a7)
pub fn i_05d752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D754;
    cpu.ir = 0x2F02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05D754: move.l d0, d3
pub fn i_05d754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D756;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D756: lsr.l d2, d3
pub fn i_05d756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D758;
    cpu.ir = 0xE4AB;
    let cnt = cpu.d[2] & 0x3F;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 1, false);
    cpu.d[3] = r;
    6 + 2 * cnt
}
/// $05D758: andi.l #$f, d3
pub fn i_05d758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D75E;
    cpu.ir = 0x283;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[3] = r2;
    8
}
/// $05D75E: move.w d3, (a2)
pub fn i_05d75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D760;
    cpu.ir = 0x3483;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D760: addq.l #$2, a2
pub fn i_05d760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D762;
    cpu.ir = 0x548A;
    cpu.a[2] = cpu.a[2].wrapping_add(2);
    8
}
/// $05D762: move.l (a7)+, d2
pub fn i_05d762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D764;
    cpu.ir = 0x241F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05D764: subq.l #$4, d2
pub fn i_05d764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D766;
    cpu.ir = 0x5982;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Long, true);
    cpu.d[2] = r2;
    4
}
/// $05D766: bne.b $5d74e
pub fn i_05d766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D768;
    cpu.ir = 0x66E6;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D74E; return 10; }
    8
}
/// $05D768: andi.l #$f, d0
pub fn i_05d768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D76E;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D76E: move.w d0, (a2)
pub fn i_05d76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D770;
    cpu.ir = 0x3480;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D770: move.w (a7)+, d3
pub fn i_05d770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D772;
    cpu.ir = 0x361F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D772: subq.w #$1, d3
pub fn i_05d772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D774;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D774: clr.w d2
pub fn i_05d774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D776;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D776: lea.l $10e21e.l, a3
pub fn i_05d776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D77C;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x10E21E;
    20
}
/// $05D77C: lea.l $5d71c(pc), a0
pub fn i_05d77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D780;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5D71C;
    16
}
/// $05D780: move.l a1, d5
pub fn i_05d780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D782;
    cpu.ir = 0x2A09;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[5] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D782: move.w (a3)+, d4
pub fn i_05d782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D784;
    cpu.ir = 0x381B;
    let ad2 = cpu.a[3]; cpu.a[3] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D784: tst.w d4
pub fn i_05d784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D786;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D786: bne.w $5d79e
pub fn i_05d786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D78A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D79E; return 10; }
    8
}
/// $05D78A: tst.w d2
pub fn i_05d78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D78C;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D78C: bne.w $5d79e
pub fn i_05d78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D790;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D79E; return 10; }
    8
}
/// $05D790: tst.w d3
pub fn i_05d790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D792;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D792: beq.w $5d79e
pub fn i_05d792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D796;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D79E; return 10; }
    8
}
/// $05D796: move.w #$10, d4
pub fn i_05d796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D79A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D79A: bra.w $5d7a2
pub fn i_05d79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D79E;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D7A2;
    10
}
/// $05D79E: move.w #$ffff, d2
pub fn i_05d79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7A2;
    cpu.ir = 0x343C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D7A2: move.b (a0, d4.w), d6
pub fn i_05d7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7A6;
    cpu.ir = 0x1C30;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D7A6: andi.w #$ff, d6
pub fn i_05d7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7AA;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D7AA: add.w d1, d6
pub fn i_05d7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7AC;
    cpu.ir = 0xDC41;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05D7AC: movem.w d5-d6, $3c0000.l
pub fn i_05d7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7B4;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[5] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[6] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05D7B4: addi.w #$20, d5
pub fn i_05d7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7B8;
    cpu.ir = 0x645;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D7B8: dbra d3, $5d782
pub fn i_05d7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7BC;
    cpu.ir = 0x51CB;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5D7BC; return 12; }
    let n = (cpu.d[3] as u16).wrapping_sub(1);
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5D7BC; return 14; }
    cpu.pc = 0x5D782;
    10
}
/// $05D7BC: rts
pub fn i_05d7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7BE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Debug_HexDrawToFix8_05d7be  ($05D7BE, 26 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D7BE: swap d1
pub fn i_05d7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7C0;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7C0: lsr.l #$4, d1
pub fn i_05d7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7C2;
    cpu.ir = 0xE889;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 1, false);
    cpu.d[1] = r;
    14
}
/// $05D7C2: andi.w #$f000, d1
pub fn i_05d7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7C6;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D7C6: lea.l $10e21e.l, a2
pub fn i_05d7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7CC;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D7CC: moveq #$1c, d2
pub fn i_05d7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7CE;
    cpu.ir = 0x741C;
    cpu.d[2] = 0x1C;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7CE: move.w #$8, d3
pub fn i_05d7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7D2;
    cpu.ir = 0x363C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D7D2: move.w d3, -(a7)
pub fn i_05d7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7D4;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D7D4: bra.w $5d7ee
pub fn i_05d7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D7EE;
    10
}
// ===== Debug_HexDrawToFix4_05d7d8  ($05D7D8, 140 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D7D8: swap d1
pub fn i_05d7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7DA;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7DA: lsr.l #$4, d1
pub fn i_05d7da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7DC;
    cpu.ir = 0xE889;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Long, 1, false);
    cpu.d[1] = r;
    14
}
/// $05D7DC: andi.w #$f000, d1
pub fn i_05d7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7E0;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0xF000) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D7E0: lea.l $10e21e.l, a2
pub fn i_05d7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7E6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D7E6: moveq #$c, d2
pub fn i_05d7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7E8;
    cpu.ir = 0x740C;
    cpu.d[2] = 0xC;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7E8: move.w #$4, d3
pub fn i_05d7e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7EC;
    cpu.ir = 0x363C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D7EC: move.w d3, -(a7)
pub fn i_05d7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7EE;
    cpu.ir = 0x3F03;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05D7EE: move.l d2, -(a7)
pub fn i_05d7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7F0;
    cpu.ir = 0x2F02;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05D7F0: move.l d0, d3
pub fn i_05d7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7F2;
    cpu.ir = 0x2600;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D7F2: lsr.l d2, d3
pub fn i_05d7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7F4;
    cpu.ir = 0xE4AB;
    let cnt = cpu.d[2] & 0x3F;
    let h1 = cpu.d[3] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 1, false);
    cpu.d[3] = r;
    6 + 2 * cnt
}
/// $05D7F4: andi.l #$f, d3
pub fn i_05d7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7FA;
    cpu.ir = 0x283;
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[3] = r2;
    8
}
/// $05D7FA: move.w d3, (a2)
pub fn i_05d7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7FC;
    cpu.ir = 0x3483;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D7FC: addq.l #$2, a2
pub fn i_05d7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D7FE;
    cpu.ir = 0x548A;
    cpu.a[2] = cpu.a[2].wrapping_add(2);
    8
}
/// $05D7FE: move.l (a7)+, d2
pub fn i_05d7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D800;
    cpu.ir = 0x241F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[2] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05D800: subq.l #$4, d2
pub fn i_05d800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D802;
    cpu.ir = 0x5982;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Long, true);
    cpu.d[2] = r2;
    4
}
/// $05D802: bne.b $5d7ee
pub fn i_05d802(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D804;
    cpu.ir = 0x66EA;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D7EE; return 10; }
    8
}
/// $05D804: andi.l #$f, d0
pub fn i_05d804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D80A;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D80A: move.w d0, (a2)
pub fn i_05d80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D80C;
    cpu.ir = 0x3480;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(cpu.a[2], v1 as u16);
    8
}
/// $05D80C: move.w (a7)+, d3
pub fn i_05d80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D80E;
    cpu.ir = 0x361F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D80E: subq.w #$1, d3
pub fn i_05d80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D810;
    cpu.ir = 0x5343;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D810: clr.w d2
pub fn i_05d810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D812;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D812: lea.l $10e21e.l, a2
pub fn i_05d812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D818;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x10E21E;
    20
}
/// $05D818: lea.l $5d864(pc), a3
pub fn i_05d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D81C;
    cpu.ir = 0x47FA;
    cpu.a[3] = 0x5D864;
    16
}
/// $05D81C: move.w (a2)+, d4
pub fn i_05d81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D81E;
    cpu.ir = 0x381A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05D81E: tst.w d4
pub fn i_05d81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D820;
    cpu.ir = 0x4A44;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D820: bne.w $5d838
pub fn i_05d820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D824;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D838; return 10; }
    8
}
/// $05D824: tst.w d2
pub fn i_05d824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D826;
    cpu.ir = 0x4A42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D826: bne.w $5d838
pub fn i_05d826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D82A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5D838; return 10; }
    8
}
/// $05D82A: tst.w d3
pub fn i_05d82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D82C;
    cpu.ir = 0x4A43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D82C: beq.w $5d838
pub fn i_05d82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D830;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D838; return 10; }
    8
}
/// $05D830: move.w #$10, d4
pub fn i_05d830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D834;
    cpu.ir = 0x383C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D834: bra.w $5d83c
pub fn i_05d834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D838;
    cpu.ir = 0x6000;
    cpu.pc = 0x5D83C;
    10
}
/// $05D838: move.w #$ffff, d2
pub fn i_05d838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D83C;
    cpu.ir = 0x343C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D83C: add.w d4, d4
pub fn i_05d83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D83E;
    cpu.ir = 0xD844;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05D83E: move.w (a3, d4.w), d0
pub fn i_05d83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D842;
    cpu.ir = 0x3033;
    let ad2 = cpu.a[3].wrapping_add(0x0).wrapping_add((cpu.d[4] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05D842: add.w d1, d0
pub fn i_05d842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D844;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05D844: movem.l d1-d4/a1-a3, -(a7)
pub fn i_05d844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D848;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05D848: move.w #$2, d1
pub fn i_05d848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D84C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D84C: move.w #$2, d2
pub fn i_05d84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D850;
    cpu.ir = 0x343C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D850: jsr $5da56.l
pub fn i_05d850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D856;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5D856);
    cpu.pc = 0x5DA56;
    16
}
/// $05D856: movem.l (a7)+, d1-d4/a1-a3
pub fn i_05d856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D85A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05D85A: adda.w #$40, a1
pub fn i_05d85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D85E;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05D85E: dbra d3, $5d81c
pub fn i_05d85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D862;
    cpu.ir = 0x51CB;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5D862; return 12; }
    let n = (cpu.d[3] as u16).wrapping_sub(1);
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5D862; return 14; }
    cpu.pc = 0x5D81C;
    10
}
/// $05D862: rts
pub fn i_05d862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D864;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Bin16_ToBcd4_05d886  ($05D886, 108 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D886: movem.l d1-d3, -(a7)
pub fn i_05d886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D88A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05D88A: moveq #$0, d3
pub fn i_05d88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D88C;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D88C: move.w #$a, d2
pub fn i_05d88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D890;
    cpu.ir = 0x343C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D890: andi.l #$ffff, d0
pub fn i_05d890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D896;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D896: beq.w $5d8ea
pub fn i_05d896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D89A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D8EA; return 10; }
    8
}
/// $05D89A: divu.w d2, d0
pub fn i_05d89a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D89C;
    cpu.ir = 0x80C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05D89C: swap d0
pub fn i_05d89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D89E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D89E: andi.w #$f, d0
pub fn i_05d89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8A2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D8A2: move.w d0, d3
pub fn i_05d8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8A4;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D8A4: swap d0
pub fn i_05d8a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8A6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8A6: andi.l #$ffff, d0
pub fn i_05d8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8AC;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D8AC: beq.w $5d8ea
pub fn i_05d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8B0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D8EA; return 10; }
    8
}
/// $05D8B0: divu.w d2, d0
pub fn i_05d8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8B2;
    cpu.ir = 0x80C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05D8B2: swap d0
pub fn i_05d8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8B4;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8B4: andi.w #$f, d0
pub fn i_05d8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8B8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D8B8: lsl.w #$4, d0
pub fn i_05d8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8BA;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05D8BA: or.w d0, d3
pub fn i_05d8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8BC;
    cpu.ir = 0x8640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[3]) & 0xFFFF;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D8BC: swap d0
pub fn i_05d8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8BE;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8BE: andi.l #$ffff, d0
pub fn i_05d8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8C4;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D8C4: beq.w $5d8ea
pub fn i_05d8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D8EA; return 10; }
    8
}
/// $05D8C8: divu.w d2, d0
pub fn i_05d8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8CA;
    cpu.ir = 0x80C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05D8CA: swap d0
pub fn i_05d8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8CC;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8CC: andi.w #$f, d0
pub fn i_05d8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8D0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D8D0: lsl.w #$8, d0
pub fn i_05d8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8D2;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05D8D2: or.w d0, d3
pub fn i_05d8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8D4;
    cpu.ir = 0x8640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[3]) & 0xFFFF;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D8D4: swap d0
pub fn i_05d8d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8D6;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8D6: andi.l #$ffff, d0
pub fn i_05d8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8DC;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $05D8DC: divu.w d2, d0
pub fn i_05d8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8DE;
    cpu.ir = 0x80C2;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set(StatusRegister::C, false);
    let dv = v1 & 0xFFFF;
    if dv == 0 { cpu.enter_group2_exception(bus, Exception::DivideByZero); return 38; }
    let q = cpu.d[0] / dv; let rm = cpu.d[0] % dv;
    if q > 0xFFFF { cpu.sr.set(StatusRegister::V, true); cpu.sr.set(StatusRegister::N, true); cpu.sr.set(StatusRegister::Z, false); return 76; }
    cpu.d[0] = (rm << 16) | (q & 0xFFFF);
    cpu.sr.set_nz(q, Size::Word); cpu.sr.set(StatusRegister::V, false);
    140
}
/// $05D8DE: swap d0
pub fn i_05d8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8E0;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D8E0: andi.w #$f, d0
pub fn i_05d8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8E4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D8E4: lsl.w #$8, d0
pub fn i_05d8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8E6;
    cpu.ir = 0xE148;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $05D8E6: lsl.w #$4, d0
pub fn i_05d8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8E8;
    cpu.ir = 0xE948;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $05D8E8: or.w d0, d3
pub fn i_05d8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8EA;
    cpu.ir = 0x8640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[3]) & 0xFFFF;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05D8EA: move.w d3, d0
pub fn i_05d8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8EC;
    cpu.ir = 0x3003;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05D8EC: movem.l (a7)+, d1-d3
pub fn i_05d8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8F0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05D8F0: rts
pub fn i_05d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Decimal_Clamp99999999_05D8F2  ($05D8F2, 18 B, kernel/math_decimal_clamp_05d8f2.s)
/// $05D8F2: cmpi.l #$5f5e0ff, d0
pub fn i_05d8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8F8;
    cpu.ir = 0xC80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    sub_with_flags(cpu, v1, 0x5F5E0FF, Size::Long, false);
    8
}
/// $05D8F8: bcs.w $5d904
pub fn i_05d8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D8FC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D904; return 10; }
    8
}
/// $05D8FC: move.l #$99999999, d0
pub fn i_05d8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D902;
    cpu.ir = 0x203C;
    let v1: u32 = 0x99999999;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D902: rts
pub fn i_05d902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D904;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_BinToDecimalDecoder_05D904  ($05D904, 28 B, kernel/math_bin_to_decimal_05d904.s)
/// $05D904: moveq #$0, d4
pub fn i_05d904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D906;
    cpu.ir = 0x7800;
    cpu.d[4] = 0x0;
    cpu.sr.set_nz(cpu.d[4], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D906: moveq #$0, d5
pub fn i_05d906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D908;
    cpu.ir = 0x7A00;
    cpu.d[5] = 0x0;
    cpu.sr.set_nz(cpu.d[5], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D908: tst.l d0
pub fn i_05d908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D90A;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D90A: beq.w $5d91c
pub fn i_05d90a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D90E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D91C; return 10; }
    8
}
/// $05D90E: moveq #$a, d1
pub fn i_05d90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D910;
    cpu.ir = 0x720A;
    cpu.d[1] = 0xA;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D910: jsr $5d920(pc)
pub fn i_05d910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D914;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5D914);
    cpu.pc = 0x5D920;
    16
}
/// $05D914: asl.l d5, d2
pub fn i_05d914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D916;
    cpu.ir = 0xEBA2;
    let cnt = cpu.d[5] & 0x3F;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, cnt, Size::Long, 0, true);
    cpu.d[2] = r;
    6 + 2 * cnt
}
/// $05D916: or.l d2, d4
pub fn i_05d916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D918;
    cpu.ir = 0x8882;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r = (v1 | cpu.d[4]) & 0xFFFFFFFF;
    cpu.d[4] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D918: addq.l #$4, d5
pub fn i_05d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D91A;
    cpu.ir = 0x5885;
    let v1 = cpu.d[5] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Long, true);
    cpu.d[5] = r2;
    4
}
/// $05D91A: bra.b $5d908
pub fn i_05d91a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D91C;
    cpu.ir = 0x60EC;
    cpu.pc = 0x5D908;
    10
}
/// $05D91C: move.l d4, d0
pub fn i_05d91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D91E;
    cpu.ir = 0x2004;
    let v1 = cpu.d[4] & 0xFFFFFFFF;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D91E: rts
pub fn i_05d91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D920;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_LongDivide_05D920  ($05D920, 36 B, kernel/math_long_divide_05d920.s)
/// $05D920: moveq #$0, d2
pub fn i_05d920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D922;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D922: tst.l d0
pub fn i_05d922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D924;
    cpu.ir = 0x4A80;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D924: beq.w $5d942
pub fn i_05d924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D928;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D942; return 10; }
    8
}
/// $05D928: tst.l d1
pub fn i_05d928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D92A;
    cpu.ir = 0x4A81;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D92A: beq.w $5d944
pub fn i_05d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D92E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5D944; return 10; }
    8
}
/// $05D92E: moveq #$1f, d3
pub fn i_05d92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D930;
    cpu.ir = 0x761F;
    cpu.d[3] = 0x1F;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05D930: add.l d0, d0
pub fn i_05d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D932;
    cpu.ir = 0xD080;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[0] = r;
    4
}
/// $05D932: addx.l d2, d2
pub fn i_05d932(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D934;
    cpu.ir = 0xD582;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let h2 = cpu.d[2] & 0xFFFFFFFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Long);
    cpu.d[2] = r;
    8
}
/// $05D934: cmp.l d1, d2
pub fn i_05d934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D936;
    cpu.ir = 0xB481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    4
}
/// $05D936: bcs.w $5d93e
pub fn i_05d936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D93A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5D93E; return 10; }
    8
}
/// $05D93A: addq.w #$1, d0
pub fn i_05d93a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D93C;
    cpu.ir = 0x5240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05D93C: sub.l d1, d2
pub fn i_05d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D93E;
    cpu.ir = 0x9481;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Long, true);
    cpu.d[2] = r;
    4
}
/// $05D93E: dbra d3, $5d930
pub fn i_05d93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D942;
    cpu.ir = 0x51CB;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5D942; return 12; }
    let n = (cpu.d[3] as u16).wrapping_sub(1);
    cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5D942; return 14; }
    cpu.pc = 0x5D930;
    10
}
/// $05D942: rts
pub fn i_05d942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D944;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Trap15_DivByZero_05D944  ($05D944, 2 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D944: trap #$f
pub fn i_05d944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D946;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
// ===== Noise_LookupByIndex_05d946  ($05D946, 16 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05D946: move.w $10e22e.l, d0
pub fn i_05d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D94C;
    cpu.ir = 0x3039;
    let v2 = bus.read16(0x10E22E) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05D94C: andi.w #$ff, d0
pub fn i_05d94c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D950;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05D950: move.b $5d956(pc, d0.w), d0
pub fn i_05d950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D954;
    cpu.ir = 0x103B;
    let ad2 = 0x5D956_u32.wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05D954: rts
pub fn i_05d954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5D956;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_BlitRectToFixLayer  ($05DA56, 70 B, video/fix_blit_rect_05da56.s)
/// $05DA56: movem.l d3-d5, -(a7)
pub fn i_05da56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA5A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DA5A: subq.w #$1, d1
pub fn i_05da5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA5C;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DA5C: subq.w #$1, d2
pub fn i_05da5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA5E;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DA5E: move.w d0, d4
pub fn i_05da5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA60;
    cpu.ir = 0x3800;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DA60: move.l a1, d3
pub fn i_05da60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA62;
    cpu.ir = 0x2609;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05DA62: move.w d1, d5
pub fn i_05da62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA64;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DA64: move.w d4, -(a7)
pub fn i_05da64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA66;
    cpu.ir = 0x3F04;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05DA66: cmpi.w #$74ff, d3
pub fn i_05da66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA6A;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $05DA6A: bgt.w $5da7e
pub fn i_05da6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA6E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DA7E; return 10; }
    8
}
/// $05DA6E: cmpi.w #$7000, d3
pub fn i_05da6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA72;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $05DA72: blt.w $5da7e
pub fn i_05da72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA76;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DA7E; return 10; }
    8
}
/// $05DA76: movem.w d3-d4, $3c0000.l
pub fn i_05da76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA7E;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[4] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05DA7E: addq.w #$1, d4
pub fn i_05da7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA80;
    cpu.ir = 0x5244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DA80: addi.w #$20, d3
pub fn i_05da80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA84;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DA84: dbra d5, $5da66
pub fn i_05da84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA88;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5DA88; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5DA88; return 14; }
    cpu.pc = 0x5DA66;
    10
}
/// $05DA88: move.w (a7)+, d4
pub fn i_05da88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA8A;
    cpu.ir = 0x381F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05DA8A: addi.w #$10, d4
pub fn i_05da8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA8E;
    cpu.ir = 0x644;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DA8E: adda.w #$1, a1
pub fn i_05da8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA92;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x1;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05DA92: dbra d2, $5da60
pub fn i_05da92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA96;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5DA96; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5DA96; return 14; }
    cpu.pc = 0x5DA60;
    10
}
/// $05DA96: movem.l (a7)+, d3-d5
pub fn i_05da96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA9A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DA9A: rts
pub fn i_05da9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DA9C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_BlitRect_05DA9C  ($05DA9C, 60 B, video/fix_blit_rect_05da9c.s)
/// $05DA9C: movem.l d3-d5, -(a7)
pub fn i_05da9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA0;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[5]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[4]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DAA0: subq.w #$1, d1
pub fn i_05daa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA2;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DAA2: subq.w #$1, d2
pub fn i_05daa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA4;
    cpu.ir = 0x5342;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DAA4: move.w d0, d4
pub fn i_05daa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA6;
    cpu.ir = 0x3800;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DAA6: move.l a1, d3
pub fn i_05daa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAA8;
    cpu.ir = 0x2609;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[3] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05DAA8: move.w d1, d5
pub fn i_05daa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAAA;
    cpu.ir = 0x3A01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DAAA: cmpi.w #$74ff, d3
pub fn i_05daaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAAE;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $05DAAE: bgt.w $5dac2
pub fn i_05daae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAB2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DAC2; return 10; }
    8
}
/// $05DAB2: cmpi.w #$7000, d3
pub fn i_05dab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAB6;
    cpu.ir = 0xC43;
    let v1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $05DAB6: blt.w $5dac2
pub fn i_05dab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DABA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DAC2; return 10; }
    8
}
/// $05DABA: movem.w d3-d4, $3c0000.l
pub fn i_05daba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAC2;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[3] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[4] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05DAC2: addi.w #$20, d3
pub fn i_05dac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAC6;
    cpu.ir = 0x643;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DAC6: dbra d5, $5daaa
pub fn i_05dac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DACA;
    cpu.ir = 0x51CD;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5DACA; return 12; }
    let n = (cpu.d[5] as u16).wrapping_sub(1);
    cpu.d[5] = (cpu.d[5] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5DACA; return 14; }
    cpu.pc = 0x5DAAA;
    10
}
/// $05DACA: adda.w #$1, a1
pub fn i_05daca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DACE;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x1;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05DACE: dbra d2, $5daa6
pub fn i_05dace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAD2;
    cpu.ir = 0x51CA;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5DAD2; return 12; }
    let n = (cpu.d[2] as u16).wrapping_sub(1);
    cpu.d[2] = (cpu.d[2] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5DAD2; return 14; }
    cpu.pc = 0x5DAA6;
    10
}
/// $05DAD2: movem.l (a7)+, d3-d5
pub fn i_05dad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAD6;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[4] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[5] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DAD6: rts
pub fn i_05dad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAD8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Fix_BlitStream_05DAD8  ($05DAD8, 66 B, video/fix_blit_stream_05dad8.s)
/// $05DAD8: moveq #$0, d2
pub fn i_05dad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DADA;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05DADA: move.l a1, d1
pub fn i_05dada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DADC;
    cpu.ir = 0x2209;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.d[1] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05DADC: move.w d1, -(a7)
pub fn i_05dadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DADE;
    cpu.ir = 0x3F01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad2;
    bus.write16(ad2, v1 as u16);
    10
}
/// $05DADE: move.b (a2)+, d2
pub fn i_05dade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAE0;
    cpu.ir = 0x141A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    8
}
/// $05DAE0: cmpi.b #$fe, d2
pub fn i_05dae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAE4;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0xFE, Size::Byte, false);
    8
}
/// $05DAE4: beq.w $5db16
pub fn i_05dae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DB16; return 10; }
    8
}
/// $05DAE8: cmpi.b #$fd, d2
pub fn i_05dae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAEC;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0xFD, Size::Byte, false);
    8
}
/// $05DAEC: beq.w $5db10
pub fn i_05daec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DB10; return 10; }
    8
}
/// $05DAF0: cmpi.w #$74ff, d1
pub fn i_05daf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAF4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x74FF, Size::Word, false);
    8
}
/// $05DAF4: bgt.w $5db0a
pub fn i_05daf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAF8;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DB0A; return 10; }
    8
}
/// $05DAF8: cmpi.w #$7000, d1
pub fn i_05daf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DAFC;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7000, Size::Word, false);
    8
}
/// $05DAFC: blt.w $5db0a
pub fn i_05dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB00;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DB0A; return 10; }
    8
}
/// $05DB00: or.w d0, d2
pub fn i_05db00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB02;
    cpu.ir = 0x8440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = (v1 | cpu.d[2]) & 0xFFFF;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05DB02: movem.w d1-d2, $3c0000.l
pub fn i_05db02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB0A;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[2] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $05DB0A: addi.w #$20, d1
pub fn i_05db0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB0E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DB0E: bra.b $5dade
pub fn i_05db0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB10;
    cpu.ir = 0x60CE;
    cpu.pc = 0x5DADE;
    10
}
/// $05DB10: move.w (a7)+, d1
pub fn i_05db10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB12;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05DB12: addq.w #$1, d1
pub fn i_05db12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB14;
    cpu.ir = 0x5241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05DB14: bra.b $5dadc
pub fn i_05db14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB16;
    cpu.ir = 0x60C6;
    cpu.pc = 0x5DADC;
    10
}
/// $05DB16: move.w (a7)+, d1
pub fn i_05db16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB18;
    cpu.ir = 0x321F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05DB18: rts
pub fn i_05db18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== FixLayer_MultiRowBlit_05DB1A  ($05DB1A, 34 B, video/fix_layer_backends_05db1a.s)
/// $05DB1A: move.w d0, d3
pub fn i_05db1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB1C;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DB1C: move.w (a2)+, d0
pub fn i_05db1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB1E;
    cpu.ir = 0x301A;
    let ad2 = cpu.a[2]; cpu.a[2] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    8
}
/// $05DB1E: cmpi.w #$ffff, d0
pub fn i_05db1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB22;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $05DB22: bne.w $5db28
pub fn i_05db22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB26;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5DB28; return 10; }
    8
}
/// $05DB26: rts
pub fn i_05db26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05DB28: or.w d3, d0
pub fn i_05db28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB2A;
    cpu.ir = 0x8043;
    let v1 = cpu.d[3] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05DB2A: movem.l d1-d3/a1, -(a7)
pub fn i_05db2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB2E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[3]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[2]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DB2E: jsr $5da56(pc)
pub fn i_05db2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DB32);
    cpu.pc = 0x5DA56;
    16
}
/// $05DB32: movem.l (a7)+, d1-d3/a1
pub fn i_05db32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB36;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[2] = bus.read32(a); a = a.wrapping_add(4);
    cpu.d[3] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DB36: adda.w #$40, a1
pub fn i_05db36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB3A;
    cpu.ir = 0xD2FC;
    let v1: u32 = 0x40;
    cpu.a[1] = cpu.a[1].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05DB3A: bra.b $5db1c
pub fn i_05db3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB3C;
    cpu.ir = 0x60E0;
    cpu.pc = 0x5DB1C;
    10
}
// ===== CompareField10_CCR_05DB3C  ($05DB3C, 28 B, video/fix_layer_backends_05db1a.s)
/// $05DB3C: movea.l $8(a6), a1
pub fn i_05db3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB40;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DB40: move.b $10(a6), d0
pub fn i_05db40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB44;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05DB44: cmp.b $10(a1), d0
pub fn i_05db44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB48;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05DB48: bcs.w $5db52
pub fn i_05db48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB4C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5DB52; return 10; }
    8
}
/// $05DB4C: andi.b #$ee, ccr
pub fn i_05db4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB50;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05DB50: rts
pub fn i_05db50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05DB52: ori.b #$11, ccr
pub fn i_05db52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB56;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05DB56: rts
pub fn i_05db56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InstallListPubHead_05DB58  ($05DB58, 18 B, video/fix_layer_backends_05db1a.s)
/// $05DB58: move.l a0, $3c(a6)
pub fn i_05db58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB5C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05DB5C: move.w $6(a0), d0
pub fn i_05db5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB60;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DB60: move.w d0, $46(a6)
pub fn i_05db60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB64;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DB64: jsr $5dbc2(pc)
pub fn i_05db64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DB68);
    cpu.pc = 0x5DBC2;
    16
}
/// $05DB68: rts
pub fn i_05db68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ListCursor_Step_05db6a  ($05DB6A, 54 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DB6A: movea.l $3c(a6), a1
pub fn i_05db6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB6E;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DB6E: cmpi.w #$ffff, (a1)
pub fn i_05db6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB72;
    cpu.ir = 0xC51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0xFFFF, Size::Word, false);
    12
}
/// $05DB72: beq.w $5dbbc
pub fn i_05db72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB76;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DBBC; return 10; }
    8
}
/// $05DB76: subq.w #$1, $46(a6)
pub fn i_05db76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB7A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DB7A: cmpi.w #$0, $46(a6)
pub fn i_05db7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB80;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $05DB80: bgt.w $5dba0
pub fn i_05db80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB84;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DBA0; return 10; }
    8
}
/// $05DB84: jsr $5dbdc(pc)
pub fn i_05db84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB88;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DB88);
    cpu.pc = 0x5DBDC;
    16
}
/// $05DB88: addq.l #$8, $3c(a6)
pub fn i_05db88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB8C;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x3C);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $05DB8C: movea.l $3c(a6), a1
pub fn i_05db8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB90;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DB90: cmpi.w #$0, (a1)
pub fn i_05db90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB94;
    cpu.ir = 0xC51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Word, false);
    12
}
/// $05DB94: bne.w $5dba6
pub fn i_05db94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5DBA6; return 10; }
    8
}
/// $05DB98: movea.l $2(a1), a0
pub fn i_05db98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DB9C;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05DB9C: jsr $5db58(pc)
pub fn i_05db9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DBA0);
    cpu.pc = 0x5DB58;
    16
}
// ===== ClearXNV_05dba0  ($05DBA0, 6 B, thunks/ccr_helpers.c)
/// $05DBA0: andi.b #$e, ccr
pub fn i_05dba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBA4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05DBA4: rts
pub fn i_05dba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBA6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ListCursor_LoadEntry_05dba6  ($05DBA6, 22 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DBA6: cmpi.w #$ffff, (a1)
pub fn i_05dba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBAA;
    cpu.ir = 0xC51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0xFFFF, Size::Word, false);
    12
}
/// $05DBAA: beq.w $5dbbc
pub fn i_05dbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DBBC; return 10; }
    8
}
/// $05DBAE: move.w $6(a1), d0
pub fn i_05dbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBB2;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBB2: move.w d0, $46(a6)
pub fn i_05dbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBB6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DBB6: jsr $5dbc2(pc)
pub fn i_05dbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBBA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5DBBA);
    cpu.pc = 0x5DBC2;
    16
}
/// $05DBBA: bra.b $5dba0
pub fn i_05dbba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBBC;
    cpu.ir = 0x60E4;
    cpu.pc = 0x5DBA0;
    10
}
// ===== SetC_05dbbc  ($05DBBC, 6 B, thunks/ccr_helpers.c)
/// $05DBBC: ori.b #$1, ccr
pub fn i_05dbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBC0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05DBC0: rts
pub fn i_05dbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ListCursor_Reinit_05DBC2  ($05DBC2, 26 B, video/camera_list_ctx_helpers_025012.s)
/// $05DBC2: movea.l $3c(a6), a0
pub fn i_05dbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBC6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05DBC6: movea.w $22(a6), a1
pub fn i_05dbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBCA;
    cpu.ir = 0x326E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = v3 as i16 as i32 as u32;
    12
}
/// $05DBCA: move.w (a0), d0
pub fn i_05dbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBCC;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05DBCC: move.w $2(a0), d1
pub fn i_05dbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBD0;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBD0: move.w $4(a0), d2
pub fn i_05dbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBD4;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBD4: jsr $5da56.l
pub fn i_05dbd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DBDA);
    cpu.pc = 0x5DA56;
    16
}
/// $05DBDA: rts
pub fn i_05dbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBDC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ListCursor_ReinitClipped_05DBDC  ($05DBDC, 36 B, video/camera_list_ctx_helpers_025012.s)
/// $05DBDC: movea.l $3c(a6), a0
pub fn i_05dbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBE0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05DBE0: cmpi.w #$ffff, (a0)
pub fn i_05dbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBE4;
    cpu.ir = 0xC50;
    let v2 = bus.read16(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0xFFFF, Size::Word, false);
    12
}
/// $05DBE4: beq.w $5dbfe
pub fn i_05dbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DBFE; return 10; }
    8
}
/// $05DBE8: movea.w $22(a6), a1
pub fn i_05dbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBEC;
    cpu.ir = 0x326E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[1] = v3 as i16 as i32 as u32;
    12
}
/// $05DBEC: move.w #$ff, d0
pub fn i_05dbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBF0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DBF0: move.w $2(a0), d1
pub fn i_05dbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBF4;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBF4: move.w $4(a0), d2
pub fn i_05dbf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBF8;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05DBF8: jsr $5da9c.l
pub fn i_05dbf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DBFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DBFE);
    cpu.pc = 0x5DA9C;
    16
}
/// $05DBFE: rts
pub fn i_05dbfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC00;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== CompareField10_CCR_05DC00  ($05DC00, 28 B, video/camera_list_ctx_helpers_025012.s)
/// $05DC00: movea.l $8(a6), a1
pub fn i_05dc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC04;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DC04: move.b $10(a6), d0
pub fn i_05dc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC08;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05DC08: cmp.b $10(a1), d0
pub fn i_05dc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC0C;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $05DC0C: bcs.w $5dc16
pub fn i_05dc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC10;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5DC16; return 10; }
    8
}
/// $05DC10: andi.b #$ee, ccr
pub fn i_05dc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC14;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05DC14: rts
pub fn i_05dc14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC16;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05DC16: ori.b #$11, ccr
pub fn i_05dc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC1A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05DC1A: rts
pub fn i_05dc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ClearPtrSlots_05DC1C  ($05DC1C, 24 B, entity/clear_ptr_slots_05dc1c.s)
/// $05DC1C: move.w #$9c, d0
pub fn i_05dc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC20;
    cpu.ir = 0x303C;
    let v1: u32 = 0x9C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DC20: move.l #$ffffffff, (a0, d0.w)
pub fn i_05dc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC28;
    cpu.ir = 0x21BC;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    18
}
/// $05DC28: subi.w #$4, d0
pub fn i_05dc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC2C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05DC2C: cmpi.w #$10, d0
pub fn i_05dc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC30;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x10, Size::Word, false);
    8
}
/// $05DC30: bcc.b $5dc20
pub fn i_05dc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC32;
    cpu.ir = 0x64EE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5DC20; return 10; }
    8
}
/// $05DC32: rts
pub fn i_05dc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_InitFields_05DC34  ($05DC34, 112 B, entity/init_fields_05dc34.s)
/// $05DC34: clr.w $14(a0)
pub fn i_05dc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC38;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x14);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DC38: move.w #$ffff, $16(a0)
pub fn i_05dc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC3E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x16);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DC3E: move.w #$ffff, $18(a0)
pub fn i_05dc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC44;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x18);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DC44: move.w #$ffff, $1a(a0)
pub fn i_05dc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC4A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DC4A: move.w #$ffff, $1c(a0)
pub fn i_05dc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC50;
    cpu.ir = 0x317C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DC50: clr.w $1e(a0)
pub fn i_05dc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC54;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x1E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DC54: clr.b $12(a0)
pub fn i_05dc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC58;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC58: clr.w $38(a0)
pub fn i_05dc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC5C;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DC5C: clr.b $13(a0)
pub fn i_05dc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC60;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC60: clr.b $5a(a0)
pub fn i_05dc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC64;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC64: clr.b $69(a0)
pub fn i_05dc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC68;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC68: clr.b $5b(a0)
pub fn i_05dc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC6C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC6C: clr.b $6b(a0)
pub fn i_05dc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC70;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC70: move.b #$ff, $32(a0)
pub fn i_05dc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC76;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05DC76: move.b #$ff, $33(a0)
pub fn i_05dc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC7C;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05DC7C: clr.b $3a(a0)
pub fn i_05dc7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC80;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC80: clr.l $28(a0)
pub fn i_05dc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC84;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x28);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $05DC84: clr.l $2c(a0)
pub fn i_05dc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC88;
    cpu.ir = 0x42A8;
    let ad1 = cpu.a[0].wrapping_add(0x2C);
    let v2 = bus.read32(ad1);
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write32(ad1, r3);
    16
}
/// $05DC88: clr.w $26(a0)
pub fn i_05dc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC8C;
    cpu.ir = 0x4268;
    let ad1 = cpu.a[0].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $05DC8C: clr.b $59(a0)
pub fn i_05dc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC90;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x59);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC90: clr.b $45(a0)
pub fn i_05dc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC94;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x45);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC94: clr.b $44(a0)
pub fn i_05dc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC98;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x44);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC98: clr.b $6a(a0)
pub fn i_05dc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DC9C;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x6A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $05DC9C: bset.b #$6, $13(a0)
pub fn i_05dc9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCA2;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05DCA2: rts
pub fn i_05dca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_NegIfFacing_05dca4  ($05DCA4, 12 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DCA4: btst.b #$0, $3a(a6)
pub fn i_05dca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCAA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05DCAA: beq.w $5dcb0
pub fn i_05dcaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCAE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DCB0; return 10; }
    8
}
/// $05DCAE: neg.w d0
pub fn i_05dcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCB0;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_05dcb0  ($05DCB0, 6 B, kernel/task_setters_w.c)
/// $05DCB0: move.w d0, $28(a6)
pub fn i_05dcb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCB4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DCB4: rts
pub fn i_05dcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCB6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SetVx_NegIfFacing_05dcb6  ($05DCB6, 18 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DCB6: move.w d0, $36(a6)
pub fn i_05dcb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCBA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DCBA: btst.b #$0, $3a(a6)
pub fn i_05dcba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCC0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05DCC0: beq.w $5dcc8
pub fn i_05dcc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DCC8; return 10; }
    8
}
/// $05DCC4: neg.w $28(a6)
pub fn i_05dcc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCC8;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetTaskW_05dcc8  ($05DCC8, 6 B, kernel/task_setters_w.c)
/// $05DCC8: move.w d0, $28(a6)
pub fn i_05dcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCCC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05DCCC: rts
pub fn i_05dccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Spawn_ChildFromDesc_05dcce  ($05DCCE, 52 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DCCE: movem.l a1/a6, -(a7)
pub fn i_05dcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCD2;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DCD2: cmpi.b #$2, (a1)
pub fn i_05dcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCD6;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $05DCD6: beq.w $5dcee
pub fn i_05dcd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DCEE; return 10; }
    8
}
/// $05DCDA: lea.l $1008a0.l, a6
pub fn i_05dcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCE0;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $05DCE0: movea.l $6(a1), a1
pub fn i_05dce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCE4;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DCE4: jsr $4ae.l
pub fn i_05dce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DCEA);
    cpu.pc = 0x4AE;
    16
}
/// $05DCEA: bra.w $5dcf8
pub fn i_05dcea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5DCF8;
    10
}
/// $05DCEE: movea.l $6(a1), a1
pub fn i_05dcee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCF2;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $05DCF2: jsr $4498e.l
pub fn i_05dcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DCF8);
    cpu.pc = 0x4498E;
    16
}
/// $05DCF8: movem.l (a7)+, a1/a6
pub fn i_05dcf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DCFC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DCFC: move.l a1, $3c(a0)
pub fn i_05dcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD00;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $05DD00: rts
pub fn i_05dd00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CopyTransform  ($05DD02, 32 B, entity/copy_transform_05dd02.s)
/// $05DD02: move.w $22(a6), $22(a0)
pub fn i_05dd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD08;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05DD08: move.w $24(a6), $24(a0)
pub fn i_05dd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD0E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05DD0E: move.w $38(a6), $38(a0)
pub fn i_05dd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD14;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05DD14: move.b $3a(a6), $3a(a0)
pub fn i_05dd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD1A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05DD1A: move.b $11(a6), $11(a0)
pub fn i_05dd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD20;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x11);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x11);
    bus.write8(ad4, v3 as u8);
    20
}
/// $05DD20: rts
pub fn i_05dd20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD22;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Handler_ApplyCameraSelf_CopyTransform_05dd22  ($05DD22, 8 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DD22: jsr $440e4.l
pub fn i_05dd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DD28);
    cpu.pc = 0x440E4;
    16
}
/// $05DD28: bra.b $5dd02
pub fn i_05dd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD2A;
    cpu.ir = 0x60D8;
    cpu.pc = 0x5DD02;
    10
}
// ===== Task_WalkTwoArenas_05DD2A  ($05DD2A, 34 B, kernel/task_walk_two_arenas_05dd2a.s)
/// $05DD2A: movem.l a6, -(a7)
pub fn i_05dd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD2E;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[6]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $05DD2E: lea.l $100800.l, a6
pub fn i_05dd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD34;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x100800;
    20
}
/// $05DD34: jsr $5b6.l
pub fn i_05dd34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DD3A);
    cpu.pc = 0x5B6;
    16
}
/// $05DD3A: lea.l $1008a0.l, a6
pub fn i_05dd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD40;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $05DD40: jsr $5b6.l
pub fn i_05dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5DD46);
    cpu.pc = 0x5B6;
    16
}
/// $05DD46: movem.l (a7)+, a6
pub fn i_05dd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD4A;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[6] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $05DD4A: rts
pub fn i_05dd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD4C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SetOffscreenFlag_05dd56  ($05DD56, 6 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DD56: bset.b #$7, $13(a6)
pub fn i_05dd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD5C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== Entity_CheckOnScreenBox_05dd5c  ($05DD5C, 86 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DD5C: cmpa.l #$ffffffff, a0
pub fn i_05dd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD62;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $05DD62: bne.w $5dd6a
pub fn i_05dd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD66;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5DD6A; return 10; }
    8
}
/// $05DD66: lea.l $5dd4c(pc), a0
pub fn i_05dd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD6A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5DD4C;
    16
}
/// $05DD6A: btst.b #$7, $13(a6)
pub fn i_05dd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD70;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05DD70: beq.w $5ddbe
pub fn i_05dd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5DDBE; return 10; }
    8
}
/// $05DD74: move.w #$0, d0
pub fn i_05dd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD78;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DD78: move.w #$140, d1
pub fn i_05dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD7C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DD7C: move.w #$100, d2
pub fn i_05dd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD80;
    cpu.ir = 0x343C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DD80: move.w #$1f0, d3
pub fn i_05dd80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD84;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1F0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05DD84: add.w (a0), d0
pub fn i_05dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD86;
    cpu.ir = 0xD050;
    let v2 = bus.read16(cpu.a[0]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05DD86: add.w $2(a0), d1
pub fn i_05dd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD8A;
    cpu.ir = 0xD268;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05DD8A: sub.w $6(a0), d2
pub fn i_05dd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD8E;
    cpu.ir = 0x9468;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05DD8E: sub.w $4(a0), d3
pub fn i_05dd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD92;
    cpu.ir = 0x9668;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05DD92: cmp.w $22(a6), d0
pub fn i_05dd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD96;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05DD96: bgt.w $5ddb8
pub fn i_05dd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD9A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DDB8; return 10; }
    8
}
/// $05DD9A: cmp.w $22(a6), d1
pub fn i_05dd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DD9E;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05DD9E: blt.w $5ddb8
pub fn i_05dd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDA2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DDB8; return 10; }
    8
}
/// $05DDA2: cmp.w $24(a6), d2
pub fn i_05dda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDA6;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05DDA6: bgt.w $5ddb8
pub fn i_05dda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDAA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5DDB8; return 10; }
    8
}
/// $05DDAA: cmp.w $24(a6), d3
pub fn i_05ddaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDAE;
    cpu.ir = 0xB66E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05DDAE: blt.w $5ddb8
pub fn i_05ddae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDB2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5DDB8; return 10; }
    8
}
// ===== ClearC_05ddb2  ($05DDB2, 6 B, thunks/ccr_helpers.c)
/// $05DDB2: andi.b #$fe, ccr
pub fn i_05ddb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDB6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05DDB6: rts
pub fn i_05ddb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_05ddb8  ($05DDB8, 6 B, thunks/ccr_helpers.c)
/// $05DDB8: ori.b #$1, ccr
pub fn i_05ddb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDBC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $05DDBC: rts
pub fn i_05ddbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDBE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CheckEnterScreen_05ddbe  ($05DDBE, 46 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DDBE: cmpi.w #$140, $22(a6)
pub fn i_05ddbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $05DDC4: bge.w $5ddf2
pub fn i_05ddc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDC8;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5DDF2; return 10; }
    8
}
/// $05DDC8: cmpi.w #$0, $22(a6)
pub fn i_05ddc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDCE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $05DDCE: ble.w $5ddf2
pub fn i_05ddce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDD2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5DDF2; return 10; }
    8
}
/// $05DDD2: cmpi.w #$1f0, $24(a6)
pub fn i_05ddd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDD8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1F0, Size::Word, false);
    16
}
/// $05DDD8: bge.w $5ddf2
pub fn i_05ddd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDDC;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5DDF2; return 10; }
    8
}
/// $05DDDC: cmpi.w #$100, $24(a6)
pub fn i_05dddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDE2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $05DDE2: ble.w $5ddf2
pub fn i_05dde2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDE6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5DDF2; return 10; }
    8
}
/// $05DDE6: bset.b #$7, $13(a6)
pub fn i_05dde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDEC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== ClearC_05ddec  ($05DDEC, 6 B, thunks/ccr_helpers.c)
/// $05DDEC: andi.b #$fe, ccr
pub fn i_05ddec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDF0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05DDF0: rts
pub fn i_05ddf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CheckLeaveScreenWide_05ddf2  ($05DDF2, 32 B, input/thunks_debug_atan_luts_05cae0.s)
/// $05DDF2: cmpi.w #$ff80, $22(a6)
pub fn i_05ddf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDF8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF80, Size::Word, false);
    16
}
/// $05DDF8: ble.b $5dde6
pub fn i_05ddf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DDFA;
    cpu.ir = 0x6FEC;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5DDE6; return 10; }
    8
}
/// $05DDFA: cmpi.w #$200, $22(a6)
pub fn i_05ddfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE00;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x200, Size::Word, false);
    16
}
/// $05DE00: bge.b $5dde6
pub fn i_05de00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE02;
    cpu.ir = 0x6CE4;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5DDE6; return 10; }
    8
}
/// $05DE02: cmpi.w #$0, $24(a6)
pub fn i_05de02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE08;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $05DE08: ble.b $5dde6
pub fn i_05de08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE0A;
    cpu.ir = 0x6FDC;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x5DDE6; return 10; }
    8
}
/// $05DE0A: cmpi.w #$280, $24(a6)
pub fn i_05de0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE10;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x280, Size::Word, false);
    16
}
/// $05DE10: bge.b $5dde6
pub fn i_05de10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE12;
    cpu.ir = 0x6CD4;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5DDE6; return 10; }
    8
}
// ===== ClearC_05de12  ($05DE12, 6 B, thunks/ccr_helpers.c)
/// $05DE12: andi.b #$fe, ccr
pub fn i_05de12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE16;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $05DE16: rts
pub fn i_05de16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5DE18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Atan2_Angle256_05e018  ($05E018, 88 B, props/late_props_turrets_05e000.s)
/// $05E018: asr.w #$1, d0
pub fn i_05e018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E01A;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E01A: smi.b d2
pub fn i_05e01a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E01C;
    cpu.ir = 0x5BC2;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[2] & 0xFF;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05E01C: eor.b d2, d0
pub fn i_05e01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E01E;
    cpu.ir = 0xB500;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E01E: asr.w #$1, d1
pub fn i_05e01e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E020;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E020: smi.b d3
pub fn i_05e020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E022;
    cpu.ir = 0x5BC3;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[3] & 0xFF;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05E022: eor.b d3, d1
pub fn i_05e022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E024;
    cpu.ir = 0xB701;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[3]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E024: move.w #$ff, d4
pub fn i_05e024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E028;
    cpu.ir = 0x383C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E028: and.w d4, d0
pub fn i_05e028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E02A;
    cpu.ir = 0xC044;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E02A: and.w d4, d1
pub fn i_05e02a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E02C;
    cpu.ir = 0xC244;
    let v1 = cpu.d[4] & 0xFFFF;
    let r = (v1 & cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E02C: cmp.b d1, d0
pub fn i_05e02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E02E;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05E02E: smi.b d4
pub fn i_05e02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E030;
    cpu.ir = 0x5BC4;
    let c1: u32 = if cpu.sr.evaluate(Condition::Mi) { 0xFF } else { 0 };
    let v2 = cpu.d[4] & 0xFF;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (c1 & 0xFF);
    4
}
/// $05E030: bcs.w $5e036
pub fn i_05e030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E034;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E036; return 10; }
    8
}
/// $05E034: exg.l d1, d0
pub fn i_05e034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E036;
    cpu.ir = 0xC340;
    cpu.d.swap(1, 0);
    6
}
/// $05E036: tst.b d0
pub fn i_05e036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E038;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05E038: bne.w $5e046
pub fn i_05e038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E03C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E046; return 10; }
    8
}
/// $05E03C: move.b d4, d0
pub fn i_05e03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E03E;
    cpu.ir = 0x1004;
    let v1 = cpu.d[4] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $05E03E: andi.b #$40, d0
pub fn i_05e03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E042;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x40) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E042: bra.w $5e062
pub fn i_05e042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E046;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E062;
    10
}
/// $05E046: lea.l $5de18(pc), a0
pub fn i_05e046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E04A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5DE18;
    16
}
/// $05E04A: move.b (a0, d0.w), d0
pub fn i_05e04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E04E;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05E04E: sub.b (a0, d1.w), d0
pub fn i_05e04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E052;
    cpu.ir = 0x9030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    14
}
/// $05E052: lea.l $5df18(pc), a0
pub fn i_05e052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E056;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5DF18;
    16
}
/// $05E056: move.b (a0, d0.w), d0
pub fn i_05e056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E05A;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $05E05A: eor.b d4, d0
pub fn i_05e05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E05C;
    cpu.ir = 0xB900;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E05C: sub.b d4, d0
pub fn i_05e05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E05E;
    cpu.ir = 0x9004;
    let v1 = cpu.d[4] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $05E05E: addi.b #$20, d0
pub fn i_05e05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E062;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E062: eor.b d2, d3
pub fn i_05e062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E064;
    cpu.ir = 0xB503;
    let v1 = cpu.d[3] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E064: eor.b d3, d0
pub fn i_05e064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E066;
    cpu.ir = 0xB700;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[3]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $05E066: sub.b d3, d0
pub fn i_05e066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E068;
    cpu.ir = 0x9003;
    let v1 = cpu.d[3] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $05E068: andi.b #$80, d2
pub fn i_05e068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E06C;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x80) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E06C: add.b d2, d0
pub fn i_05e06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E06E;
    cpu.ir = 0xD002;
    let v1 = cpu.d[2] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v1, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    4
}
/// $05E06E: rts
pub fn i_05e06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E070;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_DeltaThenAtan2_05e070  ($05E070, 16 B, props/late_props_turrets_05e000.s)
/// $05E070: move.w $22(a0), d0
pub fn i_05e070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E074;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E074: move.w $24(a0), d1
pub fn i_05e074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E078;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E078: sub.w $22(a6), d0
pub fn i_05e078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E07C;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E07C: sub.w $24(a6), d1
pub fn i_05e07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E080;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
// ===== JsrPcThunk_05e080  ($05E080, 6 B, thunks/jsr_pc_thunks.c)
/// $05E080: jsr $5e018(pc)
pub fn i_05e080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E084;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E084);
    cpu.pc = 0x5E018;
    16
}
/// $05E084: rts
pub fn i_05e084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E086;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_AcquireNearestPlayer_05e086  ($05E086, 150 B, props/late_props_turrets_05e000.s)
/// $05E086: move.l a0, -(a7)
pub fn i_05e086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E088;
    cpu.ir = 0x2F08;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05E088: jsr $5e1aa(pc)
pub fn i_05e088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E08C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E08C);
    cpu.pc = 0x5E1AA;
    16
}
/// $05E08C: bcs.w $5e0ce
pub fn i_05e08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E090;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x5E0CE; return 10; }
    8
}
/// $05E090: lea.l $100440.l, a2
pub fn i_05e090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E096;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x100440;
    20
}
/// $05E096: lea.l $1004e0.l, a3
pub fn i_05e096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E09C;
    cpu.ir = 0x47F9;
    cpu.a[3] = 0x1004E0;
    20
}
/// $05E09C: movea.l (a7)+, a1
pub fn i_05e09c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E09E;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $05E09E: btst.b #$0, d0
pub fn i_05e09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0A2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05E0A2: beq.w $5e0b4
pub fn i_05e0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E0B4; return 10; }
    8
}
/// $05E0A6: movea.l a2, a0
pub fn i_05e0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0A8;
    cpu.ir = 0x204A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05E0A8: jsr $5e260(pc)
pub fn i_05e0a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0AC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E0AC);
    cpu.pc = 0x5E260;
    16
}
/// $05E0AC: bcc.w $5e0b4
pub fn i_05e0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E0B4; return 10; }
    8
}
/// $05E0B0: eori.b #$1, d0
pub fn i_05e0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0B4;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E0B4: btst.b #$1, d0
pub fn i_05e0b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0B8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $05E0B8: beq.w $5e0ca
pub fn i_05e0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0BC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E0CA; return 10; }
    8
}
/// $05E0BC: movea.l a3, a0
pub fn i_05e0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0BE;
    cpu.ir = 0x204B;
    let v1 = cpu.a[3] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $05E0BE: jsr $5e260(pc)
pub fn i_05e0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E0C2);
    cpu.pc = 0x5E260;
    16
}
/// $05E0C2: bcc.w $5e0ca
pub fn i_05e0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E0CA; return 10; }
    8
}
/// $05E0C6: eori.b #$2, d0
pub fn i_05e0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0CA;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05E0CA: bra.w $5e0d8
pub fn i_05e0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0CE;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E0D8;
    10
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x05C6A0, i_05c6a0),
    (0x05C6A4, i_05c6a4),
    (0x05C6A6, i_05c6a6),
    (0x05C6A8, i_05c6a8),
    (0x05C6AC, i_05c6ac),
    (0x05C6B2, i_05c6b2),
    (0x05C6B4, i_05c6b4),
    (0x05C6B6, i_05c6b6),
    (0x05C6BA, i_05c6ba),
    (0x05C6BC, i_05c6bc),
    (0x05C6BE, i_05c6be),
    (0x05C6C2, i_05c6c2),
    (0x05C6C4, i_05c6c4),
    (0x05C6C6, i_05c6c6),
    (0x05C6CA, i_05c6ca),
    (0x05C6CC, i_05c6cc),
    (0x05C6CE, i_05c6ce),
    (0x05C6D0, i_05c6d0),
    (0x05C6D2, i_05c6d2),
    (0x05C6D6, i_05c6d6),
    (0x05C6D8, i_05c6d8),
    (0x05C6DA, i_05c6da),
    (0x05C6DC, i_05c6dc),
    (0x05C6DE, i_05c6de),
    (0x05C6E2, i_05c6e2),
    (0x05C6E4, i_05c6e4),
    (0x05C6E8, i_05c6e8),
    (0x05C6EA, i_05c6ea),
    (0x05C6EC, i_05c6ec),
    (0x05C6F0, i_05c6f0),
    (0x05C6F4, i_05c6f4),
    (0x05C6F8, i_05c6f8),
    (0x05C6FC, i_05c6fc),
    (0x05C700, i_05c700),
    (0x05C704, i_05c704),
    (0x05C708, i_05c708),
    (0x05C70C, i_05c70c),
    (0x05C710, i_05c710),
    (0x05C714, i_05c714),
    (0x05C718, i_05c718),
    (0x05C71C, i_05c71c),
    (0x05C720, i_05c720),
    (0x05C724, i_05c724),
    (0x05C728, i_05c728),
    (0x05C72C, i_05c72c),
    (0x05C730, i_05c730),
    (0x05C734, i_05c734),
    (0x05C738, i_05c738),
    (0x05C73C, i_05c73c),
    (0x05C740, i_05c740),
    (0x05C744, i_05c744),
    (0x05C748, i_05c748),
    (0x05C74C, i_05c74c),
    (0x05C750, i_05c750),
    (0x05C754, i_05c754),
    (0x05C758, i_05c758),
    (0x05C75C, i_05c75c),
    (0x05C760, i_05c760),
    (0x05C764, i_05c764),
    (0x05C768, i_05c768),
    (0x05C76C, i_05c76c),
    (0x05C76E, i_05c76e),
    (0x05C770, i_05c770),
    (0x05C772, i_05c772),
    (0x05C774, i_05c774),
    (0x05C776, i_05c776),
    (0x05C778, i_05c778),
    (0x05C77A, i_05c77a),
    (0x05C77C, i_05c77c),
    (0x05C77E, i_05c77e),
    (0x05C780, i_05c780),
    (0x05C782, i_05c782),
    (0x05C784, i_05c784),
    (0x05C786, i_05c786),
    (0x05C788, i_05c788),
    (0x05C78A, i_05c78a),
    (0x05C78C, i_05c78c),
    (0x05C78E, i_05c78e),
    (0x05C790, i_05c790),
    (0x05C792, i_05c792),
    (0x05C794, i_05c794),
    (0x05C796, i_05c796),
    (0x05C798, i_05c798),
    (0x05C79A, i_05c79a),
    (0x05C79C, i_05c79c),
    (0x05C79E, i_05c79e),
    (0x05C7A0, i_05c7a0),
    (0x05C7A2, i_05c7a2),
    (0x05C7A4, i_05c7a4),
    (0x05C7A6, i_05c7a6),
    (0x05C7A8, i_05c7a8),
    (0x05C7AA, i_05c7aa),
    (0x05C7AC, i_05c7ac),
    (0x05C7AE, i_05c7ae),
    (0x05C7B0, i_05c7b0),
    (0x05C7B2, i_05c7b2),
    (0x05C7B4, i_05c7b4),
    (0x05C7B6, i_05c7b6),
    (0x05C7B8, i_05c7b8),
    (0x05C7BA, i_05c7ba),
    (0x05C7BC, i_05c7bc),
    (0x05C7BE, i_05c7be),
    (0x05C7C0, i_05c7c0),
    (0x05C7C2, i_05c7c2),
    (0x05C7C4, i_05c7c4),
    (0x05C7C6, i_05c7c6),
    (0x05C7C8, i_05c7c8),
    (0x05C7CA, i_05c7ca),
    (0x05C7CC, i_05c7cc),
    (0x05C7CE, i_05c7ce),
    (0x05C7D0, i_05c7d0),
    (0x05C7D2, i_05c7d2),
    (0x05C7D4, i_05c7d4),
    (0x05C7D6, i_05c7d6),
    (0x05C7D8, i_05c7d8),
    (0x05C7DA, i_05c7da),
    (0x05C7DC, i_05c7dc),
    (0x05C7DE, i_05c7de),
    (0x05C7E0, i_05c7e0),
    (0x05C7E2, i_05c7e2),
    (0x05C7E4, i_05c7e4),
    (0x05C7E6, i_05c7e6),
    (0x05C7E8, i_05c7e8),
    (0x05C7EA, i_05c7ea),
    (0x05C7EC, i_05c7ec),
    (0x05C7EE, i_05c7ee),
    (0x05C7F0, i_05c7f0),
    (0x05C7F2, i_05c7f2),
    (0x05C7F4, i_05c7f4),
    (0x05C7F6, i_05c7f6),
    (0x05C7F8, i_05c7f8),
    (0x05C7FA, i_05c7fa),
    (0x05C7FC, i_05c7fc),
    (0x05C7FE, i_05c7fe),
    (0x05C800, i_05c800),
    (0x05C802, i_05c802),
    (0x05C804, i_05c804),
    (0x05C806, i_05c806),
    (0x05C808, i_05c808),
    (0x05C80A, i_05c80a),
    (0x05C80C, i_05c80c),
    (0x05C80E, i_05c80e),
    (0x05C810, i_05c810),
    (0x05C812, i_05c812),
    (0x05C814, i_05c814),
    (0x05C816, i_05c816),
    (0x05C818, i_05c818),
    (0x05C81A, i_05c81a),
    (0x05C81C, i_05c81c),
    (0x05C81E, i_05c81e),
    (0x05C820, i_05c820),
    (0x05C822, i_05c822),
    (0x05C824, i_05c824),
    (0x05C826, i_05c826),
    (0x05C828, i_05c828),
    (0x05C82A, i_05c82a),
    (0x05C82C, i_05c82c),
    (0x05C82E, i_05c82e),
    (0x05C830, i_05c830),
    (0x05C832, i_05c832),
    (0x05C834, i_05c834),
    (0x05C836, i_05c836),
    (0x05C838, i_05c838),
    (0x05C83A, i_05c83a),
    (0x05C83C, i_05c83c),
    (0x05C83E, i_05c83e),
    (0x05C840, i_05c840),
    (0x05C842, i_05c842),
    (0x05C844, i_05c844),
    (0x05C846, i_05c846),
    (0x05C848, i_05c848),
    (0x05C84A, i_05c84a),
    (0x05C84C, i_05c84c),
    (0x05C84E, i_05c84e),
    (0x05C850, i_05c850),
    (0x05C852, i_05c852),
    (0x05C854, i_05c854),
    (0x05C856, i_05c856),
    (0x05C858, i_05c858),
    (0x05C85A, i_05c85a),
    (0x05C85C, i_05c85c),
    (0x05C85E, i_05c85e),
    (0x05C860, i_05c860),
    (0x05C862, i_05c862),
    (0x05C864, i_05c864),
    (0x05C866, i_05c866),
    (0x05C868, i_05c868),
    (0x05C86A, i_05c86a),
    (0x05C86C, i_05c86c),
    (0x05C86E, i_05c86e),
    (0x05C870, i_05c870),
    (0x05C872, i_05c872),
    (0x05C874, i_05c874),
    (0x05C876, i_05c876),
    (0x05C878, i_05c878),
    (0x05C87A, i_05c87a),
    (0x05C87C, i_05c87c),
    (0x05C87E, i_05c87e),
    (0x05C880, i_05c880),
    (0x05C882, i_05c882),
    (0x05C884, i_05c884),
    (0x05C886, i_05c886),
    (0x05C888, i_05c888),
    (0x05C88A, i_05c88a),
    (0x05C88C, i_05c88c),
    (0x05C88E, i_05c88e),
    (0x05C890, i_05c890),
    (0x05C892, i_05c892),
    (0x05C894, i_05c894),
    (0x05C896, i_05c896),
    (0x05C898, i_05c898),
    (0x05C89A, i_05c89a),
    (0x05C89C, i_05c89c),
    (0x05C89E, i_05c89e),
    (0x05C8A0, i_05c8a0),
    (0x05C8A2, i_05c8a2),
    (0x05C8A4, i_05c8a4),
    (0x05C8A6, i_05c8a6),
    (0x05C8A8, i_05c8a8),
    (0x05C8AA, i_05c8aa),
    (0x05C8AC, i_05c8ac),
    (0x05C8AE, i_05c8ae),
    (0x05C8B0, i_05c8b0),
    (0x05C8B2, i_05c8b2),
    (0x05C8B4, i_05c8b4),
    (0x05C8B8, i_05c8b8),
    (0x05C8BA, i_05c8ba),
    (0x05C8BC, i_05c8bc),
    (0x05C8BE, i_05c8be),
    (0x05C8C0, i_05c8c0),
    (0x05C8C2, i_05c8c2),
    (0x05C8C4, i_05c8c4),
    (0x05C8C6, i_05c8c6),
    (0x05C8C8, i_05c8c8),
    (0x05C8CA, i_05c8ca),
    (0x05C8CC, i_05c8cc),
    (0x05C8CE, i_05c8ce),
    (0x05C8D0, i_05c8d0),
    (0x05C8D2, i_05c8d2),
    (0x05C8D4, i_05c8d4),
    (0x05C8D6, i_05c8d6),
    (0x05C8D8, i_05c8d8),
    (0x05C8DA, i_05c8da),
    (0x05C8DC, i_05c8dc),
    (0x05C8DE, i_05c8de),
    (0x05C8E0, i_05c8e0),
    (0x05C8E2, i_05c8e2),
    (0x05C8E4, i_05c8e4),
    (0x05C8E6, i_05c8e6),
    (0x05C8E8, i_05c8e8),
    (0x05C8EA, i_05c8ea),
    (0x05C8EC, i_05c8ec),
    (0x05C8EE, i_05c8ee),
    (0x05C8F0, i_05c8f0),
    (0x05C8F2, i_05c8f2),
    (0x05C8F4, i_05c8f4),
    (0x05C8F6, i_05c8f6),
    (0x05C8F8, i_05c8f8),
    (0x05C8FA, i_05c8fa),
    (0x05C8FC, i_05c8fc),
    (0x05C8FE, i_05c8fe),
    (0x05C900, i_05c900),
    (0x05C902, i_05c902),
    (0x05C904, i_05c904),
    (0x05C906, i_05c906),
    (0x05C908, i_05c908),
    (0x05C90A, i_05c90a),
    (0x05C90C, i_05c90c),
    (0x05C90E, i_05c90e),
    (0x05C910, i_05c910),
    (0x05C912, i_05c912),
    (0x05C914, i_05c914),
    (0x05C916, i_05c916),
    (0x05C918, i_05c918),
    (0x05C91A, i_05c91a),
    (0x05C91C, i_05c91c),
    (0x05C91E, i_05c91e),
    (0x05C920, i_05c920),
    (0x05C922, i_05c922),
    (0x05C924, i_05c924),
    (0x05C926, i_05c926),
    (0x05C928, i_05c928),
    (0x05C92A, i_05c92a),
    (0x05C92C, i_05c92c),
    (0x05C92E, i_05c92e),
    (0x05C930, i_05c930),
    (0x05C932, i_05c932),
    (0x05C934, i_05c934),
    (0x05C936, i_05c936),
    (0x05C938, i_05c938),
    (0x05C93A, i_05c93a),
    (0x05C93C, i_05c93c),
    (0x05C93E, i_05c93e),
    (0x05C940, i_05c940),
    (0x05C942, i_05c942),
    (0x05C944, i_05c944),
    (0x05C946, i_05c946),
    (0x05C948, i_05c948),
    (0x05C94A, i_05c94a),
    (0x05C94C, i_05c94c),
    (0x05C94E, i_05c94e),
    (0x05C950, i_05c950),
    (0x05C952, i_05c952),
    (0x05C954, i_05c954),
    (0x05C956, i_05c956),
    (0x05C958, i_05c958),
    (0x05C95A, i_05c95a),
    (0x05C95C, i_05c95c),
    (0x05C95E, i_05c95e),
    (0x05C960, i_05c960),
    (0x05C962, i_05c962),
    (0x05C964, i_05c964),
    (0x05C966, i_05c966),
    (0x05C968, i_05c968),
    (0x05C96A, i_05c96a),
    (0x05C96C, i_05c96c),
    (0x05C96E, i_05c96e),
    (0x05C970, i_05c970),
    (0x05C972, i_05c972),
    (0x05C974, i_05c974),
    (0x05C976, i_05c976),
    (0x05C978, i_05c978),
    (0x05C97A, i_05c97a),
    (0x05C97C, i_05c97c),
    (0x05C97E, i_05c97e),
    (0x05C980, i_05c980),
    (0x05C982, i_05c982),
    (0x05C984, i_05c984),
    (0x05C986, i_05c986),
    (0x05C988, i_05c988),
    (0x05C98A, i_05c98a),
    (0x05C98C, i_05c98c),
    (0x05C98E, i_05c98e),
    (0x05C990, i_05c990),
    (0x05C992, i_05c992),
    (0x05C994, i_05c994),
    (0x05C996, i_05c996),
    (0x05C998, i_05c998),
    (0x05C99A, i_05c99a),
    (0x05C99C, i_05c99c),
    (0x05C99E, i_05c99e),
    (0x05C9A0, i_05c9a0),
    (0x05C9A2, i_05c9a2),
    (0x05C9A4, i_05c9a4),
    (0x05C9A6, i_05c9a6),
    (0x05C9A8, i_05c9a8),
    (0x05C9AA, i_05c9aa),
    (0x05C9AC, i_05c9ac),
    (0x05C9AE, i_05c9ae),
    (0x05C9B0, i_05c9b0),
    (0x05C9B4, i_05c9b4),
    (0x05C9B6, i_05c9b6),
    (0x05C9B8, i_05c9b8),
    (0x05C9BC, i_05c9bc),
    (0x05C9C2, i_05c9c2),
    (0x05C9C4, i_05c9c4),
    (0x05C9C6, i_05c9c6),
    (0x05C9CA, i_05c9ca),
    (0x05C9CC, i_05c9cc),
    (0x05C9CE, i_05c9ce),
    (0x05C9D2, i_05c9d2),
    (0x05C9D4, i_05c9d4),
    (0x05C9D6, i_05c9d6),
    (0x05C9DC, i_05c9dc),
    (0x05C9E0, i_05c9e0),
    (0x05C9E6, i_05c9e6),
    (0x05C9EC, i_05c9ec),
    (0x05C9F2, i_05c9f2),
    (0x05C9F6, i_05c9f6),
    (0x05C9FA, i_05c9fa),
    (0x05C9FE, i_05c9fe),
    (0x05CA00, i_05ca00),
    (0x05CA02, i_05ca02),
    (0x05CA06, i_05ca06),
    (0x05CA0C, i_05ca0c),
    (0x05CA10, i_05ca10),
    (0x05CA14, i_05ca14),
    (0x05CA18, i_05ca18),
    (0x05CA1C, i_05ca1c),
    (0x05CA1E, i_05ca1e),
    (0x05CA22, i_05ca22),
    (0x05CA28, i_05ca28),
    (0x05CA2A, i_05ca2a),
    (0x05CA2E, i_05ca2e),
    (0x05CA32, i_05ca32),
    (0x05CA36, i_05ca36),
    (0x05CA3A, i_05ca3a),
    (0x05CA3E, i_05ca3e),
    (0x05CA42, i_05ca42),
    (0x05CA46, i_05ca46),
    (0x05CA4A, i_05ca4a),
    (0x05CA4E, i_05ca4e),
    (0x05CA52, i_05ca52),
    (0x05CA56, i_05ca56),
    (0x05CA5A, i_05ca5a),
    (0x05CA5E, i_05ca5e),
    (0x05CA60, i_05ca60),
    (0x05CA64, i_05ca64),
    (0x05CA68, i_05ca68),
    (0x05CA6C, i_05ca6c),
    (0x05CA70, i_05ca70),
    (0x05CA74, i_05ca74),
    (0x05CA7A, i_05ca7a),
    (0x05CA7E, i_05ca7e),
    (0x05CA80, i_05ca80),
    (0x05CA82, i_05ca82),
    (0x05CA88, i_05ca88),
    (0x05CA8A, i_05ca8a),
    (0x05CA8C, i_05ca8c),
    (0x05CA8E, i_05ca8e),
    (0x05CA90, i_05ca90),
    (0x05CA96, i_05ca96),
    (0x05CA9C, i_05ca9c),
    (0x05CA9E, i_05ca9e),
    (0x05CAA0, i_05caa0),
    (0x05CAA2, i_05caa2),
    (0x05CAA4, i_05caa4),
    (0x05CAAA, i_05caaa),
    (0x05CAAC, i_05caac),
    (0x05CAAE, i_05caae),
    (0x05CAB0, i_05cab0),
    (0x05CAB4, i_05cab4),
    (0x05CAB8, i_05cab8),
    (0x05CABC, i_05cabc),
    (0x05CAC0, i_05cac0),
    (0x05CAC4, i_05cac4),
    (0x05CAC8, i_05cac8),
    (0x05CACC, i_05cacc),
    (0x05CACE, i_05cace),
    (0x05CAD6, i_05cad6),
    (0x05CADE, i_05cade),
    (0x05CAE0, i_05cae0),
    (0x05CAE4, i_05cae4),
    (0x05CAE8, i_05cae8),
    (0x05CAEC, i_05caec),
    (0x05CAF0, i_05caf0),
    (0x05CAF4, i_05caf4),
    (0x05CAF6, i_05caf6),
    (0x05CAFA, i_05cafa),
    (0x05CAFC, i_05cafc),
    (0x05CB00, i_05cb00),
    (0x05CB06, i_05cb06),
    (0x05CB08, i_05cb08),
    (0x05CB0C, i_05cb0c),
    (0x05CB10, i_05cb10),
    (0x05CB12, i_05cb12),
    (0x05CB16, i_05cb16),
    (0x05CB18, i_05cb18),
    (0x05CB1A, i_05cb1a),
    (0x05CB1E, i_05cb1e),
    (0x05CB22, i_05cb22),
    (0x05CB26, i_05cb26),
    (0x05CB2A, i_05cb2a),
    (0x05CB30, i_05cb30),
    (0x05CB34, i_05cb34),
    (0x05CB38, i_05cb38),
    (0x05CB3E, i_05cb3e),
    (0x05CB44, i_05cb44),
    (0x05CB4C, i_05cb4c),
    (0x05CB50, i_05cb50),
    (0x05CB56, i_05cb56),
    (0x05CB5C, i_05cb5c),
    (0x05CB64, i_05cb64),
    (0x05CB68, i_05cb68),
    (0x05CB6C, i_05cb6c),
    (0x05CB72, i_05cb72),
    (0x05CB74, i_05cb74),
    (0x05CB78, i_05cb78),
    (0x05CB7C, i_05cb7c),
    (0x05CB7E, i_05cb7e),
    (0x05CB82, i_05cb82),
    (0x05CB84, i_05cb84),
    (0x05CB86, i_05cb86),
    (0x05CB8A, i_05cb8a),
    (0x05CB8E, i_05cb8e),
    (0x05CB92, i_05cb92),
    (0x05CB96, i_05cb96),
    (0x05CB9C, i_05cb9c),
    (0x05CBA0, i_05cba0),
    (0x05CBA4, i_05cba4),
    (0x05CBAA, i_05cbaa),
    (0x05CBB0, i_05cbb0),
    (0x05CBB8, i_05cbb8),
    (0x05CBC0, i_05cbc0),
    (0x05CBC4, i_05cbc4),
    (0x05CBCA, i_05cbca),
    (0x05CBD0, i_05cbd0),
    (0x05CBD8, i_05cbd8),
    (0x05CBDE, i_05cbde),
    (0x05CBE4, i_05cbe4),
    (0x05CBE8, i_05cbe8),
    (0x05CBEA, i_05cbea),
    (0x05CBEC, i_05cbec),
    (0x05CBF0, i_05cbf0),
    (0x05CBF4, i_05cbf4),
    (0x05CBF8, i_05cbf8),
    (0x05CBFC, i_05cbfc),
    (0x05CC00, i_05cc00),
    (0x05CC02, i_05cc02),
    (0x05CC06, i_05cc06),
    (0x05CC0E, i_05cc0e),
    (0x05CC14, i_05cc14),
    (0x05CC1A, i_05cc1a),
    (0x05CC20, i_05cc20),
    (0x05CC26, i_05cc26),
    (0x05CC2C, i_05cc2c),
    (0x05CC2E, i_05cc2e),
    (0x05CC30, i_05cc30),
    (0x05CC36, i_05cc36),
    (0x05CC3C, i_05cc3c),
    (0x05CC3E, i_05cc3e),
    (0x05CC40, i_05cc40),
    (0x05CC46, i_05cc46),
    (0x05CC4C, i_05cc4c),
    (0x05CC4E, i_05cc4e),
    (0x05CC50, i_05cc50),
    (0x05CC52, i_05cc52),
    (0x05CC54, i_05cc54),
    (0x05CC56, i_05cc56),
    (0x05CC5A, i_05cc5a),
    (0x05CC60, i_05cc60),
    (0x05CC64, i_05cc64),
    (0x05CC68, i_05cc68),
    (0x05CC6C, i_05cc6c),
    (0x05CC70, i_05cc70),
    (0x05CC76, i_05cc76),
    (0x05CC7C, i_05cc7c),
    (0x05CC7E, i_05cc7e),
    (0x05CC80, i_05cc80),
    (0x05CC84, i_05cc84),
    (0x05CC8A, i_05cc8a),
    (0x05CC8C, i_05cc8c),
    (0x05CC8E, i_05cc8e),
    (0x05CC94, i_05cc94),
    (0x05CC9A, i_05cc9a),
    (0x05CC9E, i_05cc9e),
    (0x05CCA4, i_05cca4),
    (0x05CCA8, i_05cca8),
    (0x05CCAE, i_05ccae),
    (0x05CCB4, i_05ccb4),
    (0x05CCB6, i_05ccb6),
    (0x05CCB8, i_05ccb8),
    (0x05CCBC, i_05ccbc),
    (0x05CCC2, i_05ccc2),
    (0x05CCC4, i_05ccc4),
    (0x05CCC6, i_05ccc6),
    (0x05CCC8, i_05ccc8),
    (0x05CCCE, i_05ccce),
    (0x05CCD0, i_05ccd0),
    (0x05CCD4, i_05ccd4),
    (0x05CCD8, i_05ccd8),
    (0x05CCDE, i_05ccde),
    (0x05CCE2, i_05cce2),
    (0x05CCE6, i_05cce6),
    (0x05CCEA, i_05ccea),
    (0x05CCF0, i_05ccf0),
    (0x05CCF4, i_05ccf4),
    (0x05CCF8, i_05ccf8),
    (0x05CCFC, i_05ccfc),
    (0x05CD02, i_05cd02),
    (0x05CD06, i_05cd06),
    (0x05CD0A, i_05cd0a),
    (0x05CD0E, i_05cd0e),
    (0x05CD14, i_05cd14),
    (0x05CD18, i_05cd18),
    (0x05CD1C, i_05cd1c),
    (0x05CD20, i_05cd20),
    (0x05CD26, i_05cd26),
    (0x05CD2A, i_05cd2a),
    (0x05CD2E, i_05cd2e),
    (0x05CD32, i_05cd32),
    (0x05CD38, i_05cd38),
    (0x05CD3C, i_05cd3c),
    (0x05CD40, i_05cd40),
    (0x05CD44, i_05cd44),
    (0x05CD4A, i_05cd4a),
    (0x05CD4E, i_05cd4e),
    (0x05CD52, i_05cd52),
    (0x05CD56, i_05cd56),
    (0x05CD5C, i_05cd5c),
    (0x05CD60, i_05cd60),
    (0x05CD64, i_05cd64),
    (0x05CD68, i_05cd68),
    (0x05CD6C, i_05cd6c),
    (0x05CD70, i_05cd70),
    (0x05CD74, i_05cd74),
    (0x05CD78, i_05cd78),
    (0x05CD7C, i_05cd7c),
    (0x05CD80, i_05cd80),
    (0x05CD84, i_05cd84),
    (0x05CD88, i_05cd88),
    (0x05CD8C, i_05cd8c),
    (0x05CD90, i_05cd90),
    (0x05CD94, i_05cd94),
    (0x05CD98, i_05cd98),
    (0x05CD9C, i_05cd9c),
    (0x05CDA0, i_05cda0),
    (0x05CDA4, i_05cda4),
    (0x05CDA8, i_05cda8),
    (0x05CDAC, i_05cdac),
    (0x05CDB0, i_05cdb0),
    (0x05CDB4, i_05cdb4),
    (0x05CDB8, i_05cdb8),
    (0x05CDBC, i_05cdbc),
    (0x05CDC0, i_05cdc0),
    (0x05CDC4, i_05cdc4),
    (0x05CDC8, i_05cdc8),
    (0x05CDCC, i_05cdcc),
    (0x05CDD0, i_05cdd0),
    (0x05CDD4, i_05cdd4),
    (0x05CDD8, i_05cdd8),
    (0x05CDDC, i_05cddc),
    (0x05CDE0, i_05cde0),
    (0x05CDE4, i_05cde4),
    (0x05CDE8, i_05cde8),
    (0x05CDEC, i_05cdec),
    (0x05CDF0, i_05cdf0),
    (0x05CDF4, i_05cdf4),
    (0x05CDF8, i_05cdf8),
    (0x05CDFC, i_05cdfc),
    (0x05CE00, i_05ce00),
    (0x05CE04, i_05ce04),
    (0x05CE08, i_05ce08),
    (0x05CE0C, i_05ce0c),
    (0x05CE10, i_05ce10),
    (0x05CE14, i_05ce14),
    (0x05CE18, i_05ce18),
    (0x05CE1C, i_05ce1c),
    (0x05CE22, i_05ce22),
    (0x05CE26, i_05ce26),
    (0x05CE2A, i_05ce2a),
    (0x05CE2E, i_05ce2e),
    (0x05CE34, i_05ce34),
    (0x05CE38, i_05ce38),
    (0x05CE3C, i_05ce3c),
    (0x05CE40, i_05ce40),
    (0x05CE46, i_05ce46),
    (0x05CE4A, i_05ce4a),
    (0x05CE4E, i_05ce4e),
    (0x05CE52, i_05ce52),
    (0x05CE58, i_05ce58),
    (0x05CE5C, i_05ce5c),
    (0x05CE60, i_05ce60),
    (0x05CE64, i_05ce64),
    (0x05CE6A, i_05ce6a),
    (0x05CE6E, i_05ce6e),
    (0x05CE72, i_05ce72),
    (0x05CE76, i_05ce76),
    (0x05CE7C, i_05ce7c),
    (0x05CE80, i_05ce80),
    (0x05CE84, i_05ce84),
    (0x05CE88, i_05ce88),
    (0x05CE8E, i_05ce8e),
    (0x05CE92, i_05ce92),
    (0x05CE96, i_05ce96),
    (0x05CE9A, i_05ce9a),
    (0x05CEA0, i_05cea0),
    (0x05CEA4, i_05cea4),
    (0x05CEA8, i_05cea8),
    (0x05CEAC, i_05ceac),
    (0x05CEB2, i_05ceb2),
    (0x05CEB6, i_05ceb6),
    (0x05CEBA, i_05ceba),
    (0x05CEBE, i_05cebe),
    (0x05CEC4, i_05cec4),
    (0x05CEC8, i_05cec8),
    (0x05CECC, i_05cecc),
    (0x05CED0, i_05ced0),
    (0x05CED6, i_05ced6),
    (0x05CEDA, i_05ceda),
    (0x05CEDE, i_05cede),
    (0x05CEE2, i_05cee2),
    (0x05CEE8, i_05cee8),
    (0x05CEEC, i_05ceec),
    (0x05CEF0, i_05cef0),
    (0x05CEF4, i_05cef4),
    (0x05CEF8, i_05cef8),
    (0x05CEFC, i_05cefc),
    (0x05CF00, i_05cf00),
    (0x05CF04, i_05cf04),
    (0x05CF08, i_05cf08),
    (0x05CF0C, i_05cf0c),
    (0x05CF10, i_05cf10),
    (0x05CF14, i_05cf14),
    (0x05CF18, i_05cf18),
    (0x05CF1C, i_05cf1c),
    (0x05CF20, i_05cf20),
    (0x05CF24, i_05cf24),
    (0x05CF28, i_05cf28),
    (0x05CF2C, i_05cf2c),
    (0x05CF30, i_05cf30),
    (0x05CF34, i_05cf34),
    (0x05CF38, i_05cf38),
    (0x05CF3C, i_05cf3c),
    (0x05CF40, i_05cf40),
    (0x05CF44, i_05cf44),
    (0x05CF48, i_05cf48),
    (0x05CF4C, i_05cf4c),
    (0x05CF50, i_05cf50),
    (0x05CF54, i_05cf54),
    (0x05CF58, i_05cf58),
    (0x05CF5C, i_05cf5c),
    (0x05CF60, i_05cf60),
    (0x05CF64, i_05cf64),
    (0x05CF68, i_05cf68),
    (0x05CF6C, i_05cf6c),
    (0x05CF70, i_05cf70),
    (0x05CF74, i_05cf74),
    (0x05CF78, i_05cf78),
    (0x05CF7C, i_05cf7c),
    (0x05CF80, i_05cf80),
    (0x05CF84, i_05cf84),
    (0x05CF88, i_05cf88),
    (0x05CF8C, i_05cf8c),
    (0x05CF90, i_05cf90),
    (0x05CF94, i_05cf94),
    (0x05CF98, i_05cf98),
    (0x05CF9C, i_05cf9c),
    (0x05CFA0, i_05cfa0),
    (0x05CFA4, i_05cfa4),
    (0x05CFA8, i_05cfa8),
    (0x05CFB0, i_05cfb0),
    (0x05CFB4, i_05cfb4),
    (0x05CFBA, i_05cfba),
    (0x05CFBE, i_05cfbe),
    (0x05CFC2, i_05cfc2),
    (0x05CFC6, i_05cfc6),
    (0x05CFC8, i_05cfc8),
    (0x05CFCC, i_05cfcc),
    (0x05CFD2, i_05cfd2),
    (0x05CFD6, i_05cfd6),
    (0x05CFDC, i_05cfdc),
    (0x05CFE0, i_05cfe0),
    (0x05CFE6, i_05cfe6),
    (0x05CFEA, i_05cfea),
    (0x05CFF0, i_05cff0),
    (0x05CFF4, i_05cff4),
    (0x05CFF8, i_05cff8),
    (0x05CFFC, i_05cffc),
    (0x05CFFE, i_05cffe),
    (0x05D002, i_05d002),
    (0x05D006, i_05d006),
    (0x05D008, i_05d008),
    (0x05D00C, i_05d00c),
    (0x05D00E, i_05d00e),
    (0x05D012, i_05d012),
    (0x05D016, i_05d016),
    (0x05D01A, i_05d01a),
    (0x05D01E, i_05d01e),
    (0x05D022, i_05d022),
    (0x05D026, i_05d026),
    (0x05D02A, i_05d02a),
    (0x05D02E, i_05d02e),
    (0x05D032, i_05d032),
    (0x05D03A, i_05d03a),
    (0x05D03E, i_05d03e),
    (0x05D044, i_05d044),
    (0x05D048, i_05d048),
    (0x05D04C, i_05d04c),
    (0x05D050, i_05d050),
    (0x05D052, i_05d052),
    (0x05D056, i_05d056),
    (0x05D05C, i_05d05c),
    (0x05D060, i_05d060),
    (0x05D066, i_05d066),
    (0x05D06A, i_05d06a),
    (0x05D070, i_05d070),
    (0x05D074, i_05d074),
    (0x05D07A, i_05d07a),
    (0x05D07E, i_05d07e),
    (0x05D082, i_05d082),
    (0x05D086, i_05d086),
    (0x05D088, i_05d088),
    (0x05D08A, i_05d08a),
    (0x05D08E, i_05d08e),
    (0x05D092, i_05d092),
    (0x05D094, i_05d094),
    (0x05D098, i_05d098),
    (0x05D09A, i_05d09a),
    (0x05D09E, i_05d09e),
    (0x05D0A2, i_05d0a2),
    (0x05D0A8, i_05d0a8),
    (0x05D0AC, i_05d0ac),
    (0x05D0B0, i_05d0b0),
    (0x05D0B4, i_05d0b4),
    (0x05D0BA, i_05d0ba),
    (0x05D0BE, i_05d0be),
    (0x05D0C2, i_05d0c2),
    (0x05D0C6, i_05d0c6),
    (0x05D0CC, i_05d0cc),
    (0x05D0D0, i_05d0d0),
    (0x05D0D4, i_05d0d4),
    (0x05D0D8, i_05d0d8),
    (0x05D0DE, i_05d0de),
    (0x05D0E2, i_05d0e2),
    (0x05D0E6, i_05d0e6),
    (0x05D0E8, i_05d0e8),
    (0x05D0EC, i_05d0ec),
    (0x05D0F0, i_05d0f0),
    (0x05D0F2, i_05d0f2),
    (0x05D0F6, i_05d0f6),
    (0x05D0F8, i_05d0f8),
    (0x05D0FC, i_05d0fc),
    (0x05D100, i_05d100),
    (0x05D106, i_05d106),
    (0x05D10A, i_05d10a),
    (0x05D10E, i_05d10e),
    (0x05D112, i_05d112),
    (0x05D118, i_05d118),
    (0x05D11C, i_05d11c),
    (0x05D120, i_05d120),
    (0x05D124, i_05d124),
    (0x05D12A, i_05d12a),
    (0x05D12E, i_05d12e),
    (0x05D132, i_05d132),
    (0x05D136, i_05d136),
    (0x05D13C, i_05d13c),
    (0x05D140, i_05d140),
    (0x05D144, i_05d144),
    (0x05D148, i_05d148),
    (0x05D14C, i_05d14c),
    (0x05D150, i_05d150),
    (0x05D154, i_05d154),
    (0x05D158, i_05d158),
    (0x05D15C, i_05d15c),
    (0x05D160, i_05d160),
    (0x05D164, i_05d164),
    (0x05D168, i_05d168),
    (0x05D16C, i_05d16c),
    (0x05D170, i_05d170),
    (0x05D178, i_05d178),
    (0x05D17C, i_05d17c),
    (0x05D182, i_05d182),
    (0x05D186, i_05d186),
    (0x05D18A, i_05d18a),
    (0x05D18E, i_05d18e),
    (0x05D190, i_05d190),
    (0x05D194, i_05d194),
    (0x05D19A, i_05d19a),
    (0x05D19E, i_05d19e),
    (0x05D1A4, i_05d1a4),
    (0x05D1A8, i_05d1a8),
    (0x05D1AE, i_05d1ae),
    (0x05D1B2, i_05d1b2),
    (0x05D1B8, i_05d1b8),
    (0x05D1BC, i_05d1bc),
    (0x05D1C0, i_05d1c0),
    (0x05D1C4, i_05d1c4),
    (0x05D1C8, i_05d1c8),
    (0x05D1CA, i_05d1ca),
    (0x05D1CE, i_05d1ce),
    (0x05D1D2, i_05d1d2),
    (0x05D1D4, i_05d1d4),
    (0x05D1D8, i_05d1d8),
    (0x05D1DA, i_05d1da),
    (0x05D1DE, i_05d1de),
    (0x05D1E2, i_05d1e2),
    (0x05D1E6, i_05d1e6),
    (0x05D1EA, i_05d1ea),
    (0x05D1EE, i_05d1ee),
    (0x05D1F4, i_05d1f4),
    (0x05D1F8, i_05d1f8),
    (0x05D1FC, i_05d1fc),
    (0x05D1FE, i_05d1fe),
    (0x05D200, i_05d200),
    (0x05D204, i_05d204),
    (0x05D208, i_05d208),
    (0x05D20A, i_05d20a),
    (0x05D20E, i_05d20e),
    (0x05D210, i_05d210),
    (0x05D214, i_05d214),
    (0x05D216, i_05d216),
    (0x05D21A, i_05d21a),
    (0x05D21E, i_05d21e),
    (0x05D224, i_05d224),
    (0x05D228, i_05d228),
    (0x05D22C, i_05d22c),
    (0x05D22E, i_05d22e),
    (0x05D230, i_05d230),
    (0x05D234, i_05d234),
    (0x05D238, i_05d238),
    (0x05D23A, i_05d23a),
    (0x05D23E, i_05d23e),
    (0x05D240, i_05d240),
    (0x05D244, i_05d244),
    (0x05D248, i_05d248),
    (0x05D24C, i_05d24c),
    (0x05D250, i_05d250),
    (0x05D254, i_05d254),
    (0x05D258, i_05d258),
    (0x05D25C, i_05d25c),
    (0x05D260, i_05d260),
    (0x05D264, i_05d264),
    (0x05D268, i_05d268),
    (0x05D26C, i_05d26c),
    (0x05D270, i_05d270),
    (0x05D274, i_05d274),
    (0x05D278, i_05d278),
    (0x05D27C, i_05d27c),
    (0x05D280, i_05d280),
    (0x05D284, i_05d284),
    (0x05D288, i_05d288),
    (0x05D28C, i_05d28c),
    (0x05D290, i_05d290),
    (0x05D296, i_05d296),
    (0x05D29A, i_05d29a),
    (0x05D29E, i_05d29e),
    (0x05D2A2, i_05d2a2),
    (0x05D2A6, i_05d2a6),
    (0x05D2AA, i_05d2aa),
    (0x05D2B0, i_05d2b0),
    (0x05D2B4, i_05d2b4),
    (0x05D2B8, i_05d2b8),
    (0x05D2BC, i_05d2bc),
    (0x05D2C0, i_05d2c0),
    (0x05D2C4, i_05d2c4),
    (0x05D2C8, i_05d2c8),
    (0x05D2CE, i_05d2ce),
    (0x05D2D2, i_05d2d2),
    (0x05D2D6, i_05d2d6),
    (0x05D2DA, i_05d2da),
    (0x05D2DE, i_05d2de),
    (0x05D2E4, i_05d2e4),
    (0x05D2E8, i_05d2e8),
    (0x05D2EA, i_05d2ea),
    (0x05D2EE, i_05d2ee),
    (0x05D2F2, i_05d2f2),
    (0x05D2F6, i_05d2f6),
    (0x05D2F8, i_05d2f8),
    (0x05D2FC, i_05d2fc),
    (0x05D300, i_05d300),
    (0x05D302, i_05d302),
    (0x05D306, i_05d306),
    (0x05D30A, i_05d30a),
    (0x05D30E, i_05d30e),
    (0x05D310, i_05d310),
    (0x05D314, i_05d314),
    (0x05D5B6, i_05d5b6),
    (0x05D5BA, i_05d5ba),
    (0x05D5BE, i_05d5be),
    (0x05D5C4, i_05d5c4),
    (0x05D5C8, i_05d5c8),
    (0x05D5CE, i_05d5ce),
    (0x05D5D2, i_05d5d2),
    (0x05D5D8, i_05d5d8),
    (0x05D5DC, i_05d5dc),
    (0x05D5E2, i_05d5e2),
    (0x05D5E6, i_05d5e6),
    (0x05D5E8, i_05d5e8),
    (0x05D5F0, i_05d5f0),
    (0x05D5F4, i_05d5f4),
    (0x05D5FA, i_05d5fa),
    (0x05D5FE, i_05d5fe),
    (0x05D602, i_05d602),
    (0x05D604, i_05d604),
    (0x05D606, i_05d606),
    (0x05D60A, i_05d60a),
    (0x05D60E, i_05d60e),
    (0x05D612, i_05d612),
    (0x05D614, i_05d614),
    (0x05D616, i_05d616),
    (0x05D61A, i_05d61a),
    (0x05D61E, i_05d61e),
    (0x05D624, i_05d624),
    (0x05D628, i_05d628),
    (0x05D62C, i_05d62c),
    (0x05D632, i_05d632),
    (0x05D638, i_05d638),
    (0x05D63C, i_05d63c),
    (0x05D642, i_05d642),
    (0x05D646, i_05d646),
    (0x05D648, i_05d648),
    (0x05D650, i_05d650),
    (0x05D654, i_05d654),
    (0x05D65A, i_05d65a),
    (0x05D65E, i_05d65e),
    (0x05D662, i_05d662),
    (0x05D664, i_05d664),
    (0x05D666, i_05d666),
    (0x05D66A, i_05d66a),
    (0x05D66E, i_05d66e),
    (0x05D672, i_05d672),
    (0x05D674, i_05d674),
    (0x05D678, i_05d678),
    (0x05D67E, i_05d67e),
    (0x05D682, i_05d682),
    (0x05D684, i_05d684),
    (0x05D68A, i_05d68a),
    (0x05D690, i_05d690),
    (0x05D694, i_05d694),
    (0x05D69A, i_05d69a),
    (0x05D69E, i_05d69e),
    (0x05D6A0, i_05d6a0),
    (0x05D6A6, i_05d6a6),
    (0x05D6AA, i_05d6aa),
    (0x05D6AE, i_05d6ae),
    (0x05D6B0, i_05d6b0),
    (0x05D6B6, i_05d6b6),
    (0x05D6B8, i_05d6b8),
    (0x05D6BC, i_05d6bc),
    (0x05D6BE, i_05d6be),
    (0x05D6C2, i_05d6c2),
    (0x05D6C8, i_05d6c8),
    (0x05D6CA, i_05d6ca),
    (0x05D6CE, i_05d6ce),
    (0x05D6D0, i_05d6d0),
    (0x05D6D4, i_05d6d4),
    (0x05D6D6, i_05d6d6),
    (0x05D6D8, i_05d6d8),
    (0x05D6DA, i_05d6da),
    (0x05D6E0, i_05d6e0),
    (0x05D6E2, i_05d6e2),
    (0x05D6E4, i_05d6e4),
    (0x05D6E6, i_05d6e6),
    (0x05D6E8, i_05d6e8),
    (0x05D6EA, i_05d6ea),
    (0x05D6F0, i_05d6f0),
    (0x05D6F2, i_05d6f2),
    (0x05D6F4, i_05d6f4),
    (0x05D6F6, i_05d6f6),
    (0x05D6FC, i_05d6fc),
    (0x05D6FE, i_05d6fe),
    (0x05D700, i_05d700),
    (0x05D704, i_05d704),
    (0x05D708, i_05d708),
    (0x05D70A, i_05d70a),
    (0x05D712, i_05d712),
    (0x05D716, i_05d716),
    (0x05D71A, i_05d71a),
    (0x05D72C, i_05d72c),
    (0x05D72E, i_05d72e),
    (0x05D734, i_05d734),
    (0x05D736, i_05d736),
    (0x05D73A, i_05d73a),
    (0x05D73C, i_05d73c),
    (0x05D740, i_05d740),
    (0x05D746, i_05d746),
    (0x05D748, i_05d748),
    (0x05D74C, i_05d74c),
    (0x05D74E, i_05d74e),
    (0x05D752, i_05d752),
    (0x05D754, i_05d754),
    (0x05D756, i_05d756),
    (0x05D758, i_05d758),
    (0x05D75E, i_05d75e),
    (0x05D760, i_05d760),
    (0x05D762, i_05d762),
    (0x05D764, i_05d764),
    (0x05D766, i_05d766),
    (0x05D768, i_05d768),
    (0x05D76E, i_05d76e),
    (0x05D770, i_05d770),
    (0x05D772, i_05d772),
    (0x05D774, i_05d774),
    (0x05D776, i_05d776),
    (0x05D77C, i_05d77c),
    (0x05D780, i_05d780),
    (0x05D782, i_05d782),
    (0x05D784, i_05d784),
    (0x05D786, i_05d786),
    (0x05D78A, i_05d78a),
    (0x05D78C, i_05d78c),
    (0x05D790, i_05d790),
    (0x05D792, i_05d792),
    (0x05D796, i_05d796),
    (0x05D79A, i_05d79a),
    (0x05D79E, i_05d79e),
    (0x05D7A2, i_05d7a2),
    (0x05D7A6, i_05d7a6),
    (0x05D7AA, i_05d7aa),
    (0x05D7AC, i_05d7ac),
    (0x05D7B4, i_05d7b4),
    (0x05D7B8, i_05d7b8),
    (0x05D7BC, i_05d7bc),
    (0x05D7BE, i_05d7be),
    (0x05D7C0, i_05d7c0),
    (0x05D7C2, i_05d7c2),
    (0x05D7C6, i_05d7c6),
    (0x05D7CC, i_05d7cc),
    (0x05D7CE, i_05d7ce),
    (0x05D7D2, i_05d7d2),
    (0x05D7D4, i_05d7d4),
    (0x05D7D8, i_05d7d8),
    (0x05D7DA, i_05d7da),
    (0x05D7DC, i_05d7dc),
    (0x05D7E0, i_05d7e0),
    (0x05D7E6, i_05d7e6),
    (0x05D7E8, i_05d7e8),
    (0x05D7EC, i_05d7ec),
    (0x05D7EE, i_05d7ee),
    (0x05D7F0, i_05d7f0),
    (0x05D7F2, i_05d7f2),
    (0x05D7F4, i_05d7f4),
    (0x05D7FA, i_05d7fa),
    (0x05D7FC, i_05d7fc),
    (0x05D7FE, i_05d7fe),
    (0x05D800, i_05d800),
    (0x05D802, i_05d802),
    (0x05D804, i_05d804),
    (0x05D80A, i_05d80a),
    (0x05D80C, i_05d80c),
    (0x05D80E, i_05d80e),
    (0x05D810, i_05d810),
    (0x05D812, i_05d812),
    (0x05D818, i_05d818),
    (0x05D81C, i_05d81c),
    (0x05D81E, i_05d81e),
    (0x05D820, i_05d820),
    (0x05D824, i_05d824),
    (0x05D826, i_05d826),
    (0x05D82A, i_05d82a),
    (0x05D82C, i_05d82c),
    (0x05D830, i_05d830),
    (0x05D834, i_05d834),
    (0x05D838, i_05d838),
    (0x05D83C, i_05d83c),
    (0x05D83E, i_05d83e),
    (0x05D842, i_05d842),
    (0x05D844, i_05d844),
    (0x05D848, i_05d848),
    (0x05D84C, i_05d84c),
    (0x05D850, i_05d850),
    (0x05D856, i_05d856),
    (0x05D85A, i_05d85a),
    (0x05D85E, i_05d85e),
    (0x05D862, i_05d862),
    (0x05D886, i_05d886),
    (0x05D88A, i_05d88a),
    (0x05D88C, i_05d88c),
    (0x05D890, i_05d890),
    (0x05D896, i_05d896),
    (0x05D89A, i_05d89a),
    (0x05D89C, i_05d89c),
    (0x05D89E, i_05d89e),
    (0x05D8A2, i_05d8a2),
    (0x05D8A4, i_05d8a4),
    (0x05D8A6, i_05d8a6),
    (0x05D8AC, i_05d8ac),
    (0x05D8B0, i_05d8b0),
    (0x05D8B2, i_05d8b2),
    (0x05D8B4, i_05d8b4),
    (0x05D8B8, i_05d8b8),
    (0x05D8BA, i_05d8ba),
    (0x05D8BC, i_05d8bc),
    (0x05D8BE, i_05d8be),
    (0x05D8C4, i_05d8c4),
    (0x05D8C8, i_05d8c8),
    (0x05D8CA, i_05d8ca),
    (0x05D8CC, i_05d8cc),
    (0x05D8D0, i_05d8d0),
    (0x05D8D2, i_05d8d2),
    (0x05D8D4, i_05d8d4),
    (0x05D8D6, i_05d8d6),
    (0x05D8DC, i_05d8dc),
    (0x05D8DE, i_05d8de),
    (0x05D8E0, i_05d8e0),
    (0x05D8E4, i_05d8e4),
    (0x05D8E6, i_05d8e6),
    (0x05D8E8, i_05d8e8),
    (0x05D8EA, i_05d8ea),
    (0x05D8EC, i_05d8ec),
    (0x05D8F0, i_05d8f0),
    (0x05D8F2, i_05d8f2),
    (0x05D8F8, i_05d8f8),
    (0x05D8FC, i_05d8fc),
    (0x05D902, i_05d902),
    (0x05D904, i_05d904),
    (0x05D906, i_05d906),
    (0x05D908, i_05d908),
    (0x05D90A, i_05d90a),
    (0x05D90E, i_05d90e),
    (0x05D910, i_05d910),
    (0x05D914, i_05d914),
    (0x05D916, i_05d916),
    (0x05D918, i_05d918),
    (0x05D91A, i_05d91a),
    (0x05D91C, i_05d91c),
    (0x05D91E, i_05d91e),
    (0x05D920, i_05d920),
    (0x05D922, i_05d922),
    (0x05D924, i_05d924),
    (0x05D928, i_05d928),
    (0x05D92A, i_05d92a),
    (0x05D92E, i_05d92e),
    (0x05D930, i_05d930),
    (0x05D932, i_05d932),
    (0x05D934, i_05d934),
    (0x05D936, i_05d936),
    (0x05D93A, i_05d93a),
    (0x05D93C, i_05d93c),
    (0x05D93E, i_05d93e),
    (0x05D942, i_05d942),
    (0x05D944, i_05d944),
    (0x05D946, i_05d946),
    (0x05D94C, i_05d94c),
    (0x05D950, i_05d950),
    (0x05D954, i_05d954),
    (0x05DA56, i_05da56),
    (0x05DA5A, i_05da5a),
    (0x05DA5C, i_05da5c),
    (0x05DA5E, i_05da5e),
    (0x05DA60, i_05da60),
    (0x05DA62, i_05da62),
    (0x05DA64, i_05da64),
    (0x05DA66, i_05da66),
    (0x05DA6A, i_05da6a),
    (0x05DA6E, i_05da6e),
    (0x05DA72, i_05da72),
    (0x05DA76, i_05da76),
    (0x05DA7E, i_05da7e),
    (0x05DA80, i_05da80),
    (0x05DA84, i_05da84),
    (0x05DA88, i_05da88),
    (0x05DA8A, i_05da8a),
    (0x05DA8E, i_05da8e),
    (0x05DA92, i_05da92),
    (0x05DA96, i_05da96),
    (0x05DA9A, i_05da9a),
    (0x05DA9C, i_05da9c),
    (0x05DAA0, i_05daa0),
    (0x05DAA2, i_05daa2),
    (0x05DAA4, i_05daa4),
    (0x05DAA6, i_05daa6),
    (0x05DAA8, i_05daa8),
    (0x05DAAA, i_05daaa),
    (0x05DAAE, i_05daae),
    (0x05DAB2, i_05dab2),
    (0x05DAB6, i_05dab6),
    (0x05DABA, i_05daba),
    (0x05DAC2, i_05dac2),
    (0x05DAC6, i_05dac6),
    (0x05DACA, i_05daca),
    (0x05DACE, i_05dace),
    (0x05DAD2, i_05dad2),
    (0x05DAD6, i_05dad6),
    (0x05DAD8, i_05dad8),
    (0x05DADA, i_05dada),
    (0x05DADC, i_05dadc),
    (0x05DADE, i_05dade),
    (0x05DAE0, i_05dae0),
    (0x05DAE4, i_05dae4),
    (0x05DAE8, i_05dae8),
    (0x05DAEC, i_05daec),
    (0x05DAF0, i_05daf0),
    (0x05DAF4, i_05daf4),
    (0x05DAF8, i_05daf8),
    (0x05DAFC, i_05dafc),
    (0x05DB00, i_05db00),
    (0x05DB02, i_05db02),
    (0x05DB0A, i_05db0a),
    (0x05DB0E, i_05db0e),
    (0x05DB10, i_05db10),
    (0x05DB12, i_05db12),
    (0x05DB14, i_05db14),
    (0x05DB16, i_05db16),
    (0x05DB18, i_05db18),
    (0x05DB1A, i_05db1a),
    (0x05DB1C, i_05db1c),
    (0x05DB1E, i_05db1e),
    (0x05DB22, i_05db22),
    (0x05DB26, i_05db26),
    (0x05DB28, i_05db28),
    (0x05DB2A, i_05db2a),
    (0x05DB2E, i_05db2e),
    (0x05DB32, i_05db32),
    (0x05DB36, i_05db36),
    (0x05DB3A, i_05db3a),
    (0x05DB3C, i_05db3c),
    (0x05DB40, i_05db40),
    (0x05DB44, i_05db44),
    (0x05DB48, i_05db48),
    (0x05DB4C, i_05db4c),
    (0x05DB50, i_05db50),
    (0x05DB52, i_05db52),
    (0x05DB56, i_05db56),
    (0x05DB58, i_05db58),
    (0x05DB5C, i_05db5c),
    (0x05DB60, i_05db60),
    (0x05DB64, i_05db64),
    (0x05DB68, i_05db68),
    (0x05DB6A, i_05db6a),
    (0x05DB6E, i_05db6e),
    (0x05DB72, i_05db72),
    (0x05DB76, i_05db76),
    (0x05DB7A, i_05db7a),
    (0x05DB80, i_05db80),
    (0x05DB84, i_05db84),
    (0x05DB88, i_05db88),
    (0x05DB8C, i_05db8c),
    (0x05DB90, i_05db90),
    (0x05DB94, i_05db94),
    (0x05DB98, i_05db98),
    (0x05DB9C, i_05db9c),
    (0x05DBA0, i_05dba0),
    (0x05DBA4, i_05dba4),
    (0x05DBA6, i_05dba6),
    (0x05DBAA, i_05dbaa),
    (0x05DBAE, i_05dbae),
    (0x05DBB2, i_05dbb2),
    (0x05DBB6, i_05dbb6),
    (0x05DBBA, i_05dbba),
    (0x05DBBC, i_05dbbc),
    (0x05DBC0, i_05dbc0),
    (0x05DBC2, i_05dbc2),
    (0x05DBC6, i_05dbc6),
    (0x05DBCA, i_05dbca),
    (0x05DBCC, i_05dbcc),
    (0x05DBD0, i_05dbd0),
    (0x05DBD4, i_05dbd4),
    (0x05DBDA, i_05dbda),
    (0x05DBDC, i_05dbdc),
    (0x05DBE0, i_05dbe0),
    (0x05DBE4, i_05dbe4),
    (0x05DBE8, i_05dbe8),
    (0x05DBEC, i_05dbec),
    (0x05DBF0, i_05dbf0),
    (0x05DBF4, i_05dbf4),
    (0x05DBF8, i_05dbf8),
    (0x05DBFE, i_05dbfe),
    (0x05DC00, i_05dc00),
    (0x05DC04, i_05dc04),
    (0x05DC08, i_05dc08),
    (0x05DC0C, i_05dc0c),
    (0x05DC10, i_05dc10),
    (0x05DC14, i_05dc14),
    (0x05DC16, i_05dc16),
    (0x05DC1A, i_05dc1a),
    (0x05DC1C, i_05dc1c),
    (0x05DC20, i_05dc20),
    (0x05DC28, i_05dc28),
    (0x05DC2C, i_05dc2c),
    (0x05DC30, i_05dc30),
    (0x05DC32, i_05dc32),
    (0x05DC34, i_05dc34),
    (0x05DC38, i_05dc38),
    (0x05DC3E, i_05dc3e),
    (0x05DC44, i_05dc44),
    (0x05DC4A, i_05dc4a),
    (0x05DC50, i_05dc50),
    (0x05DC54, i_05dc54),
    (0x05DC58, i_05dc58),
    (0x05DC5C, i_05dc5c),
    (0x05DC60, i_05dc60),
    (0x05DC64, i_05dc64),
    (0x05DC68, i_05dc68),
    (0x05DC6C, i_05dc6c),
    (0x05DC70, i_05dc70),
    (0x05DC76, i_05dc76),
    (0x05DC7C, i_05dc7c),
    (0x05DC80, i_05dc80),
    (0x05DC84, i_05dc84),
    (0x05DC88, i_05dc88),
    (0x05DC8C, i_05dc8c),
    (0x05DC90, i_05dc90),
    (0x05DC94, i_05dc94),
    (0x05DC98, i_05dc98),
    (0x05DC9C, i_05dc9c),
    (0x05DCA2, i_05dca2),
    (0x05DCA4, i_05dca4),
    (0x05DCAA, i_05dcaa),
    (0x05DCAE, i_05dcae),
    (0x05DCB0, i_05dcb0),
    (0x05DCB4, i_05dcb4),
    (0x05DCB6, i_05dcb6),
    (0x05DCBA, i_05dcba),
    (0x05DCC0, i_05dcc0),
    (0x05DCC4, i_05dcc4),
    (0x05DCC8, i_05dcc8),
    (0x05DCCC, i_05dccc),
    (0x05DCCE, i_05dcce),
    (0x05DCD2, i_05dcd2),
    (0x05DCD6, i_05dcd6),
    (0x05DCDA, i_05dcda),
    (0x05DCE0, i_05dce0),
    (0x05DCE4, i_05dce4),
    (0x05DCEA, i_05dcea),
    (0x05DCEE, i_05dcee),
    (0x05DCF2, i_05dcf2),
    (0x05DCF8, i_05dcf8),
    (0x05DCFC, i_05dcfc),
    (0x05DD00, i_05dd00),
    (0x05DD02, i_05dd02),
    (0x05DD08, i_05dd08),
    (0x05DD0E, i_05dd0e),
    (0x05DD14, i_05dd14),
    (0x05DD1A, i_05dd1a),
    (0x05DD20, i_05dd20),
    (0x05DD22, i_05dd22),
    (0x05DD28, i_05dd28),
    (0x05DD2A, i_05dd2a),
    (0x05DD2E, i_05dd2e),
    (0x05DD34, i_05dd34),
    (0x05DD3A, i_05dd3a),
    (0x05DD40, i_05dd40),
    (0x05DD46, i_05dd46),
    (0x05DD4A, i_05dd4a),
    (0x05DD56, i_05dd56),
    (0x05DD5C, i_05dd5c),
    (0x05DD62, i_05dd62),
    (0x05DD66, i_05dd66),
    (0x05DD6A, i_05dd6a),
    (0x05DD70, i_05dd70),
    (0x05DD74, i_05dd74),
    (0x05DD78, i_05dd78),
    (0x05DD7C, i_05dd7c),
    (0x05DD80, i_05dd80),
    (0x05DD84, i_05dd84),
    (0x05DD86, i_05dd86),
    (0x05DD8A, i_05dd8a),
    (0x05DD8E, i_05dd8e),
    (0x05DD92, i_05dd92),
    (0x05DD96, i_05dd96),
    (0x05DD9A, i_05dd9a),
    (0x05DD9E, i_05dd9e),
    (0x05DDA2, i_05dda2),
    (0x05DDA6, i_05dda6),
    (0x05DDAA, i_05ddaa),
    (0x05DDAE, i_05ddae),
    (0x05DDB2, i_05ddb2),
    (0x05DDB6, i_05ddb6),
    (0x05DDB8, i_05ddb8),
    (0x05DDBC, i_05ddbc),
    (0x05DDBE, i_05ddbe),
    (0x05DDC4, i_05ddc4),
    (0x05DDC8, i_05ddc8),
    (0x05DDCE, i_05ddce),
    (0x05DDD2, i_05ddd2),
    (0x05DDD8, i_05ddd8),
    (0x05DDDC, i_05dddc),
    (0x05DDE2, i_05dde2),
    (0x05DDE6, i_05dde6),
    (0x05DDEC, i_05ddec),
    (0x05DDF0, i_05ddf0),
    (0x05DDF2, i_05ddf2),
    (0x05DDF8, i_05ddf8),
    (0x05DDFA, i_05ddfa),
    (0x05DE00, i_05de00),
    (0x05DE02, i_05de02),
    (0x05DE08, i_05de08),
    (0x05DE0A, i_05de0a),
    (0x05DE10, i_05de10),
    (0x05DE12, i_05de12),
    (0x05DE16, i_05de16),
    (0x05E018, i_05e018),
    (0x05E01A, i_05e01a),
    (0x05E01C, i_05e01c),
    (0x05E01E, i_05e01e),
    (0x05E020, i_05e020),
    (0x05E022, i_05e022),
    (0x05E024, i_05e024),
    (0x05E028, i_05e028),
    (0x05E02A, i_05e02a),
    (0x05E02C, i_05e02c),
    (0x05E02E, i_05e02e),
    (0x05E030, i_05e030),
    (0x05E034, i_05e034),
    (0x05E036, i_05e036),
    (0x05E038, i_05e038),
    (0x05E03C, i_05e03c),
    (0x05E03E, i_05e03e),
    (0x05E042, i_05e042),
    (0x05E046, i_05e046),
    (0x05E04A, i_05e04a),
    (0x05E04E, i_05e04e),
    (0x05E052, i_05e052),
    (0x05E056, i_05e056),
    (0x05E05A, i_05e05a),
    (0x05E05C, i_05e05c),
    (0x05E05E, i_05e05e),
    (0x05E062, i_05e062),
    (0x05E064, i_05e064),
    (0x05E066, i_05e066),
    (0x05E068, i_05e068),
    (0x05E06C, i_05e06c),
    (0x05E06E, i_05e06e),
    (0x05E070, i_05e070),
    (0x05E074, i_05e074),
    (0x05E078, i_05e078),
    (0x05E07C, i_05e07c),
    (0x05E080, i_05e080),
    (0x05E084, i_05e084),
    (0x05E086, i_05e086),
    (0x05E088, i_05e088),
    (0x05E08C, i_05e08c),
    (0x05E090, i_05e090),
    (0x05E096, i_05e096),
    (0x05E09C, i_05e09c),
    (0x05E09E, i_05e09e),
    (0x05E0A2, i_05e0a2),
    (0x05E0A6, i_05e0a6),
    (0x05E0A8, i_05e0a8),
    (0x05E0AC, i_05e0ac),
    (0x05E0B0, i_05e0b0),
    (0x05E0B4, i_05e0b4),
    (0x05E0B8, i_05e0b8),
    (0x05E0BC, i_05e0bc),
    (0x05E0BE, i_05e0be),
    (0x05E0C2, i_05e0c2),
    (0x05E0C6, i_05e0c6),
    (0x05E0CA, i_05e0ca),
];
