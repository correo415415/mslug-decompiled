// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_6) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $08A65C: jsr $5dd22.l
pub fn i_08a65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A662;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A662);
    cpu.pc = 0x5DD22;
    16
}
/// $08A662: addi.w #$c0, $22(a0)
pub fn i_08a662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A668;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A668: subi.w #$30, $24(a0)
pub fn i_08a668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A66E;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A66E: lea.l $7808a.l, a1
pub fn i_08a66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A674;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A674: jsr $4ae.l
pub fn i_08a674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A67A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A67A);
    cpu.pc = 0x4AE;
    16
}
/// $08A67A: jsr $5dd22.l
pub fn i_08a67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A680;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A680);
    cpu.pc = 0x5DD22;
    16
}
/// $08A680: addi.w #$100, $22(a0)
pub fn i_08a680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A686;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A686: subi.w #$40, $24(a0)
pub fn i_08a686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A68C;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A68C: lea.l $2ef286.l, a1
pub fn i_08a68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A692;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08A692: jsr $77c7e.l
pub fn i_08a692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A698;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A698);
    cpu.pc = 0x77C7E;
    16
}
/// $08A698: lea.l $2ef438.l, a2
pub fn i_08a698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A69E;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF438;
    20
}
/// $08A69E: jsr $5022a.l
pub fn i_08a69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6A4);
    cpu.pc = 0x5022A;
    16
}
/// $08A6A4: lea.l $2ef424.l, a2
pub fn i_08a6a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6AA;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF424;
    20
}
/// $08A6AA: jsr $5022a.l
pub fn i_08a6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6B0);
    cpu.pc = 0x5022A;
    16
}
/// $08A6B0: lea.l $2ef488.l, a2
pub fn i_08a6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6B6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF488;
    20
}
/// $08A6B6: jsr $5022a.l
pub fn i_08a6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6BC);
    cpu.pc = 0x5022A;
    16
}
/// $08A6BC: lea.l $2ef63e.l, a1
pub fn i_08a6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6C2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF63E;
    20
}
/// $08A6C2: jsr $43fac.l
pub fn i_08a6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6C8);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6C8: lea.l $2ef66c.l, a1
pub fn i_08a6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6CE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF66C;
    20
}
/// $08A6CE: jsr $43fac.l
pub fn i_08a6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6D4);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6D4: lea.l $2ef688.l, a1
pub fn i_08a6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF688;
    20
}
/// $08A6DA: jsr $43fac.l
pub fn i_08a6da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6E0);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6E0: lea.l $2ef696.l, a1
pub fn i_08a6e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF696;
    20
}
/// $08A6E6: jsr $43fac.l
pub fn i_08a6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6EC);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6EC: lea.l $2ef704.l, a1
pub fn i_08a6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6F2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF704;
    20
}
/// $08A6F2: jsr $43fac.l
pub fn i_08a6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A6F8);
    cpu.pc = 0x43FAC;
    16
}
/// $08A6F8: move.b #$ff, $82(a6)
pub fn i_08a6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A6FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A6FE: move.b #$ff, $86(a6)
pub fn i_08a6fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A704;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A704: lea.l $8ae0c(pc), a1
pub fn i_08a704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A708;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE0C;
    16
}
/// $08A708: jsr $4ae.l
pub fn i_08a708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A70E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A70E);
    cpu.pc = 0x4AE;
    16
}
/// $08A70E: jsr $5dd22.l
pub fn i_08a70e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A714;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A714);
    cpu.pc = 0x5DD22;
    16
}
/// $08A714: addi.w #$20, $22(a0)
pub fn i_08a714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A71A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A71A: lea.l $8b07c(pc), a1
pub fn i_08a71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A71E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B07C;
    16
}
/// $08A71E: jsr $4ae.l
pub fn i_08a71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A724);
    cpu.pc = 0x4AE;
    16
}
/// $08A724: jsr $5dd22.l
pub fn i_08a724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A72A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A72A);
    cpu.pc = 0x5DD22;
    16
}
/// $08A72A: addi.w #$90, $22(a0)
pub fn i_08a72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A730;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A730: subi.w #$10, $24(a0)
pub fn i_08a730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A736;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A736: move.b #$0, $21(a0)
pub fn i_08a736(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A73C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A73C: move.b #$ff, $20(a6)
pub fn i_08a73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A742;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A742: jsr $8b490(pc)
pub fn i_08a742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A746;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8A746);
    cpu.pc = 0x8B490;
    16
}
/// $08A746: jmp $518.l
pub fn i_08a746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A74C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08A74C: rts
pub fn i_08a74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A74E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Init_08a74e  ($08A74E, 554 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A74E: movea.l $3c(a6), a1
pub fn i_08a74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A752;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08A752: jsr $2942a.l
pub fn i_08a752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A758;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A758);
    cpu.pc = 0x2942A;
    16
}
/// $08A758: move.w #$1dc, d1
pub fn i_08a758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A75C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1DC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A75C: jsr $236e.l
pub fn i_08a75c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A762;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A762);
    cpu.pc = 0x236E;
    16
}
/// $08A762: move.w #$d3, d1
pub fn i_08a762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A766;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A766: jsr $236e.l
pub fn i_08a766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A76C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A76C);
    cpu.pc = 0x236E;
    16
}
/// $08A76C: move.b #$ff, $32(a6)
pub fn i_08a76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A772;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A772: move.b #$ff, $33(a6)
pub fn i_08a772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A778;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A778: move.b #$0, $3a(a6)
pub fn i_08a778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A77E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A77E: move.b #$0, $20(a6)
pub fn i_08a77e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A784;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A784: move.w #$7fff, $66(a6)
pub fn i_08a784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A78A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x7FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A78A: move.w #$c000, $38(a6)
pub fn i_08a78a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A790;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08A790: cmpi.b #$c, $106ece.l
pub fn i_08a790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A798;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08A798: beq.w $8a7b2
pub fn i_08a798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A79C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A7B2; return 10; }
    8
}
/// $08A79C: lea.l $f0638.l, a1
pub fn i_08a79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7A2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0xF0638;
    20
}
/// $08A7A2: move.w #$82, d0
pub fn i_08a7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7A6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A7A6: move.b #$0, $82(a6)
pub fn i_08a7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A7AC: jsr $4429e.l
pub fn i_08a7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7B2);
    cpu.pc = 0x4429E;
    16
}
/// $08A7B2: lea.l $8ae56(pc), a1
pub fn i_08a7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A7B6: jsr $4ae.l
pub fn i_08a7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7BC);
    cpu.pc = 0x4AE;
    16
}
/// $08A7BC: jsr $5dd22.l
pub fn i_08a7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7C2);
    cpu.pc = 0x5DD22;
    16
}
/// $08A7C2: move.b #$0, $21(a0)
pub fn i_08a7c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7C8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A7C8: subi.w #$20, $22(a0)
pub fn i_08a7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7CE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A7CE: lea.l $2ef5c8.l, a1
pub fn i_08a7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7D4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF5C8;
    20
}
/// $08A7D4: move.l a1, $70(a0)
pub fn i_08a7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7D8;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A7D8: lea.l $2ef5dc.l, a1
pub fn i_08a7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7DE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF5DC;
    20
}
/// $08A7DE: move.l a1, $74(a0)
pub fn i_08a7de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7E2;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A7E2: lea.l $8ae56(pc), a1
pub fn i_08a7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A7E6: jsr $4ae.l
pub fn i_08a7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7EC);
    cpu.pc = 0x4AE;
    16
}
/// $08A7EC: jsr $5dd22.l
pub fn i_08a7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A7F2);
    cpu.pc = 0x5DD22;
    16
}
/// $08A7F2: move.b #$0, $21(a0)
pub fn i_08a7f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7F8;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A7F8: addi.w #$20, $22(a0)
pub fn i_08a7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A7FE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A7FE: lea.l $2ef49c.l, a1
pub fn i_08a7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A804;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF49C;
    20
}
/// $08A804: move.l a1, $70(a0)
pub fn i_08a804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A808;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A808: lea.l $2ef4b0.l, a1
pub fn i_08a808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A80E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF4B0;
    20
}
/// $08A80E: move.l a1, $74(a0)
pub fn i_08a80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A812;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A812: lea.l $8ae56(pc), a1
pub fn i_08a812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A816: jsr $4ae.l
pub fn i_08a816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A81C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A81C);
    cpu.pc = 0x4AE;
    16
}
/// $08A81C: jsr $5dd22.l
pub fn i_08a81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A822);
    cpu.pc = 0x5DD22;
    16
}
/// $08A822: move.b #$0, $21(a0)
pub fn i_08a822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A828;
    cpu.ir = 0x117C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A828: addi.w #$60, $22(a0)
pub fn i_08a828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A82E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x60, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A82E: lea.l $2ef4c4.l, a1
pub fn i_08a82e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A834;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF4C4;
    20
}
/// $08A834: move.l a1, $70(a0)
pub fn i_08a834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A838;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A838: lea.l $2ef4d8.l, a1
pub fn i_08a838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A83E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF4D8;
    20
}
/// $08A83E: move.l a1, $74(a0)
pub fn i_08a83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A842;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A842: lea.l $8ae56(pc), a1
pub fn i_08a842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A846;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE56;
    16
}
/// $08A846: jsr $4ae.l
pub fn i_08a846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A84C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A84C);
    cpu.pc = 0x4AE;
    16
}
/// $08A84C: jsr $5dd22.l
pub fn i_08a84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A852;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A852);
    cpu.pc = 0x5DD22;
    16
}
/// $08A852: move.b #$1, $21(a0)
pub fn i_08a852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A858;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A858: addi.w #$a0, $22(a0)
pub fn i_08a858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A85E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A85E: lea.l $2ef4ec.l, a1
pub fn i_08a85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A864;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF4EC;
    20
}
/// $08A864: move.l a1, $70(a0)
pub fn i_08a864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A868;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A868: lea.l $2ef500.l, a1
pub fn i_08a868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A86E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF500;
    20
}
/// $08A86E: move.l a1, $74(a0)
pub fn i_08a86e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A872;
    cpu.ir = 0x2149;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A872: move.b #$0, $86(a6)
pub fn i_08a872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A878;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A878: lea.l $77228.l, a1
pub fn i_08a878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A87E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77228;
    20
}
/// $08A87E: jsr $4ae.l
pub fn i_08a87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A884;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A884);
    cpu.pc = 0x4AE;
    16
}
/// $08A884: jsr $5dd22.l
pub fn i_08a884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A88A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A88A);
    cpu.pc = 0x5DD22;
    16
}
/// $08A88A: addi.w #$110, $22(a0)
pub fn i_08a88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A890;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x110, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A890: subi.w #$48, $24(a0)
pub fn i_08a890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A896;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x48, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A896: move.b #$49, $98(a0)
pub fn i_08a896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A89C;
    cpu.ir = 0x117C;
    let v1: u32 = 0x49;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A89C: move.b #$86, $99(a0)
pub fn i_08a89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8A2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x86;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x99);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A8A2: lea.l $2ee23c.l, a0
pub fn i_08a8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE23C;
    20
}
/// $08A8A8: jsr $28cd4.l
pub fn i_08a8a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A8AE);
    cpu.pc = 0x28CD4;
    16
}
/// $08A8AE: lea.l $2eeb50.l, a0
pub fn i_08a8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEB50;
    20
}
/// $08A8B4: move.l a0, $48(a6)
pub fn i_08a8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8B8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A8B8: move.b #$0, $87(a6)
pub fn i_08a8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A8BE: lea.l $8a8c4(pc), a1
pub fn i_08a8be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A8C4;
    16
}
/// $08A8C2: move.l a1, (a6)
pub fn i_08a8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A8C4: clr.b $10e39a.l
pub fn i_08a8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8CA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08A8CA: jsr $2783a.l
pub fn i_08a8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A8D0);
    cpu.pc = 0x2783A;
    16
}
/// $08A8D0: cmpi.w #$b0, $22(a6)
pub fn i_08a8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8D6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xB0, Size::Word, false);
    16
}
/// $08A8D6: bgt.w $8a90c
pub fn i_08a8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8DA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8A90C; return 10; }
    8
}
/// $08A8DA: move.b #$0, $87(a6)
pub fn i_08a8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A8E0: jsr $28d70.l
pub fn i_08a8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A8E6);
    cpu.pc = 0x28D70;
    16
}
/// $08A8E6: bcc.w $8a90c
pub fn i_08a8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A90C; return 10; }
    8
}
/// $08A8EA: jsr $2870a.l
pub fn i_08a8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A8F0);
    cpu.pc = 0x2870A;
    16
}
/// $08A8F0: bcc.w $8a90c
pub fn i_08a8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A90C; return 10; }
    8
}
/// $08A8F4: lea.l $2ee228.l, a0
pub fn i_08a8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A8FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE228;
    20
}
/// $08A8FA: jsr $28cd4.l
pub fn i_08a8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A900;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A900);
    cpu.pc = 0x28CD4;
    16
}
/// $08A900: move.b #$ff, $87(a6)
pub fn i_08a900(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A906;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A906: jsr $28d70.l
pub fn i_08a906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A90C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A90C);
    cpu.pc = 0x28D70;
    16
}
/// $08A90C: jsr $2870a.l
pub fn i_08a90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A912;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A912);
    cpu.pc = 0x2870A;
    16
}
/// $08A912: bcc.w $8a9ae
pub fn i_08a912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A916;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8A9AE; return 10; }
    8
}
/// $08A916: bclr.b #$3, $13(a6)
pub fn i_08a916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A91C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08A91C: cmpi.b #$1, $58(a6)
pub fn i_08a91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A922;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08A922: beq.w $8a9ae
pub fn i_08a922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A926;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A9AE; return 10; }
    8
}
/// $08A926: cmpi.b #$2, $58(a6)
pub fn i_08a926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A92C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $08A92C: beq.w $8a9ae
pub fn i_08a92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A930;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8A9AE; return 10; }
    8
}
/// $08A930: cmpi.b #$c, $106ece.l
pub fn i_08a930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A938;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $08A938: bne.w $8a980
pub fn i_08a938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A93C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8A980; return 10; }
    8
}
/// $08A93C: lea.l $2ef6a4.l, a1
pub fn i_08a93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A942;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6A4;
    20
}
/// $08A942: jsr $43fac.l
pub fn i_08a942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A948;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A948);
    cpu.pc = 0x43FAC;
    16
}
/// $08A948: lea.l $2ef6c4.l, a1
pub fn i_08a948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A94E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6C4;
    20
}
/// $08A94E: jsr $43fac.l
pub fn i_08a94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A954;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A954);
    cpu.pc = 0x43FAC;
    16
}
/// $08A954: lea.l $2ef6e4.l, a1
pub fn i_08a954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A95A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6E4;
    20
}
/// $08A95A: jsr $43fac.l
pub fn i_08a95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A960;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A960);
    cpu.pc = 0x43FAC;
    16
}
/// $08A960: lea.l $2ef53c.l, a2
pub fn i_08a960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A966;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF53C;
    20
}
/// $08A966: jsr $5022a.l
pub fn i_08a966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A96C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A96C);
    cpu.pc = 0x5022A;
    16
}
/// $08A96C: lea.l $2ef5a0.l, a2
pub fn i_08a96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A972;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF5A0;
    20
}
/// $08A972: jsr $5022a.l
pub fn i_08a972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A978);
    cpu.pc = 0x5022A;
    16
}
// ===== SetTaskHandler_08a978  ($08A978, 8 B, kernel/task_handlers.c)
/// $08A978: lea.l $8ac92(pc), a1
pub fn i_08a978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A97C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AC92;
    16
}
/// $08A97C: move.l a1, (a6)
pub fn i_08a97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A97E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A97E: rts
pub fn i_08a97e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A980;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Hit_08a980  ($08A980, 40 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A980: move.w #$1035, d0
pub fn i_08a980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A984;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08A984: jsr $2352.l
pub fn i_08a984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A98A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A98A);
    cpu.pc = 0x2352;
    16
}
/// $08A98A: lea.l $7808a.l, a1
pub fn i_08a98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A990;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08A990: jsr $4ae.l
pub fn i_08a990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A996;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A996);
    cpu.pc = 0x4AE;
    16
}
/// $08A996: jsr $5dd22.l
pub fn i_08a996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A99C);
    cpu.pc = 0x5DD22;
    16
}
/// $08A99C: addi.w #$c0, $22(a0)
pub fn i_08a99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9A2;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08A9A2: subi.w #$80, $24(a0)
pub fn i_08a9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9A8;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08a9a8  ($08A9A8, 8 B, kernel/task_handlers.c)
/// $08A9A8: lea.l $8a9b0(pc), a1
pub fn i_08a9a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9AC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8A9B0;
    16
}
/// $08A9AC: move.l a1, (a6)
pub fn i_08a9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9AE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08A9AE: rts
pub fn i_08a9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Stage2_08a9b0  ($08A9B0, 314 B, enemies/bosses/scene5_airship_088a64.s)
/// $08A9B0: jsr $434dc.l
pub fn i_08a9b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9B6);
    cpu.pc = 0x434DC;
    16
}
/// $08A9B6: jsr $2783a.l
pub fn i_08a9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9BC);
    cpu.pc = 0x2783A;
    16
}
/// $08A9BC: lea.l $2ef514.l, a2
pub fn i_08a9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9C2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF514;
    20
}
/// $08A9C2: jsr $5022a.l
pub fn i_08a9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9C8);
    cpu.pc = 0x5022A;
    16
}
/// $08A9C8: lea.l $2ef578.l, a2
pub fn i_08a9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9CE;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF578;
    20
}
/// $08A9CE: jsr $5022a.l
pub fn i_08a9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9D4);
    cpu.pc = 0x5022A;
    16
}
/// $08A9D4: lea.l $2ef6a4.l, a1
pub fn i_08a9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9DA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6A4;
    20
}
/// $08A9DA: jsr $43fac.l
pub fn i_08a9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9E0);
    cpu.pc = 0x43FAC;
    16
}
/// $08A9E0: lea.l $2ef286.l, a1
pub fn i_08a9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9E6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08A9E6: jsr $77c7e.l
pub fn i_08a9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8A9EC);
    cpu.pc = 0x77C7E;
    16
}
/// $08A9EC: lea.l $2eeba4.l, a0
pub fn i_08a9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEBA4;
    20
}
/// $08A9F2: move.l a0, $48(a6)
pub fn i_08a9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9F6;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08A9F6: move.b #$0, $87(a6)
pub fn i_08a9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8A9FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08A9FC: lea.l $8aa02(pc), a1
pub fn i_08a9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AA02;
    16
}
/// $08AA00: move.l a1, (a6)
pub fn i_08aa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AA02: move.l $106f50.l, d0
pub fn i_08aa02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA08;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08AA08: swap d0
pub fn i_08aa08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA0A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08AA0A: cmpi.w #$950, d0
pub fn i_08aa0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA0E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x950, Size::Word, false);
    8
}
/// $08AA0E: bge.w $8aa48
pub fn i_08aa0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA12;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8AA48; return 10; }
    8
}
/// $08AA12: clr.b $10e39a.l
pub fn i_08aa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA18;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08AA18: jsr $2783a.l
pub fn i_08aa18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA1E);
    cpu.pc = 0x2783A;
    16
}
/// $08AA1E: jsr $28d70.l
pub fn i_08aa1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA24);
    cpu.pc = 0x28D70;
    16
}
/// $08AA24: jsr $2870a.l
pub fn i_08aa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA2A);
    cpu.pc = 0x2870A;
    16
}
/// $08AA2A: bcc.w $8aa3a
pub fn i_08aa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AA3A; return 10; }
    8
}
/// $08AA2E: lea.l $5e766.l, a0
pub fn i_08aa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AA34: jsr $5e770.l
pub fn i_08aa34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA3A);
    cpu.pc = 0x5E770;
    16
}
/// $08AA3A: bclr.b #$3, $13(a6)
pub fn i_08aa3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA40;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AA40: bclr.b #$0, $13(a6)
pub fn i_08aa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA46;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AA46: rts
pub fn i_08aa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08AA48: lea.l $2c06aa.l, a0
pub fn i_08aa48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C06AA;
    20
}
/// $08AA4E: jsr $799de.l
pub fn i_08aa4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA54);
    cpu.pc = 0x799DE;
    16
}
/// $08AA54: move.w d0, $66(a6)
pub fn i_08aa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA58;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08AA58: lea.l $8aa5e(pc), a1
pub fn i_08aa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AA5E;
    16
}
/// $08AA5C: move.l a1, (a6)
pub fn i_08aa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AA5E: clr.b $10e39a.l
pub fn i_08aa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA64;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08AA64: jsr $2783a.l
pub fn i_08aa64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA6A);
    cpu.pc = 0x2783A;
    16
}
/// $08AA6A: move.b #$0, $87(a6)
pub fn i_08aa6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA70;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AA70: jsr $28d70.l
pub fn i_08aa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA76);
    cpu.pc = 0x28D70;
    16
}
/// $08AA76: bcc.w $8aa9c
pub fn i_08aa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA7A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AA9C; return 10; }
    8
}
/// $08AA7A: jsr $2870a.l
pub fn i_08aa7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA80);
    cpu.pc = 0x2870A;
    16
}
/// $08AA80: bcc.w $8aa9c
pub fn i_08aa80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AA9C; return 10; }
    8
}
/// $08AA84: lea.l $2ee228.l, a0
pub fn i_08aa84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE228;
    20
}
/// $08AA8A: jsr $28cd4.l
pub fn i_08aa8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA90);
    cpu.pc = 0x28CD4;
    16
}
/// $08AA90: move.b #$ff, $87(a6)
pub fn i_08aa90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA96;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AA96: jsr $28d70.l
pub fn i_08aa96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AA9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AA9C);
    cpu.pc = 0x28D70;
    16
}
/// $08AA9C: jsr $2870a.l
pub fn i_08aa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAA2);
    cpu.pc = 0x2870A;
    16
}
/// $08AAA2: bcc.w $8aab8
pub fn i_08aaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAA6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AAB8; return 10; }
    8
}
/// $08AAA6: lea.l $5e766.l, a0
pub fn i_08aaa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAAC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AAAC: jsr $5e770.l
pub fn i_08aaac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAB2);
    cpu.pc = 0x5E770;
    16
}
/// $08AAB2: bclr.b #$3, $13(a6)
pub fn i_08aab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAB8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AAB8: cmpi.w #$29a, $66(a6)
pub fn i_08aab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AABE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x29A, Size::Word, false);
    16
}
/// $08AABE: bgt.w $8aaf0
pub fn i_08aabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAC2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8AAF0; return 10; }
    8
}
/// $08AAC2: move.w #$1035, d0
pub fn i_08aac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAC6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AAC6: jsr $2352.l
pub fn i_08aac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AACC);
    cpu.pc = 0x2352;
    16
}
/// $08AACC: lea.l $7808a.l, a1
pub fn i_08aacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAD2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08AAD2: jsr $4ae.l
pub fn i_08aad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAD8);
    cpu.pc = 0x4AE;
    16
}
/// $08AAD8: jsr $5dd22.l
pub fn i_08aad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AADE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AADE);
    cpu.pc = 0x5DD22;
    16
}
/// $08AADE: addi.w #$c0, $22(a0)
pub fn i_08aade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAE4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AAE4: subi.w #$80, $24(a0)
pub fn i_08aae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAEA;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08aaea  ($08AAEA, 8 B, kernel/task_handlers.c)
/// $08AAEA: lea.l $8aaf2(pc), a1
pub fn i_08aaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AAF2;
    16
}
/// $08AAEE: move.l a1, (a6)
pub fn i_08aaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AAF0: rts
pub fn i_08aaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Stage3_08aaf2  ($08AAF2, 218 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AAF2: jsr $434dc.l
pub fn i_08aaf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAF8);
    cpu.pc = 0x434DC;
    16
}
/// $08AAF8: jsr $2783a.l
pub fn i_08aaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AAFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AAFE);
    cpu.pc = 0x2783A;
    16
}
/// $08AAFE: lea.l $2ef528.l, a2
pub fn i_08aafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB04;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF528;
    20
}
/// $08AB04: jsr $5022a.l
pub fn i_08ab04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB0A);
    cpu.pc = 0x5022A;
    16
}
/// $08AB0A: lea.l $2ef58c.l, a2
pub fn i_08ab0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB10;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF58C;
    20
}
/// $08AB10: jsr $5022a.l
pub fn i_08ab10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB16);
    cpu.pc = 0x5022A;
    16
}
/// $08AB16: lea.l $2ef6c4.l, a1
pub fn i_08ab16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB1C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6C4;
    20
}
/// $08AB1C: jsr $43fac.l
pub fn i_08ab1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB22);
    cpu.pc = 0x43FAC;
    16
}
/// $08AB22: lea.l $2ef286.l, a1
pub fn i_08ab22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB28;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08AB28: jsr $77c7e.l
pub fn i_08ab28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB2E);
    cpu.pc = 0x77C7E;
    16
}
/// $08AB2E: lea.l $2ee266.l, a0
pub fn i_08ab2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB34;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE266;
    20
}
/// $08AB34: jsr $28cd4.l
pub fn i_08ab34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB3A);
    cpu.pc = 0x28CD4;
    16
}
/// $08AB3A: lea.l $8ab40(pc), a1
pub fn i_08ab3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AB40;
    16
}
/// $08AB3E: move.l a1, (a6)
pub fn i_08ab3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AB40: clr.b $10e39a.l
pub fn i_08ab40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB46;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08AB46: jsr $2783a.l
pub fn i_08ab46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB4C);
    cpu.pc = 0x2783A;
    16
}
/// $08AB4C: move.b #$0, $87(a6)
pub fn i_08ab4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB52;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AB52: jsr $28d70.l
pub fn i_08ab52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB58);
    cpu.pc = 0x28D70;
    16
}
/// $08AB58: bcc.w $8ab7e
pub fn i_08ab58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AB7E; return 10; }
    8
}
/// $08AB5C: jsr $2870a.l
pub fn i_08ab5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB62);
    cpu.pc = 0x2870A;
    16
}
/// $08AB62: bcc.w $8ab7e
pub fn i_08ab62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AB7E; return 10; }
    8
}
/// $08AB66: lea.l $2ee252.l, a0
pub fn i_08ab66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB6C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE252;
    20
}
/// $08AB6C: jsr $28cd4.l
pub fn i_08ab6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB72);
    cpu.pc = 0x28CD4;
    16
}
/// $08AB72: move.b #$ff, $87(a6)
pub fn i_08ab72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB78;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AB78: jsr $28d70.l
pub fn i_08ab78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB7E);
    cpu.pc = 0x28D70;
    16
}
/// $08AB7E: jsr $2870a.l
pub fn i_08ab7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB84);
    cpu.pc = 0x2870A;
    16
}
/// $08AB84: bcc.w $8ab9a
pub fn i_08ab84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AB9A; return 10; }
    8
}
/// $08AB88: lea.l $5e766.l, a0
pub fn i_08ab88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB8E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AB8E: jsr $5e770.l
pub fn i_08ab8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AB94);
    cpu.pc = 0x5E770;
    16
}
/// $08AB94: bclr.b #$3, $13(a6)
pub fn i_08ab94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AB9A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AB9A: cmpi.w #$14d, $66(a6)
pub fn i_08ab9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABA0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14D, Size::Word, false);
    16
}
/// $08ABA0: bgt.w $8aaf0
pub fn i_08aba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABA4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8AAF0; return 10; }
    8
}
/// $08ABA4: move.w #$1035, d0
pub fn i_08aba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABA8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1035;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08ABA8: jsr $2352.l
pub fn i_08aba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABAE);
    cpu.pc = 0x2352;
    16
}
/// $08ABAE: lea.l $7808a.l, a1
pub fn i_08abae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABB4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08ABB4: jsr $4ae.l
pub fn i_08abb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABBA);
    cpu.pc = 0x4AE;
    16
}
/// $08ABBA: jsr $5dd22.l
pub fn i_08abba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABC0);
    cpu.pc = 0x5DD22;
    16
}
/// $08ABC0: addi.w #$c0, $22(a0)
pub fn i_08abc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABC6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ABC6: subi.w #$80, $24(a0)
pub fn i_08abc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABCC;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
// ===== SetTaskHandler_08abcc  ($08ABCC, 8 B, kernel/task_handlers.c)
/// $08ABCC: lea.l $8abd4(pc), a1
pub fn i_08abcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABD0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8ABD4;
    16
}
/// $08ABD0: move.l a1, (a6)
pub fn i_08abd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABD2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08ABD2: rts
pub fn i_08abd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_TowerB_Stage4_08abd4  ($08ABD4, 190 B, enemies/bosses/scene5_airship_088a64.s)
/// $08ABD4: jsr $434dc.l
pub fn i_08abd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABDA);
    cpu.pc = 0x434DC;
    16
}
/// $08ABDA: jsr $2783a.l
pub fn i_08abda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABE0);
    cpu.pc = 0x2783A;
    16
}
/// $08ABE0: lea.l $2ef53c.l, a2
pub fn i_08abe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABE6;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF53C;
    20
}
/// $08ABE6: jsr $5022a.l
pub fn i_08abe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABEC);
    cpu.pc = 0x5022A;
    16
}
/// $08ABEC: lea.l $2ef5a0.l, a2
pub fn i_08abec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABF2;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF5A0;
    20
}
/// $08ABF2: jsr $5022a.l
pub fn i_08abf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ABF8);
    cpu.pc = 0x5022A;
    16
}
/// $08ABF8: lea.l $2ef6e4.l, a1
pub fn i_08abf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ABFE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF6E4;
    20
}
/// $08ABFE: jsr $43fac.l
pub fn i_08abfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC04);
    cpu.pc = 0x43FAC;
    16
}
/// $08AC04: lea.l $2ef286.l, a1
pub fn i_08ac04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC0A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08AC0A: jsr $77c7e.l
pub fn i_08ac0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC10;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC10);
    cpu.pc = 0x77C7E;
    16
}
/// $08AC10: lea.l $2ee290.l, a0
pub fn i_08ac10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC16;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE290;
    20
}
/// $08AC16: jsr $28cd4.l
pub fn i_08ac16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC1C);
    cpu.pc = 0x28CD4;
    16
}
/// $08AC1C: lea.l $8ac22(pc), a1
pub fn i_08ac1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AC22;
    16
}
/// $08AC20: move.l a1, (a6)
pub fn i_08ac20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AC22: clr.b $10e39a.l
pub fn i_08ac22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC28;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $08AC28: jsr $2783a.l
pub fn i_08ac28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC2E);
    cpu.pc = 0x2783A;
    16
}
/// $08AC2E: move.b #$0, $87(a6)
pub fn i_08ac2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC34;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AC34: jsr $28d70.l
pub fn i_08ac34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC3A);
    cpu.pc = 0x28D70;
    16
}
/// $08AC3A: bcc.w $8ac60
pub fn i_08ac3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AC60; return 10; }
    8
}
/// $08AC3E: jsr $2870a.l
pub fn i_08ac3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC44);
    cpu.pc = 0x2870A;
    16
}
/// $08AC44: bcc.w $8ac60
pub fn i_08ac44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AC60; return 10; }
    8
}
/// $08AC48: lea.l $2ee27c.l, a0
pub fn i_08ac48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC4E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE27C;
    20
}
/// $08AC4E: jsr $28cd4.l
pub fn i_08ac4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC54);
    cpu.pc = 0x28CD4;
    16
}
/// $08AC54: move.b #$ff, $87(a6)
pub fn i_08ac54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC5A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AC5A: jsr $28d70.l
pub fn i_08ac5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC60);
    cpu.pc = 0x28D70;
    16
}
/// $08AC60: jsr $2870a.l
pub fn i_08ac60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC66);
    cpu.pc = 0x2870A;
    16
}
/// $08AC66: bcc.w $8ac7c
pub fn i_08ac66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC6A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AC7C; return 10; }
    8
}
/// $08AC6A: lea.l $5e766.l, a0
pub fn i_08ac6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AC70: jsr $5e770.l
pub fn i_08ac70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC76);
    cpu.pc = 0x5E770;
    16
}
/// $08AC76: bclr.b #$3, $13(a6)
pub fn i_08ac76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC7C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AC7C: jsr $28758.l
pub fn i_08ac7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC82);
    cpu.pc = 0x28758;
    16
}
/// $08AC82: bcc.w $8ae0a
pub fn i_08ac82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC86;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AE0A; return 10; }
    8
}
/// $08AC86: move.l #$5000, d0
pub fn i_08ac86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC8C;
    cpu.ir = 0x203C;
    let v1: u32 = 0x5000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08AC8C: jsr $51a28.l
pub fn i_08ac8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC92);
    cpu.pc = 0x51A28;
    16
}
// ===== S5_TowerB_Destroy_08ac92  ($08AC92, 378 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AC92: jsr $434ce.l
pub fn i_08ac92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AC98);
    cpu.pc = 0x434CE;
    16
}
/// $08AC98: move.w #$102f, d0
pub fn i_08ac98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AC9C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AC9C: jsr $2352.l
pub fn i_08ac9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACA2);
    cpu.pc = 0x2352;
    16
}
/// $08ACA2: lea.l $7808a.l, a1
pub fn i_08aca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACA8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08ACA8: jsr $4ae.l
pub fn i_08aca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACAE);
    cpu.pc = 0x4AE;
    16
}
/// $08ACAE: jsr $5dd22.l
pub fn i_08acae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACB4);
    cpu.pc = 0x5DD22;
    16
}
/// $08ACB4: addi.w #$c0, $22(a0)
pub fn i_08acb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACBA;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACBA: subi.w #$80, $24(a0)
pub fn i_08acba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACC0;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x80, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACC0: lea.l $7808a.l, a1
pub fn i_08acc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACC6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08ACC6: jsr $4ae.l
pub fn i_08acc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACCC);
    cpu.pc = 0x4AE;
    16
}
/// $08ACCC: jsr $5dd22.l
pub fn i_08accc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACD2);
    cpu.pc = 0x5DD22;
    16
}
/// $08ACD2: addi.w #$30, $22(a0)
pub fn i_08acd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACD8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACD8: subi.w #$30, $24(a0)
pub fn i_08acd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACDE;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACDE: lea.l $7808a.l, a1
pub fn i_08acde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACE4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08ACE4: jsr $4ae.l
pub fn i_08ace4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACEA);
    cpu.pc = 0x4AE;
    16
}
/// $08ACEA: jsr $5dd22.l
pub fn i_08acea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACF0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ACF0);
    cpu.pc = 0x5DD22;
    16
}
/// $08ACF0: addi.w #$70, $22(a0)
pub fn i_08acf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACF6;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACF6: subi.w #$10, $24(a0)
pub fn i_08acf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ACFC;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ACFC: lea.l $7808a.l, a1
pub fn i_08acfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD02;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08AD02: jsr $4ae.l
pub fn i_08ad02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD08);
    cpu.pc = 0x4AE;
    16
}
/// $08AD08: jsr $5dd22.l
pub fn i_08ad08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD0E);
    cpu.pc = 0x5DD22;
    16
}
/// $08AD0E: addi.w #$90, $22(a0)
pub fn i_08ad0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD14;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD14: subi.w #$40, $24(a0)
pub fn i_08ad14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD1A;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD1A: lea.l $7808a.l, a1
pub fn i_08ad1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08AD20: jsr $4ae.l
pub fn i_08ad20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD26);
    cpu.pc = 0x4AE;
    16
}
/// $08AD26: jsr $5dd22.l
pub fn i_08ad26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD2C);
    cpu.pc = 0x5DD22;
    16
}
/// $08AD2C: addi.w #$c0, $22(a0)
pub fn i_08ad2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD32;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC0, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD32: subi.w #$30, $24(a0)
pub fn i_08ad32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD38;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD38: lea.l $7808a.l, a1
pub fn i_08ad38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD3E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7808A;
    20
}
/// $08AD3E: jsr $4ae.l
pub fn i_08ad3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD44);
    cpu.pc = 0x4AE;
    16
}
/// $08AD44: jsr $5dd22.l
pub fn i_08ad44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD4A);
    cpu.pc = 0x5DD22;
    16
}
/// $08AD4A: addi.w #$100, $22(a0)
pub fn i_08ad4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD50;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x100, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD50: subi.w #$40, $24(a0)
pub fn i_08ad50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD56;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08AD56: lea.l $2ef286.l, a1
pub fn i_08ad56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD5C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF286;
    20
}
/// $08AD5C: jsr $77c7e.l
pub fn i_08ad5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD62);
    cpu.pc = 0x77C7E;
    16
}
/// $08AD62: lea.l $2ef564.l, a2
pub fn i_08ad62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD68;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF564;
    20
}
/// $08AD68: jsr $5022a.l
pub fn i_08ad68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD6E);
    cpu.pc = 0x5022A;
    16
}
/// $08AD6E: lea.l $2ef550.l, a2
pub fn i_08ad6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD74;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF550;
    20
}
/// $08AD74: jsr $5022a.l
pub fn i_08ad74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD7A);
    cpu.pc = 0x5022A;
    16
}
/// $08AD7A: lea.l $2ef5b4.l, a2
pub fn i_08ad7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD80;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF5B4;
    20
}
/// $08AD80: jsr $5022a.l
pub fn i_08ad80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD86);
    cpu.pc = 0x5022A;
    16
}
/// $08AD86: lea.l $2ef740.l, a1
pub fn i_08ad86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF740;
    20
}
/// $08AD8C: jsr $43fac.l
pub fn i_08ad8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD92);
    cpu.pc = 0x43FAC;
    16
}
/// $08AD92: lea.l $2ef77a.l, a1
pub fn i_08ad92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD98;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF77A;
    20
}
/// $08AD98: jsr $43fac.l
pub fn i_08ad98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AD9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AD9E);
    cpu.pc = 0x43FAC;
    16
}
/// $08AD9E: lea.l $2ef79a.l, a1
pub fn i_08ad9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADA4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF79A;
    20
}
/// $08ADA4: jsr $43fac.l
pub fn i_08ada4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADAA);
    cpu.pc = 0x43FAC;
    16
}
/// $08ADAA: lea.l $2ef704.l, a1
pub fn i_08adaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADB0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF704;
    20
}
/// $08ADB0: jsr $43fac.l
pub fn i_08adb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADB6);
    cpu.pc = 0x43FAC;
    16
}
/// $08ADB6: move.b #$ff, $82(a6)
pub fn i_08adb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADBC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ADBC: move.b #$ff, $86(a6)
pub fn i_08adbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADC2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ADC2: lea.l $8ae0c(pc), a1
pub fn i_08adc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADC6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE0C;
    16
}
/// $08ADC6: jsr $4ae.l
pub fn i_08adc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADCC);
    cpu.pc = 0x4AE;
    16
}
/// $08ADCC: jsr $5dd22.l
pub fn i_08adcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADD2);
    cpu.pc = 0x5DD22;
    16
}
/// $08ADD2: addi.w #$20, $22(a0)
pub fn i_08add2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADD8;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ADD8: lea.l $8b07c(pc), a1
pub fn i_08add8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B07C;
    16
}
/// $08ADDC: jsr $4ae.l
pub fn i_08addc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADE2);
    cpu.pc = 0x4AE;
    16
}
/// $08ADE2: jsr $5dd22.l
pub fn i_08ade2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8ADE8);
    cpu.pc = 0x5DD22;
    16
}
/// $08ADE8: addi.w #$90, $22(a0)
pub fn i_08ade8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADEE;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x90, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ADEE: subi.w #$10, $24(a0)
pub fn i_08adee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADF4;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08ADF4: move.b #$1, $21(a0)
pub fn i_08adf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8ADFA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08ADFA: move.b #$ff, $20(a6)
pub fn i_08adfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE00;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08AE00: jsr $8b50e(pc)
pub fn i_08ae00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE04;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8AE04);
    cpu.pc = 0x8B50E;
    16
}
/// $08AE04: jmp $518.l
pub fn i_08ae04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE0A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08AE0A: rts
pub fn i_08ae0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_FlagSet_08ae0c  ($08AE0C, 36 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE0C: lea.l $2eeec8.l, a0
pub fn i_08ae0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE12;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEEC8;
    20
}
/// $08AE12: move.l a0, $4c(a6)
pub fn i_08ae12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE16;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08AE16: jsr $283ca.l
pub fn i_08ae16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE1C);
    cpu.pc = 0x283CA;
    16
}
/// $08AE1C: jsr $283ca.l
pub fn i_08ae1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE22);
    cpu.pc = 0x283CA;
    16
}
/// $08AE22: jsr $283d8.l
pub fn i_08ae22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE28);
    cpu.pc = 0x283D8;
    16
}
/// $08AE28: move.b #$1, $10e39e.l
pub fn i_08ae28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE30;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
// ===== SetTaskHandler_08ae30  ($08AE30, 8 B, kernel/task_handlers.c)
/// $08AE30: lea.l $8ae38(pc), a1
pub fn i_08ae30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE34;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AE38;
    16
}
/// $08AE34: move.l a1, (a6)
pub fn i_08ae34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE36;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AE36: rts
pub fn i_08ae36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE38;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_FlagClear_08ae38  ($08AE38, 28 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE38: move.b #$0, $10e39e.l
pub fn i_08ae38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE40;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10E39E, v1 as u8);
    16
}
/// $08AE40: lea.l $ffff.w, a0
pub fn i_08ae40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE44;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08AE44: move.l a0, $4c(a6)
pub fn i_08ae44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE48;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08AE48: jsr $283ca.l
pub fn i_08ae48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE4E);
    cpu.pc = 0x283CA;
    16
}
/// $08AE4E: jmp $518.l
pub fn i_08ae4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE54;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08ae54  ($08AE54, 2 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE54: rts
pub fn i_08ae54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE56;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerPort_Init_08ae56  ($08AE56, 10 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE56: cmpi.w #$140, $22(a6)
pub fn i_08ae56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE5C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $08AE5C: blt.w $8ae68
pub fn i_08ae5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE60;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8AE68; return 10; }
    8
}
// ===== JsrAbsThunk_08ae60  ($08AE60, 8 B, thunks/jsr_abs_thunks.c)
/// $08AE60: jsr $2783a.l
pub fn i_08ae60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE66;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE66);
    cpu.pc = 0x2783A;
    16
}
/// $08AE66: rts
pub fn i_08ae66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerPort_Active_08ae68  ($08AE68, 246 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AE68: move.w #$14, $66(a6)
pub fn i_08ae68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08AE6E: lea.l $2eebf8.l, a0
pub fn i_08ae6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE74;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EEBF8;
    20
}
/// $08AE74: move.l a0, $48(a6)
pub fn i_08ae74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE78;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08AE78: lea.l $2ee5be.l, a0
pub fn i_08ae78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE5BE;
    20
}
/// $08AE7E: jsr $28cd4.l
pub fn i_08ae7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE84);
    cpu.pc = 0x28CD4;
    16
}
/// $08AE84: cmpi.b #$0, $21(a6)
pub fn i_08ae84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE8A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08AE8A: beq.w $8aec0
pub fn i_08ae8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE8E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AEC0; return 10; }
    8
}
/// $08AE8E: move.w #$1dd, d1
pub fn i_08ae8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE92;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1DD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AE92: jsr $236e.l
pub fn i_08ae92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AE98);
    cpu.pc = 0x236E;
    16
}
/// $08AE98: move.w #$d4, d1
pub fn i_08ae98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AE9C;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AE9C: jsr $236e.l
pub fn i_08ae9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEA2);
    cpu.pc = 0x236E;
    16
}
/// $08AEA2: lea.l $2ee58e.l, a0
pub fn i_08aea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE58E;
    20
}
/// $08AEA8: jsr $28cd4.l
pub fn i_08aea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEAE);
    cpu.pc = 0x28CD4;
    16
}
/// $08AEAE: move.w #$8000, $38(a6)
pub fn i_08aeae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08AEB4: jsr $267e2.l
pub fn i_08aeb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEBA);
    cpu.pc = 0x267E2;
    16
}
/// $08AEBA: jsr $27cee.l
pub fn i_08aeba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEC0);
    cpu.pc = 0x27CEE;
    16
}
/// $08AEC0: lea.l $8aec6(pc), a1
pub fn i_08aec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AEC6;
    16
}
/// $08AEC4: move.l a1, (a6)
pub fn i_08aec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AEC6: movea.l $c(a6), a0
pub fn i_08aec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AECA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08AECA: cmpi.b #$ff, $20(a0)
pub fn i_08aeca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AED0;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08AED0: beq.w $8af58
pub fn i_08aed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AED4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AF58; return 10; }
    8
}
/// $08AED4: move.w $18(a6), $14(a6)
pub fn i_08aed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEDA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08AEDA: cmpi.b #$0, $21(a6)
pub fn i_08aeda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEE0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08AEE0: beq.w $8aef4
pub fn i_08aee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEE4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AEF4; return 10; }
    8
}
/// $08AEE4: cmpi.b #$ff, $87(a0)
pub fn i_08aee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEEA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08AEEA: bne.w $8aef4
pub fn i_08aeea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8AEF4; return 10; }
    8
}
/// $08AEEE: move.w $16(a6), $14(a6)
pub fn i_08aeee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEF4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08AEF4: jsr $2783a.l
pub fn i_08aef4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AEFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AEFA);
    cpu.pc = 0x2783A;
    16
}
/// $08AEFA: jsr $28d70.l
pub fn i_08aefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF00);
    cpu.pc = 0x28D70;
    16
}
/// $08AF00: jsr $2870a.l
pub fn i_08af00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF06;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF06);
    cpu.pc = 0x2870A;
    16
}
/// $08AF06: bcc.w $8af1c
pub fn i_08af06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AF1C; return 10; }
    8
}
/// $08AF0A: lea.l $5e766.l, a0
pub fn i_08af0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AF10: jsr $5e770.l
pub fn i_08af10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF16);
    cpu.pc = 0x5E770;
    16
}
/// $08AF16: bclr.b #$3, $13(a6)
pub fn i_08af16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF1C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AF1C: jsr $28758.l
pub fn i_08af1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF22);
    cpu.pc = 0x28758;
    16
}
/// $08AF22: bcc.w $8af66
pub fn i_08af22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AF66; return 10; }
    8
}
/// $08AF26: bclr.b #$0, $13(a6)
pub fn i_08af26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF2C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AF2C: move.w #$1027, d0
pub fn i_08af2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF30;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AF30: jsr $2352.l
pub fn i_08af30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF36);
    cpu.pc = 0x2352;
    16
}
/// $08AF36: movea.l $70(a6), a2
pub fn i_08af36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF3A;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $08AF3A: jsr $5022a.l
pub fn i_08af3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF40);
    cpu.pc = 0x5022A;
    16
}
/// $08AF40: lea.l $2ef274.l, a1
pub fn i_08af40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF46;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF274;
    20
}
/// $08AF46: jsr $77c7e.l
pub fn i_08af46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF4C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF4C);
    cpu.pc = 0x77C7E;
    16
}
/// $08AF4C: cmpi.l #$ffffffff, $74(a6)
pub fn i_08af4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF54;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $08AF54: bne.w $8af60
pub fn i_08af54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF58;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8AF60; return 10; }
    8
}
/// $08AF58: jmp $518.l
pub fn i_08af58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF5E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Rts_08af5e  ($08AF5E, 2 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AF5E: rts
pub fn i_08af5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetTaskHandler_08af60  ($08AF60, 8 B, kernel/task_handlers.c)
/// $08AF60: lea.l $8af68(pc), a1
pub fn i_08af60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AF68;
    16
}
/// $08AF64: move.l a1, (a6)
pub fn i_08af64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AF66: rts
pub fn i_08af66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF68;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerPort_Stage2_08af68  ($08AF68, 196 B, enemies/bosses/scene5_airship_088a64.s)
/// $08AF68: move.w #$14, $66(a6)
pub fn i_08af68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF6E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x14;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08AF6E: cmpi.b #$0, $21(a6)
pub fn i_08af6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF74;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08AF74: beq.w $8af84
pub fn i_08af74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF78;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AF84; return 10; }
    8
}
/// $08AF78: lea.l $2ee59e.l, a0
pub fn i_08af78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF7E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE59E;
    20
}
/// $08AF7E: jsr $28cd4.l
pub fn i_08af7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AF84);
    cpu.pc = 0x28CD4;
    16
}
/// $08AF84: lea.l $8af8a(pc), a1
pub fn i_08af84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8AF8A;
    16
}
/// $08AF88: move.l a1, (a6)
pub fn i_08af88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08AF8A: movea.l $c(a6), a0
pub fn i_08af8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF8E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08AF8E: cmpi.b #$ff, $20(a0)
pub fn i_08af8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF94;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08AF94: beq.w $8b034
pub fn i_08af94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B034; return 10; }
    8
}
/// $08AF98: move.w $18(a6), $14(a6)
pub fn i_08af98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AF9E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08AF9E: cmpi.b #$0, $21(a6)
pub fn i_08af9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFA4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08AFA4: beq.w $8afb8
pub fn i_08afa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFA8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8AFB8; return 10; }
    8
}
/// $08AFA8: cmpi.b #$ff, $87(a0)
pub fn i_08afa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFAE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08AFAE: bne.w $8afb8
pub fn i_08afae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8AFB8; return 10; }
    8
}
/// $08AFB2: move.w $16(a6), $14(a6)
pub fn i_08afb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFB8;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08AFB8: jsr $2783a.l
pub fn i_08afb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFBE);
    cpu.pc = 0x2783A;
    16
}
/// $08AFBE: jsr $28d70.l
pub fn i_08afbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFC4);
    cpu.pc = 0x28D70;
    16
}
/// $08AFC4: jsr $2870a.l
pub fn i_08afc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFCA);
    cpu.pc = 0x2870A;
    16
}
/// $08AFCA: bcc.w $8afe0
pub fn i_08afca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8AFE0; return 10; }
    8
}
/// $08AFCE: lea.l $5e766.l, a0
pub fn i_08afce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08AFD4: jsr $5e770.l
pub fn i_08afd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFDA);
    cpu.pc = 0x5E770;
    16
}
/// $08AFDA: bclr.b #$3, $13(a6)
pub fn i_08afda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFE0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08AFE0: jsr $28758.l
pub fn i_08afe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFE6);
    cpu.pc = 0x28758;
    16
}
/// $08AFE6: bcc.w $8b03a
pub fn i_08afe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B03A; return 10; }
    8
}
/// $08AFEA: move.l #$1000, d0
pub fn i_08afea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFF0;
    cpu.ir = 0x203C;
    let v1: u32 = 0x1000;
    cpu.d[0] = v1;
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08AFF0: jsr $51a28.l
pub fn i_08aff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8AFF6);
    cpu.pc = 0x51A28;
    16
}
/// $08AFF6: move.w #$1027, d0
pub fn i_08aff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8AFFA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08AFFA: jsr $2352.l
pub fn i_08affa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B000;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B000);
    cpu.pc = 0x2352;
    16
}
/// $08B000: movea.l $74(a6), a2
pub fn i_08b000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B004;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $08B004: jsr $5022a.l
pub fn i_08b004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B00A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B00A);
    cpu.pc = 0x5022A;
    16
}
/// $08B00A: lea.l $2ef274.l, a1
pub fn i_08b00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B010;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF274;
    20
}
/// $08B010: jsr $77c7e.l
pub fn i_08b010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B016;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B016);
    cpu.pc = 0x77C7E;
    16
}
/// $08B016: cmpi.b #$0, $21(a6)
pub fn i_08b016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B01C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B01C: beq.w $8b034
pub fn i_08b01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B020;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B034; return 10; }
    8
}
/// $08B020: lea.l $2ee5ae.l, a0
pub fn i_08b020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B026;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE5AE;
    20
}
/// $08B026: jsr $28cd4.l
pub fn i_08b026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B02C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B02C);
    cpu.pc = 0x28CD4;
    16
}
// ===== SetTaskHandler_08b02c  ($08B02C, 8 B, kernel/task_handlers.c)
/// $08B02C: lea.l $8b03c(pc), a1
pub fn i_08b02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B030;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B03C;
    16
}
/// $08B030: move.l a1, (a6)
pub fn i_08b030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B032;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B032: rts
pub fn i_08b032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B034;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_08b034  ($08B034, 8 B, kernel/jmp_to_scheduler.c)
/// $08B034: jmp $518.l
pub fn i_08b034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B03A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B03A: rts
pub fn i_08b03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B03C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TowerPort_Idle_08b03c  ($08B03C, 48 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B03C: lea.l $8b042(pc), a1
pub fn i_08b03c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B040;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B042;
    16
}
/// $08B040: move.l a1, (a6)
pub fn i_08b040(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B042;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B042: movea.l $c(a6), a0
pub fn i_08b042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B046;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08B046: cmpi.b #$ff, $20(a0)
pub fn i_08b046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B04C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08B04C: beq.w $8b074
pub fn i_08b04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B050;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B074; return 10; }
    8
}
/// $08B050: move.w $18(a6), $14(a6)
pub fn i_08b050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B056;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B056: cmpi.b #$ff, $87(a0)
pub fn i_08b056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B05C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08B05C: bne.w $8b066
pub fn i_08b05c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B060;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B066; return 10; }
    8
}
/// $08B060: move.w $16(a6), $14(a6)
pub fn i_08b060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B066;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B066: jsr $2783a.l
pub fn i_08b066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B06C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B06C);
    cpu.pc = 0x2783A;
    16
}
// ===== JsrAbsThunk_08b06c  ($08B06C, 8 B, thunks/jsr_abs_thunks.c)
/// $08B06C: jsr $28d70.l
pub fn i_08b06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B072;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B072);
    cpu.pc = 0x28D70;
    16
}
/// $08B072: rts
pub fn i_08b072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B074;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_08b074  ($08B074, 8 B, kernel/jmp_to_scheduler.c)
/// $08B074: jmp $518.l
pub fn i_08b074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B07A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B07A: rts
pub fn i_08b07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B07C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_SmokeRise_08b07c  ($08B07C, 134 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B07C: move.w #$d5, d1
pub fn i_08b07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B080;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B080: jsr $236e.l
pub fn i_08b080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B086;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B086);
    cpu.pc = 0x236E;
    16
}
/// $08B086: move.b #$ff, $32(a6)
pub fn i_08b086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B08C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B08C: move.b #$50, $33(a6)
pub fn i_08b08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B092;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B092: move.b #$0, $3a(a6)
pub fn i_08b092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B098;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B098: move.w #$0, $38(a6)
pub fn i_08b098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B09E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B09E: jsr $267e2.l
pub fn i_08b09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0A4);
    cpu.pc = 0x267E2;
    16
}
/// $08B0A4: move.w #$fffa, $2e(a6)
pub fn i_08b0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0AA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B0AA: clr.l d0
pub fn i_08b0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0AC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B0AC: move.b $21(a6), d0
pub fn i_08b0ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0B0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B0B0: movea.l #$2ee2be, a0
pub fn i_08b0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0B6;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE2BE;
    cpu.a[0] = v1;
    4
}
/// $08B0B6: lsl.w #$2, d0
pub fn i_08b0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0B8;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08B0B8: movea.l (a0, d0.w), a0
pub fn i_08b0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0BC;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08B0BC: cmpa.l #$ffffffff, a0
pub fn i_08b0bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0C2;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08B0C2: beq.w $8b0cc
pub fn i_08b0c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B0CC; return 10; }
    8
}
/// $08B0C6: jsr $28cd4.l
pub fn i_08b0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0CC);
    cpu.pc = 0x28CD4;
    16
}
/// $08B0CC: move.w #$1026, d0
pub fn i_08b0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0D0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1026;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B0D0: jsr $2352.l
pub fn i_08b0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0D6);
    cpu.pc = 0x2352;
    16
}
/// $08B0D6: lea.l $8b0dc(pc), a1
pub fn i_08b0d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B0DC;
    16
}
/// $08B0DA: move.l a1, (a6)
pub fn i_08b0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B0DC: jsr $27cee.l
pub fn i_08b0dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0E2);
    cpu.pc = 0x27CEE;
    16
}
/// $08B0E2: cmpi.b #$ff, $33(a6)
pub fn i_08b0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0E8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08B0E8: beq.w $8b0f2
pub fn i_08b0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B0F2; return 10; }
    8
}
/// $08B0EC: addi.b #$1, $33(a6)
pub fn i_08b0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0F2;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B0F2: jsr $28d70.l
pub fn i_08b0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B0F8);
    cpu.pc = 0x28D70;
    16
}
/// $08B0F8: cmpi.w #$170, $24(a6)
pub fn i_08b0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B0FE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $08B0FE: bgt.w $8b108
pub fn i_08b0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B102;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8B108; return 10; }
    8
}
// ===== SetTaskHandler_08b102  ($08B102, 8 B, kernel/task_handlers.c)
/// $08B102: lea.l $8b10a(pc), a1
pub fn i_08b102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B106;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B10A;
    16
}
/// $08B106: move.l a1, (a6)
pub fn i_08b106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B108;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B108: rts
pub fn i_08b108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B10A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_SparkBurst_08b10a  ($08B10A, 232 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B10A: move.w #$1051, d0
pub fn i_08b10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B10E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1051;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B10E: jsr $2352.l
pub fn i_08b10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B114;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B114);
    cpu.pc = 0x2352;
    16
}
/// $08B114: lea.l $8b1f2(pc), a1
pub fn i_08b114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B118;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B118: jsr $4ae.l
pub fn i_08b118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B11E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B11E);
    cpu.pc = 0x4AE;
    16
}
/// $08B11E: jsr $5dd22.l
pub fn i_08b11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B124;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B124);
    cpu.pc = 0x5DD22;
    16
}
/// $08B124: addi.w #$70, $22(a0)
pub fn i_08b124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B12A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x70, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B12A: move.w #$125, $24(a0)
pub fn i_08b12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B130;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B130: move.w #$0, $72(a0)
pub fn i_08b130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B136;
    cpu.ir = 0x317C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B136: lea.l $8b1f2(pc), a1
pub fn i_08b136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B13A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B13A: jsr $4ae.l
pub fn i_08b13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B140;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B140);
    cpu.pc = 0x4AE;
    16
}
/// $08B140: jsr $5dd22.l
pub fn i_08b140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B146);
    cpu.pc = 0x5DD22;
    16
}
/// $08B146: addi.w #$40, $22(a0)
pub fn i_08b146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B14C;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B14C: move.w #$125, $24(a0)
pub fn i_08b14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B152;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B152: move.w #$6, $72(a0)
pub fn i_08b152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B158;
    cpu.ir = 0x317C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B158: lea.l $8b1f2(pc), a1
pub fn i_08b158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B15C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B15C: jsr $4ae.l
pub fn i_08b15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B162);
    cpu.pc = 0x4AE;
    16
}
/// $08B162: jsr $5dd22.l
pub fn i_08b162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B168;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B168);
    cpu.pc = 0x5DD22;
    16
}
/// $08B168: addi.w #$10, $22(a0)
pub fn i_08b168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B16E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B16E: move.w #$125, $24(a0)
pub fn i_08b16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B174;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B174: move.w #$c, $72(a0)
pub fn i_08b174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B17A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B17A: lea.l $8b1f2(pc), a1
pub fn i_08b17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B17E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B17E: jsr $4ae.l
pub fn i_08b17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B184);
    cpu.pc = 0x4AE;
    16
}
/// $08B184: jsr $5dd22.l
pub fn i_08b184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B18A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B18A);
    cpu.pc = 0x5DD22;
    16
}
/// $08B18A: subi.w #$20, $22(a0)
pub fn i_08b18a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B190;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B190: move.w #$125, $24(a0)
pub fn i_08b190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B196;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B196: move.w #$12, $72(a0)
pub fn i_08b196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B19C;
    cpu.ir = 0x317C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B19C: lea.l $8b1f2(pc), a1
pub fn i_08b19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1A0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1F2;
    16
}
/// $08B1A0: jsr $4ae.l
pub fn i_08b1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1A6);
    cpu.pc = 0x4AE;
    16
}
/// $08B1A6: jsr $5dd22.l
pub fn i_08b1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1AC);
    cpu.pc = 0x5DD22;
    16
}
/// $08B1AC: subi.w #$50, $22(a0)
pub fn i_08b1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1B2;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B1B2: move.w #$125, $24(a0)
pub fn i_08b1b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1B8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x125;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B1B8: move.w #$18, $72(a0)
pub fn i_08b1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1BE;
    cpu.ir = 0x317C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B1BE: lea.l $8b1c4(pc), a1
pub fn i_08b1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B1C4;
    16
}
/// $08B1C2: move.l a1, (a6)
pub fn i_08b1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B1C4: jsr $27cee.l
pub fn i_08b1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1CA);
    cpu.pc = 0x27CEE;
    16
}
/// $08B1CA: cmpi.b #$ff, $33(a6)
pub fn i_08b1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1D0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $08B1D0: beq.w $8b1da
pub fn i_08b1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B1DA; return 10; }
    8
}
/// $08B1D4: addi.b #$1, $33(a6)
pub fn i_08b1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1DA;
    cpu.ir = 0x62E;
    let ad1 = cpu.a[6].wrapping_add(0x33);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B1DA: jsr $28d70.l
pub fn i_08b1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1E0);
    cpu.pc = 0x28D70;
    16
}
/// $08B1E0: cmpi.w #$80, $24(a6)
pub fn i_08b1e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1E6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x80, Size::Word, false);
    16
}
/// $08B1E6: bgt.w $8b1f0
pub fn i_08b1e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1EA;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8B1F0; return 10; }
    8
}
/// $08B1EA: jmp $518.l
pub fn i_08b1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1F0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B1F0: rts
pub fn i_08b1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1F2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Wreck_Spark_08b1f2  ($08B1F2, 102 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B1F2: jsr $2783a.l
pub fn i_08b1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B1F8);
    cpu.pc = 0x2783A;
    16
}
/// $08B1F8: subq.w #$1, $72(a6)
pub fn i_08b1f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B1FC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B1FC: cmpi.w #$0, $72(a6)
pub fn i_08b1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B202;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08B202: ble.w $8b208
pub fn i_08b202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B206;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B208; return 10; }
    8
}
/// $08B206: rts
pub fn i_08b206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B208;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $08B208: move.w #$139, d1
pub fn i_08b208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B20C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x139;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B20C: jsr $236e.l
pub fn i_08b20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B212;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B212);
    cpu.pc = 0x236E;
    16
}
/// $08B212: move.b #$ff, $32(a6)
pub fn i_08b212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B218;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B218: move.b #$ff, $33(a6)
pub fn i_08b218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B21E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B21E: lea.l $2ded26.l, a0
pub fn i_08b21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B224;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2DED26;
    20
}
/// $08B224: jsr $28cd4.l
pub fn i_08b224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B22A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B22A);
    cpu.pc = 0x28CD4;
    16
}
/// $08B22A: lea.l $8b230(pc), a1
pub fn i_08b22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B22E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B230;
    16
}
/// $08B22E: move.l a1, (a6)
pub fn i_08b22e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B230;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B230: jsr $44048.l
pub fn i_08b230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B236);
    cpu.pc = 0x44048;
    16
}
/// $08B236: jsr $28d70.l
pub fn i_08b236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B23C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B23C);
    cpu.pc = 0x28D70;
    16
}
/// $08B23C: bcs.w $8b250
pub fn i_08b23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B240;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8B250; return 10; }
    8
}
/// $08B240: lea.l $2ef7a8.l, a0
pub fn i_08b240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B246;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7A8;
    20
}
/// $08B246: jsr $5dd56.l
pub fn i_08b246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B24C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B24C);
    cpu.pc = 0x5DD56;
    16
}
/// $08B24C: bcc.w $8b256
pub fn i_08b24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B250;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B256; return 10; }
    8
}
/// $08B250: jmp $518.l
pub fn i_08b250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B256;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B256: rts
pub fn i_08b256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B258;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Thrown_08b258  ($08B258, 244 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B258: cmpi.b #$0, $9c(a6)
pub fn i_08b258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B25E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B25E: bne.w $8b270
pub fn i_08b25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B262;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B270; return 10; }
    8
}
/// $08B262: move.w #$162, d1
pub fn i_08b262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B266;
    cpu.ir = 0x323C;
    let v1: u32 = 0x162;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B266: jsr $236e.l
pub fn i_08b266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B26C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B26C);
    cpu.pc = 0x236E;
    16
}
/// $08B26C: bra.w $8b27a
pub fn i_08b26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B270;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B27A;
    10
}
/// $08B270: move.w #$d8, d1
pub fn i_08b270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B274;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B274: jsr $236e.l
pub fn i_08b274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B27A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B27A);
    cpu.pc = 0x236E;
    16
}
/// $08B27A: move.b #$ff, $32(a6)
pub fn i_08b27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B280;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B280: move.b #$ff, $33(a6)
pub fn i_08b280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B286;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B286: jsr $267e2.l
pub fn i_08b286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B28C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B28C);
    cpu.pc = 0x267E2;
    16
}
/// $08B28C: jsr $8f002.l
pub fn i_08b28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B292);
    cpu.pc = 0x8F002;
    16
}
/// $08B292: jsr $8f010.l
pub fn i_08b292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B298);
    cpu.pc = 0x8F010;
    16
}
/// $08B298: cmpi.b #$1, $9a(a6)
pub fn i_08b298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B29E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08B29E: bne.w $8b2b2
pub fn i_08b29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B2B2; return 10; }
    8
}
/// $08B2A2: bset.b #$6, $12(a6)
pub fn i_08b2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2A8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B2A8: move.w #$ffff, $38(a6)
pub fn i_08b2a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2AE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B2AE: bra.w $8b2b8
pub fn i_08b2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B2B8;
    10
}
/// $08B2B2: move.w #$8000, $38(a6)
pub fn i_08b2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2B8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B2B8: clr.l d0
pub fn i_08b2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2BA;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B2BA: move.b $9d(a6), d0
pub fn i_08b2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2BE;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B2BE: movea.l #$2ee7a8, a0
pub fn i_08b2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2C4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2EE7A8;
    cpu.a[0] = v1;
    4
}
/// $08B2C4: lsl.w #$2, d0
pub fn i_08b2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2C6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08B2C6: movea.l (a0, d0.w), a0
pub fn i_08b2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2CA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08B2CA: cmpa.l #$ffffffff, a0
pub fn i_08b2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2D0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08B2D0: beq.w $8b2da
pub fn i_08b2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2D4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B2DA; return 10; }
    8
}
/// $08B2D4: jsr $28cd4.l
pub fn i_08b2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B2DA);
    cpu.pc = 0x28CD4;
    16
}
/// $08B2DA: lea.l $8b2e0(pc), a1
pub fn i_08b2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2DE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B2E0;
    16
}
/// $08B2DE: move.l a1, (a6)
pub fn i_08b2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2E0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B2E0: jsr $27cee.l
pub fn i_08b2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B2E6);
    cpu.pc = 0x27CEE;
    16
}
/// $08B2E6: jsr $28d70.l
pub fn i_08b2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B2EC);
    cpu.pc = 0x28D70;
    16
}
/// $08B2EC: cmpi.b #$0, $9b(a6)
pub fn i_08b2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2F2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B2F2: bne.w $8b2fe
pub fn i_08b2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B2FE; return 10; }
    8
}
/// $08B2F6: lea.l $ffff.w, a0
pub fn i_08b2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2FA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $08B2FA: move.l a0, $48(a6)
pub fn i_08b2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B2FE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B2FE: jsr $2870a.l
pub fn i_08b2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B304;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B304);
    cpu.pc = 0x2870A;
    16
}
/// $08B304: bcc.w $8b31e
pub fn i_08b304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B308;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B31E; return 10; }
    8
}
/// $08B308: lea.l $5e766.l, a0
pub fn i_08b308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B30E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $08B30E: jsr $5e770.l
pub fn i_08b30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B314);
    cpu.pc = 0x5E770;
    16
}
/// $08B314: bclr.b #$3, $13(a6)
pub fn i_08b314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B31A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B31A: jsr $8b558(pc)
pub fn i_08b31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B31E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B31E);
    cpu.pc = 0x8B558;
    16
}
/// $08B31E: cmpi.b #$0, $9b(a6)
pub fn i_08b31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B324;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B324: beq.w $8b334
pub fn i_08b324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B328;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B334; return 10; }
    8
}
/// $08B328: jsr $283ca.l
pub fn i_08b328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B32E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B32E);
    cpu.pc = 0x283CA;
    16
}
/// $08B32E: jsr $283d8.l
pub fn i_08b32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B334;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B334);
    cpu.pc = 0x283D8;
    16
}
/// $08B334: lea.l $2ef7bc.l, a0
pub fn i_08b334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B33A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7BC;
    20
}
/// $08B33A: jsr $5dd5c.l
pub fn i_08b33a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B340;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B340);
    cpu.pc = 0x5DD5C;
    16
}
/// $08B340: bcc.w $8b34a
pub fn i_08b340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B344;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B34A; return 10; }
    8
}
/// $08B344: jmp $518.l
pub fn i_08b344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B34A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B34A: rts
pub fn i_08b34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B34C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Drop_V0_08b34c  ($08B34C, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B34C: lea.l $2ee194.l, a0
pub fn i_08b34c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B352;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE194;
    20
}
/// $08B352: jsr $28cd4.l
pub fn i_08b352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B358;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B358);
    cpu.pc = 0x28CD4;
    16
}
/// $08B358: lea.l $2ef7c6.l, a0
pub fn i_08b358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B35E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7C6;
    20
}
/// $08B35E: move.l a0, $70(a6)
pub fn i_08b35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B362;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B362: bra.w $8b3b4
pub fn i_08b362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B366;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B3B4;
    10
}
// ===== Proj_Drop_V1_08b366  ($08B366, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B366: lea.l $2ee1a0.l, a0
pub fn i_08b366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B36C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1A0;
    20
}
/// $08B36C: jsr $28cd4.l
pub fn i_08b36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B372;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B372);
    cpu.pc = 0x28CD4;
    16
}
/// $08B372: lea.l $2ef7d0.l, a0
pub fn i_08b372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B378;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7D0;
    20
}
/// $08B378: move.l a0, $70(a6)
pub fn i_08b378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B37C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B37C: bra.w $8b3b4
pub fn i_08b37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B380;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B3B4;
    10
}
// ===== Proj_Drop_V2_08b380  ($08B380, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B380: lea.l $2ee1ac.l, a0
pub fn i_08b380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B386;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1AC;
    20
}
/// $08B386: jsr $28cd4.l
pub fn i_08b386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B38C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B38C);
    cpu.pc = 0x28CD4;
    16
}
/// $08B38C: lea.l $2ef7da.l, a0
pub fn i_08b38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B392;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7DA;
    20
}
/// $08B392: move.l a0, $70(a6)
pub fn i_08b392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B396;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B396: bra.w $8b3b4
pub fn i_08b396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B39A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B3B4;
    10
}
// ===== Proj_Drop_V3_08b39a  ($08B39A, 194 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B39A: lea.l $2ee1b8.l, a0
pub fn i_08b39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE1B8;
    20
}
/// $08B3A0: jsr $28cd4.l
pub fn i_08b3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3A6);
    cpu.pc = 0x28CD4;
    16
}
/// $08B3A6: lea.l $2ef7e4.l, a0
pub fn i_08b3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF7E4;
    20
}
/// $08B3AC: move.l a0, $70(a6)
pub fn i_08b3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3B0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08B3B0: bra.w $8b3b4
pub fn i_08b3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B3B4;
    10
}
/// $08B3B4: move.b $9a(a6), $32(a6)
pub fn i_08b3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3BA;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08B3BA: move.b $9a(a6), $33(a6)
pub fn i_08b3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3C0;
    cpu.ir = 0x1D6E;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08B3C0: jsr $267e2.l
pub fn i_08b3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3C6);
    cpu.pc = 0x267E2;
    16
}
/// $08B3C6: jsr $8f002.l
pub fn i_08b3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3CC);
    cpu.pc = 0x8F002;
    16
}
/// $08B3CC: jsr $8f010.l
pub fn i_08b3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3D2);
    cpu.pc = 0x8F010;
    16
}
/// $08B3D2: move.l $106f50.l, d0
pub fn i_08b3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3D8;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08B3D8: swap d0
pub fn i_08b3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3DA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08B3DA: cmpi.b #$0, $9b(a6)
pub fn i_08b3da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3E0;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B3E0: bne.w $8b40e
pub fn i_08b3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3E4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B40E; return 10; }
    8
}
/// $08B3E4: move.w #$0, $38(a6)
pub fn i_08b3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3EA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B3EA: cmpi.w #$670, d0
pub fn i_08b3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3EE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x670, Size::Word, false);
    8
}
/// $08B3EE: ble.w $8b400
pub fn i_08b3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3F2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B400; return 10; }
    8
}
/// $08B3F2: move.w #$16d, d1
pub fn i_08b3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3F6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B3F6: jsr $236e.l
pub fn i_08b3f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B3FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B3FC);
    cpu.pc = 0x236E;
    16
}
/// $08B3FC: bra.w $8b40a
pub fn i_08b3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B400;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B40A;
    10
}
/// $08B400: move.w #$16f, d1
pub fn i_08b400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B404;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B404: jsr $236e.l
pub fn i_08b404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B40A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B40A);
    cpu.pc = 0x236E;
    16
}
/// $08B40A: bra.w $8b434
pub fn i_08b40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B40E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B434;
    10
}
/// $08B40E: move.w #$1fff, $38(a6)
pub fn i_08b40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B414;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1FFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B414: cmpi.w #$670, d0
pub fn i_08b414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B418;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x670, Size::Word, false);
    8
}
/// $08B418: ble.w $8b42a
pub fn i_08b418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B41C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B42A; return 10; }
    8
}
/// $08B41C: move.w #$ce, d1
pub fn i_08b41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B420;
    cpu.ir = 0x323C;
    let v1: u32 = 0xCE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B420: jsr $236e.l
pub fn i_08b420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B426;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B426);
    cpu.pc = 0x236E;
    16
}
/// $08B426: bra.w $8b434
pub fn i_08b426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B42A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B434;
    10
}
/// $08B42A: move.w #$16e, d1
pub fn i_08b42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B42E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x16E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B42E: jsr $236e.l
pub fn i_08b42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B434);
    cpu.pc = 0x236E;
    16
}
/// $08B434: lea.l $8b43a(pc), a1
pub fn i_08b434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B438;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B43A;
    16
}
/// $08B438: move.l a1, (a6)
pub fn i_08b438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B43A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B43A: jsr $27cee.l
pub fn i_08b43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B440;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B440);
    cpu.pc = 0x27CEE;
    16
}
/// $08B440: jsr $28d70.l
pub fn i_08b440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B446;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B446);
    cpu.pc = 0x28D70;
    16
}
/// $08B446: movea.l $70(a6), a0
pub fn i_08b446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B44A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08B44A: jsr $5dd5c.l
pub fn i_08b44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B450;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B450);
    cpu.pc = 0x5DD5C;
    16
}
/// $08B450: bcc.w $8b45a
pub fn i_08b450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B454;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B45A; return 10; }
    8
}
/// $08B454: jmp $518.l
pub fn i_08b454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B45A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08B45A: rts
pub fn i_08b45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B45C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SprCb_Lamp2On_08b45c  ($08B45C, 52 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B45C: move.w #$11, d1
pub fn i_08b45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B460;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B460: move.w #$1ac, d2
pub fn i_08b460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B464;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B464: move.w #$ffff, d3
pub fn i_08b464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B468;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B468: move.w #$1, d4
pub fn i_08b468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B46C;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B46C: jsr $2c26.l
pub fn i_08b46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B472);
    cpu.pc = 0x2C26;
    16
}
/// $08B472: move.w #$12, d1
pub fn i_08b472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B476;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B476: move.w #$1ad, d2
pub fn i_08b476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B47A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B47A: move.w #$ffff, d3
pub fn i_08b47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B47E;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B47E: move.w #$1, d4
pub fn i_08b47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B482;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B482: jsr $2c26.l
pub fn i_08b482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B488;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B488);
    cpu.pc = 0x2C26;
    16
}
/// $08B488: move.w $16(a6), $14(a6)
pub fn i_08b488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B48E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B48E: rts
pub fn i_08b48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B490;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SprCb_Lamp2Off_08b490  ($08B490, 52 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B490: move.w #$11, d1
pub fn i_08b490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B494;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B494: move.w #$81, d2
pub fn i_08b494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B498;
    cpu.ir = 0x343C;
    let v1: u32 = 0x81;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B498: move.w #$ffff, d3
pub fn i_08b498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B49C;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B49C: move.w #$1, d4
pub fn i_08b49c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4A0;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4A0: jsr $2c26.l
pub fn i_08b4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B4A6);
    cpu.pc = 0x2C26;
    16
}
/// $08B4A6: move.w #$12, d1
pub fn i_08b4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4AA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4AA: move.w #$82, d2
pub fn i_08b4aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4AE;
    cpu.ir = 0x343C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4AE: move.w #$ffff, d3
pub fn i_08b4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4B2;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4B2: move.w #$1, d4
pub fn i_08b4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4B6;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4B6: jsr $2c26.l
pub fn i_08b4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B4BC);
    cpu.pc = 0x2C26;
    16
}
/// $08B4BC: move.w $18(a6), $14(a6)
pub fn i_08b4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4C2;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B4C2: rts
pub fn i_08b4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SprCb_Lamp3On_08b4c4  ($08B4C4, 74 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B4C4: move.w #$10, d1
pub fn i_08b4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4C8: move.w #$1ab, d2
pub fn i_08b4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4CC;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4CC: move.w #$ffff, d3
pub fn i_08b4cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4D0;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4D0: move.w #$1, d4
pub fn i_08b4d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4D4;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4D4: jsr $2c26.l
pub fn i_08b4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B4DA);
    cpu.pc = 0x2C26;
    16
}
/// $08B4DA: move.w #$11, d1
pub fn i_08b4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4DE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4DE: move.w #$1ac, d2
pub fn i_08b4de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4E2;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4E2: move.w #$ffff, d3
pub fn i_08b4e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4E6;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4E6: move.w #$1, d4
pub fn i_08b4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4EA;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4EA: jsr $2c26.l
pub fn i_08b4ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B4F0);
    cpu.pc = 0x2C26;
    16
}
/// $08B4F0: move.w #$12, d1
pub fn i_08b4f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4F4: move.w #$1ad, d2
pub fn i_08b4f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4F8;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1AD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4F8: move.w #$ffff, d3
pub fn i_08b4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B4FC;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B4FC: move.w #$1, d4
pub fn i_08b4fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B500;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B500: jsr $2c26.l
pub fn i_08b500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B506;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B506);
    cpu.pc = 0x2C26;
    16
}
/// $08B506: move.w $16(a6), $14(a6)
pub fn i_08b506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B50C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B50C: rts
pub fn i_08b50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B50E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SprCb_Lamp3Off_08b50e  ($08B50E, 74 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B50E: move.w #$10, d1
pub fn i_08b50e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B512;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B512: move.w #$80, d2
pub fn i_08b512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B516;
    cpu.ir = 0x343C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B516: move.w #$ffff, d3
pub fn i_08b516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B51A;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B51A: move.w #$1, d4
pub fn i_08b51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B51E;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B51E: jsr $2c26.l
pub fn i_08b51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B524;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B524);
    cpu.pc = 0x2C26;
    16
}
/// $08B524: move.w #$11, d1
pub fn i_08b524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B528;
    cpu.ir = 0x323C;
    let v1: u32 = 0x11;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B528: move.w #$81, d2
pub fn i_08b528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B52C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x81;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B52C: move.w #$ffff, d3
pub fn i_08b52c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B530;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B530: move.w #$1, d4
pub fn i_08b530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B534;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B534: jsr $2c26.l
pub fn i_08b534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B53A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B53A);
    cpu.pc = 0x2C26;
    16
}
/// $08B53A: move.w #$12, d1
pub fn i_08b53a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B53E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B53E: move.w #$82, d2
pub fn i_08b53e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B542;
    cpu.ir = 0x343C;
    let v1: u32 = 0x82;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B542: move.w #$ffff, d3
pub fn i_08b542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B546;
    cpu.ir = 0x363C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B546: move.w #$1, d4
pub fn i_08b546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B54A;
    cpu.ir = 0x383C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B54A: jsr $2c26.l
pub fn i_08b54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B550;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B550);
    cpu.pc = 0x2C26;
    16
}
/// $08B550: move.w $18(a6), $14(a6)
pub fn i_08b550(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B556;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x18);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B556: rts
pub fn i_08b556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B558;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Handler_ConditionalHitCounter_08B558  ($08B558, 54 B, entity/handler_conditional_hit_08b558.s)
/// $08B558: cmpi.b #$1, $58(a6)
pub fn i_08b558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B55E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $08B55E: bne.w $8b58c
pub fn i_08b55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B562;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B58C; return 10; }
    8
}
/// $08B562: addi.w #$a, $66(a6)
pub fn i_08b562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B568;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x66);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xA, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B568: jsr $28758.l
pub fn i_08b568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B56E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B56E);
    cpu.pc = 0x28758;
    16
}
/// $08B56E: bcc.w $8b578
pub fn i_08b56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B572;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B578; return 10; }
    8
}
/// $08B572: bclr.b #$0, $13(a6)
pub fn i_08b572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B578;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B578: move.b $106f28.l, d0
pub fn i_08b578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B57E;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B57E: andi.b #$1, d0
pub fn i_08b57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B582;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08B582: beq.w $8b58c
pub fn i_08b582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B586;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B58C; return 10; }
    8
}
/// $08B586: jsr $6e224.l
pub fn i_08b586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B58C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B58C);
    cpu.pc = 0x6E224;
    16
}
/// $08B58C: rts
pub fn i_08b58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B58E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Math_AbsW_08b58e  ($08B58E, 16 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B58E: move.w d0, d1
pub fn i_08b58e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B590;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B590: asr.w #$8, d1
pub fn i_08b590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B592;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08B592: btst.b #$7, d1
pub fn i_08b592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B596;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B596: beq.w $8b59c
pub fn i_08b596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B59A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B59C; return 10; }
    8
}
/// $08B59A: neg.w d0
pub fn i_08b59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B59C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B59C: rts
pub fn i_08b59c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B59E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_FollowParentPlus40_08b59e  ($08B59E, 24 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B59E: movea.l $c(a6), a0
pub fn i_08b59e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5A2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08B5A2: move.w $22(a0), $22(a6)
pub fn i_08b5a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5A8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B5A8: move.w $24(a0), $24(a6)
pub fn i_08b5a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5AE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B5AE: addi.w #$40, $24(a6)
pub fn i_08b5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5B4;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B5B4: rts
pub fn i_08b5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_FollowParent_08b5b6  ($08B5B6, 18 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B5B6: movea.l $c(a6), a0
pub fn i_08b5b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5BA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08B5BA: move.w $22(a0), $22(a6)
pub fn i_08b5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5C0;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B5C0: move.w $24(a0), $24(a6)
pub fn i_08b5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5C6;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08B5C6: rts
pub fn i_08b5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_RotateStep_08b5c8  ($08B5C8, 82 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B5C8: subq.w #$1, $92(a6)
pub fn i_08b5c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5CC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B5CC: bne.w $8b620
pub fn i_08b5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5D0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B620; return 10; }
    8
}
/// $08B5D0: move.w $90(a6), d0
pub fn i_08b5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5D4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B5D4: add.w d0, $34(a6)
pub fn i_08b5d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5D8;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B5D8: move.w $90(a6), d0
pub fn i_08b5d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5DC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B5DC: asr.w #$8, d0
pub fn i_08b5dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5DE;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08B5DE: btst.b #$7, d0
pub fn i_08b5de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5E2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((7) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B5E2: bne.w $8b5fa
pub fn i_08b5e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B5FA; return 10; }
    8
}
/// $08B5E6: cmpi.w #$3, $34(a6)
pub fn i_08b5e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5EC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $08B5EC: ble.w $8b60e
pub fn i_08b5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5F0;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B60E; return 10; }
    8
}
/// $08B5F0: move.w #$ffff, $90(a6)
pub fn i_08b5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B5F6: bra.w $8b60e
pub fn i_08b5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B5FA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B60E;
    10
}
/// $08B5FA: cmpi.w #$2, $34(a6)
pub fn i_08b5fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B600;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $08B600: bge.w $8b60e
pub fn i_08b600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B604;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8B60E; return 10; }
    8
}
/// $08B604: move.w #$1, $90(a6)
pub fn i_08b604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B60A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B60A: bra.w $8b60e
pub fn i_08b60a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B60E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B60E;
    10
}
/// $08B60E: andi.w #$7, $34(a6)
pub fn i_08b60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B614;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B614: move.w #$f, $92(a6)
pub fn i_08b614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B61A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_08b61a  ($08B61A, 6 B, thunks/ccr_helpers.c)
/// $08B61A: ori.b #$11, ccr
pub fn i_08b61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B61E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08B61E: rts
pub fn i_08b61e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B620;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_08b620  ($08B620, 6 B, thunks/ccr_helpers.c)
/// $08B620: andi.b #$ee, ccr
pub fn i_08b620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B624;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08B624: rts
pub fn i_08b624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B626;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Turret8_FireBullet_08b626  ($08B626, 130 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B626: move.b $106f28.l, d0
pub fn i_08b626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B62C;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B62C: andi.b #$7, d0
pub fn i_08b62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B630;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $08B630: bne.w $8b6a6
pub fn i_08b630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B634;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B6A6; return 10; }
    8
}
/// $08B634: move.b $106f28.l, d0
pub fn i_08b634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B63A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B63A: btst.b #$3, d0
pub fn i_08b63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B63E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B63E: bne.w $8b65e
pub fn i_08b63e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B642;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B65E; return 10; }
    8
}
/// $08B642: lea.l $89ad0(pc), a1
pub fn i_08b642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B646;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x89AD0;
    16
}
/// $08B646: jsr $6fe.l
pub fn i_08b646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B64C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B64C);
    cpu.pc = 0x6FE;
    16
}
/// $08B64C: jsr $5dd02.l
pub fn i_08b64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B652);
    cpu.pc = 0x5DD02;
    16
}
/// $08B652: addi.w #$8, $22(a0)
pub fn i_08b652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B658;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B658: addi.w #$20, $24(a0)
pub fn i_08b658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B65E;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B65E: move.w #$108c, d0
pub fn i_08b65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B662;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B662: jsr $2352.l
pub fn i_08b662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B668;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B668);
    cpu.pc = 0x2352;
    16
}
/// $08B668: lea.l $3093a.l, a1
pub fn i_08b668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B66E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3093A;
    20
}
/// $08B66E: jsr $4ae.l
pub fn i_08b66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B674;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B674);
    cpu.pc = 0x4AE;
    16
}
/// $08B674: jsr $5dd22.l
pub fn i_08b674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B67A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B67A);
    cpu.pc = 0x5DD22;
    16
}
/// $08B67A: move.w $34(a6), d0
pub fn i_08b67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B67E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B67E: asl.w #$4, d0
pub fn i_08b67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B680;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $08B680: andi.w #$ff, d0
pub fn i_08b680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B684;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B684: move.b d0, $98(a0)
pub fn i_08b684(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B688;
    cpu.ir = 0x1140;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08B688: clr.l d0
pub fn i_08b688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B68A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B68A: move.w $34(a6), d0
pub fn i_08b68a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B68E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B68E: asl.l #$2, d0
pub fn i_08b68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B690;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $08B690: lea.l $2ef7ee.l, a1
pub fn i_08b690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B696;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF7EE;
    20
}
/// $08B696: move.w (a1, d0.w), d1
pub fn i_08b696(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B69A;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08B69A: add.w d1, $22(a0)
pub fn i_08b69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B69E;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B69E: move.w $2(a1, d0.w), d1
pub fn i_08b69e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6A2;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08B6A2: add.w d1, $24(a0)
pub fn i_08b6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6A6;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B6A6: rts
pub fn i_08b6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Steer_08b6a8  ($08B6A8, 106 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B6A8: subi.w #$2, $34(a6)
pub fn i_08b6a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6AE;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B6AE: andi.w #$ff, $34(a6)
pub fn i_08b6ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6B4;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B6B4: move.w $34(a6), d0
pub fn i_08b6b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6B8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B6B8: move.w #$3e, d1
pub fn i_08b6b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6BC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B6BC: jsr $13c0e.l
pub fn i_08b6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B6C2);
    cpu.pc = 0x13C0E;
    16
}
/// $08B6C2: asl.w #$1, d1
pub fn i_08b6c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6C4;
    cpu.ir = 0xE341;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08B6C4: jsr $8b778(pc)
pub fn i_08b6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B6C8);
    cpu.pc = 0x8B778;
    16
}
/// $08B6C8: move.w d1, $28(a6)
pub fn i_08b6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6CC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B6CC: jsr $8b796(pc)
pub fn i_08b6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B6D0);
    cpu.pc = 0x8B796;
    16
}
/// $08B6D0: move.w d2, $2a(a6)
pub fn i_08b6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6D4;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B6D4: jsr $8b7f0(pc)
pub fn i_08b6d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B6D8);
    cpu.pc = 0x8B7F0;
    16
}
/// $08B6D8: bcc.w $8b6f6
pub fn i_08b6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6DC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B6F6; return 10; }
    8
}
/// $08B6DC: bclr.b #$6, $13(a6)
pub fn i_08b6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6E2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08B6E2: jsr $2783a.l
pub fn i_08b6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B6E8);
    cpu.pc = 0x2783A;
    16
}
/// $08B6E8: cmpi.w #$0, $28(a6)
pub fn i_08b6e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6EE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08B6EE: ble.w $8b6f6
pub fn i_08b6ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6F2;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8B6F6; return 10; }
    8
}
/// $08B6F2: clr.w $28(a6)
pub fn i_08b6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6F6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B6F6: jsr $27cee.l
pub fn i_08b6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B6FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B6FC);
    cpu.pc = 0x27CEE;
    16
}
/// $08B6FC: jsr $8b862(pc)
pub fn i_08b6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B700;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B700);
    cpu.pc = 0x8B862;
    16
}
/// $08B700: jsr $8b7b0(pc)
pub fn i_08b700(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B704;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B704);
    cpu.pc = 0x8B7B0;
    16
}
/// $08B704: bcc.w $8b716
pub fn i_08b704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B708;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B716; return 10; }
    8
}
/// $08B708: move.w #$1081, d0
pub fn i_08b708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B70C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1081;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B70C: jsr $2352.l
pub fn i_08b70c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B712;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B712);
    cpu.pc = 0x2352;
    16
}
// ===== JsrPcThunk_08b712  ($08B712, 6 B, thunks/jsr_pc_thunks.c)
/// $08B712: jsr $8b82c(pc)
pub fn i_08b712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B716;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B716);
    cpu.pc = 0x8B82C;
    16
}
/// $08B716: rts
pub fn i_08b716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B718;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_SpiralDescent_08b718  ($08B718, 88 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B718: move.w $36(a6), d1
pub fn i_08b718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B71C;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B71C: btst.b #$f, d1
pub fn i_08b71c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B720;
    cpu.ir = 0x801;
    let mk: u32 = 1u32 << ((15) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B720: beq.w $8b72c
pub fn i_08b720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B724;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B72C; return 10; }
    8
}
/// $08B724: clr.w d1
pub fn i_08b724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B726;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B726: clr.w d2
pub fn i_08b726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B728;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B728: bra.w $8b768
pub fn i_08b728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B72C;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B768;
    10
}
/// $08B72C: move.w $36(a6), d1
pub fn i_08b72c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B730;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B730: asr.w #$5, d1
pub fn i_08b730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B732;
    cpu.ir = 0xEA41;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $08B732: move.b $106f28.l, d0
pub fn i_08b732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B738;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B738: btst.b #$0, d0
pub fn i_08b738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B73C;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B73C: beq.w $8b742
pub fn i_08b73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B740;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B742; return 10; }
    8
}
/// $08B740: neg.w d1
pub fn i_08b740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B742;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B742: asl.w #$8, d1
pub fn i_08b742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B744;
    cpu.ir = 0xE141;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $08B744: move.w d1, $2a(a6)
pub fn i_08b744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B748;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B748: move.w #$0, $28(a6)
pub fn i_08b748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B74E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B74E: jsr $27cee.l
pub fn i_08b74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B754);
    cpu.pc = 0x27CEE;
    16
}
/// $08B754: subi.w #$3, $36(a6)
pub fn i_08b754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B75A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B75A: move.w $36(a6), d1
pub fn i_08b75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B75E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B75E: move.w #$1f, d0
pub fn i_08b75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B762;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B762: jsr $13c0e.l
pub fn i_08b762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B768;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B768);
    cpu.pc = 0x13C0E;
    16
}
/// $08B768: move.w d1, $28(a6)
pub fn i_08b768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B76C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B76C: move.w d2, $2a(a6)
pub fn i_08b76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B770;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_08b770  ($08B770, 8 B, thunks/jsr_abs_thunks.c)
/// $08B770: jsr $27cee.l
pub fn i_08b770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B776;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B776);
    cpu.pc = 0x27CEE;
    16
}
/// $08B776: rts
pub fn i_08b776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B778;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_SteerX_08b778  ($08B778, 30 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B778: move.w $7e(a6), d0
pub fn i_08b778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B77C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B77C: sub.w $22(a6), d0
pub fn i_08b77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B780;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08B780: beq.w $8b794
pub fn i_08b780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B784;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B794; return 10; }
    8
}
/// $08B784: bpl.w $8b790
pub fn i_08b784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B788;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8B790; return 10; }
    8
}
/// $08B788: subi.w #$12, d1
pub fn i_08b788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B78C;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x12, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B78C: bra.w $8b794
pub fn i_08b78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B790;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B794;
    10
}
/// $08B790: addi.w #$12, d1
pub fn i_08b790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B794;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x12, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B794: rts
pub fn i_08b794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B796;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_SteerY_08b796  ($08B796, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B796: move.w $80(a6), d0
pub fn i_08b796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B79A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B79A: sub.w $24(a6), d0
pub fn i_08b79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B79E;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08B79E: beq.w $8b7ae
pub fn i_08b79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B7AE; return 10; }
    8
}
/// $08B7A2: bpl.w $8b7ac
pub fn i_08b7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7A6;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x8B7AC; return 10; }
    8
}
/// $08B7A6: subq.w #$4, d2
pub fn i_08b7a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7A8;
    cpu.ir = 0x5942;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B7A8: bra.w $8b7ae
pub fn i_08b7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7AC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B7AE;
    10
}
/// $08B7AC: addq.w #$4, d2
pub fn i_08b7ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7AE;
    cpu.ir = 0x5842;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B7AE: rts
pub fn i_08b7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_ProbeGround_08b7b0  ($08B7B0, 48 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B7B0: move.w $22(a6), d1
pub fn i_08b7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7B4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B7B4: move.w $24(a6), d2
pub fn i_08b7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7B8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B7B8: addi.w #$a8, d1
pub fn i_08b7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7BC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B7BC: addi.w #$10, d2
pub fn i_08b7bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7C0;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B7C0: jsr $280c6.l
pub fn i_08b7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B7C6);
    cpu.pc = 0x280C6;
    16
}
/// $08B7C6: bcc.w $8b7e6
pub fn i_08b7c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B7E6; return 10; }
    8
}
/// $08B7CA: cmpi.b #$25, d7
pub fn i_08b7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7CE;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x25, Size::Byte, false);
    8
}
/// $08B7CE: bne.w $8b7e6
pub fn i_08b7ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7D2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B7E6; return 10; }
    8
}
/// $08B7D2: cmpi.b #$0, $83(a6)
pub fn i_08b7d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7D8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $08B7D8: bne.w $8b7e6
pub fn i_08b7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B7E6; return 10; }
    8
}
/// $08B7DC: move.b d0, $83(a6)
pub fn i_08b7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7E0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_08b7e0  ($08B7E0, 6 B, thunks/ccr_helpers.c)
/// $08B7E0: ori.b #$11, ccr
pub fn i_08b7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7E4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08B7E4: rts
pub fn i_08b7e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_ProbeGround_Tail_08b7e6  ($08B7E6, 4 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B7E6: move.b d0, $83(a6)
pub fn i_08b7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7EA;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x83);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_08b7ea  ($08B7EA, 6 B, thunks/ccr_helpers.c)
/// $08B7EA: andi.b #$ee, ccr
pub fn i_08b7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08B7EE: rts
pub fn i_08b7ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_CheckLanding_08b7f0  ($08B7F0, 48 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B7F0: move.w $22(a6), d1
pub fn i_08b7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7F4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B7F4: move.w $24(a6), d2
pub fn i_08b7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7F8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B7F8: addi.w #$a8, d1
pub fn i_08b7f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B7FC;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA8, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B7FC: addi.w #$10, d2
pub fn i_08b7fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B800;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B800: jsr $280c6.l
pub fn i_08b800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B806;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B806);
    cpu.pc = 0x280C6;
    16
}
/// $08B806: bcc.w $8b826
pub fn i_08b806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B80A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B826; return 10; }
    8
}
/// $08B80A: cmpi.b #$25, d7
pub fn i_08b80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B80E;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x25, Size::Byte, false);
    8
}
/// $08B80E: beq.w $8b820
pub fn i_08b80e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B812;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B820; return 10; }
    8
}
/// $08B812: cmpi.b #$1e, d7
pub fn i_08b812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B816;
    cpu.ir = 0xC07;
    let v1 = cpu.d[7] & 0xFF;
    sub_with_flags(cpu, v1, 0x1E, Size::Byte, false);
    8
}
/// $08B816: bne.w $8b826
pub fn i_08b816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B81A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B826; return 10; }
    8
}
/// $08B81A: lea.l $895fe(pc), a1
pub fn i_08b81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B81E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x895FE;
    16
}
/// $08B81E: move.l a1, (a6)
pub fn i_08b81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B820;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== SetXN_08b820  ($08B820, 6 B, thunks/ccr_helpers.c)
/// $08B820: ori.b #$11, ccr
pub fn i_08b820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B824;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08B824: rts
pub fn i_08b824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B826;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_08b826  ($08B826, 6 B, thunks/ccr_helpers.c)
/// $08B826: andi.b #$ee, ccr
pub fn i_08b826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B82A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08B82A: rts
pub fn i_08b82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B82C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_DropSoldier_08b82c  ($08B82C, 54 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B82C: move.w #$a, $84(a6)
pub fn i_08b82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B832;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B832: move.l $106f50.l, d0
pub fn i_08b832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B838;
    cpu.ir = 0x2039;
    let v2 = bus.read32(0x106F50);
    cpu.d[0] = v2;
    cpu.sr.set_nz(v2, Size::Long); cpu.sr.0 &= !0x3;
    20
}
/// $08B838: swap d0
pub fn i_08b838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B83A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $08B83A: cmpi.w #$9a0, d0
pub fn i_08b83a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B83E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x9A0, Size::Word, false);
    8
}
/// $08B83E: bge.w $8b860
pub fn i_08b83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B842;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x8B860; return 10; }
    8
}
/// $08B842: lea.l $631d0.l, a1
pub fn i_08b842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B848;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x631D0;
    20
}
/// $08B848: jsr $4ae.l
pub fn i_08b848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B84E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B84E);
    cpu.pc = 0x4AE;
    16
}
/// $08B84E: jsr $5dd22.l
pub fn i_08b84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B854);
    cpu.pc = 0x5DD22;
    16
}
/// $08B854: addi.w #$b8, $22(a0)
pub fn i_08b854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B85A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xB8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B85A: addi.w #$50, $24(a0)
pub fn i_08b85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B860;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x50, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08B860: rts
pub fn i_08b860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B862;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_Recoil_08b862  ($08B862, 78 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B862: cmpi.w #$0, $84(a6)
pub fn i_08b862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B868;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08B868: beq.w $8b8ae
pub fn i_08b868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B86C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8B8AE; return 10; }
    8
}
/// $08B86C: subq.w #$1, $84(a6)
pub fn i_08b86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B870;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08B870: move.w #$200, d1
pub fn i_08b870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B874;
    cpu.ir = 0x323C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B874: move.b $106f28.l, d0
pub fn i_08b874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B87A;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $08B87A: btst.b #$0, d0
pub fn i_08b87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B87E;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $08B87E: bne.w $8b884
pub fn i_08b87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B882;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8B884; return 10; }
    8
}
/// $08B882: neg.w d1
pub fn i_08b882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B884;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08B884: move.w $28(a6), d2
pub fn i_08b884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B888;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B888: move.w $2a(a6), d3
pub fn i_08b888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B88C;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B88C: movem.w d2, -(a7)
pub fn i_08b88c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B890;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    cpu.a[7] = a;
    20
}
/// $08B890: movem.w d3, -(a7)
pub fn i_08b890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B894;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[3] as u16);
    cpu.a[7] = a;
    20
}
/// $08B894: move.w d1, $2a(a6)
pub fn i_08b894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B898;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B898: jsr $27cee.l
pub fn i_08b898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B89E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B89E);
    cpu.pc = 0x27CEE;
    16
}
/// $08B89E: movem.w (a7)+, d3
pub fn i_08b89e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8A2;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[3] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08B8A2: movem.w (a7)+, d2
pub fn i_08b8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8A6;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $08B8A6: move.w d2, $28(a6)
pub fn i_08b8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8AA;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B8AA: move.w d3, $2a(a6)
pub fn i_08b8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8AE;
    cpu.ir = 0x3D43;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B8AE: rts
pub fn i_08b8ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Airship_TrailRecord_08b8b0  ($08B8B0, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B8B0: move.w $22(a6), d0
pub fn i_08b8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8B4: move.w $24(a6), d1
pub fn i_08b8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8B8;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8B8: subi.w #$a0, d0
pub fn i_08b8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8BC;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xA0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8BC: addi.w #$2e, d1
pub fn i_08b8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8C0;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2E, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8C0: move.w #$140, d2
pub fn i_08b8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8C4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B8C4: jmp $99812.l
pub fn i_08b8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8CA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x99812;
    8
}
// ===== Airship_TrailRecordAlt_08b8ca  ($08B8CA, 26 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B8CA: move.w $22(a6), d0
pub fn i_08b8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8CE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8CE: move.w $24(a6), d1
pub fn i_08b8ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8D2;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8D2: subi.w #$a0, d0
pub fn i_08b8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8D6;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xA0, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8D6: addi.w #$2e, d1
pub fn i_08b8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8DA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2E, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8DA: move.w #$140, d2
pub fn i_08b8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8DE;
    cpu.ir = 0x343C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08B8DE: jmp $997e2.l
pub fn i_08b8de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8E4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x997E2;
    8
}
// ===== Camera_PublishLockX_08b8e4  ($08B8E4, 16 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B8E4: move.w $22(a6), d0
pub fn i_08b8e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8E8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08B8E8: addi.w #$70, d0
pub fn i_08b8e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8EC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x70, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08B8EC: move.w d0, $106f42.l
pub fn i_08b8ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8F2;
    cpu.ir = 0x33C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x106F42, v1 as u16);
    16
}
/// $08B8F2: rts
pub fn i_08b8f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Tent_BlitRowByIdx_08b8f4  ($08B8F4, 18 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B8F4: clr.l d0
pub fn i_08b8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8F6;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B8F6: move.b $21(a6), d0
pub fn i_08b8f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8FA;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B8FA: asl.l #$2, d0
pub fn i_08b8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B8FC;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $08B8FC: lea.l $2ef61a.l, a0
pub fn i_08b8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B902;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EF61A;
    20
}
/// $08B902: movea.l (a0, d0.w), a1
pub fn i_08b902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B906;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
// ===== JsrAbsThunk_08b906  ($08B906, 8 B, thunks/jsr_abs_thunks.c)
/// $08B906: jsr $43fac.l
pub fn i_08b906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B90C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B90C);
    cpu.pc = 0x43FAC;
    16
}
/// $08B90C: rts
pub fn i_08b90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B90E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== S5_Bunker_BlitByIdx_08b90e  ($08B90E, 18 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B90E: clr.l d0
pub fn i_08b90e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B910;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $08B910: move.b $21(a6), d0
pub fn i_08b910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B914;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B914: asl.l #$2, d0
pub fn i_08b914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B916;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $08B916: lea.l $2ef39c.l, a2
pub fn i_08b916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B91C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2EF39C;
    20
}
/// $08B91C: movea.l (a2, d0.w), a2
pub fn i_08b91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B920;
    cpu.ir = 0x2472;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    18
}
// ===== JsrAbsThunk_08b920  ($08B920, 8 B, thunks/jsr_abs_thunks.c)
/// $08B920: jsr $5022a.l
pub fn i_08b920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B926;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B926);
    cpu.pc = 0x5022A;
    16
}
/// $08B926: rts
pub fn i_08b926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B928;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08b928  ($08B928, 16 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B928: movea.l $8(a6), a1
pub fn i_08b928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B92C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08B92C: move.b $10(a6), d0
pub fn i_08b92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B930;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08B930: cmp.b $10(a1), d0
pub fn i_08b930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B934;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08B934: bcs.w $8b93e
pub fn i_08b934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B938;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8B93E; return 10; }
    8
}
// ===== ClearXN_08b938  ($08B938, 6 B, thunks/ccr_helpers.c)
/// $08B938: andi.b #$ee, ccr
pub fn i_08b938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B93C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08B93C: rts
pub fn i_08b93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B93E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08b93e  ($08B93E, 6 B, thunks/ccr_helpers.c)
/// $08B93E: ori.b #$11, ccr
pub fn i_08b93e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B942;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08B942: rts
pub fn i_08b942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B944;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Tmpl168_08b9a2  ($08B9A2, 8 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B9A2: jsr $8bb5e(pc)
pub fn i_08b9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9A6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B9A6);
    cpu.pc = 0x8BB5E;
    16
}
/// $08B9A6: bra.w $8ba0c
pub fn i_08b9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BA0C;
    10
}
// ===== Proj_Tmpl169_08b9aa  ($08B9AA, 8 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B9AA: jsr $8bb34(pc)
pub fn i_08b9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B9AE);
    cpu.pc = 0x8BB34;
    16
}
/// $08B9AE: bra.w $8b9ba
pub fn i_08b9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9B2;
    cpu.ir = 0x6000;
    cpu.pc = 0x8B9BA;
    10
}
// ===== Proj_Tmpl170_08b9b2  ($08B9B2, 8 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B9B2: jsr $8bb34(pc)
pub fn i_08b9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8B9B6);
    cpu.pc = 0x8BB34;
    16
}
/// $08B9B6: bra.w $8ba52
pub fn i_08b9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BA52;
    10
}
// ===== Proj_Bounce_08b9ba  ($08B9BA, 74 B, enemies/bosses/scene5_airship_088a64.s)
/// $08B9BA: move.w #$0, $28(a6)
pub fn i_08b9ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08B9C0: lea.l $29b744.l, a0
pub fn i_08b9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B744;
    20
}
/// $08B9C6: jsr $28cd4.l
pub fn i_08b9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9CC);
    cpu.pc = 0x28CD4;
    16
}
/// $08B9CC: lea.l $29ca20.l, a0
pub fn i_08b9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9D2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CA20;
    20
}
/// $08B9D2: jsr $28cd4.l
pub fn i_08b9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9D8);
    cpu.pc = 0x28CD4;
    16
}
/// $08B9D8: lea.l $8b9de(pc), a1
pub fn i_08b9d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B9DE;
    16
}
/// $08B9DC: move.l a1, (a6)
pub fn i_08b9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B9DE: jsr $2783a.l
pub fn i_08b9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9E4);
    cpu.pc = 0x2783A;
    16
}
/// $08B9E4: jsr $28d70.l
pub fn i_08b9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9EA);
    cpu.pc = 0x28D70;
    16
}
/// $08B9EA: bcc.w $8b9f4
pub fn i_08b9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8B9F4; return 10; }
    8
}
/// $08B9EE: lea.l $8b9ba(pc), a1
pub fn i_08b9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8B9BA;
    16
}
/// $08B9F2: move.l a1, (a6)
pub fn i_08b9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08B9F4: jsr $2870a.l
pub fn i_08b9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8B9FA);
    cpu.pc = 0x2870A;
    16
}
/// $08B9FA: lea.l $8b944(pc), a0
pub fn i_08b9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8B9FE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B944;
    16
}
/// $08B9FE: movea.l #$ffffffff, a1
pub fn i_08b9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA04;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08ba04  ($08BA04, 8 B, thunks/jsr_abs_thunks.c)
/// $08BA04: jsr $772.l
pub fn i_08ba04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA0A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA0A);
    cpu.pc = 0x772;
    16
}
/// $08BA0A: rts
pub fn i_08ba0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA0C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Bounce_V1_08ba0c  ($08BA0C, 62 B, mission/cutscene_anim_08ba0c.s)
/// $08BA0C: move.w #$0, $28(a6)
pub fn i_08ba0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA12;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BA12: lea.l $29ca0a.l, a0
pub fn i_08ba12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA18;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29CA0A;
    20
}
/// $08BA18: jsr $28cd4.l
pub fn i_08ba18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA1E);
    cpu.pc = 0x28CD4;
    16
}
/// $08BA1E: lea.l $8ba24(pc), a1
pub fn i_08ba1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BA24;
    16
}
/// $08BA22: move.l a1, (a6)
pub fn i_08ba22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BA24: jsr $2783a.l
pub fn i_08ba24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA2A);
    cpu.pc = 0x2783A;
    16
}
/// $08BA2A: jsr $28d70.l
pub fn i_08ba2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA30);
    cpu.pc = 0x28D70;
    16
}
/// $08BA30: bcc.w $8ba3a
pub fn i_08ba30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8BA3A; return 10; }
    8
}
/// $08BA34: lea.l $8ba0c(pc), a1
pub fn i_08ba34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BA0C;
    16
}
/// $08BA38: move.l a1, (a6)
pub fn i_08ba38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BA3A: jsr $2870a.l
pub fn i_08ba3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA40;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA40);
    cpu.pc = 0x2870A;
    16
}
/// $08BA40: lea.l $8b944(pc), a0
pub fn i_08ba40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA44;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B944;
    16
}
/// $08BA44: movea.l #$ffffffff, a1
pub fn i_08ba44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA4A;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08ba4a  ($08BA4A, 8 B, thunks/jsr_abs_thunks.c)
/// $08BA4A: jsr $772.l
pub fn i_08ba4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA50);
    cpu.pc = 0x772;
    16
}
/// $08BA50: rts
pub fn i_08ba50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Bounce_V2_08ba52  ($08BA52, 68 B, mission/cutscene_anim_08ba0c.s)
/// $08BA52: bset.b #$0, $3a(a6)
pub fn i_08ba52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA58;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08BA58: move.w #$0, $28(a6)
pub fn i_08ba58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA5E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BA5E: lea.l $29bfc4.l, a0
pub fn i_08ba5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA64;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BFC4;
    20
}
/// $08BA64: jsr $28cd4.l
pub fn i_08ba64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA6A);
    cpu.pc = 0x28CD4;
    16
}
/// $08BA6A: lea.l $8ba70(pc), a1
pub fn i_08ba6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BA70;
    16
}
/// $08BA6E: move.l a1, (a6)
pub fn i_08ba6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BA70: jsr $2783a.l
pub fn i_08ba70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA76);
    cpu.pc = 0x2783A;
    16
}
/// $08BA76: jsr $28d70.l
pub fn i_08ba76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA7C);
    cpu.pc = 0x28D70;
    16
}
/// $08BA7C: bcc.w $8ba86
pub fn i_08ba7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA80;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8BA86; return 10; }
    8
}
/// $08BA80: lea.l $8ba0c(pc), a1
pub fn i_08ba80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BA0C;
    16
}
/// $08BA84: move.l a1, (a6)
pub fn i_08ba84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BA86: jsr $2870a.l
pub fn i_08ba86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA8C);
    cpu.pc = 0x2870A;
    16
}
/// $08BA8C: lea.l $8b944(pc), a0
pub fn i_08ba8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA90;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B944;
    16
}
/// $08BA90: movea.l #$ffffffff, a1
pub fn i_08ba90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA96;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_08ba96  ($08BA96, 8 B, thunks/jsr_abs_thunks.c)
/// $08BA96: jsr $772.l
pub fn i_08ba96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BA9C);
    cpu.pc = 0x772;
    16
}
/// $08BA9C: rts
pub fn i_08ba9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BA9E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Burst_RandDispatch_08babe  ($08BABE, 28 B, mission/cutscene_anim_08ba0c.s)
/// $08BABE: jsr $5e9b6.l
pub fn i_08babe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BAC4);
    cpu.pc = 0x5E9B6;
    16
}
/// $08BAC4: andi.w #$3, d0
pub fn i_08bac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAC8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BAC8: lea.l $8ba9e(pc), a0
pub fn i_08bac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BACC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8BA9E;
    16
}
/// $08BACC: movea.l #$ffffffff, a1
pub fn i_08bacc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAD2;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08BAD2: jsr $772.l
pub fn i_08bad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BAD8);
    cpu.pc = 0x772;
    16
}
/// $08BAD8: jmp (a1)
pub fn i_08bad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BADA;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
// ===== Proj_Burst_RandDispatch2_08bada  ($08BADA, 28 B, mission/cutscene_anim_08ba0c.s)
/// $08BADA: jsr $5e9b6.l
pub fn i_08bada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BAE0);
    cpu.pc = 0x5E9B6;
    16
}
/// $08BAE0: andi.w #$3, d0
pub fn i_08bae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAE4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BAE4: lea.l $8baae(pc), a0
pub fn i_08bae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAE8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8BAAE;
    16
}
/// $08BAE8: movea.l #$ffffffff, a1
pub fn i_08bae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAEE;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08BAEE: jsr $772.l
pub fn i_08baee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BAF4);
    cpu.pc = 0x772;
    16
}
/// $08BAF4: jmp (a1)
pub fn i_08baf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAF6;
    cpu.ir = 0x4ED1;
    cpu.jump_to(cpu.a[1]);
    8
}
// ===== Proj_Burst_08baf6  ($08BAF6, 44 B, mission/cutscene_anim_08ba0c.s)
/// $08BAF6: move.w #$2f, d1
pub fn i_08baf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BAFA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BAFA: jsr $236e.l
pub fn i_08bafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB00);
    cpu.pc = 0x236E;
    16
}
/// $08BB00: lea.l $4b136.l, a0
pub fn i_08bb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB06;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x4B136;
    20
}
/// $08BB06: jsr $28cd4.l
pub fn i_08bb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB0C);
    cpu.pc = 0x28CD4;
    16
}
/// $08BB0C: lea.l $8bb12(pc), a1
pub fn i_08bb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BB12;
    16
}
/// $08BB10: move.l a1, (a6)
pub fn i_08bb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BB12: jsr $2783a.l
pub fn i_08bb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB18);
    cpu.pc = 0x2783A;
    16
}
/// $08BB18: jsr $28d70.l
pub fn i_08bb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB1E);
    cpu.pc = 0x28D70;
    16
}
/// $08BB1E: bcc.w $8bb28
pub fn i_08bb1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB22;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8BB28; return 10; }
    8
}
// ===== SetTaskHandler_08bb22  ($08BB22, 8 B, kernel/task_handlers.c)
/// $08BB22: lea.l $8bb84(pc), a1
pub fn i_08bb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB26;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BB84;
    16
}
/// $08BB26: move.l a1, (a6)
pub fn i_08bb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB28;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BB28: rts
pub fn i_08bb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB2A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeTransformLoop_08bb2a  ($08BB2A, 10 B, mission/cutscene_anim_08ba0c.s)
/// $08BB2A: jsr $27c8c.l
pub fn i_08bb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB30);
    cpu.pc = 0x27C8C;
    16
}
/// $08BB30: bcc.b $8bb2a
pub fn i_08bb30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB32;
    cpu.ir = 0x64F8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8BB2A; return 10; }
    8
}
/// $08BB32: rts
pub fn i_08bb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB34;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Tmpl_InitHitboxProbe_08bb34  ($08BB34, 34 B, mission/cutscene_anim_08ba0c.s)
/// $08BB34: move.w #$e, d1
pub fn i_08bb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB38;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BB38: jsr $236e.l
pub fn i_08bb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB3E);
    cpu.pc = 0x236E;
    16
}
/// $08BB3E: lea.l $8b950(pc), a0
pub fn i_08bb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB42;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B950;
    16
}
/// $08BB42: move.l a0, $48(a6)
pub fn i_08bb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB46;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BB46: jsr $8bb2a(pc)
pub fn i_08bb46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB4A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8BB4A);
    cpu.pc = 0x8BB2A;
    16
}
/// $08BB4A: lea.l $776e2.l, a1
pub fn i_08bb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB50;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $08BB50: jsr $4ae.l
pub fn i_08bb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB56);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_08bb56  ($08BB56, 8 B, thunks/jsr_abs_thunks.c)
/// $08BB56: jsr $5dd02.l
pub fn i_08bb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB5C);
    cpu.pc = 0x5DD02;
    16
}
/// $08BB5C: rts
pub fn i_08bb5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB5E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Tmpl_InitHitbox_08bb5e  ($08BB5E, 30 B, mission/cutscene_anim_08ba0c.s)
/// $08BB5E: move.w #$e, d1
pub fn i_08bb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB62;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BB62: jsr $236e.l
pub fn i_08bb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB68);
    cpu.pc = 0x236E;
    16
}
/// $08BB68: lea.l $8b950(pc), a0
pub fn i_08bb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB6C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8B950;
    16
}
/// $08BB6C: move.l a0, $48(a6)
pub fn i_08bb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB70;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BB70: lea.l $776e2.l, a1
pub fn i_08bb70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB76;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x776E2;
    20
}
/// $08BB76: jsr $4ae.l
pub fn i_08bb76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB7C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB7C);
    cpu.pc = 0x4AE;
    16
}
// ===== JsrAbsThunk_08bb7c  ($08BB7C, 8 B, thunks/jsr_abs_thunks.c)
/// $08BB7C: jsr $5dd02.l
pub fn i_08bb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB82;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BB82);
    cpu.pc = 0x5DD02;
    16
}
/// $08BB82: rts
pub fn i_08bb82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_08bb84  ($08BB84, 8 B, kernel/jmp_to_scheduler.c)
/// $08BB84: jmp $518.l
pub fn i_08bb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB8A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08BB8A: rts
pub fn i_08bb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BB8C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Proj_Shell_08bc0c  ($08BC0C, 76 B, mission/cutscene_anim_08ba0c.s)
/// $08BC0C: move.w #$1b, d1
pub fn i_08bc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC10;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BC10: jsr $236e.l
pub fn i_08bc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BC16);
    cpu.pc = 0x236E;
    16
}
/// $08BC16: lea.l $7773e.l, a1
pub fn i_08bc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC1C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7773E;
    20
}
/// $08BC1C: jsr $4ae.l
pub fn i_08bc1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC22;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BC22);
    cpu.pc = 0x4AE;
    16
}
/// $08BC22: jsr $5dd02.l
pub fn i_08bc22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BC28);
    cpu.pc = 0x5DD02;
    16
}
/// $08BC28: jsr $8bb2a(pc)
pub fn i_08bc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8BC2C);
    cpu.pc = 0x8BB2A;
    16
}
/// $08BC2C: lea.l $8bb8c(pc), a0
pub fn i_08bc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC30;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8BB8C;
    16
}
/// $08BC30: move.l a0, $48(a6)
pub fn i_08bc30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC34;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BC34: lea.l $8bbde(pc), a0
pub fn i_08bc34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC38;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x8BBDE;
    16
}
/// $08BC38: jsr $28cd4.l
pub fn i_08bc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BC3E);
    cpu.pc = 0x28CD4;
    16
}
/// $08BC3E: lea.l $8bc44(pc), a1
pub fn i_08bc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8BC44;
    16
}
/// $08BC42: move.l a1, (a6)
pub fn i_08bc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08BC44: jsr $2783a.l
pub fn i_08bc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BC4A);
    cpu.pc = 0x2783A;
    16
}
/// $08BC4A: jsr $28d70.l
pub fn i_08bc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BC50);
    cpu.pc = 0x28D70;
    16
}
/// $08BC50: bclr.b #$3, $13(a6)
pub fn i_08bc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC56;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08BC56: rts
pub fn i_08bc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC58;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_CmpPrioWithSibling_08bc58  ($08BC58, 16 B, mission/cutscene_anim_08ba0c.s)
/// $08BC58: movea.l $8(a6), a1
pub fn i_08bc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC5C;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BC5C: move.b $10(a6), d0
pub fn i_08bc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC60;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08BC60: cmp.b $10(a1), d0
pub fn i_08bc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC64;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $08BC64: bcs.w $8bc6e
pub fn i_08bc64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC68;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8BC6E; return 10; }
    8
}
// ===== ClearXN_08bc68  ($08BC68, 6 B, thunks/ccr_helpers.c)
/// $08BC68: andi.b #$ee, ccr
pub fn i_08bc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC6C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08BC6C: rts
pub fn i_08bc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_08bc6e  ($08BC6E, 6 B, thunks/ccr_helpers.c)
/// $08BC6E: ori.b #$11, ccr
pub fn i_08bc6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC72;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08BC72: rts
pub fn i_08bc72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_ScriptStep_08bc74  ($08BC74, 108 B, mission/cutscene_anim_08ba0c.s)
/// $08BC74: cmpi.w #$ffff, $70(a6)
pub fn i_08bc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC7A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BC7A: beq.w $8bce0
pub fn i_08bc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BCE0; return 10; }
    8
}
/// $08BC7E: subq.w #$1, $70(a6)
pub fn i_08bc7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC82;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08BC82: cmpi.w #$0, $70(a6)
pub fn i_08bc82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BC88: bgt.w $8bcd2
pub fn i_08bc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC8C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8BCD2; return 10; }
    8
}
/// $08BC8C: movea.l $7c(a6), a1
pub fn i_08bc8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC90;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BC90: adda.l $78(a6), a1
pub fn i_08bc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC94;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $08BC94: move.w (a1), $70(a6)
pub fn i_08bc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC98;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $08BC98: cmpi.w #$ffff, $70(a6)
pub fn i_08bc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BC9E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BC9E: beq.w $8bcbc
pub fn i_08bc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCA2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BCBC; return 10; }
    8
}
/// $08BCA2: move.w $2(a1), $76(a6)
pub fn i_08bca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCA8;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08BCA8: move.b $4(a1), $92(a6)
pub fn i_08bca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCAE;
    cpu.ir = 0x1D69;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08BCAE: move.b $5(a1), $93(a6)
pub fn i_08bcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCB4;
    cpu.ir = 0x1D69;
    let ad2 = cpu.a[1].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08BCB4: addq.l #$8, $78(a6)
pub fn i_08bcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCB8;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08BCB8: bra.w $8bcce
pub fn i_08bcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BCCE;
    10
}
/// $08BCBC: move.w #$0, $76(a6)
pub fn i_08bcbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCC2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BCC2: move.b #$0, $92(a6)
pub fn i_08bcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCC8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BCC8: move.b #$0, $93(a6)
pub fn i_08bcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCCE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BCCE: bra.w $8bcd8
pub fn i_08bcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCD2;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BCD8;
    10
}
/// $08BCD2: move.b #$0, $93(a6)
pub fn i_08bcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCD8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BCD8: andi.b #$ee, ccr
pub fn i_08bcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCDC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08BCDC: bra.w $8bce4
pub fn i_08bcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BCE4;
    10
}
// ===== SetXN_08bce0  ($08BCE0, 6 B, thunks/ccr_helpers.c)
/// $08BCE0: ori.b #$11, ccr
pub fn i_08bce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCE4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08BCE4: rts
pub fn i_08bce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_ScriptStepFix_A_08bce6  ($08BCE6, 316 B, mission/cutscene_anim_08ba0c.s)
/// $08BCE6: cmpi.w #$ffff, $70(a6)
pub fn i_08bce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCEC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BCEC: beq.w $8be22
pub fn i_08bcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCF0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BE22; return 10; }
    8
}
/// $08BCF0: subq.w #$1, $70(a6)
pub fn i_08bcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCF4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08BCF4: cmpi.w #$0, $70(a6)
pub fn i_08bcf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCFA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BCFA: bgt.w $8be1a
pub fn i_08bcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BCFE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8BE1A; return 10; }
    8
}
/// $08BCFE: movea.l $7c(a6), a1
pub fn i_08bcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD02;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BD02: adda.l $82(a6), a1
pub fn i_08bd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD06;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $08BD06: move.w (a1), $70(a6)
pub fn i_08bd06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD0A;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $08BD0A: cmpi.w #$ffff, $70(a6)
pub fn i_08bd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD10;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BD10: beq.w $8be1a
pub fn i_08bd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD14;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BE1A; return 10; }
    8
}
/// $08BD14: move.w $2(a1), $80(a6)
pub fn i_08bd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD1A;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08BD1A: cmpi.l #$0, $4(a1)
pub fn i_08bd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD22;
    cpu.ir = 0xCA9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x0, Size::Long, false);
    20
}
/// $08BD22: beq.w $8bd30
pub fn i_08bd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD26;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BD30; return 10; }
    8
}
/// $08BD26: movea.l $4(a1), a0
pub fn i_08bd26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD2A;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08BD2A: jsr $28cd4.l
pub fn i_08bd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BD30);
    cpu.pc = 0x28CD4;
    16
}
/// $08BD30: cmpi.w #$1, $80(a6)
pub fn i_08bd30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD36;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $08BD36: bne.w $8bd54
pub fn i_08bd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BD54; return 10; }
    8
}
/// $08BD3A: movea.w #$7084, a1
pub fn i_08bd3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD3E;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7084;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08BD3E: move.w #$2320, d0
pub fn i_08bd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD42;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BD42: move.w #$20, d1
pub fn i_08bd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD46;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BD46: move.w #$8, d2
pub fn i_08bd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD4A;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BD4A: jsr $5da9c.l
pub fn i_08bd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BD50);
    cpu.pc = 0x5DA9C;
    16
}
/// $08BD50: bra.w $8be12
pub fn i_08bd50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD54;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BD54: cmpi.w #$d, $80(a6)
pub fn i_08bd54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD5A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD, Size::Word, false);
    16
}
/// $08BD5A: bne.w $8bd6a
pub fn i_08bd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD5E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BD6A; return 10; }
    8
}
/// $08BD5E: move.l #$70a9, $86(a6)
pub fn i_08bd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD66;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x70A9;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BD66: bra.w $8be12
pub fn i_08bd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD6A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BD6A: cmpi.w #$2, $80(a6)
pub fn i_08bd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD70;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $08BD70: bne.w $8bd82
pub fn i_08bd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BD82; return 10; }
    8
}
/// $08BD74: lea.l $8c730(pc), a1
pub fn i_08bd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C730;
    16
}
/// $08BD78: jsr $4ae.l
pub fn i_08bd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BD7E);
    cpu.pc = 0x4AE;
    16
}
/// $08BD7E: bra.w $8be12
pub fn i_08bd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD82;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BD82: cmpi.w #$3, $80(a6)
pub fn i_08bd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $08BD88: bne.w $8bdae
pub fn i_08bd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDAE; return 10; }
    8
}
/// $08BD8C: lea.l $2ae50.l, a1
pub fn i_08bd8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD92;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2AE50;
    20
}
/// $08BD92: jsr $4ae.l
pub fn i_08bd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BD98);
    cpu.pc = 0x4AE;
    16
}
/// $08BD98: move.w #$a0, $22(a0)
pub fn i_08bd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BD9E;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BD9E: move.w #$1ff, $24(a0)
pub fn i_08bd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDA4;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BDA4: move.b #$3, $98(a0)
pub fn i_08bda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDAA;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BDAA: bra.w $8be12
pub fn i_08bdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDAE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BDAE: cmpi.w #$0, $80(a6)
pub fn i_08bdae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDB4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BDB4: bne.w $8bdc0
pub fn i_08bdb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDC0; return 10; }
    8
}
/// $08BDB8: bra.w $8be12
pub fn i_08bdb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDBC;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BDBC: bra.w $8be12
pub fn i_08bdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BDC0: movea.l $86(a6), a1
pub fn i_08bdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDC4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BDC4: addi.l #$40, $86(a6)
pub fn i_08bdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDCC;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x40, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $08BDCC: move.w $80(a6), d0
pub fn i_08bdcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDD0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08BDD0: andi.w #$ff00, d0
pub fn i_08bdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDD4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF00) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BDD4: move.w #$4, d1
pub fn i_08bdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDD8;
    cpu.ir = 0x323C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BDD8: cmpi.w #$500, d0
pub fn i_08bdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDDC;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x500, Size::Word, false);
    8
}
/// $08BDDC: bne.w $8bde4
pub fn i_08bddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDE4; return 10; }
    8
}
/// $08BDE0: move.w #$9, d1
pub fn i_08bde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDE4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BDE4: cmpi.w #$600, d0
pub fn i_08bde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDE8;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x600, Size::Word, false);
    8
}
/// $08BDE8: bne.w $8bdf0
pub fn i_08bde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDF0; return 10; }
    8
}
/// $08BDEC: move.w #$9, d1
pub fn i_08bdec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDF0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BDF0: cmpi.w #$700, d0
pub fn i_08bdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDF4;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x700, Size::Word, false);
    8
}
/// $08BDF4: bne.w $8bdfc
pub fn i_08bdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BDFC; return 10; }
    8
}
/// $08BDF8: move.w #$9, d1
pub fn i_08bdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BDFC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BDFC: cmpi.w #$d00, d0
pub fn i_08bdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE00;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xD00, Size::Word, false);
    8
}
/// $08BE00: bcs.w $8be08
pub fn i_08be00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE04;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8BE08; return 10; }
    8
}
/// $08BE04: move.w #$9, d1
pub fn i_08be04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE08;
    cpu.ir = 0x323C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BE08: move.w $80(a6), d0
pub fn i_08be08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE0C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08BE0C: jsr $47872.l
pub fn i_08be0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BE12);
    cpu.pc = 0x47872;
    16
}
/// $08BE12: addq.l #$8, $82(a6)
pub fn i_08be12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE16;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08BE16: bra.w $8bcfe
pub fn i_08be16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE1A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BCFE;
    10
}
/// $08BE1A: andi.b #$ee, ccr
pub fn i_08be1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE1E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08BE1E: bra.w $8be26
pub fn i_08be1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE22;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE26;
    10
}
// ===== SetXN_08be22  ($08BE22, 6 B, thunks/ccr_helpers.c)
/// $08BE22: ori.b #$11, ccr
pub fn i_08be22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE26;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08BE26: rts
pub fn i_08be26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_ScriptStepFix_B_08be28  ($08BE28, 360 B, mission/cutscene_anim_08ba0c.s)
/// $08BE28: cmpi.w #$ffff, $70(a6)
pub fn i_08be28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE2E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BE2E: beq.w $8bf90
pub fn i_08be2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BF90; return 10; }
    8
}
/// $08BE32: subq.w #$1, $70(a6)
pub fn i_08be32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE36;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08BE36: cmpi.w #$0, $70(a6)
pub fn i_08be36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE3C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BE3C: bgt.w $8bf88
pub fn i_08be3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE40;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8BF88; return 10; }
    8
}
/// $08BE40: movea.l $7c(a6), a1
pub fn i_08be40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE44;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BE44: adda.l $82(a6), a1
pub fn i_08be44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE48;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $08BE48: move.w (a1), $70(a6)
pub fn i_08be48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE4C;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $08BE4C: cmpi.w #$ffff, $70(a6)
pub fn i_08be4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE52;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BE52: beq.w $8bf88
pub fn i_08be52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE56;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BF88; return 10; }
    8
}
/// $08BE56: move.w $2(a1), $80(a6)
pub fn i_08be56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE5C;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08BE5C: cmpi.l #$0, $4(a1)
pub fn i_08be5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE64;
    cpu.ir = 0xCA9;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0x0, Size::Long, false);
    20
}
/// $08BE64: beq.w $8be72
pub fn i_08be64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BE72; return 10; }
    8
}
/// $08BE68: movea.l $4(a1), a0
pub fn i_08be68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE6C;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08BE6C: jsr $28cd4.l
pub fn i_08be6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BE72);
    cpu.pc = 0x28CD4;
    16
}
/// $08BE72: cmpi.w #$1, $80(a6)
pub fn i_08be72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE78;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $08BE78: bne.w $8be96
pub fn i_08be78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE7C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BE96; return 10; }
    8
}
/// $08BE7C: movea.w #$7084, a1
pub fn i_08be7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE80;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7084;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08BE80: move.w #$2320, d0
pub fn i_08be80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BE84: move.w #$20, d1
pub fn i_08be84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE88;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BE88: move.w #$8, d2
pub fn i_08be88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE8C;
    cpu.ir = 0x343C;
    let v1: u32 = 0x8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08BE8C: jsr $5da9c.l
pub fn i_08be8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BE92);
    cpu.pc = 0x5DA9C;
    16
}
/// $08BE92: bra.w $8be12
pub fn i_08be92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE96;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BE96: cmpi.w #$d, $80(a6)
pub fn i_08be96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BE9C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xD, Size::Word, false);
    16
}
/// $08BE9C: bne.w $8bebe
pub fn i_08be9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEA0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BEBE; return 10; }
    8
}
/// $08BEA0: move.l $86(a6), d0
pub fn i_08bea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEA4;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BEA4: andi.l #$f00f, d0
pub fn i_08bea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEAA;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF00F) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $08BEAA: addi.l #$a0, d0
pub fn i_08beaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEB0;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0xA0, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08BEB0: addi.l #$2, d0
pub fn i_08beb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEB6;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08BEB6: move.l d0, $86(a6)
pub fn i_08beb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEBA;
    cpu.ir = 0x2D40;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BEBA: bra.w $8bf80
pub fn i_08beba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEBE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BEBE: cmpi.w #$2, $80(a6)
pub fn i_08bebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEC4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Word, false);
    16
}
/// $08BEC4: bne.w $8bed6
pub fn i_08bec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEC8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BED6; return 10; }
    8
}
/// $08BEC8: lea.l $8c730(pc), a1
pub fn i_08bec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BECC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C730;
    16
}
/// $08BECC: jsr $4ae.l
pub fn i_08becc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BED2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BED2);
    cpu.pc = 0x4AE;
    16
}
/// $08BED2: bra.w $8be12
pub fn i_08bed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BED6;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BED6: cmpi.w #$3, $80(a6)
pub fn i_08bed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEDC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Word, false);
    16
}
/// $08BEDC: bne.w $8bf02
pub fn i_08bedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEE0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BF02; return 10; }
    8
}
/// $08BEE0: lea.l $2ae50.l, a1
pub fn i_08bee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEE6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2AE50;
    20
}
/// $08BEE6: jsr $4ae.l
pub fn i_08bee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8BEEC);
    cpu.pc = 0x4AE;
    16
}
/// $08BEEC: move.w #$a0, $22(a0)
pub fn i_08beec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEF2;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BEF2: move.w #$1ff, $24(a0)
pub fn i_08bef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEF8;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BEF8: move.b #$3, $98(a0)
pub fn i_08bef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BEFE;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BEFE: bra.w $8be12
pub fn i_08befe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF02;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE12;
    10
}
/// $08BF02: cmpi.w #$0, $80(a6)
pub fn i_08bf02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF08;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BF08: bne.w $8bf14
pub fn i_08bf08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF0C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BF14; return 10; }
    8
}
/// $08BF0C: bra.w $8bf80
pub fn i_08bf0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF10;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BF10: bra.w $8bf80
pub fn i_08bf10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF14;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BF14: move.l $86(a6), d0
pub fn i_08bf14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF18;
    cpu.ir = 0x202E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08BF18: andi.l #$ffff, d0
pub fn i_08bf18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF1E;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFFFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $08BF1E: addi.l #$20, $86(a6)
pub fn i_08bf1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF26;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $08BF26: move.w $80(a6), d1
pub fn i_08bf26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF2A;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08BF2A: ori.w #$2300, d1
pub fn i_08bf2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF2E;
    cpu.ir = 0x41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 | 0x2300) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BF2E: movem.w d0-d1, $3c0000.l
pub fn i_08bf2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF36;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08BF36: cmpi.w #$e6, $80(a6)
pub fn i_08bf36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF3C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE6, Size::Word, false);
    16
}
/// $08BF3C: bne.w $8bf56
pub fn i_08bf3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF40;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8BF56; return 10; }
    8
}
/// $08BF40: addi.l #$1, d0
pub fn i_08bf40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF46;
    cpu.ir = 0x680;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = add_with_flags(cpu, v1, 0x1, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08BF46: addi.w #$10, d1
pub fn i_08bf46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF4A;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BF4A: movem.w d0-d1, $3c0000.l
pub fn i_08bf4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF52;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08BF52: bra.w $8bf80
pub fn i_08bf52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF56;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BF56: cmpi.w #$e7, $80(a6)
pub fn i_08bf56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF5C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xE7, Size::Word, false);
    16
}
/// $08BF5C: bcs.w $8bf80
pub fn i_08bf5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF60;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x8BF80; return 10; }
    8
}
/// $08BF60: cmpi.w #$ef, $80(a6)
pub fn i_08bf60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF66;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xEF, Size::Word, false);
    16
}
/// $08BF66: bhi.w $8bf80
pub fn i_08bf66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF6A;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x8BF80; return 10; }
    8
}
/// $08BF6A: subi.l #$1, d0
pub fn i_08bf6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF70;
    cpu.ir = 0x480;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Long, true);
    cpu.d[0] = r2;
    8
}
/// $08BF70: subi.w #$10, d1
pub fn i_08bf70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF74;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08BF74: movem.w d0-d1, $3c0000.l
pub fn i_08bf74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF7C;
    cpu.ir = 0x48B9;
    let mut a = 0x3C0000;
    bus.write16(a, cpu.d[0] as u16); a = a.wrapping_add(2);
    bus.write16(a, cpu.d[1] as u16); a = a.wrapping_add(2);
    let _ = a;
    20
}
/// $08BF7C: bra.w $8bf80
pub fn i_08bf7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF80;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF80;
    10
}
/// $08BF80: addq.l #$8, $82(a6)
pub fn i_08bf80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF84;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08BF84: bra.w $8be40
pub fn i_08bf84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF88;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BE40;
    10
}
/// $08BF88: andi.b #$ee, ccr
pub fn i_08bf88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF8C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08BF8C: bra.w $8bf94
pub fn i_08bf8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF90;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BF94;
    10
}
// ===== SetXN_08bf90  ($08BF90, 6 B, thunks/ccr_helpers.c)
/// $08BF90: ori.b #$11, ccr
pub fn i_08bf90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF94;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08BF94: rts
pub fn i_08bf94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_ScriptStep_Alt_08bf96  ($08BF96, 108 B, mission/cutscene_anim_08ba0c.s)
/// $08BF96: cmpi.w #$ffff, $70(a6)
pub fn i_08bf96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BF9C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BF9C: beq.w $8c002
pub fn i_08bf9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFA0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C002; return 10; }
    8
}
/// $08BFA0: subq.w #$1, $70(a6)
pub fn i_08bfa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFA4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08BFA4: cmpi.w #$0, $70(a6)
pub fn i_08bfa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFAA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08BFAA: bgt.w $8bff4
pub fn i_08bfaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFAE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8BFF4; return 10; }
    8
}
/// $08BFAE: movea.l $7c(a6), a1
pub fn i_08bfae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFB2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $08BFB2: adda.l $78(a6), a1
pub fn i_08bfb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFB6;
    cpu.ir = 0xD3EE;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    let v3 = bus.read32(ad2);
    cpu.a[1] = cpu.a[1].wrapping_add(v3);
    20
}
/// $08BFB6: move.w (a1), $70(a6)
pub fn i_08bfb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFBA;
    cpu.ir = 0x3D51;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad3, v2 as u16);
    16
}
/// $08BFBA: cmpi.w #$ffff, $70(a6)
pub fn i_08bfba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFC0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $08BFC0: beq.w $8bfde
pub fn i_08bfc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFC4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8BFDE; return 10; }
    8
}
/// $08BFC4: move.w $2(a1), $76(a6)
pub fn i_08bfc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFCA;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08BFCA: move.b $4(a1), $92(a6)
pub fn i_08bfca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFD0;
    cpu.ir = 0x1D69;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08BFD0: move.b $4(a1), $93(a6)
pub fn i_08bfd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFD6;
    cpu.ir = 0x1D69;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad4, v3 as u8);
    20
}
/// $08BFD6: addq.l #$8, $78(a6)
pub fn i_08bfd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFDA;
    cpu.ir = 0x50AE;
    let ad1 = cpu.a[6].wrapping_add(0x78);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 8, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08BFDA: bra.w $8bff0
pub fn i_08bfda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFDE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BFF0;
    10
}
/// $08BFDE: move.w #$0, $76(a6)
pub fn i_08bfde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFE4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08BFE4: move.b #$0, $92(a6)
pub fn i_08bfe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFEA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x92);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BFEA: move.b #$0, $93(a6)
pub fn i_08bfea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFF0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BFF0: bra.w $8bffa
pub fn i_08bff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFF4;
    cpu.ir = 0x6000;
    cpu.pc = 0x8BFFA;
    10
}
/// $08BFF4: move.b #$0, $93(a6)
pub fn i_08bff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFFA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x93);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08BFFA: andi.b #$ee, ccr
pub fn i_08bffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8BFFE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $08BFFE: bra.w $8c006
pub fn i_08bffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C002;
    cpu.ir = 0x6000;
    cpu.pc = 0x8C006;
    10
}
// ===== SetXN_08c002  ($08C002, 6 B, thunks/ccr_helpers.c)
/// $08C002: ori.b #$11, ccr
pub fn i_08c002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C006;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $08C006: rts
pub fn i_08c006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C008;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F1_08C008  ($08C008, 342 B, entity/anim_state_machine_08c008.s)
/// $08C008: jsr $22c8.l
pub fn i_08c008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C00E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C00E);
    cpu.pc = 0x22C8;
    16
}
/// $08C00E: move.w #$2b, d0
pub fn i_08c00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C012;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C012: jsr $2352.l
pub fn i_08c012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C018;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C018);
    cpu.pc = 0x2352;
    16
}
/// $08C018: move.w #$12e, d1
pub fn i_08c018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C01C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C01C: jsr $236e.l
pub fn i_08c01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C022;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C022);
    cpu.pc = 0x236E;
    16
}
/// $08C022: lea.l $2f2dbe.l, a0
pub fn i_08c022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C028;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2DBE;
    20
}
/// $08C028: jsr $28cd4.l
pub fn i_08c028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C02E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C02E);
    cpu.pc = 0x28CD4;
    16
}
/// $08C02E: move.w #$a0, $22(a6)
pub fn i_08c02e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C034;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C034: move.w #$1c3, $24(a6)
pub fn i_08c034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C03A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C03A: clr.w $26(a6)
pub fn i_08c03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C03E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C03E: move.w #$4000, $38(a6)
pub fn i_08c03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C044;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C044: bset.b #$6, $12(a6)
pub fn i_08c044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C04A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C04A: move.b #$9, d0
pub fn i_08c04a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C04E;
    cpu.ir = 0x103C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08C04E: jsr $43568.l
pub fn i_08c04e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C054;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C054);
    cpu.pc = 0x43568;
    16
}
/// $08C054: move.w #$0, $70(a6)
pub fn i_08c054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C05A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C05A: move.w #$0, $76(a6)
pub fn i_08c05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C060;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C060: move.l #$0, $78(a6)
pub fn i_08c060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C068;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x78);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C068: lea.l $2ef8aa.l, a1
pub fn i_08c068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C06E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2EF8AA;
    20
}
/// $08C06E: move.l a1, $7c(a6)
pub fn i_08c06e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C072;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C072: move.l #$0, $90(a6)
pub fn i_08c072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C07A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x0;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C07A: movea.l a6, a1
pub fn i_08c07a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C07C;
    cpu.ir = 0x224E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $08C07C: adda.l #$90, a1
pub fn i_08c07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C082;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x90;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $08C082: move.l a1, $72(a6)
pub fn i_08c082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C086;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08C086: lea.l $2ae50.l, a1
pub fn i_08c086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C08C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2AE50;
    20
}
/// $08C08C: jsr $4ae.l
pub fn i_08c08c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C092;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C092);
    cpu.pc = 0x4AE;
    16
}
/// $08C092: move.w #$a0, $22(a0)
pub fn i_08c092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C098;
    cpu.ir = 0x317C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C098: move.w #$1ff, $24(a0)
pub fn i_08c098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C09E;
    cpu.ir = 0x317C;
    let v1: u32 = 0x1FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C09E: move.b #$3, $98(a0)
pub fn i_08c09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0A4;
    cpu.ir = 0x117C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C0A4: lea.l $100440.l, a0
pub fn i_08c0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $08C0AA: move.l #$335a6, (a0)
pub fn i_08c0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0B0;
    cpu.ir = 0x20BC;
    let v1: u32 = 0x335A6;
    bus.write32(cpu.a[0], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C0B0: jsr $5fe.l
pub fn i_08c0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0B6);
    cpu.pc = 0x5FE;
    16
}
/// $08C0B6: lea.l $8c2b8(pc), a1
pub fn i_08c0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C2B8;
    16
}
/// $08C0BA: jsr $4ae.l
pub fn i_08c0ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0C0);
    cpu.pc = 0x4AE;
    16
}
/// $08C0C0: lea.l $8c322(pc), a1
pub fn i_08c0c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C322;
    16
}
/// $08C0C4: jsr $4ae.l
pub fn i_08c0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0CA);
    cpu.pc = 0x4AE;
    16
}
/// $08C0CA: lea.l $8c37e(pc), a1
pub fn i_08c0ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0CE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C37E;
    16
}
/// $08C0CE: jsr $4ae.l
pub fn i_08c0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0D4);
    cpu.pc = 0x4AE;
    16
}
/// $08C0D4: lea.l $8c3da(pc), a1
pub fn i_08c0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C3DA;
    16
}
/// $08C0D8: jsr $4ae.l
pub fn i_08c0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0DE);
    cpu.pc = 0x4AE;
    16
}
/// $08C0DE: lea.l $8c436(pc), a1
pub fn i_08c0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C436;
    16
}
/// $08C0E2: jsr $4ae.l
pub fn i_08c0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0E8);
    cpu.pc = 0x4AE;
    16
}
/// $08C0E8: lea.l $8c5b2(pc), a1
pub fn i_08c0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C5B2;
    16
}
/// $08C0EC: jsr $4ae.l
pub fn i_08c0ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C0F2);
    cpu.pc = 0x4AE;
    16
}
/// $08C0F2: move.b #$0, $32(a6)
pub fn i_08c0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C0F8: move.b #$0, $33(a6)
pub fn i_08c0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C0FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C0FE: move.w #$0, $34(a6)
pub fn i_08c0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C104;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C104: lea.l $8c10a(pc), a1
pub fn i_08c104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C108;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C10A;
    16
}
/// $08C108: move.l a1, (a6)
pub fn i_08c108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C10A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C10A: lea.l $2c072c.l, a1
pub fn i_08c10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C110;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $08C110: move.w $34(a6), d0
pub fn i_08c110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C114;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08C114: lsl.w #$1, d0
pub fn i_08c114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C116;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08C116: move.w (a1, d0.w), d1
pub fn i_08c116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C11A;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08C11A: cmpi.w #$ff, d1
pub fn i_08c11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C11E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Word, false);
    8
}
/// $08C11E: ble.w $8c126
pub fn i_08c11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C122;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C126; return 10; }
    8
}
/// $08C122: move.w #$ff, d1
pub fn i_08c122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C126;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C126: move.b d1, $33(a6)
pub fn i_08c126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C12A;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C12A: addq.w #$8, $34(a6)
pub fn i_08c12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C12E;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C12E: cmpi.w #$3f, $34(a6)
pub fn i_08c12e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C134;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3F, Size::Word, false);
    16
}
/// $08C134: ble.w $8c14a
pub fn i_08c134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C138;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C14A; return 10; }
    8
}
/// $08C138: move.w #$0, $34(a6)
pub fn i_08c138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C13E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C13E: move.b #$ff, $33(a6)
pub fn i_08c13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C144;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C144: lea.l $8c15e(pc), a1
pub fn i_08c144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C148;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C15E;
    16
}
/// $08C148: move.l a1, (a6)
pub fn i_08c148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C14A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C14A: jsr $967c0.l
pub fn i_08c14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C150);
    cpu.pc = 0x967C0;
    16
}
/// $08C150: jsr $436de.l
pub fn i_08c150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C156;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C156);
    cpu.pc = 0x436DE;
    16
}
/// $08C156: jsr $28d70.l
pub fn i_08c156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C15C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C15C);
    cpu.pc = 0x28D70;
    16
}
/// $08C15C: rts
pub fn i_08c15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C15E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F2_08C15E  ($08C15E, 78 B, entity/anim_state_machine_08c008.s)
/// $08C15E: lea.l $2c072c.l, a1
pub fn i_08c15e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C164;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C072C;
    20
}
/// $08C164: move.w $34(a6), d0
pub fn i_08c164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C168;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08C168: lsl.w #$1, d0
pub fn i_08c168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C16A;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08C16A: move.w (a1, d0.w), d1
pub fn i_08c16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C16E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08C16E: cmpi.w #$ff, d1
pub fn i_08c16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C172;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Word, false);
    8
}
/// $08C172: ble.w $8c17a
pub fn i_08c172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C176;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C17A; return 10; }
    8
}
/// $08C176: move.w #$ff, d1
pub fn i_08c176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C17A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C17A: move.b d1, $32(a6)
pub fn i_08c17a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C17E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C17E: addq.w #$4, $34(a6)
pub fn i_08c17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C182;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C182: cmpi.w #$3f, $34(a6)
pub fn i_08c182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C188;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3F, Size::Word, false);
    16
}
/// $08C188: ble.w $8c198
pub fn i_08c188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C18C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C198; return 10; }
    8
}
/// $08C18C: move.b #$ff, $32(a6)
pub fn i_08c18c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C192;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C192: lea.l $8c1ac(pc), a1
pub fn i_08c192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C196;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C1AC;
    16
}
/// $08C196: move.l a1, (a6)
pub fn i_08c196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C198;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C198: jsr $967c0.l
pub fn i_08c198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C19E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C19E);
    cpu.pc = 0x967C0;
    16
}
/// $08C19E: jsr $436de.l
pub fn i_08c19e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1A4);
    cpu.pc = 0x436DE;
    16
}
/// $08C1A4: jsr $28d70.l
pub fn i_08c1a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1AA);
    cpu.pc = 0x28D70;
    16
}
/// $08C1AA: rts
pub fn i_08c1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F3_08C1AC  ($08C1AC, 62 B, entity/anim_state_machine_08c008.s)
/// $08C1AC: jsr $8bc74(pc)
pub fn i_08c1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8C1B0);
    cpu.pc = 0x8BC74;
    16
}
/// $08C1B0: bcc.w $8c1d6
pub fn i_08c1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8C1D6; return 10; }
    8
}
/// $08C1B4: movea.w #$7084, a1
pub fn i_08c1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1B8;
    cpu.ir = 0x327C;
    let v1: u32 = 0x7084;
    cpu.a[1] = v1 as i16 as i32 as u32;
    4
}
/// $08C1B8: move.w #$2320, d0
pub fn i_08c1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1BC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2320;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C1BC: move.w #$20, d1
pub fn i_08c1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C1C0: move.w #$19, d2
pub fn i_08c1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1C4;
    cpu.ir = 0x343C;
    let v1: u32 = 0x19;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C1C4: jsr $5da9c.l
pub fn i_08c1c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1CA);
    cpu.pc = 0x5DA9C;
    16
}
/// $08C1CA: move.w #$0, $34(a6)
pub fn i_08c1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C1D0: lea.l $8c1ea(pc), a1
pub fn i_08c1d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C1EA;
    16
}
/// $08C1D4: move.l a1, (a6)
pub fn i_08c1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C1D6: jsr $967c0.l
pub fn i_08c1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1DC);
    cpu.pc = 0x967C0;
    16
}
/// $08C1DC: jsr $436de.l
pub fn i_08c1dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1E2);
    cpu.pc = 0x436DE;
    16
}
/// $08C1E2: jsr $28d70.l
pub fn i_08c1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C1E8);
    cpu.pc = 0x28D70;
    16
}
/// $08C1E8: rts
pub fn i_08c1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F4_08C1EA  ($08C1EA, 80 B, entity/anim_state_machine_08c008.s)
/// $08C1EA: lea.l $2c07ac.l, a1
pub fn i_08c1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1F0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $08C1F0: move.w $34(a6), d0
pub fn i_08c1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1F4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08C1F4: lsl.w #$1, d0
pub fn i_08c1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1F6;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08C1F6: move.w (a1, d0.w), d1
pub fn i_08c1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1FA;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08C1FA: subq.w #$1, d1
pub fn i_08c1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C1FC;
    cpu.ir = 0x5341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08C1FC: bcc.w $8c202
pub fn i_08c1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C200;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8C202; return 10; }
    8
}
/// $08C200: clr.w d1
pub fn i_08c200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C202;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08C202: move.b d1, $32(a6)
pub fn i_08c202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C206;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C206: addq.w #$4, $34(a6)
pub fn i_08c206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C20A;
    cpu.ir = 0x586E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 4, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C20A: cmpi.w #$3f, $34(a6)
pub fn i_08c20a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C210;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3F, Size::Word, false);
    16
}
/// $08C210: ble.w $8c226
pub fn i_08c210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C214;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C226; return 10; }
    8
}
/// $08C214: move.b #$0, $32(a6)
pub fn i_08c214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C21A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C21A: move.w #$0, $34(a6)
pub fn i_08c21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C220;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C220: lea.l $8c23a(pc), a1
pub fn i_08c220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C224;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C23A;
    16
}
/// $08C224: move.l a1, (a6)
pub fn i_08c224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C226;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C226: jsr $967c0.l
pub fn i_08c226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C22C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C22C);
    cpu.pc = 0x967C0;
    16
}
/// $08C22C: jsr $436de.l
pub fn i_08c22c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C232;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C232);
    cpu.pc = 0x436DE;
    16
}
/// $08C232: jsr $28d70.l
pub fn i_08c232(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C238;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C238);
    cpu.pc = 0x28D70;
    16
}
/// $08C238: rts
pub fn i_08c238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C23A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F5_08C23A  ($08C23A, 92 B, entity/anim_state_machine_08c008.s)
/// $08C23A: lea.l $2c07ac.l, a1
pub fn i_08c23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C240;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $08C240: move.w $34(a6), d0
pub fn i_08c240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C244;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08C244: lsl.w #$1, d0
pub fn i_08c244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C246;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $08C246: move.w (a1, d0.w), d1
pub fn i_08c246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C24A;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08C24A: move.b d1, $33(a6)
pub fn i_08c24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C24E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C24E: addq.w #$8, $34(a6)
pub fn i_08c24e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C252;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C252: cmpi.w #$3f, $34(a6)
pub fn i_08c252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C258;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3F, Size::Word, false);
    16
}
/// $08C258: ble.w $8c282
pub fn i_08c258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C25C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x8C282; return 10; }
    8
}
/// $08C25C: move.b #$80, d0
pub fn i_08c25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C260;
    cpu.ir = 0x103C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $08C260: jsr $2308.l
pub fn i_08c260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C266;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C266);
    cpu.pc = 0x2308;
    16
}
/// $08C266: move.w #$2, d0
pub fn i_08c266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C26A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C26A: jsr $5239e.l
pub fn i_08c26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C270);
    cpu.pc = 0x5239E;
    16
}
/// $08C270: move.w #$3c, $70(a6)
pub fn i_08c270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C276;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C276: move.b #$0, $33(a6)
pub fn i_08c276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C27C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C27C: lea.l $8c296(pc), a1
pub fn i_08c27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C280;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C296;
    16
}
/// $08C280: move.l a1, (a6)
pub fn i_08c280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C282;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C282: jsr $967c0.l
pub fn i_08c282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C288;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C288);
    cpu.pc = 0x967C0;
    16
}
/// $08C288: jsr $436de.l
pub fn i_08c288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C28E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C28E);
    cpu.pc = 0x436DE;
    16
}
/// $08C28E: jsr $28d70.l
pub fn i_08c28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C294;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C294);
    cpu.pc = 0x28D70;
    16
}
/// $08C294: rts
pub fn i_08c294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C296;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Anim_State_F6_08C296  ($08C296, 34 B, entity/anim_state_machine_08c008.s)
/// $08C296: subq.w #$1, $70(a6)
pub fn i_08c296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C29A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C29A: cmpi.w #$0, $70(a6)
pub fn i_08c29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2A0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08C2A0: bgt.w $8c2aa
pub fn i_08c2a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2A4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8C2AA; return 10; }
    8
}
/// $08C2A4: clr.b $106ed2.l
pub fn i_08c2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2AA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $08C2AA: jsr $967c0.l
pub fn i_08c2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C2B0);
    cpu.pc = 0x967C0;
    16
}
/// $08C2B0: jsr $436de.l
pub fn i_08c2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C2B6);
    cpu.pc = 0x436DE;
    16
}
/// $08C2B6: rts
pub fn i_08c2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Base_08c2b8  ($08C2B8, 98 B, mission/cutscene_anim_08ba0c.s)
/// $08C2B8: move.w #$e000, $38(a6)
pub fn i_08c2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C2BE: bset.b #$6, $12(a6)
pub fn i_08c2be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2C4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C2C4: move.w #$b1, d1
pub fn i_08c2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2C8;
    cpu.ir = 0x323C;
    let v1: u32 = 0xB1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C2C8: jsr $236e.l
pub fn i_08c2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C2CE);
    cpu.pc = 0x236E;
    16
}
/// $08C2CE: lea.l $2f2e0a.l, a0
pub fn i_08c2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2D4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2E0A;
    20
}
/// $08C2D4: jsr $28cd4.l
pub fn i_08c2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C2DA);
    cpu.pc = 0x28CD4;
    16
}
/// $08C2DA: move.w #$40, $22(a6)
pub fn i_08c2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2E0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C2E0: move.w #$12b, $24(a6)
pub fn i_08c2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C2E6: clr.w $26(a6)
pub fn i_08c2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2EA;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C2EA: lea.l $8c2f0(pc), a1
pub fn i_08c2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C2F0;
    16
}
/// $08C2EE: move.l a1, (a6)
pub fn i_08c2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08C2F0: movea.l $c(a6), a0
pub fn i_08c2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2F4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08C2F4: move.b $92(a0), d0
pub fn i_08c2f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2F8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x92);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $08C2F8: andi.l #$f, d0
pub fn i_08c2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C2FE;
    cpu.ir = 0x280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let r2 = (v1 & 0xF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[0] = r2;
    8
}
/// $08C2FE: movea.l #$2f2dca, a0
pub fn i_08c2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C304;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2F2DCA;
    cpu.a[0] = v1;
    4
}
/// $08C304: lsl.w #$2, d0
pub fn i_08c304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C306;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08C306: movea.l (a0, d0.w), a0
pub fn i_08c306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C30A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08C30A: cmpa.l #$ffffffff, a0
pub fn i_08c30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C310;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08C310: beq.w $8c31a
pub fn i_08c310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C314;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8C31A; return 10; }
    8
}
/// $08C314: jsr $28cd4.l
pub fn i_08c314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C31A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C31A);
    cpu.pc = 0x28CD4;
    16
}
// ===== JsrAbsThunk_08c31a  ($08C31A, 8 B, thunks/jsr_abs_thunks.c)
/// $08C31A: jsr $28d70.l
pub fn i_08c31a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C320;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C320);
    cpu.pc = 0x28D70;
    16
}
/// $08C320: rts
pub fn i_08c320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C322;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot1_08c322  ($08C322, 84 B, mission/cutscene_anim_08ba0c.s)
/// $08C322: move.w #$e000, $38(a6)
pub fn i_08c322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C328;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C328: bset.b #$6, $12(a6)
pub fn i_08c328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C32E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C32E: move.w #$ae, d1
pub fn i_08c32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C332;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C332: jsr $236e.l
pub fn i_08c332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C338;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C338);
    cpu.pc = 0x236E;
    16
}
/// $08C338: lea.l $2f2ed6.l, a0
pub fn i_08c338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C33E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2ED6;
    20
}
/// $08C33E: jsr $28cd4.l
pub fn i_08c33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C344;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C344);
    cpu.pc = 0x28CD4;
    16
}
/// $08C344: move.w #$70, $22(a6)
pub fn i_08c344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C34A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x70;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C34A: move.w #$12b, $24(a6)
pub fn i_08c34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C350;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C350: clr.w $26(a6)
pub fn i_08c350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C354;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C354: lea.l $8c4fa(pc), a1
pub fn i_08c354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C358;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4FA;
    16
}
/// $08C358: jsr $4ae.l
pub fn i_08c358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C35E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C35E);
    cpu.pc = 0x4AE;
    16
}
/// $08C35E: jsr $5dd02.l
pub fn i_08c35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C364;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C364);
    cpu.pc = 0x5DD02;
    16
}
/// $08C364: move.b #$10, $8a(a6)
pub fn i_08c364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C36A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C36A: move.w #$0, $76(a6)
pub fn i_08c36a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C370;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C370: lea.l $8c4a0(pc), a1
pub fn i_08c370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C374;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4A0;
    16
}
/// $08C374: move.l a1, (a6)
pub fn i_08c374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C376;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08c376  ($08C376, 8 B, thunks/jsr_abs_thunks.c)
/// $08C376: jsr $28d70.l
pub fn i_08c376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C37C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C37C);
    cpu.pc = 0x28D70;
    16
}
/// $08C37C: rts
pub fn i_08c37c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C37E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Icon_Slot2_08c37e  ($08C37E, 84 B, mission/cutscene_anim_08ba0c.s)
/// $08C37E: move.w #$e000, $38(a6)
pub fn i_08c37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C384;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C384: bset.b #$6, $12(a6)
pub fn i_08c384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C38A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08C38A: move.w #$af, d1
pub fn i_08c38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C38E;
    cpu.ir = 0x323C;
    let v1: u32 = 0xAF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08C38E: jsr $236e.l
pub fn i_08c38e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C394);
    cpu.pc = 0x236E;
    16
}
/// $08C394: lea.l $2f2ed6.l, a0
pub fn i_08c394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C39A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2F2ED6;
    20
}
/// $08C39A: jsr $28cd4.l
pub fn i_08c39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3A0);
    cpu.pc = 0x28CD4;
    16
}
/// $08C3A0: move.w #$a0, $22(a6)
pub fn i_08c3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3A6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C3A6: move.w #$12b, $24(a6)
pub fn i_08c3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3AC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C3AC: clr.w $26(a6)
pub fn i_08c3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3B0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x26);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08C3B0: lea.l $8c526(pc), a1
pub fn i_08c3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3B4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C526;
    16
}
/// $08C3B4: jsr $4ae.l
pub fn i_08c3b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3BA);
    cpu.pc = 0x4AE;
    16
}
/// $08C3BA: jsr $5dd02.l
pub fn i_08c3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3C0);
    cpu.pc = 0x5DD02;
    16
}
/// $08C3C0: move.b #$20, $8a(a6)
pub fn i_08c3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3C6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08C3C6: move.w #$0, $76(a6)
pub fn i_08c3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08C3CC: lea.l $8c4a0(pc), a1
pub fn i_08c3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8C4A0;
    16
}
/// $08C3D0: move.l a1, (a6)
pub fn i_08c3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrAbsThunk_08c3d2  ($08C3D2, 8 B, thunks/jsr_abs_thunks.c)
/// $08C3D2: jsr $28d70.l
pub fn i_08c3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8C3D8);
    cpu.pc = 0x28D70;
    16
}
/// $08C3D8: rts
pub fn i_08c3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8C3DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x08A65C, i_08a65c),
    (0x08A662, i_08a662),
    (0x08A668, i_08a668),
    (0x08A66E, i_08a66e),
    (0x08A674, i_08a674),
    (0x08A67A, i_08a67a),
    (0x08A680, i_08a680),
    (0x08A686, i_08a686),
    (0x08A68C, i_08a68c),
    (0x08A692, i_08a692),
    (0x08A698, i_08a698),
    (0x08A69E, i_08a69e),
    (0x08A6A4, i_08a6a4),
    (0x08A6AA, i_08a6aa),
    (0x08A6B0, i_08a6b0),
    (0x08A6B6, i_08a6b6),
    (0x08A6BC, i_08a6bc),
    (0x08A6C2, i_08a6c2),
    (0x08A6C8, i_08a6c8),
    (0x08A6CE, i_08a6ce),
    (0x08A6D4, i_08a6d4),
    (0x08A6DA, i_08a6da),
    (0x08A6E0, i_08a6e0),
    (0x08A6E6, i_08a6e6),
    (0x08A6EC, i_08a6ec),
    (0x08A6F2, i_08a6f2),
    (0x08A6F8, i_08a6f8),
    (0x08A6FE, i_08a6fe),
    (0x08A704, i_08a704),
    (0x08A708, i_08a708),
    (0x08A70E, i_08a70e),
    (0x08A714, i_08a714),
    (0x08A71A, i_08a71a),
    (0x08A71E, i_08a71e),
    (0x08A724, i_08a724),
    (0x08A72A, i_08a72a),
    (0x08A730, i_08a730),
    (0x08A736, i_08a736),
    (0x08A73C, i_08a73c),
    (0x08A742, i_08a742),
    (0x08A746, i_08a746),
    (0x08A74C, i_08a74c),
    (0x08A74E, i_08a74e),
    (0x08A752, i_08a752),
    (0x08A758, i_08a758),
    (0x08A75C, i_08a75c),
    (0x08A762, i_08a762),
    (0x08A766, i_08a766),
    (0x08A76C, i_08a76c),
    (0x08A772, i_08a772),
    (0x08A778, i_08a778),
    (0x08A77E, i_08a77e),
    (0x08A784, i_08a784),
    (0x08A78A, i_08a78a),
    (0x08A790, i_08a790),
    (0x08A798, i_08a798),
    (0x08A79C, i_08a79c),
    (0x08A7A2, i_08a7a2),
    (0x08A7A6, i_08a7a6),
    (0x08A7AC, i_08a7ac),
    (0x08A7B2, i_08a7b2),
    (0x08A7B6, i_08a7b6),
    (0x08A7BC, i_08a7bc),
    (0x08A7C2, i_08a7c2),
    (0x08A7C8, i_08a7c8),
    (0x08A7CE, i_08a7ce),
    (0x08A7D4, i_08a7d4),
    (0x08A7D8, i_08a7d8),
    (0x08A7DE, i_08a7de),
    (0x08A7E2, i_08a7e2),
    (0x08A7E6, i_08a7e6),
    (0x08A7EC, i_08a7ec),
    (0x08A7F2, i_08a7f2),
    (0x08A7F8, i_08a7f8),
    (0x08A7FE, i_08a7fe),
    (0x08A804, i_08a804),
    (0x08A808, i_08a808),
    (0x08A80E, i_08a80e),
    (0x08A812, i_08a812),
    (0x08A816, i_08a816),
    (0x08A81C, i_08a81c),
    (0x08A822, i_08a822),
    (0x08A828, i_08a828),
    (0x08A82E, i_08a82e),
    (0x08A834, i_08a834),
    (0x08A838, i_08a838),
    (0x08A83E, i_08a83e),
    (0x08A842, i_08a842),
    (0x08A846, i_08a846),
    (0x08A84C, i_08a84c),
    (0x08A852, i_08a852),
    (0x08A858, i_08a858),
    (0x08A85E, i_08a85e),
    (0x08A864, i_08a864),
    (0x08A868, i_08a868),
    (0x08A86E, i_08a86e),
    (0x08A872, i_08a872),
    (0x08A878, i_08a878),
    (0x08A87E, i_08a87e),
    (0x08A884, i_08a884),
    (0x08A88A, i_08a88a),
    (0x08A890, i_08a890),
    (0x08A896, i_08a896),
    (0x08A89C, i_08a89c),
    (0x08A8A2, i_08a8a2),
    (0x08A8A8, i_08a8a8),
    (0x08A8AE, i_08a8ae),
    (0x08A8B4, i_08a8b4),
    (0x08A8B8, i_08a8b8),
    (0x08A8BE, i_08a8be),
    (0x08A8C2, i_08a8c2),
    (0x08A8C4, i_08a8c4),
    (0x08A8CA, i_08a8ca),
    (0x08A8D0, i_08a8d0),
    (0x08A8D6, i_08a8d6),
    (0x08A8DA, i_08a8da),
    (0x08A8E0, i_08a8e0),
    (0x08A8E6, i_08a8e6),
    (0x08A8EA, i_08a8ea),
    (0x08A8F0, i_08a8f0),
    (0x08A8F4, i_08a8f4),
    (0x08A8FA, i_08a8fa),
    (0x08A900, i_08a900),
    (0x08A906, i_08a906),
    (0x08A90C, i_08a90c),
    (0x08A912, i_08a912),
    (0x08A916, i_08a916),
    (0x08A91C, i_08a91c),
    (0x08A922, i_08a922),
    (0x08A926, i_08a926),
    (0x08A92C, i_08a92c),
    (0x08A930, i_08a930),
    (0x08A938, i_08a938),
    (0x08A93C, i_08a93c),
    (0x08A942, i_08a942),
    (0x08A948, i_08a948),
    (0x08A94E, i_08a94e),
    (0x08A954, i_08a954),
    (0x08A95A, i_08a95a),
    (0x08A960, i_08a960),
    (0x08A966, i_08a966),
    (0x08A96C, i_08a96c),
    (0x08A972, i_08a972),
    (0x08A978, i_08a978),
    (0x08A97C, i_08a97c),
    (0x08A97E, i_08a97e),
    (0x08A980, i_08a980),
    (0x08A984, i_08a984),
    (0x08A98A, i_08a98a),
    (0x08A990, i_08a990),
    (0x08A996, i_08a996),
    (0x08A99C, i_08a99c),
    (0x08A9A2, i_08a9a2),
    (0x08A9A8, i_08a9a8),
    (0x08A9AC, i_08a9ac),
    (0x08A9AE, i_08a9ae),
    (0x08A9B0, i_08a9b0),
    (0x08A9B6, i_08a9b6),
    (0x08A9BC, i_08a9bc),
    (0x08A9C2, i_08a9c2),
    (0x08A9C8, i_08a9c8),
    (0x08A9CE, i_08a9ce),
    (0x08A9D4, i_08a9d4),
    (0x08A9DA, i_08a9da),
    (0x08A9E0, i_08a9e0),
    (0x08A9E6, i_08a9e6),
    (0x08A9EC, i_08a9ec),
    (0x08A9F2, i_08a9f2),
    (0x08A9F6, i_08a9f6),
    (0x08A9FC, i_08a9fc),
    (0x08AA00, i_08aa00),
    (0x08AA02, i_08aa02),
    (0x08AA08, i_08aa08),
    (0x08AA0A, i_08aa0a),
    (0x08AA0E, i_08aa0e),
    (0x08AA12, i_08aa12),
    (0x08AA18, i_08aa18),
    (0x08AA1E, i_08aa1e),
    (0x08AA24, i_08aa24),
    (0x08AA2A, i_08aa2a),
    (0x08AA2E, i_08aa2e),
    (0x08AA34, i_08aa34),
    (0x08AA3A, i_08aa3a),
    (0x08AA40, i_08aa40),
    (0x08AA46, i_08aa46),
    (0x08AA48, i_08aa48),
    (0x08AA4E, i_08aa4e),
    (0x08AA54, i_08aa54),
    (0x08AA58, i_08aa58),
    (0x08AA5C, i_08aa5c),
    (0x08AA5E, i_08aa5e),
    (0x08AA64, i_08aa64),
    (0x08AA6A, i_08aa6a),
    (0x08AA70, i_08aa70),
    (0x08AA76, i_08aa76),
    (0x08AA7A, i_08aa7a),
    (0x08AA80, i_08aa80),
    (0x08AA84, i_08aa84),
    (0x08AA8A, i_08aa8a),
    (0x08AA90, i_08aa90),
    (0x08AA96, i_08aa96),
    (0x08AA9C, i_08aa9c),
    (0x08AAA2, i_08aaa2),
    (0x08AAA6, i_08aaa6),
    (0x08AAAC, i_08aaac),
    (0x08AAB2, i_08aab2),
    (0x08AAB8, i_08aab8),
    (0x08AABE, i_08aabe),
    (0x08AAC2, i_08aac2),
    (0x08AAC6, i_08aac6),
    (0x08AACC, i_08aacc),
    (0x08AAD2, i_08aad2),
    (0x08AAD8, i_08aad8),
    (0x08AADE, i_08aade),
    (0x08AAE4, i_08aae4),
    (0x08AAEA, i_08aaea),
    (0x08AAEE, i_08aaee),
    (0x08AAF0, i_08aaf0),
    (0x08AAF2, i_08aaf2),
    (0x08AAF8, i_08aaf8),
    (0x08AAFE, i_08aafe),
    (0x08AB04, i_08ab04),
    (0x08AB0A, i_08ab0a),
    (0x08AB10, i_08ab10),
    (0x08AB16, i_08ab16),
    (0x08AB1C, i_08ab1c),
    (0x08AB22, i_08ab22),
    (0x08AB28, i_08ab28),
    (0x08AB2E, i_08ab2e),
    (0x08AB34, i_08ab34),
    (0x08AB3A, i_08ab3a),
    (0x08AB3E, i_08ab3e),
    (0x08AB40, i_08ab40),
    (0x08AB46, i_08ab46),
    (0x08AB4C, i_08ab4c),
    (0x08AB52, i_08ab52),
    (0x08AB58, i_08ab58),
    (0x08AB5C, i_08ab5c),
    (0x08AB62, i_08ab62),
    (0x08AB66, i_08ab66),
    (0x08AB6C, i_08ab6c),
    (0x08AB72, i_08ab72),
    (0x08AB78, i_08ab78),
    (0x08AB7E, i_08ab7e),
    (0x08AB84, i_08ab84),
    (0x08AB88, i_08ab88),
    (0x08AB8E, i_08ab8e),
    (0x08AB94, i_08ab94),
    (0x08AB9A, i_08ab9a),
    (0x08ABA0, i_08aba0),
    (0x08ABA4, i_08aba4),
    (0x08ABA8, i_08aba8),
    (0x08ABAE, i_08abae),
    (0x08ABB4, i_08abb4),
    (0x08ABBA, i_08abba),
    (0x08ABC0, i_08abc0),
    (0x08ABC6, i_08abc6),
    (0x08ABCC, i_08abcc),
    (0x08ABD0, i_08abd0),
    (0x08ABD2, i_08abd2),
    (0x08ABD4, i_08abd4),
    (0x08ABDA, i_08abda),
    (0x08ABE0, i_08abe0),
    (0x08ABE6, i_08abe6),
    (0x08ABEC, i_08abec),
    (0x08ABF2, i_08abf2),
    (0x08ABF8, i_08abf8),
    (0x08ABFE, i_08abfe),
    (0x08AC04, i_08ac04),
    (0x08AC0A, i_08ac0a),
    (0x08AC10, i_08ac10),
    (0x08AC16, i_08ac16),
    (0x08AC1C, i_08ac1c),
    (0x08AC20, i_08ac20),
    (0x08AC22, i_08ac22),
    (0x08AC28, i_08ac28),
    (0x08AC2E, i_08ac2e),
    (0x08AC34, i_08ac34),
    (0x08AC3A, i_08ac3a),
    (0x08AC3E, i_08ac3e),
    (0x08AC44, i_08ac44),
    (0x08AC48, i_08ac48),
    (0x08AC4E, i_08ac4e),
    (0x08AC54, i_08ac54),
    (0x08AC5A, i_08ac5a),
    (0x08AC60, i_08ac60),
    (0x08AC66, i_08ac66),
    (0x08AC6A, i_08ac6a),
    (0x08AC70, i_08ac70),
    (0x08AC76, i_08ac76),
    (0x08AC7C, i_08ac7c),
    (0x08AC82, i_08ac82),
    (0x08AC86, i_08ac86),
    (0x08AC8C, i_08ac8c),
    (0x08AC92, i_08ac92),
    (0x08AC98, i_08ac98),
    (0x08AC9C, i_08ac9c),
    (0x08ACA2, i_08aca2),
    (0x08ACA8, i_08aca8),
    (0x08ACAE, i_08acae),
    (0x08ACB4, i_08acb4),
    (0x08ACBA, i_08acba),
    (0x08ACC0, i_08acc0),
    (0x08ACC6, i_08acc6),
    (0x08ACCC, i_08accc),
    (0x08ACD2, i_08acd2),
    (0x08ACD8, i_08acd8),
    (0x08ACDE, i_08acde),
    (0x08ACE4, i_08ace4),
    (0x08ACEA, i_08acea),
    (0x08ACF0, i_08acf0),
    (0x08ACF6, i_08acf6),
    (0x08ACFC, i_08acfc),
    (0x08AD02, i_08ad02),
    (0x08AD08, i_08ad08),
    (0x08AD0E, i_08ad0e),
    (0x08AD14, i_08ad14),
    (0x08AD1A, i_08ad1a),
    (0x08AD20, i_08ad20),
    (0x08AD26, i_08ad26),
    (0x08AD2C, i_08ad2c),
    (0x08AD32, i_08ad32),
    (0x08AD38, i_08ad38),
    (0x08AD3E, i_08ad3e),
    (0x08AD44, i_08ad44),
    (0x08AD4A, i_08ad4a),
    (0x08AD50, i_08ad50),
    (0x08AD56, i_08ad56),
    (0x08AD5C, i_08ad5c),
    (0x08AD62, i_08ad62),
    (0x08AD68, i_08ad68),
    (0x08AD6E, i_08ad6e),
    (0x08AD74, i_08ad74),
    (0x08AD7A, i_08ad7a),
    (0x08AD80, i_08ad80),
    (0x08AD86, i_08ad86),
    (0x08AD8C, i_08ad8c),
    (0x08AD92, i_08ad92),
    (0x08AD98, i_08ad98),
    (0x08AD9E, i_08ad9e),
    (0x08ADA4, i_08ada4),
    (0x08ADAA, i_08adaa),
    (0x08ADB0, i_08adb0),
    (0x08ADB6, i_08adb6),
    (0x08ADBC, i_08adbc),
    (0x08ADC2, i_08adc2),
    (0x08ADC6, i_08adc6),
    (0x08ADCC, i_08adcc),
    (0x08ADD2, i_08add2),
    (0x08ADD8, i_08add8),
    (0x08ADDC, i_08addc),
    (0x08ADE2, i_08ade2),
    (0x08ADE8, i_08ade8),
    (0x08ADEE, i_08adee),
    (0x08ADF4, i_08adf4),
    (0x08ADFA, i_08adfa),
    (0x08AE00, i_08ae00),
    (0x08AE04, i_08ae04),
    (0x08AE0A, i_08ae0a),
    (0x08AE0C, i_08ae0c),
    (0x08AE12, i_08ae12),
    (0x08AE16, i_08ae16),
    (0x08AE1C, i_08ae1c),
    (0x08AE22, i_08ae22),
    (0x08AE28, i_08ae28),
    (0x08AE30, i_08ae30),
    (0x08AE34, i_08ae34),
    (0x08AE36, i_08ae36),
    (0x08AE38, i_08ae38),
    (0x08AE40, i_08ae40),
    (0x08AE44, i_08ae44),
    (0x08AE48, i_08ae48),
    (0x08AE4E, i_08ae4e),
    (0x08AE54, i_08ae54),
    (0x08AE56, i_08ae56),
    (0x08AE5C, i_08ae5c),
    (0x08AE60, i_08ae60),
    (0x08AE66, i_08ae66),
    (0x08AE68, i_08ae68),
    (0x08AE6E, i_08ae6e),
    (0x08AE74, i_08ae74),
    (0x08AE78, i_08ae78),
    (0x08AE7E, i_08ae7e),
    (0x08AE84, i_08ae84),
    (0x08AE8A, i_08ae8a),
    (0x08AE8E, i_08ae8e),
    (0x08AE92, i_08ae92),
    (0x08AE98, i_08ae98),
    (0x08AE9C, i_08ae9c),
    (0x08AEA2, i_08aea2),
    (0x08AEA8, i_08aea8),
    (0x08AEAE, i_08aeae),
    (0x08AEB4, i_08aeb4),
    (0x08AEBA, i_08aeba),
    (0x08AEC0, i_08aec0),
    (0x08AEC4, i_08aec4),
    (0x08AEC6, i_08aec6),
    (0x08AECA, i_08aeca),
    (0x08AED0, i_08aed0),
    (0x08AED4, i_08aed4),
    (0x08AEDA, i_08aeda),
    (0x08AEE0, i_08aee0),
    (0x08AEE4, i_08aee4),
    (0x08AEEA, i_08aeea),
    (0x08AEEE, i_08aeee),
    (0x08AEF4, i_08aef4),
    (0x08AEFA, i_08aefa),
    (0x08AF00, i_08af00),
    (0x08AF06, i_08af06),
    (0x08AF0A, i_08af0a),
    (0x08AF10, i_08af10),
    (0x08AF16, i_08af16),
    (0x08AF1C, i_08af1c),
    (0x08AF22, i_08af22),
    (0x08AF26, i_08af26),
    (0x08AF2C, i_08af2c),
    (0x08AF30, i_08af30),
    (0x08AF36, i_08af36),
    (0x08AF3A, i_08af3a),
    (0x08AF40, i_08af40),
    (0x08AF46, i_08af46),
    (0x08AF4C, i_08af4c),
    (0x08AF54, i_08af54),
    (0x08AF58, i_08af58),
    (0x08AF5E, i_08af5e),
    (0x08AF60, i_08af60),
    (0x08AF64, i_08af64),
    (0x08AF66, i_08af66),
    (0x08AF68, i_08af68),
    (0x08AF6E, i_08af6e),
    (0x08AF74, i_08af74),
    (0x08AF78, i_08af78),
    (0x08AF7E, i_08af7e),
    (0x08AF84, i_08af84),
    (0x08AF88, i_08af88),
    (0x08AF8A, i_08af8a),
    (0x08AF8E, i_08af8e),
    (0x08AF94, i_08af94),
    (0x08AF98, i_08af98),
    (0x08AF9E, i_08af9e),
    (0x08AFA4, i_08afa4),
    (0x08AFA8, i_08afa8),
    (0x08AFAE, i_08afae),
    (0x08AFB2, i_08afb2),
    (0x08AFB8, i_08afb8),
    (0x08AFBE, i_08afbe),
    (0x08AFC4, i_08afc4),
    (0x08AFCA, i_08afca),
    (0x08AFCE, i_08afce),
    (0x08AFD4, i_08afd4),
    (0x08AFDA, i_08afda),
    (0x08AFE0, i_08afe0),
    (0x08AFE6, i_08afe6),
    (0x08AFEA, i_08afea),
    (0x08AFF0, i_08aff0),
    (0x08AFF6, i_08aff6),
    (0x08AFFA, i_08affa),
    (0x08B000, i_08b000),
    (0x08B004, i_08b004),
    (0x08B00A, i_08b00a),
    (0x08B010, i_08b010),
    (0x08B016, i_08b016),
    (0x08B01C, i_08b01c),
    (0x08B020, i_08b020),
    (0x08B026, i_08b026),
    (0x08B02C, i_08b02c),
    (0x08B030, i_08b030),
    (0x08B032, i_08b032),
    (0x08B034, i_08b034),
    (0x08B03A, i_08b03a),
    (0x08B03C, i_08b03c),
    (0x08B040, i_08b040),
    (0x08B042, i_08b042),
    (0x08B046, i_08b046),
    (0x08B04C, i_08b04c),
    (0x08B050, i_08b050),
    (0x08B056, i_08b056),
    (0x08B05C, i_08b05c),
    (0x08B060, i_08b060),
    (0x08B066, i_08b066),
    (0x08B06C, i_08b06c),
    (0x08B072, i_08b072),
    (0x08B074, i_08b074),
    (0x08B07A, i_08b07a),
    (0x08B07C, i_08b07c),
    (0x08B080, i_08b080),
    (0x08B086, i_08b086),
    (0x08B08C, i_08b08c),
    (0x08B092, i_08b092),
    (0x08B098, i_08b098),
    (0x08B09E, i_08b09e),
    (0x08B0A4, i_08b0a4),
    (0x08B0AA, i_08b0aa),
    (0x08B0AC, i_08b0ac),
    (0x08B0B0, i_08b0b0),
    (0x08B0B6, i_08b0b6),
    (0x08B0B8, i_08b0b8),
    (0x08B0BC, i_08b0bc),
    (0x08B0C2, i_08b0c2),
    (0x08B0C6, i_08b0c6),
    (0x08B0CC, i_08b0cc),
    (0x08B0D0, i_08b0d0),
    (0x08B0D6, i_08b0d6),
    (0x08B0DA, i_08b0da),
    (0x08B0DC, i_08b0dc),
    (0x08B0E2, i_08b0e2),
    (0x08B0E8, i_08b0e8),
    (0x08B0EC, i_08b0ec),
    (0x08B0F2, i_08b0f2),
    (0x08B0F8, i_08b0f8),
    (0x08B0FE, i_08b0fe),
    (0x08B102, i_08b102),
    (0x08B106, i_08b106),
    (0x08B108, i_08b108),
    (0x08B10A, i_08b10a),
    (0x08B10E, i_08b10e),
    (0x08B114, i_08b114),
    (0x08B118, i_08b118),
    (0x08B11E, i_08b11e),
    (0x08B124, i_08b124),
    (0x08B12A, i_08b12a),
    (0x08B130, i_08b130),
    (0x08B136, i_08b136),
    (0x08B13A, i_08b13a),
    (0x08B140, i_08b140),
    (0x08B146, i_08b146),
    (0x08B14C, i_08b14c),
    (0x08B152, i_08b152),
    (0x08B158, i_08b158),
    (0x08B15C, i_08b15c),
    (0x08B162, i_08b162),
    (0x08B168, i_08b168),
    (0x08B16E, i_08b16e),
    (0x08B174, i_08b174),
    (0x08B17A, i_08b17a),
    (0x08B17E, i_08b17e),
    (0x08B184, i_08b184),
    (0x08B18A, i_08b18a),
    (0x08B190, i_08b190),
    (0x08B196, i_08b196),
    (0x08B19C, i_08b19c),
    (0x08B1A0, i_08b1a0),
    (0x08B1A6, i_08b1a6),
    (0x08B1AC, i_08b1ac),
    (0x08B1B2, i_08b1b2),
    (0x08B1B8, i_08b1b8),
    (0x08B1BE, i_08b1be),
    (0x08B1C2, i_08b1c2),
    (0x08B1C4, i_08b1c4),
    (0x08B1CA, i_08b1ca),
    (0x08B1D0, i_08b1d0),
    (0x08B1D4, i_08b1d4),
    (0x08B1DA, i_08b1da),
    (0x08B1E0, i_08b1e0),
    (0x08B1E6, i_08b1e6),
    (0x08B1EA, i_08b1ea),
    (0x08B1F0, i_08b1f0),
    (0x08B1F2, i_08b1f2),
    (0x08B1F8, i_08b1f8),
    (0x08B1FC, i_08b1fc),
    (0x08B202, i_08b202),
    (0x08B206, i_08b206),
    (0x08B208, i_08b208),
    (0x08B20C, i_08b20c),
    (0x08B212, i_08b212),
    (0x08B218, i_08b218),
    (0x08B21E, i_08b21e),
    (0x08B224, i_08b224),
    (0x08B22A, i_08b22a),
    (0x08B22E, i_08b22e),
    (0x08B230, i_08b230),
    (0x08B236, i_08b236),
    (0x08B23C, i_08b23c),
    (0x08B240, i_08b240),
    (0x08B246, i_08b246),
    (0x08B24C, i_08b24c),
    (0x08B250, i_08b250),
    (0x08B256, i_08b256),
    (0x08B258, i_08b258),
    (0x08B25E, i_08b25e),
    (0x08B262, i_08b262),
    (0x08B266, i_08b266),
    (0x08B26C, i_08b26c),
    (0x08B270, i_08b270),
    (0x08B274, i_08b274),
    (0x08B27A, i_08b27a),
    (0x08B280, i_08b280),
    (0x08B286, i_08b286),
    (0x08B28C, i_08b28c),
    (0x08B292, i_08b292),
    (0x08B298, i_08b298),
    (0x08B29E, i_08b29e),
    (0x08B2A2, i_08b2a2),
    (0x08B2A8, i_08b2a8),
    (0x08B2AE, i_08b2ae),
    (0x08B2B2, i_08b2b2),
    (0x08B2B8, i_08b2b8),
    (0x08B2BA, i_08b2ba),
    (0x08B2BE, i_08b2be),
    (0x08B2C4, i_08b2c4),
    (0x08B2C6, i_08b2c6),
    (0x08B2CA, i_08b2ca),
    (0x08B2D0, i_08b2d0),
    (0x08B2D4, i_08b2d4),
    (0x08B2DA, i_08b2da),
    (0x08B2DE, i_08b2de),
    (0x08B2E0, i_08b2e0),
    (0x08B2E6, i_08b2e6),
    (0x08B2EC, i_08b2ec),
    (0x08B2F2, i_08b2f2),
    (0x08B2F6, i_08b2f6),
    (0x08B2FA, i_08b2fa),
    (0x08B2FE, i_08b2fe),
    (0x08B304, i_08b304),
    (0x08B308, i_08b308),
    (0x08B30E, i_08b30e),
    (0x08B314, i_08b314),
    (0x08B31A, i_08b31a),
    (0x08B31E, i_08b31e),
    (0x08B324, i_08b324),
    (0x08B328, i_08b328),
    (0x08B32E, i_08b32e),
    (0x08B334, i_08b334),
    (0x08B33A, i_08b33a),
    (0x08B340, i_08b340),
    (0x08B344, i_08b344),
    (0x08B34A, i_08b34a),
    (0x08B34C, i_08b34c),
    (0x08B352, i_08b352),
    (0x08B358, i_08b358),
    (0x08B35E, i_08b35e),
    (0x08B362, i_08b362),
    (0x08B366, i_08b366),
    (0x08B36C, i_08b36c),
    (0x08B372, i_08b372),
    (0x08B378, i_08b378),
    (0x08B37C, i_08b37c),
    (0x08B380, i_08b380),
    (0x08B386, i_08b386),
    (0x08B38C, i_08b38c),
    (0x08B392, i_08b392),
    (0x08B396, i_08b396),
    (0x08B39A, i_08b39a),
    (0x08B3A0, i_08b3a0),
    (0x08B3A6, i_08b3a6),
    (0x08B3AC, i_08b3ac),
    (0x08B3B0, i_08b3b0),
    (0x08B3B4, i_08b3b4),
    (0x08B3BA, i_08b3ba),
    (0x08B3C0, i_08b3c0),
    (0x08B3C6, i_08b3c6),
    (0x08B3CC, i_08b3cc),
    (0x08B3D2, i_08b3d2),
    (0x08B3D8, i_08b3d8),
    (0x08B3DA, i_08b3da),
    (0x08B3E0, i_08b3e0),
    (0x08B3E4, i_08b3e4),
    (0x08B3EA, i_08b3ea),
    (0x08B3EE, i_08b3ee),
    (0x08B3F2, i_08b3f2),
    (0x08B3F6, i_08b3f6),
    (0x08B3FC, i_08b3fc),
    (0x08B400, i_08b400),
    (0x08B404, i_08b404),
    (0x08B40A, i_08b40a),
    (0x08B40E, i_08b40e),
    (0x08B414, i_08b414),
    (0x08B418, i_08b418),
    (0x08B41C, i_08b41c),
    (0x08B420, i_08b420),
    (0x08B426, i_08b426),
    (0x08B42A, i_08b42a),
    (0x08B42E, i_08b42e),
    (0x08B434, i_08b434),
    (0x08B438, i_08b438),
    (0x08B43A, i_08b43a),
    (0x08B440, i_08b440),
    (0x08B446, i_08b446),
    (0x08B44A, i_08b44a),
    (0x08B450, i_08b450),
    (0x08B454, i_08b454),
    (0x08B45A, i_08b45a),
    (0x08B45C, i_08b45c),
    (0x08B460, i_08b460),
    (0x08B464, i_08b464),
    (0x08B468, i_08b468),
    (0x08B46C, i_08b46c),
    (0x08B472, i_08b472),
    (0x08B476, i_08b476),
    (0x08B47A, i_08b47a),
    (0x08B47E, i_08b47e),
    (0x08B482, i_08b482),
    (0x08B488, i_08b488),
    (0x08B48E, i_08b48e),
    (0x08B490, i_08b490),
    (0x08B494, i_08b494),
    (0x08B498, i_08b498),
    (0x08B49C, i_08b49c),
    (0x08B4A0, i_08b4a0),
    (0x08B4A6, i_08b4a6),
    (0x08B4AA, i_08b4aa),
    (0x08B4AE, i_08b4ae),
    (0x08B4B2, i_08b4b2),
    (0x08B4B6, i_08b4b6),
    (0x08B4BC, i_08b4bc),
    (0x08B4C2, i_08b4c2),
    (0x08B4C4, i_08b4c4),
    (0x08B4C8, i_08b4c8),
    (0x08B4CC, i_08b4cc),
    (0x08B4D0, i_08b4d0),
    (0x08B4D4, i_08b4d4),
    (0x08B4DA, i_08b4da),
    (0x08B4DE, i_08b4de),
    (0x08B4E2, i_08b4e2),
    (0x08B4E6, i_08b4e6),
    (0x08B4EA, i_08b4ea),
    (0x08B4F0, i_08b4f0),
    (0x08B4F4, i_08b4f4),
    (0x08B4F8, i_08b4f8),
    (0x08B4FC, i_08b4fc),
    (0x08B500, i_08b500),
    (0x08B506, i_08b506),
    (0x08B50C, i_08b50c),
    (0x08B50E, i_08b50e),
    (0x08B512, i_08b512),
    (0x08B516, i_08b516),
    (0x08B51A, i_08b51a),
    (0x08B51E, i_08b51e),
    (0x08B524, i_08b524),
    (0x08B528, i_08b528),
    (0x08B52C, i_08b52c),
    (0x08B530, i_08b530),
    (0x08B534, i_08b534),
    (0x08B53A, i_08b53a),
    (0x08B53E, i_08b53e),
    (0x08B542, i_08b542),
    (0x08B546, i_08b546),
    (0x08B54A, i_08b54a),
    (0x08B550, i_08b550),
    (0x08B556, i_08b556),
    (0x08B558, i_08b558),
    (0x08B55E, i_08b55e),
    (0x08B562, i_08b562),
    (0x08B568, i_08b568),
    (0x08B56E, i_08b56e),
    (0x08B572, i_08b572),
    (0x08B578, i_08b578),
    (0x08B57E, i_08b57e),
    (0x08B582, i_08b582),
    (0x08B586, i_08b586),
    (0x08B58C, i_08b58c),
    (0x08B58E, i_08b58e),
    (0x08B590, i_08b590),
    (0x08B592, i_08b592),
    (0x08B596, i_08b596),
    (0x08B59A, i_08b59a),
    (0x08B59C, i_08b59c),
    (0x08B59E, i_08b59e),
    (0x08B5A2, i_08b5a2),
    (0x08B5A8, i_08b5a8),
    (0x08B5AE, i_08b5ae),
    (0x08B5B4, i_08b5b4),
    (0x08B5B6, i_08b5b6),
    (0x08B5BA, i_08b5ba),
    (0x08B5C0, i_08b5c0),
    (0x08B5C6, i_08b5c6),
    (0x08B5C8, i_08b5c8),
    (0x08B5CC, i_08b5cc),
    (0x08B5D0, i_08b5d0),
    (0x08B5D4, i_08b5d4),
    (0x08B5D8, i_08b5d8),
    (0x08B5DC, i_08b5dc),
    (0x08B5DE, i_08b5de),
    (0x08B5E2, i_08b5e2),
    (0x08B5E6, i_08b5e6),
    (0x08B5EC, i_08b5ec),
    (0x08B5F0, i_08b5f0),
    (0x08B5F6, i_08b5f6),
    (0x08B5FA, i_08b5fa),
    (0x08B600, i_08b600),
    (0x08B604, i_08b604),
    (0x08B60A, i_08b60a),
    (0x08B60E, i_08b60e),
    (0x08B614, i_08b614),
    (0x08B61A, i_08b61a),
    (0x08B61E, i_08b61e),
    (0x08B620, i_08b620),
    (0x08B624, i_08b624),
    (0x08B626, i_08b626),
    (0x08B62C, i_08b62c),
    (0x08B630, i_08b630),
    (0x08B634, i_08b634),
    (0x08B63A, i_08b63a),
    (0x08B63E, i_08b63e),
    (0x08B642, i_08b642),
    (0x08B646, i_08b646),
    (0x08B64C, i_08b64c),
    (0x08B652, i_08b652),
    (0x08B658, i_08b658),
    (0x08B65E, i_08b65e),
    (0x08B662, i_08b662),
    (0x08B668, i_08b668),
    (0x08B66E, i_08b66e),
    (0x08B674, i_08b674),
    (0x08B67A, i_08b67a),
    (0x08B67E, i_08b67e),
    (0x08B680, i_08b680),
    (0x08B684, i_08b684),
    (0x08B688, i_08b688),
    (0x08B68A, i_08b68a),
    (0x08B68E, i_08b68e),
    (0x08B690, i_08b690),
    (0x08B696, i_08b696),
    (0x08B69A, i_08b69a),
    (0x08B69E, i_08b69e),
    (0x08B6A2, i_08b6a2),
    (0x08B6A6, i_08b6a6),
    (0x08B6A8, i_08b6a8),
    (0x08B6AE, i_08b6ae),
    (0x08B6B4, i_08b6b4),
    (0x08B6B8, i_08b6b8),
    (0x08B6BC, i_08b6bc),
    (0x08B6C2, i_08b6c2),
    (0x08B6C4, i_08b6c4),
    (0x08B6C8, i_08b6c8),
    (0x08B6CC, i_08b6cc),
    (0x08B6D0, i_08b6d0),
    (0x08B6D4, i_08b6d4),
    (0x08B6D8, i_08b6d8),
    (0x08B6DC, i_08b6dc),
    (0x08B6E2, i_08b6e2),
    (0x08B6E8, i_08b6e8),
    (0x08B6EE, i_08b6ee),
    (0x08B6F2, i_08b6f2),
    (0x08B6F6, i_08b6f6),
    (0x08B6FC, i_08b6fc),
    (0x08B700, i_08b700),
    (0x08B704, i_08b704),
    (0x08B708, i_08b708),
    (0x08B70C, i_08b70c),
    (0x08B712, i_08b712),
    (0x08B716, i_08b716),
    (0x08B718, i_08b718),
    (0x08B71C, i_08b71c),
    (0x08B720, i_08b720),
    (0x08B724, i_08b724),
    (0x08B726, i_08b726),
    (0x08B728, i_08b728),
    (0x08B72C, i_08b72c),
    (0x08B730, i_08b730),
    (0x08B732, i_08b732),
    (0x08B738, i_08b738),
    (0x08B73C, i_08b73c),
    (0x08B740, i_08b740),
    (0x08B742, i_08b742),
    (0x08B744, i_08b744),
    (0x08B748, i_08b748),
    (0x08B74E, i_08b74e),
    (0x08B754, i_08b754),
    (0x08B75A, i_08b75a),
    (0x08B75E, i_08b75e),
    (0x08B762, i_08b762),
    (0x08B768, i_08b768),
    (0x08B76C, i_08b76c),
    (0x08B770, i_08b770),
    (0x08B776, i_08b776),
    (0x08B778, i_08b778),
    (0x08B77C, i_08b77c),
    (0x08B780, i_08b780),
    (0x08B784, i_08b784),
    (0x08B788, i_08b788),
    (0x08B78C, i_08b78c),
    (0x08B790, i_08b790),
    (0x08B794, i_08b794),
    (0x08B796, i_08b796),
    (0x08B79A, i_08b79a),
    (0x08B79E, i_08b79e),
    (0x08B7A2, i_08b7a2),
    (0x08B7A6, i_08b7a6),
    (0x08B7A8, i_08b7a8),
    (0x08B7AC, i_08b7ac),
    (0x08B7AE, i_08b7ae),
    (0x08B7B0, i_08b7b0),
    (0x08B7B4, i_08b7b4),
    (0x08B7B8, i_08b7b8),
    (0x08B7BC, i_08b7bc),
    (0x08B7C0, i_08b7c0),
    (0x08B7C6, i_08b7c6),
    (0x08B7CA, i_08b7ca),
    (0x08B7CE, i_08b7ce),
    (0x08B7D2, i_08b7d2),
    (0x08B7D8, i_08b7d8),
    (0x08B7DC, i_08b7dc),
    (0x08B7E0, i_08b7e0),
    (0x08B7E4, i_08b7e4),
    (0x08B7E6, i_08b7e6),
    (0x08B7EA, i_08b7ea),
    (0x08B7EE, i_08b7ee),
    (0x08B7F0, i_08b7f0),
    (0x08B7F4, i_08b7f4),
    (0x08B7F8, i_08b7f8),
    (0x08B7FC, i_08b7fc),
    (0x08B800, i_08b800),
    (0x08B806, i_08b806),
    (0x08B80A, i_08b80a),
    (0x08B80E, i_08b80e),
    (0x08B812, i_08b812),
    (0x08B816, i_08b816),
    (0x08B81A, i_08b81a),
    (0x08B81E, i_08b81e),
    (0x08B820, i_08b820),
    (0x08B824, i_08b824),
    (0x08B826, i_08b826),
    (0x08B82A, i_08b82a),
    (0x08B82C, i_08b82c),
    (0x08B832, i_08b832),
    (0x08B838, i_08b838),
    (0x08B83A, i_08b83a),
    (0x08B83E, i_08b83e),
    (0x08B842, i_08b842),
    (0x08B848, i_08b848),
    (0x08B84E, i_08b84e),
    (0x08B854, i_08b854),
    (0x08B85A, i_08b85a),
    (0x08B860, i_08b860),
    (0x08B862, i_08b862),
    (0x08B868, i_08b868),
    (0x08B86C, i_08b86c),
    (0x08B870, i_08b870),
    (0x08B874, i_08b874),
    (0x08B87A, i_08b87a),
    (0x08B87E, i_08b87e),
    (0x08B882, i_08b882),
    (0x08B884, i_08b884),
    (0x08B888, i_08b888),
    (0x08B88C, i_08b88c),
    (0x08B890, i_08b890),
    (0x08B894, i_08b894),
    (0x08B898, i_08b898),
    (0x08B89E, i_08b89e),
    (0x08B8A2, i_08b8a2),
    (0x08B8A6, i_08b8a6),
    (0x08B8AA, i_08b8aa),
    (0x08B8AE, i_08b8ae),
    (0x08B8B0, i_08b8b0),
    (0x08B8B4, i_08b8b4),
    (0x08B8B8, i_08b8b8),
    (0x08B8BC, i_08b8bc),
    (0x08B8C0, i_08b8c0),
    (0x08B8C4, i_08b8c4),
    (0x08B8CA, i_08b8ca),
    (0x08B8CE, i_08b8ce),
    (0x08B8D2, i_08b8d2),
    (0x08B8D6, i_08b8d6),
    (0x08B8DA, i_08b8da),
    (0x08B8DE, i_08b8de),
    (0x08B8E4, i_08b8e4),
    (0x08B8E8, i_08b8e8),
    (0x08B8EC, i_08b8ec),
    (0x08B8F2, i_08b8f2),
    (0x08B8F4, i_08b8f4),
    (0x08B8F6, i_08b8f6),
    (0x08B8FA, i_08b8fa),
    (0x08B8FC, i_08b8fc),
    (0x08B902, i_08b902),
    (0x08B906, i_08b906),
    (0x08B90C, i_08b90c),
    (0x08B90E, i_08b90e),
    (0x08B910, i_08b910),
    (0x08B914, i_08b914),
    (0x08B916, i_08b916),
    (0x08B91C, i_08b91c),
    (0x08B920, i_08b920),
    (0x08B926, i_08b926),
    (0x08B928, i_08b928),
    (0x08B92C, i_08b92c),
    (0x08B930, i_08b930),
    (0x08B934, i_08b934),
    (0x08B938, i_08b938),
    (0x08B93C, i_08b93c),
    (0x08B93E, i_08b93e),
    (0x08B942, i_08b942),
    (0x08B9A2, i_08b9a2),
    (0x08B9A6, i_08b9a6),
    (0x08B9AA, i_08b9aa),
    (0x08B9AE, i_08b9ae),
    (0x08B9B2, i_08b9b2),
    (0x08B9B6, i_08b9b6),
    (0x08B9BA, i_08b9ba),
    (0x08B9C0, i_08b9c0),
    (0x08B9C6, i_08b9c6),
    (0x08B9CC, i_08b9cc),
    (0x08B9D2, i_08b9d2),
    (0x08B9D8, i_08b9d8),
    (0x08B9DC, i_08b9dc),
    (0x08B9DE, i_08b9de),
    (0x08B9E4, i_08b9e4),
    (0x08B9EA, i_08b9ea),
    (0x08B9EE, i_08b9ee),
    (0x08B9F2, i_08b9f2),
    (0x08B9F4, i_08b9f4),
    (0x08B9FA, i_08b9fa),
    (0x08B9FE, i_08b9fe),
    (0x08BA04, i_08ba04),
    (0x08BA0A, i_08ba0a),
    (0x08BA0C, i_08ba0c),
    (0x08BA12, i_08ba12),
    (0x08BA18, i_08ba18),
    (0x08BA1E, i_08ba1e),
    (0x08BA22, i_08ba22),
    (0x08BA24, i_08ba24),
    (0x08BA2A, i_08ba2a),
    (0x08BA30, i_08ba30),
    (0x08BA34, i_08ba34),
    (0x08BA38, i_08ba38),
    (0x08BA3A, i_08ba3a),
    (0x08BA40, i_08ba40),
    (0x08BA44, i_08ba44),
    (0x08BA4A, i_08ba4a),
    (0x08BA50, i_08ba50),
    (0x08BA52, i_08ba52),
    (0x08BA58, i_08ba58),
    (0x08BA5E, i_08ba5e),
    (0x08BA64, i_08ba64),
    (0x08BA6A, i_08ba6a),
    (0x08BA6E, i_08ba6e),
    (0x08BA70, i_08ba70),
    (0x08BA76, i_08ba76),
    (0x08BA7C, i_08ba7c),
    (0x08BA80, i_08ba80),
    (0x08BA84, i_08ba84),
    (0x08BA86, i_08ba86),
    (0x08BA8C, i_08ba8c),
    (0x08BA90, i_08ba90),
    (0x08BA96, i_08ba96),
    (0x08BA9C, i_08ba9c),
    (0x08BABE, i_08babe),
    (0x08BAC4, i_08bac4),
    (0x08BAC8, i_08bac8),
    (0x08BACC, i_08bacc),
    (0x08BAD2, i_08bad2),
    (0x08BAD8, i_08bad8),
    (0x08BADA, i_08bada),
    (0x08BAE0, i_08bae0),
    (0x08BAE4, i_08bae4),
    (0x08BAE8, i_08bae8),
    (0x08BAEE, i_08baee),
    (0x08BAF4, i_08baf4),
    (0x08BAF6, i_08baf6),
    (0x08BAFA, i_08bafa),
    (0x08BB00, i_08bb00),
    (0x08BB06, i_08bb06),
    (0x08BB0C, i_08bb0c),
    (0x08BB10, i_08bb10),
    (0x08BB12, i_08bb12),
    (0x08BB18, i_08bb18),
    (0x08BB1E, i_08bb1e),
    (0x08BB22, i_08bb22),
    (0x08BB26, i_08bb26),
    (0x08BB28, i_08bb28),
    (0x08BB2A, i_08bb2a),
    (0x08BB30, i_08bb30),
    (0x08BB32, i_08bb32),
    (0x08BB34, i_08bb34),
    (0x08BB38, i_08bb38),
    (0x08BB3E, i_08bb3e),
    (0x08BB42, i_08bb42),
    (0x08BB46, i_08bb46),
    (0x08BB4A, i_08bb4a),
    (0x08BB50, i_08bb50),
    (0x08BB56, i_08bb56),
    (0x08BB5C, i_08bb5c),
    (0x08BB5E, i_08bb5e),
    (0x08BB62, i_08bb62),
    (0x08BB68, i_08bb68),
    (0x08BB6C, i_08bb6c),
    (0x08BB70, i_08bb70),
    (0x08BB76, i_08bb76),
    (0x08BB7C, i_08bb7c),
    (0x08BB82, i_08bb82),
    (0x08BB84, i_08bb84),
    (0x08BB8A, i_08bb8a),
    (0x08BC0C, i_08bc0c),
    (0x08BC10, i_08bc10),
    (0x08BC16, i_08bc16),
    (0x08BC1C, i_08bc1c),
    (0x08BC22, i_08bc22),
    (0x08BC28, i_08bc28),
    (0x08BC2C, i_08bc2c),
    (0x08BC30, i_08bc30),
    (0x08BC34, i_08bc34),
    (0x08BC38, i_08bc38),
    (0x08BC3E, i_08bc3e),
    (0x08BC42, i_08bc42),
    (0x08BC44, i_08bc44),
    (0x08BC4A, i_08bc4a),
    (0x08BC50, i_08bc50),
    (0x08BC56, i_08bc56),
    (0x08BC58, i_08bc58),
    (0x08BC5C, i_08bc5c),
    (0x08BC60, i_08bc60),
    (0x08BC64, i_08bc64),
    (0x08BC68, i_08bc68),
    (0x08BC6C, i_08bc6c),
    (0x08BC6E, i_08bc6e),
    (0x08BC72, i_08bc72),
    (0x08BC74, i_08bc74),
    (0x08BC7A, i_08bc7a),
    (0x08BC7E, i_08bc7e),
    (0x08BC82, i_08bc82),
    (0x08BC88, i_08bc88),
    (0x08BC8C, i_08bc8c),
    (0x08BC90, i_08bc90),
    (0x08BC94, i_08bc94),
    (0x08BC98, i_08bc98),
    (0x08BC9E, i_08bc9e),
    (0x08BCA2, i_08bca2),
    (0x08BCA8, i_08bca8),
    (0x08BCAE, i_08bcae),
    (0x08BCB4, i_08bcb4),
    (0x08BCB8, i_08bcb8),
    (0x08BCBC, i_08bcbc),
    (0x08BCC2, i_08bcc2),
    (0x08BCC8, i_08bcc8),
    (0x08BCCE, i_08bcce),
    (0x08BCD2, i_08bcd2),
    (0x08BCD8, i_08bcd8),
    (0x08BCDC, i_08bcdc),
    (0x08BCE0, i_08bce0),
    (0x08BCE4, i_08bce4),
    (0x08BCE6, i_08bce6),
    (0x08BCEC, i_08bcec),
    (0x08BCF0, i_08bcf0),
    (0x08BCF4, i_08bcf4),
    (0x08BCFA, i_08bcfa),
    (0x08BCFE, i_08bcfe),
    (0x08BD02, i_08bd02),
    (0x08BD06, i_08bd06),
    (0x08BD0A, i_08bd0a),
    (0x08BD10, i_08bd10),
    (0x08BD14, i_08bd14),
    (0x08BD1A, i_08bd1a),
    (0x08BD22, i_08bd22),
    (0x08BD26, i_08bd26),
    (0x08BD2A, i_08bd2a),
    (0x08BD30, i_08bd30),
    (0x08BD36, i_08bd36),
    (0x08BD3A, i_08bd3a),
    (0x08BD3E, i_08bd3e),
    (0x08BD42, i_08bd42),
    (0x08BD46, i_08bd46),
    (0x08BD4A, i_08bd4a),
    (0x08BD50, i_08bd50),
    (0x08BD54, i_08bd54),
    (0x08BD5A, i_08bd5a),
    (0x08BD5E, i_08bd5e),
    (0x08BD66, i_08bd66),
    (0x08BD6A, i_08bd6a),
    (0x08BD70, i_08bd70),
    (0x08BD74, i_08bd74),
    (0x08BD78, i_08bd78),
    (0x08BD7E, i_08bd7e),
    (0x08BD82, i_08bd82),
    (0x08BD88, i_08bd88),
    (0x08BD8C, i_08bd8c),
    (0x08BD92, i_08bd92),
    (0x08BD98, i_08bd98),
    (0x08BD9E, i_08bd9e),
    (0x08BDA4, i_08bda4),
    (0x08BDAA, i_08bdaa),
    (0x08BDAE, i_08bdae),
    (0x08BDB4, i_08bdb4),
    (0x08BDB8, i_08bdb8),
    (0x08BDBC, i_08bdbc),
    (0x08BDC0, i_08bdc0),
    (0x08BDC4, i_08bdc4),
    (0x08BDCC, i_08bdcc),
    (0x08BDD0, i_08bdd0),
    (0x08BDD4, i_08bdd4),
    (0x08BDD8, i_08bdd8),
    (0x08BDDC, i_08bddc),
    (0x08BDE0, i_08bde0),
    (0x08BDE4, i_08bde4),
    (0x08BDE8, i_08bde8),
    (0x08BDEC, i_08bdec),
    (0x08BDF0, i_08bdf0),
    (0x08BDF4, i_08bdf4),
    (0x08BDF8, i_08bdf8),
    (0x08BDFC, i_08bdfc),
    (0x08BE00, i_08be00),
    (0x08BE04, i_08be04),
    (0x08BE08, i_08be08),
    (0x08BE0C, i_08be0c),
    (0x08BE12, i_08be12),
    (0x08BE16, i_08be16),
    (0x08BE1A, i_08be1a),
    (0x08BE1E, i_08be1e),
    (0x08BE22, i_08be22),
    (0x08BE26, i_08be26),
    (0x08BE28, i_08be28),
    (0x08BE2E, i_08be2e),
    (0x08BE32, i_08be32),
    (0x08BE36, i_08be36),
    (0x08BE3C, i_08be3c),
    (0x08BE40, i_08be40),
    (0x08BE44, i_08be44),
    (0x08BE48, i_08be48),
    (0x08BE4C, i_08be4c),
    (0x08BE52, i_08be52),
    (0x08BE56, i_08be56),
    (0x08BE5C, i_08be5c),
    (0x08BE64, i_08be64),
    (0x08BE68, i_08be68),
    (0x08BE6C, i_08be6c),
    (0x08BE72, i_08be72),
    (0x08BE78, i_08be78),
    (0x08BE7C, i_08be7c),
    (0x08BE80, i_08be80),
    (0x08BE84, i_08be84),
    (0x08BE88, i_08be88),
    (0x08BE8C, i_08be8c),
    (0x08BE92, i_08be92),
    (0x08BE96, i_08be96),
    (0x08BE9C, i_08be9c),
    (0x08BEA0, i_08bea0),
    (0x08BEA4, i_08bea4),
    (0x08BEAA, i_08beaa),
    (0x08BEB0, i_08beb0),
    (0x08BEB6, i_08beb6),
    (0x08BEBA, i_08beba),
    (0x08BEBE, i_08bebe),
    (0x08BEC4, i_08bec4),
    (0x08BEC8, i_08bec8),
    (0x08BECC, i_08becc),
    (0x08BED2, i_08bed2),
    (0x08BED6, i_08bed6),
    (0x08BEDC, i_08bedc),
    (0x08BEE0, i_08bee0),
    (0x08BEE6, i_08bee6),
    (0x08BEEC, i_08beec),
    (0x08BEF2, i_08bef2),
    (0x08BEF8, i_08bef8),
    (0x08BEFE, i_08befe),
    (0x08BF02, i_08bf02),
    (0x08BF08, i_08bf08),
    (0x08BF0C, i_08bf0c),
    (0x08BF10, i_08bf10),
    (0x08BF14, i_08bf14),
    (0x08BF18, i_08bf18),
    (0x08BF1E, i_08bf1e),
    (0x08BF26, i_08bf26),
    (0x08BF2A, i_08bf2a),
    (0x08BF2E, i_08bf2e),
    (0x08BF36, i_08bf36),
    (0x08BF3C, i_08bf3c),
    (0x08BF40, i_08bf40),
    (0x08BF46, i_08bf46),
    (0x08BF4A, i_08bf4a),
    (0x08BF52, i_08bf52),
    (0x08BF56, i_08bf56),
    (0x08BF5C, i_08bf5c),
    (0x08BF60, i_08bf60),
    (0x08BF66, i_08bf66),
    (0x08BF6A, i_08bf6a),
    (0x08BF70, i_08bf70),
    (0x08BF74, i_08bf74),
    (0x08BF7C, i_08bf7c),
    (0x08BF80, i_08bf80),
    (0x08BF84, i_08bf84),
    (0x08BF88, i_08bf88),
    (0x08BF8C, i_08bf8c),
    (0x08BF90, i_08bf90),
    (0x08BF94, i_08bf94),
    (0x08BF96, i_08bf96),
    (0x08BF9C, i_08bf9c),
    (0x08BFA0, i_08bfa0),
    (0x08BFA4, i_08bfa4),
    (0x08BFAA, i_08bfaa),
    (0x08BFAE, i_08bfae),
    (0x08BFB2, i_08bfb2),
    (0x08BFB6, i_08bfb6),
    (0x08BFBA, i_08bfba),
    (0x08BFC0, i_08bfc0),
    (0x08BFC4, i_08bfc4),
    (0x08BFCA, i_08bfca),
    (0x08BFD0, i_08bfd0),
    (0x08BFD6, i_08bfd6),
    (0x08BFDA, i_08bfda),
    (0x08BFDE, i_08bfde),
    (0x08BFE4, i_08bfe4),
    (0x08BFEA, i_08bfea),
    (0x08BFF0, i_08bff0),
    (0x08BFF4, i_08bff4),
    (0x08BFFA, i_08bffa),
    (0x08BFFE, i_08bffe),
    (0x08C002, i_08c002),
    (0x08C006, i_08c006),
    (0x08C008, i_08c008),
    (0x08C00E, i_08c00e),
    (0x08C012, i_08c012),
    (0x08C018, i_08c018),
    (0x08C01C, i_08c01c),
    (0x08C022, i_08c022),
    (0x08C028, i_08c028),
    (0x08C02E, i_08c02e),
    (0x08C034, i_08c034),
    (0x08C03A, i_08c03a),
    (0x08C03E, i_08c03e),
    (0x08C044, i_08c044),
    (0x08C04A, i_08c04a),
    (0x08C04E, i_08c04e),
    (0x08C054, i_08c054),
    (0x08C05A, i_08c05a),
    (0x08C060, i_08c060),
    (0x08C068, i_08c068),
    (0x08C06E, i_08c06e),
    (0x08C072, i_08c072),
    (0x08C07A, i_08c07a),
    (0x08C07C, i_08c07c),
    (0x08C082, i_08c082),
    (0x08C086, i_08c086),
    (0x08C08C, i_08c08c),
    (0x08C092, i_08c092),
    (0x08C098, i_08c098),
    (0x08C09E, i_08c09e),
    (0x08C0A4, i_08c0a4),
    (0x08C0AA, i_08c0aa),
    (0x08C0B0, i_08c0b0),
    (0x08C0B6, i_08c0b6),
    (0x08C0BA, i_08c0ba),
    (0x08C0C0, i_08c0c0),
    (0x08C0C4, i_08c0c4),
    (0x08C0CA, i_08c0ca),
    (0x08C0CE, i_08c0ce),
    (0x08C0D4, i_08c0d4),
    (0x08C0D8, i_08c0d8),
    (0x08C0DE, i_08c0de),
    (0x08C0E2, i_08c0e2),
    (0x08C0E8, i_08c0e8),
    (0x08C0EC, i_08c0ec),
    (0x08C0F2, i_08c0f2),
    (0x08C0F8, i_08c0f8),
    (0x08C0FE, i_08c0fe),
    (0x08C104, i_08c104),
    (0x08C108, i_08c108),
    (0x08C10A, i_08c10a),
    (0x08C110, i_08c110),
    (0x08C114, i_08c114),
    (0x08C116, i_08c116),
    (0x08C11A, i_08c11a),
    (0x08C11E, i_08c11e),
    (0x08C122, i_08c122),
    (0x08C126, i_08c126),
    (0x08C12A, i_08c12a),
    (0x08C12E, i_08c12e),
    (0x08C134, i_08c134),
    (0x08C138, i_08c138),
    (0x08C13E, i_08c13e),
    (0x08C144, i_08c144),
    (0x08C148, i_08c148),
    (0x08C14A, i_08c14a),
    (0x08C150, i_08c150),
    (0x08C156, i_08c156),
    (0x08C15C, i_08c15c),
    (0x08C15E, i_08c15e),
    (0x08C164, i_08c164),
    (0x08C168, i_08c168),
    (0x08C16A, i_08c16a),
    (0x08C16E, i_08c16e),
    (0x08C172, i_08c172),
    (0x08C176, i_08c176),
    (0x08C17A, i_08c17a),
    (0x08C17E, i_08c17e),
    (0x08C182, i_08c182),
    (0x08C188, i_08c188),
    (0x08C18C, i_08c18c),
    (0x08C192, i_08c192),
    (0x08C196, i_08c196),
    (0x08C198, i_08c198),
    (0x08C19E, i_08c19e),
    (0x08C1A4, i_08c1a4),
    (0x08C1AA, i_08c1aa),
    (0x08C1AC, i_08c1ac),
    (0x08C1B0, i_08c1b0),
    (0x08C1B4, i_08c1b4),
    (0x08C1B8, i_08c1b8),
    (0x08C1BC, i_08c1bc),
    (0x08C1C0, i_08c1c0),
    (0x08C1C4, i_08c1c4),
    (0x08C1CA, i_08c1ca),
    (0x08C1D0, i_08c1d0),
    (0x08C1D4, i_08c1d4),
    (0x08C1D6, i_08c1d6),
    (0x08C1DC, i_08c1dc),
    (0x08C1E2, i_08c1e2),
    (0x08C1E8, i_08c1e8),
    (0x08C1EA, i_08c1ea),
    (0x08C1F0, i_08c1f0),
    (0x08C1F4, i_08c1f4),
    (0x08C1F6, i_08c1f6),
    (0x08C1FA, i_08c1fa),
    (0x08C1FC, i_08c1fc),
    (0x08C200, i_08c200),
    (0x08C202, i_08c202),
    (0x08C206, i_08c206),
    (0x08C20A, i_08c20a),
    (0x08C210, i_08c210),
    (0x08C214, i_08c214),
    (0x08C21A, i_08c21a),
    (0x08C220, i_08c220),
    (0x08C224, i_08c224),
    (0x08C226, i_08c226),
    (0x08C22C, i_08c22c),
    (0x08C232, i_08c232),
    (0x08C238, i_08c238),
    (0x08C23A, i_08c23a),
    (0x08C240, i_08c240),
    (0x08C244, i_08c244),
    (0x08C246, i_08c246),
    (0x08C24A, i_08c24a),
    (0x08C24E, i_08c24e),
    (0x08C252, i_08c252),
    (0x08C258, i_08c258),
    (0x08C25C, i_08c25c),
    (0x08C260, i_08c260),
    (0x08C266, i_08c266),
    (0x08C26A, i_08c26a),
    (0x08C270, i_08c270),
    (0x08C276, i_08c276),
    (0x08C27C, i_08c27c),
    (0x08C280, i_08c280),
    (0x08C282, i_08c282),
    (0x08C288, i_08c288),
    (0x08C28E, i_08c28e),
    (0x08C294, i_08c294),
    (0x08C296, i_08c296),
    (0x08C29A, i_08c29a),
    (0x08C2A0, i_08c2a0),
    (0x08C2A4, i_08c2a4),
    (0x08C2AA, i_08c2aa),
    (0x08C2B0, i_08c2b0),
    (0x08C2B6, i_08c2b6),
    (0x08C2B8, i_08c2b8),
    (0x08C2BE, i_08c2be),
    (0x08C2C4, i_08c2c4),
    (0x08C2C8, i_08c2c8),
    (0x08C2CE, i_08c2ce),
    (0x08C2D4, i_08c2d4),
    (0x08C2DA, i_08c2da),
    (0x08C2E0, i_08c2e0),
    (0x08C2E6, i_08c2e6),
    (0x08C2EA, i_08c2ea),
    (0x08C2EE, i_08c2ee),
    (0x08C2F0, i_08c2f0),
    (0x08C2F4, i_08c2f4),
    (0x08C2F8, i_08c2f8),
    (0x08C2FE, i_08c2fe),
    (0x08C304, i_08c304),
    (0x08C306, i_08c306),
    (0x08C30A, i_08c30a),
    (0x08C310, i_08c310),
    (0x08C314, i_08c314),
    (0x08C31A, i_08c31a),
    (0x08C320, i_08c320),
    (0x08C322, i_08c322),
    (0x08C328, i_08c328),
    (0x08C32E, i_08c32e),
    (0x08C332, i_08c332),
    (0x08C338, i_08c338),
    (0x08C33E, i_08c33e),
    (0x08C344, i_08c344),
    (0x08C34A, i_08c34a),
    (0x08C350, i_08c350),
    (0x08C354, i_08c354),
    (0x08C358, i_08c358),
    (0x08C35E, i_08c35e),
    (0x08C364, i_08c364),
    (0x08C36A, i_08c36a),
    (0x08C370, i_08c370),
    (0x08C374, i_08c374),
    (0x08C376, i_08c376),
    (0x08C37C, i_08c37c),
    (0x08C37E, i_08c37e),
    (0x08C384, i_08c384),
    (0x08C38A, i_08c38a),
    (0x08C38E, i_08c38e),
    (0x08C394, i_08c394),
    (0x08C39A, i_08c39a),
    (0x08C3A0, i_08c3a0),
    (0x08C3A6, i_08c3a6),
    (0x08C3AC, i_08c3ac),
    (0x08C3B0, i_08c3b0),
    (0x08C3B4, i_08c3b4),
    (0x08C3BA, i_08c3ba),
    (0x08C3C0, i_08c3c0),
    (0x08C3C6, i_08c3c6),
    (0x08C3CC, i_08c3cc),
    (0x08C3D0, i_08c3d0),
    (0x08C3D2, i_08c3d2),
    (0x08C3D8, i_08c3d8),
];
