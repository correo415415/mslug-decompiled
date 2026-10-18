// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $02DD66: move.l a0, $48(a6)
pub fn i_02dd66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD6A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02DD6A: lea.l $2de18(pc), a1
pub fn i_02dd6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD6E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DE18;
    16
}
/// $02DD6E: move.l a1, (a6)
pub fn i_02dd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD70;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DD70: jmp $2de18(pc)
pub fn i_02dd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DD74;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DE18;
    8
}
// ===== Slug_Idle_02de18  ($02DE18, 156 B, slug/vehicle_02dd20.s)
/// $02DE18: jsr $2aa24(pc)
pub fn i_02de18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE1C);
    cpu.pc = 0x2AA24;
    16
}
/// $02DE1C: bset.b #$4, $8d(a6)
pub fn i_02de1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE22;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DE22: bset.b #$5, $8d(a6)
pub fn i_02de22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE28;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02DE28: clr.w $28(a6)
pub fn i_02de28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE2C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02DE2C: jsr $2a824(pc)
pub fn i_02de2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE30);
    cpu.pc = 0x2A824;
    16
}
/// $02DE30: bcc.w $2de3a
pub fn i_02de30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE3A; return 10; }
    8
}
/// $02DE34: lea.l $2deb4(pc), a1
pub fn i_02de34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE38;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DEB4;
    16
}
/// $02DE38: move.l a1, (a6)
pub fn i_02de38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE3A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DE3A: jsr $2a478(pc)
pub fn i_02de3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE3E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE3E);
    cpu.pc = 0x2A478;
    16
}
/// $02DE3E: jsr $28d70.l
pub fn i_02de3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DE44);
    cpu.pc = 0x28D70;
    16
}
/// $02DE44: jsr $2a8c0(pc)
pub fn i_02de44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE48);
    cpu.pc = 0x2A8C0;
    16
}
/// $02DE48: bcc.w $2de52
pub fn i_02de48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE52; return 10; }
    8
}
/// $02DE4C: lea.l $2e092(pc), a1
pub fn i_02de4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E092;
    16
}
/// $02DE50: move.l a1, (a6)
pub fn i_02de50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DE52: jsr $2acb8(pc)
pub fn i_02de52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE56);
    cpu.pc = 0x2ACB8;
    16
}
/// $02DE56: bcc.w $2de60
pub fn i_02de56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE5A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE60; return 10; }
    8
}
/// $02DE5A: lea.l $2e3b0(pc), a1
pub fn i_02de5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE5E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E3B0;
    16
}
/// $02DE5E: move.l a1, (a6)
pub fn i_02de5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE60;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DE60: jsr $2a664(pc)
pub fn i_02de60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE64;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2DE64);
    cpu.pc = 0x2A664;
    16
}
/// $02DE64: bcc.w $2de6e
pub fn i_02de64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE6E; return 10; }
    8
}
/// $02DE68: lea.l $2da38(pc), a1
pub fn i_02de68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DA38;
    16
}
/// $02DE6C: move.l a1, (a6)
pub fn i_02de6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02DE6E: cmpi.b #$0, $106f4a.l
pub fn i_02de6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE76;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106F4A) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02DE76: beq.w $2de98
pub fn i_02de76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE7A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2DE98; return 10; }
    8
}
/// $02DE7A: movea.l #$ffffffff, a0
pub fn i_02de7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE80;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02DE80: lea.l $27964e.l, a0
pub fn i_02de80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE86;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27964E;
    20
}
/// $02DE86: jsr $5dd56.l
pub fn i_02de86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DE8C);
    cpu.pc = 0x5DD56;
    16
}
/// $02DE8C: bcc.w $2de94
pub fn i_02de8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE90;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DE94; return 10; }
    8
}
/// $02DE90: jmp $2dcbc(pc)
pub fn i_02de90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE94;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02DE94: bra.w $2deb2
pub fn i_02de94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE98;
    cpu.ir = 0x6000;
    cpu.pc = 0x2DEB2;
    10
}
/// $02DE98: movea.l #$ffffffff, a0
pub fn i_02de98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DE9E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $02DE9E: lea.l $27965a.l, a0
pub fn i_02de9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEA4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27965A;
    20
}
/// $02DEA4: jsr $5dd56.l
pub fn i_02dea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2DEAA);
    cpu.pc = 0x5DD56;
    16
}
/// $02DEAA: bcc.w $2deb2
pub fn i_02deaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2DEB2; return 10; }
    8
}
/// $02DEAE: jmp $2dcbc(pc)
pub fn i_02deae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEB2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2DCBC;
    8
}
/// $02DEB2: rts
pub fn i_02deb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2DEB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_Hunker_02e3b0  ($02E3B0, 222 B, slug/vehicle_02dd20.s)
/// $02E3B0: bset.b #$0, $6b(a6)
pub fn i_02e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3B6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E3B6: bclr.b #$5, $8c(a6)
pub fn i_02e3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3BC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E3BC: bclr.b #$1, $8c(a6)
pub fn i_02e3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3C2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E3C2: jsr $2a9a0(pc)
pub fn i_02e3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E3C6);
    cpu.pc = 0x2A9A0;
    16
}
/// $02E3C6: movea.l #$279600, a0
pub fn i_02e3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3CC;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279600;
    cpu.a[0] = v1;
    4
}
/// $02E3CC: lsl.w #$2, d0
pub fn i_02e3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3CE;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E3CE: movea.l (a0, d0.w), a0
pub fn i_02e3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3D2;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E3D2: cmpa.l #$ffffffff, a0
pub fn i_02e3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3D8;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E3D8: beq.w $2e3e2
pub fn i_02e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E3E2; return 10; }
    8
}
/// $02E3DC: jsr $28cd4.l
pub fn i_02e3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E3E2);
    cpu.pc = 0x28CD4;
    16
}
/// $02E3E2: lea.l $2a024(pc), a1
pub fn i_02e3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A024;
    16
}
/// $02E3E6: movea.l (a1, d0.w), a0
pub fn i_02e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3EA;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E3EA: move.l a0, $48(a6)
pub fn i_02e3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3EE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E3EE: move.b #$28, $20(a6)
pub fn i_02e3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3F4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02E3F4: andi.w #$ff, d0
pub fn i_02e3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02E3F8: lsr.w #$2, d0
pub fn i_02e3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3FA;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E3FA: add.b d0, $20(a6)
pub fn i_02e3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E3FE;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02E3FE: lea.l $3d944.l, a1
pub fn i_02e3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E404;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x3D944;
    20
}
/// $02E404: jsr $4ae.l
pub fn i_02e404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E40A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E40A);
    cpu.pc = 0x4AE;
    16
}
/// $02E40A: jsr $5dd02.l
pub fn i_02e40a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E410;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E410);
    cpu.pc = 0x5DD02;
    16
}
/// $02E410: move.l #$ffffffff, $60(a6)
pub fn i_02e410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E418;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E418: lea.l $2999e(pc), a0
pub fn i_02e418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E41C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2999E;
    16
}
/// $02E41C: move.l a0, $48(a6)
pub fn i_02e41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E420;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E420: clr.w $2c(a6)
pub fn i_02e420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E424;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E424: lea.l $2e42a(pc), a1
pub fn i_02e424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E428;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E42A;
    16
}
/// $02E428: move.l a1, (a6)
pub fn i_02e428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E42A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E42A: jsr $2aa24(pc)
pub fn i_02e42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E42E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E42E);
    cpu.pc = 0x2AA24;
    16
}
/// $02E42E: clr.w $28(a6)
pub fn i_02e42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E432;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E432: jsr $28992.l
pub fn i_02e432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E438;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E438);
    cpu.pc = 0x28992;
    16
}
/// $02E438: jsr $2a7d8(pc)
pub fn i_02e438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E43C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E43C);
    cpu.pc = 0x2A7D8;
    16
}
/// $02E43C: clr.w $28(a6)
pub fn i_02e43c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E440;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02E440: jsr $2a478(pc)
pub fn i_02e440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E444;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E444);
    cpu.pc = 0x2A478;
    16
}
/// $02E444: jsr $28d70.l
pub fn i_02e444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E44A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E44A);
    cpu.pc = 0x28D70;
    16
}
/// $02E44A: bcc.w $2e464
pub fn i_02e44a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E44E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E464; return 10; }
    8
}
/// $02E44E: lea.l $29a14(pc), a0
pub fn i_02e44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E452;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29A14;
    16
}
/// $02E452: move.l a0, $48(a6)
pub fn i_02e452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E456;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E456: move.l #$29600, $60(a6)
pub fn i_02e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E45E;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29600;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02E45E: lea.l $2e9c8(pc), a1
pub fn i_02e45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E462;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E9C8;
    16
}
/// $02E462: move.l a1, (a6)
pub fn i_02e462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E464;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E464: jsr $2a8c0(pc)
pub fn i_02e464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E468;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E468);
    cpu.pc = 0x2A8C0;
    16
}
/// $02E468: bcc.w $2e472
pub fn i_02e468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E46C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E472; return 10; }
    8
}
/// $02E46C: lea.l $2d02e(pc), a1
pub fn i_02e46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E470;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02E470: move.l a1, (a6)
pub fn i_02e470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E472;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E472: jsr $2a5cc(pc)
pub fn i_02e472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E476;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E476);
    cpu.pc = 0x2A5CC;
    16
}
/// $02E476: bcc.w $2e480
pub fn i_02e476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E47A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2E480; return 10; }
    8
}
/// $02E47A: lea.l $2bbf2(pc), a1
pub fn i_02e47a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E47E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02E47E: move.l a1, (a6)
pub fn i_02e47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E480;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02E480: jsr $2a690(pc)
pub fn i_02e480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E484;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E484);
    cpu.pc = 0x2A690;
    16
}
/// $02E484: lea.l $2a060(pc), a0
pub fn i_02e484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E488;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02E488: movea.l #$ffffffff, a1
pub fn i_02e488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E48E;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02e48e  ($02E48E, 8 B, thunks/jsr_abs_thunks.c)
/// $02E48E: jsr $772.l
pub fn i_02e48e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E494;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E494);
    cpu.pc = 0x772;
    16
}
/// $02E494: rts
pub fn i_02e494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E496;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_DriveAlt_02e9c8  ($02E9C8, 324 B, slug/vehicle_02dd20.s)
/// $02E9C8: bset.b #$0, $6b(a6)
pub fn i_02e9c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9CE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E9CE: bclr.b #$5, $8c(a6)
pub fn i_02e9ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9D4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E9D4: bclr.b #$1, $8c(a6)
pub fn i_02e9d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9DA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02E9DA: jsr $2a9a0(pc)
pub fn i_02e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9DE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2E9DE);
    cpu.pc = 0x2A9A0;
    16
}
/// $02E9DE: movea.l #$279344, a0
pub fn i_02e9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9E4;
    cpu.ir = 0x207C;
    let v1: u32 = 0x279344;
    cpu.a[0] = v1;
    4
}
/// $02E9E4: lsl.w #$2, d0
pub fn i_02e9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9E6;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02E9E6: movea.l (a0, d0.w), a0
pub fn i_02e9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9EA;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02E9EA: cmpa.l #$ffffffff, a0
pub fn i_02e9ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9F0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02E9F0: beq.w $2e9fa
pub fn i_02e9f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9F4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2E9FA; return 10; }
    8
}
/// $02E9F4: jsr $28cd4.l
pub fn i_02e9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2E9FA);
    cpu.pc = 0x28CD4;
    16
}
/// $02E9FA: lea.l $2a024(pc), a1
pub fn i_02e9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2E9FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2A024;
    16
}
/// $02E9FE: movea.l (a1, d0.w), a0
pub fn i_02e9fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA02;
    cpu.ir = 0x2071;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02EA02: move.l a0, $48(a6)
pub fn i_02ea02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA06;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02EA06: move.b #$9, $8b(a6)
pub fn i_02ea06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA0C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x9;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EA0C: move.b #$28, $20(a6)
pub fn i_02ea0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA12;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02EA12: andi.w #$ff, d0
pub fn i_02ea12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA16;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02EA16: lsr.w #$2, d0
pub fn i_02ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA18;
    cpu.ir = 0xE448;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02EA18: add.b d0, $20(a6)
pub fn i_02ea18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA1C;
    cpu.ir = 0xD12E;
    let ad1 = cpu.a[6].wrapping_add(0x20);
    let v2 = bus.read8(ad1) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $02EA1C: clr.w $2c(a6)
pub fn i_02ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA20;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EA20: lea.l $2ea26(pc), a1
pub fn i_02ea20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2EA26;
    16
}
/// $02EA24: move.l a1, (a6)
pub fn i_02ea24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EA26: jsr $2aa24(pc)
pub fn i_02ea26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA2A);
    cpu.pc = 0x2AA24;
    16
}
/// $02EA2A: clr.w $28(a6)
pub fn i_02ea2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA2E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EA2E: jsr $2abc0(pc)
pub fn i_02ea2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA32;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA32);
    cpu.pc = 0x2ABC0;
    16
}
/// $02EA32: bcs.w $2ea52
pub fn i_02ea32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA36;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2EA52; return 10; }
    8
}
/// $02EA36: jsr $2a328(pc)
pub fn i_02ea36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA3A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA3A);
    cpu.pc = 0x2A328;
    16
}
/// $02EA3A: bcc.w $2ea44
pub fn i_02ea3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EA44; return 10; }
    8
}
/// $02EA3E: move.w #$2a0, $28(a6)
pub fn i_02ea3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA44;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2A0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02EA44: jsr $2a34e(pc)
pub fn i_02ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA48;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA48);
    cpu.pc = 0x2A34E;
    16
}
/// $02EA48: bcc.w $2ea52
pub fn i_02ea48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EA52; return 10; }
    8
}
/// $02EA4C: move.w #$fd60, $28(a6)
pub fn i_02ea4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA52;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD60;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02EA52: jsr $28992.l
pub fn i_02ea52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EA58);
    cpu.pc = 0x28992;
    16
}
/// $02EA58: jsr $2a7d8(pc)
pub fn i_02ea58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA5C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA5C);
    cpu.pc = 0x2A7D8;
    16
}
/// $02EA5C: bcc.w $2ea66
pub fn i_02ea5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA60;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EA66; return 10; }
    8
}
/// $02EA60: lea.l $2b4d2(pc), a1
pub fn i_02ea60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B4D2;
    16
}
/// $02EA64: move.l a1, (a6)
pub fn i_02ea64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EA66: clr.w $28(a6)
pub fn i_02ea66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA6A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02EA6A: jsr $2a478(pc)
pub fn i_02ea6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA6E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA6E);
    cpu.pc = 0x2A478;
    16
}
/// $02EA6E: jsr $28d70.l
pub fn i_02ea6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EA74);
    cpu.pc = 0x28D70;
    16
}
/// $02EA74: bcc.w $2eaaa
pub fn i_02ea74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA78;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAAA; return 10; }
    8
}
/// $02EA78: lea.l $29a14(pc), a0
pub fn i_02ea78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA7C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x29A14;
    16
}
/// $02EA7C: move.l a0, $48(a6)
pub fn i_02ea7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA80;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02EA80: move.l #$29600, $60(a6)
pub fn i_02ea80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA88;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x29600;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02EA88: lea.l $2b38c(pc), a1
pub fn i_02ea88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2B38C;
    16
}
/// $02EA8C: move.l a1, (a6)
pub fn i_02ea8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EA8E: jsr $2a328(pc)
pub fn i_02ea8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA92;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EA92);
    cpu.pc = 0x2A328;
    16
}
/// $02EA92: bcc.w $2ea9c
pub fn i_02ea92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA96;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EA9C; return 10; }
    8
}
/// $02EA96: lea.l $2bf64(pc), a1
pub fn i_02ea96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BF64;
    16
}
/// $02EA9A: move.l a1, (a6)
pub fn i_02ea9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EA9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EA9C: jsr $2a34e(pc)
pub fn i_02ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAA0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAA0);
    cpu.pc = 0x2A34E;
    16
}
/// $02EAA0: bcc.w $2eaaa
pub fn i_02eaa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAA4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAAA; return 10; }
    8
}
/// $02EAA4: lea.l $2c07a(pc), a1
pub fn i_02eaa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C07A;
    16
}
/// $02EAA8: move.l a1, (a6)
pub fn i_02eaa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAAA: jsr $2a59a(pc)
pub fn i_02eaaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAAE);
    cpu.pc = 0x2A59A;
    16
}
/// $02EAAE: bcc.w $2eab8
pub fn i_02eaae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAB8; return 10; }
    8
}
/// $02EAB2: lea.l $2e924(pc), a1
pub fn i_02eab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2E924;
    16
}
/// $02EAB6: move.l a1, (a6)
pub fn i_02eab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAB8: jsr $2a8c0(pc)
pub fn i_02eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EABC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EABC);
    cpu.pc = 0x2A8C0;
    16
}
/// $02EABC: bcc.w $2eac6
pub fn i_02eabc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAC6; return 10; }
    8
}
/// $02EAC0: lea.l $2d02e(pc), a1
pub fn i_02eac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2D02E;
    16
}
/// $02EAC4: move.l a1, (a6)
pub fn i_02eac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAC6: jsr $2aaf0(pc)
pub fn i_02eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EACA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EACA);
    cpu.pc = 0x2AAF0;
    16
}
/// $02EACA: bcc.w $2ead4
pub fn i_02eaca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EACE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAD4; return 10; }
    8
}
/// $02EACE: lea.l $2f614(pc), a1
pub fn i_02eace(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAD2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F614;
    16
}
/// $02EAD2: move.l a1, (a6)
pub fn i_02ead2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAD4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAD4: jsr $2aac0(pc)
pub fn i_02ead4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAD8);
    cpu.pc = 0x2AAC0;
    16
}
/// $02EAD8: bcc.w $2eae2
pub fn i_02ead8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EADC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAE2; return 10; }
    8
}
/// $02EADC: lea.l $2c24a(pc), a1
pub fn i_02eadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2C24A;
    16
}
/// $02EAE0: move.l a1, (a6)
pub fn i_02eae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAE2: jsr $2ac80(pc)
pub fn i_02eae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAE6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAE6);
    cpu.pc = 0x2AC80;
    16
}
/// $02EAE6: bcc.w $2eaf0
pub fn i_02eae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAEA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAF0; return 10; }
    8
}
/// $02EAEA: lea.l $2dd74(pc), a1
pub fn i_02eaea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2DD74;
    16
}
/// $02EAEE: move.l a1, (a6)
pub fn i_02eaee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAF0: jsr $2a5cc(pc)
pub fn i_02eaf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EAF4);
    cpu.pc = 0x2A5CC;
    16
}
/// $02EAF4: bcc.w $2eafe
pub fn i_02eaf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAF8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2EAFE; return 10; }
    8
}
/// $02EAF8: lea.l $2bbf2(pc), a1
pub fn i_02eaf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2BBF2;
    16
}
/// $02EAFC: move.l a1, (a6)
pub fn i_02eafc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EAFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02EAFE: jsr $2a690(pc)
pub fn i_02eafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB02;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2EB02);
    cpu.pc = 0x2A690;
    16
}
/// $02EB02: lea.l $2a060(pc), a0
pub fn i_02eb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB06;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A060;
    16
}
/// $02EB06: movea.l #$ffffffff, a1
pub fn i_02eb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB0C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
// ===== JsrAbsThunk_02eb0c  ($02EB0C, 8 B, thunks/jsr_abs_thunks.c)
/// $02EB0C: jsr $772.l
pub fn i_02eb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2EB12);
    cpu.pc = 0x772;
    16
}
/// $02EB12: rts
pub fn i_02eb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2EB14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TypeIfAir_02f84a  ($02F84A, 16 B, slug/vehicle_02dd20.s)
/// $02F84A: btst.b #$2, $8d(a6)
pub fn i_02f84a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F850;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02F850: beq.w $2f858
pub fn i_02f850(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F854;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2F858; return 10; }
    8
}
/// $02F854: move.w $1c(a6), d2
pub fn i_02f854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F858;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02F858: rts
pub fn i_02f858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F85A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AnyFlag8D3_02f85a  ($02F85A, 32 B, slug/vehicle_02dd20.s)
/// $02F85A: lea.l $100440.l, a0
pub fn i_02f85a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F860;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $02F860: btst.b #$3, $8d(a0)
pub fn i_02f860(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F866;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02F866: bne.w $2f880
pub fn i_02f866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F86A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2F880; return 10; }
    8
}
/// $02F86A: lea.l $1004e0.l, a0
pub fn i_02f86a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F870;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $02F870: btst.b #$3, $8d(a0)
pub fn i_02f870(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F876;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02F876: bne.w $2f880
pub fn i_02f876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F87A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2F880; return 10; }
    8
}
// ===== ClearXN_02f87a  ($02F87A, 6 B, thunks/ccr_helpers.c)
/// $02F87A: andi.b #$ee, ccr
pub fn i_02f87a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F87E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02F87E: rts
pub fn i_02f87e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F880;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AnyFlag8D3_SetC_02f880  ($02F880, 6 B, slug/vehicle_02dd20.s)
/// $02F880: ori.b #$11, ccr
pub fn i_02f880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F884;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02F884: rts
pub fn i_02f884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2F886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ResetDamageIdx_02fada  ($02FADA, 10 B, slug/vehicle_02dd20.s)
/// $02FADA: clr.b $88(a6)
pub fn i_02fada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FADE;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02FADE: clr.w $86(a6)
pub fn i_02fade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAE2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x86);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FAE2: rts
pub fn i_02fae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateDamageSprite_02fae4  ($02FAE4, 168 B, slug/vehicle_02dd20.s)
/// $02FAE4: cmpi.w #$1, $66(a6)
pub fn i_02fae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAEA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $02FAEA: bhi.w $2faf6
pub fn i_02faea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAEE;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x2FAF6; return 10; }
    8
}
/// $02FAEE: lea.l $2f886(pc), a1
pub fn i_02faee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAF2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2F886;
    16
}
/// $02FAF2: bra.w $2fb0c
pub fn i_02faf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FB0C;
    10
}
/// $02FAF6: andi.b #$ee, ccr
pub fn i_02faf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAFA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02FAFA: rts
pub fn i_02fafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FAFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02FB0C: moveq #$0, d0
pub fn i_02fb0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB0E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02FB0E: moveq #$0, d7
pub fn i_02fb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB10;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02FB10: move.w $86(a6), d0
pub fn i_02fb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB14;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x86);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02FB14: move.b (a1, d0.w), d2
pub fn i_02fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB18;
    cpu.ir = 0x1431;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $02FB18: cmpi.b #$4, d2
pub fn i_02fb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB1C;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $02FB1C: bcs.w $2fb2c
pub fn i_02fb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB20;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2FB2C; return 10; }
    8
}
/// $02FB20: nop
pub fn i_02fb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB22;
    cpu.ir = 0x4E71;
    4
}
/// $02FB22: nop
pub fn i_02fb22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB24;
    cpu.ir = 0x4E71;
    4
}
/// $02FB24: cmpi.b #$4, d2
pub fn i_02fb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB28;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $02FB28: nop
pub fn i_02fb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB2A;
    cpu.ir = 0x4E71;
    4
}
/// $02FB2A: trap #$f
pub fn i_02fb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB2C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02FB2C: andi.l #$ff, d2
pub fn i_02fb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB32;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $02FB32: lsl.l #$2, d2
pub fn i_02fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB34;
    cpu.ir = 0xE58A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 1, true);
    cpu.d[2] = r;
    10
}
/// $02FB34: lea.l $2faca(pc), a2
pub fn i_02fb34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB38;
    cpu.ir = 0x45FA;
    cpu.a[2] = 0x2FACA;
    16
}
/// $02FB38: movea.l (a2, d2.w), a3
pub fn i_02fb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB3C;
    cpu.ir = 0x2672;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $02FB3C: jmp (a3)
pub fn i_02fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB3E;
    cpu.ir = 0x4ED3;
    cpu.jump_to(cpu.a[3]);
    8
}
/// $02FB9A: clr.w $70(a6)
pub fn i_02fb9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FB9E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FB9E: move.w #$196, d1
pub fn i_02fb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBA2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x196;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FBA2: jsr $236e.l
pub fn i_02fba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FBA8);
    cpu.pc = 0x236E;
    16
}
/// $02FBA8: move.w #$197, d1
pub fn i_02fba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBAC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x197;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FBAC: jsr $236e.l
pub fn i_02fbac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBB2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FBB2);
    cpu.pc = 0x236E;
    16
}
/// $02FBB2: move.w #$198, d1
pub fn i_02fbb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBB6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FBB6: jsr $236e.l
pub fn i_02fbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBBC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FBBC);
    cpu.pc = 0x236E;
    16
}
/// $02FBBC: move.w #$199, d1
pub fn i_02fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBC0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x199;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FBC0: jsr $236e.l
pub fn i_02fbc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FBC6);
    cpu.pc = 0x236E;
    16
}
/// $02FBC6: lea.l $2fbcc(pc), a1
pub fn i_02fbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBCA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FBCC;
    16
}
/// $02FBCA: move.l a1, (a6)
pub fn i_02fbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBCC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FBCC: movea.l $c(a6), a0
pub fn i_02fbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBD0;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $02FBD0: btst.b #$0, $13(a0)
pub fn i_02fbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBD6;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02FBD6: beq.w $2fbe8
pub fn i_02fbd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2FBE8; return 10; }
    8
}
/// $02FBDA: lea.l $2fc6a(pc), a1
pub fn i_02fbda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FC6A;
    16
}
/// $02FBDE: move.l a1, (a6)
pub fn i_02fbde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $02FBE0: clr.w $70(a6)
pub fn i_02fbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBE4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FBE4: bra.w $2fbfa
pub fn i_02fbe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBE8;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FBFA;
    10
}
/// $02FBE8: cmpi.w #$10, $66(a0)
pub fn i_02fbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBEE;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x10, Size::Word, false);
    16
}
/// $02FBEE: bhi.w $2fbf6
pub fn i_02fbee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBF2;
    cpu.ir = 0x6200;
    if cpu.sr.evaluate(Condition::Hi) { cpu.pc = 0x2FBF6; return 10; }
    8
}
/// $02FBF2: bra.w $2fbfa
pub fn i_02fbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBF6;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FBFA;
    10
}
/// $02FBF6: jmp $2fc54(pc)
pub fn i_02fbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FBFA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2FC54;
    8
}
/// $02FBFA: btst.b #$3, $12(a0)
pub fn i_02fbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC00;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02FC00: beq.w $2fc54
pub fn i_02fc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2FC54; return 10; }
    8
}
/// $02FC04: addq.w #$2, $70(a6)
pub fn i_02fc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC08;
    cpu.ir = 0x546E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $02FC08: andi.w #$6, $70(a6)
pub fn i_02fc08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC0E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x6) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02FC0E: moveq #$0, d0
pub fn i_02fc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC10;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02FC10: moveq #$0, d1
pub fn i_02fc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC12;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02FC12: move.w $70(a6), d0
pub fn i_02fc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC16;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02FC16: lea.l $2fb92(pc), a1
pub fn i_02fc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC1A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x2FB92;
    16
}
/// $02FC1A: move.w (a1, d0.w), d1
pub fn i_02fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC1E;
    cpu.ir = 0x3231;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $02FC1E: move.w (a6, d1.w), $14(a6)
pub fn i_02fc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC24;
    cpu.ir = 0x3D76;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    22
}
/// $02FC24: cmpi.w #$ffff, $40(a0)
pub fn i_02fc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC2A;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x40);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $02FC2A: bne.w $2fc36
pub fn i_02fc2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC2E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2FC36; return 10; }
    8
}
/// $02FC2E: movea.l $3c(a0), a2
pub fn i_02fc2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC32;
    cpu.ir = 0x2468;
    let ad2 = cpu.a[0].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02FC32: bra.w $2fc3a
pub fn i_02fc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC36;
    cpu.ir = 0x6000;
    cpu.pc = 0x2FC3A;
    10
}
/// $02FC36: movea.l $40(a0), a2
pub fn i_02fc36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC3A;
    cpu.ir = 0x2468;
    let ad2 = cpu.a[0].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02FC3A: move.l a2, $3c(a6)
pub fn i_02fc3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC3E;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FC3E: move.l a2, $40(a6)
pub fn i_02fc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC42;
    cpu.ir = 0x2D4A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02FC42: jsr $5e4b2.l
pub fn i_02fc42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FC48);
    cpu.pc = 0x5E4B2;
    16
}
/// $02FC48: subi.w #$2, $38(a6)
pub fn i_02fc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC4E;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $02FC4E: jsr $28d70.l
pub fn i_02fc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2FC54);
    cpu.pc = 0x28D70;
    16
}
/// $02FC54: movea.l $c(a6), a0
pub fn i_02fc54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC58;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $02FC58: cmpi.l #$2dc5c, (a0)
pub fn i_02fc58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC5E;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x2DC5C, Size::Long, false);
    16
}
/// $02FC5E: bne.w $2fc68
pub fn i_02fc5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC62;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2FC68; return 10; }
    8
}
/// $02FC62: jmp $518.l
pub fn i_02fc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC68;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $02FC68: rts
pub fn i_02fc68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FC6A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_AngleIndex_02ff8e  ($02FF8E, 34 B, slug/vehicle_02dd20.s)
/// $02FF8E: lea.l $100580.l, a0
pub fn i_02ff8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF94;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02FF94: move.w $80(a0), d2
pub fn i_02ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF98;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02FF98: move.w d2, d0
pub fn i_02ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF9A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FF9A: asr.w #$1, d0
pub fn i_02ff9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF9C;
    cpu.ir = 0xE240;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02FF9C: neg.w d0
pub fn i_02ff9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FF9E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02FF9E: addi.w #$20, d0
pub fn i_02ff9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFA2;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x20, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02FFA2: cmpi.w #$40, d0
pub fn i_02ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFA6;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $02FFA6: bls.w $2ffae
pub fn i_02ffa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFAA;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2FFAE; return 10; }
    8
}
/// $02FFAA: move.w #$40, d0
pub fn i_02ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFAE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02FFAE: rts
pub fn i_02ffae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2FFB0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x02DD66, i_02dd66),
    (0x02DD6A, i_02dd6a),
    (0x02DD6E, i_02dd6e),
    (0x02DD70, i_02dd70),
    (0x02DE18, i_02de18),
    (0x02DE1C, i_02de1c),
    (0x02DE22, i_02de22),
    (0x02DE28, i_02de28),
    (0x02DE2C, i_02de2c),
    (0x02DE30, i_02de30),
    (0x02DE34, i_02de34),
    (0x02DE38, i_02de38),
    (0x02DE3A, i_02de3a),
    (0x02DE3E, i_02de3e),
    (0x02DE44, i_02de44),
    (0x02DE48, i_02de48),
    (0x02DE4C, i_02de4c),
    (0x02DE50, i_02de50),
    (0x02DE52, i_02de52),
    (0x02DE56, i_02de56),
    (0x02DE5A, i_02de5a),
    (0x02DE5E, i_02de5e),
    (0x02DE60, i_02de60),
    (0x02DE64, i_02de64),
    (0x02DE68, i_02de68),
    (0x02DE6C, i_02de6c),
    (0x02DE6E, i_02de6e),
    (0x02DE76, i_02de76),
    (0x02DE7A, i_02de7a),
    (0x02DE80, i_02de80),
    (0x02DE86, i_02de86),
    (0x02DE8C, i_02de8c),
    (0x02DE90, i_02de90),
    (0x02DE94, i_02de94),
    (0x02DE98, i_02de98),
    (0x02DE9E, i_02de9e),
    (0x02DEA4, i_02dea4),
    (0x02DEAA, i_02deaa),
    (0x02DEAE, i_02deae),
    (0x02DEB2, i_02deb2),
    (0x02E3B0, i_02e3b0),
    (0x02E3B6, i_02e3b6),
    (0x02E3BC, i_02e3bc),
    (0x02E3C2, i_02e3c2),
    (0x02E3C6, i_02e3c6),
    (0x02E3CC, i_02e3cc),
    (0x02E3CE, i_02e3ce),
    (0x02E3D2, i_02e3d2),
    (0x02E3D8, i_02e3d8),
    (0x02E3DC, i_02e3dc),
    (0x02E3E2, i_02e3e2),
    (0x02E3E6, i_02e3e6),
    (0x02E3EA, i_02e3ea),
    (0x02E3EE, i_02e3ee),
    (0x02E3F4, i_02e3f4),
    (0x02E3F8, i_02e3f8),
    (0x02E3FA, i_02e3fa),
    (0x02E3FE, i_02e3fe),
    (0x02E404, i_02e404),
    (0x02E40A, i_02e40a),
    (0x02E410, i_02e410),
    (0x02E418, i_02e418),
    (0x02E41C, i_02e41c),
    (0x02E420, i_02e420),
    (0x02E424, i_02e424),
    (0x02E428, i_02e428),
    (0x02E42A, i_02e42a),
    (0x02E42E, i_02e42e),
    (0x02E432, i_02e432),
    (0x02E438, i_02e438),
    (0x02E43C, i_02e43c),
    (0x02E440, i_02e440),
    (0x02E444, i_02e444),
    (0x02E44A, i_02e44a),
    (0x02E44E, i_02e44e),
    (0x02E452, i_02e452),
    (0x02E456, i_02e456),
    (0x02E45E, i_02e45e),
    (0x02E462, i_02e462),
    (0x02E464, i_02e464),
    (0x02E468, i_02e468),
    (0x02E46C, i_02e46c),
    (0x02E470, i_02e470),
    (0x02E472, i_02e472),
    (0x02E476, i_02e476),
    (0x02E47A, i_02e47a),
    (0x02E47E, i_02e47e),
    (0x02E480, i_02e480),
    (0x02E484, i_02e484),
    (0x02E488, i_02e488),
    (0x02E48E, i_02e48e),
    (0x02E494, i_02e494),
    (0x02E9C8, i_02e9c8),
    (0x02E9CE, i_02e9ce),
    (0x02E9D4, i_02e9d4),
    (0x02E9DA, i_02e9da),
    (0x02E9DE, i_02e9de),
    (0x02E9E4, i_02e9e4),
    (0x02E9E6, i_02e9e6),
    (0x02E9EA, i_02e9ea),
    (0x02E9F0, i_02e9f0),
    (0x02E9F4, i_02e9f4),
    (0x02E9FA, i_02e9fa),
    (0x02E9FE, i_02e9fe),
    (0x02EA02, i_02ea02),
    (0x02EA06, i_02ea06),
    (0x02EA0C, i_02ea0c),
    (0x02EA12, i_02ea12),
    (0x02EA16, i_02ea16),
    (0x02EA18, i_02ea18),
    (0x02EA1C, i_02ea1c),
    (0x02EA20, i_02ea20),
    (0x02EA24, i_02ea24),
    (0x02EA26, i_02ea26),
    (0x02EA2A, i_02ea2a),
    (0x02EA2E, i_02ea2e),
    (0x02EA32, i_02ea32),
    (0x02EA36, i_02ea36),
    (0x02EA3A, i_02ea3a),
    (0x02EA3E, i_02ea3e),
    (0x02EA44, i_02ea44),
    (0x02EA48, i_02ea48),
    (0x02EA4C, i_02ea4c),
    (0x02EA52, i_02ea52),
    (0x02EA58, i_02ea58),
    (0x02EA5C, i_02ea5c),
    (0x02EA60, i_02ea60),
    (0x02EA64, i_02ea64),
    (0x02EA66, i_02ea66),
    (0x02EA6A, i_02ea6a),
    (0x02EA6E, i_02ea6e),
    (0x02EA74, i_02ea74),
    (0x02EA78, i_02ea78),
    (0x02EA7C, i_02ea7c),
    (0x02EA80, i_02ea80),
    (0x02EA88, i_02ea88),
    (0x02EA8C, i_02ea8c),
    (0x02EA8E, i_02ea8e),
    (0x02EA92, i_02ea92),
    (0x02EA96, i_02ea96),
    (0x02EA9A, i_02ea9a),
    (0x02EA9C, i_02ea9c),
    (0x02EAA0, i_02eaa0),
    (0x02EAA4, i_02eaa4),
    (0x02EAA8, i_02eaa8),
    (0x02EAAA, i_02eaaa),
    (0x02EAAE, i_02eaae),
    (0x02EAB2, i_02eab2),
    (0x02EAB6, i_02eab6),
    (0x02EAB8, i_02eab8),
    (0x02EABC, i_02eabc),
    (0x02EAC0, i_02eac0),
    (0x02EAC4, i_02eac4),
    (0x02EAC6, i_02eac6),
    (0x02EACA, i_02eaca),
    (0x02EACE, i_02eace),
    (0x02EAD2, i_02ead2),
    (0x02EAD4, i_02ead4),
    (0x02EAD8, i_02ead8),
    (0x02EADC, i_02eadc),
    (0x02EAE0, i_02eae0),
    (0x02EAE2, i_02eae2),
    (0x02EAE6, i_02eae6),
    (0x02EAEA, i_02eaea),
    (0x02EAEE, i_02eaee),
    (0x02EAF0, i_02eaf0),
    (0x02EAF4, i_02eaf4),
    (0x02EAF8, i_02eaf8),
    (0x02EAFC, i_02eafc),
    (0x02EAFE, i_02eafe),
    (0x02EB02, i_02eb02),
    (0x02EB06, i_02eb06),
    (0x02EB0C, i_02eb0c),
    (0x02EB12, i_02eb12),
    (0x02F84A, i_02f84a),
    (0x02F850, i_02f850),
    (0x02F854, i_02f854),
    (0x02F858, i_02f858),
    (0x02F85A, i_02f85a),
    (0x02F860, i_02f860),
    (0x02F866, i_02f866),
    (0x02F86A, i_02f86a),
    (0x02F870, i_02f870),
    (0x02F876, i_02f876),
    (0x02F87A, i_02f87a),
    (0x02F87E, i_02f87e),
    (0x02F880, i_02f880),
    (0x02F884, i_02f884),
    (0x02FADA, i_02fada),
    (0x02FADE, i_02fade),
    (0x02FAE2, i_02fae2),
    (0x02FAE4, i_02fae4),
    (0x02FAEA, i_02faea),
    (0x02FAEE, i_02faee),
    (0x02FAF2, i_02faf2),
    (0x02FAF6, i_02faf6),
    (0x02FAFA, i_02fafa),
    (0x02FB0C, i_02fb0c),
    (0x02FB0E, i_02fb0e),
    (0x02FB10, i_02fb10),
    (0x02FB14, i_02fb14),
    (0x02FB18, i_02fb18),
    (0x02FB1C, i_02fb1c),
    (0x02FB20, i_02fb20),
    (0x02FB22, i_02fb22),
    (0x02FB24, i_02fb24),
    (0x02FB28, i_02fb28),
    (0x02FB2A, i_02fb2a),
    (0x02FB2C, i_02fb2c),
    (0x02FB32, i_02fb32),
    (0x02FB34, i_02fb34),
    (0x02FB38, i_02fb38),
    (0x02FB3C, i_02fb3c),
    (0x02FB9A, i_02fb9a),
    (0x02FB9E, i_02fb9e),
    (0x02FBA2, i_02fba2),
    (0x02FBA8, i_02fba8),
    (0x02FBAC, i_02fbac),
    (0x02FBB2, i_02fbb2),
    (0x02FBB6, i_02fbb6),
    (0x02FBBC, i_02fbbc),
    (0x02FBC0, i_02fbc0),
    (0x02FBC6, i_02fbc6),
    (0x02FBCA, i_02fbca),
    (0x02FBCC, i_02fbcc),
    (0x02FBD0, i_02fbd0),
    (0x02FBD6, i_02fbd6),
    (0x02FBDA, i_02fbda),
    (0x02FBDE, i_02fbde),
    (0x02FBE0, i_02fbe0),
    (0x02FBE4, i_02fbe4),
    (0x02FBE8, i_02fbe8),
    (0x02FBEE, i_02fbee),
    (0x02FBF2, i_02fbf2),
    (0x02FBF6, i_02fbf6),
    (0x02FBFA, i_02fbfa),
    (0x02FC00, i_02fc00),
    (0x02FC04, i_02fc04),
    (0x02FC08, i_02fc08),
    (0x02FC0E, i_02fc0e),
    (0x02FC10, i_02fc10),
    (0x02FC12, i_02fc12),
    (0x02FC16, i_02fc16),
    (0x02FC1A, i_02fc1a),
    (0x02FC1E, i_02fc1e),
    (0x02FC24, i_02fc24),
    (0x02FC2A, i_02fc2a),
    (0x02FC2E, i_02fc2e),
    (0x02FC32, i_02fc32),
    (0x02FC36, i_02fc36),
    (0x02FC3A, i_02fc3a),
    (0x02FC3E, i_02fc3e),
    (0x02FC42, i_02fc42),
    (0x02FC48, i_02fc48),
    (0x02FC4E, i_02fc4e),
    (0x02FC54, i_02fc54),
    (0x02FC58, i_02fc58),
    (0x02FC5E, i_02fc5e),
    (0x02FC62, i_02fc62),
    (0x02FC68, i_02fc68),
    (0x02FF8E, i_02ff8e),
    (0x02FF94, i_02ff94),
    (0x02FF98, i_02ff98),
    (0x02FF9A, i_02ff9a),
    (0x02FF9C, i_02ff9c),
    (0x02FF9E, i_02ff9e),
    (0x02FFA2, i_02ffa2),
    (0x02FFA6, i_02ffa6),
    (0x02FFAA, i_02ffaa),
    (0x02FFAE, i_02ffae),
];
