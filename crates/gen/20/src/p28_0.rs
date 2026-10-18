// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p28_0) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $28D562: lea.l $28d586(pc), a1
pub fn i_28d562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D566;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D586;
    16
}
/// $28D566: jsr $4ae.l
pub fn i_28d566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D56C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D56C);
    cpu.pc = 0x4AE;
    16
}
/// $28D56C: jsr $517fe.l
pub fn i_28d56c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D572;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D572);
    cpu.pc = 0x517FE;
    16
}
/// $28D572: rts
pub fn i_28d572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D574;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D574: lea.l $28d5aa(pc), a1
pub fn i_28d574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D578;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D5AA;
    16
}
/// $28D578: jsr $4ae.l
pub fn i_28d578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D57E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D57E);
    cpu.pc = 0x4AE;
    16
}
/// $28D57E: jsr $517fe.l
pub fn i_28d57e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D584;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D584);
    cpu.pc = 0x517FE;
    16
}
/// $28D584: rts
pub fn i_28d584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D586;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D586: move.w #$f64e, d0
pub fn i_28d586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D58A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF64E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D58A: jsr $5dca4.l
pub fn i_28d58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D590;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D590);
    cpu.pc = 0x5DCA4;
    16
}
/// $28D590: move.w d0, $28(a6)
pub fn i_28d590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D594;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D594: move.w #$b9a, $2a(a6)
pub fn i_28d594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D59A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB9A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D59A: move.w #$ff4c, $2e(a6)
pub fn i_28d59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5A0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF4C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D5A0: move.w #$0, $2c(a6)
pub fn i_28d5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5A6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D5A6: bra.w $28d5ca
pub fn i_28d5a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D5CA;
    10
}
/// $28D5AA: move.w #$f000, d0
pub fn i_28d5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5AE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D5AE: jsr $5dca4.l
pub fn i_28d5ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D5B4);
    cpu.pc = 0x5DCA4;
    16
}
/// $28D5B4: move.w d0, $28(a6)
pub fn i_28d5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5B8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D5B8: move.w #$663, $2a(a6)
pub fn i_28d5b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5BE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x663;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D5BE: move.w #$feb9, $2e(a6)
pub fn i_28d5be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEB9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D5C4: move.w #$0, $2c(a6)
pub fn i_28d5c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5CA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D5CA: bset.b #$4, $6b(a6)
pub fn i_28d5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5D0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $28D5D0: move.w #$d000, $38(a6)
pub fn i_28d5d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D5D6: lea.l $28d3e2(pc), a0
pub fn i_28d5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5DA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D3E2;
    16
}
/// $28D5DA: jsr $28cd4.l
pub fn i_28d5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D5E0);
    cpu.pc = 0x28CD4;
    16
}
/// $28D5E0: lea.l $ffff.w, a0
pub fn i_28d5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5E4;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $28D5E4: move.l a0, $48(a6)
pub fn i_28d5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5E8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D5E8: lea.l $28d152(pc), a0
pub fn i_28d5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5EC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D152;
    16
}
/// $28D5EC: move.l a0, $4c(a6)
pub fn i_28d5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5F0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D5F0: jsr $283ca.l
pub fn i_28d5f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D5F6);
    cpu.pc = 0x283CA;
    16
}
/// $28D5F6: jsr $283ca.l
pub fn i_28d5f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D5FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D5FC);
    cpu.pc = 0x283CA;
    16
}
/// $28D5FC: movea.l $c(a6), a0
pub fn i_28d5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D600;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $28D600: cmpa.l #$100440, a0
pub fn i_28d600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D606;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $28D606: beq.b $28d612
pub fn i_28d606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D608;
    cpu.ir = 0x670A;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28D612; return 10; }
    8
}
/// $28D608: cmpa.l #$1004e0, a0
pub fn i_28d608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D60E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $28D60E: bne.w $28d616
pub fn i_28d60e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D612;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D616; return 10; }
    8
}
/// $28D612: bra.w $28d61a
pub fn i_28d612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D616;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D61A;
    10
}
/// $28D616: movea.l $c(a0), a0
pub fn i_28d616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D61A;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $28D61A: cmpa.l #$100440, a0
pub fn i_28d61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D620;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $28D620: bne.w $28d62c
pub fn i_28d620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D624;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D62C; return 10; }
    8
}
/// $28D624: move.w #$7c, d1
pub fn i_28d624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D628;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D628: bra.w $28d630
pub fn i_28d628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D62C;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D630;
    10
}
/// $28D62C: move.w #$14c, d1
pub fn i_28d62c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D630;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D630: jsr $236e.l
pub fn i_28d630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D636;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D636);
    cpu.pc = 0x236E;
    16
}
/// $28D636: movea.l $c(a6), a0
pub fn i_28d636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D63A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $28D63A: move.b $3a(a0), $3a(a6)
pub fn i_28d63a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D640;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $28D640: move.w $22(a0), $22(a6)
pub fn i_28d640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D646;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $28D646: move.w $24(a0), d0
pub fn i_28d646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D64A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D64A: addi.w #$20, d0
pub fn i_28d64a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D64E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $28D64E: move.w d0, $82(a6)
pub fn i_28d64e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D652;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D652: move.w d0, $24(a6)
pub fn i_28d652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D656;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D656: btst.b #$0, $3a(a6)
pub fn i_28d656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D65C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D65C: beq.w $28d664
pub fn i_28d65c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D660;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28D664; return 10; }
    8
}
/// $28D660: neg.w $28(a6)
pub fn i_28d660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D664;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D664: lea.l $28d66a(pc), a1
pub fn i_28d664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D668;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D66A;
    16
}
/// $28D668: move.l a1, (a6)
pub fn i_28d668(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D66A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D66A: move.w $2a(a6), d0
pub fn i_28d66a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D66E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D66E: bpl.w $28d67c
pub fn i_28d66e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D672;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x28D67C; return 10; }
    8
}
/// $28D672: move.w $28(a6), d0
pub fn i_28d672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D676;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D676: asr.w #$4, d0
pub fn i_28d676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D678;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $28D678: sub.w d0, $28(a6)
pub fn i_28d678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D67C;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D67C: jsr $27d50.l
pub fn i_28d67c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D682;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D682);
    cpu.pc = 0x27D50;
    16
}
/// $28D682: bcs.w $28d6f8
pub fn i_28d682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D686;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28D6F8; return 10; }
    8
}
/// $28D686: jsr $28d70.l
pub fn i_28d686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D68C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D68C);
    cpu.pc = 0x28D70;
    16
}
/// $28D68C: jsr $283d8.l
pub fn i_28d68c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D692;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D692);
    cpu.pc = 0x283D8;
    16
}
/// $28D692: btst.b #$1, $13(a6)
pub fn i_28d692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D698;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D698: bne.w $28d714
pub fn i_28d698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D69C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D714; return 10; }
    8
}
/// $28D69C: btst.b #$3, $13(a6)
pub fn i_28d69c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6A2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D6A2: bne.w $28d706
pub fn i_28d6a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D706; return 10; }
    8
}
/// $28D6A6: cmpi.w #$100, $24(a6)
pub fn i_28d6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6AC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $28D6AC: bmi.w $28d794
pub fn i_28d6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6B0;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28D6B0: cmpi.w #$150, $22(a6)
pub fn i_28d6b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6B6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $28D6B6: bcc.w $28d794
pub fn i_28d6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28D6BA: rts
pub fn i_28d6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D6BC: move.w #$1027, d0
pub fn i_28d6bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6C0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D6C0: jsr $2352.l
pub fn i_28d6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6C6);
    cpu.pc = 0x2352;
    16
}
/// $28D6C6: jsr $13600.l
pub fn i_28d6c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6CC);
    cpu.pc = 0x13600;
    16
}
/// $28D6CC: move.w #$178, d1
pub fn i_28d6cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6D0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D6D0: jsr $236e.l
pub fn i_28d6d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6D6);
    cpu.pc = 0x236E;
    16
}
/// $28D6D6: lea.l $29e76c.l, a0
pub fn i_28d6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29E76C;
    20
}
/// $28D6DC: jsr $28cd4.l
pub fn i_28d6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6E2);
    cpu.pc = 0x28CD4;
    16
}
/// $28D6E2: lea.l $28d29a(pc), a0
pub fn i_28d6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6E6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D29A;
    16
}
/// $28D6E6: move.l a0, $4c(a6)
pub fn i_28d6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6EA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D6EA: jsr $283ca.l
pub fn i_28d6ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D6F0);
    cpu.pc = 0x283CA;
    16
}
/// $28D6F0: lea.l $28d74e(pc), a1
pub fn i_28d6f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D74E;
    16
}
/// $28D6F4: move.l a1, (a6)
pub fn i_28d6f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D6F6: rts
pub fn i_28d6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D6F8: move.w #$1027, d0
pub fn i_28d6f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D6FC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D6FC: jsr $2352.l
pub fn i_28d6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D702);
    cpu.pc = 0x2352;
    16
}
/// $28D702: bra.w $28d71e
pub fn i_28d702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D706;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D71E;
    10
}
/// $28D706: move.w #$1027, d0
pub fn i_28d706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D70A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D70A: jsr $2352.l
pub fn i_28d70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D710);
    cpu.pc = 0x2352;
    16
}
/// $28D710: bra.w $28d71e
pub fn i_28d710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D714;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D71E;
    10
}
/// $28D714: move.w #$1027, d0
pub fn i_28d714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D718;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1027;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D718: jsr $2352.l
pub fn i_28d718(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D71E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D71E);
    cpu.pc = 0x2352;
    16
}
/// $28D71E: jsr $13600.l
pub fn i_28d71e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D724;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D724);
    cpu.pc = 0x13600;
    16
}
/// $28D724: move.w #$d, d1
pub fn i_28d724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D728;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D728: jsr $236e.l
pub fn i_28d728(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D72E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D72E);
    cpu.pc = 0x236E;
    16
}
/// $28D72E: lea.l $28d52e(pc), a0
pub fn i_28d72e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D732;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D52E;
    16
}
/// $28D732: jsr $28cd4.l
pub fn i_28d732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D738;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D738);
    cpu.pc = 0x28CD4;
    16
}
/// $28D738: lea.l $28d74e(pc), a1
pub fn i_28d738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D73C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D74E;
    16
}
/// $28D73C: move.l a1, (a6)
pub fn i_28d73c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D73E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D73E: lea.l $28d1f6(pc), a0
pub fn i_28d73e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D742;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D1F6;
    16
}
/// $28D742: move.l a0, $4c(a6)
pub fn i_28d742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D746;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D746: jsr $283ca.l
pub fn i_28d746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D74C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D74C);
    cpu.pc = 0x283CA;
    16
}
/// $28D74C: rts
pub fn i_28d74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D74E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D74E: jsr $283ca.l
pub fn i_28d74e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D754;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D754);
    cpu.pc = 0x283CA;
    16
}
/// $28D754: jsr $283d8.l
pub fn i_28d754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D75A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D75A);
    cpu.pc = 0x283D8;
    16
}
/// $28D75A: lea.l $28d76e(pc), a1
pub fn i_28d75a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D75E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D76E;
    16
}
/// $28D75E: move.l a1, (a6)
pub fn i_28d75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D760;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D760: jsr $2783a.l
pub fn i_28d760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D766;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D766);
    cpu.pc = 0x2783A;
    16
}
/// $28D766: jsr $28d70.l
pub fn i_28d766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D76C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D76C);
    cpu.pc = 0x28D70;
    16
}
/// $28D76C: rts
pub fn i_28d76c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D76E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D76E: lea.l $ffff.w, a0
pub fn i_28d76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D772;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $28D772: move.l a0, $4c(a6)
pub fn i_28d772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D776;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D776: jsr $283ca.l
pub fn i_28d776(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D77C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D77C);
    cpu.pc = 0x283CA;
    16
}
/// $28D77C: jsr $2783a.l
pub fn i_28d77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D782;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D782);
    cpu.pc = 0x2783A;
    16
}
/// $28D782: jsr $28d70.l
pub fn i_28d782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D788);
    cpu.pc = 0x28D70;
    16
}
/// $28D788: bcc.w $28d792
pub fn i_28d788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D78C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D792; return 10; }
    8
}
/// $28D78C: jmp $518.l
pub fn i_28d78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D792;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $28D792: rts
pub fn i_28d792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D794;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D794: jmp $518.l
pub fn i_28d794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D79A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $28D79A: rts
pub fn i_28d79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D79C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D79C: jsr $5b6.l
pub fn i_28d79c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D7A2);
    cpu.pc = 0x5B6;
    16
}
/// $28D7A2: jmp $518.l
pub fn i_28d7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7A8;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $28D7A8: rts
pub fn i_28d7a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D7AA: bset.b #$4, $6b(a6)
pub fn i_28d7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7B0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $28D7B0: move.w #$d000, $38(a6)
pub fn i_28d7b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D7B6: lea.l $28d3e2(pc), a0
pub fn i_28d7b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7BA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D3E2;
    16
}
/// $28D7BA: jsr $28cd4.l
pub fn i_28d7ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D7C0);
    cpu.pc = 0x28CD4;
    16
}
/// $28D7C0: lea.l $ffff.w, a0
pub fn i_28d7c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7C4;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $28D7C4: move.l a0, $48(a6)
pub fn i_28d7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7C8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D7C8: lea.l $28d152(pc), a0
pub fn i_28d7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7CC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D152;
    16
}
/// $28D7CC: move.l a0, $4c(a6)
pub fn i_28d7cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7D0;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D7D0: jsr $283ca.l
pub fn i_28d7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D7D6);
    cpu.pc = 0x283CA;
    16
}
/// $28D7D6: jsr $283ca.l
pub fn i_28d7d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D7DC);
    cpu.pc = 0x283CA;
    16
}
/// $28D7DC: move.w #$7c, d1
pub fn i_28d7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7E0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D7E0: jsr $236e.l
pub fn i_28d7e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D7E6);
    cpu.pc = 0x236E;
    16
}
/// $28D7E6: movea.l $c(a6), a0
pub fn i_28d7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $28D7EA: cmpa.l #$1008a0, a0
pub fn i_28d7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7F0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1008A0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $28D7F0: bne.w $28d7fe
pub fn i_28d7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D7FE; return 10; }
    8
}
/// $28D7F4: addi.w #$20, $24(a6)
pub fn i_28d7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7FA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $28D7FA: bra.w $28d814
pub fn i_28d7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D814;
    10
}
/// $28D7FE: move.w $22(a0), $22(a6)
pub fn i_28d7fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D804;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $28D804: move.w $24(a0), d0
pub fn i_28d804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D808;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D808: addi.w #$20, d0
pub fn i_28d808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D80C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $28D80C: move.w d0, $82(a6)
pub fn i_28d80c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D810;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D810: move.w d0, $24(a6)
pub fn i_28d810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D814;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D814: move.w #$fc00, d0
pub fn i_28d814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D818;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D818: jsr $5dca4.l
pub fn i_28d818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D81E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D81E);
    cpu.pc = 0x5DCA4;
    16
}
/// $28D81E: move.w d0, $28(a6)
pub fn i_28d81e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D822;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D822: move.w #$f800, $2a(a6)
pub fn i_28d822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D828;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xF800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D828: clr.w $2c(a6)
pub fn i_28d828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D82C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D82C: clr.w $2e(a6)
pub fn i_28d82c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D830;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D830: lea.l $28d836(pc), a1
pub fn i_28d830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D834;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D836;
    16
}
/// $28D834: move.l a1, (a6)
pub fn i_28d834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D836;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D836: jsr $27d50.l
pub fn i_28d836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D83C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D83C);
    cpu.pc = 0x27D50;
    16
}
/// $28D83C: bcs.w $28d6f8
pub fn i_28d83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D840;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28D6F8; return 10; }
    8
}
/// $28D840: jsr $28d70.l
pub fn i_28d840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D846;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D846);
    cpu.pc = 0x28D70;
    16
}
/// $28D846: jsr $283d8.l
pub fn i_28d846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D84C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D84C);
    cpu.pc = 0x283D8;
    16
}
/// $28D84C: btst.b #$1, $13(a6)
pub fn i_28d84c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D852;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D852: bne.w $28d714
pub fn i_28d852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D856;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D714; return 10; }
    8
}
/// $28D856: btst.b #$3, $13(a6)
pub fn i_28d856(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D85C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D85C: bne.w $28d706
pub fn i_28d85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D860;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D706; return 10; }
    8
}
/// $28D860: cmpi.w #$100, $24(a6)
pub fn i_28d860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D866;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $28D866: bmi.w $28d794
pub fn i_28d866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D86A;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28D86A: cmpi.w #$150, $22(a6)
pub fn i_28d86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D870;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $28D870: bcc.w $28d794
pub fn i_28d870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D874;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28D874: rts
pub fn i_28d874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D876;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D876: move.w #$fae2, d0
pub fn i_28d876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D87A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFAE2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D87A: jsr $5dca4.l
pub fn i_28d87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D880;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D880);
    cpu.pc = 0x5DCA4;
    16
}
/// $28D880: move.w d0, $28(a6)
pub fn i_28d880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D884;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D884: move.w #$ccb, $2a(a6)
pub fn i_28d884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D88A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xCCB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D88A: move.w #$fefa, $2e(a6)
pub fn i_28d88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D890;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEFA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D890: move.w #$0, $2c(a6)
pub fn i_28d890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D896;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D896: bset.b #$4, $6b(a6)
pub fn i_28d896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D89C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $28D89C: move.w #$d000, $38(a6)
pub fn i_28d89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D8A2: lea.l $28d3e2(pc), a0
pub fn i_28d8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8A6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D3E2;
    16
}
/// $28D8A6: jsr $28cd4.l
pub fn i_28d8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8AC);
    cpu.pc = 0x28CD4;
    16
}
/// $28D8AC: lea.l $ffff.w, a0
pub fn i_28d8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8B0;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $28D8B0: move.l a0, $48(a6)
pub fn i_28d8b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8B4;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D8B4: lea.l $28d152(pc), a0
pub fn i_28d8b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8B8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D152;
    16
}
/// $28D8B8: move.l a0, $4c(a6)
pub fn i_28d8b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8BC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28D8BC: jsr $283ca.l
pub fn i_28d8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8C2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8C2);
    cpu.pc = 0x283CA;
    16
}
/// $28D8C2: jsr $283ca.l
pub fn i_28d8c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8C8);
    cpu.pc = 0x283CA;
    16
}
/// $28D8C8: move.w #$14c, d1
pub fn i_28d8c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8CC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D8CC: jsr $236e.l
pub fn i_28d8cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8D2);
    cpu.pc = 0x236E;
    16
}
/// $28D8D2: move.w #$1, d1
pub fn i_28d8d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8D6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D8D6: jsr $236e.l
pub fn i_28d8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8DC);
    cpu.pc = 0x236E;
    16
}
/// $28D8DC: move.w #$14c, d1
pub fn i_28d8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8E0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x14C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D8E0: jsr $236e.l
pub fn i_28d8e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D8E6);
    cpu.pc = 0x236E;
    16
}
/// $28D8E6: movea.l $c(a6), a0
pub fn i_28d8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8EA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $28D8EA: cmpa.l #$1008a0, a0
pub fn i_28d8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8F0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1008A0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $28D8F0: bne.w $28d8fe
pub fn i_28d8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D8FE; return 10; }
    8
}
/// $28D8F4: addi.w #$10, $24(a6)
pub fn i_28d8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8FA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $28D8FA: bra.w $28d914
pub fn i_28d8fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D914;
    10
}
/// $28D8FE: move.w $22(a0), $22(a6)
pub fn i_28d8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D904;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $28D904: move.w $24(a0), d0
pub fn i_28d904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D908;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D908: addi.w #$10, d0
pub fn i_28d908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D90C;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $28D90C: move.w d0, $82(a6)
pub fn i_28d90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D910;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D910: move.w d0, $24(a6)
pub fn i_28d910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D914;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D914: clr.w $5c(a6)
pub fn i_28d914(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D918;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D918: lea.l $28d91e(pc), a1
pub fn i_28d918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D91C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D91E;
    16
}
/// $28D91C: move.l a1, (a6)
pub fn i_28d91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D91E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D91E: move.w $2a(a6), d0
pub fn i_28d91e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D922;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D922: cmpi.w #$ff00, d0
pub fn i_28d922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D926;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF00, Size::Word, false);
    8
}
/// $28D926: bpl.w $28d934
pub fn i_28d926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D92A;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x28D934; return 10; }
    8
}
/// $28D92A: move.w $28(a6), d0
pub fn i_28d92a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D92E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28D92E: asr.w #$4, d0
pub fn i_28d92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D930;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $28D930: sub.w d0, $28(a6)
pub fn i_28d930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D934;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D934: cmpi.b #$0, $1081ae.l
pub fn i_28d934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D93C;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x1081AE) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $28D93C: bne.w $28d94a
pub fn i_28d93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D940;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D94A; return 10; }
    8
}
/// $28D940: jsr $27d50.l
pub fn i_28d940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D946;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D946);
    cpu.pc = 0x27D50;
    16
}
/// $28D946: bra.w $28d950
pub fn i_28d946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D94A;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D950;
    10
}
/// $28D94A: jsr $27bc8.l
pub fn i_28d94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D950;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D950);
    cpu.pc = 0x27BC8;
    16
}
/// $28D950: bcc.w $28d996
pub fn i_28d950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D954;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D996; return 10; }
    8
}
/// $28D954: cmpi.w #$1, $5c(a6)
pub fn i_28d954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D95A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $28D95A: bcs.w $28d966
pub fn i_28d95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D95E;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28D966; return 10; }
    8
}
/// $28D95E: bra.w $28d6bc
pub fn i_28d95e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D962;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D6BC;
    10
}
/// $28D962: bra.w $28d996
pub fn i_28d962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D966;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D996;
    10
}
/// $28D966: move.w #$fce0, d0
pub fn i_28d966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D96A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFCE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D96A: jsr $5dca4.l
pub fn i_28d96a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D970;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D970);
    cpu.pc = 0x5DCA4;
    16
}
/// $28D970: move.w d0, $28(a6)
pub fn i_28d970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D974;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D974: move.w #$3c0, $2a(a6)
pub fn i_28d974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D97A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D97A: move.w #$ff88, $2e(a6)
pub fn i_28d97a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D980;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF88;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D980: move.w #$0, $2c(a6)
pub fn i_28d980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D986;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D986: addq.w #$8, $24(a6)
pub fn i_28d986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D98A;
    cpu.ir = 0x506E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $28D98A: addi.w #$1, $5c(a6)
pub fn i_28d98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D990;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $28D990: lea.l $28d91e(pc), a1
pub fn i_28d990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D994;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28D91E;
    16
}
/// $28D994: move.l a1, (a6)
pub fn i_28d994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D996;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28D996: jsr $28d70.l
pub fn i_28d996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D99C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D99C);
    cpu.pc = 0x28D70;
    16
}
/// $28D99C: cmpi.b #$1, $3b(a6)
pub fn i_28d99c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9A2;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $28D9A2: bcs.w $28d9b6
pub fn i_28d9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9A6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28D9B6; return 10; }
    8
}
/// $28D9A6: jsr $283d8.l
pub fn i_28d9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D9AC);
    cpu.pc = 0x283D8;
    16
}
/// $28D9AC: btst.b #$1, $13(a6)
pub fn i_28d9ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D9B2: bne.w $28d6bc
pub fn i_28d9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9B6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D6BC; return 10; }
    8
}
/// $28D9B6: btst.b #$3, $13(a6)
pub fn i_28d9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9BC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28D9BC: bne.w $28d706
pub fn i_28d9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9C0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D706; return 10; }
    8
}
/// $28D9C0: cmpi.w #$100, $24(a6)
pub fn i_28d9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9C6;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $28D9C6: bmi.w $28d794
pub fn i_28d9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9CA;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28D9CA: cmpi.w #$150, $22(a6)
pub fn i_28d9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9D0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $28D9D0: bcc.w $28d794
pub fn i_28d9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28D9D4: jsr $32afa.l
pub fn i_28d9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9DA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D9DA);
    cpu.pc = 0x32AFA;
    16
}
/// $28D9DA: rts
pub fn i_28d9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28D9DC: move.w #$feab, d0
pub fn i_28d9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9E0;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFEAB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28D9E0: jsr $5dca4.l
pub fn i_28d9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28D9E6);
    cpu.pc = 0x5DCA4;
    16
}
/// $28D9E6: move.w d0, $28(a6)
pub fn i_28d9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9EA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D9EA: move.w #$2a6, $2a(a6)
pub fn i_28d9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9F0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A6;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D9F0: move.w #$ff8f, $2e(a6)
pub fn i_28d9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF8F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D9F6: move.w #$0, $2c(a6)
pub fn i_28d9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9FC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28D9FC: bset.b #$4, $6b(a6)
pub fn i_28d9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA02;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $28DA02: move.w #$d000, $38(a6)
pub fn i_28da02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA08;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DA08: lea.l $28d3e2(pc), a0
pub fn i_28da08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA0C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D3E2;
    16
}
/// $28DA0C: jsr $28cd4.l
pub fn i_28da0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DA12);
    cpu.pc = 0x28CD4;
    16
}
/// $28DA12: lea.l $ffff.w, a0
pub fn i_28da12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA16;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $28DA16: move.l a0, $48(a6)
pub fn i_28da16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA1A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28DA1A: lea.l $28d152(pc), a0
pub fn i_28da1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA1E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28D152;
    16
}
/// $28DA1E: move.l a0, $4c(a6)
pub fn i_28da1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA22;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $28DA22: jsr $283ca.l
pub fn i_28da22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DA28);
    cpu.pc = 0x283CA;
    16
}
/// $28DA28: jsr $283ca.l
pub fn i_28da28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DA2E);
    cpu.pc = 0x283CA;
    16
}
/// $28DA2E: move.w #$7c, d1
pub fn i_28da2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA32;
    cpu.ir = 0x323C;
    let v1: u32 = 0x7C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28DA32: jsr $236e.l
pub fn i_28da32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DA38);
    cpu.pc = 0x236E;
    16
}
/// $28DA38: movea.l $c(a6), a0
pub fn i_28da38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA3C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $28DA3C: cmpa.l #$1008a0, a0
pub fn i_28da3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA42;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1008A0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $28DA42: bne.w $28da50
pub fn i_28da42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA46;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28DA50; return 10; }
    8
}
/// $28DA46: addi.w #$10, $24(a6)
pub fn i_28da46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA4C;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x10, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $28DA4C: bra.w $28da66
pub fn i_28da4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA50;
    cpu.ir = 0x6000;
    cpu.pc = 0x28DA66;
    10
}
/// $28DA50: move.w $22(a0), $22(a6)
pub fn i_28da50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA56;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $28DA56: move.w $24(a0), d0
pub fn i_28da56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA5A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28DA5A: addi.w #$10, d0
pub fn i_28da5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA5E;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $28DA5E: move.w d0, $82(a6)
pub fn i_28da5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA62;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DA62: move.w d0, $24(a6)
pub fn i_28da62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA66;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DA66: move.w #$12, $5c(a6)
pub fn i_28da66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DA6C: lea.l $28daf4(pc), a1
pub fn i_28da6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA70;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28DAF4;
    16
}
/// $28DA70: move.l a1, (a6)
pub fn i_28da70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA72;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28DA72: bra.w $28daf4
pub fn i_28da72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA76;
    cpu.ir = 0x6000;
    cpu.pc = 0x28DAF4;
    10
}
/// $28DA76: move.w $2a(a6), d0
pub fn i_28da76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA7A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28DA7A: bpl.w $28da88
pub fn i_28da7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA7E;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x28DA88; return 10; }
    8
}
/// $28DA7E: move.w $28(a6), d0
pub fn i_28da7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA82;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28DA82: asr.w #$4, d0
pub fn i_28da82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA84;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $28DA84: sub.w d0, $28(a6)
pub fn i_28da84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA88;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $28DA88: jsr $27bc8.l
pub fn i_28da88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DA8E);
    cpu.pc = 0x27BC8;
    16
}
/// $28DA8E: bcc.w $28dabe
pub fn i_28da8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28DABE; return 10; }
    8
}
/// $28DA92: move.w #$ff2b, d0
pub fn i_28da92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA96;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF2B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28DA96: jsr $5dca4.l
pub fn i_28da96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DA9C);
    cpu.pc = 0x5DCA4;
    16
}
/// $28DA9C: move.w d0, $28(a6)
pub fn i_28da9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAA0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DAA0: move.w #$4fe, $2a(a6)
pub fn i_28daa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAA6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4FE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DAA6: move.w #$ff2b, $2e(a6)
pub fn i_28daa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAAC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF2B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DAAC: move.w #$0, $2c(a6)
pub fn i_28daac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAB2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DAB2: move.w #$12, $5c(a6)
pub fn i_28dab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAB8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $28DAB8: lea.l $28daf4(pc), a1
pub fn i_28dab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DABC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28DAF4;
    16
}
/// $28DABC: move.l a1, (a6)
pub fn i_28dabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DABE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28DABE: jsr $28d70.l
pub fn i_28dabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DAC4);
    cpu.pc = 0x28D70;
    16
}
/// $28DAC4: jsr $283d8.l
pub fn i_28dac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DACA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DACA);
    cpu.pc = 0x283D8;
    16
}
/// $28DACA: btst.b #$1, $13(a6)
pub fn i_28daca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAD0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28DAD0: bne.w $28d714
pub fn i_28dad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D714; return 10; }
    8
}
/// $28DAD4: btst.b #$3, $13(a6)
pub fn i_28dad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DADA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28DADA: bne.w $28d706
pub fn i_28dada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DADE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D706; return 10; }
    8
}
/// $28DADE: cmpi.w #$100, $24(a6)
pub fn i_28dade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAE4;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $28DAE4: bmi.w $28d794
pub fn i_28dae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAE8;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28DAE8: cmpi.w #$150, $22(a6)
pub fn i_28dae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAEE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $28DAEE: bcc.w $28d794
pub fn i_28daee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28DAF2: rts
pub fn i_28daf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28DAF4: move.w $2a(a6), d0
pub fn i_28daf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAF8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28DAF8: bpl.w $28db06
pub fn i_28daf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAFC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x28DB06; return 10; }
    8
}
/// $28DAFC: move.w $28(a6), d0
pub fn i_28dafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB00;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $28DB00: asr.w #$4, d0
pub fn i_28db00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB02;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $28DB02: sub.w d0, $28(a6)
pub fn i_28db02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB06;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $28DB06: jsr $27d50.l
pub fn i_28db06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DB0C);
    cpu.pc = 0x27D50;
    16
}
/// $28DB0C: bcc.w $28db14
pub fn i_28db0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28DB14; return 10; }
    8
}
/// $28DB10: bra.w $28d714
pub fn i_28db10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB14;
    cpu.ir = 0x6000;
    cpu.pc = 0x28D714;
    10
}
/// $28DB14: subi.w #$1, $5c(a6)
pub fn i_28db14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB1A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $28DB1A: bne.w $28db24
pub fn i_28db1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB1E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28DB24; return 10; }
    8
}
/// $28DB1E: lea.l $28da76(pc), a1
pub fn i_28db1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28DA76;
    16
}
/// $28DB22: move.l a1, (a6)
pub fn i_28db22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB24;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28DB24: jsr $28d70.l
pub fn i_28db24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DB2A);
    cpu.pc = 0x28D70;
    16
}
/// $28DB2A: jsr $283d8.l
pub fn i_28db2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DB30);
    cpu.pc = 0x283D8;
    16
}
/// $28DB30: btst.b #$1, $13(a6)
pub fn i_28db30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB36;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28DB36: bne.w $28d714
pub fn i_28db36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB3A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D714; return 10; }
    8
}
/// $28DB3A: btst.b #$3, $13(a6)
pub fn i_28db3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB40;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $28DB40: bne.w $28d706
pub fn i_28db40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB44;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D706; return 10; }
    8
}
/// $28DB44: cmpi.w #$100, $24(a6)
pub fn i_28db44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB4A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x100, Size::Word, false);
    16
}
/// $28DB4A: bmi.w $28d794
pub fn i_28db4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB4E;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28DB4E: cmpi.w #$150, $22(a6)
pub fn i_28db4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB54;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $28DB54: bcc.w $28d794
pub fn i_28db54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28D794; return 10; }
    8
}
/// $28DB58: rts
pub fn i_28db58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB5A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28DB5A: move.b #$ff, $1081ae.l
pub fn i_28db5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB62;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x1081AE, v1 as u8);
    16
}
/// $28DB62: jmp $518.l
pub fn i_28db62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $28DB68: rts
pub fn i_28db68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28DB6A: clr.b $1081ae.l
pub fn i_28db6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB70;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x1081AE) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x1081AE, r2 as u8);
    16
}
/// $28DB70: jmp $518.l
pub fn i_28db70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB76;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $28DB76: rts
pub fn i_28db76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB78;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $28DD3C: move.w #$3, d1
pub fn i_28dd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD40;
    cpu.ir = 0x323C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28DD40: jsr $29f2.l
pub fn i_28dd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DD46);
    cpu.pc = 0x29F2;
    16
}
/// $28DD46: move.b #$7f, d0
pub fn i_28dd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD4A;
    cpu.ir = 0x103C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $28DD4A: move.b d0, $32(a6)
pub fn i_28dd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD4E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $28DD4E: move.b d0, $33(a6)
pub fn i_28dd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD52;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $28DD52: move.w #$e000, d0
pub fn i_28dd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD56;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $28DD56: jsr $28134.l
pub fn i_28dd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DD5C);
    cpu.pc = 0x28134;
    16
}
/// $28DD5C: andi.w #$ffe3, $38(a6)
pub fn i_28dd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD62;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $28DD62: ori.w #$10, $38(a6)
pub fn i_28dd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD68;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x10) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $28DD68: lea.l $28dc6e(pc), a0
pub fn i_28dd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD6C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x28DC6E;
    16
}
/// $28DD6C: jsr $28cd4.l
pub fn i_28dd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DD72);
    cpu.pc = 0x28CD4;
    16
}
/// $28DD72: jsr $267e2.l
pub fn i_28dd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DD78);
    cpu.pc = 0x267E2;
    16
}
/// $28DD78: lea.l $28dd7e(pc), a1
pub fn i_28dd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x28DD7E;
    16
}
/// $28DD7C: move.l a1, (a6)
pub fn i_28dd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $28DD7E: jsr $28d70.l
pub fn i_28dd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD84;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28DD84);
    cpu.pc = 0x28D70;
    16
}
/// $28DD84: cmpi.b #$3c, $3b(a6)
pub fn i_28dd84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD8A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Byte, false);
    16
}
/// $28DD8A: bls.w $28dd94
pub fn i_28dd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD8E;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x28DD94; return 10; }
    8
}
/// $28DD8E: jmp $518.l
pub fn i_28dd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD94;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $28DD94: rts
pub fn i_28dd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD96;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x28D562, i_28d562),
    (0x28D566, i_28d566),
    (0x28D56C, i_28d56c),
    (0x28D572, i_28d572),
    (0x28D574, i_28d574),
    (0x28D578, i_28d578),
    (0x28D57E, i_28d57e),
    (0x28D584, i_28d584),
    (0x28D586, i_28d586),
    (0x28D58A, i_28d58a),
    (0x28D590, i_28d590),
    (0x28D594, i_28d594),
    (0x28D59A, i_28d59a),
    (0x28D5A0, i_28d5a0),
    (0x28D5A6, i_28d5a6),
    (0x28D5AA, i_28d5aa),
    (0x28D5AE, i_28d5ae),
    (0x28D5B4, i_28d5b4),
    (0x28D5B8, i_28d5b8),
    (0x28D5BE, i_28d5be),
    (0x28D5C4, i_28d5c4),
    (0x28D5CA, i_28d5ca),
    (0x28D5D0, i_28d5d0),
    (0x28D5D6, i_28d5d6),
    (0x28D5DA, i_28d5da),
    (0x28D5E0, i_28d5e0),
    (0x28D5E4, i_28d5e4),
    (0x28D5E8, i_28d5e8),
    (0x28D5EC, i_28d5ec),
    (0x28D5F0, i_28d5f0),
    (0x28D5F6, i_28d5f6),
    (0x28D5FC, i_28d5fc),
    (0x28D600, i_28d600),
    (0x28D606, i_28d606),
    (0x28D608, i_28d608),
    (0x28D60E, i_28d60e),
    (0x28D612, i_28d612),
    (0x28D616, i_28d616),
    (0x28D61A, i_28d61a),
    (0x28D620, i_28d620),
    (0x28D624, i_28d624),
    (0x28D628, i_28d628),
    (0x28D62C, i_28d62c),
    (0x28D630, i_28d630),
    (0x28D636, i_28d636),
    (0x28D63A, i_28d63a),
    (0x28D640, i_28d640),
    (0x28D646, i_28d646),
    (0x28D64A, i_28d64a),
    (0x28D64E, i_28d64e),
    (0x28D652, i_28d652),
    (0x28D656, i_28d656),
    (0x28D65C, i_28d65c),
    (0x28D660, i_28d660),
    (0x28D664, i_28d664),
    (0x28D668, i_28d668),
    (0x28D66A, i_28d66a),
    (0x28D66E, i_28d66e),
    (0x28D672, i_28d672),
    (0x28D676, i_28d676),
    (0x28D678, i_28d678),
    (0x28D67C, i_28d67c),
    (0x28D682, i_28d682),
    (0x28D686, i_28d686),
    (0x28D68C, i_28d68c),
    (0x28D692, i_28d692),
    (0x28D698, i_28d698),
    (0x28D69C, i_28d69c),
    (0x28D6A2, i_28d6a2),
    (0x28D6A6, i_28d6a6),
    (0x28D6AC, i_28d6ac),
    (0x28D6B0, i_28d6b0),
    (0x28D6B6, i_28d6b6),
    (0x28D6BA, i_28d6ba),
    (0x28D6BC, i_28d6bc),
    (0x28D6C0, i_28d6c0),
    (0x28D6C6, i_28d6c6),
    (0x28D6CC, i_28d6cc),
    (0x28D6D0, i_28d6d0),
    (0x28D6D6, i_28d6d6),
    (0x28D6DC, i_28d6dc),
    (0x28D6E2, i_28d6e2),
    (0x28D6E6, i_28d6e6),
    (0x28D6EA, i_28d6ea),
    (0x28D6F0, i_28d6f0),
    (0x28D6F4, i_28d6f4),
    (0x28D6F6, i_28d6f6),
    (0x28D6F8, i_28d6f8),
    (0x28D6FC, i_28d6fc),
    (0x28D702, i_28d702),
    (0x28D706, i_28d706),
    (0x28D70A, i_28d70a),
    (0x28D710, i_28d710),
    (0x28D714, i_28d714),
    (0x28D718, i_28d718),
    (0x28D71E, i_28d71e),
    (0x28D724, i_28d724),
    (0x28D728, i_28d728),
    (0x28D72E, i_28d72e),
    (0x28D732, i_28d732),
    (0x28D738, i_28d738),
    (0x28D73C, i_28d73c),
    (0x28D73E, i_28d73e),
    (0x28D742, i_28d742),
    (0x28D746, i_28d746),
    (0x28D74C, i_28d74c),
    (0x28D74E, i_28d74e),
    (0x28D754, i_28d754),
    (0x28D75A, i_28d75a),
    (0x28D75E, i_28d75e),
    (0x28D760, i_28d760),
    (0x28D766, i_28d766),
    (0x28D76C, i_28d76c),
    (0x28D76E, i_28d76e),
    (0x28D772, i_28d772),
    (0x28D776, i_28d776),
    (0x28D77C, i_28d77c),
    (0x28D782, i_28d782),
    (0x28D788, i_28d788),
    (0x28D78C, i_28d78c),
    (0x28D792, i_28d792),
    (0x28D794, i_28d794),
    (0x28D79A, i_28d79a),
    (0x28D79C, i_28d79c),
    (0x28D7A2, i_28d7a2),
    (0x28D7A8, i_28d7a8),
    (0x28D7AA, i_28d7aa),
    (0x28D7B0, i_28d7b0),
    (0x28D7B6, i_28d7b6),
    (0x28D7BA, i_28d7ba),
    (0x28D7C0, i_28d7c0),
    (0x28D7C4, i_28d7c4),
    (0x28D7C8, i_28d7c8),
    (0x28D7CC, i_28d7cc),
    (0x28D7D0, i_28d7d0),
    (0x28D7D6, i_28d7d6),
    (0x28D7DC, i_28d7dc),
    (0x28D7E0, i_28d7e0),
    (0x28D7E6, i_28d7e6),
    (0x28D7EA, i_28d7ea),
    (0x28D7F0, i_28d7f0),
    (0x28D7F4, i_28d7f4),
    (0x28D7FA, i_28d7fa),
    (0x28D7FE, i_28d7fe),
    (0x28D804, i_28d804),
    (0x28D808, i_28d808),
    (0x28D80C, i_28d80c),
    (0x28D810, i_28d810),
    (0x28D814, i_28d814),
    (0x28D818, i_28d818),
    (0x28D81E, i_28d81e),
    (0x28D822, i_28d822),
    (0x28D828, i_28d828),
    (0x28D82C, i_28d82c),
    (0x28D830, i_28d830),
    (0x28D834, i_28d834),
    (0x28D836, i_28d836),
    (0x28D83C, i_28d83c),
    (0x28D840, i_28d840),
    (0x28D846, i_28d846),
    (0x28D84C, i_28d84c),
    (0x28D852, i_28d852),
    (0x28D856, i_28d856),
    (0x28D85C, i_28d85c),
    (0x28D860, i_28d860),
    (0x28D866, i_28d866),
    (0x28D86A, i_28d86a),
    (0x28D870, i_28d870),
    (0x28D874, i_28d874),
    (0x28D876, i_28d876),
    (0x28D87A, i_28d87a),
    (0x28D880, i_28d880),
    (0x28D884, i_28d884),
    (0x28D88A, i_28d88a),
    (0x28D890, i_28d890),
    (0x28D896, i_28d896),
    (0x28D89C, i_28d89c),
    (0x28D8A2, i_28d8a2),
    (0x28D8A6, i_28d8a6),
    (0x28D8AC, i_28d8ac),
    (0x28D8B0, i_28d8b0),
    (0x28D8B4, i_28d8b4),
    (0x28D8B8, i_28d8b8),
    (0x28D8BC, i_28d8bc),
    (0x28D8C2, i_28d8c2),
    (0x28D8C8, i_28d8c8),
    (0x28D8CC, i_28d8cc),
    (0x28D8D2, i_28d8d2),
    (0x28D8D6, i_28d8d6),
    (0x28D8DC, i_28d8dc),
    (0x28D8E0, i_28d8e0),
    (0x28D8E6, i_28d8e6),
    (0x28D8EA, i_28d8ea),
    (0x28D8F0, i_28d8f0),
    (0x28D8F4, i_28d8f4),
    (0x28D8FA, i_28d8fa),
    (0x28D8FE, i_28d8fe),
    (0x28D904, i_28d904),
    (0x28D908, i_28d908),
    (0x28D90C, i_28d90c),
    (0x28D910, i_28d910),
    (0x28D914, i_28d914),
    (0x28D918, i_28d918),
    (0x28D91C, i_28d91c),
    (0x28D91E, i_28d91e),
    (0x28D922, i_28d922),
    (0x28D926, i_28d926),
    (0x28D92A, i_28d92a),
    (0x28D92E, i_28d92e),
    (0x28D930, i_28d930),
    (0x28D934, i_28d934),
    (0x28D93C, i_28d93c),
    (0x28D940, i_28d940),
    (0x28D946, i_28d946),
    (0x28D94A, i_28d94a),
    (0x28D950, i_28d950),
    (0x28D954, i_28d954),
    (0x28D95A, i_28d95a),
    (0x28D95E, i_28d95e),
    (0x28D962, i_28d962),
    (0x28D966, i_28d966),
    (0x28D96A, i_28d96a),
    (0x28D970, i_28d970),
    (0x28D974, i_28d974),
    (0x28D97A, i_28d97a),
    (0x28D980, i_28d980),
    (0x28D986, i_28d986),
    (0x28D98A, i_28d98a),
    (0x28D990, i_28d990),
    (0x28D994, i_28d994),
    (0x28D996, i_28d996),
    (0x28D99C, i_28d99c),
    (0x28D9A2, i_28d9a2),
    (0x28D9A6, i_28d9a6),
    (0x28D9AC, i_28d9ac),
    (0x28D9B2, i_28d9b2),
    (0x28D9B6, i_28d9b6),
    (0x28D9BC, i_28d9bc),
    (0x28D9C0, i_28d9c0),
    (0x28D9C6, i_28d9c6),
    (0x28D9CA, i_28d9ca),
    (0x28D9D0, i_28d9d0),
    (0x28D9D4, i_28d9d4),
    (0x28D9DA, i_28d9da),
    (0x28D9DC, i_28d9dc),
    (0x28D9E0, i_28d9e0),
    (0x28D9E6, i_28d9e6),
    (0x28D9EA, i_28d9ea),
    (0x28D9F0, i_28d9f0),
    (0x28D9F6, i_28d9f6),
    (0x28D9FC, i_28d9fc),
    (0x28DA02, i_28da02),
    (0x28DA08, i_28da08),
    (0x28DA0C, i_28da0c),
    (0x28DA12, i_28da12),
    (0x28DA16, i_28da16),
    (0x28DA1A, i_28da1a),
    (0x28DA1E, i_28da1e),
    (0x28DA22, i_28da22),
    (0x28DA28, i_28da28),
    (0x28DA2E, i_28da2e),
    (0x28DA32, i_28da32),
    (0x28DA38, i_28da38),
    (0x28DA3C, i_28da3c),
    (0x28DA42, i_28da42),
    (0x28DA46, i_28da46),
    (0x28DA4C, i_28da4c),
    (0x28DA50, i_28da50),
    (0x28DA56, i_28da56),
    (0x28DA5A, i_28da5a),
    (0x28DA5E, i_28da5e),
    (0x28DA62, i_28da62),
    (0x28DA66, i_28da66),
    (0x28DA6C, i_28da6c),
    (0x28DA70, i_28da70),
    (0x28DA72, i_28da72),
    (0x28DA76, i_28da76),
    (0x28DA7A, i_28da7a),
    (0x28DA7E, i_28da7e),
    (0x28DA82, i_28da82),
    (0x28DA84, i_28da84),
    (0x28DA88, i_28da88),
    (0x28DA8E, i_28da8e),
    (0x28DA92, i_28da92),
    (0x28DA96, i_28da96),
    (0x28DA9C, i_28da9c),
    (0x28DAA0, i_28daa0),
    (0x28DAA6, i_28daa6),
    (0x28DAAC, i_28daac),
    (0x28DAB2, i_28dab2),
    (0x28DAB8, i_28dab8),
    (0x28DABC, i_28dabc),
    (0x28DABE, i_28dabe),
    (0x28DAC4, i_28dac4),
    (0x28DACA, i_28daca),
    (0x28DAD0, i_28dad0),
    (0x28DAD4, i_28dad4),
    (0x28DADA, i_28dada),
    (0x28DADE, i_28dade),
    (0x28DAE4, i_28dae4),
    (0x28DAE8, i_28dae8),
    (0x28DAEE, i_28daee),
    (0x28DAF2, i_28daf2),
    (0x28DAF4, i_28daf4),
    (0x28DAF8, i_28daf8),
    (0x28DAFC, i_28dafc),
    (0x28DB00, i_28db00),
    (0x28DB02, i_28db02),
    (0x28DB06, i_28db06),
    (0x28DB0C, i_28db0c),
    (0x28DB10, i_28db10),
    (0x28DB14, i_28db14),
    (0x28DB1A, i_28db1a),
    (0x28DB1E, i_28db1e),
    (0x28DB22, i_28db22),
    (0x28DB24, i_28db24),
    (0x28DB2A, i_28db2a),
    (0x28DB30, i_28db30),
    (0x28DB36, i_28db36),
    (0x28DB3A, i_28db3a),
    (0x28DB40, i_28db40),
    (0x28DB44, i_28db44),
    (0x28DB4A, i_28db4a),
    (0x28DB4E, i_28db4e),
    (0x28DB54, i_28db54),
    (0x28DB58, i_28db58),
    (0x28DB5A, i_28db5a),
    (0x28DB62, i_28db62),
    (0x28DB68, i_28db68),
    (0x28DB6A, i_28db6a),
    (0x28DB70, i_28db70),
    (0x28DB76, i_28db76),
    (0x28DD3C, i_28dd3c),
    (0x28DD40, i_28dd40),
    (0x28DD46, i_28dd46),
    (0x28DD4A, i_28dd4a),
    (0x28DD4E, i_28dd4e),
    (0x28DD52, i_28dd52),
    (0x28DD56, i_28dd56),
    (0x28DD5C, i_28dd5c),
    (0x28DD62, i_28dd62),
    (0x28DD68, i_28dd68),
    (0x28DD6C, i_28dd6c),
    (0x28DD72, i_28dd72),
    (0x28DD78, i_28dd78),
    (0x28DD7C, i_28dd7c),
    (0x28DD7E, i_28dd7e),
    (0x28DD84, i_28dd84),
    (0x28DD8A, i_28dd8a),
    (0x28DD8E, i_28dd8e),
    (0x28DD94, i_28dd94),
];
