// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p08_1) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $081A18: jsr $28d70.l
pub fn i_081a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81A1E);
    cpu.pc = 0x28D70;
    16
}
/// $081A1E: jsr $82dae(pc)
pub fn i_081a1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81A22);
    cpu.pc = 0x82DAE;
    16
}
/// $081A22: bcc.w $81a2c
pub fn i_081a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81A2C; return 10; }
    8
}
/// $081A26: lea.l $81a30(pc), a1
pub fn i_081a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81A30;
    16
}
/// $081A2A: move.l a1, (a6)
pub fn i_081a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081A2C: bra.w $81cac
pub fn i_081a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A30;
    cpu.ir = 0x6000;
    cpu.pc = 0x81CAC;
    10
}
/// $081A30: clr.b $7e(a6)
pub fn i_081a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A34;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081A34: move.w $5c(a6), d0
pub fn i_081a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A38;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081A38: movea.l #$2e541e, a0
pub fn i_081a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A3E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E541E;
    cpu.a[0] = v1;
    4
}
/// $081A3E: lsl.w #$2, d0
pub fn i_081a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A40;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $081A40: movea.l (a0, d0.w), a0
pub fn i_081a40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A44;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $081A44: cmpa.l #$ffffffff, a0
pub fn i_081a44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A4A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $081A4A: beq.w $81a54
pub fn i_081a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A4E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81A54; return 10; }
    8
}
/// $081A4E: jsr $28cd4.l
pub fn i_081a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81A54);
    cpu.pc = 0x28CD4;
    16
}
/// $081A54: lea.l $81a5a(pc), a1
pub fn i_081a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81A5A;
    16
}
/// $081A58: move.l a1, (a6)
pub fn i_081a58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081A5A: jsr $2783a.l
pub fn i_081a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81A60);
    cpu.pc = 0x2783A;
    16
}
/// $081A60: jsr $82d12(pc)
pub fn i_081a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81A64);
    cpu.pc = 0x82D12;
    16
}
/// $081A64: lea.l $2e590a.l, a0
pub fn i_081a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A6A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E590A;
    20
}
/// $081A6A: jsr $82c7c(pc)
pub fn i_081a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81A6E);
    cpu.pc = 0x82C7C;
    16
}
/// $081A6E: bcc.w $81a78
pub fn i_081a6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A72;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81A78; return 10; }
    8
}
/// $081A72: lea.l $81a82(pc), a1
pub fn i_081a72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81A82;
    16
}
/// $081A76: move.l a1, (a6)
pub fn i_081a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081A78: jsr $28d70.l
pub fn i_081a78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81A7E);
    cpu.pc = 0x28D70;
    16
}
/// $081A7E: bra.w $81cac
pub fn i_081a7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A82;
    cpu.ir = 0x6000;
    cpu.pc = 0x81CAC;
    10
}
/// $081A82: jsr $82f90(pc)
pub fn i_081a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A86;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81A86);
    cpu.pc = 0x82F90;
    16
}
/// $081A86: lea.l $2be732.l, a0
pub fn i_081a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE732;
    20
}
/// $081A8C: tst.b $73(a6)
pub fn i_081a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A90;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081A90: beq.w $81a9a
pub fn i_081a90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81A9A; return 10; }
    8
}
/// $081A94: lea.l $2be7b4.l, a0
pub fn i_081a94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81A9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE7B4;
    20
}
/// $081A9A: jsr $799de.l
pub fn i_081a9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81AA0);
    cpu.pc = 0x799DE;
    16
}
/// $081AA0: move.w d0, $36(a6)
pub fn i_081aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AA4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081AA4: clr.w $28(a6)
pub fn i_081aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AA8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $081AA8: btst.b #$0, $74(a6)
pub fn i_081aa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AAE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $081AAE: beq.w $81ae4
pub fn i_081aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AB2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81AE4; return 10; }
    8
}
/// $081AB2: neg.w $36(a6)
pub fn i_081ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AB6;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $081AB6: move.w #$ffe0, $2c(a6)
pub fn i_081ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ABC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081ABC: lea.l $2e5b30.l, a0
pub fn i_081abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5B30;
    20
}
/// $081AC2: jsr $28cd4.l
pub fn i_081ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81AC8);
    cpu.pc = 0x28CD4;
    16
}
/// $081AC8: lea.l $81ace(pc), a1
pub fn i_081ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ACC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81ACE;
    16
}
/// $081ACC: move.l a1, (a6)
pub fn i_081acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ACE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081ACE: move.w $36(a6), d0
pub fn i_081ace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AD2;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081AD2: cmp.w $28(a6), d0
pub fn i_081ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AD6;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $081AD6: blt.w $81ae0
pub fn i_081ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ADA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x81AE0; return 10; }
    8
}
/// $081ADA: lea.l $81b12(pc), a1
pub fn i_081ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ADE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81B12;
    16
}
/// $081ADE: move.l a1, (a6)
pub fn i_081ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081AE0: bra.w $81b22
pub fn i_081ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AE4;
    cpu.ir = 0x6000;
    cpu.pc = 0x81B22;
    10
}
/// $081AE4: move.w #$20, $2c(a6)
pub fn i_081ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081AEA: lea.l $2e5b7a.l, a0
pub fn i_081aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AF0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5B7A;
    20
}
/// $081AF0: jsr $28cd4.l
pub fn i_081af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81AF6);
    cpu.pc = 0x28CD4;
    16
}
/// $081AF6: lea.l $81afc(pc), a1
pub fn i_081af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81AFC;
    16
}
/// $081AFA: move.l a1, (a6)
pub fn i_081afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81AFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081AFC: move.w $36(a6), d0
pub fn i_081afc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B00;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081B00: cmp.w $28(a6), d0
pub fn i_081b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B04;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $081B04: blt.w $81b0e
pub fn i_081b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B08;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x81B0E; return 10; }
    8
}
/// $081B08: lea.l $81b12(pc), a1
pub fn i_081b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81B12;
    16
}
/// $081B0C: move.l a1, (a6)
pub fn i_081b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081B0E: bra.w $81b22
pub fn i_081b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B12;
    cpu.ir = 0x6000;
    cpu.pc = 0x81B22;
    10
}
/// $081B12: clr.w $2c(a6)
pub fn i_081b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B16;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $081B16: move.w $36(a6), $28(a6)
pub fn i_081b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B1C;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    20
}
/// $081B1C: lea.l $81b22(pc), a1
pub fn i_081b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81B22;
    16
}
/// $081B20: move.l a1, (a6)
pub fn i_081b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081B22: jsr $82e04(pc)
pub fn i_081b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81B26);
    cpu.pc = 0x82E04;
    16
}
/// $081B26: bcc.w $81b30
pub fn i_081b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81B30; return 10; }
    8
}
/// $081B2A: lea.l $81b4e(pc), a1
pub fn i_081b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81B4E;
    16
}
/// $081B2E: move.l a1, (a6)
pub fn i_081b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081B30: jsr $27cee.l
pub fn i_081b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81B36);
    cpu.pc = 0x27CEE;
    16
}
/// $081B36: jsr $82d12(pc)
pub fn i_081b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81B3A);
    cpu.pc = 0x82D12;
    16
}
/// $081B3A: lea.l $2e58ea.l, a0
pub fn i_081b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B40;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E58EA;
    20
}
/// $081B40: jsr $82c7c(pc)
pub fn i_081b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B44;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81B44);
    cpu.pc = 0x82C7C;
    16
}
/// $081B44: jsr $28d70.l
pub fn i_081b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81B4A);
    cpu.pc = 0x28D70;
    16
}
/// $081B4A: bra.w $81cac
pub fn i_081b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x81CAC;
    10
}
/// $081B4E: btst.b #$0, $74(a6)
pub fn i_081b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B54;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $081B54: beq.w $81b76
pub fn i_081b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B58;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81B76; return 10; }
    8
}
/// $081B58: move.w #$60, $2c(a6)
pub fn i_081b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B5E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081B5E: lea.l $81b64(pc), a1
pub fn i_081b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81B64;
    16
}
/// $081B62: move.l a1, (a6)
pub fn i_081b62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081B64: cmpi.w #$0, $28(a6)
pub fn i_081b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B6A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $081B6A: blt.w $81b74
pub fn i_081b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B6E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x81B74; return 10; }
    8
}
/// $081B6E: lea.l $81b94(pc), a1
pub fn i_081b6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B72;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81B94;
    16
}
/// $081B72: move.l a1, (a6)
pub fn i_081b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B74;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081B74: bra.b $81b30
pub fn i_081b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B76;
    cpu.ir = 0x60BA;
    cpu.pc = 0x81B30;
    10
}
/// $081B76: move.w #$ffa0, $2c(a6)
pub fn i_081b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081B7C: lea.l $81b82(pc), a1
pub fn i_081b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B80;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81B82;
    16
}
/// $081B80: move.l a1, (a6)
pub fn i_081b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B82;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081B82: cmpi.w #$0, $28(a6)
pub fn i_081b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B88;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $081B88: blt.w $81b92
pub fn i_081b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B8C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x81B92; return 10; }
    8
}
/// $081B8C: lea.l $81b94(pc), a1
pub fn i_081b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B90;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81B94;
    16
}
/// $081B90: move.l a1, (a6)
pub fn i_081b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B92;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081B92: bra.b $81b30
pub fn i_081b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B94;
    cpu.ir = 0x609C;
    cpu.pc = 0x81B30;
    10
}
/// $081B94: clr.b $7e(a6)
pub fn i_081b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B98;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081B98: clr.w $2c(a6)
pub fn i_081b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81B9C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $081B9C: clr.w $28(a6)
pub fn i_081b9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BA0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $081BA0: move.w $5c(a6), d0
pub fn i_081ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BA4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081BA4: movea.l #$2e541e, a0
pub fn i_081ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BAA;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E541E;
    cpu.a[0] = v1;
    4
}
/// $081BAA: lsl.w #$2, d0
pub fn i_081baa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BAC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $081BAC: movea.l (a0, d0.w), a0
pub fn i_081bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BB0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $081BB0: cmpa.l #$ffffffff, a0
pub fn i_081bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BB6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $081BB6: beq.w $81bc0
pub fn i_081bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81BC0; return 10; }
    8
}
/// $081BBA: jsr $28cd4.l
pub fn i_081bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81BC0);
    cpu.pc = 0x28CD4;
    16
}
/// $081BC0: lea.l $81bc6(pc), a1
pub fn i_081bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81BC6;
    16
}
/// $081BC4: move.l a1, (a6)
pub fn i_081bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081BC6: jsr $2783a.l
pub fn i_081bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BCC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81BCC);
    cpu.pc = 0x2783A;
    16
}
/// $081BCC: jsr $82d12(pc)
pub fn i_081bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81BD0);
    cpu.pc = 0x82D12;
    16
}
/// $081BD0: lea.l $2e592a.l, a0
pub fn i_081bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E592A;
    20
}
/// $081BD6: jsr $82c7c(pc)
pub fn i_081bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81BDA);
    cpu.pc = 0x82C7C;
    16
}
/// $081BDA: bcc.w $81be4
pub fn i_081bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BDE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81BE4; return 10; }
    8
}
/// $081BDE: lea.l $819e0(pc), a1
pub fn i_081bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BE2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x819E0;
    16
}
/// $081BE2: move.l a1, (a6)
pub fn i_081be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BE4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081BE4: jsr $28d70.l
pub fn i_081be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81BEA);
    cpu.pc = 0x28D70;
    16
}
/// $081BEA: bra.w $81cac
pub fn i_081bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BEE;
    cpu.ir = 0x6000;
    cpu.pc = 0x81CAC;
    10
}
/// $081BEE: move.w #$1071, d0
pub fn i_081bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BF2;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1071;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081BF2: jsr $2222.l
pub fn i_081bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81BF8);
    cpu.pc = 0x2222;
    16
}
/// $081BF8: clr.b $72(a6)
pub fn i_081bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81BFC;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081BFC: clr.b $106ed3.l
pub fn i_081bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C02;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED3) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED3, r2 as u8);
    16
}
/// $081C02: move.b #$1, $20(a6)
pub fn i_081c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C08;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $081C08: move.w #$b4, $70(a6)
pub fn i_081c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C0E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081C0E: move.l #$ffffffff, $48(a6)
pub fn i_081c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C16;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $081C16: move.b #$1, $10a2d1.l
pub fn i_081c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C1E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $081C1E: lea.l $82bb8(pc), a1
pub fn i_081c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C22;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82BB8;
    16
}
/// $081C22: jsr $4ae.l
pub fn i_081c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81C28);
    cpu.pc = 0x4AE;
    16
}
/// $081C28: jsr $5dd02.l
pub fn i_081c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81C2E);
    cpu.pc = 0x5DD02;
    16
}
/// $081C2E: move.w $5c(a6), d0
pub fn i_081c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C32;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $081C32: movea.l #$2e541e, a0
pub fn i_081c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C38;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E541E;
    cpu.a[0] = v1;
    4
}
/// $081C38: lsl.w #$2, d0
pub fn i_081c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C3A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $081C3A: movea.l (a0, d0.w), a0
pub fn i_081c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C3E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $081C3E: cmpa.l #$ffffffff, a0
pub fn i_081c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C44;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $081C44: beq.w $81c4e
pub fn i_081c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81C4E; return 10; }
    8
}
/// $081C48: jsr $28cd4.l
pub fn i_081c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81C4E);
    cpu.pc = 0x28CD4;
    16
}
/// $081C4E: move.w #$1033, d0
pub fn i_081c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C52;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081C52: jsr $2352.l
pub fn i_081c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81C58);
    cpu.pc = 0x2352;
    16
}
/// $081C58: lea.l $81c5e(pc), a1
pub fn i_081c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81C5E;
    16
}
/// $081C5C: move.l a1, (a6)
pub fn i_081c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081C5E: jsr $2783a.l
pub fn i_081c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81C64);
    cpu.pc = 0x2783A;
    16
}
/// $081C64: jsr $82cb2(pc)
pub fn i_081c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81C68);
    cpu.pc = 0x82CB2;
    16
}
/// $081C68: jsr $28d70.l
pub fn i_081c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81C6E);
    cpu.pc = 0x28D70;
    16
}
/// $081C6E: subq.w #$1, $70(a6)
pub fn i_081c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C72;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $081C72: cmpi.w #$3c, $70(a6)
pub fn i_081c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C78;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3C, Size::Word, false);
    16
}
/// $081C78: bne.w $81c88
pub fn i_081c78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C7C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81C88; return 10; }
    8
}
/// $081C7C: lea.l $2e5aee.l, a1
pub fn i_081c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C82;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5AEE;
    20
}
/// $081C82: jsr $77c7e.l
pub fn i_081c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81C88);
    cpu.pc = 0x77C7E;
    16
}
/// $081C88: cmpi.w #$6e, $70(a6)
pub fn i_081c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C8E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x6E, Size::Word, false);
    16
}
/// $081C8E: bne.w $81c9c
pub fn i_081c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81C9C; return 10; }
    8
}
/// $081C92: move.w #$1032, d0
pub fn i_081c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C96;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1032;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081C96: jsr $2352.l
pub fn i_081c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81C9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81C9C);
    cpu.pc = 0x2352;
    16
}
/// $081C9C: tst.w $70(a6)
pub fn i_081c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CA0;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $081CA0: bne.w $81caa
pub fn i_081ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81CAA; return 10; }
    8
}
// ===== ClrRamWord_081ca4  ($081CA4, 8 B, thunks/clr_ram_word.c)
/// $081CA4: clr.b $106ed2.l
pub fn i_081ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CAA;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED2) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED2, r2 as u8);
    16
}
/// $081CAA: rts
pub fn i_081caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00081CAC  ($081CAC, 34 B, enemies/squad/para_squad_module_081816.s)
/// $081CAC: jsr $82e7a(pc)
pub fn i_081cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CB0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81CB0);
    cpu.pc = 0x82E7A;
    16
}
/// $081CB0: jsr $82d70(pc)
pub fn i_081cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81CB4);
    cpu.pc = 0x82D70;
    16
}
/// $081CB4: jsr $2870a.l
pub fn i_081cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CBA);
    cpu.pc = 0x2870A;
    16
}
/// $081CBA: bcc.w $81cc4
pub fn i_081cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CBE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81CC4; return 10; }
    8
}
/// $081CBE: bclr.b #$3, $13(a6)
pub fn i_081cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CC4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081CC4: jsr $28758.l
pub fn i_081cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CCA);
    cpu.pc = 0x28758;
    16
}
/// $081CCA: bcc.w $81cd4
pub fn i_081cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81CD4; return 10; }
    8
}
// ===== SetTaskHandler_081cce  ($081CCE, 8 B, kernel/task_handlers.c)
/// $081CCE: lea.l $81bee(pc), a1
pub fn i_081cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81BEE;
    16
}
/// $081CD2: move.l a1, (a6)
pub fn i_081cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081CD4: rts
pub fn i_081cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CD6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081cd6  ($081CD6, 134 B, enemies/squad/para_squad_module_081816.s)
/// $081CD6: move.w #$71, d1
pub fn i_081cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CDA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x71;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081CDA: jsr $236e.l
pub fn i_081cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CE0);
    cpu.pc = 0x236E;
    16
}
/// $081CE0: move.w #$d, $1c(a6)
pub fn i_081ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CE6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081CE6: jsr $138fe.l
pub fn i_081ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CEC);
    cpu.pc = 0x138FE;
    16
}
/// $081CEC: lea.l $2be21e.l, a0
pub fn i_081cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE21E;
    20
}
/// $081CF2: jsr $799de.l
pub fn i_081cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81CF8);
    cpu.pc = 0x799DE;
    16
}
/// $081CF8: move.w d0, $66(a6)
pub fn i_081cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81CFC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081CFC: move.b $7c(a6), d0
pub fn i_081cfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D00;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $081D00: andi.w #$7, d0
pub fn i_081d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D04;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081D04: movea.l #$2e542e, a0
pub fn i_081d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D0A;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E542E;
    cpu.a[0] = v1;
    4
}
/// $081D0A: lsl.w #$2, d0
pub fn i_081d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D0C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $081D0C: movea.l (a0, d0.w), a0
pub fn i_081d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D10;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $081D10: cmpa.l #$ffffffff, a0
pub fn i_081d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D16;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $081D16: beq.w $81d20
pub fn i_081d16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81D20; return 10; }
    8
}
/// $081D1A: jsr $28cd4.l
pub fn i_081d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D20);
    cpu.pc = 0x28CD4;
    16
}
/// $081D20: lea.l $81d26(pc), a1
pub fn i_081d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81D26;
    16
}
/// $081D24: move.l a1, (a6)
pub fn i_081d24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081D26: jsr $82d40(pc)
pub fn i_081d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81D2A);
    cpu.pc = 0x82D40;
    16
}
/// $081D2A: jsr $82cc4(pc)
pub fn i_081d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D2E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81D2E);
    cpu.pc = 0x82CC4;
    16
}
/// $081D2E: jsr $28d70.l
pub fn i_081d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D34);
    cpu.pc = 0x28D70;
    16
}
/// $081D34: jsr $2870a.l
pub fn i_081d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D3A);
    cpu.pc = 0x2870A;
    16
}
/// $081D3A: bcc.w $81d44
pub fn i_081d3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81D44; return 10; }
    8
}
/// $081D3E: bclr.b #$3, $13(a6)
pub fn i_081d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D44;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081D44: jsr $28758.l
pub fn i_081d44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D4A);
    cpu.pc = 0x28758;
    16
}
/// $081D4A: bcc.w $81d54
pub fn i_081d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81D54; return 10; }
    8
}
/// $081D4E: lea.l $81d64(pc), a1
pub fn i_081d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81D64;
    16
}
/// $081D52: move.l a1, (a6)
pub fn i_081d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081D54: jsr $82ee0(pc)
pub fn i_081d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81D58);
    cpu.pc = 0x82EE0;
    16
}
/// $081D58: bcc.w $81d62
pub fn i_081d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81D62; return 10; }
    8
}
// ===== SetTaskHandler_081d5c  ($081D5C, 8 B, kernel/task_handlers.c)
/// $081D5C: lea.l $81d64(pc), a1
pub fn i_081d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81D64;
    16
}
/// $081D60: move.l a1, (a6)
pub fn i_081d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081D62: rts
pub fn i_081d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081d64  ($081D64, 68 B, enemies/squad/para_squad_module_081816.s)
/// $081D64: jsr $831da(pc)
pub fn i_081d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81D68);
    cpu.pc = 0x831DA;
    16
}
/// $081D68: bclr.b #$0, $13(a6)
pub fn i_081d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D6E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081D6E: move.l #$ffffffff, $48(a6)
pub fn i_081d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D76;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $081D76: move.b $7c(a6), d0
pub fn i_081d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D7A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $081D7A: andi.w #$7, d0
pub fn i_081d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D7E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $081D7E: movea.l #$2e5446, a0
pub fn i_081d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D84;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E5446;
    cpu.a[0] = v1;
    4
}
/// $081D84: lsl.w #$2, d0
pub fn i_081d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D86;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $081D86: movea.l (a0, d0.w), a0
pub fn i_081d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D8A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $081D8A: cmpa.l #$ffffffff, a0
pub fn i_081d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D90;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $081D90: beq.w $81d9a
pub fn i_081d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81D9A; return 10; }
    8
}
/// $081D94: jsr $28cd4.l
pub fn i_081d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D9A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81D9A);
    cpu.pc = 0x28CD4;
    16
}
/// $081D9A: lea.l $81da0(pc), a1
pub fn i_081d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81D9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81DA0;
    16
}
/// $081D9E: move.l a1, (a6)
pub fn i_081d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081DA0: jsr $82d40(pc)
pub fn i_081da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DA4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81DA4);
    cpu.pc = 0x82D40;
    16
}
/// $081DA4: jsr $82cc4(pc)
pub fn i_081da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81DA8);
    cpu.pc = 0x82CC4;
    16
}
// ===== JsrAbsThunk_081da8  ($081DA8, 8 B, thunks/jsr_abs_thunks.c)
/// $081DA8: jsr $28d70.l
pub fn i_081da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81DAE);
    cpu.pc = 0x28D70;
    16
}
/// $081DAE: rts
pub fn i_081dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081db0  ($081DB0, 568 B, enemies/squad/para_squad_module_081816.s)
/// $081DB0: move.w #$71, d1
pub fn i_081db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DB4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x71;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081DB4: jsr $236e.l
pub fn i_081db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81DBA);
    cpu.pc = 0x236E;
    16
}
/// $081DBA: move.w #$d, $1c(a6)
pub fn i_081dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xD;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081DC0: jsr $138fe.l
pub fn i_081dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81DC6);
    cpu.pc = 0x138FE;
    16
}
/// $081DC6: clr.b $7c(a6)
pub fn i_081dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DCA;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081DCA: clr.b $20(a6)
pub fn i_081dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DCE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081DCE: clr.w $80(a6)
pub fn i_081dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DD2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x80);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $081DD2: move.w #$38, $82(a6)
pub fn i_081dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DD8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x38;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081DD8: move.w #$3, $5c(a6)
pub fn i_081dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DDE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081DDE: lea.l $2be322.l, a0
pub fn i_081dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE322;
    20
}
/// $081DE4: tst.b $73(a6)
pub fn i_081de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DE8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081DE8: beq.w $81df2
pub fn i_081de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DEC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81DF2; return 10; }
    8
}
/// $081DEC: lea.l $2be3a4.l, a0
pub fn i_081dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DF2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE3A4;
    20
}
/// $081DF2: jsr $799de.l
pub fn i_081df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DF8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81DF8);
    cpu.pc = 0x799DE;
    16
}
/// $081DF8: move.w d0, $70(a6)
pub fn i_081df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81DFC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081DFC: lea.l $2e5c98.l, a0
pub fn i_081dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E02;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5C98;
    20
}
/// $081E02: jsr $28cd4.l
pub fn i_081e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81E08);
    cpu.pc = 0x28CD4;
    16
}
/// $081E08: lea.l $81e0e(pc), a1
pub fn i_081e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81E0E;
    16
}
/// $081E0C: move.l a1, (a6)
pub fn i_081e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081E0E: jsr $82ce2(pc)
pub fn i_081e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E12;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81E12);
    cpu.pc = 0x82CE2;
    16
}
/// $081E12: jsr $82d40(pc)
pub fn i_081e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E16;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81E16);
    cpu.pc = 0x82D40;
    16
}
/// $081E16: jsr $28d70.l
pub fn i_081e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81E1C);
    cpu.pc = 0x28D70;
    16
}
/// $081E1C: jsr $82f24(pc)
pub fn i_081e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81E20);
    cpu.pc = 0x82F24;
    16
}
/// $081E20: bcc.w $81e36
pub fn i_081e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E24;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81E36; return 10; }
    8
}
/// $081E24: move.b $106f28.l, d1
pub fn i_081e24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E2A;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $081E2A: andi.b #$7, d1
pub fn i_081e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E2E;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $081E2E: bne.w $81e36
pub fn i_081e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E32;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81E36; return 10; }
    8
}
/// $081E32: add.w d0, $5c(a6)
pub fn i_081e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E36;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $081E36: movea.l $c(a6), a0
pub fn i_081e36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E3A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081E3A: btst.b #$2, $72(a0)
pub fn i_081e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E40;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $081E40: beq.w $81e4a
pub fn i_081e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E44;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81E4A; return 10; }
    8
}
/// $081E44: lea.l $81e4e(pc), a1
pub fn i_081e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81E4E;
    16
}
/// $081E48: move.l a1, (a6)
pub fn i_081e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081E4A: bra.w $8202a
pub fn i_081e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E4E;
    cpu.ir = 0x6000;
    cpu.pc = 0x8202A;
    10
}
/// $081E4E: lea.l $2be62e.l, a0
pub fn i_081e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E54;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE62E;
    20
}
/// $081E54: tst.b $73(a6)
pub fn i_081e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E58;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081E58: beq.w $81e62
pub fn i_081e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E5C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81E62; return 10; }
    8
}
/// $081E5C: lea.l $2be6b0.l, a0
pub fn i_081e5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E62;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE6B0;
    20
}
/// $081E62: jsr $799de.l
pub fn i_081e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81E68);
    cpu.pc = 0x799DE;
    16
}
/// $081E68: move.w d0, $70(a6)
pub fn i_081e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E6C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081E6C: jsr $83122(pc)
pub fn i_081e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E70;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81E70);
    cpu.pc = 0x83122;
    16
}
/// $081E70: lea.l $82514(pc), a1
pub fn i_081e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82514;
    16
}
/// $081E74: jsr $4ae.l
pub fn i_081e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E7A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81E7A);
    cpu.pc = 0x4AE;
    16
}
/// $081E7A: jsr $5dd02.l
pub fn i_081e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81E80);
    cpu.pc = 0x5DD02;
    16
}
/// $081E80: move.w #$1099, d0
pub fn i_081e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E84;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1099;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $081E84: jsr $2352.l
pub fn i_081e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81E8A);
    cpu.pc = 0x2352;
    16
}
/// $081E8A: lea.l $81e90(pc), a1
pub fn i_081e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81E90;
    16
}
/// $081E8E: move.l a1, (a6)
pub fn i_081e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081E90: jsr $82f24(pc)
pub fn i_081e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E94;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81E94);
    cpu.pc = 0x82F24;
    16
}
/// $081E94: bcc.w $81eca
pub fn i_081e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81ECA; return 10; }
    8
}
/// $081E98: tst.b $73(a6)
pub fn i_081e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81E9C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081E9C: beq.w $81eca
pub fn i_081e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EA0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81ECA; return 10; }
    8
}
/// $081EA0: move.b $106f28.l, d1
pub fn i_081ea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EA6;
    cpu.ir = 0x1239;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v2 & 0xFF);
    16
}
/// $081EA6: andi.b #$7, d1
pub fn i_081ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EAA;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $081EAA: bne.w $81eca
pub fn i_081eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EAE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81ECA; return 10; }
    8
}
/// $081EAE: add.w d0, $5c(a6)
pub fn i_081eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EB2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $081EB2: jsr $5e3fc.l
pub fn i_081eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EB8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81EB8);
    cpu.pc = 0x5E3FC;
    16
}
/// $081EB8: bcc.w $81eca
pub fn i_081eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EBC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x81ECA; return 10; }
    8
}
/// $081EBC: lea.l $81ec6(pc), a1
pub fn i_081ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EC0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81EC6;
    16
}
/// $081EC0: move.l a1, (a6)
pub fn i_081ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EC2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081EC2: bra.w $81eca
pub fn i_081ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EC6;
    cpu.ir = 0x6000;
    cpu.pc = 0x81ECA;
    10
}
/// $081EC6: jsr $82f24(pc)
pub fn i_081ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ECA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81ECA);
    cpu.pc = 0x82F24;
    16
}
/// $081ECA: jsr $82ce2(pc)
pub fn i_081eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ECE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81ECE);
    cpu.pc = 0x82CE2;
    16
}
/// $081ECE: jsr $82d40(pc)
pub fn i_081ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ED2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81ED2);
    cpu.pc = 0x82D40;
    16
}
/// $081ED2: jsr $28d70.l
pub fn i_081ed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81ED8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81ED8);
    cpu.pc = 0x28D70;
    16
}
/// $081ED8: subq.w #$1, $70(a6)
pub fn i_081ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EDC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $081EDC: cmpi.w #$0, $70(a6)
pub fn i_081edc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EE2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $081EE2: bgt.w $81ef2
pub fn i_081ee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EE6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x81EF2; return 10; }
    8
}
/// $081EE6: move.b #$1, $20(a6)
pub fn i_081ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EEC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $081EEC: lea.l $81f0a(pc), a1
pub fn i_081eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81F0A;
    16
}
/// $081EF0: move.l a1, (a6)
pub fn i_081ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081EF2: cmpi.w #$f, $70(a6)
pub fn i_081ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EF8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF, Size::Word, false);
    16
}
/// $081EF8: bgt.w $81f06
pub fn i_081ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81EFC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x81F06; return 10; }
    8
}
/// $081EFC: movea.l $c(a6), a0
pub fn i_081efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F00;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081F00: bset.b #$5, $72(a0)
pub fn i_081f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F06;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081F06: bra.w $8202a
pub fn i_081f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F0A;
    cpu.ir = 0x6000;
    cpu.pc = 0x8202A;
    10
}
/// $081F0A: clr.b $20(a6)
pub fn i_081f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F0E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081F0E: movea.l $c(a6), a0
pub fn i_081f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F12;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081F12: move.b #$1, $7b(a0)
pub fn i_081f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F18;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $081F18: movea.l $c(a6), a0
pub fn i_081f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F1C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081F1C: bset.b #$5, $72(a0)
pub fn i_081f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F22;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081F22: clr.b $21(a6)
pub fn i_081f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F26;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $081F26: lea.l $82626(pc), a1
pub fn i_081f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82626;
    16
}
/// $081F2A: jsr $4ae.l
pub fn i_081f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81F30);
    cpu.pc = 0x4AE;
    16
}
/// $081F30: jsr $5dd02.l
pub fn i_081f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F36;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81F36);
    cpu.pc = 0x5DD02;
    16
}
/// $081F36: move.w $5c(a6), $5c(a0)
pub fn i_081f36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F3C;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $081F3C: lea.l $81f42(pc), a1
pub fn i_081f3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81F42;
    16
}
/// $081F40: move.l a1, (a6)
pub fn i_081f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081F42: move.w #$2, $70(a6)
pub fn i_081f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F48;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081F48: jsr $82ce2(pc)
pub fn i_081f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81F4C);
    cpu.pc = 0x82CE2;
    16
}
/// $081F4C: jsr $82d40(pc)
pub fn i_081f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81F50);
    cpu.pc = 0x82D40;
    16
}
/// $081F50: jsr $28d70.l
pub fn i_081f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81F56);
    cpu.pc = 0x28D70;
    16
}
/// $081F56: tst.b $21(a6)
pub fn i_081f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F5A;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081F5A: beq.w $81f64
pub fn i_081f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81F64; return 10; }
    8
}
/// $081F5E: lea.l $81f68(pc), a1
pub fn i_081f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81F68;
    16
}
/// $081F62: move.l a1, (a6)
pub fn i_081f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081F64: bra.w $8202a
pub fn i_081f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F68;
    cpu.ir = 0x6000;
    cpu.pc = 0x8202A;
    10
}
/// $081F68: movea.l $c(a6), a0
pub fn i_081f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F6C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081F6C: bclr.b #$2, $72(a0)
pub fn i_081f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F72;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081F72: movea.l $c(a6), a0
pub fn i_081f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F76;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081F76: bclr.b #$4, $72(a0)
pub fn i_081f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F7C;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081F7C: movea.l $c(a6), a0
pub fn i_081f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F80;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081F80: bclr.b #$5, $72(a0)
pub fn i_081f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F86;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081F86: lea.l $2be322.l, a0
pub fn i_081f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F8C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE322;
    20
}
/// $081F8C: tst.b $73(a6)
pub fn i_081f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F90;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $081F90: beq.w $81f9a
pub fn i_081f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F94;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x81F9A; return 10; }
    8
}
/// $081F94: lea.l $2be3a4.l, a0
pub fn i_081f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81F9A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE3A4;
    20
}
/// $081F9A: jsr $799de.l
pub fn i_081f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FA0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81FA0);
    cpu.pc = 0x799DE;
    16
}
/// $081FA0: move.w d0, $70(a6)
pub fn i_081fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FA4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $081FA4: jsr $82f90(pc)
pub fn i_081fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FA8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81FA8);
    cpu.pc = 0x82F90;
    16
}
/// $081FA8: bra.w $81dfc
pub fn i_081fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FAC;
    cpu.ir = 0x6000;
    cpu.pc = 0x81DFC;
    10
}
/// $081FAC: movea.l $c(a6), a0
pub fn i_081fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FB0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081FB0: bclr.b #$2, $72(a0)
pub fn i_081fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FB6;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081FB6: movea.l $c(a6), a0
pub fn i_081fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FBA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081FBA: bclr.b #$4, $72(a0)
pub fn i_081fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FC0;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $081FC0: move.b #$1, $20(a6)
pub fn i_081fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FC6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $081FC6: lea.l $81fcc(pc), a1
pub fn i_081fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81FCC;
    16
}
/// $081FCA: move.l a1, (a6)
pub fn i_081fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081FCC: jsr $82ce2(pc)
pub fn i_081fcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81FD0);
    cpu.pc = 0x82CE2;
    16
}
/// $081FD0: jsr $82d40(pc)
pub fn i_081fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x81FD4);
    cpu.pc = 0x82D40;
    16
}
/// $081FD4: jsr $28d70.l
pub fn i_081fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81FDA);
    cpu.pc = 0x28D70;
    16
}
/// $081FDA: movea.l $c(a6), a0
pub fn i_081fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FDE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $081FDE: cmpi.b #$77, $20(a0)
pub fn i_081fde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FE4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x77, Size::Byte, false);
    16
}
/// $081FE4: bne.w $81fee
pub fn i_081fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x81FEE; return 10; }
    8
}
// ===== SetTaskHandler_081fe8  ($081FE8, 8 B, kernel/task_handlers.c)
/// $081FE8: lea.l $81ff0(pc), a1
pub fn i_081fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81FF0;
    16
}
/// $081FEC: move.l a1, (a6)
pub fn i_081fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $081FEE: rts
pub fn i_081fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_081ff0  ($081FF0, 50 B, enemies/squad/para_squad_module_081816.s)
/// $081FF0: lea.l $2e5aa6.l, a1
pub fn i_081ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FF6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5AA6;
    20
}
/// $081FF6: jsr $77c7e.l
pub fn i_081ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x81FFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x81FFC);
    cpu.pc = 0x77C7E;
    16
}
/// $081FFC: lea.l $2e59fe.l, a1
pub fn i_081ffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82002;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E59FE;
    20
}
/// $082002: jsr $77c7e.l
pub fn i_082002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82008;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82008);
    cpu.pc = 0x77C7E;
    16
}
/// $082008: lea.l $2e5d20.l, a0
pub fn i_082008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8200E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5D20;
    20
}
/// $08200E: jsr $28cd4.l
pub fn i_08200e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82014);
    cpu.pc = 0x28CD4;
    16
}
/// $082014: lea.l $8201a(pc), a1
pub fn i_082014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82018;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8201A;
    16
}
/// $082018: move.l a1, (a6)
pub fn i_082018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8201A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08201A: jsr $82ce2(pc)
pub fn i_08201a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8201E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8201E);
    cpu.pc = 0x82CE2;
    16
}
/// $08201E: jsr $82d40(pc)
pub fn i_08201e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82022;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82022);
    cpu.pc = 0x82D40;
    16
}
// ===== JsrAbsThunk_082022  ($082022, 8 B, thunks/jsr_abs_thunks.c)
/// $082022: jsr $28d70.l
pub fn i_082022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82028;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82028);
    cpu.pc = 0x28D70;
    16
}
/// $082028: rts
pub fn i_082028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8202A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008202A  ($08202A, 32 B, enemies/squad/para_squad_module_081816.s)
/// $08202A: subq.w #$1, $70(a6)
pub fn i_08202a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8202E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08202E: cmpi.w #$0, $70(a6)
pub fn i_08202e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82034;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $082034: bgt.w $82042
pub fn i_082034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82038;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82042; return 10; }
    8
}
/// $082038: movea.l $c(a6), a0
pub fn i_082038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8203C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08203C: bset.b #$4, $72(a0)
pub fn i_08203c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82042;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082042: jsr $82ee0(pc)
pub fn i_082042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82046;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82046);
    cpu.pc = 0x82EE0;
    16
}
/// $082046: bcc.w $82050
pub fn i_082046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8204A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82050; return 10; }
    8
}
// ===== SetTaskHandler_08204a  ($08204A, 8 B, kernel/task_handlers.c)
/// $08204A: lea.l $81fac(pc), a1
pub fn i_08204a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8204E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x81FAC;
    16
}
/// $08204E: move.l a1, (a6)
pub fn i_08204e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82050;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082050: rts
pub fn i_082050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82052;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082052  ($082052, 936 B, enemies/squad/para_squad_module_081816.s)
/// $082052: move.w #$4d, $80(a6)
pub fn i_082052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82058;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082058: move.w #$2e, $82(a6)
pub fn i_082058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8205E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08205E: bra.w $82074
pub fn i_08205e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82062;
    cpu.ir = 0x6000;
    cpu.pc = 0x82074;
    10
}
/// $082062: move.b #$1, $3a(a6)
pub fn i_082062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82068;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $082068: move.w #$ffc0, $80(a6)
pub fn i_082068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8206E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08206E: move.w #$2a, $82(a6)
pub fn i_08206e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82074;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082074: move.w #$71, d1
pub fn i_082074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82078;
    cpu.ir = 0x323C;
    let v1: u32 = 0x71;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082078: jsr $236e.l
pub fn i_082078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8207E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8207E);
    cpu.pc = 0x236E;
    16
}
/// $08207E: move.w #$e, $1c(a6)
pub fn i_08207e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82084;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082084: jsr $138fe.l
pub fn i_082084(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8208A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8208A);
    cpu.pc = 0x138FE;
    16
}
/// $08208A: lea.l $2be19c.l, a0
pub fn i_08208a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82090;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE19C;
    20
}
/// $082090: jsr $799de.l
pub fn i_082090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82096;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82096);
    cpu.pc = 0x799DE;
    16
}
/// $082096: move.w d0, $66(a6)
pub fn i_082096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8209A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08209A: clr.b $7c(a6)
pub fn i_08209a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8209E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08209E: move.l #$2e3fca, $48(a6)
pub fn i_08209e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820A6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2E3FCA;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0820A6: lea.l $2be426.l, a0
pub fn i_0820a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE426;
    20
}
/// $0820AC: tst.b $73(a6)
pub fn i_0820ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820B0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0820B0: beq.w $820ba
pub fn i_0820b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x820BA; return 10; }
    8
}
/// $0820B4: lea.l $2be4a8.l, a0
pub fn i_0820b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820BA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE4A8;
    20
}
/// $0820BA: jsr $799de.l
pub fn i_0820ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x820C0);
    cpu.pc = 0x799DE;
    16
}
/// $0820C0: move.w d0, $70(a6)
pub fn i_0820c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820C4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0820C4: clr.w $5c(a6)
pub fn i_0820c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820C8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0820C8: lea.l $2e5d2c.l, a0
pub fn i_0820c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5D2C;
    20
}
/// $0820CE: jsr $28cd4.l
pub fn i_0820ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x820D4);
    cpu.pc = 0x28CD4;
    16
}
/// $0820D4: lea.l $820da(pc), a1
pub fn i_0820d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820D8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x820DA;
    16
}
/// $0820D8: move.l a1, (a6)
pub fn i_0820d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820DA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0820DA: jsr $82ce2(pc)
pub fn i_0820da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x820DE);
    cpu.pc = 0x82CE2;
    16
}
/// $0820DE: jsr $28d70.l
pub fn i_0820de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x820E4);
    cpu.pc = 0x28D70;
    16
}
/// $0820E4: jsr $82ef8(pc)
pub fn i_0820e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820E8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x820E8);
    cpu.pc = 0x82EF8;
    16
}
/// $0820E8: lea.l $2e5846.l, a0
pub fn i_0820e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5846;
    20
}
/// $0820EE: movea.l #$ffffffff, a1
pub fn i_0820ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820F4;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $0820F4: jsr $772.l
pub fn i_0820f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x820FA);
    cpu.pc = 0x772;
    16
}
/// $0820FA: bra.w $82402
pub fn i_0820fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x820FE;
    cpu.ir = 0x6000;
    cpu.pc = 0x82402;
    10
}
/// $0820FE: tst.b $73(a6)
pub fn i_0820fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82102;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082102: bne.w $82160
pub fn i_082102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82106;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82160; return 10; }
    8
}
/// $082106: move.w #$1e, $70(a6)
pub fn i_082106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8210C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08210C: lea.l $2e5d2c.l, a0
pub fn i_08210c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82112;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5D2C;
    20
}
/// $082112: jsr $28cd4.l
pub fn i_082112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82118;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82118);
    cpu.pc = 0x28CD4;
    16
}
/// $082118: lea.l $8211e(pc), a1
pub fn i_082118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8211C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8211E;
    16
}
/// $08211C: move.l a1, (a6)
pub fn i_08211c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8211E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08211E: movea.l $c(a6), a0
pub fn i_08211e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82122;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082122: bset.b #$0, $72(a0)
pub fn i_082122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82128;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082128: jsr $82ce2(pc)
pub fn i_082128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8212C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8212C);
    cpu.pc = 0x82CE2;
    16
}
/// $08212C: jsr $28d70.l
pub fn i_08212c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82132;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82132);
    cpu.pc = 0x28D70;
    16
}
/// $082132: subq.w #$1, $70(a6)
pub fn i_082132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82136;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082136: cmpi.w #$0, $70(a6)
pub fn i_082136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8213C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08213C: bgt.w $8215c
pub fn i_08213c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82140;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8215C; return 10; }
    8
}
/// $082140: addq.w #$1, $5c(a6)
pub fn i_082140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82144;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082144: move.w #$1e, $70(a6)
pub fn i_082144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8214A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08214A: move.w $5c(a6), d0
pub fn i_08214a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8214E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08214E: andi.w #$1, d0
pub fn i_08214e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82152;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082152: bne.w $8215c
pub fn i_082152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82156;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8215C; return 10; }
    8
}
/// $082156: lea.l $82160(pc), a1
pub fn i_082156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8215A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82160;
    16
}
/// $08215A: move.l a1, (a6)
pub fn i_08215a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8215C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08215C: bra.w $82402
pub fn i_08215c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82160;
    cpu.ir = 0x6000;
    cpu.pc = 0x82402;
    10
}
/// $082160: jsr $82fca(pc)
pub fn i_082160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82164;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82164);
    cpu.pc = 0x82FCA;
    16
}
/// $082164: move.w $5c(a6), d0
pub fn i_082164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82168;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082168: movea.l #$2e5832, a0
pub fn i_082168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8216E;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E5832;
    cpu.a[0] = v1;
    4
}
/// $08216E: lsl.w #$2, d0
pub fn i_08216e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82170;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $082170: movea.l (a0, d0.w), a0
pub fn i_082170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82174;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $082174: cmpa.l #$ffffffff, a0
pub fn i_082174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8217A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $08217A: beq.w $82184
pub fn i_08217a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8217E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82184; return 10; }
    8
}
/// $08217E: jsr $28cd4.l
pub fn i_08217e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82184);
    cpu.pc = 0x28CD4;
    16
}
/// $082184: lea.l $8218a(pc), a1
pub fn i_082184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82188;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8218A;
    16
}
/// $082188: move.l a1, (a6)
pub fn i_082188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8218A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08218A: movea.l $c(a6), a0
pub fn i_08218a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8218E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08218E: bset.b #$0, $72(a0)
pub fn i_08218e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82194;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082194: jsr $82ce2(pc)
pub fn i_082194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82198;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82198);
    cpu.pc = 0x82CE2;
    16
}
/// $082198: jsr $28d70.l
pub fn i_082198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8219E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8219E);
    cpu.pc = 0x28D70;
    16
}
/// $08219E: bcc.w $821b8
pub fn i_08219e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821A2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x821B8; return 10; }
    8
}
/// $0821A2: lea.l $8210c(pc), a1
pub fn i_0821a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8210C;
    16
}
/// $0821A6: move.l a1, (a6)
pub fn i_0821a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0821A8: cmpi.w #$4, $5c(a6)
pub fn i_0821a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821AE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Word, false);
    16
}
/// $0821AE: blt.w $821b8
pub fn i_0821ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821B2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x821B8; return 10; }
    8
}
/// $0821B2: lea.l $821bc(pc), a1
pub fn i_0821b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821B6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x821BC;
    16
}
/// $0821B6: move.l a1, (a6)
pub fn i_0821b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821B8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0821B8: bra.w $82402
pub fn i_0821b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x82402;
    10
}
/// $0821BC: lea.l $2e5d2c.l, a0
pub fn i_0821bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821C2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5D2C;
    20
}
/// $0821C2: jsr $28cd4.l
pub fn i_0821c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x821C8);
    cpu.pc = 0x28CD4;
    16
}
/// $0821C8: lea.l $821ce(pc), a1
pub fn i_0821c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x821CE;
    16
}
/// $0821CC: move.l a1, (a6)
pub fn i_0821cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0821CE: jsr $82ce2(pc)
pub fn i_0821ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821D2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x821D2);
    cpu.pc = 0x82CE2;
    16
}
/// $0821D2: jsr $28d70.l
pub fn i_0821d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x821D8);
    cpu.pc = 0x28D70;
    16
}
/// $0821D8: move.b $106f28.l, d0
pub fn i_0821d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821DE;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0821DE: andi.b #$f, d0
pub fn i_0821de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821E2;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xF) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0821E2: bne.w $821f8
pub fn i_0821e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821E6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x821F8; return 10; }
    8
}
/// $0821E6: subq.w #$1, $5c(a6)
pub fn i_0821e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821EA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0821EA: tst.w $5c(a6)
pub fn i_0821ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821EE;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0821EE: bne.w $821f8
pub fn i_0821ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821F2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x821F8; return 10; }
    8
}
/// $0821F2: lea.l $82344(pc), a1
pub fn i_0821f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82344;
    16
}
/// $0821F6: move.l a1, (a6)
pub fn i_0821f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0821F8: bra.w $82402
pub fn i_0821f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x821FC;
    cpu.ir = 0x6000;
    cpu.pc = 0x82402;
    10
}
/// $0821FC: lea.l $2e5e94.l, a0
pub fn i_0821fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82202;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5E94;
    20
}
/// $082202: jsr $28cd4.l
pub fn i_082202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82208;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82208);
    cpu.pc = 0x28CD4;
    16
}
/// $082208: lea.l $8220e(pc), a1
pub fn i_082208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8220C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8220E;
    16
}
/// $08220C: move.l a1, (a6)
pub fn i_08220c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8220E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08220E: jsr $82ce2(pc)
pub fn i_08220e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82212;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82212);
    cpu.pc = 0x82CE2;
    16
}
/// $082212: jsr $28d70.l
pub fn i_082212(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82218;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82218);
    cpu.pc = 0x28D70;
    16
}
/// $082218: bcc.w $82222
pub fn i_082218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8221C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82222; return 10; }
    8
}
/// $08221C: lea.l $82226(pc), a1
pub fn i_08221c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82220;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82226;
    16
}
/// $082220: move.l a1, (a6)
pub fn i_082220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82222;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082222: bra.w $82402
pub fn i_082222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82226;
    cpu.ir = 0x6000;
    cpu.pc = 0x82402;
    10
}
/// $082226: lea.l $2be836.l, a0
pub fn i_082226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8222C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE836;
    20
}
/// $08222C: tst.b $73(a6)
pub fn i_08222c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82230;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082230: beq.w $8223a
pub fn i_082230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82234;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8223A; return 10; }
    8
}
/// $082234: lea.l $2be8b8.l, a0
pub fn i_082234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8223A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE8B8;
    20
}
/// $08223A: jsr $799de.l
pub fn i_08223a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82240;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82240);
    cpu.pc = 0x799DE;
    16
}
/// $082240: move.b d0, $7f(a6)
pub fn i_082240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82244;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $082244: lea.l $2be93a.l, a0
pub fn i_082244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8224A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE93A;
    20
}
/// $08224A: tst.b $73(a6)
pub fn i_08224a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8224E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08224E: beq.w $82258
pub fn i_08224e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82252;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82258; return 10; }
    8
}
/// $082252: lea.l $2be9bc.l, a0
pub fn i_082252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82258;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE9BC;
    20
}
/// $082258: jsr $799de.l
pub fn i_082258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8225E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8225E);
    cpu.pc = 0x799DE;
    16
}
/// $08225E: move.w d0, $70(a6)
pub fn i_08225e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82262;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082262: clr.w $5e(a6)
pub fn i_082262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82266;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $082266: clr.b $20(a6)
pub fn i_082266(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8226A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $08226A: lea.l $828ee(pc), a1
pub fn i_08226a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8226E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x828EE;
    16
}
/// $08226E: jsr $4ae.l
pub fn i_08226e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82274;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82274);
    cpu.pc = 0x4AE;
    16
}
/// $082274: jsr $5dd02.l
pub fn i_082274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8227A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8227A);
    cpu.pc = 0x5DD02;
    16
}
/// $08227A: lea.l $2e5f4a.l, a0
pub fn i_08227a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82280;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5F4A;
    20
}
/// $082280: jsr $28cd4.l
pub fn i_082280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82286;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82286);
    cpu.pc = 0x28CD4;
    16
}
/// $082286: lea.l $8228c(pc), a1
pub fn i_082286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8228A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8228C;
    16
}
/// $08228A: move.l a1, (a6)
pub fn i_08228a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8228C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08228C: movea.l $c(a6), a0
pub fn i_08228c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82290;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082290: bset.b #$1, $72(a0)
pub fn i_082290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82296;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082296: jsr $82ce2(pc)
pub fn i_082296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8229A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8229A);
    cpu.pc = 0x82CE2;
    16
}
/// $08229A: jsr $28d70.l
pub fn i_08229a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x822A0);
    cpu.pc = 0x28D70;
    16
}
/// $0822A0: subq.w #$1, $70(a6)
pub fn i_0822a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822A4;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0822A4: cmpi.w #$0, $70(a6)
pub fn i_0822a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822AA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0822AA: bgt.w $82310
pub fn i_0822aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822AE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82310; return 10; }
    8
}
/// $0822AE: jsr $83064(pc)
pub fn i_0822ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x822B2);
    cpu.pc = 0x83064;
    16
}
/// $0822B2: lea.l $2be93a.l, a0
pub fn i_0822b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822B8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE93A;
    20
}
/// $0822B8: tst.b $73(a6)
pub fn i_0822b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822BC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0822BC: beq.w $822c6
pub fn i_0822bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x822C6; return 10; }
    8
}
/// $0822C0: lea.l $2be9bc.l, a0
pub fn i_0822c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE9BC;
    20
}
/// $0822C6: jsr $799de.l
pub fn i_0822c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x822CC);
    cpu.pc = 0x799DE;
    16
}
/// $0822CC: move.w d0, $70(a6)
pub fn i_0822cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822D0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0822D0: subq.b #$1, $7f(a6)
pub fn i_0822d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822D4;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7F);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0822D4: cmpi.b #$0, $7f(a6)
pub fn i_0822d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822DA;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0822DA: bgt.w $82310
pub fn i_0822da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822DE;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82310; return 10; }
    8
}
/// $0822DE: lea.l $2be836.l, a0
pub fn i_0822de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE836;
    20
}
/// $0822E4: tst.b $73(a6)
pub fn i_0822e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822E8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $0822E8: beq.w $822f2
pub fn i_0822e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822EC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x822F2; return 10; }
    8
}
/// $0822EC: lea.l $2be8b8.l, a0
pub fn i_0822ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE8B8;
    20
}
/// $0822F2: jsr $799de.l
pub fn i_0822f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x822F8);
    cpu.pc = 0x799DE;
    16
}
/// $0822F8: move.b d0, $7f(a6)
pub fn i_0822f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x822FC;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7F);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0822FC: addq.w #$1, $5e(a6)
pub fn i_0822fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82300;
    cpu.ir = 0x526E;
    let ad1 = cpu.a[6].wrapping_add(0x5E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082300: cmpi.w #$5, $5e(a6)
pub fn i_082300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82306;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x5, Size::Word, false);
    16
}
/// $082306: bne.w $82310
pub fn i_082306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8230A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82310; return 10; }
    8
}
/// $08230A: lea.l $82314(pc), a1
pub fn i_08230a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8230E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82314;
    16
}
/// $08230E: move.l a1, (a6)
pub fn i_08230e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82310;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082310: bra.w $82402
pub fn i_082310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82314;
    cpu.ir = 0x6000;
    cpu.pc = 0x82402;
    10
}
/// $082314: move.b #$1, $20(a6)
pub fn i_082314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8231A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08231A: lea.l $2e60a0.l, a0
pub fn i_08231a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82320;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E60A0;
    20
}
/// $082320: jsr $28cd4.l
pub fn i_082320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82326;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82326);
    cpu.pc = 0x28CD4;
    16
}
/// $082326: lea.l $8232c(pc), a1
pub fn i_082326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8232A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8232C;
    16
}
/// $08232A: move.l a1, (a6)
pub fn i_08232a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8232C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08232C: jsr $82ce2(pc)
pub fn i_08232c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82330;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82330);
    cpu.pc = 0x82CE2;
    16
}
/// $082330: jsr $28d70.l
pub fn i_082330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82336;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82336);
    cpu.pc = 0x28D70;
    16
}
/// $082336: bcc.w $82340
pub fn i_082336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8233A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82340; return 10; }
    8
}
/// $08233A: lea.l $82344(pc), a1
pub fn i_08233a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8233E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82344;
    16
}
/// $08233E: move.l a1, (a6)
pub fn i_08233e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82340;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082340: bra.w $82402
pub fn i_082340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82344;
    cpu.ir = 0x6000;
    cpu.pc = 0x82402;
    10
}
/// $082344: movea.l $c(a6), a0
pub fn i_082344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82348;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082348: bclr.b #$0, $72(a0)
pub fn i_082348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8234E;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08234E: movea.l $c(a6), a0
pub fn i_08234e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82352;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082352: bclr.b #$1, $72(a0)
pub fn i_082352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82358;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082358: movea.l $c(a6), a0
pub fn i_082358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8235C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08235C: bclr.b #$3, $72(a0)
pub fn i_08235c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82362;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082362: lea.l $2be426.l, a0
pub fn i_082362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82368;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE426;
    20
}
/// $082368: tst.b $73(a6)
pub fn i_082368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8236C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $08236C: beq.w $82376
pub fn i_08236c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82370;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82376; return 10; }
    8
}
/// $082370: lea.l $2be4a8.l, a0
pub fn i_082370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82376;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE4A8;
    20
}
/// $082376: jsr $799de.l
pub fn i_082376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8237C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8237C);
    cpu.pc = 0x799DE;
    16
}
/// $08237C: move.w d0, $70(a6)
pub fn i_08237c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82380;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082380: jsr $82f90(pc)
pub fn i_082380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82384;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82384);
    cpu.pc = 0x82F90;
    16
}
/// $082384: bra.w $820c4
pub fn i_082384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82388;
    cpu.ir = 0x6000;
    cpu.pc = 0x820C4;
    10
}
/// $082388: movea.l $c(a6), a0
pub fn i_082388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8238C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08238C: bclr.b #$0, $72(a0)
pub fn i_08238c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82392;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082392: movea.l $c(a6), a0
pub fn i_082392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82396;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082396: bclr.b #$1, $72(a0)
pub fn i_082396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8239C;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08239C: movea.l $c(a6), a0
pub fn i_08239c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823A0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0823A0: bclr.b #$3, $72(a0)
pub fn i_0823a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823A6;
    cpu.ir = 0x8A8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0823A6: clr.w d0
pub fn i_0823a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823A8;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0823A8: btst.b #$0, $3a(a6)
pub fn i_0823a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823AE;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0823AE: beq.w $823b6
pub fn i_0823ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823B2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x823B6; return 10; }
    8
}
/// $0823B2: move.w #$1, d0
pub fn i_0823b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823B6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0823B6: movea.l $c(a6), a0
pub fn i_0823b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823BA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0823BA: bclr.b d0, $7d(a0)
pub fn i_0823ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823BE;
    cpu.ir = 0x1A8;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x7D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0823BE: move.b #$1, $20(a6)
pub fn i_0823be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0823C4: move.l #$ffffffff, $48(a6)
pub fn i_0823c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823CC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0823CC: lea.l $77fd6.l, a1
pub fn i_0823cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $0823D2: jsr $4ae.l
pub fn i_0823d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x823D8);
    cpu.pc = 0x4AE;
    16
}
/// $0823D8: jsr $5dd02.l
pub fn i_0823d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x823DE);
    cpu.pc = 0x5DD02;
    16
}
/// $0823DE: addi.w #$8, $24(a0)
pub fn i_0823de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823E4;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0823E4: lea.l $2e6142.l, a0
pub fn i_0823e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823EA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6142;
    20
}
/// $0823EA: jsr $28cd4.l
pub fn i_0823ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x823F0);
    cpu.pc = 0x28CD4;
    16
}
/// $0823F0: lea.l $823f6(pc), a1
pub fn i_0823f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823F4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x823F6;
    16
}
/// $0823F4: move.l a1, (a6)
pub fn i_0823f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823F6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0823F6: jsr $82ce2(pc)
pub fn i_0823f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x823FA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x823FA);
    cpu.pc = 0x82CE2;
    16
}
// ===== JsrAbsThunk_0823fa  ($0823FA, 8 B, thunks/jsr_abs_thunks.c)
/// $0823FA: jsr $28d70.l
pub fn i_0823fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82400;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82400);
    cpu.pc = 0x28D70;
    16
}
/// $082400: rts
pub fn i_082400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82402;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082402  ($082402, 76 B, enemies/squad/para_squad_module_081816.s)
/// $082402: subq.w #$1, $70(a6)
pub fn i_082402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82406;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082406: cmpi.w #$0, $70(a6)
pub fn i_082406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8240C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $08240C: bgt.w $8241a
pub fn i_08240c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82410;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x8241A; return 10; }
    8
}
/// $082410: movea.l $c(a6), a0
pub fn i_082410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82414;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082414: bset.b #$3, $72(a0)
pub fn i_082414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8241A;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08241A: jsr $2870a.l
pub fn i_08241a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82420);
    cpu.pc = 0x2870A;
    16
}
/// $082420: bcc.w $82436
pub fn i_082420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82436; return 10; }
    8
}
/// $082424: bclr.b #$3, $13(a6)
pub fn i_082424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8242A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08242A: lea.l $5e766.l, a0
pub fn i_08242a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82430;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x5E766;
    20
}
/// $082430: jsr $5e770.l
pub fn i_082430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82436;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82436);
    cpu.pc = 0x5E770;
    16
}
/// $082436: jsr $82ee0(pc)
pub fn i_082436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8243A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x8243A);
    cpu.pc = 0x82EE0;
    16
}
/// $08243A: bcc.w $82444
pub fn i_08243a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8243E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82444; return 10; }
    8
}
/// $08243E: lea.l $82388(pc), a1
pub fn i_08243e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82442;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82388;
    16
}
/// $082442: move.l a1, (a6)
pub fn i_082442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82444;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082444: jsr $28758.l
pub fn i_082444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8244A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8244A);
    cpu.pc = 0x28758;
    16
}
/// $08244A: bcc.w $82454
pub fn i_08244a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8244E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82454; return 10; }
    8
}
// ===== SetTaskHandler_08244e  ($08244E, 8 B, kernel/task_handlers.c)
/// $08244E: lea.l $82388(pc), a1
pub fn i_08244e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82452;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82388;
    16
}
/// $082452: move.l a1, (a6)
pub fn i_082452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82454;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082454: rts
pub fn i_082454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82456;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_082456  ($082456, 14 B, kernel/jsr5b6_jmp518.c)
/// $082456: jsr $5b6.l
pub fn i_082456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8245C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8245C);
    cpu.pc = 0x5B6;
    16
}
/// $08245C: jmp $518.l
pub fn i_08245c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82462;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $082462: rts
pub fn i_082462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82464;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_082464  ($082464, 8 B, kernel/jmp_to_scheduler.c)
/// $082464: jmp $518.l
pub fn i_082464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8246A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08246A: rts
pub fn i_08246a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8246C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08246c  ($08246C, 160 B, enemies/squad/para_squad_module_081816.s)
/// $08246C: jsr $13600.l
pub fn i_08246c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82472;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82472);
    cpu.pc = 0x13600;
    16
}
/// $082472: move.l #$ffffffff, $48(a6)
pub fn i_082472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8247A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $08247A: jmp $77f6a.l
pub fn i_08247a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82480;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x77F6A;
    8
}
/// $082480: move.w #$74, d1
pub fn i_082480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82484;
    cpu.ir = 0x323C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082484: jsr $236e.l
pub fn i_082484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8248A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8248A);
    cpu.pc = 0x236E;
    16
}
/// $08248A: lea.l $2e54a2.l, a0
pub fn i_08248a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82490;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E54A2;
    20
}
/// $082490: move.w $5c(a6), d0
pub fn i_082490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82494;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082494: lsl.w #$2, d0
pub fn i_082494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82496;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $082496: movea.l (a0, d0.w), a0
pub fn i_082496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8249A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $08249A: move.w $5e(a6), d0
pub fn i_08249a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8249E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08249E: lsl.w #$2, d0
pub fn i_08249e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824A0;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0824A0: move.w (a0, d0.w), d1
pub fn i_0824a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824A4;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0824A4: move.w $2(a0, d0.w), d2
pub fn i_0824a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824A8;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0824A8: move.w d1, $80(a6)
pub fn i_0824a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824AC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0824AC: move.w d2, $82(a6)
pub fn i_0824ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824B0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0824B0: move.w $5e(a6), d0
pub fn i_0824b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824B4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0824B4: movea.l #$2e547a, a0
pub fn i_0824b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824BA;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E547A;
    cpu.a[0] = v1;
    4
}
/// $0824BA: lsl.w #$2, d0
pub fn i_0824ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824BC;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0824BC: movea.l (a0, d0.w), a0
pub fn i_0824bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824C0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0824C0: cmpa.l #$ffffffff, a0
pub fn i_0824c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824C6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0824C6: beq.w $824d0
pub fn i_0824c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824CA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x824D0; return 10; }
    8
}
/// $0824CA: jsr $28cd4.l
pub fn i_0824ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x824D0);
    cpu.pc = 0x28CD4;
    16
}
/// $0824D0: lea.l $824d6(pc), a1
pub fn i_0824d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824D4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x824D6;
    16
}
/// $0824D4: move.l a1, (a6)
pub fn i_0824d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824D6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0824D6: jsr $5e506.l
pub fn i_0824d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x824DC);
    cpu.pc = 0x5E506;
    16
}
/// $0824DC: move.w $80(a6), d1
pub fn i_0824dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824E0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0824E0: move.w $82(a6), d2
pub fn i_0824e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824E4;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0824E4: add.w d1, $22(a6)
pub fn i_0824e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824E8;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0824E8: add.w d2, $24(a6)
pub fn i_0824e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824EC;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0824EC: jsr $28d70.l
pub fn i_0824ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x824F2);
    cpu.pc = 0x28D70;
    16
}
/// $0824F2: bcc.w $824fc
pub fn i_0824f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824F6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x824FC; return 10; }
    8
}
/// $0824F6: lea.l $82456(pc), a1
pub fn i_0824f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824FA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $0824FA: move.l a1, (a6)
pub fn i_0824fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x824FC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0824FC: movea.l #$ffffffff, a0
pub fn i_0824fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82502;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $082502: jsr $5dd56.l
pub fn i_082502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82508;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82508);
    cpu.pc = 0x5DD56;
    16
}
/// $082508: bcc.w $82512
pub fn i_082508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8250C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82512; return 10; }
    8
}
// ===== SetTaskHandler_08250c  ($08250C, 8 B, kernel/task_handlers.c)
/// $08250C: lea.l $82456(pc), a1
pub fn i_08250c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82510;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082510: move.l a1, (a6)
pub fn i_082510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82512;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082512: rts
pub fn i_082512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82514;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082514  ($082514, 70 B, enemies/squad/para_squad_module_081816.s)
/// $082514: jsr $5e1aa.l
pub fn i_082514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8251A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8251A);
    cpu.pc = 0x5E1AA;
    16
}
/// $08251A: move.b d0, $7a(a6)
pub fn i_08251a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8251E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08251E: move.w #$74, d1
pub fn i_08251e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82522;
    cpu.ir = 0x323C;
    let v1: u32 = 0x74;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082522: jsr $236e.l
pub fn i_082522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82528;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82528);
    cpu.pc = 0x236E;
    16
}
/// $082528: lea.l $2e62fc.l, a0
pub fn i_082528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8252E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E62FC;
    20
}
/// $08252E: jsr $28cd4.l
pub fn i_08252e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82534;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82534);
    cpu.pc = 0x28CD4;
    16
}
/// $082534: lea.l $8253a(pc), a1
pub fn i_082534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82538;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8253A;
    16
}
/// $082538: move.l a1, (a6)
pub fn i_082538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8253A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08253A: jsr $5e506.l
pub fn i_08253a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82540;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82540);
    cpu.pc = 0x5E506;
    16
}
/// $082540: move.w $5c(a0), $5c(a6)
pub fn i_082540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82546;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082546: subi.w #$38, $24(a6)
pub fn i_082546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8254C;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $08254C: jsr $28d70.l
pub fn i_08254c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82552;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82552);
    cpu.pc = 0x28D70;
    16
}
/// $082552: jsr $82ee0(pc)
pub fn i_082552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82556;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82556);
    cpu.pc = 0x82EE0;
    16
}
/// $082556: bcc.w $82560
pub fn i_082556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8255A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82560; return 10; }
    8
}
// ===== SetTaskHandler_08255a  ($08255A, 8 B, kernel/task_handlers.c)
/// $08255A: lea.l $82464(pc), a1
pub fn i_08255a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8255E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $08255E: move.l a1, (a6)
pub fn i_08255e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82560;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082560: rts
pub fn i_082560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82562;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082562  ($082562, 188 B, enemies/squad/para_squad_module_081816.s)
/// $082562: lea.l $2e55d6.l, a0
pub fn i_082562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82568;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E55D6;
    20
}
/// $082568: move.w $5c(a6), d0
pub fn i_082568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8256C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08256C: lsl.w #$3, d0
pub fn i_08256c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8256E;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $08256E: move.w $5e(a6), d1
pub fn i_08256e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82572;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082572: add.w d1, d1
pub fn i_082572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82574;
    cpu.ir = 0xD241;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082574: add.w d1, d0
pub fn i_082574(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82576;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082576: move.w (a0, d0.w), d2
pub fn i_082576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8257A;
    cpu.ir = 0x3430;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08257A: add.w d2, $22(a6)
pub fn i_08257a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8257E;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08257E: subi.w #$18, $24(a6)
pub fn i_08257e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82584;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x18, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $082584: jsr $5e9b6.l
pub fn i_082584(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8258A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8258A);
    cpu.pc = 0x5E9B6;
    16
}
/// $08258A: andi.w #$f, d0
pub fn i_08258a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8258E;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08258E: move.w d0, d1
pub fn i_08258e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82590;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082590: add.w d0, d0
pub fn i_082590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82592;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082592: add.w d1, d0
pub fn i_082592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82594;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082594: add.w d0, d0
pub fn i_082594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82596;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082596: lea.l $2e560e.l, a1
pub fn i_082596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8259C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E560E;
    20
}
/// $08259C: move.w $5c(a6), d1
pub fn i_08259c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825A0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0825A0: lsl.w #$2, d1
pub fn i_0825a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825A2;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0825A2: movea.l (a1, d1.w), a0
pub fn i_0825a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825A6;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $0825A6: move.w (a0, d0.w), $28(a6)
pub fn i_0825a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825AC;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0825AC: move.w $2(a0, d0.w), $2e(a6)
pub fn i_0825ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825B2;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0825B2: move.w $4(a0, d0.w), $2a(a6)
pub fn i_0825b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825B8;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0825B8: neg.w $2a(a6)
pub fn i_0825b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825BC;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0825BC: neg.w $2e(a6)
pub fn i_0825bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825C0;
    cpu.ir = 0x446E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    let r3 = 0u32.wrapping_sub(v2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.set(StatusRegister::V, v2 == 0x8000); cpu.sr.set(StatusRegister::C, v2 != 0); cpu.sr.set(StatusRegister::X, v2 != 0);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0825C0: move.w #$76, d1
pub fn i_0825c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825C4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0825C4: jsr $236e.l
pub fn i_0825c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825CA);
    cpu.pc = 0x236E;
    16
}
/// $0825CA: move.w #$c000, d0
pub fn i_0825ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825CE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0825CE: jsr $28134.l
pub fn i_0825ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825D4);
    cpu.pc = 0x28134;
    16
}
/// $0825D4: andi.w #$ffe3, $38(a6)
pub fn i_0825d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825DA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0825DA: ori.w #$14, $38(a6)
pub fn i_0825da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825E0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0825E0: lea.l $2e69e4.l, a0
pub fn i_0825e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825E6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E69E4;
    20
}
/// $0825E6: jsr $28cd4.l
pub fn i_0825e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825EC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825EC);
    cpu.pc = 0x28CD4;
    16
}
/// $0825EC: lea.l $825f2(pc), a1
pub fn i_0825ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825F0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x825F2;
    16
}
/// $0825F0: move.l a1, (a6)
pub fn i_0825f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825F2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0825F2: jsr $27d50.l
pub fn i_0825f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825F8);
    cpu.pc = 0x27D50;
    16
}
/// $0825F8: jsr $28d70.l
pub fn i_0825f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x825FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x825FE);
    cpu.pc = 0x28D70;
    16
}
/// $0825FE: bcc.w $82608
pub fn i_0825fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82602;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82608; return 10; }
    8
}
/// $082602: lea.l $82456(pc), a1
pub fn i_082602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82606;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082606: move.l a1, (a6)
pub fn i_082606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82608;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082608: movea.l #$ffffffff, a0
pub fn i_082608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8260E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08260E: lea.l $2e5416.l, a0
pub fn i_08260e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82614;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5416;
    20
}
/// $082614: jsr $5dd56.l
pub fn i_082614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8261A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8261A);
    cpu.pc = 0x5DD56;
    16
}
/// $08261A: bcc.w $82624
pub fn i_08261a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8261E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82624; return 10; }
    8
}
// ===== SetTaskHandler_08261e  ($08261E, 8 B, kernel/task_handlers.c)
/// $08261E: lea.l $82456(pc), a1
pub fn i_08261e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82622;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082622: move.l a1, (a6)
pub fn i_082622(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82624;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082624: rts
pub fn i_082624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82626;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082626  ($082626, 78 B, enemies/squad/para_squad_module_081816.s)
/// $082626: move.w #$75, d1
pub fn i_082626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8262A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x75;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08262A: jsr $236e.l
pub fn i_08262a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82630;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82630);
    cpu.pc = 0x236E;
    16
}
/// $082630: lea.l $2e6152.l, a0
pub fn i_082630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82636;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6152;
    20
}
/// $082636: jsr $28cd4.l
pub fn i_082636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8263C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8263C);
    cpu.pc = 0x28CD4;
    16
}
/// $08263C: lea.l $82642(pc), a1
pub fn i_08263c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82640;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82642;
    16
}
/// $082640: move.l a1, (a6)
pub fn i_082640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82642;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082642: jsr $5e506.l
pub fn i_082642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82648;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82648);
    cpu.pc = 0x5E506;
    16
}
/// $082648: subq.w #$1, $38(a6)
pub fn i_082648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8264C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08264C: jsr $28d70.l
pub fn i_08264c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82652);
    cpu.pc = 0x28D70;
    16
}
/// $082652: bcc.w $82666
pub fn i_082652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82656;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82666; return 10; }
    8
}
/// $082656: movea.l $c(a6), a0
pub fn i_082656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8265A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $08265A: move.b #$1, $21(a0)
pub fn i_08265a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82660;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $082660: lea.l $82464(pc), a1
pub fn i_082660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82664;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $082664: move.l a1, (a6)
pub fn i_082664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82666;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082666: jsr $283d8.l
pub fn i_082666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8266C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8266C);
    cpu.pc = 0x283D8;
    16
}
/// $08266C: jsr $82ee0(pc)
pub fn i_08266c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82670;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82670);
    cpu.pc = 0x82EE0;
    16
}
/// $082670: bcc.w $8267a
pub fn i_082670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82674;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8267A; return 10; }
    8
}
// ===== SetTaskHandler_082674  ($082674, 8 B, kernel/task_handlers.c)
/// $082674: lea.l $82464(pc), a1
pub fn i_082674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82678;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $082678: move.l a1, (a6)
pub fn i_082678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8267A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08267A: rts
pub fn i_08267a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8267C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08267c  ($08267C, 440 B, enemies/squad/para_squad_module_081816.s)
/// $08267C: clr.w $70(a6)
pub fn i_08267c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82680;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $082680: lea.l $2e571a.l, a0
pub fn i_082680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82686;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E571A;
    20
}
/// $082686: move.w $5c(a6), d0
pub fn i_082686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8268A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08268A: lsl.w #$2, d0
pub fn i_08268a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8268C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08268C: move.l (a0, d0.w), $5c(a6)
pub fn i_08268c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82692;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $082692: move.b #$6, $7a(a6)
pub fn i_082692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82698;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $082698: lea.l $8269e(pc), a1
pub fn i_082698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8269C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8269E;
    16
}
/// $08269C: move.l a1, (a6)
pub fn i_08269c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8269E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08269E: btst.b #$0, $70(a6)
pub fn i_08269e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826A4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0826A4: bne.w $8271a
pub fn i_0826a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8271A; return 10; }
    8
}
/// $0826A8: move.w #$4000, d0
pub fn i_0826a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826AC;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0826AC: jsr $28134.l
pub fn i_0826ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x826B2);
    cpu.pc = 0x28134;
    16
}
/// $0826B2: andi.w #$ffe3, $38(a6)
pub fn i_0826b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826B8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0826B8: ori.w #$14, $38(a6)
pub fn i_0826b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826BE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0826BE: subq.b #$1, $7a(a6)
pub fn i_0826be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826C2;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $0826C2: bne.w $826cc
pub fn i_0826c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826C6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x826CC; return 10; }
    8
}
/// $0826C6: lea.l $82464(pc), a1
pub fn i_0826c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $0826CA: move.l a1, (a6)
pub fn i_0826ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0826CC: lea.l $77f22.l, a1
pub fn i_0826cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826D2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F22;
    20
}
/// $0826D2: jsr $4ae.l
pub fn i_0826d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x826D8);
    cpu.pc = 0x4AE;
    16
}
/// $0826D8: jsr $5dd02.l
pub fn i_0826d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x826DE);
    cpu.pc = 0x5DD02;
    16
}
/// $0826DE: movea.l $5c(a6), a1
pub fn i_0826de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826E2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0826E2: move.w (a1), d0
pub fn i_0826e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826E4;
    cpu.ir = 0x3011;
    let v2 = bus.read16(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $0826E4: move.w $4(a1), d1
pub fn i_0826e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826E8;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0826E8: add.w d0, $22(a0)
pub fn i_0826e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826EC;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0826EC: add.w d1, $24(a0)
pub fn i_0826ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826F0;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0826F0: lea.l $77f22.l, a1
pub fn i_0826f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826F6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77F22;
    20
}
/// $0826F6: jsr $4ae.l
pub fn i_0826f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x826FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x826FC);
    cpu.pc = 0x4AE;
    16
}
/// $0826FC: jsr $5dd02.l
pub fn i_0826fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82702);
    cpu.pc = 0x5DD02;
    16
}
/// $082702: movea.l $5c(a6), a1
pub fn i_082702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82706;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $082706: move.w $2(a1), d0
pub fn i_082706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8270A;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08270A: move.w $4(a1), d1
pub fn i_08270a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8270E;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $08270E: add.w d0, $22(a0)
pub fn i_08270e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82712;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082712: add.w d1, $24(a0)
pub fn i_082712(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82716;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082716: addq.l #$6, $5c(a6)
pub fn i_082716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8271A;
    cpu.ir = 0x5CAE;
    let ad1 = cpu.a[6].wrapping_add(0x5C);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 6, Size::Long, true);
    bus.write32(ad1, r3);
    16
}
/// $08271A: addq.b #$1, $70(a6)
pub fn i_08271a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8271E;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08271E: rts
pub fn i_08271e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82720;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $082720: move.w #$73, d1
pub fn i_082720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82724;
    cpu.ir = 0x323C;
    let v1: u32 = 0x73;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082724: jsr $236e.l
pub fn i_082724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8272A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8272A);
    cpu.pc = 0x236E;
    16
}
/// $08272A: move.w #$d000, d0
pub fn i_08272a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8272E;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08272E: jsr $28134.l
pub fn i_08272e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82734;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82734);
    cpu.pc = 0x28134;
    16
}
/// $082734: andi.w #$ffe3, $38(a6)
pub fn i_082734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8273A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08273A: ori.w #$14, $38(a6)
pub fn i_08273a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82740;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082740: lea.l $2be2a0.l, a0
pub fn i_082740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82746;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE2A0;
    20
}
/// $082746: jsr $799de.l
pub fn i_082746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8274C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8274C);
    cpu.pc = 0x799DE;
    16
}
/// $08274C: move.w d0, $66(a6)
pub fn i_08274c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82750;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082750: lea.l $2e6908.l, a0
pub fn i_082750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82756;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6908;
    20
}
/// $082756: jsr $28cd4.l
pub fn i_082756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8275C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8275C);
    cpu.pc = 0x28CD4;
    16
}
/// $08275C: lea.l $82762(pc), a1
pub fn i_08275c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82760;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82762;
    16
}
/// $082760: move.l a1, (a6)
pub fn i_082760(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82762;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082762: jsr $27bc8.l
pub fn i_082762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82768;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82768);
    cpu.pc = 0x27BC8;
    16
}
/// $082768: bcc.w $82772
pub fn i_082768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8276C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82772; return 10; }
    8
}
/// $08276C: lea.l $82884(pc), a1
pub fn i_08276c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82770;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82884;
    16
}
/// $082770: move.l a1, (a6)
pub fn i_082770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82772;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082772: jsr $28d70.l
pub fn i_082772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82778);
    cpu.pc = 0x28D70;
    16
}
/// $082778: jsr $2870a.l
pub fn i_082778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8277E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8277E);
    cpu.pc = 0x2870A;
    16
}
/// $08277E: bcc.w $827fe
pub fn i_08277e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82782;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x827FE; return 10; }
    8
}
/// $082782: bclr.b #$3, $13(a6)
pub fn i_082782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82788;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082788: cmpi.b #$20, $58(a6)
pub fn i_082788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8278E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Byte, false);
    16
}
/// $08278E: beq.w $8279c
pub fn i_08278e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82792;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8279C; return 10; }
    8
}
/// $082792: cmpi.b #$1, $58(a6)
pub fn i_082792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82798;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $082798: bne.w $827fe
pub fn i_082798(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8279C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x827FE; return 10; }
    8
}
/// $08279C: cmpi.w #$20, $58(a6)
pub fn i_08279c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827A2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x58);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $0827A2: bne.w $827c6
pub fn i_0827a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x827C6; return 10; }
    8
}
/// $0827A6: movea.l $50(a6), a0
pub fn i_0827a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827AA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0827AA: move.w $28(a0), d0
pub fn i_0827aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827AE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0827AE: add.w $28(a6), d0
pub fn i_0827ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827B2;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0827B2: move.w d0, $28(a6)
pub fn i_0827b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827B6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0827B6: move.w #$f800, d0
pub fn i_0827b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827BA;
    cpu.ir = 0x303C;
    let v1: u32 = 0xF800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0827BA: add.w $2a(a6), d0
pub fn i_0827ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827BE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0827BE: move.w d0, $2a(a6)
pub fn i_0827be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827C2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0827C2: bra.w $827e2
pub fn i_0827c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827C6;
    cpu.ir = 0x6000;
    cpu.pc = 0x827E2;
    10
}
/// $0827C6: movea.l $50(a6), a0
pub fn i_0827c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827CA;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x50);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0827CA: move.w $28(a0), d0
pub fn i_0827ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827CE;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0827CE: add.w $28(a6), d0
pub fn i_0827ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827D2;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0827D2: move.w d0, $28(a6)
pub fn i_0827d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827D6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0827D6: move.w $2a(a0), d0
pub fn i_0827d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827DA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0827DA: add.w $2a(a6), d0
pub fn i_0827da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827DE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0827DE: move.w d0, $2a(a6)
pub fn i_0827de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827E2;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0827E2: bclr.b #$0, $13(a6)
pub fn i_0827e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827E8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0827E8: lea.l $2be2a0.l, a0
pub fn i_0827e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827EE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE2A0;
    20
}
/// $0827EE: jsr $799de.l
pub fn i_0827ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x827F4);
    cpu.pc = 0x799DE;
    16
}
/// $0827F4: move.w d0, $66(a6)
pub fn i_0827f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827F8;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0827F8: lea.l $8283c(pc), a1
pub fn i_0827f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8283C;
    16
}
/// $0827FC: move.l a1, (a6)
pub fn i_0827fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x827FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0827FE: jsr $283d8.l
pub fn i_0827fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82804;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82804);
    cpu.pc = 0x283D8;
    16
}
/// $082804: btst.b #$1, $13(a6)
pub fn i_082804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8280A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $08280A: beq.w $82814
pub fn i_08280a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8280E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82814; return 10; }
    8
}
/// $08280E: lea.l $8246c(pc), a1
pub fn i_08280e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82812;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8246C;
    16
}
/// $082812: move.l a1, (a6)
pub fn i_082812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82814;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082814: jsr $28758.l
pub fn i_082814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8281A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8281A);
    cpu.pc = 0x28758;
    16
}
/// $08281A: bcc.w $82824
pub fn i_08281a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8281E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82824; return 10; }
    8
}
/// $08281E: lea.l $828e0(pc), a1
pub fn i_08281e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82822;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x828E0;
    16
}
/// $082822: move.l a1, (a6)
pub fn i_082822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82824;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082824: movea.l #$ffffffff, a0
pub fn i_082824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8282A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08282A: jsr $5dd56.l
pub fn i_08282a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82830;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82830);
    cpu.pc = 0x5DD56;
    16
}
/// $082830: bcc.w $8283a
pub fn i_082830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82834;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8283A; return 10; }
    8
}
// ===== SetTaskHandler_082834  ($082834, 8 B, kernel/task_handlers.c)
/// $082834: lea.l $82456(pc), a1
pub fn i_082834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82838;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082838: move.l a1, (a6)
pub fn i_082838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8283A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08283A: rts
pub fn i_08283a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8283C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08283c  ($08283C, 72 B, enemies/squad/para_squad_helpers_08283c.s)
/// $08283C: bset.b #$4, $6b(a6)
pub fn i_08283c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82842;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082842: move.w #$ffc0, $2e(a6)
pub fn i_082842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82848;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082848: lea.l $2e6940.l, a0
pub fn i_082848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8284E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6940;
    20
}
/// $08284E: jsr $28cd4.l
pub fn i_08284e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82854;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82854);
    cpu.pc = 0x28CD4;
    16
}
/// $082854: lea.l $8285a(pc), a1
pub fn i_082854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82858;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8285A;
    16
}
/// $082858: move.l a1, (a6)
pub fn i_082858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8285A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08285A: jsr $27d50.l
pub fn i_08285a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82860;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82860);
    cpu.pc = 0x27D50;
    16
}
/// $082860: bcc.w $8286a
pub fn i_082860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82864;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8286A; return 10; }
    8
}
/// $082864: lea.l $82884(pc), a1
pub fn i_082864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82868;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82884;
    16
}
/// $082868: move.l a1, (a6)
pub fn i_082868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8286A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08286A: jsr $28d70.l
pub fn i_08286a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82870;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82870);
    cpu.pc = 0x28D70;
    16
}
/// $082870: jsr $2870a.l
pub fn i_082870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82876;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82876);
    cpu.pc = 0x2870A;
    16
}
/// $082876: bcc.w $82880
pub fn i_082876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8287A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82880; return 10; }
    8
}
/// $08287A: bclr.b #$3, $13(a6)
pub fn i_08287a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82880;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082880: bra.w $827fe
pub fn i_082880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82884;
    cpu.ir = 0x6000;
    cpu.pc = 0x827FE;
    10
}
// ===== TaskHandler_082884  ($082884, 92 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082884: lea.l $77e10.l, a1
pub fn i_082884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8288A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77E10;
    20
}
/// $08288A: jsr $4ae.l
pub fn i_08288a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82890;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82890);
    cpu.pc = 0x4AE;
    16
}
/// $082890: jsr $5dd02.l
pub fn i_082890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82896;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82896);
    cpu.pc = 0x5DD02;
    16
}
/// $082896: lea.l $2e6956.l, a0
pub fn i_082896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8289C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6956;
    20
}
/// $08289C: jsr $28cd4.l
pub fn i_08289c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828A2);
    cpu.pc = 0x28CD4;
    16
}
/// $0828A2: lea.l $828a8(pc), a1
pub fn i_0828a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x828A8;
    16
}
/// $0828A6: move.l a1, (a6)
pub fn i_0828a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0828A8: jsr $2783a.l
pub fn i_0828a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828AE);
    cpu.pc = 0x2783A;
    16
}
/// $0828AE: jsr $28d70.l
pub fn i_0828ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828B4);
    cpu.pc = 0x28D70;
    16
}
/// $0828B4: bcc.w $828be
pub fn i_0828b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828B8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x828BE; return 10; }
    8
}
/// $0828B8: lea.l $828d2(pc), a1
pub fn i_0828b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828BC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x828D2;
    16
}
/// $0828BC: move.l a1, (a6)
pub fn i_0828bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828BE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0828BE: jsr $2870a.l
pub fn i_0828be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828C4);
    cpu.pc = 0x2870A;
    16
}
/// $0828C4: bcc.w $828ce
pub fn i_0828c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828C8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x828CE; return 10; }
    8
}
/// $0828C8: bclr.b #$3, $13(a6)
pub fn i_0828c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828CE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0828CE: bra.w $827fe
pub fn i_0828ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x827FE;
    10
}
/// $0828D2: move.l #$ffffffff, $48(a6)
pub fn i_0828d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828DA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0828DA: jmp $56204.l
pub fn i_0828da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828E0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x56204;
    8
}
// ===== TaskHandler_0828e0  ($0828E0, 14 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0828E0: move.w #$109f, d0
pub fn i_0828e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x109F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0828E4: jsr $2352.l
pub fn i_0828e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828EA);
    cpu.pc = 0x2352;
    16
}
/// $0828EA: bra.w $8246c
pub fn i_0828ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x8246C;
    10
}
// ===== TaskHandler_0828ee  ($0828EE, 54 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0828EE: move.w #$193, d1
pub fn i_0828ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x193;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0828F2: jsr $236e.l
pub fn i_0828f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x828F8);
    cpu.pc = 0x236E;
    16
}
/// $0828F8: lea.l $2e6728.l, a0
pub fn i_0828f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x828FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6728;
    20
}
/// $0828FE: jsr $28cd4.l
pub fn i_0828fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82904;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82904);
    cpu.pc = 0x28CD4;
    16
}
/// $082904: lea.l $8290a(pc), a1
pub fn i_082904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82908;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8290A;
    16
}
/// $082908: move.l a1, (a6)
pub fn i_082908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8290A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08290A: jsr $5e506.l
pub fn i_08290a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82910;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82910);
    cpu.pc = 0x5E506;
    16
}
/// $082910: move.w $5e(a0), $5e(a6)
pub fn i_082910(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82916;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082916: jsr $28d70.l
pub fn i_082916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8291C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8291C);
    cpu.pc = 0x28D70;
    16
}
/// $08291C: jsr $82ee0(pc)
pub fn i_08291c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82920;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x82920);
    cpu.pc = 0x82EE0;
    16
}
/// $082920: bcc.w $8292a
pub fn i_082920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82924;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8292A; return 10; }
    8
}
// ===== SetTaskHandler_082924  ($082924, 8 B, kernel/task_handlers.c)
/// $082924: lea.l $82464(pc), a1
pub fn i_082924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82928;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $082928: move.l a1, (a6)
pub fn i_082928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8292A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08292A: rts
pub fn i_08292a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8292C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08292c  ($08292C, 120 B, enemies/squad/para_squad_helpers_08283c.s)
/// $08292C: move.w #$d2, d1
pub fn i_08292c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82930;
    cpu.ir = 0x323C;
    let v1: u32 = 0xD2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082930: jsr $236e.l
pub fn i_082930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82936;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82936);
    cpu.pc = 0x236E;
    16
}
/// $082936: move.w #$c000, d0
pub fn i_082936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8293A;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08293A: jsr $28134.l
pub fn i_08293a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82940);
    cpu.pc = 0x28134;
    16
}
/// $082940: andi.w #$ffe3, $38(a6)
pub fn i_082940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82946;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082946: ori.w #$14, $38(a6)
pub fn i_082946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8294C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $08294C: jsr $5e9b6.l
pub fn i_08294c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82952;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82952);
    cpu.pc = 0x5E9B6;
    16
}
/// $082952: andi.w #$7f, d0
pub fn i_082952(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82956;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082956: subi.w #$40, d0
pub fn i_082956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8295A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x40, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08295A: add.w d0, $28(a6)
pub fn i_08295a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8295E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08295E: jsr $5e9b6.l
pub fn i_08295e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82964;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82964);
    cpu.pc = 0x5E9B6;
    16
}
/// $082964: andi.w #$3f, d0
pub fn i_082964(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82968;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082968: add.w d0, $2a(a6)
pub fn i_082968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8296C;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08296C: lea.l $2ee4e8.l, a0
pub fn i_08296c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82972;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2EE4E8;
    20
}
/// $082972: jsr $28cd4.l
pub fn i_082972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82978);
    cpu.pc = 0x28CD4;
    16
}
/// $082978: lea.l $8297e(pc), a1
pub fn i_082978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8297C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8297E;
    16
}
/// $08297C: move.l a1, (a6)
pub fn i_08297c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8297E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08297E: jsr $27bc8.l
pub fn i_08297e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82984;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82984);
    cpu.pc = 0x27BC8;
    16
}
/// $082984: bcc.w $8298e
pub fn i_082984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82988;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8298E; return 10; }
    8
}
/// $082988: lea.l $82456(pc), a1
pub fn i_082988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8298C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $08298C: move.l a1, (a6)
pub fn i_08298c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8298E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $08298E: jsr $28d70.l
pub fn i_08298e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82994;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82994);
    cpu.pc = 0x28D70;
    16
}
/// $082994: movea.l #$ffffffff, a0
pub fn i_082994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8299A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08299A: jsr $5dd56.l
pub fn i_08299a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x829A0);
    cpu.pc = 0x5DD56;
    16
}
/// $0829A0: bcc.w $829aa
pub fn i_0829a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x829AA; return 10; }
    8
}
// ===== SetTaskHandler_0829a4  ($0829A4, 8 B, kernel/task_handlers.c)
/// $0829A4: lea.l $82456(pc), a1
pub fn i_0829a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829A8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $0829A8: move.l a1, (a6)
pub fn i_0829a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829AA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0829AA: rts
pub fn i_0829aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0829ac  ($0829AC, 178 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0829AC: move.w #$1b4, d1
pub fn i_0829ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829B0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1B4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0829B0: jsr $236e.l
pub fn i_0829b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x829B6);
    cpu.pc = 0x236E;
    16
}
/// $0829B6: move.w $5c(a6), d0
pub fn i_0829b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829BA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0829BA: add.w d0, d0
pub fn i_0829ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829BC;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0829BC: lea.l $2e588e.l, a1
pub fn i_0829bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829C2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E588E;
    20
}
/// $0829C2: move.w (a1, d0.w), d0
pub fn i_0829c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829C6;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0829C6: btst.b #$0, $3a(a6)
pub fn i_0829c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829CC;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0829CC: beq.w $829d2
pub fn i_0829cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829D0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x829D2; return 10; }
    8
}
/// $0829D0: neg.w d0
pub fn i_0829d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829D2;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0829D2: add.w $5e(a6), d0
pub fn i_0829d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829D6;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0829D6: move.w d0, $22(a6)
pub fn i_0829d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829DA;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0829DA: move.w #$e4c, d0
pub fn i_0829da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829DE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE4C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0829DE: move.w #$68, d1
pub fn i_0829de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829E2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x68;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0829E2: jsr $440d0.l
pub fn i_0829e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x829E8);
    cpu.pc = 0x440D0;
    16
}
/// $0829E8: move.w d1, $24(a6)
pub fn i_0829e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829EC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0829EC: jsr $5e9b6.l
pub fn i_0829ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x829F2);
    cpu.pc = 0x5E9B6;
    16
}
/// $0829F2: andi.w #$7, d0
pub fn i_0829f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829F6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0829F6: subq.w #$4, d0
pub fn i_0829f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829F8;
    cpu.ir = 0x5940;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 4, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0829F8: add.w d0, $22(a6)
pub fn i_0829f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x829FC;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0829FC: jsr $5e9b6.l
pub fn i_0829fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A02);
    cpu.pc = 0x5E9B6;
    16
}
/// $082A02: andi.w #$3, d0
pub fn i_082a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A06;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082A06: sub.w d0, $24(a6)
pub fn i_082a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A0A;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082A0A: move.w #$8000, d0
pub fn i_082a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A0E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082A0E: jsr $28134.l
pub fn i_082a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A14);
    cpu.pc = 0x28134;
    16
}
/// $082A14: andi.w #$ffe3, $38(a6)
pub fn i_082a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A1A;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082A1A: ori.w #$14, $38(a6)
pub fn i_082a1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A20;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082A20: lea.l $2e687e.l, a0
pub fn i_082a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E687E;
    20
}
/// $082A26: jsr $28cd4.l
pub fn i_082a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A2C);
    cpu.pc = 0x28CD4;
    16
}
/// $082A2C: lea.l $82a32(pc), a1
pub fn i_082a2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82A32;
    16
}
/// $082A30: move.l a1, (a6)
pub fn i_082a30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082A32: jsr $2783a.l
pub fn i_082a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A38);
    cpu.pc = 0x2783A;
    16
}
/// $082A38: jsr $28d70.l
pub fn i_082a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A3E);
    cpu.pc = 0x28D70;
    16
}
/// $082A3E: bcc.w $82a48
pub fn i_082a3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82A48; return 10; }
    8
}
/// $082A42: lea.l $82456(pc), a1
pub fn i_082a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082A46: move.l a1, (a6)
pub fn i_082a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082A48: jsr $283d8.l
pub fn i_082a48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A4E);
    cpu.pc = 0x283D8;
    16
}
/// $082A4E: movea.l #$ffffffff, a0
pub fn i_082a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A54;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $082A54: jsr $5dd56.l
pub fn i_082a54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82A5A);
    cpu.pc = 0x5DD56;
    16
}
/// $082A5A: bcc.w $82a64
pub fn i_082a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A5E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82A64; return 10; }
    8
}
// ===== SetTaskHandler_082a5e  ($082A5E, 8 B, kernel/task_handlers.c)
/// $082A5E: lea.l $82456(pc), a1
pub fn i_082a5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082A62: move.l a1, (a6)
pub fn i_082a62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082A64: rts
pub fn i_082a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082a66  ($082A66, 216 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082A66: move.w $34(a6), d3
pub fn i_082a66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A6A;
    cpu.ir = 0x362E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082A6A: add.w d3, d3
pub fn i_082a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A6C;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082A6C: move.w #$2000, d0
pub fn i_082a6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082A70: lea.l $2c07ac.l, a1
pub fn i_082a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A76;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2C07AC;
    20
}
/// $082A76: lea.l $2c072c.l, a2
pub fn i_082a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A7C;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2C072C;
    20
}
/// $082A7C: move.w (a1, d3.w), d1
pub fn i_082a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A80;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082A80: move.w (a2, d3.w), d2
pub fn i_082a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A84;
    cpu.ir = 0x3432;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082A84: muls.w d0, d1
pub fn i_082a84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A86;
    cpu.ir = 0xC3C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[1] as u16 as i16 as i32)) as u32;
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $082A86: muls.w d0, d2
pub fn i_082a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A88;
    cpu.ir = 0xC5C0;
    let v1 = cpu.d[0] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[2] as u16 as i16 as i32)) as u32;
    cpu.d[2] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $082A88: asr.l #$8, d1
pub fn i_082a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A8A;
    cpu.ir = 0xE081;
    let h1 = cpu.d[1] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[1] = r;
    22
}
/// $082A8A: asr.l #$8, d2
pub fn i_082a8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A8C;
    cpu.ir = 0xE082;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[2] = r;
    22
}
/// $082A8C: btst.b #$0, $3a(a6)
pub fn i_082a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A92;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082A92: beq.w $82a98
pub fn i_082a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A96;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82A98; return 10; }
    8
}
/// $082A96: neg.w d1
pub fn i_082a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A98;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082A98: move.w d1, $28(a6)
pub fn i_082a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82A9C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082A9C: move.w d2, $2a(a6)
pub fn i_082a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AA0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082AA0: clr.w $34(a6)
pub fn i_082aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AA4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x34);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $082AA4: move.w $22(a6), $5e(a6)
pub fn i_082aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AAA;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082AAA: move.w #$2, d1
pub fn i_082aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AAE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082AAE: jsr $236e.l
pub fn i_082aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82AB4);
    cpu.pc = 0x236E;
    16
}
/// $082AB4: bset.b #$4, $6b(a6)
pub fn i_082ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ABA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082ABA: move.w #$d000, d0
pub fn i_082aba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ABE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082ABE: jsr $28134.l
pub fn i_082abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82AC4);
    cpu.pc = 0x28134;
    16
}
/// $082AC4: andi.w #$ffe3, $38(a6)
pub fn i_082ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ACA;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082ACA: ori.w #$14, $38(a6)
pub fn i_082aca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AD0;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082AD0: lea.l $2e68f0.l, a0
pub fn i_082ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AD6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E68F0;
    20
}
/// $082AD6: jsr $28cd4.l
pub fn i_082ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ADC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82ADC);
    cpu.pc = 0x28CD4;
    16
}
/// $082ADC: lea.l $82ae2(pc), a1
pub fn i_082adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82AE2;
    16
}
/// $082AE0: move.l a1, (a6)
pub fn i_082ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082AE2: jsr $27cee.l
pub fn i_082ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82AE8);
    cpu.pc = 0x27CEE;
    16
}
/// $082AE8: bcc.w $82af4
pub fn i_082ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AEC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82AF4; return 10; }
    8
}
/// $082AEC: lea.l $31d26.l, a1
pub fn i_082aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AF2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31D26;
    20
}
/// $082AF2: move.l a1, (a6)
pub fn i_082af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AF4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082AF4: jsr $28d70.l
pub fn i_082af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82AFA);
    cpu.pc = 0x28D70;
    16
}
/// $082AFA: move.w #$e4c, d0
pub fn i_082afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82AFE;
    cpu.ir = 0x303C;
    let v1: u32 = 0xE4C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082AFE: move.w #$68, d1
pub fn i_082afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B02;
    cpu.ir = 0x323C;
    let v1: u32 = 0x68;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082B02: jsr $440d0.l
pub fn i_082b02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B08);
    cpu.pc = 0x440D0;
    16
}
/// $082B08: cmp.w $24(a6), d1
pub fn i_082b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B0C;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082B0C: bgt.w $82b16
pub fn i_082b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B10;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82B16; return 10; }
    8
}
/// $082B10: lea.l $829ac(pc), a1
pub fn i_082b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x829AC;
    16
}
/// $082B14: move.l a1, (a6)
pub fn i_082b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B16: jsr $283d8.l
pub fn i_082b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B1C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B1C);
    cpu.pc = 0x283D8;
    16
}
/// $082B1C: btst.b #$1, $13(a6)
pub fn i_082b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B22;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082B22: beq.w $82b2e
pub fn i_082b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B26;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82B2E; return 10; }
    8
}
/// $082B26: lea.l $31d26.l, a1
pub fn i_082b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B2C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x31D26;
    20
}
/// $082B2C: move.l a1, (a6)
pub fn i_082b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B2E: movea.l #$ffffffff, a0
pub fn i_082b2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B34;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $082B34: jsr $5dd56.l
pub fn i_082b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B3A);
    cpu.pc = 0x5DD56;
    16
}
/// $082B3A: bcc.w $82b44
pub fn i_082b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82B44; return 10; }
    8
}
// ===== SetTaskHandler_082b3e  ($082B3E, 8 B, kernel/task_handlers.c)
/// $082B3E: lea.l $82456(pc), a1
pub fn i_082b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082B42: move.l a1, (a6)
pub fn i_082b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B44: rts
pub fn i_082b44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B46;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082b46  ($082B46, 106 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082B46: move.w #$76, d1
pub fn i_082b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B4A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x76;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082B4A: jsr $236e.l
pub fn i_082b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B50);
    cpu.pc = 0x236E;
    16
}
/// $082B50: move.w #$d000, d0
pub fn i_082b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B54;
    cpu.ir = 0x303C;
    let v1: u32 = 0xD000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082B54: jsr $28134.l
pub fn i_082b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B5A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B5A);
    cpu.pc = 0x28134;
    16
}
/// $082B5A: andi.w #$ffe3, $38(a6)
pub fn i_082b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B60;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082B60: ori.w #$1c, $38(a6)
pub fn i_082b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B66;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082B66: move.w #$10, $28(a6)
pub fn i_082b66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082B6C: move.w #$c0, $2a(a6)
pub fn i_082b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B72;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082B72: lea.l $2e6aea.l, a0
pub fn i_082b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B78;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6AEA;
    20
}
/// $082B78: jsr $28cd4.l
pub fn i_082b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B7E);
    cpu.pc = 0x28CD4;
    16
}
/// $082B7E: lea.l $82b84(pc), a1
pub fn i_082b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B82;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82B84;
    16
}
/// $082B82: move.l a1, (a6)
pub fn i_082b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B84;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B84: jsr $27cee.l
pub fn i_082b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B8A);
    cpu.pc = 0x27CEE;
    16
}
/// $082B8A: jsr $28d70.l
pub fn i_082b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B90;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82B90);
    cpu.pc = 0x28D70;
    16
}
/// $082B90: bcc.w $82b9a
pub fn i_082b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B94;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82B9A; return 10; }
    8
}
/// $082B94: lea.l $82456(pc), a1
pub fn i_082b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082B98: move.l a1, (a6)
pub fn i_082b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82B9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082B9A: movea.l #$ffffffff, a0
pub fn i_082b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BA0;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $082BA0: lea.l $2e5416.l, a0
pub fn i_082ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5416;
    20
}
/// $082BA6: jsr $5dd56.l
pub fn i_082ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BAC);
    cpu.pc = 0x5DD56;
    16
}
/// $082BAC: bcc.w $82bb6
pub fn i_082bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BB0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82BB6; return 10; }
    8
}
// ===== SetTaskHandler_082bb0  ($082BB0, 8 B, kernel/task_handlers.c)
/// $082BB0: lea.l $82456(pc), a1
pub fn i_082bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BB4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082BB4: move.l a1, (a6)
pub fn i_082bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BB6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082BB6: rts
pub fn i_082bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082bb8  ($082BB8, 72 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082BB8: move.w #$c000, d0
pub fn i_082bb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BBC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082BBC: jsr $28134.l
pub fn i_082bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BC2);
    cpu.pc = 0x28134;
    16
}
/// $082BC2: andi.w #$ffe3, $38(a6)
pub fn i_082bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BC8;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082BC8: ori.w #$1c, $38(a6)
pub fn i_082bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BCE;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $082BCE: lea.l $2e6992.l, a0
pub fn i_082bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BD4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6992;
    20
}
/// $082BD4: jsr $28cd4.l
pub fn i_082bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BDA);
    cpu.pc = 0x28CD4;
    16
}
/// $082BDA: lea.l $82be0(pc), a1
pub fn i_082bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82BE0;
    16
}
/// $082BDE: move.l a1, (a6)
pub fn i_082bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082BE0: jsr $2783a.l
pub fn i_082be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BE6);
    cpu.pc = 0x2783A;
    16
}
/// $082BE6: jsr $28d70.l
pub fn i_082be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BEC);
    cpu.pc = 0x28D70;
    16
}
/// $082BEC: bcc.w $82bf6
pub fn i_082bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BF0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82BF6; return 10; }
    8
}
/// $082BF0: lea.l $82464(pc), a1
pub fn i_082bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BF4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82464;
    16
}
/// $082BF4: move.l a1, (a6)
pub fn i_082bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BF6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082BF6: jsr $5e45a.l
pub fn i_082bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82BFC);
    cpu.pc = 0x5E45A;
    16
}
/// $082BFC: bcc.w $82c06
pub fn i_082bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82C06; return 10; }
    8
}
// ===== SetTaskHandler_082c00  ($082C00, 8 B, kernel/task_handlers.c)
/// $082C00: lea.l $82456(pc), a1
pub fn i_082c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82456;
    16
}
/// $082C04: move.l a1, (a6)
pub fn i_082c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $082C06: rts
pub fn i_082c06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C08;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_082c08  ($082C08, 108 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082C08: jsr $5e9b6.l
pub fn i_082c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82C0E);
    cpu.pc = 0x5E9B6;
    16
}
/// $082C0E: andi.w #$7, d0
pub fn i_082c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C12;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082C12: move.w d0, $5c(a6)
pub fn i_082c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C16;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082C16: add.w d0, d0
pub fn i_082c16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C18;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082C18: lea.l $2e596a.l, a0
pub fn i_082c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C1E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E596A;
    20
}
/// $082C1E: move.w (a0, d0.w), d1
pub fn i_082c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C22;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082C22: jsr $236e.l
pub fn i_082c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82C28);
    cpu.pc = 0x236E;
    16
}
/// $082C28: move.w $5c(a6), d0
pub fn i_082c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C2C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082C2C: movea.l #$2e594a, a0
pub fn i_082c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C32;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2E594A;
    cpu.a[0] = v1;
    4
}
/// $082C32: lsl.w #$2, d0
pub fn i_082c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C34;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $082C34: movea.l (a0, d0.w), a0
pub fn i_082c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C38;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $082C38: cmpa.l #$ffffffff, a0
pub fn i_082c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C3E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $082C3E: beq.w $82c48
pub fn i_082c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C42;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82C48; return 10; }
    8
}
/// $082C42: jsr $28cd4.l
pub fn i_082c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82C48);
    cpu.pc = 0x28CD4;
    16
}
/// $082C48: move.w #$800, $36(a6)
pub fn i_082c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C4E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082C4E: jmp $6dce0.l
pub fn i_082c4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C54;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x6DCE0;
    8
}
/// $082C54: movea.l $c(a6), a0
pub fn i_082c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C58;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082C58: cmpi.w #$120, $22(a0)
pub fn i_082c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C5E;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $082C5E: bgt.w $82c6e
pub fn i_082c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C62;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82C6E; return 10; }
    8
}
/// $082C62: clr.b $45(a0)
pub fn i_082c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C66;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x45);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $082C66: jmp $518.l
pub fn i_082c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C6C;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $082C6C: rts
pub fn i_082c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $082C6E: move.b #$2, $45(a0)
pub fn i_082c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C74;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClrRamWord_082c74  ($082C74, 8 B, thunks/clr_ram_word.c)
/// $082C74: clr.b $10e39a.l
pub fn i_082c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C7A;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x10E39A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x10E39A, r2 as u8);
    16
}
/// $082C7A: rts
pub fn i_082c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082C7C  ($082C7C, 42 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082C7C: move.b $7e(a6), d0
pub fn i_082c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C80;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $082C80: addq.b #$1, $7e(a6)
pub fn i_082c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C84;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $082C84: andi.w #$f, d0
pub fn i_082c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C88;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082C88: add.w d0, d0
pub fn i_082c88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C8A;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $082C8A: move.w $22(a6), $80(a6)
pub fn i_082c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C90;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082C90: move.w $24(a6), d1
pub fn i_082c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C94;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082C94: add.w (a0, d0.w), d1
pub fn i_082c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C98;
    cpu.ir = 0xD270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $082C98: move.w d1, $82(a6)
pub fn i_082c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82C9C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082C9C: andi.b #$f, $7e(a6)
pub fn i_082c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CA2;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xF) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082CA2: beq.w $82cac
pub fn i_082ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82CAC; return 10; }
    8
}
// ===== ClearXN_082ca6  ($082CA6, 6 B, thunks/ccr_helpers.c)
/// $082CA6: andi.b #$ee, ccr
pub fn i_082ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CAA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082CAA: rts
pub fn i_082caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CAC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_082cac  ($082CAC, 6 B, thunks/ccr_helpers.c)
/// $082CAC: ori.b #$11, ccr
pub fn i_082cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CB0;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082CB0: rts
pub fn i_082cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082CB2  ($082CB2, 18 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082CB2: clr.b $7e(a6)
pub fn i_082cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CB6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x7E);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $082CB6: move.w $22(a6), $80(a6)
pub fn i_082cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CBC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CBC: move.w $24(a6), $82(a6)
pub fn i_082cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CC2;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CC2: rts
pub fn i_082cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CC4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082CC4  ($082CC4, 30 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082CC4: movea.l $c(a6), a0
pub fn i_082cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CC8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082CC8: move.w $38(a0), $38(a6)
pub fn i_082cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CCE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CCE: move.w $80(a0), $22(a6)
pub fn i_082cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CD4;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CD4: move.w $82(a0), $24(a6)
pub fn i_082cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CDA;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CDA: move.b $73(a0), $73(a6)
pub fn i_082cda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CE0;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $082CE0: rts
pub fn i_082ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CE2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082CE2  ($082CE2, 48 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082CE2: movea.l $c(a6), a0
pub fn i_082ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CE6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082CE6: move.l $94(a0), $94(a6)
pub fn i_082ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CEC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x94);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $082CEC: move.w $38(a0), $38(a6)
pub fn i_082cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CF2;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $082CF2: move.w $80(a0), d0
pub fn i_082cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CF6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082CF6: add.w $80(a6), d0
pub fn i_082cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CFA;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $082CFA: move.w d0, $22(a6)
pub fn i_082cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82CFE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082CFE: move.w $82(a0), d0
pub fn i_082cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D02;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082D02: add.w $82(a6), d0
pub fn i_082d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D06;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $082D06: move.w d0, $24(a6)
pub fn i_082d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D0A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $082D0A: move.b $73(a0), $73(a6)
pub fn i_082d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D10;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad4, v3 as u8);
    20
}
/// $082D10: rts
pub fn i_082d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D12;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082D12  ($082D12, 46 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082D12: btst.b #$6, $72(a6)
pub fn i_082d12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D18;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082D18: beq.w $82d22
pub fn i_082d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D1C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D22; return 10; }
    8
}
/// $082D1C: bset.b #$0, $5a(a6)
pub fn i_082d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D22;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D22: bclr.b #$6, $72(a6)
pub fn i_082d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D28;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D28: bclr.b #$7, $72(a6)
pub fn i_082d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D2E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D2E: btst.b #$0, $5a(a6)
pub fn i_082d2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D34;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082D34: beq.w $82d3e
pub fn i_082d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D3E; return 10; }
    8
}
/// $082D38: bset.b #$7, $72(a6)
pub fn i_082d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D3E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D3E: rts
pub fn i_082d3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D40;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082D40  ($082D40, 48 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082D40: btst.b #$0, $5a(a6)
pub fn i_082d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D46;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082D46: beq.w $82d5a
pub fn i_082d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D5A; return 10; }
    8
}
/// $082D4A: bclr.b #$0, $5a(a6)
pub fn i_082d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D50;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D50: movea.l $c(a6), a0
pub fn i_082d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D54;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082D54: bset.b #$6, $72(a0)
pub fn i_082d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D5A;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D5A: movea.l $c(a6), a0
pub fn i_082d5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D5E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082D5E: btst.b #$7, $72(a0)
pub fn i_082d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D64;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082D64: beq.w $82d6e
pub fn i_082d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D6E; return 10; }
    8
}
/// $082D68: bset.b #$0, $5a(a6)
pub fn i_082d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D6E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082D6E: rts
pub fn i_082d6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082D70  ($082D70, 62 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082D70: tst.b $73(a6)
pub fn i_082d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D74;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082D74: bne.w $82dac
pub fn i_082d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82DAC; return 10; }
    8
}
/// $082D78: move.b $7d(a6), d0
pub fn i_082d78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D7C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $082D7C: andi.b #$3, d0
pub fn i_082d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D80;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $082D80: beq.w $82da6
pub fn i_082d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D84;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82DA6; return 10; }
    8
}
/// $082D84: lea.l $2be098.l, a0
pub fn i_082d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D8A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE098;
    20
}
/// $082D8A: cmpi.b #$3, d0
pub fn i_082d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D8E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $082D8E: beq.w $82d98
pub fn i_082d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D92;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82D98; return 10; }
    8
}
/// $082D92: lea.l $2be11a.l, a0
pub fn i_082d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BE11A;
    20
}
/// $082D98: jsr $799de.l
pub fn i_082d98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82D9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82D9E);
    cpu.pc = 0x799DE;
    16
}
/// $082D9E: cmp.w $66(a6), d0
pub fn i_082d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DA2;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082DA2: blt.w $82dac
pub fn i_082da2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DA6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82DAC; return 10; }
    8
}
/// $082DA6: move.b #$ff, $73(a6)
pub fn i_082da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DAC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $082DAC: rts
pub fn i_082dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DAE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082DAE  ($082DAE, 58 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082DAE: btst.b #$5, $72(a6)
pub fn i_082dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DB4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082DB4: bne.w $82de8
pub fn i_082db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DB8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82DE8; return 10; }
    8
}
/// $082DB8: cmpi.w #$20, $22(a6)
pub fn i_082db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DBE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $082DBE: blt.w $82dee
pub fn i_082dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DC2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82DEE; return 10; }
    8
}
/// $082DC2: cmpi.w #$120, $22(a6)
pub fn i_082dc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DC8;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $082DC8: bgt.w $82df8
pub fn i_082dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DCC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82DF8; return 10; }
    8
}
/// $082DCC: tst.b $7b(a6)
pub fn i_082dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DD0;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082DD0: bne.w $82de8
pub fn i_082dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DD4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82DE8; return 10; }
    8
}
/// $082DD4: movea.l $94(a6), a0
pub fn i_082dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DD8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082DD8: move.w $22(a0), d0
pub fn i_082dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DDC;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082DDC: cmp.w $22(a6), d0
pub fn i_082ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DE0;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082DE0: bgt.w $82dee
pub fn i_082de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DE4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82DEE; return 10; }
    8
}
/// $082DE4: bra.w $82df8
pub fn i_082de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x82DF8;
    10
}
// ===== ClearXN_082de8  ($082DE8, 6 B, thunks/ccr_helpers.c)
/// $082DE8: andi.b #$ee, ccr
pub fn i_082de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DEC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082DEC: rts
pub fn i_082dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrFlag74_082dee  ($082DEE, 4 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082DEE: clr.b $74(a6)
pub fn i_082dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DF2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetXN_082df2  ($082DF2, 6 B, thunks/ccr_helpers.c)
/// $082DF2: ori.b #$11, ccr
pub fn i_082df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DF6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082DF6: rts
pub fn i_082df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetFlag74_082df8  ($082DF8, 6 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082DF8: move.b #$1, $74(a6)
pub fn i_082df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82DFE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== SetXN_082dfe  ($082DFE, 6 B, thunks/ccr_helpers.c)
/// $082DFE: ori.b #$11, ccr
pub fn i_082dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E02;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082E02: rts
pub fn i_082e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E04;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082E04  ($082E04, 106 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082E04: btst.b #$5, $72(a6)
pub fn i_082e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E0A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082E0A: bne.w $82e6e
pub fn i_082e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E0E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E0E: cmpi.w #$20, $22(a6)
pub fn i_082e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x20, Size::Word, false);
    16
}
/// $082E14: blt.w $82e2a
pub fn i_082e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E18;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82E2A; return 10; }
    8
}
/// $082E18: cmpi.w #$120, $22(a6)
pub fn i_082e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E1E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $082E1E: bgt.w $82e2a
pub fn i_082e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E22;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82E2A; return 10; }
    8
}
/// $082E22: tst.b $7b(a6)
pub fn i_082e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E26;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082E26: bne.w $82e6e
pub fn i_082e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E2A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E2A: tst.b $74(a6)
pub fn i_082e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E2E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082E2E: beq.w $82e50
pub fn i_082e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82E50; return 10; }
    8
}
/// $082E32: cmpi.w #$30, $22(a6)
pub fn i_082e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E38;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $082E38: blt.w $82e6e
pub fn i_082e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E3C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E3C: movea.l $94(a6), a0
pub fn i_082e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E40;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082E40: move.w $22(a0), d0
pub fn i_082e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E44;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082E44: cmp.w $22(a6), d0
pub fn i_082e44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E48;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082E48: bgt.w $82e6e
pub fn i_082e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E4C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E4C: bra.w $82e74
pub fn i_082e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E50;
    cpu.ir = 0x6000;
    cpu.pc = 0x82E74;
    10
}
/// $082E50: cmpi.w #$120, $22(a6)
pub fn i_082e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E56;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x120, Size::Word, false);
    16
}
/// $082E56: bgt.w $82e6e
pub fn i_082e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E5A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E5A: movea.l $94(a6), a0
pub fn i_082e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E5E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082E5E: move.w $22(a0), d0
pub fn i_082e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E62;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082E62: cmp.w $22(a6), d0
pub fn i_082e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E66;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082E66: blt.w $82e6e
pub fn i_082e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E6A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x82E6E; return 10; }
    8
}
/// $082E6A: bra.w $82e74
pub fn i_082e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x82E74;
    10
}
// ===== SetXN_082e6e  ($082E6E, 6 B, thunks/ccr_helpers.c)
/// $082E6E: ori.b #$11, ccr
pub fn i_082e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E72;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082E72: rts
pub fn i_082e72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_082e74  ($082E74, 6 B, thunks/ccr_helpers.c)
/// $082E74: andi.b #$ee, ccr
pub fn i_082e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E78;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082E78: rts
pub fn i_082e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E7A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082E7A  ($082E7A, 90 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082E7A: move.b $72(a6), d0
pub fn i_082e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E7E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $082E7E: andi.b #$7, d0
pub fn i_082e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E82;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x7) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $082E82: bne.w $82eda
pub fn i_082e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82EDA; return 10; }
    8
}
/// $082E86: btst.b #$4, $72(a6)
pub fn i_082e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E8C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082E8C: beq.w $82ea2
pub fn i_082e8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82EA2; return 10; }
    8
}
/// $082E90: tst.b $7b(a6)
pub fn i_082e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E94;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x7B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082E94: beq.w $82ea2
pub fn i_082e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82EA2; return 10; }
    8
}
/// $082E98: bset.b #$2, $72(a6)
pub fn i_082e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82E9E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082E9E: bra.w $82ed4
pub fn i_082e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x82ED4;
    10
}
/// $082EA2: btst.b #$3, $72(a6)
pub fn i_082ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EA8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082EA8: beq.w $82eda
pub fn i_082ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EAC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82EDA; return 10; }
    8
}
/// $082EAC: jsr $5e3fc.l
pub fn i_082eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82EB2);
    cpu.pc = 0x5E3FC;
    16
}
/// $082EB2: bcs.w $82ece
pub fn i_082eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EB6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x82ECE; return 10; }
    8
}
/// $082EB6: jsr $5e9b6.l
pub fn i_082eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82EBC);
    cpu.pc = 0x5E9B6;
    16
}
/// $082EBC: andi.w #$7, d0
pub fn i_082ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EC0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082EC0: bne.w $82ece
pub fn i_082ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EC4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82ECE; return 10; }
    8
}
/// $082EC4: bset.b #$1, $72(a6)
pub fn i_082ec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ECA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $082ECA: bra.w $82ed4
pub fn i_082eca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ECE;
    cpu.ir = 0x6000;
    cpu.pc = 0x82ED4;
    10
}
/// $082ECE: bset.b #$0, $72(a6)
pub fn i_082ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ED4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetXN_082ed4  ($082ED4, 6 B, thunks/ccr_helpers.c)
/// $082ED4: ori.b #$11, ccr
pub fn i_082ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82ED8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082ED8: rts
pub fn i_082ed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_082eda  ($082EDA, 6 B, thunks/ccr_helpers.c)
/// $082EDA: andi.b #$ee, ccr
pub fn i_082eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EDE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082EDE: rts
pub fn i_082ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082EE0  ($082EE0, 12 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082EE0: movea.l $c(a6), a0
pub fn i_082ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EE4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082EE4: tst.b $20(a0)
pub fn i_082ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EE8;
    cpu.ir = 0x4A28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $082EE8: bne.w $82ef2
pub fn i_082ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82EF2; return 10; }
    8
}
// ===== ClearXN_082eec  ($082EEC, 6 B, thunks/ccr_helpers.c)
/// $082EEC: andi.b #$ee, ccr
pub fn i_082eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EF0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082EF0: rts
pub fn i_082ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_082ef2  ($082EF2, 6 B, thunks/ccr_helpers.c)
/// $082EF2: ori.b #$11, ccr
pub fn i_082ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EF6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082EF6: rts
pub fn i_082ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EF8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082EF8  ($082EF8, 44 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082EF8: movea.l $c(a6), a0
pub fn i_082ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82EFC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082EFC: btst.b #$0, $72(a0)
pub fn i_082efc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F02;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082F02: bne.w $82f18
pub fn i_082f02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82F18; return 10; }
    8
}
/// $082F06: movea.l $c(a6), a0
pub fn i_082f06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F0A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082F0A: btst.b #$1, $72(a0)
pub fn i_082f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F10;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082F10: bne.w $82f1e
pub fn i_082f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F14;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x82F1E; return 10; }
    8
}
/// $082F14: clr.w d0
pub fn i_082f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F16;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F16: rts
pub fn i_082f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F18;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $082F18: move.w #$1, d0
pub fn i_082f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F1C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082F1C: rts
pub fn i_082f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F1E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $082F1E: move.w #$2, d0
pub fn i_082f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F22;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082F22: rts
pub fn i_082f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F24;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082F24  ($082F24, 84 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082F24: movea.l $94(a6), a0
pub fn i_082f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F28;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082F28: addi.w #$38, $24(a6)
pub fn i_082f28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F2E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $082F2E: jsr $5e070.l
pub fn i_082f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82F34);
    cpu.pc = 0x5E070;
    16
}
/// $082F34: subi.w #$38, $24(a6)
pub fn i_082f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F3A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x38, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $082F3A: subi.w #$24, d0
pub fn i_082f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F3E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x24, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $082F3E: asr.w #$3, d0
pub fn i_082f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F40;
    cpu.ir = 0xE640;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $082F40: cmp.w $5c(a6), d0
pub fn i_082f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F44;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082F44: beq.w $82f7e
pub fn i_082f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F48;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82F7E; return 10; }
    8
}
/// $082F48: move.w #$1, d1
pub fn i_082f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F4C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082F4C: cmp.w $5c(a6), d0
pub fn i_082f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F50;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $082F50: bgt.w $82f56
pub fn i_082f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F54;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x82F56; return 10; }
    8
}
/// $082F54: neg.w d1
pub fn i_082f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F56;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F56: move.w d1, d0
pub fn i_082f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F58;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $082F58: add.w $5c(a6), d1
pub fn i_082f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F5C;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $082F5C: cmpi.w #$0, d1
pub fn i_082f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F60;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $082F60: bge.w $82f66
pub fn i_082f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F64;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x82F66; return 10; }
    8
}
/// $082F64: clr.w d0
pub fn i_082f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F66;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F66: cmpi.w #$6, d1
pub fn i_082f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F6A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x6, Size::Word, false);
    8
}
/// $082F6A: ble.w $82f70
pub fn i_082f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F6E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x82F70; return 10; }
    8
}
/// $082F6E: clr.w d0
pub fn i_082f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F70;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F70: movea.l $c(a6), a0
pub fn i_082f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F74;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082F74: clr.b $7b(a0)
pub fn i_082f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F78;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x7B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== SetXN_082f78  ($082F78, 6 B, thunks/ccr_helpers.c)
/// $082F78: ori.b #$11, ccr
pub fn i_082f78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F7C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $082F7C: rts
pub fn i_082f7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F7E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetFlag7b_082f7e  ($082F7E, 12 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082F7E: clr.w d0
pub fn i_082f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F80;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $082F80: movea.l $c(a6), a0
pub fn i_082f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F84;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082F84: move.b #$1, $7b(a0)
pub fn i_082f84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F8A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x7B);
    bus.write8(ad2, v1 as u8);
    12
}
// ===== ClearXN_082f8a  ($082F8A, 6 B, thunks/ccr_helpers.c)
/// $082F8A: andi.b #$ee, ccr
pub fn i_082f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F8E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $082F8E: rts
pub fn i_082f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F90;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082F90  ($082F90, 58 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082F90: movea.l a6, a1
pub fn i_082f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F92;
    cpu.ir = 0x224E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $082F92: cmpi.b #$ff, $7c(a6)
pub fn i_082f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F98;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $082F98: beq.w $82fa0
pub fn i_082f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82F9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82FA0; return 10; }
    8
}
/// $082F9C: movea.l $c(a6), a1
pub fn i_082f9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FA0;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $082FA0: movea.l $94(a1), a0
pub fn i_082fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FA4;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x94);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $082FA4: jsr $5e338.l
pub fn i_082fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82FAA);
    cpu.pc = 0x5E338;
    16
}
/// $082FAA: bcc.w $82fc8
pub fn i_082faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x82FC8; return 10; }
    8
}
/// $082FAE: jsr $5e0d4.l
pub fn i_082fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82FB4);
    cpu.pc = 0x5E0D4;
    16
}
/// $082FB4: movea.l a6, a1
pub fn i_082fb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FB6;
    cpu.ir = 0x224E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $082FB6: cmpi.b #$ff, $7c(a6)
pub fn i_082fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FBC;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $082FBC: beq.w $82fc4
pub fn i_082fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FC0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x82FC4; return 10; }
    8
}
/// $082FC0: movea.l $c(a6), a1
pub fn i_082fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FC4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $082FC4: move.l a0, $94(a1)
pub fn i_082fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FC8;
    cpu.ir = 0x2348;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x94);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $082FC8: rts
pub fn i_082fc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00082FCA  ($082FCA, 154 B, enemies/squad/para_squad_helpers_08283c.s)
/// $082FCA: lea.l $77e10.l, a1
pub fn i_082fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FD0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77E10;
    20
}
/// $082FD0: jsr $4ae.l
pub fn i_082fd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82FD6);
    cpu.pc = 0x4AE;
    16
}
/// $082FD6: jsr $5dd02.l
pub fn i_082fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FDC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x82FDC);
    cpu.pc = 0x5DD02;
    16
}
/// $082FDC: move.w $5c(a6), d0
pub fn i_082fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FE0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $082FE0: lsl.w #$2, d0
pub fn i_082fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FE2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $082FE2: lea.l $2e5852.l, a1
pub fn i_082fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FE8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5852;
    20
}
/// $082FE8: move.w (a1, d0.w), d1
pub fn i_082fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FEC;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082FEC: move.w $2(a1, d0.w), d2
pub fn i_082fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FF0;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $082FF0: add.w d2, $24(a0)
pub fn i_082ff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FF4;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $082FF4: btst.b #$0, $3a(a6)
pub fn i_082ff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FFA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $082FFA: beq.w $83000
pub fn i_082ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x82FFE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x83000; return 10; }
    8
}
/// $082FFE: neg.w d1
pub fn i_082ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83000;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $083000: add.w d1, $22(a0)
pub fn i_083000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83004;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083004: lea.l $82720(pc), a1
pub fn i_083004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83008;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82720;
    16
}
/// $083008: jsr $4ae.l
pub fn i_083008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8300E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8300E);
    cpu.pc = 0x4AE;
    16
}
/// $08300E: jsr $5dd02.l
pub fn i_08300e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83014;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83014);
    cpu.pc = 0x5DD02;
    16
}
/// $083014: move.w $5c(a6), d0
pub fn i_083014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83018;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $083018: move.w d0, d3
pub fn i_083018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8301A;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08301A: lsl.w #$2, d0
pub fn i_08301a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8301C;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08301C: lea.l $2e5852.l, a1
pub fn i_08301c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83022;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5852;
    20
}
/// $083022: move.w (a1, d0.w), d1
pub fn i_083022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83026;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $083026: move.w $2(a1, d0.w), d2
pub fn i_083026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8302A;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08302A: add.w d2, $24(a0)
pub fn i_08302a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8302E;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08302E: lea.l $2e586a.l, a2
pub fn i_08302e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83034;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E586A;
    20
}
/// $083034: move.w d3, d4
pub fn i_083034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83036;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083036: add.w d4, d3
pub fn i_083036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83038;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $083038: add.w d4, d3
pub fn i_083038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8303A;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08303A: add.w d3, d3
pub fn i_08303a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8303C;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08303C: move.w $2(a2, d3.w), $2e(a0)
pub fn i_08303c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83042;
    cpu.ir = 0x3172;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $083042: move.w $4(a2, d3.w), $2a(a0)
pub fn i_083042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83048;
    cpu.ir = 0x3172;
    let ad2 = cpu.a[2].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $083048: move.w (a2, d3.w), d4
pub fn i_083048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8304C;
    cpu.ir = 0x3832;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08304C: btst.b #$0, $3a(a6)
pub fn i_08304c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83052;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $083052: beq.w $8305a
pub fn i_083052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83056;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8305A; return 10; }
    8
}
/// $083056: neg.w d1
pub fn i_083056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83058;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $083058: neg.w d4
pub fn i_083058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8305A;
    cpu.ir = 0x4444;
    let v1 = cpu.d[4] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08305A: add.w d1, $22(a0)
pub fn i_08305a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8305E;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08305E: move.w d4, $28(a0)
pub fn i_08305e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83062;
    cpu.ir = 0x3144;
    let v1 = cpu.d[4] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083062: rts
pub fn i_083062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83064;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00083064  ($083064, 190 B, enemies/squad/para_squad_helpers_08283c.s)
/// $083064: lea.l $8292c(pc), a1
pub fn i_083064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83068;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8292C;
    16
}
/// $083068: jsr $4ae.l
pub fn i_083068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8306E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8306E);
    cpu.pc = 0x4AE;
    16
}
/// $08306E: jsr $5dd02.l
pub fn i_08306e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83074;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83074);
    cpu.pc = 0x5DD02;
    16
}
/// $083074: move.w $5e(a6), d0
pub fn i_083074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83078;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $083078: move.w d0, d3
pub fn i_083078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8307A;
    cpu.ir = 0x3600;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08307A: lea.l $2e58ae.l, a1
pub fn i_08307a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83080;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E58AE;
    20
}
/// $083080: lsl.w #$2, d0
pub fn i_083080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83082;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $083082: move.w (a1, d0.w), d1
pub fn i_083082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83086;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $083086: move.w $2(a1, d0.w), d2
pub fn i_083086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8308A;
    cpu.ir = 0x3431;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $08308A: add.w d2, $24(a0)
pub fn i_08308a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8308E;
    cpu.ir = 0xD568;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08308E: lea.l $2e58c6.l, a2
pub fn i_08308e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83094;
    cpu.ir = 0x45F9;
    cpu.a[2] = 0x2E58C6;
    20
}
/// $083094: move.w d3, d4
pub fn i_083094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83096;
    cpu.ir = 0x3803;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083096: add.w d4, d3
pub fn i_083096(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83098;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $083098: add.w d4, d3
pub fn i_083098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8309A;
    cpu.ir = 0xD644;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08309A: add.w d3, d3
pub fn i_08309a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8309C;
    cpu.ir = 0xD643;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $08309C: move.w $2(a2, d3.w), $2e(a0)
pub fn i_08309c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830A2;
    cpu.ir = 0x3172;
    let ad2 = cpu.a[2].wrapping_add(0x2).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2E);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0830A2: move.w $4(a2, d3.w), $2a(a0)
pub fn i_0830a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830A8;
    cpu.ir = 0x3172;
    let ad2 = cpu.a[2].wrapping_add(0x4).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x2A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $0830A8: move.w (a2, d3.w), d3
pub fn i_0830a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830AC;
    cpu.ir = 0x3632;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[3] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0830AC: btst.b #$0, $3a(a6)
pub fn i_0830ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0830B2: beq.w $830ba
pub fn i_0830b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x830BA; return 10; }
    8
}
/// $0830B6: neg.w d1
pub fn i_0830b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830B8;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0830B8: neg.w d3
pub fn i_0830b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830BA;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0830BA: add.w d1, $22(a0)
pub fn i_0830ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830BE;
    cpu.ir = 0xD368;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0830BE: add.w d3, $28(a0)
pub fn i_0830be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830C2;
    cpu.ir = 0xD768;
    let ad1 = cpu.a[0].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0830C2: move.w #$10ff, d0
pub fn i_0830c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830C6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0830C6: jsr $2352.l
pub fn i_0830c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x830CC);
    cpu.pc = 0x2352;
    16
}
/// $0830CC: lea.l $82a66(pc), a1
pub fn i_0830cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82A66;
    16
}
/// $0830D0: jsr $4ae.l
pub fn i_0830d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x830D6);
    cpu.pc = 0x4AE;
    16
}
/// $0830D6: jsr $5dd02.l
pub fn i_0830d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x830DC);
    cpu.pc = 0x5DD02;
    16
}
/// $0830DC: move.w $5e(a6), d0
pub fn i_0830dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830E0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0830E0: move.w d0, $5c(a0)
pub fn i_0830e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830E4;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0830E4: move.w d0, d1
pub fn i_0830e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830E6;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0830E6: lsl.w #$3, d0
pub fn i_0830e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830E8;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0830E8: addi.w #$20, d0
pub fn i_0830e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830EC;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0830EC: cmpi.w #$80, d0
pub fn i_0830ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830F0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x80, Size::Word, false);
    8
}
/// $0830F0: bcs.w $830f8
pub fn i_0830f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830F4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x830F8; return 10; }
    8
}
/// $0830F4: move.w #$80, d0
pub fn i_0830f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0830F8: move.w d0, $34(a0)
pub fn i_0830f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830FC;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x34);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0830FC: lsl.w #$2, d1
pub fn i_0830fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x830FE;
    cpu.ir = 0xE549;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0830FE: lea.l $2e589a.l, a1
pub fn i_0830fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83104;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E589A;
    20
}
/// $083104: move.w $2(a1, d1.w), d0
pub fn i_083104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83108;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $083108: add.w d0, $24(a0)
pub fn i_083108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8310C;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08310C: move.w (a1, d1.w), d0
pub fn i_08310c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83110;
    cpu.ir = 0x3031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $083110: btst.b #$0, $3a(a6)
pub fn i_083110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83116;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $083116: beq.w $8311c
pub fn i_083116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8311A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8311C; return 10; }
    8
}
/// $08311A: neg.w d0
pub fn i_08311a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8311C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08311C: add.w d0, $22(a0)
pub fn i_08311c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83120;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083120: rts
pub fn i_083120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83122;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00083122  ($083122, 54 B, enemies/squad/para_squad_helpers_08283c.s)
/// $083122: move.b #$a, $7a(a6)
pub fn i_083122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83128;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083128: subq.b #$1, $7a(a6)
pub fn i_083128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8312C;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x7A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $08312C: lea.l $82480(pc), a1
pub fn i_08312c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83130;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82480;
    16
}
/// $083130: jsr $4ae.l
pub fn i_083130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83136;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83136);
    cpu.pc = 0x4AE;
    16
}
/// $083136: jsr $5dd02.l
pub fn i_083136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8313C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8313C);
    cpu.pc = 0x5DD02;
    16
}
/// $08313C: move.b $7a(a6), d0
pub fn i_08313c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83140;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $083140: andi.w #$f, d0
pub fn i_083140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83144;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $083144: move.w d0, $5e(a0)
pub fn i_083144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83148;
    cpu.ir = 0x3140;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $083148: move.w $5c(a6), $5c(a0)
pub fn i_083148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8314E;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08314E: cmpi.b #$0, $7a(a6)
pub fn i_08314e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83154;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $083154: bgt.b $83128
pub fn i_083154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83156;
    cpu.ir = 0x6ED2;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x83128; return 10; }
    8
}
/// $083156: rts
pub fn i_083156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83158;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00083158  ($083158, 96 B, enemies/squad/para_squad_helpers_08283c.s)
/// $083158: cmpi.b #$3, $7a(a6)
pub fn i_083158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8315E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $08315E: bne.w $83164
pub fn i_08315e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83162;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x83164; return 10; }
    8
}
/// $083162: rts
pub fn i_083162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83164;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $083164: lea.l $82562(pc), a1
pub fn i_083164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83168;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82562;
    16
}
/// $083168: jsr $4ae.l
pub fn i_083168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8316E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8316E);
    cpu.pc = 0x4AE;
    16
}
/// $08316E: jsr $5dd02.l
pub fn i_08316e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83174;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83174);
    cpu.pc = 0x5DD02;
    16
}
/// $083174: move.w $5c(a6), $5c(a0)
pub fn i_083174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8317A;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $08317A: clr.w $5e(a6)
pub fn i_08317a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8317E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x5E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $08317E: lea.l $82562(pc), a1
pub fn i_08317e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83182;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82562;
    16
}
/// $083182: jsr $4ae.l
pub fn i_083182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83188);
    cpu.pc = 0x4AE;
    16
}
/// $083188: jsr $5dd02.l
pub fn i_083188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8318E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8318E);
    cpu.pc = 0x5DD02;
    16
}
/// $08318E: move.w $5c(a6), $5c(a0)
pub fn i_08318e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83194;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $083194: move.w #$1, $5e(a6)
pub fn i_083194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8319A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08319A: lea.l $82562(pc), a1
pub fn i_08319a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8319E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x82562;
    16
}
/// $08319E: jsr $4ae.l
pub fn i_08319e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831A4);
    cpu.pc = 0x4AE;
    16
}
/// $0831A4: jsr $5dd02.l
pub fn i_0831a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831AA);
    cpu.pc = 0x5DD02;
    16
}
/// $0831AA: move.w $5c(a6), $5c(a0)
pub fn i_0831aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831B0;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0831B0: move.w #$2, $5e(a6)
pub fn i_0831b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831B6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0831B6: rts
pub fn i_0831b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000831B8  ($0831B8, 10 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0831B8: move.b #$1, $10a2d1.l
pub fn i_0831b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831C0;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $0831C0: rts
pub fn i_0831c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000831C2  ($0831C2, 24 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0831C2: lea.l $8267c(pc), a1
pub fn i_0831c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831C6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x8267C;
    16
}
/// $0831C6: jsr $4ae.l
pub fn i_0831c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831CC);
    cpu.pc = 0x4AE;
    16
}
/// $0831CC: jsr $5dd02.l
pub fn i_0831cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831D2);
    cpu.pc = 0x5DD02;
    16
}
/// $0831D2: move.w $5c(a6), $5c(a0)
pub fn i_0831d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831D8;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0831D8: rts
pub fn i_0831d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831DA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000831DA  ($0831DA, 66 B, enemies/squad/para_squad_helpers_08283c.s)
/// $0831DA: lea.l $2e5a22.l, a0
pub fn i_0831da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831E0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E5A22;
    20
}
/// $0831E0: move.b $7c(a6), d0
pub fn i_0831e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831E4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0831E4: andi.w #$7, d0
pub fn i_0831e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831E8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0831E8: lsl.w #$2, d0
pub fn i_0831e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831EA;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0831EA: movea.l (a0, d0.w), a1
pub fn i_0831ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831EE;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $0831EE: jsr $77c7e.l
pub fn i_0831ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831F4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x831F4);
    cpu.pc = 0x77C7E;
    16
}
/// $0831F4: move.w #$c000, $38(a0)
pub fn i_0831f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x831FA;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0831FA: lea.l $2e597a.l, a0
pub fn i_0831fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83200;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E597A;
    20
}
/// $083200: move.b $7c(a6), d0
pub fn i_083200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83204;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $083204: andi.w #$7, d0
pub fn i_083204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83208;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $083208: lsl.w #$2, d0
pub fn i_083208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8320A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $08320A: movea.l (a0, d0.w), a1
pub fn i_08320a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8320E;
    cpu.ir = 0x2270;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    18
}
/// $08320E: jsr $77c7e.l
pub fn i_08320e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83214;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83214);
    cpu.pc = 0x77C7E;
    16
}
/// $083214: move.w #$c000, $38(a0)
pub fn i_083214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8321A;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08321A: rts
pub fn i_08321a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8321C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008321C  ($08321C, 56 B, enemies/squad/para_squad_helpers_08283c.s)
/// $08321C: lea.l $77fd6.l, a1
pub fn i_08321c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83222;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $083222: jsr $4ae.l
pub fn i_083222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83228;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83228);
    cpu.pc = 0x4AE;
    16
}
/// $083228: jsr $5dd02.l
pub fn i_083228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8322E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8322E);
    cpu.pc = 0x5DD02;
    16
}
/// $08322E: jsr $5e9b6.l
pub fn i_08322e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83234;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83234);
    cpu.pc = 0x5E9B6;
    16
}
/// $083234: andi.w #$1f, d0
pub fn i_083234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83238;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $083238: subi.w #$10, d0
pub fn i_083238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8323C;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08323C: add.w d0, $22(a0)
pub fn i_08323c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83240;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083240: jsr $5e9b6.l
pub fn i_083240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83246;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83246);
    cpu.pc = 0x5E9B6;
    16
}
/// $083246: andi.w #$3f, d0
pub fn i_083246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8324A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3F) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08324A: subi.w #$10, d0
pub fn i_08324a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8324E;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $08324E: add.w d0, $24(a0)
pub fn i_08324e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83252;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $083252: rts
pub fn i_083252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83254;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaList_083254  ($083254, 6 B, enemies/squad/para_squad_helpers_08283c.s)
/// $083254: lea.l $2e5ab8.l, a1
pub fn i_083254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8325A;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5AB8;
    20
}
// ===== JsrAbsThunk_08325a  ($08325A, 8 B, thunks/jsr_abs_thunks.c)
/// $08325A: jsr $77c7e.l
pub fn i_08325a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83260;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83260);
    cpu.pc = 0x77C7E;
    16
}
/// $083260: rts
pub fn i_083260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83262;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== LeaList_083262  ($083262, 6 B, enemies/bosses/miniboss_module_083262.s)
/// $083262: lea.l $2e5aca.l, a1
pub fn i_083262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83268;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5ACA;
    20
}
// ===== JsrAbsThunk_083268  ($083268, 8 B, thunks/jsr_abs_thunks.c)
/// $083268: jsr $77c7e.l
pub fn i_083268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8326E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8326E);
    cpu.pc = 0x77C7E;
    16
}
/// $08326E: rts
pub fn i_08326e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83270;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_00083270  ($083270, 28 B, enemies/bosses/miniboss_module_083262.s)
/// $083270: move.b #$4, $10a2d0.l
pub fn i_083270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83278;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D0, v1 as u8);
    16
}
/// $083278: move.b #$0, $10a2d1.l
pub fn i_083278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83280;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $083280: movea.l $c(a6), a0
pub fn i_083280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83284;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $083284: move.b #$77, $20(a0)
pub fn i_083284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8328A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x77;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08328A: rts
pub fn i_08328a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8328C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_0008328C  ($08328C, 34 B, enemies/bosses/miniboss_module_083262.s)
/// $08328C: lea.l $2e5adc.l, a1
pub fn i_08328c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83292;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5ADC;
    20
}
/// $083292: jsr $77c7e.l
pub fn i_083292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83298;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83298);
    cpu.pc = 0x77C7E;
    16
}
/// $083298: lea.l $2e5a10.l, a1
pub fn i_083298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8329E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E5A10;
    20
}
/// $08329E: jsr $77c7e.l
pub fn i_08329e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832A4);
    cpu.pc = 0x77C7E;
    16
}
/// $0832A4: move.w #$102f, d0
pub fn i_0832a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832A8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x102F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0832A8: jsr $2352.l
pub fn i_0832a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832AE);
    cpu.pc = 0x2352;
    16
}
// ===== JsrAbsThunk_0832ae  ($0832AE, 8 B, thunks/jsr_abs_thunks.c)
/// $0832AE: jsr $434ce.l
pub fn i_0832ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832B4);
    cpu.pc = 0x434CE;
    16
}
/// $0832B4: rts
pub fn i_0832b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000832B6  ($0832B6, 18 B, enemies/bosses/miniboss_module_083262.s)
/// $0832B6: lea.l $2e545e.l, a0
pub fn i_0832b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832BC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E545E;
    20
}
/// $0832BC: move.w $5c(a6), d0
pub fn i_0832bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832C0;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0832C0: lsl.w #$2, d0
pub fn i_0832c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832C2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $0832C2: move.l (a0, d0.w), $4c(a6)
pub fn i_0832c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832C8;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
// ===== JsrAbsThunk_0832c8  ($0832C8, 8 B, thunks/jsr_abs_thunks.c)
/// $0832C8: jsr $283ca.l
pub fn i_0832c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832CE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832CE);
    cpu.pc = 0x283CA;
    16
}
/// $0832CE: rts
pub fn i_0832ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Sub_000832D0  ($0832D0, 16 B, enemies/bosses/miniboss_module_083262.s)
/// $0832D0: movea.l $8(a6), a1
pub fn i_0832d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832D4;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0832D4: move.b $10(a6), d0
pub fn i_0832d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832D8;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0832D8: cmp.b $10(a1), d0
pub fn i_0832d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832DC;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0832DC: bcs.w $832e6
pub fn i_0832dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832E0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x832E6; return 10; }
    8
}
// ===== ClearXN_0832e0  ($0832E0, 6 B, thunks/ccr_helpers.c)
/// $0832E0: andi.b #$ee, ccr
pub fn i_0832e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832E4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0832E4: rts
pub fn i_0832e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832E6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0832e6  ($0832E6, 6 B, thunks/ccr_helpers.c)
/// $0832E6: ori.b #$11, ccr
pub fn i_0832e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832EA;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0832EA: rts
pub fn i_0832ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_0832ec  ($0832EC, 128 B, enemies/bosses/miniboss_module_083262.s)
/// $0832EC: move.w #$a4, d1
pub fn i_0832ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832F0;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0832F0: jsr $236e.l
pub fn i_0832f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x832F6);
    cpu.pc = 0x236E;
    16
}
/// $0832F6: move.b #$ff, $32(a6)
pub fn i_0832f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x832FC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0832FC: move.b #$ff, $33(a6)
pub fn i_0832fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83302;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083302: move.b #$0, $3a(a6)
pub fn i_083302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83308;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083308: move.w #$0, $38(a6)
pub fn i_083308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8330E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08330E: jsr $267e2.l
pub fn i_08330e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83314;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83314);
    cpu.pc = 0x267E2;
    16
}
/// $083314: move.w #$80, d0
pub fn i_083314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83318;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083318: cmpi.w #$a0, $22(a6)
pub fn i_083318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8331E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $08331E: blt.w $8332a
pub fn i_08331e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83322;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x8332A; return 10; }
    8
}
/// $083322: neg.w d0
pub fn i_083322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83324;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $083324: bset.b #$0, $3a(a6)
pub fn i_083324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8332A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $08332A: move.w d0, $28(a6)
pub fn i_08332a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8332E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08332E: lea.l $ffff.w, a0
pub fn i_08332e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83332;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $083332: move.l a0, $48(a6)
pub fn i_083332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83336;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $083336: lea.l $2e6cdc.l, a0
pub fn i_083336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8333C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6CDC;
    20
}
/// $08333C: jsr $28cd4.l
pub fn i_08333c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83342;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83342);
    cpu.pc = 0x28CD4;
    16
}
/// $083342: lea.l $83348(pc), a1
pub fn i_083342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83346;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x83348;
    16
}
/// $083346: move.l a1, (a6)
pub fn i_083346(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83348;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $083348: jsr $27cee.l
pub fn i_083348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8334E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8334E);
    cpu.pc = 0x27CEE;
    16
}
/// $08334E: jsr $28d70.l
pub fn i_08334e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83354;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83354);
    cpu.pc = 0x28D70;
    16
}
/// $083354: movea.l #$ffffffff, a0
pub fn i_083354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8335A;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $08335A: jsr $5dd5c.l
pub fn i_08335a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83360;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83360);
    cpu.pc = 0x5DD5C;
    16
}
/// $083360: bcc.w $8336a
pub fn i_083360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83364;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x8336A; return 10; }
    8
}
/// $083364: jmp $518.l
pub fn i_083364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8336A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $08336A: rts
pub fn i_08336a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8336C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_08336c  ($08336C, 186 B, enemies/bosses/miniboss_module_083262.s)
/// $08336C: move.w #$a4, d1
pub fn i_08336c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83370;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $083370: jsr $236e.l
pub fn i_083370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83376;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83376);
    cpu.pc = 0x236E;
    16
}
/// $083376: move.b #$ff, $32(a6)
pub fn i_083376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8337C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08337C: move.b #$ff, $33(a6)
pub fn i_08337c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83382;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083382: move.b #$0, $3a(a6)
pub fn i_083382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83388;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083388: move.w #$0, $38(a6)
pub fn i_083388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8338E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08338E: jsr $267e2.l
pub fn i_08338e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83394;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83394);
    cpu.pc = 0x267E2;
    16
}
/// $083394: move.w #$0, $72(a6)
pub fn i_083394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8339A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $08339A: cmpi.w #$a0, $22(a6)
pub fn i_08339a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833A0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xA0, Size::Word, false);
    16
}
/// $0833A0: blt.w $833aa
pub fn i_0833a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833A4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x833AA; return 10; }
    8
}
/// $0833A4: bset.b #$0, $3a(a6)
pub fn i_0833a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833AA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0833AA: lea.l $2e6cdc.l, a0
pub fn i_0833aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E6CDC;
    20
}
/// $0833B0: jsr $28cd4.l
pub fn i_0833b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0833B6: lea.l $ffff.w, a0
pub fn i_0833b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833BA;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0833BA: move.l a0, $48(a6)
pub fn i_0833ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833BE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0833BE: lea.l $833c4(pc), a1
pub fn i_0833be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833C2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x833C4;
    16
}
/// $0833C2: move.l a1, (a6)
pub fn i_0833c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833C4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0833C4: jsr $2783a.l
pub fn i_0833c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833CA);
    cpu.pc = 0x2783A;
    16
}
/// $0833CA: jsr $28d70.l
pub fn i_0833ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833D0);
    cpu.pc = 0x28D70;
    16
}
/// $0833D0: addi.w #$1, $72(a6)
pub fn i_0833d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833D6;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $0833D6: move.w $72(a6), d0
pub fn i_0833d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833DA;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0833DA: btst.b #$1, d0
pub fn i_0833da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833DE;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0833DE: beq.w $8340e
pub fn i_0833de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8340E; return 10; }
    8
}
/// $0833E2: jsr $5e9b6.l
pub fn i_0833e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833E8);
    cpu.pc = 0x5E9B6;
    16
}
/// $0833E8: andi.w #$f, d0
pub fn i_0833e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833EC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0833EC: cmpi.w #$0, d0
pub fn i_0833ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833F0;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $0833F0: bne.w $8340e
pub fn i_0833f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x8340E; return 10; }
    8
}
/// $0833F4: move.w #$50, d0
pub fn i_0833f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833F8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x50;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0833F8: jsr $5e9e4.l
pub fn i_0833f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x833FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x833FE);
    cpu.pc = 0x5E9E4;
    16
}
/// $0833FE: btst.b #$0, $3a(a6)
pub fn i_0833fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83404;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $083404: beq.w $8340a
pub fn i_083404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83408;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x8340A; return 10; }
    8
}
/// $083408: neg.w d0
pub fn i_083408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8340A;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $08340A: add.w d0, $22(a6)
pub fn i_08340a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8340E;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $08340E: movea.l #$ffffffff, a0
pub fn i_08340e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83414;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $083414: jsr $5dd5c.l
pub fn i_083414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8341A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x8341A);
    cpu.pc = 0x5DD5C;
    16
}
/// $08341A: bcc.w $83424
pub fn i_08341a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8341E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x83424; return 10; }
    8
}
/// $08341E: jmp $518.l
pub fn i_08341e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83424;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $083424: rts
pub fn i_083424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83426;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== TaskHandler_083426  ($083426, 126 B, enemies/bosses/miniboss_module_083262.s)
/// $083426: move.w #$a4, d1
pub fn i_083426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8342A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xA4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $08342A: jsr $236e.l
pub fn i_08342a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83430;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x83430);
    cpu.pc = 0x236E;
    16
}
/// $083430: move.b #$ff, $32(a6)
pub fn i_083430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83436;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083436: move.b #$ff, $33(a6)
pub fn i_083436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x8343C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $08343C: move.b #$0, $3a(a6)
pub fn i_08343c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83442;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $083442: move.w #$0, $38(a6)
pub fn i_083442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x83448;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x081A18, i_081a18),
    (0x081A1E, i_081a1e),
    (0x081A22, i_081a22),
    (0x081A26, i_081a26),
    (0x081A2A, i_081a2a),
    (0x081A2C, i_081a2c),
    (0x081A30, i_081a30),
    (0x081A34, i_081a34),
    (0x081A38, i_081a38),
    (0x081A3E, i_081a3e),
    (0x081A40, i_081a40),
    (0x081A44, i_081a44),
    (0x081A4A, i_081a4a),
    (0x081A4E, i_081a4e),
    (0x081A54, i_081a54),
    (0x081A58, i_081a58),
    (0x081A5A, i_081a5a),
    (0x081A60, i_081a60),
    (0x081A64, i_081a64),
    (0x081A6A, i_081a6a),
    (0x081A6E, i_081a6e),
    (0x081A72, i_081a72),
    (0x081A76, i_081a76),
    (0x081A78, i_081a78),
    (0x081A7E, i_081a7e),
    (0x081A82, i_081a82),
    (0x081A86, i_081a86),
    (0x081A8C, i_081a8c),
    (0x081A90, i_081a90),
    (0x081A94, i_081a94),
    (0x081A9A, i_081a9a),
    (0x081AA0, i_081aa0),
    (0x081AA4, i_081aa4),
    (0x081AA8, i_081aa8),
    (0x081AAE, i_081aae),
    (0x081AB2, i_081ab2),
    (0x081AB6, i_081ab6),
    (0x081ABC, i_081abc),
    (0x081AC2, i_081ac2),
    (0x081AC8, i_081ac8),
    (0x081ACC, i_081acc),
    (0x081ACE, i_081ace),
    (0x081AD2, i_081ad2),
    (0x081AD6, i_081ad6),
    (0x081ADA, i_081ada),
    (0x081ADE, i_081ade),
    (0x081AE0, i_081ae0),
    (0x081AE4, i_081ae4),
    (0x081AEA, i_081aea),
    (0x081AF0, i_081af0),
    (0x081AF6, i_081af6),
    (0x081AFA, i_081afa),
    (0x081AFC, i_081afc),
    (0x081B00, i_081b00),
    (0x081B04, i_081b04),
    (0x081B08, i_081b08),
    (0x081B0C, i_081b0c),
    (0x081B0E, i_081b0e),
    (0x081B12, i_081b12),
    (0x081B16, i_081b16),
    (0x081B1C, i_081b1c),
    (0x081B20, i_081b20),
    (0x081B22, i_081b22),
    (0x081B26, i_081b26),
    (0x081B2A, i_081b2a),
    (0x081B2E, i_081b2e),
    (0x081B30, i_081b30),
    (0x081B36, i_081b36),
    (0x081B3A, i_081b3a),
    (0x081B40, i_081b40),
    (0x081B44, i_081b44),
    (0x081B4A, i_081b4a),
    (0x081B4E, i_081b4e),
    (0x081B54, i_081b54),
    (0x081B58, i_081b58),
    (0x081B5E, i_081b5e),
    (0x081B62, i_081b62),
    (0x081B64, i_081b64),
    (0x081B6A, i_081b6a),
    (0x081B6E, i_081b6e),
    (0x081B72, i_081b72),
    (0x081B74, i_081b74),
    (0x081B76, i_081b76),
    (0x081B7C, i_081b7c),
    (0x081B80, i_081b80),
    (0x081B82, i_081b82),
    (0x081B88, i_081b88),
    (0x081B8C, i_081b8c),
    (0x081B90, i_081b90),
    (0x081B92, i_081b92),
    (0x081B94, i_081b94),
    (0x081B98, i_081b98),
    (0x081B9C, i_081b9c),
    (0x081BA0, i_081ba0),
    (0x081BA4, i_081ba4),
    (0x081BAA, i_081baa),
    (0x081BAC, i_081bac),
    (0x081BB0, i_081bb0),
    (0x081BB6, i_081bb6),
    (0x081BBA, i_081bba),
    (0x081BC0, i_081bc0),
    (0x081BC4, i_081bc4),
    (0x081BC6, i_081bc6),
    (0x081BCC, i_081bcc),
    (0x081BD0, i_081bd0),
    (0x081BD6, i_081bd6),
    (0x081BDA, i_081bda),
    (0x081BDE, i_081bde),
    (0x081BE2, i_081be2),
    (0x081BE4, i_081be4),
    (0x081BEA, i_081bea),
    (0x081BEE, i_081bee),
    (0x081BF2, i_081bf2),
    (0x081BF8, i_081bf8),
    (0x081BFC, i_081bfc),
    (0x081C02, i_081c02),
    (0x081C08, i_081c08),
    (0x081C0E, i_081c0e),
    (0x081C16, i_081c16),
    (0x081C1E, i_081c1e),
    (0x081C22, i_081c22),
    (0x081C28, i_081c28),
    (0x081C2E, i_081c2e),
    (0x081C32, i_081c32),
    (0x081C38, i_081c38),
    (0x081C3A, i_081c3a),
    (0x081C3E, i_081c3e),
    (0x081C44, i_081c44),
    (0x081C48, i_081c48),
    (0x081C4E, i_081c4e),
    (0x081C52, i_081c52),
    (0x081C58, i_081c58),
    (0x081C5C, i_081c5c),
    (0x081C5E, i_081c5e),
    (0x081C64, i_081c64),
    (0x081C68, i_081c68),
    (0x081C6E, i_081c6e),
    (0x081C72, i_081c72),
    (0x081C78, i_081c78),
    (0x081C7C, i_081c7c),
    (0x081C82, i_081c82),
    (0x081C88, i_081c88),
    (0x081C8E, i_081c8e),
    (0x081C92, i_081c92),
    (0x081C96, i_081c96),
    (0x081C9C, i_081c9c),
    (0x081CA0, i_081ca0),
    (0x081CA4, i_081ca4),
    (0x081CAA, i_081caa),
    (0x081CAC, i_081cac),
    (0x081CB0, i_081cb0),
    (0x081CB4, i_081cb4),
    (0x081CBA, i_081cba),
    (0x081CBE, i_081cbe),
    (0x081CC4, i_081cc4),
    (0x081CCA, i_081cca),
    (0x081CCE, i_081cce),
    (0x081CD2, i_081cd2),
    (0x081CD4, i_081cd4),
    (0x081CD6, i_081cd6),
    (0x081CDA, i_081cda),
    (0x081CE0, i_081ce0),
    (0x081CE6, i_081ce6),
    (0x081CEC, i_081cec),
    (0x081CF2, i_081cf2),
    (0x081CF8, i_081cf8),
    (0x081CFC, i_081cfc),
    (0x081D00, i_081d00),
    (0x081D04, i_081d04),
    (0x081D0A, i_081d0a),
    (0x081D0C, i_081d0c),
    (0x081D10, i_081d10),
    (0x081D16, i_081d16),
    (0x081D1A, i_081d1a),
    (0x081D20, i_081d20),
    (0x081D24, i_081d24),
    (0x081D26, i_081d26),
    (0x081D2A, i_081d2a),
    (0x081D2E, i_081d2e),
    (0x081D34, i_081d34),
    (0x081D3A, i_081d3a),
    (0x081D3E, i_081d3e),
    (0x081D44, i_081d44),
    (0x081D4A, i_081d4a),
    (0x081D4E, i_081d4e),
    (0x081D52, i_081d52),
    (0x081D54, i_081d54),
    (0x081D58, i_081d58),
    (0x081D5C, i_081d5c),
    (0x081D60, i_081d60),
    (0x081D62, i_081d62),
    (0x081D64, i_081d64),
    (0x081D68, i_081d68),
    (0x081D6E, i_081d6e),
    (0x081D76, i_081d76),
    (0x081D7A, i_081d7a),
    (0x081D7E, i_081d7e),
    (0x081D84, i_081d84),
    (0x081D86, i_081d86),
    (0x081D8A, i_081d8a),
    (0x081D90, i_081d90),
    (0x081D94, i_081d94),
    (0x081D9A, i_081d9a),
    (0x081D9E, i_081d9e),
    (0x081DA0, i_081da0),
    (0x081DA4, i_081da4),
    (0x081DA8, i_081da8),
    (0x081DAE, i_081dae),
    (0x081DB0, i_081db0),
    (0x081DB4, i_081db4),
    (0x081DBA, i_081dba),
    (0x081DC0, i_081dc0),
    (0x081DC6, i_081dc6),
    (0x081DCA, i_081dca),
    (0x081DCE, i_081dce),
    (0x081DD2, i_081dd2),
    (0x081DD8, i_081dd8),
    (0x081DDE, i_081dde),
    (0x081DE4, i_081de4),
    (0x081DE8, i_081de8),
    (0x081DEC, i_081dec),
    (0x081DF2, i_081df2),
    (0x081DF8, i_081df8),
    (0x081DFC, i_081dfc),
    (0x081E02, i_081e02),
    (0x081E08, i_081e08),
    (0x081E0C, i_081e0c),
    (0x081E0E, i_081e0e),
    (0x081E12, i_081e12),
    (0x081E16, i_081e16),
    (0x081E1C, i_081e1c),
    (0x081E20, i_081e20),
    (0x081E24, i_081e24),
    (0x081E2A, i_081e2a),
    (0x081E2E, i_081e2e),
    (0x081E32, i_081e32),
    (0x081E36, i_081e36),
    (0x081E3A, i_081e3a),
    (0x081E40, i_081e40),
    (0x081E44, i_081e44),
    (0x081E48, i_081e48),
    (0x081E4A, i_081e4a),
    (0x081E4E, i_081e4e),
    (0x081E54, i_081e54),
    (0x081E58, i_081e58),
    (0x081E5C, i_081e5c),
    (0x081E62, i_081e62),
    (0x081E68, i_081e68),
    (0x081E6C, i_081e6c),
    (0x081E70, i_081e70),
    (0x081E74, i_081e74),
    (0x081E7A, i_081e7a),
    (0x081E80, i_081e80),
    (0x081E84, i_081e84),
    (0x081E8A, i_081e8a),
    (0x081E8E, i_081e8e),
    (0x081E90, i_081e90),
    (0x081E94, i_081e94),
    (0x081E98, i_081e98),
    (0x081E9C, i_081e9c),
    (0x081EA0, i_081ea0),
    (0x081EA6, i_081ea6),
    (0x081EAA, i_081eaa),
    (0x081EAE, i_081eae),
    (0x081EB2, i_081eb2),
    (0x081EB8, i_081eb8),
    (0x081EBC, i_081ebc),
    (0x081EC0, i_081ec0),
    (0x081EC2, i_081ec2),
    (0x081EC6, i_081ec6),
    (0x081ECA, i_081eca),
    (0x081ECE, i_081ece),
    (0x081ED2, i_081ed2),
    (0x081ED8, i_081ed8),
    (0x081EDC, i_081edc),
    (0x081EE2, i_081ee2),
    (0x081EE6, i_081ee6),
    (0x081EEC, i_081eec),
    (0x081EF0, i_081ef0),
    (0x081EF2, i_081ef2),
    (0x081EF8, i_081ef8),
    (0x081EFC, i_081efc),
    (0x081F00, i_081f00),
    (0x081F06, i_081f06),
    (0x081F0A, i_081f0a),
    (0x081F0E, i_081f0e),
    (0x081F12, i_081f12),
    (0x081F18, i_081f18),
    (0x081F1C, i_081f1c),
    (0x081F22, i_081f22),
    (0x081F26, i_081f26),
    (0x081F2A, i_081f2a),
    (0x081F30, i_081f30),
    (0x081F36, i_081f36),
    (0x081F3C, i_081f3c),
    (0x081F40, i_081f40),
    (0x081F42, i_081f42),
    (0x081F48, i_081f48),
    (0x081F4C, i_081f4c),
    (0x081F50, i_081f50),
    (0x081F56, i_081f56),
    (0x081F5A, i_081f5a),
    (0x081F5E, i_081f5e),
    (0x081F62, i_081f62),
    (0x081F64, i_081f64),
    (0x081F68, i_081f68),
    (0x081F6C, i_081f6c),
    (0x081F72, i_081f72),
    (0x081F76, i_081f76),
    (0x081F7C, i_081f7c),
    (0x081F80, i_081f80),
    (0x081F86, i_081f86),
    (0x081F8C, i_081f8c),
    (0x081F90, i_081f90),
    (0x081F94, i_081f94),
    (0x081F9A, i_081f9a),
    (0x081FA0, i_081fa0),
    (0x081FA4, i_081fa4),
    (0x081FA8, i_081fa8),
    (0x081FAC, i_081fac),
    (0x081FB0, i_081fb0),
    (0x081FB6, i_081fb6),
    (0x081FBA, i_081fba),
    (0x081FC0, i_081fc0),
    (0x081FC6, i_081fc6),
    (0x081FCA, i_081fca),
    (0x081FCC, i_081fcc),
    (0x081FD0, i_081fd0),
    (0x081FD4, i_081fd4),
    (0x081FDA, i_081fda),
    (0x081FDE, i_081fde),
    (0x081FE4, i_081fe4),
    (0x081FE8, i_081fe8),
    (0x081FEC, i_081fec),
    (0x081FEE, i_081fee),
    (0x081FF0, i_081ff0),
    (0x081FF6, i_081ff6),
    (0x081FFC, i_081ffc),
    (0x082002, i_082002),
    (0x082008, i_082008),
    (0x08200E, i_08200e),
    (0x082014, i_082014),
    (0x082018, i_082018),
    (0x08201A, i_08201a),
    (0x08201E, i_08201e),
    (0x082022, i_082022),
    (0x082028, i_082028),
    (0x08202A, i_08202a),
    (0x08202E, i_08202e),
    (0x082034, i_082034),
    (0x082038, i_082038),
    (0x08203C, i_08203c),
    (0x082042, i_082042),
    (0x082046, i_082046),
    (0x08204A, i_08204a),
    (0x08204E, i_08204e),
    (0x082050, i_082050),
    (0x082052, i_082052),
    (0x082058, i_082058),
    (0x08205E, i_08205e),
    (0x082062, i_082062),
    (0x082068, i_082068),
    (0x08206E, i_08206e),
    (0x082074, i_082074),
    (0x082078, i_082078),
    (0x08207E, i_08207e),
    (0x082084, i_082084),
    (0x08208A, i_08208a),
    (0x082090, i_082090),
    (0x082096, i_082096),
    (0x08209A, i_08209a),
    (0x08209E, i_08209e),
    (0x0820A6, i_0820a6),
    (0x0820AC, i_0820ac),
    (0x0820B0, i_0820b0),
    (0x0820B4, i_0820b4),
    (0x0820BA, i_0820ba),
    (0x0820C0, i_0820c0),
    (0x0820C4, i_0820c4),
    (0x0820C8, i_0820c8),
    (0x0820CE, i_0820ce),
    (0x0820D4, i_0820d4),
    (0x0820D8, i_0820d8),
    (0x0820DA, i_0820da),
    (0x0820DE, i_0820de),
    (0x0820E4, i_0820e4),
    (0x0820E8, i_0820e8),
    (0x0820EE, i_0820ee),
    (0x0820F4, i_0820f4),
    (0x0820FA, i_0820fa),
    (0x0820FE, i_0820fe),
    (0x082102, i_082102),
    (0x082106, i_082106),
    (0x08210C, i_08210c),
    (0x082112, i_082112),
    (0x082118, i_082118),
    (0x08211C, i_08211c),
    (0x08211E, i_08211e),
    (0x082122, i_082122),
    (0x082128, i_082128),
    (0x08212C, i_08212c),
    (0x082132, i_082132),
    (0x082136, i_082136),
    (0x08213C, i_08213c),
    (0x082140, i_082140),
    (0x082144, i_082144),
    (0x08214A, i_08214a),
    (0x08214E, i_08214e),
    (0x082152, i_082152),
    (0x082156, i_082156),
    (0x08215A, i_08215a),
    (0x08215C, i_08215c),
    (0x082160, i_082160),
    (0x082164, i_082164),
    (0x082168, i_082168),
    (0x08216E, i_08216e),
    (0x082170, i_082170),
    (0x082174, i_082174),
    (0x08217A, i_08217a),
    (0x08217E, i_08217e),
    (0x082184, i_082184),
    (0x082188, i_082188),
    (0x08218A, i_08218a),
    (0x08218E, i_08218e),
    (0x082194, i_082194),
    (0x082198, i_082198),
    (0x08219E, i_08219e),
    (0x0821A2, i_0821a2),
    (0x0821A6, i_0821a6),
    (0x0821A8, i_0821a8),
    (0x0821AE, i_0821ae),
    (0x0821B2, i_0821b2),
    (0x0821B6, i_0821b6),
    (0x0821B8, i_0821b8),
    (0x0821BC, i_0821bc),
    (0x0821C2, i_0821c2),
    (0x0821C8, i_0821c8),
    (0x0821CC, i_0821cc),
    (0x0821CE, i_0821ce),
    (0x0821D2, i_0821d2),
    (0x0821D8, i_0821d8),
    (0x0821DE, i_0821de),
    (0x0821E2, i_0821e2),
    (0x0821E6, i_0821e6),
    (0x0821EA, i_0821ea),
    (0x0821EE, i_0821ee),
    (0x0821F2, i_0821f2),
    (0x0821F6, i_0821f6),
    (0x0821F8, i_0821f8),
    (0x0821FC, i_0821fc),
    (0x082202, i_082202),
    (0x082208, i_082208),
    (0x08220C, i_08220c),
    (0x08220E, i_08220e),
    (0x082212, i_082212),
    (0x082218, i_082218),
    (0x08221C, i_08221c),
    (0x082220, i_082220),
    (0x082222, i_082222),
    (0x082226, i_082226),
    (0x08222C, i_08222c),
    (0x082230, i_082230),
    (0x082234, i_082234),
    (0x08223A, i_08223a),
    (0x082240, i_082240),
    (0x082244, i_082244),
    (0x08224A, i_08224a),
    (0x08224E, i_08224e),
    (0x082252, i_082252),
    (0x082258, i_082258),
    (0x08225E, i_08225e),
    (0x082262, i_082262),
    (0x082266, i_082266),
    (0x08226A, i_08226a),
    (0x08226E, i_08226e),
    (0x082274, i_082274),
    (0x08227A, i_08227a),
    (0x082280, i_082280),
    (0x082286, i_082286),
    (0x08228A, i_08228a),
    (0x08228C, i_08228c),
    (0x082290, i_082290),
    (0x082296, i_082296),
    (0x08229A, i_08229a),
    (0x0822A0, i_0822a0),
    (0x0822A4, i_0822a4),
    (0x0822AA, i_0822aa),
    (0x0822AE, i_0822ae),
    (0x0822B2, i_0822b2),
    (0x0822B8, i_0822b8),
    (0x0822BC, i_0822bc),
    (0x0822C0, i_0822c0),
    (0x0822C6, i_0822c6),
    (0x0822CC, i_0822cc),
    (0x0822D0, i_0822d0),
    (0x0822D4, i_0822d4),
    (0x0822DA, i_0822da),
    (0x0822DE, i_0822de),
    (0x0822E4, i_0822e4),
    (0x0822E8, i_0822e8),
    (0x0822EC, i_0822ec),
    (0x0822F2, i_0822f2),
    (0x0822F8, i_0822f8),
    (0x0822FC, i_0822fc),
    (0x082300, i_082300),
    (0x082306, i_082306),
    (0x08230A, i_08230a),
    (0x08230E, i_08230e),
    (0x082310, i_082310),
    (0x082314, i_082314),
    (0x08231A, i_08231a),
    (0x082320, i_082320),
    (0x082326, i_082326),
    (0x08232A, i_08232a),
    (0x08232C, i_08232c),
    (0x082330, i_082330),
    (0x082336, i_082336),
    (0x08233A, i_08233a),
    (0x08233E, i_08233e),
    (0x082340, i_082340),
    (0x082344, i_082344),
    (0x082348, i_082348),
    (0x08234E, i_08234e),
    (0x082352, i_082352),
    (0x082358, i_082358),
    (0x08235C, i_08235c),
    (0x082362, i_082362),
    (0x082368, i_082368),
    (0x08236C, i_08236c),
    (0x082370, i_082370),
    (0x082376, i_082376),
    (0x08237C, i_08237c),
    (0x082380, i_082380),
    (0x082384, i_082384),
    (0x082388, i_082388),
    (0x08238C, i_08238c),
    (0x082392, i_082392),
    (0x082396, i_082396),
    (0x08239C, i_08239c),
    (0x0823A0, i_0823a0),
    (0x0823A6, i_0823a6),
    (0x0823A8, i_0823a8),
    (0x0823AE, i_0823ae),
    (0x0823B2, i_0823b2),
    (0x0823B6, i_0823b6),
    (0x0823BA, i_0823ba),
    (0x0823BE, i_0823be),
    (0x0823C4, i_0823c4),
    (0x0823CC, i_0823cc),
    (0x0823D2, i_0823d2),
    (0x0823D8, i_0823d8),
    (0x0823DE, i_0823de),
    (0x0823E4, i_0823e4),
    (0x0823EA, i_0823ea),
    (0x0823F0, i_0823f0),
    (0x0823F4, i_0823f4),
    (0x0823F6, i_0823f6),
    (0x0823FA, i_0823fa),
    (0x082400, i_082400),
    (0x082402, i_082402),
    (0x082406, i_082406),
    (0x08240C, i_08240c),
    (0x082410, i_082410),
    (0x082414, i_082414),
    (0x08241A, i_08241a),
    (0x082420, i_082420),
    (0x082424, i_082424),
    (0x08242A, i_08242a),
    (0x082430, i_082430),
    (0x082436, i_082436),
    (0x08243A, i_08243a),
    (0x08243E, i_08243e),
    (0x082442, i_082442),
    (0x082444, i_082444),
    (0x08244A, i_08244a),
    (0x08244E, i_08244e),
    (0x082452, i_082452),
    (0x082454, i_082454),
    (0x082456, i_082456),
    (0x08245C, i_08245c),
    (0x082462, i_082462),
    (0x082464, i_082464),
    (0x08246A, i_08246a),
    (0x08246C, i_08246c),
    (0x082472, i_082472),
    (0x08247A, i_08247a),
    (0x082480, i_082480),
    (0x082484, i_082484),
    (0x08248A, i_08248a),
    (0x082490, i_082490),
    (0x082494, i_082494),
    (0x082496, i_082496),
    (0x08249A, i_08249a),
    (0x08249E, i_08249e),
    (0x0824A0, i_0824a0),
    (0x0824A4, i_0824a4),
    (0x0824A8, i_0824a8),
    (0x0824AC, i_0824ac),
    (0x0824B0, i_0824b0),
    (0x0824B4, i_0824b4),
    (0x0824BA, i_0824ba),
    (0x0824BC, i_0824bc),
    (0x0824C0, i_0824c0),
    (0x0824C6, i_0824c6),
    (0x0824CA, i_0824ca),
    (0x0824D0, i_0824d0),
    (0x0824D4, i_0824d4),
    (0x0824D6, i_0824d6),
    (0x0824DC, i_0824dc),
    (0x0824E0, i_0824e0),
    (0x0824E4, i_0824e4),
    (0x0824E8, i_0824e8),
    (0x0824EC, i_0824ec),
    (0x0824F2, i_0824f2),
    (0x0824F6, i_0824f6),
    (0x0824FA, i_0824fa),
    (0x0824FC, i_0824fc),
    (0x082502, i_082502),
    (0x082508, i_082508),
    (0x08250C, i_08250c),
    (0x082510, i_082510),
    (0x082512, i_082512),
    (0x082514, i_082514),
    (0x08251A, i_08251a),
    (0x08251E, i_08251e),
    (0x082522, i_082522),
    (0x082528, i_082528),
    (0x08252E, i_08252e),
    (0x082534, i_082534),
    (0x082538, i_082538),
    (0x08253A, i_08253a),
    (0x082540, i_082540),
    (0x082546, i_082546),
    (0x08254C, i_08254c),
    (0x082552, i_082552),
    (0x082556, i_082556),
    (0x08255A, i_08255a),
    (0x08255E, i_08255e),
    (0x082560, i_082560),
    (0x082562, i_082562),
    (0x082568, i_082568),
    (0x08256C, i_08256c),
    (0x08256E, i_08256e),
    (0x082572, i_082572),
    (0x082574, i_082574),
    (0x082576, i_082576),
    (0x08257A, i_08257a),
    (0x08257E, i_08257e),
    (0x082584, i_082584),
    (0x08258A, i_08258a),
    (0x08258E, i_08258e),
    (0x082590, i_082590),
    (0x082592, i_082592),
    (0x082594, i_082594),
    (0x082596, i_082596),
    (0x08259C, i_08259c),
    (0x0825A0, i_0825a0),
    (0x0825A2, i_0825a2),
    (0x0825A6, i_0825a6),
    (0x0825AC, i_0825ac),
    (0x0825B2, i_0825b2),
    (0x0825B8, i_0825b8),
    (0x0825BC, i_0825bc),
    (0x0825C0, i_0825c0),
    (0x0825C4, i_0825c4),
    (0x0825CA, i_0825ca),
    (0x0825CE, i_0825ce),
    (0x0825D4, i_0825d4),
    (0x0825DA, i_0825da),
    (0x0825E0, i_0825e0),
    (0x0825E6, i_0825e6),
    (0x0825EC, i_0825ec),
    (0x0825F0, i_0825f0),
    (0x0825F2, i_0825f2),
    (0x0825F8, i_0825f8),
    (0x0825FE, i_0825fe),
    (0x082602, i_082602),
    (0x082606, i_082606),
    (0x082608, i_082608),
    (0x08260E, i_08260e),
    (0x082614, i_082614),
    (0x08261A, i_08261a),
    (0x08261E, i_08261e),
    (0x082622, i_082622),
    (0x082624, i_082624),
    (0x082626, i_082626),
    (0x08262A, i_08262a),
    (0x082630, i_082630),
    (0x082636, i_082636),
    (0x08263C, i_08263c),
    (0x082640, i_082640),
    (0x082642, i_082642),
    (0x082648, i_082648),
    (0x08264C, i_08264c),
    (0x082652, i_082652),
    (0x082656, i_082656),
    (0x08265A, i_08265a),
    (0x082660, i_082660),
    (0x082664, i_082664),
    (0x082666, i_082666),
    (0x08266C, i_08266c),
    (0x082670, i_082670),
    (0x082674, i_082674),
    (0x082678, i_082678),
    (0x08267A, i_08267a),
    (0x08267C, i_08267c),
    (0x082680, i_082680),
    (0x082686, i_082686),
    (0x08268A, i_08268a),
    (0x08268C, i_08268c),
    (0x082692, i_082692),
    (0x082698, i_082698),
    (0x08269C, i_08269c),
    (0x08269E, i_08269e),
    (0x0826A4, i_0826a4),
    (0x0826A8, i_0826a8),
    (0x0826AC, i_0826ac),
    (0x0826B2, i_0826b2),
    (0x0826B8, i_0826b8),
    (0x0826BE, i_0826be),
    (0x0826C2, i_0826c2),
    (0x0826C6, i_0826c6),
    (0x0826CA, i_0826ca),
    (0x0826CC, i_0826cc),
    (0x0826D2, i_0826d2),
    (0x0826D8, i_0826d8),
    (0x0826DE, i_0826de),
    (0x0826E2, i_0826e2),
    (0x0826E4, i_0826e4),
    (0x0826E8, i_0826e8),
    (0x0826EC, i_0826ec),
    (0x0826F0, i_0826f0),
    (0x0826F6, i_0826f6),
    (0x0826FC, i_0826fc),
    (0x082702, i_082702),
    (0x082706, i_082706),
    (0x08270A, i_08270a),
    (0x08270E, i_08270e),
    (0x082712, i_082712),
    (0x082716, i_082716),
    (0x08271A, i_08271a),
    (0x08271E, i_08271e),
    (0x082720, i_082720),
    (0x082724, i_082724),
    (0x08272A, i_08272a),
    (0x08272E, i_08272e),
    (0x082734, i_082734),
    (0x08273A, i_08273a),
    (0x082740, i_082740),
    (0x082746, i_082746),
    (0x08274C, i_08274c),
    (0x082750, i_082750),
    (0x082756, i_082756),
    (0x08275C, i_08275c),
    (0x082760, i_082760),
    (0x082762, i_082762),
    (0x082768, i_082768),
    (0x08276C, i_08276c),
    (0x082770, i_082770),
    (0x082772, i_082772),
    (0x082778, i_082778),
    (0x08277E, i_08277e),
    (0x082782, i_082782),
    (0x082788, i_082788),
    (0x08278E, i_08278e),
    (0x082792, i_082792),
    (0x082798, i_082798),
    (0x08279C, i_08279c),
    (0x0827A2, i_0827a2),
    (0x0827A6, i_0827a6),
    (0x0827AA, i_0827aa),
    (0x0827AE, i_0827ae),
    (0x0827B2, i_0827b2),
    (0x0827B6, i_0827b6),
    (0x0827BA, i_0827ba),
    (0x0827BE, i_0827be),
    (0x0827C2, i_0827c2),
    (0x0827C6, i_0827c6),
    (0x0827CA, i_0827ca),
    (0x0827CE, i_0827ce),
    (0x0827D2, i_0827d2),
    (0x0827D6, i_0827d6),
    (0x0827DA, i_0827da),
    (0x0827DE, i_0827de),
    (0x0827E2, i_0827e2),
    (0x0827E8, i_0827e8),
    (0x0827EE, i_0827ee),
    (0x0827F4, i_0827f4),
    (0x0827F8, i_0827f8),
    (0x0827FC, i_0827fc),
    (0x0827FE, i_0827fe),
    (0x082804, i_082804),
    (0x08280A, i_08280a),
    (0x08280E, i_08280e),
    (0x082812, i_082812),
    (0x082814, i_082814),
    (0x08281A, i_08281a),
    (0x08281E, i_08281e),
    (0x082822, i_082822),
    (0x082824, i_082824),
    (0x08282A, i_08282a),
    (0x082830, i_082830),
    (0x082834, i_082834),
    (0x082838, i_082838),
    (0x08283A, i_08283a),
    (0x08283C, i_08283c),
    (0x082842, i_082842),
    (0x082848, i_082848),
    (0x08284E, i_08284e),
    (0x082854, i_082854),
    (0x082858, i_082858),
    (0x08285A, i_08285a),
    (0x082860, i_082860),
    (0x082864, i_082864),
    (0x082868, i_082868),
    (0x08286A, i_08286a),
    (0x082870, i_082870),
    (0x082876, i_082876),
    (0x08287A, i_08287a),
    (0x082880, i_082880),
    (0x082884, i_082884),
    (0x08288A, i_08288a),
    (0x082890, i_082890),
    (0x082896, i_082896),
    (0x08289C, i_08289c),
    (0x0828A2, i_0828a2),
    (0x0828A6, i_0828a6),
    (0x0828A8, i_0828a8),
    (0x0828AE, i_0828ae),
    (0x0828B4, i_0828b4),
    (0x0828B8, i_0828b8),
    (0x0828BC, i_0828bc),
    (0x0828BE, i_0828be),
    (0x0828C4, i_0828c4),
    (0x0828C8, i_0828c8),
    (0x0828CE, i_0828ce),
    (0x0828D2, i_0828d2),
    (0x0828DA, i_0828da),
    (0x0828E0, i_0828e0),
    (0x0828E4, i_0828e4),
    (0x0828EA, i_0828ea),
    (0x0828EE, i_0828ee),
    (0x0828F2, i_0828f2),
    (0x0828F8, i_0828f8),
    (0x0828FE, i_0828fe),
    (0x082904, i_082904),
    (0x082908, i_082908),
    (0x08290A, i_08290a),
    (0x082910, i_082910),
    (0x082916, i_082916),
    (0x08291C, i_08291c),
    (0x082920, i_082920),
    (0x082924, i_082924),
    (0x082928, i_082928),
    (0x08292A, i_08292a),
    (0x08292C, i_08292c),
    (0x082930, i_082930),
    (0x082936, i_082936),
    (0x08293A, i_08293a),
    (0x082940, i_082940),
    (0x082946, i_082946),
    (0x08294C, i_08294c),
    (0x082952, i_082952),
    (0x082956, i_082956),
    (0x08295A, i_08295a),
    (0x08295E, i_08295e),
    (0x082964, i_082964),
    (0x082968, i_082968),
    (0x08296C, i_08296c),
    (0x082972, i_082972),
    (0x082978, i_082978),
    (0x08297C, i_08297c),
    (0x08297E, i_08297e),
    (0x082984, i_082984),
    (0x082988, i_082988),
    (0x08298C, i_08298c),
    (0x08298E, i_08298e),
    (0x082994, i_082994),
    (0x08299A, i_08299a),
    (0x0829A0, i_0829a0),
    (0x0829A4, i_0829a4),
    (0x0829A8, i_0829a8),
    (0x0829AA, i_0829aa),
    (0x0829AC, i_0829ac),
    (0x0829B0, i_0829b0),
    (0x0829B6, i_0829b6),
    (0x0829BA, i_0829ba),
    (0x0829BC, i_0829bc),
    (0x0829C2, i_0829c2),
    (0x0829C6, i_0829c6),
    (0x0829CC, i_0829cc),
    (0x0829D0, i_0829d0),
    (0x0829D2, i_0829d2),
    (0x0829D6, i_0829d6),
    (0x0829DA, i_0829da),
    (0x0829DE, i_0829de),
    (0x0829E2, i_0829e2),
    (0x0829E8, i_0829e8),
    (0x0829EC, i_0829ec),
    (0x0829F2, i_0829f2),
    (0x0829F6, i_0829f6),
    (0x0829F8, i_0829f8),
    (0x0829FC, i_0829fc),
    (0x082A02, i_082a02),
    (0x082A06, i_082a06),
    (0x082A0A, i_082a0a),
    (0x082A0E, i_082a0e),
    (0x082A14, i_082a14),
    (0x082A1A, i_082a1a),
    (0x082A20, i_082a20),
    (0x082A26, i_082a26),
    (0x082A2C, i_082a2c),
    (0x082A30, i_082a30),
    (0x082A32, i_082a32),
    (0x082A38, i_082a38),
    (0x082A3E, i_082a3e),
    (0x082A42, i_082a42),
    (0x082A46, i_082a46),
    (0x082A48, i_082a48),
    (0x082A4E, i_082a4e),
    (0x082A54, i_082a54),
    (0x082A5A, i_082a5a),
    (0x082A5E, i_082a5e),
    (0x082A62, i_082a62),
    (0x082A64, i_082a64),
    (0x082A66, i_082a66),
    (0x082A6A, i_082a6a),
    (0x082A6C, i_082a6c),
    (0x082A70, i_082a70),
    (0x082A76, i_082a76),
    (0x082A7C, i_082a7c),
    (0x082A80, i_082a80),
    (0x082A84, i_082a84),
    (0x082A86, i_082a86),
    (0x082A88, i_082a88),
    (0x082A8A, i_082a8a),
    (0x082A8C, i_082a8c),
    (0x082A92, i_082a92),
    (0x082A96, i_082a96),
    (0x082A98, i_082a98),
    (0x082A9C, i_082a9c),
    (0x082AA0, i_082aa0),
    (0x082AA4, i_082aa4),
    (0x082AAA, i_082aaa),
    (0x082AAE, i_082aae),
    (0x082AB4, i_082ab4),
    (0x082ABA, i_082aba),
    (0x082ABE, i_082abe),
    (0x082AC4, i_082ac4),
    (0x082ACA, i_082aca),
    (0x082AD0, i_082ad0),
    (0x082AD6, i_082ad6),
    (0x082ADC, i_082adc),
    (0x082AE0, i_082ae0),
    (0x082AE2, i_082ae2),
    (0x082AE8, i_082ae8),
    (0x082AEC, i_082aec),
    (0x082AF2, i_082af2),
    (0x082AF4, i_082af4),
    (0x082AFA, i_082afa),
    (0x082AFE, i_082afe),
    (0x082B02, i_082b02),
    (0x082B08, i_082b08),
    (0x082B0C, i_082b0c),
    (0x082B10, i_082b10),
    (0x082B14, i_082b14),
    (0x082B16, i_082b16),
    (0x082B1C, i_082b1c),
    (0x082B22, i_082b22),
    (0x082B26, i_082b26),
    (0x082B2C, i_082b2c),
    (0x082B2E, i_082b2e),
    (0x082B34, i_082b34),
    (0x082B3A, i_082b3a),
    (0x082B3E, i_082b3e),
    (0x082B42, i_082b42),
    (0x082B44, i_082b44),
    (0x082B46, i_082b46),
    (0x082B4A, i_082b4a),
    (0x082B50, i_082b50),
    (0x082B54, i_082b54),
    (0x082B5A, i_082b5a),
    (0x082B60, i_082b60),
    (0x082B66, i_082b66),
    (0x082B6C, i_082b6c),
    (0x082B72, i_082b72),
    (0x082B78, i_082b78),
    (0x082B7E, i_082b7e),
    (0x082B82, i_082b82),
    (0x082B84, i_082b84),
    (0x082B8A, i_082b8a),
    (0x082B90, i_082b90),
    (0x082B94, i_082b94),
    (0x082B98, i_082b98),
    (0x082B9A, i_082b9a),
    (0x082BA0, i_082ba0),
    (0x082BA6, i_082ba6),
    (0x082BAC, i_082bac),
    (0x082BB0, i_082bb0),
    (0x082BB4, i_082bb4),
    (0x082BB6, i_082bb6),
    (0x082BB8, i_082bb8),
    (0x082BBC, i_082bbc),
    (0x082BC2, i_082bc2),
    (0x082BC8, i_082bc8),
    (0x082BCE, i_082bce),
    (0x082BD4, i_082bd4),
    (0x082BDA, i_082bda),
    (0x082BDE, i_082bde),
    (0x082BE0, i_082be0),
    (0x082BE6, i_082be6),
    (0x082BEC, i_082bec),
    (0x082BF0, i_082bf0),
    (0x082BF4, i_082bf4),
    (0x082BF6, i_082bf6),
    (0x082BFC, i_082bfc),
    (0x082C00, i_082c00),
    (0x082C04, i_082c04),
    (0x082C06, i_082c06),
    (0x082C08, i_082c08),
    (0x082C0E, i_082c0e),
    (0x082C12, i_082c12),
    (0x082C16, i_082c16),
    (0x082C18, i_082c18),
    (0x082C1E, i_082c1e),
    (0x082C22, i_082c22),
    (0x082C28, i_082c28),
    (0x082C2C, i_082c2c),
    (0x082C32, i_082c32),
    (0x082C34, i_082c34),
    (0x082C38, i_082c38),
    (0x082C3E, i_082c3e),
    (0x082C42, i_082c42),
    (0x082C48, i_082c48),
    (0x082C4E, i_082c4e),
    (0x082C54, i_082c54),
    (0x082C58, i_082c58),
    (0x082C5E, i_082c5e),
    (0x082C62, i_082c62),
    (0x082C66, i_082c66),
    (0x082C6C, i_082c6c),
    (0x082C6E, i_082c6e),
    (0x082C74, i_082c74),
    (0x082C7A, i_082c7a),
    (0x082C7C, i_082c7c),
    (0x082C80, i_082c80),
    (0x082C84, i_082c84),
    (0x082C88, i_082c88),
    (0x082C8A, i_082c8a),
    (0x082C90, i_082c90),
    (0x082C94, i_082c94),
    (0x082C98, i_082c98),
    (0x082C9C, i_082c9c),
    (0x082CA2, i_082ca2),
    (0x082CA6, i_082ca6),
    (0x082CAA, i_082caa),
    (0x082CAC, i_082cac),
    (0x082CB0, i_082cb0),
    (0x082CB2, i_082cb2),
    (0x082CB6, i_082cb6),
    (0x082CBC, i_082cbc),
    (0x082CC2, i_082cc2),
    (0x082CC4, i_082cc4),
    (0x082CC8, i_082cc8),
    (0x082CCE, i_082cce),
    (0x082CD4, i_082cd4),
    (0x082CDA, i_082cda),
    (0x082CE0, i_082ce0),
    (0x082CE2, i_082ce2),
    (0x082CE6, i_082ce6),
    (0x082CEC, i_082cec),
    (0x082CF2, i_082cf2),
    (0x082CF6, i_082cf6),
    (0x082CFA, i_082cfa),
    (0x082CFE, i_082cfe),
    (0x082D02, i_082d02),
    (0x082D06, i_082d06),
    (0x082D0A, i_082d0a),
    (0x082D10, i_082d10),
    (0x082D12, i_082d12),
    (0x082D18, i_082d18),
    (0x082D1C, i_082d1c),
    (0x082D22, i_082d22),
    (0x082D28, i_082d28),
    (0x082D2E, i_082d2e),
    (0x082D34, i_082d34),
    (0x082D38, i_082d38),
    (0x082D3E, i_082d3e),
    (0x082D40, i_082d40),
    (0x082D46, i_082d46),
    (0x082D4A, i_082d4a),
    (0x082D50, i_082d50),
    (0x082D54, i_082d54),
    (0x082D5A, i_082d5a),
    (0x082D5E, i_082d5e),
    (0x082D64, i_082d64),
    (0x082D68, i_082d68),
    (0x082D6E, i_082d6e),
    (0x082D70, i_082d70),
    (0x082D74, i_082d74),
    (0x082D78, i_082d78),
    (0x082D7C, i_082d7c),
    (0x082D80, i_082d80),
    (0x082D84, i_082d84),
    (0x082D8A, i_082d8a),
    (0x082D8E, i_082d8e),
    (0x082D92, i_082d92),
    (0x082D98, i_082d98),
    (0x082D9E, i_082d9e),
    (0x082DA2, i_082da2),
    (0x082DA6, i_082da6),
    (0x082DAC, i_082dac),
    (0x082DAE, i_082dae),
    (0x082DB4, i_082db4),
    (0x082DB8, i_082db8),
    (0x082DBE, i_082dbe),
    (0x082DC2, i_082dc2),
    (0x082DC8, i_082dc8),
    (0x082DCC, i_082dcc),
    (0x082DD0, i_082dd0),
    (0x082DD4, i_082dd4),
    (0x082DD8, i_082dd8),
    (0x082DDC, i_082ddc),
    (0x082DE0, i_082de0),
    (0x082DE4, i_082de4),
    (0x082DE8, i_082de8),
    (0x082DEC, i_082dec),
    (0x082DEE, i_082dee),
    (0x082DF2, i_082df2),
    (0x082DF6, i_082df6),
    (0x082DF8, i_082df8),
    (0x082DFE, i_082dfe),
    (0x082E02, i_082e02),
    (0x082E04, i_082e04),
    (0x082E0A, i_082e0a),
    (0x082E0E, i_082e0e),
    (0x082E14, i_082e14),
    (0x082E18, i_082e18),
    (0x082E1E, i_082e1e),
    (0x082E22, i_082e22),
    (0x082E26, i_082e26),
    (0x082E2A, i_082e2a),
    (0x082E2E, i_082e2e),
    (0x082E32, i_082e32),
    (0x082E38, i_082e38),
    (0x082E3C, i_082e3c),
    (0x082E40, i_082e40),
    (0x082E44, i_082e44),
    (0x082E48, i_082e48),
    (0x082E4C, i_082e4c),
    (0x082E50, i_082e50),
    (0x082E56, i_082e56),
    (0x082E5A, i_082e5a),
    (0x082E5E, i_082e5e),
    (0x082E62, i_082e62),
    (0x082E66, i_082e66),
    (0x082E6A, i_082e6a),
    (0x082E6E, i_082e6e),
    (0x082E72, i_082e72),
    (0x082E74, i_082e74),
    (0x082E78, i_082e78),
    (0x082E7A, i_082e7a),
    (0x082E7E, i_082e7e),
    (0x082E82, i_082e82),
    (0x082E86, i_082e86),
    (0x082E8C, i_082e8c),
    (0x082E90, i_082e90),
    (0x082E94, i_082e94),
    (0x082E98, i_082e98),
    (0x082E9E, i_082e9e),
    (0x082EA2, i_082ea2),
    (0x082EA8, i_082ea8),
    (0x082EAC, i_082eac),
    (0x082EB2, i_082eb2),
    (0x082EB6, i_082eb6),
    (0x082EBC, i_082ebc),
    (0x082EC0, i_082ec0),
    (0x082EC4, i_082ec4),
    (0x082ECA, i_082eca),
    (0x082ECE, i_082ece),
    (0x082ED4, i_082ed4),
    (0x082ED8, i_082ed8),
    (0x082EDA, i_082eda),
    (0x082EDE, i_082ede),
    (0x082EE0, i_082ee0),
    (0x082EE4, i_082ee4),
    (0x082EE8, i_082ee8),
    (0x082EEC, i_082eec),
    (0x082EF0, i_082ef0),
    (0x082EF2, i_082ef2),
    (0x082EF6, i_082ef6),
    (0x082EF8, i_082ef8),
    (0x082EFC, i_082efc),
    (0x082F02, i_082f02),
    (0x082F06, i_082f06),
    (0x082F0A, i_082f0a),
    (0x082F10, i_082f10),
    (0x082F14, i_082f14),
    (0x082F16, i_082f16),
    (0x082F18, i_082f18),
    (0x082F1C, i_082f1c),
    (0x082F1E, i_082f1e),
    (0x082F22, i_082f22),
    (0x082F24, i_082f24),
    (0x082F28, i_082f28),
    (0x082F2E, i_082f2e),
    (0x082F34, i_082f34),
    (0x082F3A, i_082f3a),
    (0x082F3E, i_082f3e),
    (0x082F40, i_082f40),
    (0x082F44, i_082f44),
    (0x082F48, i_082f48),
    (0x082F4C, i_082f4c),
    (0x082F50, i_082f50),
    (0x082F54, i_082f54),
    (0x082F56, i_082f56),
    (0x082F58, i_082f58),
    (0x082F5C, i_082f5c),
    (0x082F60, i_082f60),
    (0x082F64, i_082f64),
    (0x082F66, i_082f66),
    (0x082F6A, i_082f6a),
    (0x082F6E, i_082f6e),
    (0x082F70, i_082f70),
    (0x082F74, i_082f74),
    (0x082F78, i_082f78),
    (0x082F7C, i_082f7c),
    (0x082F7E, i_082f7e),
    (0x082F80, i_082f80),
    (0x082F84, i_082f84),
    (0x082F8A, i_082f8a),
    (0x082F8E, i_082f8e),
    (0x082F90, i_082f90),
    (0x082F92, i_082f92),
    (0x082F98, i_082f98),
    (0x082F9C, i_082f9c),
    (0x082FA0, i_082fa0),
    (0x082FA4, i_082fa4),
    (0x082FAA, i_082faa),
    (0x082FAE, i_082fae),
    (0x082FB4, i_082fb4),
    (0x082FB6, i_082fb6),
    (0x082FBC, i_082fbc),
    (0x082FC0, i_082fc0),
    (0x082FC4, i_082fc4),
    (0x082FC8, i_082fc8),
    (0x082FCA, i_082fca),
    (0x082FD0, i_082fd0),
    (0x082FD6, i_082fd6),
    (0x082FDC, i_082fdc),
    (0x082FE0, i_082fe0),
    (0x082FE2, i_082fe2),
    (0x082FE8, i_082fe8),
    (0x082FEC, i_082fec),
    (0x082FF0, i_082ff0),
    (0x082FF4, i_082ff4),
    (0x082FFA, i_082ffa),
    (0x082FFE, i_082ffe),
    (0x083000, i_083000),
    (0x083004, i_083004),
    (0x083008, i_083008),
    (0x08300E, i_08300e),
    (0x083014, i_083014),
    (0x083018, i_083018),
    (0x08301A, i_08301a),
    (0x08301C, i_08301c),
    (0x083022, i_083022),
    (0x083026, i_083026),
    (0x08302A, i_08302a),
    (0x08302E, i_08302e),
    (0x083034, i_083034),
    (0x083036, i_083036),
    (0x083038, i_083038),
    (0x08303A, i_08303a),
    (0x08303C, i_08303c),
    (0x083042, i_083042),
    (0x083048, i_083048),
    (0x08304C, i_08304c),
    (0x083052, i_083052),
    (0x083056, i_083056),
    (0x083058, i_083058),
    (0x08305A, i_08305a),
    (0x08305E, i_08305e),
    (0x083062, i_083062),
    (0x083064, i_083064),
    (0x083068, i_083068),
    (0x08306E, i_08306e),
    (0x083074, i_083074),
    (0x083078, i_083078),
    (0x08307A, i_08307a),
    (0x083080, i_083080),
    (0x083082, i_083082),
    (0x083086, i_083086),
    (0x08308A, i_08308a),
    (0x08308E, i_08308e),
    (0x083094, i_083094),
    (0x083096, i_083096),
    (0x083098, i_083098),
    (0x08309A, i_08309a),
    (0x08309C, i_08309c),
    (0x0830A2, i_0830a2),
    (0x0830A8, i_0830a8),
    (0x0830AC, i_0830ac),
    (0x0830B2, i_0830b2),
    (0x0830B6, i_0830b6),
    (0x0830B8, i_0830b8),
    (0x0830BA, i_0830ba),
    (0x0830BE, i_0830be),
    (0x0830C2, i_0830c2),
    (0x0830C6, i_0830c6),
    (0x0830CC, i_0830cc),
    (0x0830D0, i_0830d0),
    (0x0830D6, i_0830d6),
    (0x0830DC, i_0830dc),
    (0x0830E0, i_0830e0),
    (0x0830E4, i_0830e4),
    (0x0830E6, i_0830e6),
    (0x0830E8, i_0830e8),
    (0x0830EC, i_0830ec),
    (0x0830F0, i_0830f0),
    (0x0830F4, i_0830f4),
    (0x0830F8, i_0830f8),
    (0x0830FC, i_0830fc),
    (0x0830FE, i_0830fe),
    (0x083104, i_083104),
    (0x083108, i_083108),
    (0x08310C, i_08310c),
    (0x083110, i_083110),
    (0x083116, i_083116),
    (0x08311A, i_08311a),
    (0x08311C, i_08311c),
    (0x083120, i_083120),
    (0x083122, i_083122),
    (0x083128, i_083128),
    (0x08312C, i_08312c),
    (0x083130, i_083130),
    (0x083136, i_083136),
    (0x08313C, i_08313c),
    (0x083140, i_083140),
    (0x083144, i_083144),
    (0x083148, i_083148),
    (0x08314E, i_08314e),
    (0x083154, i_083154),
    (0x083156, i_083156),
    (0x083158, i_083158),
    (0x08315E, i_08315e),
    (0x083162, i_083162),
    (0x083164, i_083164),
    (0x083168, i_083168),
    (0x08316E, i_08316e),
    (0x083174, i_083174),
    (0x08317A, i_08317a),
    (0x08317E, i_08317e),
    (0x083182, i_083182),
    (0x083188, i_083188),
    (0x08318E, i_08318e),
    (0x083194, i_083194),
    (0x08319A, i_08319a),
    (0x08319E, i_08319e),
    (0x0831A4, i_0831a4),
    (0x0831AA, i_0831aa),
    (0x0831B0, i_0831b0),
    (0x0831B6, i_0831b6),
    (0x0831B8, i_0831b8),
    (0x0831C0, i_0831c0),
    (0x0831C2, i_0831c2),
    (0x0831C6, i_0831c6),
    (0x0831CC, i_0831cc),
    (0x0831D2, i_0831d2),
    (0x0831D8, i_0831d8),
    (0x0831DA, i_0831da),
    (0x0831E0, i_0831e0),
    (0x0831E4, i_0831e4),
    (0x0831E8, i_0831e8),
    (0x0831EA, i_0831ea),
    (0x0831EE, i_0831ee),
    (0x0831F4, i_0831f4),
    (0x0831FA, i_0831fa),
    (0x083200, i_083200),
    (0x083204, i_083204),
    (0x083208, i_083208),
    (0x08320A, i_08320a),
    (0x08320E, i_08320e),
    (0x083214, i_083214),
    (0x08321A, i_08321a),
    (0x08321C, i_08321c),
    (0x083222, i_083222),
    (0x083228, i_083228),
    (0x08322E, i_08322e),
    (0x083234, i_083234),
    (0x083238, i_083238),
    (0x08323C, i_08323c),
    (0x083240, i_083240),
    (0x083246, i_083246),
    (0x08324A, i_08324a),
    (0x08324E, i_08324e),
    (0x083252, i_083252),
    (0x083254, i_083254),
    (0x08325A, i_08325a),
    (0x083260, i_083260),
    (0x083262, i_083262),
    (0x083268, i_083268),
    (0x08326E, i_08326e),
    (0x083270, i_083270),
    (0x083278, i_083278),
    (0x083280, i_083280),
    (0x083284, i_083284),
    (0x08328A, i_08328a),
    (0x08328C, i_08328c),
    (0x083292, i_083292),
    (0x083298, i_083298),
    (0x08329E, i_08329e),
    (0x0832A4, i_0832a4),
    (0x0832A8, i_0832a8),
    (0x0832AE, i_0832ae),
    (0x0832B4, i_0832b4),
    (0x0832B6, i_0832b6),
    (0x0832BC, i_0832bc),
    (0x0832C0, i_0832c0),
    (0x0832C2, i_0832c2),
    (0x0832C8, i_0832c8),
    (0x0832CE, i_0832ce),
    (0x0832D0, i_0832d0),
    (0x0832D4, i_0832d4),
    (0x0832D8, i_0832d8),
    (0x0832DC, i_0832dc),
    (0x0832E0, i_0832e0),
    (0x0832E4, i_0832e4),
    (0x0832E6, i_0832e6),
    (0x0832EA, i_0832ea),
    (0x0832EC, i_0832ec),
    (0x0832F0, i_0832f0),
    (0x0832F6, i_0832f6),
    (0x0832FC, i_0832fc),
    (0x083302, i_083302),
    (0x083308, i_083308),
    (0x08330E, i_08330e),
    (0x083314, i_083314),
    (0x083318, i_083318),
    (0x08331E, i_08331e),
    (0x083322, i_083322),
    (0x083324, i_083324),
    (0x08332A, i_08332a),
    (0x08332E, i_08332e),
    (0x083332, i_083332),
    (0x083336, i_083336),
    (0x08333C, i_08333c),
    (0x083342, i_083342),
    (0x083346, i_083346),
    (0x083348, i_083348),
    (0x08334E, i_08334e),
    (0x083354, i_083354),
    (0x08335A, i_08335a),
    (0x083360, i_083360),
    (0x083364, i_083364),
    (0x08336A, i_08336a),
    (0x08336C, i_08336c),
    (0x083370, i_083370),
    (0x083376, i_083376),
    (0x08337C, i_08337c),
    (0x083382, i_083382),
    (0x083388, i_083388),
    (0x08338E, i_08338e),
    (0x083394, i_083394),
    (0x08339A, i_08339a),
    (0x0833A0, i_0833a0),
    (0x0833A4, i_0833a4),
    (0x0833AA, i_0833aa),
    (0x0833B0, i_0833b0),
    (0x0833B6, i_0833b6),
    (0x0833BA, i_0833ba),
    (0x0833BE, i_0833be),
    (0x0833C2, i_0833c2),
    (0x0833C4, i_0833c4),
    (0x0833CA, i_0833ca),
    (0x0833D0, i_0833d0),
    (0x0833D6, i_0833d6),
    (0x0833DA, i_0833da),
    (0x0833DE, i_0833de),
    (0x0833E2, i_0833e2),
    (0x0833E8, i_0833e8),
    (0x0833EC, i_0833ec),
    (0x0833F0, i_0833f0),
    (0x0833F4, i_0833f4),
    (0x0833F8, i_0833f8),
    (0x0833FE, i_0833fe),
    (0x083404, i_083404),
    (0x083408, i_083408),
    (0x08340A, i_08340a),
    (0x08340E, i_08340e),
    (0x083414, i_083414),
    (0x08341A, i_08341a),
    (0x08341E, i_08341e),
    (0x083424, i_083424),
    (0x083426, i_083426),
    (0x08342A, i_08342a),
    (0x083430, i_083430),
    (0x083436, i_083436),
    (0x08343C, i_08343c),
    (0x083442, i_083442),
];
