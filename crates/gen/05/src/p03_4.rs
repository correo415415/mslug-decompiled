// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p03_4) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $037A80: jsr $28cd4.l
pub fn i_037a80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A86);
    cpu.pc = 0x28CD4;
    16
}
/// $037A86: jsr $267e2.l
pub fn i_037a86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A8C);
    cpu.pc = 0x267E2;
    16
}
/// $037A8C: move.b #$28, $90(a6)
pub fn i_037a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A92;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037A92: move.w #$fee9, d0
pub fn i_037a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A96;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFEE9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $037A96: jsr $5dca4.l
pub fn i_037a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37A9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37A9C);
    cpu.pc = 0x5DCA4;
    16
}
/// $037A9C: move.w d0, $28(a6)
pub fn i_037a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AA0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037AA0: move.w #$457, $2a(a6)
pub fn i_037aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AA6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x457;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037AA6: move.w #$ff9b, $2e(a6)
pub fn i_037aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AAC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF9B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037AAC: move.w #$0, $2c(a6)
pub fn i_037aac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AB2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037AB2: lea.l $37ab8(pc), a1
pub fn i_037ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37AB8;
    16
}
/// $037AB6: move.l a1, (a6)
pub fn i_037ab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037AB8: bset.b #$1, $8d(a6)
pub fn i_037ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37ABE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037ABE: jsr $32ff2(pc)
pub fn i_037abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37AC2);
    cpu.pc = 0x32FF2;
    16
}
/// $037AC2: jsr $27bc8.l
pub fn i_037ac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37AC8);
    cpu.pc = 0x27BC8;
    16
}
/// $037AC8: bcc.w $37ad4
pub fn i_037ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37ACC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37AD4; return 10; }
    8
}
/// $037ACC: clr.w $28(a6)
pub fn i_037acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AD0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037AD0: clr.w $2a(a6)
pub fn i_037ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AD4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037AD4: jsr $32aa8(pc)
pub fn i_037ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37AD8);
    cpu.pc = 0x32AA8;
    16
}
/// $037AD8: bcs.w $37ae6
pub fn i_037ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37ADC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x37AE6; return 10; }
    8
}
/// $037ADC: subi.b #$1, $90(a6)
pub fn i_037adc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AE2;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $037AE2: bne.w $37af0
pub fn i_037ae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AE6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37AF0; return 10; }
    8
}
/// $037AE6: lea.l $37c1a(pc), a1
pub fn i_037ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C1A;
    16
}
/// $037AEA: move.l a1, (a6)
pub fn i_037aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037AEC: bra.w $37af0
pub fn i_037aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AF0;
    cpu.ir = 0x6000;
    cpu.pc = 0x37AF0;
    10
}
/// $037AF0: rts
pub fn i_037af0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AF2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_Blood_037af2  ($037AF2, 14 B, player/air_death_crouch_036632.s)
/// $037AF2: jsr $33358(pc)
pub fn i_037af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37AF6);
    cpu.pc = 0x33358;
    16
}
/// $037AF6: move.w #$10fb, d0
pub fn i_037af6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37AFA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $037AFA: jsr $2352.l
pub fn i_037afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37B00);
    cpu.pc = 0x2352;
    16
}
// ===== Player_Death_Timed_037b00  ($037B00, 142 B, player/air_death_crouch_036632.s)
/// $037B00: move.w #$0, $7c(a6)
pub fn i_037b00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B06;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037B06: move.w #$0, $7e(a6)
pub fn i_037b06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B0C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037B0C: bclr.b #$2, $8c(a6)
pub fn i_037b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B12;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037B12: bclr.b #$1, $8c(a6)
pub fn i_037b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B18;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037B18: bclr.b #$3, $8c(a6)
pub fn i_037b18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B1E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037B1E: bset.b #$0, $13(a6)
pub fn i_037b1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B24;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037B24: move.b #$0, $70(a6)
pub fn i_037b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B2A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037B2A: lea.l $279f6c.l, a0
pub fn i_037b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B30;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F6C;
    20
}
/// $037B30: move.l -$4(a0), $74(a6)
pub fn i_037b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B36;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037B36: move.b #$ff, $21(a6)
pub fn i_037b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B3C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037B3C: lea.l $279f6c.l, a0
pub fn i_037b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B42;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F6C;
    20
}
/// $037B42: jsr $28cd4.l
pub fn i_037b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37B48);
    cpu.pc = 0x28CD4;
    16
}
/// $037B48: jsr $267e2.l
pub fn i_037b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37B4E);
    cpu.pc = 0x267E2;
    16
}
/// $037B4E: move.b #$28, $90(a6)
pub fn i_037b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B54;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037B54: lea.l $37b5a(pc), a1
pub fn i_037b54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B5A;
    16
}
/// $037B58: move.l a1, (a6)
pub fn i_037b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037B5A: jsr $32ff2(pc)
pub fn i_037b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37B5E);
    cpu.pc = 0x32FF2;
    16
}
/// $037B5E: jsr $27a92.l
pub fn i_037b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37B64);
    cpu.pc = 0x27A92;
    16
}
/// $037B64: bcc.w $37b70
pub fn i_037b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37B70; return 10; }
    8
}
/// $037B68: clr.w $28(a6)
pub fn i_037b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B6C;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037B6C: clr.w $2a(a6)
pub fn i_037b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B70;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037B70: jsr $32aa8(pc)
pub fn i_037b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B74;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37B74);
    cpu.pc = 0x32AA8;
    16
}
/// $037B74: bcs.w $37b82
pub fn i_037b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B78;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x37B82; return 10; }
    8
}
/// $037B78: subi.b #$1, $90(a6)
pub fn i_037b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B7E;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $037B7E: bne.w $37b8c
pub fn i_037b7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B82;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37B8C; return 10; }
    8
}
/// $037B82: lea.l $37c1a(pc), a1
pub fn i_037b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B86;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C1A;
    16
}
/// $037B86: move.l a1, (a6)
pub fn i_037b86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B88;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037B88: bra.w $37b8c
pub fn i_037b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x37B8C;
    10
}
/// $037B8C: rts
pub fn i_037b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_DeathPit_037b8e  ($037B8E, 140 B, player/air_death_crouch_036632.s)
/// $037B8E: btst.b #$0, $13(a6)
pub fn i_037b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B94;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037B94: bne.w $37bba
pub fn i_037b94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B98;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37BBA; return 10; }
    8
}
/// $037B98: cmpa.l #$100440, a6
pub fn i_037b98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37B9E;
    cpu.ir = 0xBDFC;
    let v1: u32 = 0x100440;
    let h1 = cpu.a[6];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $037B9E: bne.w $37bb0
pub fn i_037b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BA2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37BB0; return 10; }
    8
}
/// $037BA2: move.w #$1123, d0
pub fn i_037ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BA6;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1123;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $037BA6: jsr $2352.l
pub fn i_037ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BAC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37BAC);
    cpu.pc = 0x2352;
    16
}
/// $037BAC: bra.w $37bba
pub fn i_037bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BB0;
    cpu.ir = 0x6000;
    cpu.pc = 0x37BBA;
    10
}
/// $037BB0: move.w #$1087, d0
pub fn i_037bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BB4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1087;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $037BB4: jsr $2352.l
pub fn i_037bb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BBA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37BBA);
    cpu.pc = 0x2352;
    16
}
/// $037BBA: bset.b #$0, $13(a6)
pub fn i_037bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BC0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037BC0: move.w #$0, $7c(a6)
pub fn i_037bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037BC6: move.w #$0, $7e(a6)
pub fn i_037bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BCC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037BCC: move.b #$6, $90(a6)
pub fn i_037bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BD2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x6;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037BD2: lea.l $ffff.w, a0
pub fn i_037bd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BD6;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $037BD6: move.l a0, $48(a6)
pub fn i_037bd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BDA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037BDA: lea.l $37be0(pc), a1
pub fn i_037bda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BDE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37BE0;
    16
}
/// $037BDE: move.l a1, (a6)
pub fn i_037bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037BE0: jsr $27cee.l
pub fn i_037be0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BE6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37BE6);
    cpu.pc = 0x27CEE;
    16
}
/// $037BE6: subi.b #$1, $90(a6)
pub fn i_037be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BEC;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $037BEC: bne.w $37c18
pub fn i_037bec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BF0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37C18; return 10; }
    8
}
/// $037BF0: jsr $267e2.l
pub fn i_037bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37BF6);
    cpu.pc = 0x267E2;
    16
}
/// $037BF6: jsr $5b6.l
pub fn i_037bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37BFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37BFC);
    cpu.pc = 0x5B6;
    16
}
/// $037BFC: bclr.b #$7, $5b(a6)
pub fn i_037bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C02;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C02: jsr $13600.l
pub fn i_037c02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C08);
    cpu.pc = 0x13600;
    16
}
/// $037C08: lea.l $400.l, a1
pub fn i_037c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C0E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $037C0E: move.l a1, (a6)
pub fn i_037c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037C10: movea.l a6, a0
pub fn i_037c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C12;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $037C12: jsr $5fe.l
pub fn i_037c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C18);
    cpu.pc = 0x5FE;
    16
}
/// $037C18: rts
pub fn i_037c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Death_Despawn_037c1a  ($037C1A, 90 B, player/air_death_crouch_036632.s)
/// $037C1A: jsr $267e2.l
pub fn i_037c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C20;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C20);
    cpu.pc = 0x267E2;
    16
}
/// $037C20: move.w #$0, $7c(a6)
pub fn i_037c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C26;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037C26: move.w #$0, $7e(a6)
pub fn i_037c26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C2C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037C2C: move.b #$28, $90(a6)
pub fn i_037c2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C32;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037C32: lea.l $ffff.w, a0
pub fn i_037c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C36;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $037C36: move.l a0, $48(a6)
pub fn i_037c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C3A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037C3A: lea.l $37c40(pc), a1
pub fn i_037c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C3E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37C40;
    16
}
/// $037C3E: move.l a1, (a6)
pub fn i_037c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C40;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037C40: jsr $2783a.l
pub fn i_037c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C46;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C46);
    cpu.pc = 0x2783A;
    16
}
/// $037C46: subi.b #$1, $90(a6)
pub fn i_037c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C4C;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C4C: bne.w $37c72
pub fn i_037c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37C72; return 10; }
    8
}
/// $037C50: jsr $5b6.l
pub fn i_037c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C56;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C56);
    cpu.pc = 0x5B6;
    16
}
/// $037C56: bclr.b #$7, $5b(a6)
pub fn i_037c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C5C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C5C: jsr $13600.l
pub fn i_037c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C62;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C62);
    cpu.pc = 0x13600;
    16
}
/// $037C62: lea.l $400.l, a1
pub fn i_037c62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C68;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x400;
    20
}
/// $037C68: move.l a1, (a6)
pub fn i_037c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037C6A: movea.l a6, a0
pub fn i_037c6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C6C;
    cpu.ir = 0x204E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $037C6C: jsr $5fe.l
pub fn i_037c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C72;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37C72);
    cpu.pc = 0x5FE;
    16
}
/// $037C72: rts
pub fn i_037c72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C74;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchEnter_037c74  ($037C74, 326 B, player/air_death_crouch_036632.s)
/// $037C74: move.w #$0, $7c(a6)
pub fn i_037c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C7A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037C7A: move.w #$0, $7e(a6)
pub fn i_037c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C80;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037C80: bclr.b #$2, $8c(a6)
pub fn i_037c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C86;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C86: bclr.b #$1, $8c(a6)
pub fn i_037c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C8C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C8C: bclr.b #$3, $8c(a6)
pub fn i_037c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C92;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037C92: move.b #$30, $70(a6)
pub fn i_037c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C98;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037C98: lea.l $279864.l, a0
pub fn i_037c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37C9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279864;
    20
}
/// $037C9E: move.l -$4(a0), $74(a6)
pub fn i_037c9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CA4;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037CA4: move.b #$ff, $21(a6)
pub fn i_037ca4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CAA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037CAA: lea.l $279864.l, a0
pub fn i_037caa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279864;
    20
}
/// $037CB0: jsr $28cd4.l
pub fn i_037cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37CB6);
    cpu.pc = 0x28CD4;
    16
}
/// $037CB6: move.l #$32598, $60(a6)
pub fn i_037cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CBE;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037CBE: clr.w $2c(a6)
pub fn i_037cbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CC2;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037CC2: lea.l $32734(pc), a0
pub fn i_037cc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CC6;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $037CC6: move.l a0, $48(a6)
pub fn i_037cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CCA;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037CCA: lea.l $37cd0(pc), a1
pub fn i_037cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37CD0;
    16
}
/// $037CCE: move.l a1, (a6)
pub fn i_037cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037CD0: jsr $32ff2(pc)
pub fn i_037cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CD4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37CD4);
    cpu.pc = 0x32FF2;
    16
}
/// $037CD4: jsr $32d32(pc)
pub fn i_037cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CD8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37CD8);
    cpu.pc = 0x32D32;
    16
}
/// $037CD8: clr.w $28(a6)
pub fn i_037cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CDC;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037CDC: jsr $32e3c(pc)
pub fn i_037cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CE0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37CE0);
    cpu.pc = 0x32E3C;
    16
}
/// $037CE0: bcc.w $37cea
pub fn i_037ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CE4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37CEA; return 10; }
    8
}
/// $037CE4: move.w #$300, $28(a6)
pub fn i_037ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CEA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037CEA: jsr $32e42(pc)
pub fn i_037cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CEE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37CEE);
    cpu.pc = 0x32E42;
    16
}
/// $037CEE: bcc.w $37cf8
pub fn i_037cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37CF8; return 10; }
    8
}
/// $037CF2: move.w #$fd00, $28(a6)
pub fn i_037cf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CF8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037CF8: jsr $27a92.l
pub fn i_037cf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37CFE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37CFE);
    cpu.pc = 0x27A92;
    16
}
/// $037CFE: clr.w $28(a6)
pub fn i_037cfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D02;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037D02: jsr $32aa8(pc)
pub fn i_037d02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D06;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37D06);
    cpu.pc = 0x32AA8;
    16
}
/// $037D06: bcc.w $37d2c
pub fn i_037d06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D0A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37D2C; return 10; }
    8
}
/// $037D0A: lea.l $3827a(pc), a1
pub fn i_037d0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D0E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3827A;
    16
}
/// $037D0E: move.l a1, (a6)
pub fn i_037d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D10;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D10: jsr $32e3c(pc)
pub fn i_037d10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D14;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37D14);
    cpu.pc = 0x32E3C;
    16
}
/// $037D14: bcc.w $37d1e
pub fn i_037d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D18;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37D1E; return 10; }
    8
}
/// $037D18: lea.l $3842c(pc), a1
pub fn i_037d18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D1C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3842C;
    16
}
/// $037D1C: move.l a1, (a6)
pub fn i_037d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D1E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D1E: jsr $32e42(pc)
pub fn i_037d1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D22;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37D22);
    cpu.pc = 0x32E42;
    16
}
/// $037D22: bcc.w $37d2c
pub fn i_037d22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D26;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37D2C; return 10; }
    8
}
/// $037D26: lea.l $3855a(pc), a1
pub fn i_037d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D2A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3855A;
    16
}
/// $037D2A: move.l a1, (a6)
pub fn i_037d2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D2C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D2C: jsr $330d0(pc)
pub fn i_037d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D30;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37D30);
    cpu.pc = 0x330D0;
    16
}
/// $037D30: bcc.w $37d62
pub fn i_037d30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D34;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37D62; return 10; }
    8
}
/// $037D34: cmpi.b #$ff, d1
pub fn i_037d34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D38;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $037D38: bne.w $37d46
pub fn i_037d38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D3C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D46; return 10; }
    8
}
/// $037D3C: lea.l $38a28(pc), a1
pub fn i_037d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $037D40: move.l a1, (a6)
pub fn i_037d40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D42: bra.w $37d62
pub fn i_037d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D46;
    cpu.ir = 0x6000;
    cpu.pc = 0x37D62;
    10
}
/// $037D46: cmpi.b #$3, d1
pub fn i_037d46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D4A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $037D4A: bne.w $37d58
pub fn i_037d4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D4E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D58; return 10; }
    8
}
/// $037D4E: lea.l $388f0(pc), a1
pub fn i_037d4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $037D52: move.l a1, (a6)
pub fn i_037d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D54: bra.w $37d62
pub fn i_037d54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D58;
    cpu.ir = 0x6000;
    cpu.pc = 0x37D62;
    10
}
/// $037D58: lea.l $3873c(pc), a1
pub fn i_037d58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $037D5C: move.l a1, (a6)
pub fn i_037d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D5E: bra.w $37d62
pub fn i_037d5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D62;
    cpu.ir = 0x6000;
    cpu.pc = 0x37D62;
    10
}
/// $037D62: cmpi.b #$0, $85(a6)
pub fn i_037d62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D68;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $037D68: bne.w $37d7c
pub fn i_037d68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D7C; return 10; }
    8
}
/// $037D6C: cmpi.b #$1, $71(a6)
pub fn i_037d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D72;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $037D72: bne.w $37d7c
pub fn i_037d72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D76;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D7C; return 10; }
    8
}
/// $037D76: lea.l $33b9a(pc), a1
pub fn i_037d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D7A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $037D7A: move.l a1, (a6)
pub fn i_037d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D7C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D7C: cmpi.w #$0, $82(a6)
pub fn i_037d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D82;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $037D82: bne.w $37d96
pub fn i_037d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D86;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37D96; return 10; }
    8
}
/// $037D86: cmpi.b #$0, $71(a6)
pub fn i_037d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D8C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $037D8C: beq.w $37d96
pub fn i_037d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D90;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37D96; return 10; }
    8
}
/// $037D90: lea.l $38086(pc), a1
pub fn i_037d90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D94;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38086;
    16
}
/// $037D94: move.l a1, (a6)
pub fn i_037d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D96;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037D96: jsr $27eba.l
pub fn i_037d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37D9C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37D9C);
    cpu.pc = 0x27EBA;
    16
}
/// $037D9C: bcc.w $37da6
pub fn i_037d9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37DA6; return 10; }
    8
}
/// $037DA0: lea.l $36d64(pc), a1
pub fn i_037da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $037DA4: move.l a1, (a6)
pub fn i_037da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037DA6: jsr $33034(pc)
pub fn i_037da6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DAA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37DAA);
    cpu.pc = 0x33034;
    16
}
/// $037DAA: bcc.w $37db4
pub fn i_037daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DAE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37DB4; return 10; }
    8
}
/// $037DAE: lea.l $36914(pc), a1
pub fn i_037dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $037DB2: move.l a1, (a6)
pub fn i_037db2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037DB4: jsr $33522(pc)
pub fn i_037db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37DB8);
    cpu.pc = 0x33522;
    16
}
/// $037DB8: rts
pub fn i_037db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DBA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchEnterB_037dba  ($037DBA, 264 B, player/air_death_crouch_036632.s)
/// $037DBA: move.w #$0, $7c(a6)
pub fn i_037dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DC0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037DC0: move.w #$0, $7e(a6)
pub fn i_037dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DC6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037DC6: bclr.b #$2, $8c(a6)
pub fn i_037dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DCC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037DCC: bclr.b #$1, $8c(a6)
pub fn i_037dcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DD2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037DD2: bclr.b #$3, $8c(a6)
pub fn i_037dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DD8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037DD8: bset.b #$7, $5b(a6)
pub fn i_037dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DDE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037DDE: move.b #$30, $70(a6)
pub fn i_037dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DE4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037DE4: lea.l $279864.l, a0
pub fn i_037de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DEA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279864;
    20
}
/// $037DEA: move.l -$4(a0), $74(a6)
pub fn i_037dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DF0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037DF0: move.b #$ff, $21(a6)
pub fn i_037df0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DF6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037DF6: lea.l $279864.l, a0
pub fn i_037df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37DFC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279864;
    20
}
/// $037DFC: jsr $28cd4.l
pub fn i_037dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37E02);
    cpu.pc = 0x28CD4;
    16
}
/// $037E02: lea.l $32734(pc), a0
pub fn i_037e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E06;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $037E06: move.l a0, $48(a6)
pub fn i_037e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E0A;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037E0A: clr.w $2c(a6)
pub fn i_037e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E0E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037E0E: lea.l $37e14(pc), a1
pub fn i_037e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37E14;
    16
}
/// $037E12: move.l a1, (a6)
pub fn i_037e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037E14: jsr $32ff2(pc)
pub fn i_037e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37E18);
    cpu.pc = 0x32FF2;
    16
}
/// $037E18: jsr $32d32(pc)
pub fn i_037e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37E1C);
    cpu.pc = 0x32D32;
    16
}
/// $037E1C: clr.w $28(a6)
pub fn i_037e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E20;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037E20: move.w #$300, $28(a6)
pub fn i_037e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E26;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037E26: jsr $32e42(pc)
pub fn i_037e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E2A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37E2A);
    cpu.pc = 0x32E42;
    16
}
/// $037E2A: bcc.w $37e34
pub fn i_037e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E2E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37E34; return 10; }
    8
}
/// $037E2E: move.w #$fd00, $28(a6)
pub fn i_037e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E34;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037E34: jsr $27a92.l
pub fn i_037e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37E3A);
    cpu.pc = 0x27A92;
    16
}
/// $037E3A: clr.w $28(a6)
pub fn i_037e3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E3E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037E3E: jsr $32aa8(pc)
pub fn i_037e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37E42);
    cpu.pc = 0x32AA8;
    16
}
/// $037E42: bcc.w $37e68
pub fn i_037e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E46;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37E68; return 10; }
    8
}
/// $037E46: lea.l $3827a(pc), a1
pub fn i_037e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E4A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3827A;
    16
}
/// $037E4A: move.l a1, (a6)
pub fn i_037e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E4C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037E4C: jsr $32e3c(pc)
pub fn i_037e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E50;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37E50);
    cpu.pc = 0x32E3C;
    16
}
/// $037E50: bcc.w $37e5a
pub fn i_037e50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E54;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37E5A; return 10; }
    8
}
/// $037E54: lea.l $3842c(pc), a1
pub fn i_037e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E58;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3842C;
    16
}
/// $037E58: move.l a1, (a6)
pub fn i_037e58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E5A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037E5A: jsr $32e42(pc)
pub fn i_037e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37E5E);
    cpu.pc = 0x32E42;
    16
}
/// $037E5E: bcc.w $37e68
pub fn i_037e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E62;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37E68; return 10; }
    8
}
/// $037E62: lea.l $3855a(pc), a1
pub fn i_037e62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E66;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3855A;
    16
}
/// $037E66: move.l a1, (a6)
pub fn i_037e66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E68;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037E68: jsr $330d0(pc)
pub fn i_037e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E6C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37E6C);
    cpu.pc = 0x330D0;
    16
}
/// $037E6C: bcc.w $37e9e
pub fn i_037e6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37E9E; return 10; }
    8
}
/// $037E70: cmpi.b #$ff, d1
pub fn i_037e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E74;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $037E74: bne.w $37e82
pub fn i_037e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E78;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37E82; return 10; }
    8
}
/// $037E78: lea.l $38a28(pc), a1
pub fn i_037e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $037E7C: move.l a1, (a6)
pub fn i_037e7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037E7E: bra.w $37e9e
pub fn i_037e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E82;
    cpu.ir = 0x6000;
    cpu.pc = 0x37E9E;
    10
}
/// $037E82: cmpi.b #$3, d1
pub fn i_037e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E86;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $037E86: bne.w $37e94
pub fn i_037e86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E8A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37E94; return 10; }
    8
}
/// $037E8A: lea.l $388f0(pc), a1
pub fn i_037e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E8E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $037E8E: move.l a1, (a6)
pub fn i_037e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E90;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037E90: bra.w $37e9e
pub fn i_037e90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E94;
    cpu.ir = 0x6000;
    cpu.pc = 0x37E9E;
    10
}
/// $037E94: lea.l $3873c(pc), a1
pub fn i_037e94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $037E98: move.l a1, (a6)
pub fn i_037e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037E9A: bra.w $37e9e
pub fn i_037e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37E9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x37E9E;
    10
}
/// $037E9E: jsr $27eba.l
pub fn i_037e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37EA4);
    cpu.pc = 0x27EBA;
    16
}
/// $037EA4: bcc.w $37eae
pub fn i_037ea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EA8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37EAE; return 10; }
    8
}
/// $037EA8: lea.l $36d64(pc), a1
pub fn i_037ea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EAC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $037EAC: move.l a1, (a6)
pub fn i_037eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EAE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037EAE: jsr $33080(pc)
pub fn i_037eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EB2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37EB2);
    cpu.pc = 0x33080;
    16
}
/// $037EB2: bcc.w $37ebc
pub fn i_037eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EB6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37EBC; return 10; }
    8
}
/// $037EB6: lea.l $36914(pc), a1
pub fn i_037eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $037EBA: move.l a1, (a6)
pub fn i_037eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037EBC: jsr $33522(pc)
pub fn i_037ebc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EC0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37EC0);
    cpu.pc = 0x33522;
    16
}
/// $037EC0: rts
pub fn i_037ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchExit_037ec2  ($037EC2, 386 B, player/air_death_crouch_036632.s)
/// $037EC2: move.w #$0, $7c(a6)
pub fn i_037ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EC8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037EC8: move.w #$0, $7e(a6)
pub fn i_037ec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37ECE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037ECE: bclr.b #$2, $8c(a6)
pub fn i_037ece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37ED4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037ED4: bclr.b #$1, $8c(a6)
pub fn i_037ed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EDA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037EDA: bclr.b #$3, $8c(a6)
pub fn i_037eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EE0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $037EE0: move.b #$30, $70(a6)
pub fn i_037ee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EE6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037EE6: lea.l $27986e.l, a0
pub fn i_037ee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EEC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27986E;
    20
}
/// $037EEC: move.l -$4(a0), $74(a6)
pub fn i_037eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EF2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $037EF2: move.b #$ff, $21(a6)
pub fn i_037ef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EF8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $037EF8: lea.l $27986e.l, a0
pub fn i_037ef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37EFE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27986E;
    20
}
/// $037EFE: jsr $28cd4.l
pub fn i_037efe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37F04);
    cpu.pc = 0x28CD4;
    16
}
/// $037F04: clr.w $2c(a6)
pub fn i_037f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F08;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037F08: lea.l $32734(pc), a0
pub fn i_037f08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F0C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $037F0C: move.l a0, $48(a6)
pub fn i_037f0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F10;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $037F10: lea.l $37f16(pc), a1
pub fn i_037f10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37F16;
    16
}
/// $037F14: move.l a1, (a6)
pub fn i_037f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F16: jsr $32ff2(pc)
pub fn i_037f16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F1A);
    cpu.pc = 0x32FF2;
    16
}
/// $037F1A: jsr $32d32(pc)
pub fn i_037f1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F1E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F1E);
    cpu.pc = 0x32D32;
    16
}
/// $037F1E: clr.w $28(a6)
pub fn i_037f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F22;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037F22: jsr $32e3c(pc)
pub fn i_037f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F26);
    cpu.pc = 0x32E3C;
    16
}
/// $037F26: bcc.w $37f30
pub fn i_037f26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F2A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37F30; return 10; }
    8
}
/// $037F2A: move.w #$300, $28(a6)
pub fn i_037f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F30;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x300;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037F30: jsr $32e42(pc)
pub fn i_037f30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F34;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F34);
    cpu.pc = 0x32E42;
    16
}
/// $037F34: bcc.w $37f3e
pub fn i_037f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F38;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37F3E; return 10; }
    8
}
/// $037F38: move.w #$fd00, $28(a6)
pub fn i_037f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFD00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $037F3E: jsr $27a92.l
pub fn i_037f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37F44);
    cpu.pc = 0x27A92;
    16
}
/// $037F44: clr.w $28(a6)
pub fn i_037f44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F48;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $037F48: jsr $32aa8(pc)
pub fn i_037f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F4C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37F4C);
    cpu.pc = 0x32AA8;
    16
}
/// $037F4C: bcc.w $37fc6
pub fn i_037f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F50;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37FC6; return 10; }
    8
}
/// $037F50: lea.l $34704(pc), a1
pub fn i_037f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x34704;
    16
}
/// $037F54: move.l a1, (a6)
pub fn i_037f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F56: btst.b #$0, $88(a6)
pub fn i_037f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F5C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $037F5C: beq.w $37f66
pub fn i_037f5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x37F66; return 10; }
    8
}
/// $037F60: lea.l $33a5e(pc), a1
pub fn i_037f60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33A5E;
    16
}
/// $037F64: move.l a1, (a6)
pub fn i_037f64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F66: cmpi.b #$0, $85(a6)
pub fn i_037f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F6C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $037F6C: bne.w $37f80
pub fn i_037f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F70;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37F80; return 10; }
    8
}
/// $037F70: cmpi.b #$1, $71(a6)
pub fn i_037f70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F76;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $037F76: bne.w $37f80
pub fn i_037f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F7A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37F80; return 10; }
    8
}
/// $037F7A: lea.l $33b9a(pc), a1
pub fn i_037f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F7E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33B9A;
    16
}
/// $037F7E: move.l a1, (a6)
pub fn i_037f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F80;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F80: jsr $5e9b6.l
pub fn i_037f80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F86;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x37F86);
    cpu.pc = 0x5E9B6;
    16
}
/// $037F86: andi.w #$7, d0
pub fn i_037f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F8A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $037F8A: moveq #$4, d0
pub fn i_037f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F8C;
    cpu.ir = 0x7004;
    cpu.d[0] = 0x4;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $037F8C: cmp.w $82(a6), d0
pub fn i_037f8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F90;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $037F90: bne.w $37f9a
pub fn i_037f90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37F9A; return 10; }
    8
}
/// $037F94: lea.l $33afc(pc), a1
pub fn i_037f94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33AFC;
    16
}
/// $037F98: move.l a1, (a6)
pub fn i_037f98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37F9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037F9A: cmpi.w #$0, $82(a6)
pub fn i_037f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FA0;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $037FA0: bne.w $37faa
pub fn i_037fa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FA4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37FAA; return 10; }
    8
}
/// $037FA4: lea.l $33d64(pc), a1
pub fn i_037fa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x33D64;
    16
}
/// $037FA8: move.l a1, (a6)
pub fn i_037fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FAA: jsr $32e3c(pc)
pub fn i_037faa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37FAE);
    cpu.pc = 0x32E3C;
    16
}
/// $037FAE: bcc.w $37fb8
pub fn i_037fae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FB2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37FB8; return 10; }
    8
}
/// $037FB2: lea.l $351d8(pc), a1
pub fn i_037fb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FB6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x351D8;
    16
}
/// $037FB6: move.l a1, (a6)
pub fn i_037fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FB8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FB8: jsr $32e42(pc)
pub fn i_037fb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FBC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37FBC);
    cpu.pc = 0x32E42;
    16
}
/// $037FBC: bcc.w $37fc6
pub fn i_037fbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FC0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x37FC6; return 10; }
    8
}
/// $037FC0: lea.l $354f2(pc), a1
pub fn i_037fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x354F2;
    16
}
/// $037FC4: move.l a1, (a6)
pub fn i_037fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FC6: jsr $330d0(pc)
pub fn i_037fc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FCA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x37FCA);
    cpu.pc = 0x330D0;
    16
}
/// $037FCA: bcc.w $38020
pub fn i_037fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FCE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38020; return 10; }
    8
}
/// $037FCE: cmpi.b #$ff, d1
pub fn i_037fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FD2;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $037FD2: bne.w $37fe0
pub fn i_037fd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FD6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37FE0; return 10; }
    8
}
/// $037FD6: lea.l $35d34(pc), a1
pub fn i_037fd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FDA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x35D34;
    16
}
/// $037FDA: move.l a1, (a6)
pub fn i_037fda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FDC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FDC: bra.w $38020
pub fn i_037fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FE0;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $037FE0: cmpi.b #$3, d1
pub fn i_037fe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FE4;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $037FE4: bne.w $37ff2
pub fn i_037fe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x37FF2; return 10; }
    8
}
/// $037FE8: lea.l $360bc(pc), a1
pub fn i_037fe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x360BC;
    16
}
/// $037FEC: move.l a1, (a6)
pub fn i_037fec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $037FEE: bra.w $38020
pub fn i_037fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FF2;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $037FF2: cmpi.b #$4, d1
pub fn i_037ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FF6;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $037FF6: bne.w $38004
pub fn i_037ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FFA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38004; return 10; }
    8
}
/// $037FFA: lea.l $3873c(pc), a1
pub fn i_037ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x37FFE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $037FFE: move.l a1, (a6)
pub fn i_037ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38000;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038000: bra.w $38020
pub fn i_038000(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38004;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $038004: cmpi.b #$1, d1
pub fn i_038004(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38008;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $038008: bne.w $38016
pub fn i_038008(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3800C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38016; return 10; }
    8
}
/// $03800C: lea.l $3437e(pc), a1
pub fn i_03800c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38010;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3437E;
    16
}
/// $038010: move.l a1, (a6)
pub fn i_038010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38012;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038012: bra.w $38020
pub fn i_038012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38016;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $038016: lea.l $342c4(pc), a1
pub fn i_038016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3801A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x342C4;
    16
}
/// $03801A: move.l a1, (a6)
pub fn i_03801a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3801C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03801C: bra.w $38020
pub fn i_03801c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38020;
    cpu.ir = 0x6000;
    cpu.pc = 0x38020;
    10
}
/// $038020: jsr $27eba.l
pub fn i_038020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38026;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38026);
    cpu.pc = 0x27EBA;
    16
}
/// $038026: bcc.w $38030
pub fn i_038026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3802A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38030; return 10; }
    8
}
/// $03802A: lea.l $36d64(pc), a1
pub fn i_03802a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3802E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $03802E: move.l a1, (a6)
pub fn i_03802e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38030;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038030: jsr $33034(pc)
pub fn i_038030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38034;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38034);
    cpu.pc = 0x33034;
    16
}
/// $038034: bcc.w $3803e
pub fn i_038034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38038;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3803E; return 10; }
    8
}
/// $038038: lea.l $36914(pc), a1
pub fn i_038038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3803C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $03803C: move.l a1, (a6)
pub fn i_03803c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3803E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03803E: jsr $33522(pc)
pub fn i_03803e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38042;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38042);
    cpu.pc = 0x33522;
    16
}
/// $038042: rts
pub fn i_038042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38044;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchReenterByInput_038044  ($038044, 66 B, player/air_death_crouch_036632.s)
/// $038044: btst.b #$0, $3a(a6)
pub fn i_038044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3804A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03804A: bne.w $3806a
pub fn i_03804a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3804E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3806A; return 10; }
    8
}
/// $03804E: jsr $32e3c(pc)
pub fn i_03804e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38052;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38052);
    cpu.pc = 0x32E3C;
    16
}
/// $038052: bcc.w $3805a
pub fn i_038052(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38056;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3805A; return 10; }
    8
}
/// $038056: jmp $3842c(pc)
pub fn i_038056(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3805A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3842C;
    8
}
/// $03805A: jsr $32e42(pc)
pub fn i_03805a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3805E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3805E);
    cpu.pc = 0x32E42;
    16
}
/// $03805E: bcc.w $38066
pub fn i_03805e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38062;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38066; return 10; }
    8
}
/// $038062: jmp $386e2(pc)
pub fn i_038062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38066;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x386E2;
    8
}
/// $038066: bra.w $38082
pub fn i_038066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3806A;
    cpu.ir = 0x6000;
    cpu.pc = 0x38082;
    10
}
/// $03806A: jsr $32e42(pc)
pub fn i_03806a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3806E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3806E);
    cpu.pc = 0x32E42;
    16
}
/// $03806E: bcc.w $38076
pub fn i_03806e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38072;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38076; return 10; }
    8
}
/// $038072: jmp $3855a(pc)
pub fn i_038072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38076;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3855A;
    8
}
/// $038076: jsr $32e3c(pc)
pub fn i_038076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3807A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3807A);
    cpu.pc = 0x32E3C;
    16
}
/// $03807A: bcc.w $38082
pub fn i_03807a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3807E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38082; return 10; }
    8
}
/// $03807E: jmp $38688(pc)
pub fn i_03807e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38082;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x38688;
    8
}
/// $038082: jmp $3827a(pc)
pub fn i_038082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38086;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x3827A;
    8
}
// ===== Player_CrouchWeaponEmpty_038086  ($038086, 284 B, player/air_death_crouch_036632.s)
/// $038086: cmpi.b #$0, $71(a6)
pub fn i_038086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3808C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $03808C: bne.w $38094
pub fn i_03808c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38090;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38094; return 10; }
    8
}
/// $038090: jmp $381a2(pc)
pub fn i_038090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38094;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x381A2;
    8
}
/// $038094: bset.b #$2, $8c(a6)
pub fn i_038094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3809A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03809A: bclr.b #$1, $8c(a6)
pub fn i_03809a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380A0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0380A0: bclr.b #$3, $8c(a6)
pub fn i_0380a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380A6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0380A6: move.b #$20, $70(a6)
pub fn i_0380a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380AC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x20;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0380AC: lea.l $279832.l, a0
pub fn i_0380ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279832;
    20
}
/// $0380B2: move.l -$4(a0), $74(a6)
pub fn i_0380b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380B8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0380B8: move.b #$ff, $21(a6)
pub fn i_0380b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0380BE: lea.l $279832.l, a0
pub fn i_0380be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380C4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279832;
    20
}
/// $0380C4: jsr $28cd4.l
pub fn i_0380c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x380CA);
    cpu.pc = 0x28CD4;
    16
}
/// $0380CA: lea.l $ffff.w, a0
pub fn i_0380ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380CE;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $0380CE: move.l a0, $48(a6)
pub fn i_0380ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380D2;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0380D2: lea.l $38be4(pc), a1
pub fn i_0380d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38BE4;
    16
}
/// $0380D6: jsr $4ae.l
pub fn i_0380d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x380DC);
    cpu.pc = 0x4AE;
    16
}
/// $0380DC: jsr $5dd02.l
pub fn i_0380dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x380E2);
    cpu.pc = 0x5DD02;
    16
}
/// $0380E2: move.b #$0, $71(a6)
pub fn i_0380e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380E8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0380E8: move.w #$a, $82(a6)
pub fn i_0380e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380EE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0380EE: bset.b #$7, $5b(a6)
pub fn i_0380ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380F4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0380F4: move.l #$32500, $60(a6)
pub fn i_0380f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x380FC;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0380FC: jsr $267e6.l
pub fn i_0380fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38102;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38102);
    cpu.pc = 0x267E6;
    16
}
/// $038102: clr.w $28(a6)
pub fn i_038102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38106;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $038106: lea.l $3810c(pc), a1
pub fn i_038106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3810A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3810C;
    16
}
/// $03810A: move.l a1, (a6)
pub fn i_03810a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3810C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03810C: bset.b #$6, $8d(a6)
pub fn i_03810c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38112;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038112: jsr $32ff2(pc)
pub fn i_038112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38116;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38116);
    cpu.pc = 0x32FF2;
    16
}
/// $038116: jsr $32d32(pc)
pub fn i_038116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3811A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3811A);
    cpu.pc = 0x32D32;
    16
}
/// $03811A: jsr $27a92.l
pub fn i_03811a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38120;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38120);
    cpu.pc = 0x27A92;
    16
}
/// $038120: jsr $32aa8(pc)
pub fn i_038120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38124;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38124);
    cpu.pc = 0x32AA8;
    16
}
/// $038124: bcc.w $3812e
pub fn i_038124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38128;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3812E; return 10; }
    8
}
/// $038128: lea.l $381a2(pc), a1
pub fn i_038128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3812C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x381A2;
    16
}
/// $03812C: move.l a1, (a6)
pub fn i_03812c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3812E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03812E: jsr $5cef8.l
pub fn i_03812e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38134;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38134);
    cpu.pc = 0x5CEF8;
    16
}
/// $038134: bcs.w $3813e
pub fn i_038134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38138;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3813E; return 10; }
    8
}
/// $038138: lea.l $37ec2(pc), a1
pub fn i_038138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3813C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $03813C: move.l a1, (a6)
pub fn i_03813c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3813E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03813E: btst.b #$2, $8c(a6)
pub fn i_03813e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38144;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $038144: bne.w $3817e
pub fn i_038144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38148;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3817E; return 10; }
    8
}
/// $038148: jsr $330d0(pc)
pub fn i_038148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3814C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3814C);
    cpu.pc = 0x330D0;
    16
}
/// $03814C: bcc.w $3817e
pub fn i_03814c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38150;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3817E; return 10; }
    8
}
/// $038150: cmpi.b #$ff, d1
pub fn i_038150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38154;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $038154: bne.w $38162
pub fn i_038154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38158;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38162; return 10; }
    8
}
/// $038158: lea.l $38a28(pc), a1
pub fn i_038158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3815C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $03815C: move.l a1, (a6)
pub fn i_03815c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3815E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03815E: bra.w $3817e
pub fn i_03815e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38162;
    cpu.ir = 0x6000;
    cpu.pc = 0x3817E;
    10
}
/// $038162: cmpi.b #$3, d1
pub fn i_038162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38166;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $038166: bne.w $38174
pub fn i_038166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3816A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38174; return 10; }
    8
}
/// $03816A: lea.l $388f0(pc), a1
pub fn i_03816a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3816E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $03816E: move.l a1, (a6)
pub fn i_03816e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38170;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038170: bra.w $3817e
pub fn i_038170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38174;
    cpu.ir = 0x6000;
    cpu.pc = 0x3817E;
    10
}
/// $038174: lea.l $3873c(pc), a1
pub fn i_038174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38178;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $038178: move.l a1, (a6)
pub fn i_038178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3817A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03817A: bra.w $3817e
pub fn i_03817a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3817E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3817E;
    10
}
/// $03817E: jsr $27eba.l
pub fn i_03817e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38184;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38184);
    cpu.pc = 0x27EBA;
    16
}
/// $038184: bcc.w $3818e
pub fn i_038184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38188;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3818E; return 10; }
    8
}
/// $038188: lea.l $36d64(pc), a1
pub fn i_038188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3818C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36D64;
    16
}
/// $03818C: move.l a1, (a6)
pub fn i_03818c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3818E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03818E: jsr $33034(pc)
pub fn i_03818e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38192;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38192);
    cpu.pc = 0x33034;
    16
}
/// $038192: bcc.w $3819c
pub fn i_038192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38196;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3819C; return 10; }
    8
}
/// $038196: lea.l $36914(pc), a1
pub fn i_038196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3819A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $03819A: move.l a1, (a6)
pub fn i_03819a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3819C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03819C: jsr $33522(pc)
pub fn i_03819c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381A0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x381A0);
    cpu.pc = 0x33522;
    16
}
/// $0381A0: rts
pub fn i_0381a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchIdleB_0381a2  ($0381A2, 216 B, player/air_death_crouch_036632.s)
/// $0381A2: bclr.b #$2, $8c(a6)
pub fn i_0381a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381A8;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0381A8: bclr.b #$1, $8c(a6)
pub fn i_0381a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381AE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0381AE: bclr.b #$3, $8c(a6)
pub fn i_0381ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381B4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0381B4: jsr $2abcc.l
pub fn i_0381b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x381BA);
    cpu.pc = 0x2ABCC;
    16
}
/// $0381BA: bcs.w $381e6
pub fn i_0381ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x381E6; return 10; }
    8
}
/// $0381BE: move.b #$30, $70(a6)
pub fn i_0381be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0381C4: lea.l $279850.l, a0
pub fn i_0381c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279850;
    20
}
/// $0381CA: move.l -$4(a0), $74(a6)
pub fn i_0381ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381D0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0381D0: move.b #$ff, $21(a6)
pub fn i_0381d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0381D6: lea.l $279850.l, a0
pub fn i_0381d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279850;
    20
}
/// $0381DC: jsr $28cd4.l
pub fn i_0381dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x381E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0381E2: bra.w $3820a
pub fn i_0381e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x3820A;
    10
}
/// $0381E6: move.b #$30, $70(a6)
pub fn i_0381e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381EC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0381EC: lea.l $279a82.l, a0
pub fn i_0381ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381F2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $0381F2: move.l -$4(a0), $74(a6)
pub fn i_0381f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381F8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0381F8: move.b #$ff, $21(a6)
pub fn i_0381f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x381FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0381FE: lea.l $279a82.l, a0
pub fn i_0381fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38204;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $038204: jsr $28cd4.l
pub fn i_038204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3820A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3820A);
    cpu.pc = 0x28CD4;
    16
}
/// $03820A: bra.w $382e2
pub fn i_03820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3820E;
    cpu.ir = 0x6000;
    cpu.pc = 0x382E2;
    10
}
/// $03820E: bclr.b #$2, $8c(a6)
pub fn i_03820e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38214;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038214: bclr.b #$1, $8c(a6)
pub fn i_038214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3821A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03821A: bclr.b #$3, $8c(a6)
pub fn i_03821a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38220;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038220: jsr $2abcc.l
pub fn i_038220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38226;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38226);
    cpu.pc = 0x2ABCC;
    16
}
/// $038226: bcs.w $38252
pub fn i_038226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3822A;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38252; return 10; }
    8
}
/// $03822A: move.b #$30, $70(a6)
pub fn i_03822a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38230;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038230: lea.l $27985a.l, a0
pub fn i_038230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38236;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27985A;
    20
}
/// $038236: move.l -$4(a0), $74(a6)
pub fn i_038236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3823C;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03823C: move.b #$ff, $21(a6)
pub fn i_03823c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38242;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038242: lea.l $27985a.l, a0
pub fn i_038242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38248;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27985A;
    20
}
/// $038248: jsr $28cd4.l
pub fn i_038248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3824E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3824E);
    cpu.pc = 0x28CD4;
    16
}
/// $03824E: bra.w $38276
pub fn i_03824e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38252;
    cpu.ir = 0x6000;
    cpu.pc = 0x38276;
    10
}
/// $038252: move.b #$30, $70(a6)
pub fn i_038252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38258;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038258: lea.l $279a82.l, a0
pub fn i_038258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3825E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $03825E: move.l -$4(a0), $74(a6)
pub fn i_03825e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38264;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038264: move.b #$ff, $21(a6)
pub fn i_038264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3826A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03826A: lea.l $279a82.l, a0
pub fn i_03826a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38270;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $038270: jsr $28cd4.l
pub fn i_038270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38276;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38276);
    cpu.pc = 0x28CD4;
    16
}
/// $038276: bra.w $382e2
pub fn i_038276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3827A;
    cpu.ir = 0x6000;
    cpu.pc = 0x382E2;
    10
}
// ===== Player_CrouchIdle_03827a  ($03827A, 434 B, player/air_death_crouch_036632.s)
/// $03827A: bclr.b #$2, $8c(a6)
pub fn i_03827a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38280;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038280: bclr.b #$1, $8c(a6)
pub fn i_038280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38286;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038286: bclr.b #$3, $8c(a6)
pub fn i_038286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3828C;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03828C: jsr $2abcc.l
pub fn i_03828c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38292;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38292);
    cpu.pc = 0x2ABCC;
    16
}
/// $038292: bcs.w $382be
pub fn i_038292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38296;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x382BE; return 10; }
    8
}
/// $038296: move.b #$30, $70(a6)
pub fn i_038296(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3829C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03829C: lea.l $27983c.l, a0
pub fn i_03829c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382A2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27983C;
    20
}
/// $0382A2: move.l -$4(a0), $74(a6)
pub fn i_0382a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382A8;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0382A8: move.b #$ff, $21(a6)
pub fn i_0382a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382AE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0382AE: lea.l $27983c.l, a0
pub fn i_0382ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382B4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27983C;
    20
}
/// $0382B4: jsr $28cd4.l
pub fn i_0382b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x382BA);
    cpu.pc = 0x28CD4;
    16
}
/// $0382BA: bra.w $382e2
pub fn i_0382ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x382E2;
    10
}
/// $0382BE: move.b #$30, $70(a6)
pub fn i_0382be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382C4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0382C4: lea.l $279a82.l, a0
pub fn i_0382c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382CA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $0382CA: move.l -$4(a0), $74(a6)
pub fn i_0382ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382D0;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0382D0: move.b #$ff, $21(a6)
pub fn i_0382d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382D6;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0382D6: lea.l $279a82.l, a0
pub fn i_0382d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382DC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $0382DC: jsr $28cd4.l
pub fn i_0382dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x382E2);
    cpu.pc = 0x28CD4;
    16
}
/// $0382E2: move.l #$32598, $60(a6)
pub fn i_0382e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382EA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0382EA: jsr $267e6.l
pub fn i_0382ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x382F0);
    cpu.pc = 0x267E6;
    16
}
/// $0382F0: clr.w $28(a6)
pub fn i_0382f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382F4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0382F4: clr.w $2c(a6)
pub fn i_0382f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382F8;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0382F8: lea.l $32734(pc), a0
pub fn i_0382f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x382FC;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $0382FC: move.l a0, $48(a6)
pub fn i_0382fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38300;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038300: lea.l $38306(pc), a1
pub fn i_038300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38304;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38306;
    16
}
/// $038304: move.l a1, (a6)
pub fn i_038304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38306;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038306: bset.b #$6, $8d(a6)
pub fn i_038306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3830C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03830C: jsr $32ff2(pc)
pub fn i_03830c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38310;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38310);
    cpu.pc = 0x32FF2;
    16
}
/// $038310: jsr $32d32(pc)
pub fn i_038310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38314;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38314);
    cpu.pc = 0x32D32;
    16
}
/// $038314: bcc.w $3831e
pub fn i_038314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38318;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3831E; return 10; }
    8
}
/// $038318: lea.l $38044(pc), a1
pub fn i_038318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3831C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38044;
    16
}
/// $03831C: move.l a1, (a6)
pub fn i_03831c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3831E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03831E: jsr $27a92.l
pub fn i_03831e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38324);
    cpu.pc = 0x27A92;
    16
}
/// $038324: jsr $32aa8(pc)
pub fn i_038324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38328;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38328);
    cpu.pc = 0x32AA8;
    16
}
/// $038328: btst.b #$0, $3a(a6)
pub fn i_038328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3832E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03832E: bne.w $38352
pub fn i_03832e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38332;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38352; return 10; }
    8
}
/// $038332: jsr $32e3c(pc)
pub fn i_038332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38336;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38336);
    cpu.pc = 0x32E3C;
    16
}
/// $038336: bcc.w $38340
pub fn i_038336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3833A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38340; return 10; }
    8
}
/// $03833A: lea.l $3842c(pc), a1
pub fn i_03833a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3833E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3842C;
    16
}
/// $03833E: move.l a1, (a6)
pub fn i_03833e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38340;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038340: jsr $32e42(pc)
pub fn i_038340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38344;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38344);
    cpu.pc = 0x32E42;
    16
}
/// $038344: bcc.w $3834e
pub fn i_038344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38348;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3834E; return 10; }
    8
}
/// $038348: lea.l $386e2(pc), a1
pub fn i_038348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3834C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x386E2;
    16
}
/// $03834C: move.l a1, (a6)
pub fn i_03834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3834E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03834E: bra.w $3836e
pub fn i_03834e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38352;
    cpu.ir = 0x6000;
    cpu.pc = 0x3836E;
    10
}
/// $038352: jsr $32e3c(pc)
pub fn i_038352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38356;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38356);
    cpu.pc = 0x32E3C;
    16
}
/// $038356: bcc.w $38360
pub fn i_038356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3835A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38360; return 10; }
    8
}
/// $03835A: lea.l $38688(pc), a1
pub fn i_03835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3835E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38688;
    16
}
/// $03835E: move.l a1, (a6)
pub fn i_03835e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38360;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038360: jsr $32e42(pc)
pub fn i_038360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38364;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38364);
    cpu.pc = 0x32E42;
    16
}
/// $038364: bcc.w $3836e
pub fn i_038364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38368;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3836E; return 10; }
    8
}
/// $038368: lea.l $3855a(pc), a1
pub fn i_038368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3836C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3855A;
    16
}
/// $03836C: move.l a1, (a6)
pub fn i_03836c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3836E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03836E: jsr $5cef8.l
pub fn i_03836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38374;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38374);
    cpu.pc = 0x5CEF8;
    16
}
/// $038374: bcs.w $3837e
pub fn i_038374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38378;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3837E; return 10; }
    8
}
/// $038378: lea.l $37ec2(pc), a1
pub fn i_038378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3837C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $03837C: move.l a1, (a6)
pub fn i_03837c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3837E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03837E: jsr $330d0(pc)
pub fn i_03837e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38382;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38382);
    cpu.pc = 0x330D0;
    16
}
/// $038382: bcc.w $383b4
pub fn i_038382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38386;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x383B4; return 10; }
    8
}
/// $038386: cmpi.b #$ff, d1
pub fn i_038386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3838A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03838A: bne.w $38398
pub fn i_03838a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3838E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38398; return 10; }
    8
}
/// $03838E: lea.l $38a28(pc), a1
pub fn i_03838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38392;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $038392: move.l a1, (a6)
pub fn i_038392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38394;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038394: bra.w $383b4
pub fn i_038394(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38398;
    cpu.ir = 0x6000;
    cpu.pc = 0x383B4;
    10
}
/// $038398: cmpi.b #$3, d1
pub fn i_038398(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3839C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03839C: bne.w $383aa
pub fn i_03839c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383A0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x383AA; return 10; }
    8
}
/// $0383A0: lea.l $388f0(pc), a1
pub fn i_0383a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383A4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $0383A4: move.l a1, (a6)
pub fn i_0383a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383A6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383A6: bra.w $383b4
pub fn i_0383a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x383B4;
    10
}
/// $0383AA: lea.l $3873c(pc), a1
pub fn i_0383aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $0383AE: move.l a1, (a6)
pub fn i_0383ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383B0: bra.w $383b4
pub fn i_0383b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x383B4;
    10
}
/// $0383B4: cmpi.w #$0, $82(a6)
pub fn i_0383b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383BA;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $0383BA: bne.w $383ce
pub fn i_0383ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383BE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x383CE; return 10; }
    8
}
/// $0383BE: cmpi.b #$0, $71(a6)
pub fn i_0383be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383C4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $0383C4: beq.w $383ce
pub fn i_0383c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383C8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x383CE; return 10; }
    8
}
/// $0383C8: lea.l $38086(pc), a1
pub fn i_0383c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383CC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38086;
    16
}
/// $0383CC: move.l a1, (a6)
pub fn i_0383cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383CE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383CE: jsr $27eba.l
pub fn i_0383ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x383D4);
    cpu.pc = 0x27EBA;
    16
}
/// $0383D4: bcc.w $383de
pub fn i_0383d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383D8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x383DE; return 10; }
    8
}
/// $0383D8: lea.l $36fc2(pc), a1
pub fn i_0383d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383DC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FC2;
    16
}
/// $0383DC: move.l a1, (a6)
pub fn i_0383dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383DE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383DE: jsr $33034(pc)
pub fn i_0383de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x383E2);
    cpu.pc = 0x33034;
    16
}
/// $0383E2: bcc.w $383ec
pub fn i_0383e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x383EC; return 10; }
    8
}
/// $0383E6: lea.l $36914(pc), a1
pub fn i_0383e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $0383EA: move.l a1, (a6)
pub fn i_0383ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0383EC: jsr $33522(pc)
pub fn i_0383ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383F0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x383F0);
    cpu.pc = 0x33522;
    16
}
/// $0383F0: cmpi.b #$3, $106ece.l
pub fn i_0383f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383F8;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0383F8: beq.w $38410
pub fn i_0383f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x383FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38410; return 10; }
    8
}
/// $0383FC: movea.l #$ffffffff, a0
pub fn i_0383fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38402;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $038402: lea.l $324c6(pc), a0
pub fn i_038402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38406;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $038406: jsr $5dd56.l
pub fn i_038406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3840C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3840C);
    cpu.pc = 0x5DD56;
    16
}
/// $03840C: bra.w $38420
pub fn i_03840c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38410;
    cpu.ir = 0x6000;
    cpu.pc = 0x38420;
    10
}
/// $038410: movea.l #$ffffffff, a0
pub fn i_038410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38416;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $038416: lea.l $324bc(pc), a0
pub fn i_038416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3841A;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $03841A: jsr $5dd56.l
pub fn i_03841a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38420;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38420);
    cpu.pc = 0x5DD56;
    16
}
/// $038420: bcc.w $3842a
pub fn i_038420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38424;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3842A; return 10; }
    8
}
/// $038424: lea.l $37b8e(pc), a1
pub fn i_038424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38428;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $038428: move.l a1, (a6)
pub fn i_038428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3842A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03842A: rts
pub fn i_03842a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3842C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrawlRight_03842c  ($03842C, 302 B, player/air_death_crouch_036632.s)
/// $03842C: jsr $2abcc.l
pub fn i_03842c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38432;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38432);
    cpu.pc = 0x2ABCC;
    16
}
/// $038432: bcs.w $3845e
pub fn i_038432(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38436;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3845E; return 10; }
    8
}
/// $038436: move.b #$30, $70(a6)
pub fn i_038436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3843C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03843C: lea.l $279a82.l, a0
pub fn i_03843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38442;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $038442: move.l -$4(a0), $74(a6)
pub fn i_038442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38448;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038448: move.b #$ff, $21(a6)
pub fn i_038448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3844E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03844E: lea.l $279a82.l, a0
pub fn i_03844e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38454;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $038454: jsr $28cd4.l
pub fn i_038454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3845A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3845A);
    cpu.pc = 0x28CD4;
    16
}
/// $03845A: bra.w $38482
pub fn i_03845a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3845E;
    cpu.ir = 0x6000;
    cpu.pc = 0x38482;
    10
}
/// $03845E: move.b #$30, $70(a6)
pub fn i_03845e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38464;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038464: lea.l $279a82.l, a0
pub fn i_038464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3846A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $03846A: move.l -$4(a0), $74(a6)
pub fn i_03846a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38470;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038470: move.b #$ff, $21(a6)
pub fn i_038470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38476;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038476: lea.l $279a82.l, a0
pub fn i_038476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3847C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $03847C: jsr $28cd4.l
pub fn i_03847c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38482;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38482);
    cpu.pc = 0x28CD4;
    16
}
/// $038482: bclr.b #$2, $8c(a6)
pub fn i_038482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38488;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038488: bclr.b #$1, $8c(a6)
pub fn i_038488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3848E;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03848E: bclr.b #$3, $8c(a6)
pub fn i_03848e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38494;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038494: move.l #$32598, $60(a6)
pub fn i_038494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3849C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $03849C: move.w #$120, $28(a6)
pub fn i_03849c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0384A2: clr.w $2c(a6)
pub fn i_0384a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384A6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0384A6: lea.l $32734(pc), a0
pub fn i_0384a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384AA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $0384AA: move.l a0, $48(a6)
pub fn i_0384aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384AE;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0384AE: bclr.b #$0, $3a(a6)
pub fn i_0384ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384B4;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0384B4: lea.l $384ba(pc), a1
pub fn i_0384b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x384BA;
    16
}
/// $0384B8: move.l a1, (a6)
pub fn i_0384b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0384BA: bset.b #$6, $8d(a6)
pub fn i_0384ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384C0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0384C0: jsr $32ff2(pc)
pub fn i_0384c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x384C4);
    cpu.pc = 0x32FF2;
    16
}
/// $0384C4: jsr $32d32(pc)
pub fn i_0384c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384C8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x384C8);
    cpu.pc = 0x32D32;
    16
}
/// $0384C8: bcc.w $384d2
pub fn i_0384c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384CC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x384D2; return 10; }
    8
}
/// $0384CC: lea.l $38044(pc), a1
pub fn i_0384cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38044;
    16
}
/// $0384D0: move.l a1, (a6)
pub fn i_0384d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0384D2: move.w #$120, $28(a6)
pub fn i_0384d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384D8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0384D8: jsr $27a92.l
pub fn i_0384d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x384DE);
    cpu.pc = 0x27A92;
    16
}
/// $0384DE: jsr $32aa8(pc)
pub fn i_0384de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384E2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x384E2);
    cpu.pc = 0x32AA8;
    16
}
/// $0384E2: jsr $32e3c(pc)
pub fn i_0384e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x384E6);
    cpu.pc = 0x32E3C;
    16
}
/// $0384E6: bcs.w $384f0
pub fn i_0384e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384EA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x384F0; return 10; }
    8
}
/// $0384EA: lea.l $3827a(pc), a1
pub fn i_0384ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3827A;
    16
}
/// $0384EE: move.l a1, (a6)
pub fn i_0384ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0384F0: jsr $5cef8.l
pub fn i_0384f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x384F6);
    cpu.pc = 0x5CEF8;
    16
}
/// $0384F6: bcs.w $38500
pub fn i_0384f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384FA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38500; return 10; }
    8
}
/// $0384FA: lea.l $37ec2(pc), a1
pub fn i_0384fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x384FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $0384FE: move.l a1, (a6)
pub fn i_0384fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38500;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038500: jsr $330d0(pc)
pub fn i_038500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38504;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38504);
    cpu.pc = 0x330D0;
    16
}
/// $038504: bcc.w $38536
pub fn i_038504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38508;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38536; return 10; }
    8
}
/// $038508: cmpi.b #$ff, d1
pub fn i_038508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3850C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03850C: bne.w $3851a
pub fn i_03850c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38510;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3851A; return 10; }
    8
}
/// $038510: lea.l $38a28(pc), a1
pub fn i_038510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38514;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $038514: move.l a1, (a6)
pub fn i_038514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38516;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038516: bra.w $38536
pub fn i_038516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3851A;
    cpu.ir = 0x6000;
    cpu.pc = 0x38536;
    10
}
/// $03851A: cmpi.b #$3, d1
pub fn i_03851a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3851E;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03851E: bne.w $3852c
pub fn i_03851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38522;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3852C; return 10; }
    8
}
/// $038522: lea.l $388f0(pc), a1
pub fn i_038522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38526;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $038526: move.l a1, (a6)
pub fn i_038526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38528;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038528: bra.w $38536
pub fn i_038528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3852C;
    cpu.ir = 0x6000;
    cpu.pc = 0x38536;
    10
}
/// $03852C: lea.l $3873c(pc), a1
pub fn i_03852c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38530;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $038530: move.l a1, (a6)
pub fn i_038530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38532;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038532: bra.w $38536
pub fn i_038532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38536;
    cpu.ir = 0x6000;
    cpu.pc = 0x38536;
    10
}
/// $038536: jsr $27eba.l
pub fn i_038536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3853C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3853C);
    cpu.pc = 0x27EBA;
    16
}
/// $03853C: bcc.w $38546
pub fn i_03853c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38540;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38546; return 10; }
    8
}
/// $038540: lea.l $36fc2(pc), a1
pub fn i_038540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38544;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FC2;
    16
}
/// $038544: move.l a1, (a6)
pub fn i_038544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38546;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038546: jsr $33034(pc)
pub fn i_038546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3854A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3854A);
    cpu.pc = 0x33034;
    16
}
/// $03854A: bcc.w $38554
pub fn i_03854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3854E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38554; return 10; }
    8
}
/// $03854E: lea.l $36914(pc), a1
pub fn i_03854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38552;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $038552: move.l a1, (a6)
pub fn i_038552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38554;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038554: jsr $33522(pc)
pub fn i_038554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38558;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38558);
    cpu.pc = 0x33522;
    16
}
/// $038558: rts
pub fn i_038558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3855A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrawlLeft_03855a  ($03855A, 302 B, player/air_death_crouch_036632.s)
/// $03855A: jsr $2abcc.l
pub fn i_03855a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38560;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38560);
    cpu.pc = 0x2ABCC;
    16
}
/// $038560: bcs.w $3858c
pub fn i_038560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38564;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3858C; return 10; }
    8
}
/// $038564: move.b #$30, $70(a6)
pub fn i_038564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3856A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03856A: lea.l $279a82.l, a0
pub fn i_03856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38570;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $038570: move.l -$4(a0), $74(a6)
pub fn i_038570(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38576;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038576: move.b #$ff, $21(a6)
pub fn i_038576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3857C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03857C: lea.l $279a82.l, a0
pub fn i_03857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38582;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $038582: jsr $28cd4.l
pub fn i_038582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38588;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38588);
    cpu.pc = 0x28CD4;
    16
}
/// $038588: bra.w $385b0
pub fn i_038588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3858C;
    cpu.ir = 0x6000;
    cpu.pc = 0x385B0;
    10
}
/// $03858C: move.b #$30, $70(a6)
pub fn i_03858c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38592;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038592: lea.l $279a82.l, a0
pub fn i_038592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38598;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $038598: move.l -$4(a0), $74(a6)
pub fn i_038598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3859E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03859E: move.b #$ff, $21(a6)
pub fn i_03859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385A4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0385A4: lea.l $279a82.l, a0
pub fn i_0385a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385AA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A82;
    20
}
/// $0385AA: jsr $28cd4.l
pub fn i_0385aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x385B0);
    cpu.pc = 0x28CD4;
    16
}
/// $0385B0: bclr.b #$2, $8c(a6)
pub fn i_0385b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385B6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0385B6: bclr.b #$1, $8c(a6)
pub fn i_0385b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385BC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0385BC: bclr.b #$3, $8c(a6)
pub fn i_0385bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385C2;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0385C2: move.l #$32598, $60(a6)
pub fn i_0385c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385CA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0385CA: move.w #$fee0, $28(a6)
pub fn i_0385ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385D0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0385D0: clr.w $2c(a6)
pub fn i_0385d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385D4;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0385D4: lea.l $32734(pc), a0
pub fn i_0385d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385D8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $0385D8: move.l a0, $48(a6)
pub fn i_0385d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385DC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0385DC: bset.b #$0, $3a(a6)
pub fn i_0385dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385E2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0385E2: lea.l $385e8(pc), a1
pub fn i_0385e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385E6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x385E8;
    16
}
/// $0385E6: move.l a1, (a6)
pub fn i_0385e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385E8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0385E8: bset.b #$6, $8d(a6)
pub fn i_0385e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385EE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0385EE: jsr $32ff2(pc)
pub fn i_0385ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385F2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x385F2);
    cpu.pc = 0x32FF2;
    16
}
/// $0385F2: jsr $32d32(pc)
pub fn i_0385f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385F6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x385F6);
    cpu.pc = 0x32D32;
    16
}
/// $0385F6: bcc.w $38600
pub fn i_0385f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385FA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38600; return 10; }
    8
}
/// $0385FA: lea.l $38044(pc), a1
pub fn i_0385fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x385FE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38044;
    16
}
/// $0385FE: move.l a1, (a6)
pub fn i_0385fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38600;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038600: move.w #$fee0, $28(a6)
pub fn i_038600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38606;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFEE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038606: jsr $27a92.l
pub fn i_038606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3860C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3860C);
    cpu.pc = 0x27A92;
    16
}
/// $03860C: jsr $32aa8(pc)
pub fn i_03860c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38610;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38610);
    cpu.pc = 0x32AA8;
    16
}
/// $038610: jsr $32e42(pc)
pub fn i_038610(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38614;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38614);
    cpu.pc = 0x32E42;
    16
}
/// $038614: bcs.w $3861e
pub fn i_038614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38618;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3861E; return 10; }
    8
}
/// $038618: lea.l $3827a(pc), a1
pub fn i_038618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3861C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3827A;
    16
}
/// $03861C: move.l a1, (a6)
pub fn i_03861c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3861E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03861E: jsr $5cef8.l
pub fn i_03861e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38624;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38624);
    cpu.pc = 0x5CEF8;
    16
}
/// $038624: bcs.w $3862e
pub fn i_038624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38628;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3862E; return 10; }
    8
}
/// $038628: lea.l $37ec2(pc), a1
pub fn i_038628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3862C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $03862C: move.l a1, (a6)
pub fn i_03862c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3862E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03862E: jsr $330d0(pc)
pub fn i_03862e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38632;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38632);
    cpu.pc = 0x330D0;
    16
}
/// $038632: bcc.w $38664
pub fn i_038632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38636;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38664; return 10; }
    8
}
/// $038636: cmpi.b #$ff, d1
pub fn i_038636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3863A;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $03863A: bne.w $38648
pub fn i_03863a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3863E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38648; return 10; }
    8
}
/// $03863E: lea.l $38a28(pc), a1
pub fn i_03863e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38642;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $038642: move.l a1, (a6)
pub fn i_038642(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38644;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038644: bra.w $38664
pub fn i_038644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38648;
    cpu.ir = 0x6000;
    cpu.pc = 0x38664;
    10
}
/// $038648: cmpi.b #$3, d1
pub fn i_038648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3864C;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $03864C: bne.w $3865a
pub fn i_03864c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38650;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3865A; return 10; }
    8
}
/// $038650: lea.l $388f0(pc), a1
pub fn i_038650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38654;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $038654: move.l a1, (a6)
pub fn i_038654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38656;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038656: bra.w $38664
pub fn i_038656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3865A;
    cpu.ir = 0x6000;
    cpu.pc = 0x38664;
    10
}
/// $03865A: lea.l $3873c(pc), a1
pub fn i_03865a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3865E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $03865E: move.l a1, (a6)
pub fn i_03865e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38660;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038660: bra.w $38664
pub fn i_038660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38664;
    cpu.ir = 0x6000;
    cpu.pc = 0x38664;
    10
}
/// $038664: jsr $27eba.l
pub fn i_038664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3866A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3866A);
    cpu.pc = 0x27EBA;
    16
}
/// $03866A: bcc.w $38674
pub fn i_03866a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3866E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38674; return 10; }
    8
}
/// $03866E: lea.l $36fc2(pc), a1
pub fn i_03866e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38672;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FC2;
    16
}
/// $038672: move.l a1, (a6)
pub fn i_038672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38674;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038674: jsr $33034(pc)
pub fn i_038674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38678;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38678);
    cpu.pc = 0x33034;
    16
}
/// $038678: bcc.w $38682
pub fn i_038678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3867C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38682; return 10; }
    8
}
/// $03867C: lea.l $36914(pc), a1
pub fn i_03867c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38680;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $038680: move.l a1, (a6)
pub fn i_038680(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38682;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038682: jsr $33522(pc)
pub fn i_038682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38686;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38686);
    cpu.pc = 0x33522;
    16
}
/// $038686: rts
pub fn i_038686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38688;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrawlRight_Alt_038688  ($038688, 90 B, player/air_death_crouch_036632.s)
/// $038688: jsr $2abcc.l
pub fn i_038688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3868E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3868E);
    cpu.pc = 0x2ABCC;
    16
}
/// $03868E: bcs.w $386ba
pub fn i_03868e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38692;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x386BA; return 10; }
    8
}
/// $038692: move.b #$30, $70(a6)
pub fn i_038692(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38698;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038698: lea.l $279a78.l, a0
pub fn i_038698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3869E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A78;
    20
}
/// $03869E: move.l -$4(a0), $74(a6)
pub fn i_03869e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386A4;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0386A4: move.b #$ff, $21(a6)
pub fn i_0386a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0386AA: lea.l $279a78.l, a0
pub fn i_0386aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A78;
    20
}
/// $0386B0: jsr $28cd4.l
pub fn i_0386b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x386B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0386B6: bra.w $386de
pub fn i_0386b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x386DE;
    10
}
/// $0386BA: move.b #$30, $70(a6)
pub fn i_0386ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386C0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0386C0: lea.l $279a78.l, a0
pub fn i_0386c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386C6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A78;
    20
}
/// $0386C6: move.l -$4(a0), $74(a6)
pub fn i_0386c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386CC;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0386CC: move.b #$ff, $21(a6)
pub fn i_0386cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386D2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0386D2: lea.l $279a78.l, a0
pub fn i_0386d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A78;
    20
}
/// $0386D8: jsr $28cd4.l
pub fn i_0386d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x386DE);
    cpu.pc = 0x28CD4;
    16
}
/// $0386DE: bra.w $38482
pub fn i_0386de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386E2;
    cpu.ir = 0x6000;
    cpu.pc = 0x38482;
    10
}
// ===== Player_CrawlLeft_Alt_0386e2  ($0386E2, 90 B, player/air_death_crouch_036632.s)
/// $0386E2: jsr $2abcc.l
pub fn i_0386e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386E8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x386E8);
    cpu.pc = 0x2ABCC;
    16
}
/// $0386E8: bcs.w $38714
pub fn i_0386e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386EC;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38714; return 10; }
    8
}
/// $0386EC: move.b #$30, $70(a6)
pub fn i_0386ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0386F2: lea.l $279a78.l, a0
pub fn i_0386f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386F8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A78;
    20
}
/// $0386F8: move.l -$4(a0), $74(a6)
pub fn i_0386f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x386FE;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0386FE: move.b #$ff, $21(a6)
pub fn i_0386fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38704;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038704: lea.l $279a78.l, a0
pub fn i_038704(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3870A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A78;
    20
}
/// $03870A: jsr $28cd4.l
pub fn i_03870a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38710;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38710);
    cpu.pc = 0x28CD4;
    16
}
/// $038710: bra.w $38738
pub fn i_038710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38714;
    cpu.ir = 0x6000;
    cpu.pc = 0x38738;
    10
}
/// $038714: move.b #$30, $70(a6)
pub fn i_038714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3871A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03871A: lea.l $279a78.l, a0
pub fn i_03871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38720;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A78;
    20
}
/// $038720: move.l -$4(a0), $74(a6)
pub fn i_038720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38726;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038726: move.b #$ff, $21(a6)
pub fn i_038726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3872C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03872C: lea.l $279a78.l, a0
pub fn i_03872c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38732;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279A78;
    20
}
/// $038732: jsr $28cd4.l
pub fn i_038732(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38738;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38738);
    cpu.pc = 0x28CD4;
    16
}
/// $038738: bra.w $385b0
pub fn i_038738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3873C;
    cpu.ir = 0x6000;
    cpu.pc = 0x385B0;
    10
}
// ===== Player_CrouchShoot_03873c  ($03873C, 436 B, player/air_death_crouch_036632.s)
/// $03873C: bset.b #$1, $8c(a6)
pub fn i_03873c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38742;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038742: bclr.b #$1, $8c(a6)
pub fn i_038742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38748;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038748: move.l #$32598, $60(a6)
pub fn i_038748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38750;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038750: jsr $267e6.l
pub fn i_038750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38756;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38756);
    cpu.pc = 0x267E6;
    16
}
/// $038756: clr.w $28(a6)
pub fn i_038756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3875A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03875A: clr.w $2c(a6)
pub fn i_03875a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3875E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03875E: jsr $2abcc.l
pub fn i_03875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38764;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38764);
    cpu.pc = 0x2ABCC;
    16
}
/// $038764: bcs.w $38790
pub fn i_038764(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38768;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38790; return 10; }
    8
}
/// $038768: move.b #$30, $70(a6)
pub fn i_038768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3876E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03876E: lea.l $279d06.l, a0
pub fn i_03876e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38774;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D06;
    20
}
/// $038774: move.l -$4(a0), $74(a6)
pub fn i_038774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3877A;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03877A: move.b #$ff, $21(a6)
pub fn i_03877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38780;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038780: lea.l $279d06.l, a0
pub fn i_038780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38786;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D06;
    20
}
/// $038786: jsr $28cd4.l
pub fn i_038786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3878C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3878C);
    cpu.pc = 0x28CD4;
    16
}
/// $03878C: bra.w $387b4
pub fn i_03878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38790;
    cpu.ir = 0x6000;
    cpu.pc = 0x387B4;
    10
}
/// $038790: move.b #$30, $70(a6)
pub fn i_038790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38796;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038796: lea.l $279d06.l, a0
pub fn i_038796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3879C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D06;
    20
}
/// $03879C: move.l -$4(a0), $74(a6)
pub fn i_03879c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387A2;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $0387A2: move.b #$ff, $21(a6)
pub fn i_0387a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387A8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0387A8: lea.l $279d06.l, a0
pub fn i_0387a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387AE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D06;
    20
}
/// $0387AE: jsr $28cd4.l
pub fn i_0387ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387B4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x387B4);
    cpu.pc = 0x28CD4;
    16
}
/// $0387B4: lea.l $32734(pc), a0
pub fn i_0387b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387B8;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $0387B8: move.l a0, $48(a6)
pub fn i_0387b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387BC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0387BC: lea.l $387c2(pc), a1
pub fn i_0387bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x387C2;
    16
}
/// $0387C0: move.l a1, (a6)
pub fn i_0387c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0387C2: bset.b #$6, $8d(a6)
pub fn i_0387c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387C8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0387C8: jsr $32ff2(pc)
pub fn i_0387c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x387CC);
    cpu.pc = 0x32FF2;
    16
}
/// $0387CC: jsr $32d32(pc)
pub fn i_0387cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x387D0);
    cpu.pc = 0x32D32;
    16
}
/// $0387D0: jsr $27a92.l
pub fn i_0387d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x387D6);
    cpu.pc = 0x27A92;
    16
}
/// $0387D6: jsr $32aa8(pc)
pub fn i_0387d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387DA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x387DA);
    cpu.pc = 0x32AA8;
    16
}
/// $0387DA: bcc.w $387e4
pub fn i_0387da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387DE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x387E4; return 10; }
    8
}
/// $0387DE: lea.l $381a2(pc), a1
pub fn i_0387de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x381A2;
    16
}
/// $0387E2: move.l a1, (a6)
pub fn i_0387e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0387E4: btst.b #$2, $8c(a6)
pub fn i_0387e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387EA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0387EA: bne.w $38882
pub fn i_0387ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38882; return 10; }
    8
}
/// $0387EE: btst.b #$0, $3a(a6)
pub fn i_0387ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387F4;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0387F4: bne.w $3880a
pub fn i_0387f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387F8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3880A; return 10; }
    8
}
/// $0387F8: jsr $32e42(pc)
pub fn i_0387f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x387FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x387FC);
    cpu.pc = 0x32E42;
    16
}
/// $0387FC: bcc.w $38806
pub fn i_0387fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38800;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38806; return 10; }
    8
}
/// $038800: lea.l $386e2(pc), a1
pub fn i_038800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38804;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x386E2;
    16
}
/// $038804: move.l a1, (a6)
pub fn i_038804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38806;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038806: bra.w $38818
pub fn i_038806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3880A;
    cpu.ir = 0x6000;
    cpu.pc = 0x38818;
    10
}
/// $03880A: jsr $32e3c(pc)
pub fn i_03880a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3880E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3880E);
    cpu.pc = 0x32E3C;
    16
}
/// $03880E: bcc.w $38818
pub fn i_03880e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38812;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38818; return 10; }
    8
}
/// $038812: lea.l $38688(pc), a1
pub fn i_038812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38816;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38688;
    16
}
/// $038816: move.l a1, (a6)
pub fn i_038816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38818;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038818: jsr $330d0(pc)
pub fn i_038818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3881C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3881C);
    cpu.pc = 0x330D0;
    16
}
/// $03881C: bcc.w $3884e
pub fn i_03881c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38820;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3884E; return 10; }
    8
}
/// $038820: cmpi.b #$ff, d1
pub fn i_038820(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38824;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $038824: bne.w $38832
pub fn i_038824(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38828;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38832; return 10; }
    8
}
/// $038828: lea.l $38a28(pc), a1
pub fn i_038828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3882C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $03882C: move.l a1, (a6)
pub fn i_03882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3882E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03882E: bra.w $3884e
pub fn i_03882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38832;
    cpu.ir = 0x6000;
    cpu.pc = 0x3884E;
    10
}
/// $038832: cmpi.b #$3, d1
pub fn i_038832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38836;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $038836: bne.w $38844
pub fn i_038836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3883A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38844; return 10; }
    8
}
/// $03883A: lea.l $388f0(pc), a1
pub fn i_03883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3883E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $03883E: move.l a1, (a6)
pub fn i_03883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38840;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038840: bra.w $3884e
pub fn i_038840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38844;
    cpu.ir = 0x6000;
    cpu.pc = 0x3884E;
    10
}
/// $038844: lea.l $3873c(pc), a1
pub fn i_038844(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38848;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $038848: move.l a1, (a6)
pub fn i_038848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3884A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03884A: bra.w $3884e
pub fn i_03884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3884E;
    cpu.ir = 0x6000;
    cpu.pc = 0x3884E;
    10
}
/// $03884E: cmpi.b #$0, $85(a6)
pub fn i_03884e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38854;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $038854: bne.w $38868
pub fn i_038854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38858;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38868; return 10; }
    8
}
/// $038858: cmpi.b #$1, $71(a6)
pub fn i_038858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3885E;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $03885E: bne.w $38868
pub fn i_03885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38862;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38868; return 10; }
    8
}
/// $038862: lea.l $38ae6(pc), a1
pub fn i_038862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38866;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38AE6;
    16
}
/// $038866: move.l a1, (a6)
pub fn i_038866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38868;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038868: cmpi.w #$0, $82(a6)
pub fn i_038868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3886E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $03886E: bne.w $38882
pub fn i_03886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38872;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38882; return 10; }
    8
}
/// $038872: cmpi.b #$0, $71(a6)
pub fn i_038872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38878;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $038878: beq.w $38882
pub fn i_038878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3887C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38882; return 10; }
    8
}
/// $03887C: lea.l $38086(pc), a1
pub fn i_03887c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38880;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38086;
    16
}
/// $038880: move.l a1, (a6)
pub fn i_038880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38882;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038882: jsr $5cef8.l
pub fn i_038882(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38888;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38888);
    cpu.pc = 0x5CEF8;
    16
}
/// $038888: bcs.w $38892
pub fn i_038888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3888C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38892; return 10; }
    8
}
/// $03888C: lea.l $37ec2(pc), a1
pub fn i_03888c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38890;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $038890: move.l a1, (a6)
pub fn i_038890(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38892;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038892: jsr $33034(pc)
pub fn i_038892(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38896;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38896);
    cpu.pc = 0x33034;
    16
}
/// $038896: bcc.w $388a0
pub fn i_038896(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3889A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x388A0; return 10; }
    8
}
/// $03889A: lea.l $36914(pc), a1
pub fn i_03889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3889E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $03889E: move.l a1, (a6)
pub fn i_03889e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388A0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0388A0: jsr $27eba.l
pub fn i_0388a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x388A6);
    cpu.pc = 0x27EBA;
    16
}
/// $0388A6: bcc.w $388b0
pub fn i_0388a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388AA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x388B0; return 10; }
    8
}
/// $0388AA: lea.l $36fc2(pc), a1
pub fn i_0388aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388AE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FC2;
    16
}
/// $0388AE: move.l a1, (a6)
pub fn i_0388ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388B0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0388B0: jsr $33522(pc)
pub fn i_0388b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388B4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x388B4);
    cpu.pc = 0x33522;
    16
}
/// $0388B4: cmpi.b #$3, $106ece.l
pub fn i_0388b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388BC;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $0388BC: beq.w $388d4
pub fn i_0388bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388C0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x388D4; return 10; }
    8
}
/// $0388C0: movea.l #$ffffffff, a0
pub fn i_0388c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388C6;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0388C6: lea.l $324c6(pc), a0
pub fn i_0388c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388CA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324C6;
    16
}
/// $0388CA: jsr $5dd56.l
pub fn i_0388ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x388D0);
    cpu.pc = 0x5DD56;
    16
}
/// $0388D0: bra.w $388e4
pub fn i_0388d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x388E4;
    10
}
/// $0388D4: movea.l #$ffffffff, a0
pub fn i_0388d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388DA;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $0388DA: lea.l $324bc(pc), a0
pub fn i_0388da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388DE;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x324BC;
    16
}
/// $0388DE: jsr $5dd56.l
pub fn i_0388de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x388E4);
    cpu.pc = 0x5DD56;
    16
}
/// $0388E4: bcc.w $388ee
pub fn i_0388e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x388EE; return 10; }
    8
}
/// $0388E8: lea.l $37b8e(pc), a1
pub fn i_0388e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37B8E;
    16
}
/// $0388EC: move.l a1, (a6)
pub fn i_0388ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0388EE: rts
pub fn i_0388ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchThrowGrenade_0388f0  ($0388F0, 306 B, player/arm_weapon_fx_0388f0.s)
/// $0388F0: bset.b #$1, $8c(a6)
pub fn i_0388f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388F6;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0388F6: bclr.b #$1, $8c(a6)
pub fn i_0388f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x388FC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0388FC: move.l #$32598, $60(a6)
pub fn i_0388fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38904;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32598;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038904: jsr $267e6.l
pub fn i_038904(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3890A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3890A);
    cpu.pc = 0x267E6;
    16
}
/// $03890A: clr.w $28(a6)
pub fn i_03890a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3890E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $03890E: clr.w $2c(a6)
pub fn i_03890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38912;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $038912: jsr $2abcc.l
pub fn i_038912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38918;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38918);
    cpu.pc = 0x2ABCC;
    16
}
/// $038918: bcs.w $38944
pub fn i_038918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3891C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38944; return 10; }
    8
}
/// $03891C: move.b #$30, $70(a6)
pub fn i_03891c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38922;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038922: lea.l $279d1a.l, a0
pub fn i_038922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38928;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D1A;
    20
}
/// $038928: move.l -$4(a0), $74(a6)
pub fn i_038928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3892E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $03892E: move.b #$ff, $21(a6)
pub fn i_03892e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38934;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038934: lea.l $279d1a.l, a0
pub fn i_038934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3893A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D1A;
    20
}
/// $03893A: jsr $28cd4.l
pub fn i_03893a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38940;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38940);
    cpu.pc = 0x28CD4;
    16
}
/// $038940: bra.w $38968
pub fn i_038940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38944;
    cpu.ir = 0x6000;
    cpu.pc = 0x38968;
    10
}
/// $038944: move.b #$30, $70(a6)
pub fn i_038944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3894A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03894A: lea.l $279d1a.l, a0
pub fn i_03894a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38950;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D1A;
    20
}
/// $038950: move.l -$4(a0), $74(a6)
pub fn i_038950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38956;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038956: move.b #$ff, $21(a6)
pub fn i_038956(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3895C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03895C: lea.l $279d1a.l, a0
pub fn i_03895c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38962;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279D1A;
    20
}
/// $038962: jsr $28cd4.l
pub fn i_038962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38968;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38968);
    cpu.pc = 0x28CD4;
    16
}
/// $038968: lea.l $32734(pc), a0
pub fn i_038968(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3896C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $03896C: move.l a0, $48(a6)
pub fn i_03896c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38970;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038970: lea.l $38976(pc), a1
pub fn i_038970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38974;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38976;
    16
}
/// $038974: move.l a1, (a6)
pub fn i_038974(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38976;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038976: bset.b #$6, $8d(a6)
pub fn i_038976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3897C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03897C: jsr $32ff2(pc)
pub fn i_03897c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38980;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38980);
    cpu.pc = 0x32FF2;
    16
}
/// $038980: jsr $32d32(pc)
pub fn i_038980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38984;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38984);
    cpu.pc = 0x32D32;
    16
}
/// $038984: jsr $27a92.l
pub fn i_038984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3898A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3898A);
    cpu.pc = 0x27A92;
    16
}
/// $03898A: jsr $32aa8(pc)
pub fn i_03898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3898E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3898E);
    cpu.pc = 0x32AA8;
    16
}
/// $03898E: bcc.w $38998
pub fn i_03898e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38992;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38998; return 10; }
    8
}
/// $038992: lea.l $3820e(pc), a1
pub fn i_038992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38996;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3820E;
    16
}
/// $038996: move.l a1, (a6)
pub fn i_038996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38998;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038998: btst.b #$2, $8c(a6)
pub fn i_038998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3899E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03899E: bne.w $38a02
pub fn i_03899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38A02; return 10; }
    8
}
/// $0389A2: btst.b #$0, $3a(a6)
pub fn i_0389a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389A8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0389A8: bne.w $389be
pub fn i_0389a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389AC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x389BE; return 10; }
    8
}
/// $0389AC: jsr $32e42(pc)
pub fn i_0389ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x389B0);
    cpu.pc = 0x32E42;
    16
}
/// $0389B0: bcc.w $389ba
pub fn i_0389b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x389BA; return 10; }
    8
}
/// $0389B4: lea.l $386e2(pc), a1
pub fn i_0389b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389B8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x386E2;
    16
}
/// $0389B8: move.l a1, (a6)
pub fn i_0389b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389BA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0389BA: bra.w $389cc
pub fn i_0389ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389BE;
    cpu.ir = 0x6000;
    cpu.pc = 0x389CC;
    10
}
/// $0389BE: jsr $32e3c(pc)
pub fn i_0389be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x389C2);
    cpu.pc = 0x32E3C;
    16
}
/// $0389C2: bcc.w $389cc
pub fn i_0389c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389C6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x389CC; return 10; }
    8
}
/// $0389C6: lea.l $38688(pc), a1
pub fn i_0389c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38688;
    16
}
/// $0389CA: move.l a1, (a6)
pub fn i_0389ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0389CC: jsr $330d0(pc)
pub fn i_0389cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x389D0);
    cpu.pc = 0x330D0;
    16
}
/// $0389D0: bcc.w $38a02
pub fn i_0389d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38A02; return 10; }
    8
}
/// $0389D4: cmpi.b #$ff, d1
pub fn i_0389d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389D8;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0389D8: bne.w $389e6
pub fn i_0389d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389DC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x389E6; return 10; }
    8
}
/// $0389DC: lea.l $38a28(pc), a1
pub fn i_0389dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389E0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $0389E0: move.l a1, (a6)
pub fn i_0389e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389E2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0389E2: bra.w $38a02
pub fn i_0389e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x38A02;
    10
}
/// $0389E6: cmpi.b #$3, d1
pub fn i_0389e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389EA;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0389EA: bne.w $389f8
pub fn i_0389ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389EE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x389F8; return 10; }
    8
}
/// $0389EE: lea.l $388f0(pc), a1
pub fn i_0389ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $0389F2: move.l a1, (a6)
pub fn i_0389f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0389F4: bra.w $38a02
pub fn i_0389f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389F8;
    cpu.ir = 0x6000;
    cpu.pc = 0x38A02;
    10
}
/// $0389F8: lea.l $3873c(pc), a1
pub fn i_0389f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $0389FC: move.l a1, (a6)
pub fn i_0389fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x389FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0389FE: bra.w $38a02
pub fn i_0389fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A02;
    cpu.ir = 0x6000;
    cpu.pc = 0x38A02;
    10
}
/// $038A02: jsr $5cef8.l
pub fn i_038a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38A08);
    cpu.pc = 0x5CEF8;
    16
}
/// $038A08: bcs.w $38a12
pub fn i_038a08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A0C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38A12; return 10; }
    8
}
/// $038A0C: lea.l $37ec2(pc), a1
pub fn i_038a0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A10;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $038A10: move.l a1, (a6)
pub fn i_038a10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A12;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038A12: jsr $27eba.l
pub fn i_038a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A18;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38A18);
    cpu.pc = 0x27EBA;
    16
}
/// $038A18: bcc.w $38a22
pub fn i_038a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A1C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38A22; return 10; }
    8
}
/// $038A1C: lea.l $36fc2(pc), a1
pub fn i_038a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A20;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FC2;
    16
}
/// $038A20: move.l a1, (a6)
pub fn i_038a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A22;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_038a22  ($038A22, 6 B, thunks/jsr_pc_thunks.c)
/// $038A22: jsr $33522(pc)
pub fn i_038a22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A26;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38A26);
    cpu.pc = 0x33522;
    16
}
/// $038A26: rts
pub fn i_038a26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchMelee_038a28  ($038A28, 184 B, player/arm_weapon_fx_0388f0.s)
/// $038A28: bset.b #$1, $8c(a6)
pub fn i_038a28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A2E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038A2E: bclr.b #$1, $8c(a6)
pub fn i_038a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A34;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038A34: lea.l $32638(pc), a0
pub fn i_038a34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A38;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32638;
    16
}
/// $038A38: move.l a0, $4c(a6)
pub fn i_038a38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A3C;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038A3C: jsr $283ca.l
pub fn i_038a3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38A42);
    cpu.pc = 0x283CA;
    16
}
/// $038A42: clr.w $28(a6)
pub fn i_038a42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A46;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $038A46: clr.w $2c(a6)
pub fn i_038a46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A4A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $038A4A: clr.w $2a(a6)
pub fn i_038a4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A4E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $038A4E: clr.w $2e(a6)
pub fn i_038a4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A52;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $038A52: move.l #$32500, $60(a6)
pub fn i_038a52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A5A;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038A5A: jsr $2abcc.l
pub fn i_038a5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A60;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38A60);
    cpu.pc = 0x2ABCC;
    16
}
/// $038A60: bcs.w $38a8c
pub fn i_038a60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A64;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38A8C; return 10; }
    8
}
/// $038A64: move.b #$0, $70(a6)
pub fn i_038a64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A6A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038A6A: lea.l $279eea.l, a0
pub fn i_038a6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A70;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EEA;
    20
}
/// $038A70: move.l -$4(a0), $74(a6)
pub fn i_038a70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A76;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038A76: move.b #$ff, $21(a6)
pub fn i_038a76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A7C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038A7C: lea.l $279eea.l, a0
pub fn i_038a7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A82;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EEA;
    20
}
/// $038A82: jsr $28cd4.l
pub fn i_038a82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A88;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38A88);
    cpu.pc = 0x28CD4;
    16
}
/// $038A88: bra.w $38ab0
pub fn i_038a88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A8C;
    cpu.ir = 0x6000;
    cpu.pc = 0x38AB0;
    10
}
/// $038A8C: move.b #$0, $70(a6)
pub fn i_038a8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A92;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038A92: lea.l $279eea.l, a0
pub fn i_038a92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A98;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EEA;
    20
}
/// $038A98: move.l -$4(a0), $74(a6)
pub fn i_038a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38A9E;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038A9E: move.b #$ff, $21(a6)
pub fn i_038a9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AA4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038AA4: lea.l $279eea.l, a0
pub fn i_038aa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AAA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279EEA;
    20
}
/// $038AAA: jsr $28cd4.l
pub fn i_038aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AB0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38AB0);
    cpu.pc = 0x28CD4;
    16
}
/// $038AB0: lea.l $327dc(pc), a0
pub fn i_038ab0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AB4;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x327DC;
    16
}
/// $038AB4: move.l a0, $48(a6)
pub fn i_038ab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AB8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038AB8: lea.l $38abe(pc), a1
pub fn i_038ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38ABC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38ABE;
    16
}
/// $038ABC: move.l a1, (a6)
pub fn i_038abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38ABE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038ABE: bset.b #$6, $8d(a6)
pub fn i_038abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AC4;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038AC4: jsr $32ff2(pc)
pub fn i_038ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AC8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38AC8);
    cpu.pc = 0x32FF2;
    16
}
/// $038AC8: jsr $32d32(pc)
pub fn i_038ac8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38ACC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38ACC);
    cpu.pc = 0x32D32;
    16
}
/// $038ACC: jsr $27a92.l
pub fn i_038acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AD2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38AD2);
    cpu.pc = 0x27A92;
    16
}
/// $038AD2: jsr $32aa8(pc)
pub fn i_038ad2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AD6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38AD6);
    cpu.pc = 0x32AA8;
    16
}
/// $038AD6: bcc.w $38ae0
pub fn i_038ad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38ADA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38AE0; return 10; }
    8
}
/// $038ADA: lea.l $3820e(pc), a1
pub fn i_038ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38ADE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3820E;
    16
}
/// $038ADE: move.l a1, (a6)
pub fn i_038ade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AE0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_038ae0  ($038AE0, 6 B, thunks/jsr_pc_thunks.c)
/// $038AE0: jsr $33522(pc)
pub fn i_038ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AE4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38AE4);
    cpu.pc = 0x33522;
    16
}
/// $038AE4: rts
pub fn i_038ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AE6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_CrouchReload_038ae6  ($038AE6, 248 B, player/arm_weapon_fx_0388f0.s)
/// $038AE6: move.w #$0, $7c(a6)
pub fn i_038ae6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AEC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038AEC: move.w #$0, $7e(a6)
pub fn i_038aec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AF2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038AF2: bset.b #$2, $8c(a6)
pub fn i_038af2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AF8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038AF8: bclr.b #$1, $8c(a6)
pub fn i_038af8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38AFE;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038AFE: bclr.b #$3, $8c(a6)
pub fn i_038afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B04;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038B04: move.b #$33, $70(a6)
pub fn i_038b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B0A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x33;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038B0A: lea.l $27970c.l, a0
pub fn i_038b0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27970C;
    20
}
/// $038B10: move.l -$4(a0), $74(a6)
pub fn i_038b10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B16;
    cpu.ir = 0x2D68;
    let ad2 = cpu.a[0].wrapping_add(0xFFFFFFFC);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $038B16: move.b #$ff, $21(a6)
pub fn i_038b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B1C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038B1C: lea.l $27970c.l, a0
pub fn i_038b1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27970C;
    20
}
/// $038B22: jsr $28cd4.l
pub fn i_038b22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38B28);
    cpu.pc = 0x28CD4;
    16
}
/// $038B28: lea.l $32734(pc), a0
pub fn i_038b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B2C;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x32734;
    16
}
/// $038B2C: move.l a0, $48(a6)
pub fn i_038b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B30;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038B30: move.l #$32500, $60(a6)
pub fn i_038b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B38;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x32500;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $038B38: jsr $267e6.l
pub fn i_038b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38B3E);
    cpu.pc = 0x267E6;
    16
}
/// $038B3E: clr.w $28(a6)
pub fn i_038b3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B42;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x28);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $038B42: lea.l $38b48(pc), a1
pub fn i_038b42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38B48;
    16
}
/// $038B46: move.l a1, (a6)
pub fn i_038b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038B48: bset.b #$6, $8d(a6)
pub fn i_038b48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B4E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038B4E: jsr $32ff2(pc)
pub fn i_038b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B52;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38B52);
    cpu.pc = 0x32FF2;
    16
}
/// $038B52: jsr $32d32(pc)
pub fn i_038b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B56;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38B56);
    cpu.pc = 0x32D32;
    16
}
/// $038B56: jsr $27a92.l
pub fn i_038b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38B5C);
    cpu.pc = 0x27A92;
    16
}
/// $038B5C: jsr $32aa8(pc)
pub fn i_038b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B60;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38B60);
    cpu.pc = 0x32AA8;
    16
}
/// $038B60: bcc.w $38b6a
pub fn i_038b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B64;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38B6A; return 10; }
    8
}
/// $038B64: lea.l $381a2(pc), a1
pub fn i_038b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x381A2;
    16
}
/// $038B68: move.l a1, (a6)
pub fn i_038b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038B6A: btst.b #$2, $8c(a6)
pub fn i_038b6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B70;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $038B70: bne.w $38b7a
pub fn i_038b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B74;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38B7A; return 10; }
    8
}
/// $038B74: move.b #$1, $85(a6)
pub fn i_038b74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B7A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038B7A: jsr $5cef8.l
pub fn i_038b7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38B80);
    cpu.pc = 0x5CEF8;
    16
}
/// $038B80: bcs.w $38b8a
pub fn i_038b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B84;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38B8A; return 10; }
    8
}
/// $038B84: lea.l $37ec2(pc), a1
pub fn i_038b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x37EC2;
    16
}
/// $038B88: move.l a1, (a6)
pub fn i_038b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038B8A: jsr $330d0(pc)
pub fn i_038b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38B8E);
    cpu.pc = 0x330D0;
    16
}
/// $038B8E: bcc.w $38bc0
pub fn i_038b8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38BC0; return 10; }
    8
}
/// $038B92: cmpi.b #$ff, d1
pub fn i_038b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B96;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $038B96: bne.w $38ba4
pub fn i_038b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B9A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38BA4; return 10; }
    8
}
/// $038B9A: lea.l $38a28(pc), a1
pub fn i_038b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38B9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38A28;
    16
}
/// $038B9E: move.l a1, (a6)
pub fn i_038b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038BA0: bra.w $38bc0
pub fn i_038ba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BA4;
    cpu.ir = 0x6000;
    cpu.pc = 0x38BC0;
    10
}
/// $038BA4: cmpi.b #$3, d1
pub fn i_038ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BA8;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $038BA8: bne.w $38bb6
pub fn i_038ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BAC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38BB6; return 10; }
    8
}
/// $038BAC: lea.l $388f0(pc), a1
pub fn i_038bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BB0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x388F0;
    16
}
/// $038BB0: move.l a1, (a6)
pub fn i_038bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BB2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038BB2: bra.w $38bc0
pub fn i_038bb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x38BC0;
    10
}
/// $038BB6: lea.l $3873c(pc), a1
pub fn i_038bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3873C;
    16
}
/// $038BBA: move.l a1, (a6)
pub fn i_038bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038BBC: bra.w $38bc0
pub fn i_038bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BC0;
    cpu.ir = 0x6000;
    cpu.pc = 0x38BC0;
    10
}
/// $038BC0: jsr $33034(pc)
pub fn i_038bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BC4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38BC4);
    cpu.pc = 0x33034;
    16
}
/// $038BC4: bcc.w $38bce
pub fn i_038bc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BC8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38BCE; return 10; }
    8
}
/// $038BC8: lea.l $36914(pc), a1
pub fn i_038bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36914;
    16
}
/// $038BCC: move.l a1, (a6)
pub fn i_038bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038BCE: jsr $27eba.l
pub fn i_038bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38BD4);
    cpu.pc = 0x27EBA;
    16
}
/// $038BD4: bcc.w $38bde
pub fn i_038bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BD8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38BDE; return 10; }
    8
}
/// $038BD8: lea.l $36fc2(pc), a1
pub fn i_038bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BDC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x36FC2;
    16
}
/// $038BDC: move.l a1, (a6)
pub fn i_038bdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BDE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_038bde  ($038BDE, 6 B, thunks/jsr_pc_thunks.c)
/// $038BDE: jsr $33522(pc)
pub fn i_038bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BE2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38BE2);
    cpu.pc = 0x33522;
    16
}
/// $038BE2: rts
pub fn i_038be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DroppedWeapon_Spawn_038be4  ($038BE4, 140 B, player/arm_weapon_fx_0388f0.s)
/// $038BE4: movea.l $c(a6), a0
pub fn i_038be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BE8;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038BE8: cmpi.b #$2, $6d(a0)
pub fn i_038be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BEE;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x6D);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $038BEE: bne.w $38bfa
pub fn i_038bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BF2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38BFA; return 10; }
    8
}
/// $038BF2: move.w #$176, d1
pub fn i_038bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BF6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x176;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038BF6: bra.w $38bfe
pub fn i_038bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x38BFE;
    10
}
/// $038BFA: move.w #$177, d1
pub fn i_038bfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38BFE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x177;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038BFE: jsr $236e.l
pub fn i_038bfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38C04);
    cpu.pc = 0x236E;
    16
}
/// $038C04: move.w #$fef0, d0
pub fn i_038c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C08;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFEF0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038C08: jsr $5dca4.l
pub fn i_038c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38C0E);
    cpu.pc = 0x5DCA4;
    16
}
/// $038C0E: move.w d0, $28(a6)
pub fn i_038c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C12;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038C12: move.w #$50c, $2a(a6)
pub fn i_038c12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C18;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x50C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038C18: move.w #$ffc9, $2e(a6)
pub fn i_038c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038C1E: move.w #$0, $2c(a6)
pub fn i_038c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C24;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038C24: lea.l $27a28e.l, a0
pub fn i_038c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C2A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27A28E;
    20
}
/// $038C2A: jsr $28cd4.l
pub fn i_038c2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C30;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38C30);
    cpu.pc = 0x28CD4;
    16
}
/// $038C30: andi.w #$ffe3, $38(a6)
pub fn i_038c30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C36;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $038C36: ori.w #$18, $38(a6)
pub fn i_038c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C3C;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x18) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $038C3C: lea.l $38c42(pc), a1
pub fn i_038c3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C40;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38C42;
    16
}
/// $038C40: move.l a1, (a6)
pub fn i_038c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C42;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038C42: jsr $27bc8.l
pub fn i_038c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38C48);
    cpu.pc = 0x27BC8;
    16
}
/// $038C48: bcc.w $38c52
pub fn i_038c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38C52; return 10; }
    8
}
/// $038C4C: lea.l $38c70(pc), a1
pub fn i_038c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38C70;
    16
}
/// $038C50: move.l a1, (a6)
pub fn i_038c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038C52: jsr $28d70.l
pub fn i_038c52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38C58);
    cpu.pc = 0x28D70;
    16
}
/// $038C58: movea.l #$ffffffff, a0
pub fn i_038c58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C5E;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $038C5E: jsr $5dd56.l
pub fn i_038c5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38C64);
    cpu.pc = 0x5DD56;
    16
}
/// $038C64: bcc.w $38c6e
pub fn i_038c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C68;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38C6E; return 10; }
    8
}
/// $038C68: lea.l $38cee(pc), a1
pub fn i_038c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38CEE;
    16
}
/// $038C6C: move.l a1, (a6)
pub fn i_038c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038C6E: rts
pub fn i_038c6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C70;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DroppedWeapon_Bounce_038c70  ($038C70, 126 B, player/arm_weapon_fx_0388f0.s)
/// $038C70: jsr $77d88.l
pub fn i_038c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38C76);
    cpu.pc = 0x77D88;
    16
}
/// $038C76: move.w $22(a6), $22(a0)
pub fn i_038c76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C7C;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $038C7C: move.w $24(a6), $24(a0)
pub fn i_038c7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C82;
    cpu.ir = 0x316E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $038C82: move.w #$ff67, d0
pub fn i_038c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C86;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF67;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038C86: jsr $5dca4.l
pub fn i_038c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C8C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38C8C);
    cpu.pc = 0x5DCA4;
    16
}
/// $038C8C: move.w d0, $28(a6)
pub fn i_038c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C90;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038C90: move.w #$3fc, $2a(a6)
pub fn i_038c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C96;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3FC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038C96: move.w #$ffbc, $2e(a6)
pub fn i_038c96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38C9C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFBC;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038C9C: move.w #$0, $2c(a6)
pub fn i_038c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CA2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $038CA2: move.b #$28, $45(a6)
pub fn i_038ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CA8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x45);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038CA8: move.b #$28, $44(a6)
pub fn i_038ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CAE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038CAE: lea.l $38cb4(pc), a1
pub fn i_038cae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38CB4;
    16
}
/// $038CB2: move.l a1, (a6)
pub fn i_038cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038CB4: eori.b #$8, $12(a6)
pub fn i_038cb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CBA;
    cpu.ir = 0xA2E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ 0x8) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038CBA: andi.b #$fe, $46(a6)
pub fn i_038cba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CC0;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFE) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $038CC0: jsr $27bc8.l
pub fn i_038cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38CC6);
    cpu.pc = 0x27BC8;
    16
}
/// $038CC6: bcc.w $38cd0
pub fn i_038cc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38CD0; return 10; }
    8
}
/// $038CCA: lea.l $38cee(pc), a1
pub fn i_038cca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38CEE;
    16
}
/// $038CCE: move.l a1, (a6)
pub fn i_038cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038CD0: jsr $28d70.l
pub fn i_038cd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CD6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38CD6);
    cpu.pc = 0x28D70;
    16
}
/// $038CD6: movea.l #$ffffffff, a0
pub fn i_038cd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CDC;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $038CDC: jsr $5dd56.l
pub fn i_038cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CE2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38CE2);
    cpu.pc = 0x5DD56;
    16
}
/// $038CE2: bcc.w $38cec
pub fn i_038ce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CE6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38CEC; return 10; }
    8
}
/// $038CE6: lea.l $38cee(pc), a1
pub fn i_038ce6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CEA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38CEE;
    16
}
/// $038CEA: move.l a1, (a6)
pub fn i_038cea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CEC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038CEC: rts
pub fn i_038cec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CEE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DroppedWeapon_Free_038cee  ($038CEE, 6 B, player/arm_weapon_fx_0388f0.s)
/// $038CEE: jmp $518.l
pub fn i_038cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CF4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== DroppedWeapon_Rts_038cf4  ($038CF4, 2 B, player/arm_weapon_fx_0388f0.s)
/// $038CF4: rts
pub fn i_038cf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CF6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parachute_Spawn_038cf6  ($038CF6, 18 B, player/arm_weapon_fx_0388f0.s)
/// $038CF6: move.w #$18d, d1
pub fn i_038cf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38CFA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x18D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038CFA: jsr $236e.l
pub fn i_038cfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D00);
    cpu.pc = 0x236E;
    16
}
/// $038D00: lea.l $ffff.w, a0
pub fn i_038d00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D04;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $038D04: move.l a0, $48(a6)
pub fn i_038d04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D08;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== Parachute_Open_038d08  ($038D08, 128 B, player/arm_weapon_fx_0388f0.s)
/// $038D08: lea.l $38d0e(pc), a1
pub fn i_038d08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38D0E;
    16
}
/// $038D0C: move.l a1, (a6)
pub fn i_038d0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D0E: jsr $5e4ee.l
pub fn i_038d0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D14;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D14);
    cpu.pc = 0x5E4EE;
    16
}
/// $038D14: subi.w #$20, $38(a6)
pub fn i_038d14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D1A;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x20, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $038D1A: clr.l d0
pub fn i_038d1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D1C;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $038D1C: jsr $38e9a(pc)
pub fn i_038d1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D20;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38D20);
    cpu.pc = 0x38E9A;
    16
}
/// $038D20: movea.l #$28f8d4, a0
pub fn i_038d20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D26;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28F8D4;
    cpu.a[0] = v1;
    4
}
/// $038D26: lsl.w #$2, d0
pub fn i_038d26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D28;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $038D28: movea.l (a0, d0.w), a0
pub fn i_038d28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D2C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $038D2C: cmpa.l #$ffffffff, a0
pub fn i_038d2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D32;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $038D32: beq.w $38d3c
pub fn i_038d32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D36;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38D3C; return 10; }
    8
}
/// $038D36: jsr $28cd4.l
pub fn i_038d36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D3C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D3C);
    cpu.pc = 0x28CD4;
    16
}
/// $038D3C: jsr $28d70.l
pub fn i_038d3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D42);
    cpu.pc = 0x28D70;
    16
}
/// $038D42: jsr $28758.l
pub fn i_038d42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38D48);
    cpu.pc = 0x28758;
    16
}
/// $038D48: bcc.w $38d52
pub fn i_038d48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D4C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38D52; return 10; }
    8
}
/// $038D4C: lea.l $38e4a(pc), a1
pub fn i_038d4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D50;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E4A;
    16
}
/// $038D50: move.l a1, (a6)
pub fn i_038d50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D52;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D52: movea.l $c(a6), a0
pub fn i_038d52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D56;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038D56: btst.b #$2, $8c(a0)
pub fn i_038d56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D5C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $038D5C: beq.w $38d66
pub fn i_038d5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38D66; return 10; }
    8
}
/// $038D60: lea.l $38d88(pc), a1
pub fn i_038d60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D64;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38D88;
    16
}
/// $038D64: move.l a1, (a6)
pub fn i_038d64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D66;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D66: cmpi.b #$4, $70(a0)
pub fn i_038d66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D6C;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $038D6C: beq.w $38d76
pub fn i_038d6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D70;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38D76; return 10; }
    8
}
/// $038D70: lea.l $38e10(pc), a1
pub fn i_038d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E10;
    16
}
/// $038D74: move.l a1, (a6)
pub fn i_038d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D76: btst.b #$0, $13(a0)
pub fn i_038d76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D7C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $038D7C: beq.w $38d86
pub fn i_038d7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D80;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38D86; return 10; }
    8
}
/// $038D80: lea.l $38e4a(pc), a1
pub fn i_038d80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D84;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E4A;
    16
}
/// $038D84: move.l a1, (a6)
pub fn i_038d84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D86;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038D86: rts
pub fn i_038d86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D88;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parachute_Swing_038d88  ($038D88, 136 B, player/arm_weapon_fx_0388f0.s)
/// $038D88: clr.l d0
pub fn i_038d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D8A;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $038D8A: jsr $38e9a(pc)
pub fn i_038d8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D8E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x38D8E);
    cpu.pc = 0x38E9A;
    16
}
/// $038D8E: movea.l #$28fdbc, a0
pub fn i_038d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D94;
    cpu.ir = 0x207C;
    let v1: u32 = 0x28FDBC;
    cpu.a[0] = v1;
    4
}
/// $038D94: lsl.w #$2, d0
pub fn i_038d94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D96;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $038D96: movea.l (a0, d0.w), a0
pub fn i_038d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38D9A;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $038D9A: cmpa.l #$ffffffff, a0
pub fn i_038d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DA0;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $038DA0: beq.w $38daa
pub fn i_038da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38DAA; return 10; }
    8
}
/// $038DA4: jsr $28cd4.l
pub fn i_038da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38DAA);
    cpu.pc = 0x28CD4;
    16
}
/// $038DAA: lea.l $38db0(pc), a1
pub fn i_038daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38DB0;
    16
}
/// $038DAE: move.l a1, (a6)
pub fn i_038dae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038DB0: jsr $5e4ee.l
pub fn i_038db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38DB6);
    cpu.pc = 0x5E4EE;
    16
}
/// $038DB6: subq.w #$1, $38(a6)
pub fn i_038db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DBA;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $038DBA: jsr $28d70.l
pub fn i_038dba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38DC0);
    cpu.pc = 0x28D70;
    16
}
/// $038DC0: bcc.w $38dca
pub fn i_038dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DC4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38DCA; return 10; }
    8
}
/// $038DC4: lea.l $38d08(pc), a1
pub fn i_038dc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DC8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38D08;
    16
}
/// $038DC8: move.l a1, (a6)
pub fn i_038dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DCA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038DCA: jsr $28758.l
pub fn i_038dca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38DD0);
    cpu.pc = 0x28758;
    16
}
/// $038DD0: bcc.w $38dda
pub fn i_038dd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38DDA; return 10; }
    8
}
/// $038DD4: lea.l $38e4a(pc), a1
pub fn i_038dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DD8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E4A;
    16
}
/// $038DD8: move.l a1, (a6)
pub fn i_038dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DDA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038DDA: movea.l $c(a6), a0
pub fn i_038dda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DDE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038DDE: cmpi.b #$4, $70(a0)
pub fn i_038dde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DE4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $038DE4: bne.w $38dfe
pub fn i_038de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38DFE; return 10; }
    8
}
/// $038DE8: lea.l $38e10(pc), a1
pub fn i_038de8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E10;
    16
}
/// $038DEC: move.l a1, (a6)
pub fn i_038dec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038DEE: cmpi.b #$26, $70(a0)
pub fn i_038dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DF4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x26, Size::Byte, false);
    16
}
/// $038DF4: bne.w $38dfe
pub fn i_038df4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DF8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38DFE; return 10; }
    8
}
/// $038DF8: lea.l $38e4a(pc), a1
pub fn i_038df8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DFC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E4A;
    16
}
/// $038DFC: move.l a1, (a6)
pub fn i_038dfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38DFE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038DFE: btst.b #$0, $13(a0)
pub fn i_038dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E04;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $038E04: beq.w $38e0e
pub fn i_038e04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E08;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38E0E; return 10; }
    8
}
/// $038E08: lea.l $38e4a(pc), a1
pub fn i_038e08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E0C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E4A;
    16
}
/// $038E0C: move.l a1, (a6)
pub fn i_038e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E0E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038E0E: rts
pub fn i_038e0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parachute_Release_038e10  ($038E10, 58 B, player/arm_weapon_fx_0388f0.s)
/// $038E10: movea.l $c(a6), a0
pub fn i_038e10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E14;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038E14: move.b $3a(a0), d0
pub fn i_038e14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E18;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $038E18: eori.b #$1, d0
pub fn i_038e18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E1C;
    cpu.ir = 0xA00;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $038E1C: move.b d0, $3a(a6)
pub fn i_038e1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E20;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038E20: lea.l $28fe2c.l, a0
pub fn i_038e20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E26;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x28FE2C;
    20
}
/// $038E26: jsr $28cd4.l
pub fn i_038e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E2C);
    cpu.pc = 0x28CD4;
    16
}
/// $038E2C: lea.l $38e32(pc), a1
pub fn i_038e2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E30;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E32;
    16
}
/// $038E30: move.l a1, (a6)
pub fn i_038e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E32;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038E32: jsr $5e4ee.l
pub fn i_038e32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E38);
    cpu.pc = 0x5E4EE;
    16
}
/// $038E38: jsr $28d70.l
pub fn i_038e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E3E);
    cpu.pc = 0x28D70;
    16
}
/// $038E3E: bcc.w $38e48
pub fn i_038e3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E42;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38E48; return 10; }
    8
}
/// $038E42: jmp $518.l
pub fn i_038e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E48;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $038E48: rts
pub fn i_038e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E4A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parachute_FallAway_038e4a  ($038E4A, 80 B, player/arm_weapon_fx_0388f0.s)
/// $038E4A: movea.l $c(a6), a0
pub fn i_038e4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E4E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038E4E: move.b $3a(a0), $3a(a6)
pub fn i_038e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E54;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad4, v3 as u8);
    20
}
/// $038E54: move.b #$0, $71(a6)
pub fn i_038e54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E5A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    bus.write8(ad2, v1 as u8);
    12
}
/// $038E5A: lea.l $38ec8(pc), a0
pub fn i_038e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E5E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x38EC8;
    16
}
/// $038E5E: jsr $28cd4.l
pub fn i_038e5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E64);
    cpu.pc = 0x28CD4;
    16
}
/// $038E64: lea.l $38e6a(pc), a1
pub fn i_038e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E68;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38E6A;
    16
}
/// $038E68: move.l a1, (a6)
pub fn i_038e68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E6A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $038E6A: jsr $2783a.l
pub fn i_038e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E70;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E70);
    cpu.pc = 0x2783A;
    16
}
/// $038E70: cmpi.b #$0, $71(a6)
pub fn i_038e70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E76;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $038E76: beq.w $38e88
pub fn i_038e76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E7A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x38E88; return 10; }
    8
}
/// $038E7A: move.b $106f28.l, d0
pub fn i_038e7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E80;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x106F28) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $038E80: btst.b #$0, d0
pub fn i_038e80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E84;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $038E84: bne.w $38e8e
pub fn i_038e84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E88;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x38E8E; return 10; }
    8
}
/// $038E88: jsr $28d70.l
pub fn i_038e88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E8E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38E8E);
    cpu.pc = 0x28D70;
    16
}
/// $038E8E: bcc.w $38e98
pub fn i_038e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E92;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x38E98; return 10; }
    8
}
/// $038E92: jmp $518.l
pub fn i_038e92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E98;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $038E98: rts
pub fn i_038e98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E9A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parachute_SwingIndexFromVelX_038e9a  ($038E9A, 46 B, player/arm_weapon_fx_0388f0.s)
/// $038E9A: movea.l $c(a6), a0
pub fn i_038e9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38E9E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $038E9E: move.w $28(a0), d1
pub fn i_038e9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EA2;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $038EA2: addi.w #$400, d1
pub fn i_038ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EA6;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x400, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $038EA6: move.w #$400, d0
pub fn i_038ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EAA;
    cpu.ir = 0x303C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038EAA: addi.w #$400, d0
pub fn i_038eaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EAE;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x400, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $038EAE: move.w d0, d2
pub fn i_038eae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB0;
    cpu.ir = 0x3400;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $038EB0: asr.w #$5, d2
pub fn i_038eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB2;
    cpu.ir = 0xEA42;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, false);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $038EB2: add.w d2, d1
pub fn i_038eb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB4;
    cpu.ir = 0xD242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $038EB4: add.w d2, d0
pub fn i_038eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB6;
    cpu.ir = 0xD042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $038EB6: sub.w d1, d0
pub fn i_038eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EB8;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $038EB8: lsr.w #$8, d0
pub fn i_038eb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EBA;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $038EBA: cmpi.w #$a, d0
pub fn i_038eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EBE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA, Size::Word, false);
    8
}
/// $038EBE: bcs.w $38ec6
pub fn i_038ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EC2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x38EC6; return 10; }
    8
}
/// $038EC2: andi.w #$7, d0
pub fn i_038ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EC6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $038EC6: rts
pub fn i_038ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38EC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DuckTrigger_SpawnPairRight_038f12  ($038F12, 52 B, player/arm_weapon_fx_0388f0.s)
/// $038F12: move.b d0, d3
pub fn i_038f12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F14;
    cpu.ir = 0x1600;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v1 & 0xFF);
    4
}
/// $038F14: lea.l $39010(pc), a1
pub fn i_038f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39010;
    16
}
/// $038F18: jsr $4ae.l
pub fn i_038f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38F1E);
    cpu.pc = 0x4AE;
    16
}
/// $038F1E: jsr $5dd02.l
pub fn i_038f1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38F24);
    cpu.pc = 0x5DD02;
    16
}
/// $038F24: move.b $98(a6), $98(a0)
pub fn i_038f24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F2A;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $038F2A: lea.l $39050(pc), a1
pub fn i_038f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39050;
    16
}
/// $038F2E: jsr $4ae.l
pub fn i_038f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38F34);
    cpu.pc = 0x4AE;
    16
}
/// $038F34: jsr $5dd02.l
pub fn i_038f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38F3A);
    cpu.pc = 0x5DD02;
    16
}
/// $038F3A: move.b $98(a6), $98(a0)
pub fn i_038f3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F40;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $038F40: jmp $518.l
pub fn i_038f40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F46;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== DuckTrigger_Rts_038f46  ($038F46, 2 B, player/arm_weapon_fx_0388f0.s)
/// $038F46: rts
pub fn i_038f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DuckTrigger_SpawnPairLeft_038f48  ($038F48, 50 B, player/arm_weapon_fx_0388f0.s)
/// $038F48: lea.l $39030(pc), a1
pub fn i_038f48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39030;
    16
}
/// $038F4C: jsr $4ae.l
pub fn i_038f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F52;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38F52);
    cpu.pc = 0x4AE;
    16
}
/// $038F52: jsr $5dd02.l
pub fn i_038f52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F58;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38F58);
    cpu.pc = 0x5DD02;
    16
}
/// $038F58: move.b $98(a6), $98(a0)
pub fn i_038f58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F5E;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $038F5E: lea.l $39070(pc), a1
pub fn i_038f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39070;
    16
}
/// $038F62: jsr $4ae.l
pub fn i_038f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F68;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38F68);
    cpu.pc = 0x4AE;
    16
}
/// $038F68: jsr $5dd02.l
pub fn i_038f68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x38F6E);
    cpu.pc = 0x5DD02;
    16
}
/// $038F6E: move.b $98(a6), $98(a0)
pub fn i_038f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F74;
    cpu.ir = 0x116E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad4, v3 as u8);
    20
}
/// $038F74: jmp $518.l
pub fn i_038f74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F7A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== DuckTrigger_RtsB_038f7a  ($038F7A, 2 B, player/arm_weapon_fx_0388f0.s)
/// $038F7A: rts
pub fn i_038f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x38F7C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== DuckTrigger_P1Right_039010  ($039010, 32 B, player/arm_weapon_fx_0388f0.s)
/// $039010: lea.l $38fbc(pc), a0
pub fn i_039010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39014;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x38FBC;
    16
}
/// $039014: move.l a0, $48(a6)
pub fn i_039014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39018;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $039018: lea.l $100440.l, a0
pub fn i_039018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3901E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $03901E: lea.l $38fb4(pc), a1
pub fn i_03901e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39022;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38FB4;
    16
}
/// $039022: bclr.b #$0, $3a(a6)
pub fn i_039022(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39028;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039028: move.w #$2, d1
pub fn i_039028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3902C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03902C: bra.w $39090
pub fn i_03902c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39030;
    cpu.ir = 0x6000;
    cpu.pc = 0x39090;
    10
}
// ===== DuckTrigger_P1Left_039030  ($039030, 32 B, player/arm_weapon_fx_0388f0.s)
/// $039030: lea.l $38fbc(pc), a0
pub fn i_039030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39034;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x38FBC;
    16
}
/// $039034: move.l a0, $48(a6)
pub fn i_039034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39038;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $039038: lea.l $100440.l, a0
pub fn i_039038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3903E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $03903E: lea.l $38fb4(pc), a1
pub fn i_03903e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39042;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38FB4;
    16
}
/// $039042: bset.b #$0, $3a(a6)
pub fn i_039042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39048;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039048: move.w #$1, d1
pub fn i_039048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3904C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03904C: bra.w $39090
pub fn i_03904c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39050;
    cpu.ir = 0x6000;
    cpu.pc = 0x39090;
    10
}
// ===== DuckTrigger_P2Right_039050  ($039050, 32 B, player/arm_weapon_fx_0388f0.s)
/// $039050: lea.l $38fbc(pc), a0
pub fn i_039050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39054;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x38FBC;
    16
}
/// $039054: move.l a0, $48(a6)
pub fn i_039054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39058;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $039058: lea.l $1004e0.l, a0
pub fn i_039058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3905E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $03905E: lea.l $38fb4(pc), a1
pub fn i_03905e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39062;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38FB4;
    16
}
/// $039062: bclr.b #$0, $3a(a6)
pub fn i_039062(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39068;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039068: move.w #$2, d1
pub fn i_039068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3906C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03906C: bra.w $39090
pub fn i_03906c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39070;
    cpu.ir = 0x6000;
    cpu.pc = 0x39090;
    10
}
// ===== DuckTrigger_P2Left_039070  ($039070, 112 B, player/arm_weapon_fx_0388f0.s)
/// $039070: lea.l $38fbc(pc), a0
pub fn i_039070(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39074;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x38FBC;
    16
}
/// $039074: move.l a0, $48(a6)
pub fn i_039074(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39078;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $039078: lea.l $1004e0.l, a0
pub fn i_039078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3907E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
/// $03907E: lea.l $38fb4(pc), a1
pub fn i_03907e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39082;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x38FB4;
    16
}
/// $039082: bset.b #$0, $3a(a6)
pub fn i_039082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39088;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039088: move.w #$1, d1
pub fn i_039088(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3908C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03908C: bra.w $39090
pub fn i_03908c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39090;
    cpu.ir = 0x6000;
    cpu.pc = 0x39090;
    10
}
/// $039090: move.l a0, $70(a6)
pub fn i_039090(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39094;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $039094: move.l a1, $74(a6)
pub fn i_039094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39098;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $039098: jsr $236e.l
pub fn i_039098(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3909E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3909E);
    cpu.pc = 0x236E;
    16
}
/// $03909E: lea.l $38fa8(pc), a0
pub fn i_03909e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390A2;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x38FA8;
    16
}
/// $0390A2: jsr $28cd4.l
pub fn i_0390a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390A8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x390A8);
    cpu.pc = 0x28CD4;
    16
}
/// $0390A8: jsr $267e2.l
pub fn i_0390a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x390AE);
    cpu.pc = 0x267E2;
    16
}
/// $0390AE: cmpi.b #$1, $98(a6)
pub fn i_0390ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390B4;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $0390B4: bne.w $390d6
pub fn i_0390b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390B8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x390D6; return 10; }
    8
}
/// $0390B8: jsr $5e7c0.l
pub fn i_0390b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x390BE);
    cpu.pc = 0x5E7C0;
    16
}
/// $0390BE: bcs.w $390d6
pub fn i_0390be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390C2;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x390D6; return 10; }
    8
}
/// $0390C2: addq.w #$3, $24(a6)
pub fn i_0390c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390C6;
    cpu.ir = 0x566E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0390C6: move.w #$fc00, $2a(a6)
pub fn i_0390c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0390CC: lea.l $390e0(pc), a1
pub fn i_0390cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390D0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x390E0;
    16
}
/// $0390D0: move.l a1, (a6)
pub fn i_0390d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390D2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0390D2: bra.w $390e0
pub fn i_0390d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390D6;
    cpu.ir = 0x6000;
    cpu.pc = 0x390E0;
    10
}
/// $0390D6: lea.l $390f0(pc), a1
pub fn i_0390d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x390F0;
    16
}
/// $0390DA: move.l a1, (a6)
pub fn i_0390da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0390DC: bra.w $390f0
pub fn i_0390dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390E0;
    cpu.ir = 0x6000;
    cpu.pc = 0x390F0;
    10
}
// ===== DuckTrigger_Fall_0390e0  ($0390E0, 16 B, player/arm_weapon_fx_0388f0.s)
/// $0390E0: jsr $27bc8.l
pub fn i_0390e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390E6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x390E6);
    cpu.pc = 0x27BC8;
    16
}
/// $0390E6: bcc.w $390f0
pub fn i_0390e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x390F0; return 10; }
    8
}
/// $0390EA: lea.l $390f0(pc), a1
pub fn i_0390ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390EE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x390F0;
    16
}
/// $0390EE: move.l a1, (a6)
pub fn i_0390ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390F0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== DuckTrigger_Ground_0390f0  ($0390F0, 88 B, player/arm_weapon_fx_0388f0.s)
/// $0390F0: jsr $2783a.l
pub fn i_0390f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390F6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x390F6);
    cpu.pc = 0x2783A;
    16
}
/// $0390F6: jsr $28d70.l
pub fn i_0390f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x390FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x390FC);
    cpu.pc = 0x28D70;
    16
}
/// $0390FC: movea.l #$ffffffff, a0
pub fn i_0390fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39102;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $039102: lea.l $38f9e(pc), a0
pub fn i_039102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39106;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x38F9E;
    16
}
/// $039106: jsr $5dd5c.l
pub fn i_039106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3910C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3910C);
    cpu.pc = 0x5DD5C;
    16
}
/// $03910C: bcc.w $39118
pub fn i_03910c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39110;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x39118; return 10; }
    8
}
/// $039110: jmp $518.l
pub fn i_039110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39116;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $039116: rts
pub fn i_039116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39118;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $039118: movea.l $70(a6), a0
pub fn i_039118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3911C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03911C: move.b $3a(a6), d0
pub fn i_03911c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39120;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $039120: move.b $3a(a0), d1
pub fn i_039120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39124;
    cpu.ir = 0x1228;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $039124: eor.b d1, d0
pub fn i_039124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39126;
    cpu.ir = 0xB300;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $039126: btst.b #$0, d0
pub fn i_039126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3912A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $03912A: bne.w $39146
pub fn i_03912a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3912E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x39146; return 10; }
    8
}
/// $03912E: movea.l $74(a6), a1
pub fn i_03912e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39132;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039132: jsr $5e260.l
pub fn i_039132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39138;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39138);
    cpu.pc = 0x5E260;
    16
}
/// $039138: bcs.w $39146
pub fn i_039138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3913C;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x39146; return 10; }
    8
}
/// $03913C: movea.l $70(a6), a0
pub fn i_03913c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39140;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039140: bset.b #$0, $88(a0)
pub fn i_039140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39146;
    cpu.ir = 0x8E8;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[0].wrapping_add(0x88);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039146: rts
pub fn i_039146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39148;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerDeathFx_Splash_039148  ($039148, 98 B, player/arm_weapon_fx_0388f0.s)
/// $039148: move.w #$185, d1
pub fn i_039148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3914C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x185;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $03914C: jsr $236e.l
pub fn i_03914c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39152;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39152);
    cpu.pc = 0x236E;
    16
}
/// $039152: lea.l $27a182.l, a0
pub fn i_039152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39158;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27A182;
    20
}
/// $039158: jsr $28cd4.l
pub fn i_039158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3915E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3915E);
    cpu.pc = 0x28CD4;
    16
}
/// $03915E: move.w #$0, $5c(a6)
pub fn i_03915e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39164;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039164: lea.l $3916a(pc), a1
pub fn i_039164(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39168;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x3916A;
    16
}
/// $039168: move.l a1, (a6)
pub fn i_039168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3916A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $03916A: jsr $5e4ca.l
pub fn i_03916a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39170;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39170);
    cpu.pc = 0x5E4CA;
    16
}
/// $039170: move.w d1, $22(a6)
pub fn i_039170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39174;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039174: move.w d2, $24(a6)
pub fn i_039174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39178;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039178: andi.w #$ffe3, d0
pub fn i_039178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3917C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $03917C: or.w $5c(a6), d0
pub fn i_03917c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39180;
    cpu.ir = 0x806E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $039180: move.w d0, $38(a6)
pub fn i_039180(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39184;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $039184: jsr $28d70.l
pub fn i_039184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3918A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3918A);
    cpu.pc = 0x28D70;
    16
}
/// $03918A: bcc.w $39194
pub fn i_03918a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3918E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x39194; return 10; }
    8
}
/// $03918E: lea.l $391aa(pc), a1
pub fn i_03918e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39192;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x391AA;
    16
}
/// $039192: move.l a1, (a6)
pub fn i_039192(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39194;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $039194: movea.l $c(a6), a0
pub fn i_039194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39198;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039198: btst.b #$1, $8d(a0)
pub fn i_039198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3919E;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $03919E: bne.w $391a8
pub fn i_03919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391A2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x391A8; return 10; }
    8
}
/// $0391A2: lea.l $391aa(pc), a1
pub fn i_0391a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391A6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x391AA;
    16
}
/// $0391A6: move.l a1, (a6)
pub fn i_0391a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391A8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0391A8: rts
pub fn i_0391a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerDeathFx_Loop_0391aa  ($0391AA, 68 B, player/arm_weapon_fx_0388f0.s)
/// $0391AA: lea.l $27a228.l, a0
pub fn i_0391aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391B0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27A228;
    20
}
/// $0391B0: jsr $28cd4.l
pub fn i_0391b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x391B6);
    cpu.pc = 0x28CD4;
    16
}
/// $0391B6: move.w #$0, $5c(a6)
pub fn i_0391b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391BC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0391BC: lea.l $391c2(pc), a1
pub fn i_0391bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391C0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x391C2;
    16
}
/// $0391C0: move.l a1, (a6)
pub fn i_0391c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391C2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $0391C2: jsr $5e4ca.l
pub fn i_0391c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391C8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x391C8);
    cpu.pc = 0x5E4CA;
    16
}
/// $0391C8: move.w d1, $22(a6)
pub fn i_0391c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391CC;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0391CC: move.w d2, $24(a6)
pub fn i_0391cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391D0;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0391D0: andi.w #$ffe3, d0
pub fn i_0391d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391D4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0391D4: or.w $5c(a6), d0
pub fn i_0391d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391D8;
    cpu.ir = 0x806E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    let r = (v3 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0391D8: move.w d0, $38(a6)
pub fn i_0391d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391DC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0391DC: jsr $28d70.l
pub fn i_0391dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x391E2);
    cpu.pc = 0x28D70;
    16
}
/// $0391E2: bcc.w $391ec
pub fn i_0391e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391E6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x391EC; return 10; }
    8
}
/// $0391E6: jmp $518.l
pub fn i_0391e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391EC;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $0391EC: rts
pub fn i_0391ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerDeathFx_Ripple_0391ee  ($0391EE, 38 B, player/arm_weapon_fx_0388f0.s)
/// $0391EE: move.w #$185, d1
pub fn i_0391ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391F2;
    cpu.ir = 0x323C;
    let v1: u32 = 0x185;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0391F2: jsr $236e.l
pub fn i_0391f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391F8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x391F8);
    cpu.pc = 0x236E;
    16
}
/// $0391F8: lea.l $27a0fe.l, a0
pub fn i_0391f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x391FE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27A0FE;
    20
}
/// $0391FE: jsr $28cd4.l
pub fn i_0391fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39204;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39204);
    cpu.pc = 0x28CD4;
    16
}
/// $039204: move.w #$1c, $5c(a6)
pub fn i_039204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3920A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03920A: lea.l $39210(pc), a1
pub fn i_03920a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3920E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39210;
    16
}
/// $03920E: move.l a1, (a6)
pub fn i_03920e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39210;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $039210: jmp $391c2(pc)
pub fn i_039210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39214;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x391C2;
    8
}
// ===== PlayerDeathFx_Alt_039214  ($039214, 32 B, player/arm_weapon_fx_0388f0.s)
/// $039214: move.w #$1df, d1
pub fn i_039214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39218;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1DF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $039218: jsr $236e.l
pub fn i_039218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3921E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3921E);
    cpu.pc = 0x236E;
    16
}
/// $03921E: lea.l $279f9a.l, a0
pub fn i_03921e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39224;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x279F9A;
    20
}
/// $039224: jsr $28cd4.l
pub fn i_039224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3922A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3922A);
    cpu.pc = 0x28CD4;
    16
}
/// $03922A: lea.l $39230(pc), a1
pub fn i_03922a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3922E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39230;
    16
}
/// $03922E: move.l a1, (a6)
pub fn i_03922e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39230;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $039230: jmp $391c2(pc)
pub fn i_039230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39234;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x391C2;
    8
}
// ===== Entity_CmpField10WithLink8_039234  ($039234, 28 B, player/arm_weapon_fx_0388f0.s)
/// $039234: movea.l $8(a6), a1
pub fn i_039234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39238;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039238: move.b $10(a6), d0
pub fn i_039238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3923C;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03923C: cmp.b $10(a1), d0
pub fn i_03923c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39240;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $039240: bcs.w $3924a
pub fn i_039240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39244;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x3924A; return 10; }
    8
}
/// $039244: andi.b #$ee, ccr
pub fn i_039244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39248;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $039248: rts
pub fn i_039248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3924A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03924A: ori.b #$11, ccr
pub fn i_03924a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3924E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $03924E: rts
pub fn i_03924e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39250;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_ResetState_0392f8  ($0392F8, 66 B, player/arm_weapon_fx_0388f0.s)
/// $0392F8: move.w $16(a6), $14(a6)
pub fn i_0392f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x392FE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0392FE: bset.b #$6, $6b(a6)
pub fn i_0392fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39304;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039304: move.b #$ff, $32(a6)
pub fn i_039304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3930A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03930A: move.b #$ff, $33(a6)
pub fn i_03930a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39310;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $039310: jsr $5e98a.l
pub fn i_039310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39316;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39316);
    cpu.pc = 0x5E98A;
    16
}
/// $039316: clr.b $87(a1)
pub fn i_039316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3931A;
    cpu.ir = 0x4229;
    let ad1 = cpu.a[1].wrapping_add(0x87);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03931A: clr.b $8c(a6)
pub fn i_03931a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3931E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03931E: clr.w $72(a6)
pub fn i_03931e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39322;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $039322: clr.w $36(a6)
pub fn i_039322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39326;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $039326: clr.b $91(a6)
pub fn i_039326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3932A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x91);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03932A: bclr.b #$3, $13(a6)
pub fn i_03932a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39330;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039330: lea.l $ffff.w, a0
pub fn i_039330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39334;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $039334: move.l a0, $48(a6)
pub fn i_039334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39338;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $039338: rts
pub fn i_039338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3933A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_WeaponTableIndex_03933a  ($03933A, 26 B, player/arm_weapon_fx_0388f0.s)
/// $03933A: movea.l $c(a6), a0
pub fn i_03933a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3933E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03933E: moveq #$0, d0
pub fn i_03933e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39340;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $039340: move.b $71(a0), d0
pub fn i_039340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39344;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x71);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $039344: cmpa.l #$1004e0, a0
pub fn i_039344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3934A;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0x1004E0;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $03934A: bne.w $39350
pub fn i_03934a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3934E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x39350; return 10; }
    8
}
/// $03934E: addq.w #$5, d0
pub fn i_03934e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39350;
    cpu.ir = 0x5A40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 5, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $039350: lsl.w #$2, d0
pub fn i_039350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39352;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $039352: rts
pub fn i_039352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39354;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_AnimStepToParentDeath_039354  ($039354, 40 B, player/arm_weapon_fx_0388f0.s)
/// $039354: jsr $28d70.l
pub fn i_039354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3935A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3935A);
    cpu.pc = 0x28D70;
    16
}
/// $03935A: bcc.w $3936c
pub fn i_03935a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3935E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3936C; return 10; }
    8
}
/// $03935E: movea.l $c(a6), a0
pub fn i_03935e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39362;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039362: clr.b $21(a0)
pub fn i_039362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39366;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $039366: ori.b #$11, ccr
pub fn i_039366(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3936A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $03936A: rts
pub fn i_03936a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3936C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03936C: movea.l $c(a6), a0
pub fn i_03936c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39370;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039370: move.b #$ff, $21(a0)
pub fn i_039370(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39376;
    cpu.ir = 0x117C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    bus.write8(ad2, v1 as u8);
    12
}
/// $039376: andi.b #$ee, ccr
pub fn i_039376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3937A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $03937A: rts
pub fn i_03937a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3937C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_FollowParent_03937c  ($03937C, 82 B, player/arm_weapon_fx_0388f0.s)
/// $03937C: jsr $5e4b2.l
pub fn i_03937c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39382;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39382);
    cpu.pc = 0x5E4B2;
    16
}
/// $039382: move.w $7c(a0), d1
pub fn i_039382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39386;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $039386: move.w $7e(a0), d2
pub fn i_039386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3938A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x7E);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03938A: move.b $3a(a0), d3
pub fn i_03938a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3938E;
    cpu.ir = 0x1628;
    let ad2 = cpu.a[0].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03938E: btst.b #$0, d3
pub fn i_03938e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39392;
    cpu.ir = 0x803;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[3] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $039392: bne.w $3939e
pub fn i_039392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39396;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x3939E; return 10; }
    8
}
/// $039396: add.w d1, $22(a6)
pub fn i_039396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3939A;
    cpu.ir = 0xD36E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $03939A: bra.w $393a2
pub fn i_03939a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3939E;
    cpu.ir = 0x6000;
    cpu.pc = 0x393A2;
    10
}
/// $03939E: sub.w d1, $22(a6)
pub fn i_03939e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393A2;
    cpu.ir = 0x936E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0393A2: add.w d2, $24(a6)
pub fn i_0393a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393A6;
    cpu.ir = 0xD56E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $0393A6: move.b d3, $3a(a6)
pub fn i_0393a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393AA;
    cpu.ir = 0x1D43;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $0393AA: btst.b #$7, $5a(a0)
pub fn i_0393aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393B0;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0393B0: beq.w $393ba
pub fn i_0393b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393B4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x393BA; return 10; }
    8
}
/// $0393B4: bset.b #$7, $5a(a6)
pub fn i_0393b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393BA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0393BA: andi.w #$fffc, $38(a6)
pub fn i_0393ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393C0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFFC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0393C0: move.b $59(a0), $59(a6)
pub fn i_0393c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393C6;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x59);
    bus.write8(ad4, v3 as u8);
    20
}
/// $0393C6: jsr $32d00.l
pub fn i_0393c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393CC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x393CC);
    cpu.pc = 0x32D00;
    16
}
/// $0393CC: rts
pub fn i_0393cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393CE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputThunk_Left_0393ce  ($0393CE, 6 B, player/arm_weapon_fx_0388f0.s)
/// $0393CE: jmp $5cf04.l
pub fn i_0393ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393D4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CF04;
    8
}
// ===== InputThunk_Right_0393d4  ($0393D4, 6 B, player/arm_weapon_fx_0388f0.s)
/// $0393D4: jmp $5cf10.l
pub fn i_0393d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393DA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CF10;
    8
}
// ===== InputThunk_ShootHeld_0393da  ($0393DA, 4 B, player/arm_weapon_fx_0388f0.s)
/// $0393DA: jmp $393de(pc)
pub fn i_0393da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393DE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x393DE;
    8
}
// ===== Input_ShootHeldCheck_0393de  ($0393DE, 30 B, player/arm_weapon_fx_0388f0.s)
/// $0393DE: jsr $5cf84.l
pub fn i_0393de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393E4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x393E4);
    cpu.pc = 0x5CF84;
    16
}
/// $0393E4: bcc.w $393ea
pub fn i_0393e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x393EA; return 10; }
    8
}
/// $0393E8: rts
pub fn i_0393e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0393EA: jsr $5ceec.l
pub fn i_0393ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x393F0);
    cpu.pc = 0x5CEEC;
    16
}
/// $0393F0: bcc.w $393fa
pub fn i_0393f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393F4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x393FA; return 10; }
    8
}
/// $0393F4: jmp $5cf9c.l
pub fn i_0393f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393FA;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CF9C;
    8
}
/// $0393FA: rts
pub fn i_0393fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x393FC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Input_DownToCCR_0393fc  ($0393FC, 20 B, player/arm_weapon_fx_0388f0.s)
/// $0393FC: jsr $39410(pc)
pub fn i_0393fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39400;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39400);
    cpu.pc = 0x39410;
    16
}
/// $039400: bcc.w $3940a
pub fn i_039400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39404;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x3940A; return 10; }
    8
}
/// $039404: ori.b #$11, ccr
pub fn i_039404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39408;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $039408: rts
pub fn i_039408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3940A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03940A: andi.b #$ee, ccr
pub fn i_03940a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3940E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $03940E: rts
pub fn i_03940e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39410;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== InputThunk_Down_039410  ($039410, 6 B, player/arm_weapon_fx_0388f0.s)
/// $039410: jmp $5cef8.l
pub fn i_039410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39416;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CEF8;
    8
}
// ===== PlayerArm_CopyBit2ToParent_039416  ($039416, 24 B, player/arm_weapon_fx_0388f0.s)
/// $039416: movea.l $c(a6), a0
pub fn i_039416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3941A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03941A: move.b $8c(a6), d0
pub fn i_03941a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3941E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03941E: andi.b #$4, d0
pub fn i_03941e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39422;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $039422: andi.b #$fb, $8c(a0)
pub fn i_039422(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39428;
    cpu.ir = 0x228;
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0xFB) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039428: or.b d0, $8c(a0)
pub fn i_039428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3942C;
    cpu.ir = 0x8128;
    let ad1 = cpu.a[0].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $03942C: rts
pub fn i_03942c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3942E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AnimCb_AttackProbeMusic_03942e  ($03942E, 34 B, player/arm_weapon_fx_0388f0.s)
/// $03942E: jsr $283ca.l
pub fn i_03942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39434);
    cpu.pc = 0x283CA;
    16
}
/// $039434: jsr $283d8.l
pub fn i_039434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3943A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3943A);
    cpu.pc = 0x283D8;
    16
}
/// $03943A: btst.b #$2, $5a(a6)
pub fn i_03943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39440;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $039440: beq.w $3944e
pub fn i_039440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39444;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3944E; return 10; }
    8
}
/// $039444: move.w #$10fb, d0
pub fn i_039444(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39448;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10FB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $039448: jsr $2352.l
pub fn i_039448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3944E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3944E);
    cpu.pc = 0x2352;
    16
}
/// $03944E: rts
pub fn i_03944e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39450;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== AnimCb_RandField47_039450  ($039450, 16 B, player/arm_weapon_fx_0388f0.s)
/// $039450: jsr $5e9b6.l
pub fn i_039450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39456;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39456);
    cpu.pc = 0x5E9B6;
    16
}
/// $039456: andi.b #$a, d0
pub fn i_039456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3945A;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0xA) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $03945A: move.b d0, $47(a6)
pub fn i_03945a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3945E;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    bus.write8(ad2, v1 as u8);
    12
}
/// $03945E: rts
pub fn i_03945e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39460;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== PlayerArm_Pose00_039460  ($039460, 28 B, player/arm_weapon_fx_0388f0.s)
/// $039460: bclr.b #$2, $8c(a6)
pub fn i_039460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39466;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $039466: jsr $3933a(pc)
pub fn i_039466(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3946A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3946A);
    cpu.pc = 0x3933A;
    16
}
/// $03946A: lea.l $3947c(pc), a0
pub fn i_03946a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3946E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x3947C;
    16
}
/// $03946E: movea.l (a0, d0.w), a0
pub fn i_03946e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39472;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $039472: jsr $28cd4.l
pub fn i_039472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39478;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39478);
    cpu.pc = 0x28CD4;
    16
}
/// $039478: bra.w $394a4
pub fn i_039478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3947C;
    cpu.ir = 0x6000;
    cpu.pc = 0x394A4;
    10
}
/// $0394A4: bra.w $3960e
pub fn i_0394a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394A8;
    cpu.ir = 0x6000;
    cpu.pc = 0x3960E;
    10
}
// ===== PlayerArm_Spawn_0394a8  ($0394A8, 1024 B, player/arm_weapon_fx_0388f0.s)
/// $0394A8: move.w #$176, d1
pub fn i_0394a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394AC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x176;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394AC: jsr $236e.l
pub fn i_0394ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394B2);
    cpu.pc = 0x236E;
    16
}
/// $0394B2: move.w #$190, d1
pub fn i_0394b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394B6;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394B6: jsr $236e.l
pub fn i_0394b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394BC);
    cpu.pc = 0x236E;
    16
}
/// $0394BC: move.w #$191, d1
pub fn i_0394bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394C0;
    cpu.ir = 0x323C;
    let v1: u32 = 0x191;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394C0: jsr $236e.l
pub fn i_0394c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394C6);
    cpu.pc = 0x236E;
    16
}
/// $0394C6: move.w #$1b, $1c(a6)
pub fn i_0394c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394CC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0394CC: jsr $138fe.l
pub fn i_0394cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394D2);
    cpu.pc = 0x138FE;
    16
}
/// $0394D2: jsr $392f8(pc)
pub fn i_0394d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x394D6);
    cpu.pc = 0x392F8;
    16
}
/// $0394D6: bclr.b #$4, $12(a6)
pub fn i_0394d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394DC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0394DC: ori.w #$2, $38(a6)
pub fn i_0394dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394E2;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x2) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $0394E2: bra.w $39520
pub fn i_0394e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394E6;
    cpu.ir = 0x6000;
    cpu.pc = 0x39520;
    10
}
/// $0394E6: move.w #$177, d1
pub fn i_0394e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394EA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x177;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394EA: jsr $236e.l
pub fn i_0394ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394F0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394F0);
    cpu.pc = 0x236E;
    16
}
/// $0394F0: move.w #$190, d1
pub fn i_0394f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394F4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394F4: jsr $236e.l
pub fn i_0394f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x394FA);
    cpu.pc = 0x236E;
    16
}
/// $0394FA: move.w #$192, d1
pub fn i_0394fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x394FE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x192;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0394FE: jsr $236e.l
pub fn i_0394fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39504;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39504);
    cpu.pc = 0x236E;
    16
}
/// $039504: move.w #$1c, $1c(a6)
pub fn i_039504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3950A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03950A: jsr $138fe.l
pub fn i_03950a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39510;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x39510);
    cpu.pc = 0x138FE;
    16
}
/// $039510: jsr $392f8(pc)
pub fn i_039510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39514;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39514);
    cpu.pc = 0x392F8;
    16
}
/// $039514: bset.b #$4, $12(a6)
pub fn i_039514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3951A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $03951A: ori.w #$0, $38(a6)
pub fn i_03951a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39520;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $039520: lea.l $27aaea.l, a0
pub fn i_039520(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39526;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x27AAEA;
    20
}
/// $039526: jsr $28cd4.l
pub fn i_039526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3952C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x3952C);
    cpu.pc = 0x28CD4;
    16
}
/// $03952C: lea.l $39532(pc), a1
pub fn i_03952c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39530;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x39532;
    16
}
/// $039530: move.l a1, (a6)
pub fn i_039530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39532;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $039532: jsr $3937c(pc)
pub fn i_039532(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39536;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39536);
    cpu.pc = 0x3937C;
    16
}
/// $039536: movea.l $c(a6), a0
pub fn i_039536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3953A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $03953A: move.b $70(a0), d0
pub fn i_03953a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3953E;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x70);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $03953E: cmpi.b #$10, d0
pub fn i_03953e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39542;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x10, Size::Byte, false);
    8
}
/// $039542: beq.w $39644
pub fn i_039542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39546;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39644; return 10; }
    8
}
/// $039546: cmpi.b #$21, d0
pub fn i_039546(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3954A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x21, Size::Byte, false);
    8
}
/// $03954A: beq.w $3961c
pub fn i_03954a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3954E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3961C; return 10; }
    8
}
/// $03954E: cmpi.b #$11, d0
pub fn i_03954e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39552;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x11, Size::Byte, false);
    8
}
/// $039552: beq.w $396d8
pub fn i_039552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39556;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x396D8; return 10; }
    8
}
/// $039556: cmpi.b #$22, d0
pub fn i_039556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3955A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x22, Size::Byte, false);
    8
}
/// $03955A: beq.w $396b0
pub fn i_03955a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3955E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x396B0; return 10; }
    8
}
/// $03955E: cmpi.b #$12, d0
pub fn i_03955e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39562;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x12, Size::Byte, false);
    8
}
/// $039562: beq.w $3976c
pub fn i_039562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39566;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3976C; return 10; }
    8
}
/// $039566: cmpi.b #$23, d0
pub fn i_039566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3956A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x23, Size::Byte, false);
    8
}
/// $03956A: beq.w $39744
pub fn i_03956a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3956E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39744; return 10; }
    8
}
/// $03956E: cmpi.b #$25, d0
pub fn i_03956e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39572;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x25, Size::Byte, false);
    8
}
/// $039572: beq.w $3984a
pub fn i_039572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39576;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3984A; return 10; }
    8
}
/// $039576: cmpi.b #$27, d0
pub fn i_039576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3957A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x27, Size::Byte, false);
    8
}
/// $03957A: beq.w $39842
pub fn i_03957a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3957E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39842; return 10; }
    8
}
/// $03957E: cmpi.b #$26, d0
pub fn i_03957e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39582;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x26, Size::Byte, false);
    8
}
/// $039582: beq.w $397e4
pub fn i_039582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39586;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x397E4; return 10; }
    8
}
/// $039586: cmpi.b #$28, d0
pub fn i_039586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3958A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x28, Size::Byte, false);
    8
}
/// $03958A: beq.w $397dc
pub fn i_03958a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3958E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x397DC; return 10; }
    8
}
/// $03958E: cmpi.b #$30, d0
pub fn i_03958e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39592;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x30, Size::Byte, false);
    8
}
/// $039592: beq.w $395ee
pub fn i_039592(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39596;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $039596: cmpi.b #$31, d0
pub fn i_039596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3959A;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x31, Size::Byte, false);
    8
}
/// $03959A: beq.w $395ee
pub fn i_03959a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3959E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $03959E: cmpi.b #$1, d0
pub fn i_03959e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395A2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0395A2: beq.w $395ee
pub fn i_0395a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395A6: cmpi.b #$32, d0
pub fn i_0395a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395AA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x32, Size::Byte, false);
    8
}
/// $0395AA: beq.w $395ee
pub fn i_0395aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395AE: cmpi.b #$2, d0
pub fn i_0395ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395B2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $0395B2: beq.w $395ee
pub fn i_0395b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395B6: cmpi.b #$3, d0
pub fn i_0395b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395BA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $0395BA: beq.w $395ee
pub fn i_0395ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395BE: cmpi.b #$20, d0
pub fn i_0395be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395C2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $0395C2: beq.w $395ee
pub fn i_0395c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395C6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395C6: cmpi.b #$4, d0
pub fn i_0395c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395CA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $0395CA: beq.w $395ee
pub fn i_0395ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395CE: cmpi.b #$0, d0
pub fn i_0395ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395D2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0395D2: beq.w $395ee
pub fn i_0395d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x395EE; return 10; }
    8
}
/// $0395D6: cmpi.b #$34, d0
pub fn i_0395d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395DA;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x34, Size::Byte, false);
    8
}
/// $0395DA: bcs.w $395ea
pub fn i_0395da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395DE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x395EA; return 10; }
    8
}
/// $0395DE: nop
pub fn i_0395de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395E0;
    cpu.ir = 0x4E71;
    4
}
/// $0395E0: nop
pub fn i_0395e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395E2;
    cpu.ir = 0x4E71;
    4
}
/// $0395E2: cmpi.b #$34, d0
pub fn i_0395e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395E6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x34, Size::Byte, false);
    8
}
/// $0395E6: nop
pub fn i_0395e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395E8;
    cpu.ir = 0x4E71;
    4
}
/// $0395E8: trap #$f
pub fn i_0395e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395EA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0395EA: bra.w $395ee
pub fn i_0395ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395EE;
    cpu.ir = 0x6000;
    cpu.pc = 0x395EE;
    10
}
/// $0395EE: movea.l $c(a6), a0
pub fn i_0395ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0395F2: btst.b #$5, $69(a0)
pub fn i_0395f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395F8;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0395F8: beq.w $3960e
pub fn i_0395f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x395FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x3960E; return 10; }
    8
}
/// $0395FC: movea.l $74(a0), a1
pub fn i_0395fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39600;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $039600: jsr (a1)
pub fn i_039600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39602;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x39602);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $039602: movea.l $c(a6), a0
pub fn i_039602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39606;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039606: move.w $72(a0), d0
pub fn i_039606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3960A;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $03960A: move.w d0, $72(a6)
pub fn i_03960a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3960E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write16(ad2, v1 as u16);
    12
}
/// $03960E: clr.w $72(a6)
pub fn i_03960e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39612;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $039612: jsr $39354(pc)
pub fn i_039612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39616;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x39616);
    cpu.pc = 0x39354;
    16
}
/// $039616: jsr $39416(pc)
pub fn i_039616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3961A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x3961A);
    cpu.pc = 0x39416;
    16
}
/// $03961A: rts
pub fn i_03961a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3961C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $03961C: movea.l $c(a6), a0
pub fn i_03961c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39620;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $039620: btst.b #$5, $69(a0)
pub fn i_039620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x39626;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $039626: beq.w $39640
pub fn i_039626(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3962A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x39640; return 10; }
    8
}
/// $03962A: movea.l $74(a0), a1
pub fn i_03962a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x3962E;
    cpu.ir = 0x2268;
    let ad2 = cpu.a[0].wrapping_add(0x74);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x037A80, i_037a80),
    (0x037A86, i_037a86),
    (0x037A8C, i_037a8c),
    (0x037A92, i_037a92),
    (0x037A96, i_037a96),
    (0x037A9C, i_037a9c),
    (0x037AA0, i_037aa0),
    (0x037AA6, i_037aa6),
    (0x037AAC, i_037aac),
    (0x037AB2, i_037ab2),
    (0x037AB6, i_037ab6),
    (0x037AB8, i_037ab8),
    (0x037ABE, i_037abe),
    (0x037AC2, i_037ac2),
    (0x037AC8, i_037ac8),
    (0x037ACC, i_037acc),
    (0x037AD0, i_037ad0),
    (0x037AD4, i_037ad4),
    (0x037AD8, i_037ad8),
    (0x037ADC, i_037adc),
    (0x037AE2, i_037ae2),
    (0x037AE6, i_037ae6),
    (0x037AEA, i_037aea),
    (0x037AEC, i_037aec),
    (0x037AF0, i_037af0),
    (0x037AF2, i_037af2),
    (0x037AF6, i_037af6),
    (0x037AFA, i_037afa),
    (0x037B00, i_037b00),
    (0x037B06, i_037b06),
    (0x037B0C, i_037b0c),
    (0x037B12, i_037b12),
    (0x037B18, i_037b18),
    (0x037B1E, i_037b1e),
    (0x037B24, i_037b24),
    (0x037B2A, i_037b2a),
    (0x037B30, i_037b30),
    (0x037B36, i_037b36),
    (0x037B3C, i_037b3c),
    (0x037B42, i_037b42),
    (0x037B48, i_037b48),
    (0x037B4E, i_037b4e),
    (0x037B54, i_037b54),
    (0x037B58, i_037b58),
    (0x037B5A, i_037b5a),
    (0x037B5E, i_037b5e),
    (0x037B64, i_037b64),
    (0x037B68, i_037b68),
    (0x037B6C, i_037b6c),
    (0x037B70, i_037b70),
    (0x037B74, i_037b74),
    (0x037B78, i_037b78),
    (0x037B7E, i_037b7e),
    (0x037B82, i_037b82),
    (0x037B86, i_037b86),
    (0x037B88, i_037b88),
    (0x037B8C, i_037b8c),
    (0x037B8E, i_037b8e),
    (0x037B94, i_037b94),
    (0x037B98, i_037b98),
    (0x037B9E, i_037b9e),
    (0x037BA2, i_037ba2),
    (0x037BA6, i_037ba6),
    (0x037BAC, i_037bac),
    (0x037BB0, i_037bb0),
    (0x037BB4, i_037bb4),
    (0x037BBA, i_037bba),
    (0x037BC0, i_037bc0),
    (0x037BC6, i_037bc6),
    (0x037BCC, i_037bcc),
    (0x037BD2, i_037bd2),
    (0x037BD6, i_037bd6),
    (0x037BDA, i_037bda),
    (0x037BDE, i_037bde),
    (0x037BE0, i_037be0),
    (0x037BE6, i_037be6),
    (0x037BEC, i_037bec),
    (0x037BF0, i_037bf0),
    (0x037BF6, i_037bf6),
    (0x037BFC, i_037bfc),
    (0x037C02, i_037c02),
    (0x037C08, i_037c08),
    (0x037C0E, i_037c0e),
    (0x037C10, i_037c10),
    (0x037C12, i_037c12),
    (0x037C18, i_037c18),
    (0x037C1A, i_037c1a),
    (0x037C20, i_037c20),
    (0x037C26, i_037c26),
    (0x037C2C, i_037c2c),
    (0x037C32, i_037c32),
    (0x037C36, i_037c36),
    (0x037C3A, i_037c3a),
    (0x037C3E, i_037c3e),
    (0x037C40, i_037c40),
    (0x037C46, i_037c46),
    (0x037C4C, i_037c4c),
    (0x037C50, i_037c50),
    (0x037C56, i_037c56),
    (0x037C5C, i_037c5c),
    (0x037C62, i_037c62),
    (0x037C68, i_037c68),
    (0x037C6A, i_037c6a),
    (0x037C6C, i_037c6c),
    (0x037C72, i_037c72),
    (0x037C74, i_037c74),
    (0x037C7A, i_037c7a),
    (0x037C80, i_037c80),
    (0x037C86, i_037c86),
    (0x037C8C, i_037c8c),
    (0x037C92, i_037c92),
    (0x037C98, i_037c98),
    (0x037C9E, i_037c9e),
    (0x037CA4, i_037ca4),
    (0x037CAA, i_037caa),
    (0x037CB0, i_037cb0),
    (0x037CB6, i_037cb6),
    (0x037CBE, i_037cbe),
    (0x037CC2, i_037cc2),
    (0x037CC6, i_037cc6),
    (0x037CCA, i_037cca),
    (0x037CCE, i_037cce),
    (0x037CD0, i_037cd0),
    (0x037CD4, i_037cd4),
    (0x037CD8, i_037cd8),
    (0x037CDC, i_037cdc),
    (0x037CE0, i_037ce0),
    (0x037CE4, i_037ce4),
    (0x037CEA, i_037cea),
    (0x037CEE, i_037cee),
    (0x037CF2, i_037cf2),
    (0x037CF8, i_037cf8),
    (0x037CFE, i_037cfe),
    (0x037D02, i_037d02),
    (0x037D06, i_037d06),
    (0x037D0A, i_037d0a),
    (0x037D0E, i_037d0e),
    (0x037D10, i_037d10),
    (0x037D14, i_037d14),
    (0x037D18, i_037d18),
    (0x037D1C, i_037d1c),
    (0x037D1E, i_037d1e),
    (0x037D22, i_037d22),
    (0x037D26, i_037d26),
    (0x037D2A, i_037d2a),
    (0x037D2C, i_037d2c),
    (0x037D30, i_037d30),
    (0x037D34, i_037d34),
    (0x037D38, i_037d38),
    (0x037D3C, i_037d3c),
    (0x037D40, i_037d40),
    (0x037D42, i_037d42),
    (0x037D46, i_037d46),
    (0x037D4A, i_037d4a),
    (0x037D4E, i_037d4e),
    (0x037D52, i_037d52),
    (0x037D54, i_037d54),
    (0x037D58, i_037d58),
    (0x037D5C, i_037d5c),
    (0x037D5E, i_037d5e),
    (0x037D62, i_037d62),
    (0x037D68, i_037d68),
    (0x037D6C, i_037d6c),
    (0x037D72, i_037d72),
    (0x037D76, i_037d76),
    (0x037D7A, i_037d7a),
    (0x037D7C, i_037d7c),
    (0x037D82, i_037d82),
    (0x037D86, i_037d86),
    (0x037D8C, i_037d8c),
    (0x037D90, i_037d90),
    (0x037D94, i_037d94),
    (0x037D96, i_037d96),
    (0x037D9C, i_037d9c),
    (0x037DA0, i_037da0),
    (0x037DA4, i_037da4),
    (0x037DA6, i_037da6),
    (0x037DAA, i_037daa),
    (0x037DAE, i_037dae),
    (0x037DB2, i_037db2),
    (0x037DB4, i_037db4),
    (0x037DB8, i_037db8),
    (0x037DBA, i_037dba),
    (0x037DC0, i_037dc0),
    (0x037DC6, i_037dc6),
    (0x037DCC, i_037dcc),
    (0x037DD2, i_037dd2),
    (0x037DD8, i_037dd8),
    (0x037DDE, i_037dde),
    (0x037DE4, i_037de4),
    (0x037DEA, i_037dea),
    (0x037DF0, i_037df0),
    (0x037DF6, i_037df6),
    (0x037DFC, i_037dfc),
    (0x037E02, i_037e02),
    (0x037E06, i_037e06),
    (0x037E0A, i_037e0a),
    (0x037E0E, i_037e0e),
    (0x037E12, i_037e12),
    (0x037E14, i_037e14),
    (0x037E18, i_037e18),
    (0x037E1C, i_037e1c),
    (0x037E20, i_037e20),
    (0x037E26, i_037e26),
    (0x037E2A, i_037e2a),
    (0x037E2E, i_037e2e),
    (0x037E34, i_037e34),
    (0x037E3A, i_037e3a),
    (0x037E3E, i_037e3e),
    (0x037E42, i_037e42),
    (0x037E46, i_037e46),
    (0x037E4A, i_037e4a),
    (0x037E4C, i_037e4c),
    (0x037E50, i_037e50),
    (0x037E54, i_037e54),
    (0x037E58, i_037e58),
    (0x037E5A, i_037e5a),
    (0x037E5E, i_037e5e),
    (0x037E62, i_037e62),
    (0x037E66, i_037e66),
    (0x037E68, i_037e68),
    (0x037E6C, i_037e6c),
    (0x037E70, i_037e70),
    (0x037E74, i_037e74),
    (0x037E78, i_037e78),
    (0x037E7C, i_037e7c),
    (0x037E7E, i_037e7e),
    (0x037E82, i_037e82),
    (0x037E86, i_037e86),
    (0x037E8A, i_037e8a),
    (0x037E8E, i_037e8e),
    (0x037E90, i_037e90),
    (0x037E94, i_037e94),
    (0x037E98, i_037e98),
    (0x037E9A, i_037e9a),
    (0x037E9E, i_037e9e),
    (0x037EA4, i_037ea4),
    (0x037EA8, i_037ea8),
    (0x037EAC, i_037eac),
    (0x037EAE, i_037eae),
    (0x037EB2, i_037eb2),
    (0x037EB6, i_037eb6),
    (0x037EBA, i_037eba),
    (0x037EBC, i_037ebc),
    (0x037EC0, i_037ec0),
    (0x037EC2, i_037ec2),
    (0x037EC8, i_037ec8),
    (0x037ECE, i_037ece),
    (0x037ED4, i_037ed4),
    (0x037EDA, i_037eda),
    (0x037EE0, i_037ee0),
    (0x037EE6, i_037ee6),
    (0x037EEC, i_037eec),
    (0x037EF2, i_037ef2),
    (0x037EF8, i_037ef8),
    (0x037EFE, i_037efe),
    (0x037F04, i_037f04),
    (0x037F08, i_037f08),
    (0x037F0C, i_037f0c),
    (0x037F10, i_037f10),
    (0x037F14, i_037f14),
    (0x037F16, i_037f16),
    (0x037F1A, i_037f1a),
    (0x037F1E, i_037f1e),
    (0x037F22, i_037f22),
    (0x037F26, i_037f26),
    (0x037F2A, i_037f2a),
    (0x037F30, i_037f30),
    (0x037F34, i_037f34),
    (0x037F38, i_037f38),
    (0x037F3E, i_037f3e),
    (0x037F44, i_037f44),
    (0x037F48, i_037f48),
    (0x037F4C, i_037f4c),
    (0x037F50, i_037f50),
    (0x037F54, i_037f54),
    (0x037F56, i_037f56),
    (0x037F5C, i_037f5c),
    (0x037F60, i_037f60),
    (0x037F64, i_037f64),
    (0x037F66, i_037f66),
    (0x037F6C, i_037f6c),
    (0x037F70, i_037f70),
    (0x037F76, i_037f76),
    (0x037F7A, i_037f7a),
    (0x037F7E, i_037f7e),
    (0x037F80, i_037f80),
    (0x037F86, i_037f86),
    (0x037F8A, i_037f8a),
    (0x037F8C, i_037f8c),
    (0x037F90, i_037f90),
    (0x037F94, i_037f94),
    (0x037F98, i_037f98),
    (0x037F9A, i_037f9a),
    (0x037FA0, i_037fa0),
    (0x037FA4, i_037fa4),
    (0x037FA8, i_037fa8),
    (0x037FAA, i_037faa),
    (0x037FAE, i_037fae),
    (0x037FB2, i_037fb2),
    (0x037FB6, i_037fb6),
    (0x037FB8, i_037fb8),
    (0x037FBC, i_037fbc),
    (0x037FC0, i_037fc0),
    (0x037FC4, i_037fc4),
    (0x037FC6, i_037fc6),
    (0x037FCA, i_037fca),
    (0x037FCE, i_037fce),
    (0x037FD2, i_037fd2),
    (0x037FD6, i_037fd6),
    (0x037FDA, i_037fda),
    (0x037FDC, i_037fdc),
    (0x037FE0, i_037fe0),
    (0x037FE4, i_037fe4),
    (0x037FE8, i_037fe8),
    (0x037FEC, i_037fec),
    (0x037FEE, i_037fee),
    (0x037FF2, i_037ff2),
    (0x037FF6, i_037ff6),
    (0x037FFA, i_037ffa),
    (0x037FFE, i_037ffe),
    (0x038000, i_038000),
    (0x038004, i_038004),
    (0x038008, i_038008),
    (0x03800C, i_03800c),
    (0x038010, i_038010),
    (0x038012, i_038012),
    (0x038016, i_038016),
    (0x03801A, i_03801a),
    (0x03801C, i_03801c),
    (0x038020, i_038020),
    (0x038026, i_038026),
    (0x03802A, i_03802a),
    (0x03802E, i_03802e),
    (0x038030, i_038030),
    (0x038034, i_038034),
    (0x038038, i_038038),
    (0x03803C, i_03803c),
    (0x03803E, i_03803e),
    (0x038042, i_038042),
    (0x038044, i_038044),
    (0x03804A, i_03804a),
    (0x03804E, i_03804e),
    (0x038052, i_038052),
    (0x038056, i_038056),
    (0x03805A, i_03805a),
    (0x03805E, i_03805e),
    (0x038062, i_038062),
    (0x038066, i_038066),
    (0x03806A, i_03806a),
    (0x03806E, i_03806e),
    (0x038072, i_038072),
    (0x038076, i_038076),
    (0x03807A, i_03807a),
    (0x03807E, i_03807e),
    (0x038082, i_038082),
    (0x038086, i_038086),
    (0x03808C, i_03808c),
    (0x038090, i_038090),
    (0x038094, i_038094),
    (0x03809A, i_03809a),
    (0x0380A0, i_0380a0),
    (0x0380A6, i_0380a6),
    (0x0380AC, i_0380ac),
    (0x0380B2, i_0380b2),
    (0x0380B8, i_0380b8),
    (0x0380BE, i_0380be),
    (0x0380C4, i_0380c4),
    (0x0380CA, i_0380ca),
    (0x0380CE, i_0380ce),
    (0x0380D2, i_0380d2),
    (0x0380D6, i_0380d6),
    (0x0380DC, i_0380dc),
    (0x0380E2, i_0380e2),
    (0x0380E8, i_0380e8),
    (0x0380EE, i_0380ee),
    (0x0380F4, i_0380f4),
    (0x0380FC, i_0380fc),
    (0x038102, i_038102),
    (0x038106, i_038106),
    (0x03810A, i_03810a),
    (0x03810C, i_03810c),
    (0x038112, i_038112),
    (0x038116, i_038116),
    (0x03811A, i_03811a),
    (0x038120, i_038120),
    (0x038124, i_038124),
    (0x038128, i_038128),
    (0x03812C, i_03812c),
    (0x03812E, i_03812e),
    (0x038134, i_038134),
    (0x038138, i_038138),
    (0x03813C, i_03813c),
    (0x03813E, i_03813e),
    (0x038144, i_038144),
    (0x038148, i_038148),
    (0x03814C, i_03814c),
    (0x038150, i_038150),
    (0x038154, i_038154),
    (0x038158, i_038158),
    (0x03815C, i_03815c),
    (0x03815E, i_03815e),
    (0x038162, i_038162),
    (0x038166, i_038166),
    (0x03816A, i_03816a),
    (0x03816E, i_03816e),
    (0x038170, i_038170),
    (0x038174, i_038174),
    (0x038178, i_038178),
    (0x03817A, i_03817a),
    (0x03817E, i_03817e),
    (0x038184, i_038184),
    (0x038188, i_038188),
    (0x03818C, i_03818c),
    (0x03818E, i_03818e),
    (0x038192, i_038192),
    (0x038196, i_038196),
    (0x03819A, i_03819a),
    (0x03819C, i_03819c),
    (0x0381A0, i_0381a0),
    (0x0381A2, i_0381a2),
    (0x0381A8, i_0381a8),
    (0x0381AE, i_0381ae),
    (0x0381B4, i_0381b4),
    (0x0381BA, i_0381ba),
    (0x0381BE, i_0381be),
    (0x0381C4, i_0381c4),
    (0x0381CA, i_0381ca),
    (0x0381D0, i_0381d0),
    (0x0381D6, i_0381d6),
    (0x0381DC, i_0381dc),
    (0x0381E2, i_0381e2),
    (0x0381E6, i_0381e6),
    (0x0381EC, i_0381ec),
    (0x0381F2, i_0381f2),
    (0x0381F8, i_0381f8),
    (0x0381FE, i_0381fe),
    (0x038204, i_038204),
    (0x03820A, i_03820a),
    (0x03820E, i_03820e),
    (0x038214, i_038214),
    (0x03821A, i_03821a),
    (0x038220, i_038220),
    (0x038226, i_038226),
    (0x03822A, i_03822a),
    (0x038230, i_038230),
    (0x038236, i_038236),
    (0x03823C, i_03823c),
    (0x038242, i_038242),
    (0x038248, i_038248),
    (0x03824E, i_03824e),
    (0x038252, i_038252),
    (0x038258, i_038258),
    (0x03825E, i_03825e),
    (0x038264, i_038264),
    (0x03826A, i_03826a),
    (0x038270, i_038270),
    (0x038276, i_038276),
    (0x03827A, i_03827a),
    (0x038280, i_038280),
    (0x038286, i_038286),
    (0x03828C, i_03828c),
    (0x038292, i_038292),
    (0x038296, i_038296),
    (0x03829C, i_03829c),
    (0x0382A2, i_0382a2),
    (0x0382A8, i_0382a8),
    (0x0382AE, i_0382ae),
    (0x0382B4, i_0382b4),
    (0x0382BA, i_0382ba),
    (0x0382BE, i_0382be),
    (0x0382C4, i_0382c4),
    (0x0382CA, i_0382ca),
    (0x0382D0, i_0382d0),
    (0x0382D6, i_0382d6),
    (0x0382DC, i_0382dc),
    (0x0382E2, i_0382e2),
    (0x0382EA, i_0382ea),
    (0x0382F0, i_0382f0),
    (0x0382F4, i_0382f4),
    (0x0382F8, i_0382f8),
    (0x0382FC, i_0382fc),
    (0x038300, i_038300),
    (0x038304, i_038304),
    (0x038306, i_038306),
    (0x03830C, i_03830c),
    (0x038310, i_038310),
    (0x038314, i_038314),
    (0x038318, i_038318),
    (0x03831C, i_03831c),
    (0x03831E, i_03831e),
    (0x038324, i_038324),
    (0x038328, i_038328),
    (0x03832E, i_03832e),
    (0x038332, i_038332),
    (0x038336, i_038336),
    (0x03833A, i_03833a),
    (0x03833E, i_03833e),
    (0x038340, i_038340),
    (0x038344, i_038344),
    (0x038348, i_038348),
    (0x03834C, i_03834c),
    (0x03834E, i_03834e),
    (0x038352, i_038352),
    (0x038356, i_038356),
    (0x03835A, i_03835a),
    (0x03835E, i_03835e),
    (0x038360, i_038360),
    (0x038364, i_038364),
    (0x038368, i_038368),
    (0x03836C, i_03836c),
    (0x03836E, i_03836e),
    (0x038374, i_038374),
    (0x038378, i_038378),
    (0x03837C, i_03837c),
    (0x03837E, i_03837e),
    (0x038382, i_038382),
    (0x038386, i_038386),
    (0x03838A, i_03838a),
    (0x03838E, i_03838e),
    (0x038392, i_038392),
    (0x038394, i_038394),
    (0x038398, i_038398),
    (0x03839C, i_03839c),
    (0x0383A0, i_0383a0),
    (0x0383A4, i_0383a4),
    (0x0383A6, i_0383a6),
    (0x0383AA, i_0383aa),
    (0x0383AE, i_0383ae),
    (0x0383B0, i_0383b0),
    (0x0383B4, i_0383b4),
    (0x0383BA, i_0383ba),
    (0x0383BE, i_0383be),
    (0x0383C4, i_0383c4),
    (0x0383C8, i_0383c8),
    (0x0383CC, i_0383cc),
    (0x0383CE, i_0383ce),
    (0x0383D4, i_0383d4),
    (0x0383D8, i_0383d8),
    (0x0383DC, i_0383dc),
    (0x0383DE, i_0383de),
    (0x0383E2, i_0383e2),
    (0x0383E6, i_0383e6),
    (0x0383EA, i_0383ea),
    (0x0383EC, i_0383ec),
    (0x0383F0, i_0383f0),
    (0x0383F8, i_0383f8),
    (0x0383FC, i_0383fc),
    (0x038402, i_038402),
    (0x038406, i_038406),
    (0x03840C, i_03840c),
    (0x038410, i_038410),
    (0x038416, i_038416),
    (0x03841A, i_03841a),
    (0x038420, i_038420),
    (0x038424, i_038424),
    (0x038428, i_038428),
    (0x03842A, i_03842a),
    (0x03842C, i_03842c),
    (0x038432, i_038432),
    (0x038436, i_038436),
    (0x03843C, i_03843c),
    (0x038442, i_038442),
    (0x038448, i_038448),
    (0x03844E, i_03844e),
    (0x038454, i_038454),
    (0x03845A, i_03845a),
    (0x03845E, i_03845e),
    (0x038464, i_038464),
    (0x03846A, i_03846a),
    (0x038470, i_038470),
    (0x038476, i_038476),
    (0x03847C, i_03847c),
    (0x038482, i_038482),
    (0x038488, i_038488),
    (0x03848E, i_03848e),
    (0x038494, i_038494),
    (0x03849C, i_03849c),
    (0x0384A2, i_0384a2),
    (0x0384A6, i_0384a6),
    (0x0384AA, i_0384aa),
    (0x0384AE, i_0384ae),
    (0x0384B4, i_0384b4),
    (0x0384B8, i_0384b8),
    (0x0384BA, i_0384ba),
    (0x0384C0, i_0384c0),
    (0x0384C4, i_0384c4),
    (0x0384C8, i_0384c8),
    (0x0384CC, i_0384cc),
    (0x0384D0, i_0384d0),
    (0x0384D2, i_0384d2),
    (0x0384D8, i_0384d8),
    (0x0384DE, i_0384de),
    (0x0384E2, i_0384e2),
    (0x0384E6, i_0384e6),
    (0x0384EA, i_0384ea),
    (0x0384EE, i_0384ee),
    (0x0384F0, i_0384f0),
    (0x0384F6, i_0384f6),
    (0x0384FA, i_0384fa),
    (0x0384FE, i_0384fe),
    (0x038500, i_038500),
    (0x038504, i_038504),
    (0x038508, i_038508),
    (0x03850C, i_03850c),
    (0x038510, i_038510),
    (0x038514, i_038514),
    (0x038516, i_038516),
    (0x03851A, i_03851a),
    (0x03851E, i_03851e),
    (0x038522, i_038522),
    (0x038526, i_038526),
    (0x038528, i_038528),
    (0x03852C, i_03852c),
    (0x038530, i_038530),
    (0x038532, i_038532),
    (0x038536, i_038536),
    (0x03853C, i_03853c),
    (0x038540, i_038540),
    (0x038544, i_038544),
    (0x038546, i_038546),
    (0x03854A, i_03854a),
    (0x03854E, i_03854e),
    (0x038552, i_038552),
    (0x038554, i_038554),
    (0x038558, i_038558),
    (0x03855A, i_03855a),
    (0x038560, i_038560),
    (0x038564, i_038564),
    (0x03856A, i_03856a),
    (0x038570, i_038570),
    (0x038576, i_038576),
    (0x03857C, i_03857c),
    (0x038582, i_038582),
    (0x038588, i_038588),
    (0x03858C, i_03858c),
    (0x038592, i_038592),
    (0x038598, i_038598),
    (0x03859E, i_03859e),
    (0x0385A4, i_0385a4),
    (0x0385AA, i_0385aa),
    (0x0385B0, i_0385b0),
    (0x0385B6, i_0385b6),
    (0x0385BC, i_0385bc),
    (0x0385C2, i_0385c2),
    (0x0385CA, i_0385ca),
    (0x0385D0, i_0385d0),
    (0x0385D4, i_0385d4),
    (0x0385D8, i_0385d8),
    (0x0385DC, i_0385dc),
    (0x0385E2, i_0385e2),
    (0x0385E6, i_0385e6),
    (0x0385E8, i_0385e8),
    (0x0385EE, i_0385ee),
    (0x0385F2, i_0385f2),
    (0x0385F6, i_0385f6),
    (0x0385FA, i_0385fa),
    (0x0385FE, i_0385fe),
    (0x038600, i_038600),
    (0x038606, i_038606),
    (0x03860C, i_03860c),
    (0x038610, i_038610),
    (0x038614, i_038614),
    (0x038618, i_038618),
    (0x03861C, i_03861c),
    (0x03861E, i_03861e),
    (0x038624, i_038624),
    (0x038628, i_038628),
    (0x03862C, i_03862c),
    (0x03862E, i_03862e),
    (0x038632, i_038632),
    (0x038636, i_038636),
    (0x03863A, i_03863a),
    (0x03863E, i_03863e),
    (0x038642, i_038642),
    (0x038644, i_038644),
    (0x038648, i_038648),
    (0x03864C, i_03864c),
    (0x038650, i_038650),
    (0x038654, i_038654),
    (0x038656, i_038656),
    (0x03865A, i_03865a),
    (0x03865E, i_03865e),
    (0x038660, i_038660),
    (0x038664, i_038664),
    (0x03866A, i_03866a),
    (0x03866E, i_03866e),
    (0x038672, i_038672),
    (0x038674, i_038674),
    (0x038678, i_038678),
    (0x03867C, i_03867c),
    (0x038680, i_038680),
    (0x038682, i_038682),
    (0x038686, i_038686),
    (0x038688, i_038688),
    (0x03868E, i_03868e),
    (0x038692, i_038692),
    (0x038698, i_038698),
    (0x03869E, i_03869e),
    (0x0386A4, i_0386a4),
    (0x0386AA, i_0386aa),
    (0x0386B0, i_0386b0),
    (0x0386B6, i_0386b6),
    (0x0386BA, i_0386ba),
    (0x0386C0, i_0386c0),
    (0x0386C6, i_0386c6),
    (0x0386CC, i_0386cc),
    (0x0386D2, i_0386d2),
    (0x0386D8, i_0386d8),
    (0x0386DE, i_0386de),
    (0x0386E2, i_0386e2),
    (0x0386E8, i_0386e8),
    (0x0386EC, i_0386ec),
    (0x0386F2, i_0386f2),
    (0x0386F8, i_0386f8),
    (0x0386FE, i_0386fe),
    (0x038704, i_038704),
    (0x03870A, i_03870a),
    (0x038710, i_038710),
    (0x038714, i_038714),
    (0x03871A, i_03871a),
    (0x038720, i_038720),
    (0x038726, i_038726),
    (0x03872C, i_03872c),
    (0x038732, i_038732),
    (0x038738, i_038738),
    (0x03873C, i_03873c),
    (0x038742, i_038742),
    (0x038748, i_038748),
    (0x038750, i_038750),
    (0x038756, i_038756),
    (0x03875A, i_03875a),
    (0x03875E, i_03875e),
    (0x038764, i_038764),
    (0x038768, i_038768),
    (0x03876E, i_03876e),
    (0x038774, i_038774),
    (0x03877A, i_03877a),
    (0x038780, i_038780),
    (0x038786, i_038786),
    (0x03878C, i_03878c),
    (0x038790, i_038790),
    (0x038796, i_038796),
    (0x03879C, i_03879c),
    (0x0387A2, i_0387a2),
    (0x0387A8, i_0387a8),
    (0x0387AE, i_0387ae),
    (0x0387B4, i_0387b4),
    (0x0387B8, i_0387b8),
    (0x0387BC, i_0387bc),
    (0x0387C0, i_0387c0),
    (0x0387C2, i_0387c2),
    (0x0387C8, i_0387c8),
    (0x0387CC, i_0387cc),
    (0x0387D0, i_0387d0),
    (0x0387D6, i_0387d6),
    (0x0387DA, i_0387da),
    (0x0387DE, i_0387de),
    (0x0387E2, i_0387e2),
    (0x0387E4, i_0387e4),
    (0x0387EA, i_0387ea),
    (0x0387EE, i_0387ee),
    (0x0387F4, i_0387f4),
    (0x0387F8, i_0387f8),
    (0x0387FC, i_0387fc),
    (0x038800, i_038800),
    (0x038804, i_038804),
    (0x038806, i_038806),
    (0x03880A, i_03880a),
    (0x03880E, i_03880e),
    (0x038812, i_038812),
    (0x038816, i_038816),
    (0x038818, i_038818),
    (0x03881C, i_03881c),
    (0x038820, i_038820),
    (0x038824, i_038824),
    (0x038828, i_038828),
    (0x03882C, i_03882c),
    (0x03882E, i_03882e),
    (0x038832, i_038832),
    (0x038836, i_038836),
    (0x03883A, i_03883a),
    (0x03883E, i_03883e),
    (0x038840, i_038840),
    (0x038844, i_038844),
    (0x038848, i_038848),
    (0x03884A, i_03884a),
    (0x03884E, i_03884e),
    (0x038854, i_038854),
    (0x038858, i_038858),
    (0x03885E, i_03885e),
    (0x038862, i_038862),
    (0x038866, i_038866),
    (0x038868, i_038868),
    (0x03886E, i_03886e),
    (0x038872, i_038872),
    (0x038878, i_038878),
    (0x03887C, i_03887c),
    (0x038880, i_038880),
    (0x038882, i_038882),
    (0x038888, i_038888),
    (0x03888C, i_03888c),
    (0x038890, i_038890),
    (0x038892, i_038892),
    (0x038896, i_038896),
    (0x03889A, i_03889a),
    (0x03889E, i_03889e),
    (0x0388A0, i_0388a0),
    (0x0388A6, i_0388a6),
    (0x0388AA, i_0388aa),
    (0x0388AE, i_0388ae),
    (0x0388B0, i_0388b0),
    (0x0388B4, i_0388b4),
    (0x0388BC, i_0388bc),
    (0x0388C0, i_0388c0),
    (0x0388C6, i_0388c6),
    (0x0388CA, i_0388ca),
    (0x0388D0, i_0388d0),
    (0x0388D4, i_0388d4),
    (0x0388DA, i_0388da),
    (0x0388DE, i_0388de),
    (0x0388E4, i_0388e4),
    (0x0388E8, i_0388e8),
    (0x0388EC, i_0388ec),
    (0x0388EE, i_0388ee),
    (0x0388F0, i_0388f0),
    (0x0388F6, i_0388f6),
    (0x0388FC, i_0388fc),
    (0x038904, i_038904),
    (0x03890A, i_03890a),
    (0x03890E, i_03890e),
    (0x038912, i_038912),
    (0x038918, i_038918),
    (0x03891C, i_03891c),
    (0x038922, i_038922),
    (0x038928, i_038928),
    (0x03892E, i_03892e),
    (0x038934, i_038934),
    (0x03893A, i_03893a),
    (0x038940, i_038940),
    (0x038944, i_038944),
    (0x03894A, i_03894a),
    (0x038950, i_038950),
    (0x038956, i_038956),
    (0x03895C, i_03895c),
    (0x038962, i_038962),
    (0x038968, i_038968),
    (0x03896C, i_03896c),
    (0x038970, i_038970),
    (0x038974, i_038974),
    (0x038976, i_038976),
    (0x03897C, i_03897c),
    (0x038980, i_038980),
    (0x038984, i_038984),
    (0x03898A, i_03898a),
    (0x03898E, i_03898e),
    (0x038992, i_038992),
    (0x038996, i_038996),
    (0x038998, i_038998),
    (0x03899E, i_03899e),
    (0x0389A2, i_0389a2),
    (0x0389A8, i_0389a8),
    (0x0389AC, i_0389ac),
    (0x0389B0, i_0389b0),
    (0x0389B4, i_0389b4),
    (0x0389B8, i_0389b8),
    (0x0389BA, i_0389ba),
    (0x0389BE, i_0389be),
    (0x0389C2, i_0389c2),
    (0x0389C6, i_0389c6),
    (0x0389CA, i_0389ca),
    (0x0389CC, i_0389cc),
    (0x0389D0, i_0389d0),
    (0x0389D4, i_0389d4),
    (0x0389D8, i_0389d8),
    (0x0389DC, i_0389dc),
    (0x0389E0, i_0389e0),
    (0x0389E2, i_0389e2),
    (0x0389E6, i_0389e6),
    (0x0389EA, i_0389ea),
    (0x0389EE, i_0389ee),
    (0x0389F2, i_0389f2),
    (0x0389F4, i_0389f4),
    (0x0389F8, i_0389f8),
    (0x0389FC, i_0389fc),
    (0x0389FE, i_0389fe),
    (0x038A02, i_038a02),
    (0x038A08, i_038a08),
    (0x038A0C, i_038a0c),
    (0x038A10, i_038a10),
    (0x038A12, i_038a12),
    (0x038A18, i_038a18),
    (0x038A1C, i_038a1c),
    (0x038A20, i_038a20),
    (0x038A22, i_038a22),
    (0x038A26, i_038a26),
    (0x038A28, i_038a28),
    (0x038A2E, i_038a2e),
    (0x038A34, i_038a34),
    (0x038A38, i_038a38),
    (0x038A3C, i_038a3c),
    (0x038A42, i_038a42),
    (0x038A46, i_038a46),
    (0x038A4A, i_038a4a),
    (0x038A4E, i_038a4e),
    (0x038A52, i_038a52),
    (0x038A5A, i_038a5a),
    (0x038A60, i_038a60),
    (0x038A64, i_038a64),
    (0x038A6A, i_038a6a),
    (0x038A70, i_038a70),
    (0x038A76, i_038a76),
    (0x038A7C, i_038a7c),
    (0x038A82, i_038a82),
    (0x038A88, i_038a88),
    (0x038A8C, i_038a8c),
    (0x038A92, i_038a92),
    (0x038A98, i_038a98),
    (0x038A9E, i_038a9e),
    (0x038AA4, i_038aa4),
    (0x038AAA, i_038aaa),
    (0x038AB0, i_038ab0),
    (0x038AB4, i_038ab4),
    (0x038AB8, i_038ab8),
    (0x038ABC, i_038abc),
    (0x038ABE, i_038abe),
    (0x038AC4, i_038ac4),
    (0x038AC8, i_038ac8),
    (0x038ACC, i_038acc),
    (0x038AD2, i_038ad2),
    (0x038AD6, i_038ad6),
    (0x038ADA, i_038ada),
    (0x038ADE, i_038ade),
    (0x038AE0, i_038ae0),
    (0x038AE4, i_038ae4),
    (0x038AE6, i_038ae6),
    (0x038AEC, i_038aec),
    (0x038AF2, i_038af2),
    (0x038AF8, i_038af8),
    (0x038AFE, i_038afe),
    (0x038B04, i_038b04),
    (0x038B0A, i_038b0a),
    (0x038B10, i_038b10),
    (0x038B16, i_038b16),
    (0x038B1C, i_038b1c),
    (0x038B22, i_038b22),
    (0x038B28, i_038b28),
    (0x038B2C, i_038b2c),
    (0x038B30, i_038b30),
    (0x038B38, i_038b38),
    (0x038B3E, i_038b3e),
    (0x038B42, i_038b42),
    (0x038B46, i_038b46),
    (0x038B48, i_038b48),
    (0x038B4E, i_038b4e),
    (0x038B52, i_038b52),
    (0x038B56, i_038b56),
    (0x038B5C, i_038b5c),
    (0x038B60, i_038b60),
    (0x038B64, i_038b64),
    (0x038B68, i_038b68),
    (0x038B6A, i_038b6a),
    (0x038B70, i_038b70),
    (0x038B74, i_038b74),
    (0x038B7A, i_038b7a),
    (0x038B80, i_038b80),
    (0x038B84, i_038b84),
    (0x038B88, i_038b88),
    (0x038B8A, i_038b8a),
    (0x038B8E, i_038b8e),
    (0x038B92, i_038b92),
    (0x038B96, i_038b96),
    (0x038B9A, i_038b9a),
    (0x038B9E, i_038b9e),
    (0x038BA0, i_038ba0),
    (0x038BA4, i_038ba4),
    (0x038BA8, i_038ba8),
    (0x038BAC, i_038bac),
    (0x038BB0, i_038bb0),
    (0x038BB2, i_038bb2),
    (0x038BB6, i_038bb6),
    (0x038BBA, i_038bba),
    (0x038BBC, i_038bbc),
    (0x038BC0, i_038bc0),
    (0x038BC4, i_038bc4),
    (0x038BC8, i_038bc8),
    (0x038BCC, i_038bcc),
    (0x038BCE, i_038bce),
    (0x038BD4, i_038bd4),
    (0x038BD8, i_038bd8),
    (0x038BDC, i_038bdc),
    (0x038BDE, i_038bde),
    (0x038BE2, i_038be2),
    (0x038BE4, i_038be4),
    (0x038BE8, i_038be8),
    (0x038BEE, i_038bee),
    (0x038BF2, i_038bf2),
    (0x038BF6, i_038bf6),
    (0x038BFA, i_038bfa),
    (0x038BFE, i_038bfe),
    (0x038C04, i_038c04),
    (0x038C08, i_038c08),
    (0x038C0E, i_038c0e),
    (0x038C12, i_038c12),
    (0x038C18, i_038c18),
    (0x038C1E, i_038c1e),
    (0x038C24, i_038c24),
    (0x038C2A, i_038c2a),
    (0x038C30, i_038c30),
    (0x038C36, i_038c36),
    (0x038C3C, i_038c3c),
    (0x038C40, i_038c40),
    (0x038C42, i_038c42),
    (0x038C48, i_038c48),
    (0x038C4C, i_038c4c),
    (0x038C50, i_038c50),
    (0x038C52, i_038c52),
    (0x038C58, i_038c58),
    (0x038C5E, i_038c5e),
    (0x038C64, i_038c64),
    (0x038C68, i_038c68),
    (0x038C6C, i_038c6c),
    (0x038C6E, i_038c6e),
    (0x038C70, i_038c70),
    (0x038C76, i_038c76),
    (0x038C7C, i_038c7c),
    (0x038C82, i_038c82),
    (0x038C86, i_038c86),
    (0x038C8C, i_038c8c),
    (0x038C90, i_038c90),
    (0x038C96, i_038c96),
    (0x038C9C, i_038c9c),
    (0x038CA2, i_038ca2),
    (0x038CA8, i_038ca8),
    (0x038CAE, i_038cae),
    (0x038CB2, i_038cb2),
    (0x038CB4, i_038cb4),
    (0x038CBA, i_038cba),
    (0x038CC0, i_038cc0),
    (0x038CC6, i_038cc6),
    (0x038CCA, i_038cca),
    (0x038CCE, i_038cce),
    (0x038CD0, i_038cd0),
    (0x038CD6, i_038cd6),
    (0x038CDC, i_038cdc),
    (0x038CE2, i_038ce2),
    (0x038CE6, i_038ce6),
    (0x038CEA, i_038cea),
    (0x038CEC, i_038cec),
    (0x038CEE, i_038cee),
    (0x038CF4, i_038cf4),
    (0x038CF6, i_038cf6),
    (0x038CFA, i_038cfa),
    (0x038D00, i_038d00),
    (0x038D04, i_038d04),
    (0x038D08, i_038d08),
    (0x038D0C, i_038d0c),
    (0x038D0E, i_038d0e),
    (0x038D14, i_038d14),
    (0x038D1A, i_038d1a),
    (0x038D1C, i_038d1c),
    (0x038D20, i_038d20),
    (0x038D26, i_038d26),
    (0x038D28, i_038d28),
    (0x038D2C, i_038d2c),
    (0x038D32, i_038d32),
    (0x038D36, i_038d36),
    (0x038D3C, i_038d3c),
    (0x038D42, i_038d42),
    (0x038D48, i_038d48),
    (0x038D4C, i_038d4c),
    (0x038D50, i_038d50),
    (0x038D52, i_038d52),
    (0x038D56, i_038d56),
    (0x038D5C, i_038d5c),
    (0x038D60, i_038d60),
    (0x038D64, i_038d64),
    (0x038D66, i_038d66),
    (0x038D6C, i_038d6c),
    (0x038D70, i_038d70),
    (0x038D74, i_038d74),
    (0x038D76, i_038d76),
    (0x038D7C, i_038d7c),
    (0x038D80, i_038d80),
    (0x038D84, i_038d84),
    (0x038D86, i_038d86),
    (0x038D88, i_038d88),
    (0x038D8A, i_038d8a),
    (0x038D8E, i_038d8e),
    (0x038D94, i_038d94),
    (0x038D96, i_038d96),
    (0x038D9A, i_038d9a),
    (0x038DA0, i_038da0),
    (0x038DA4, i_038da4),
    (0x038DAA, i_038daa),
    (0x038DAE, i_038dae),
    (0x038DB0, i_038db0),
    (0x038DB6, i_038db6),
    (0x038DBA, i_038dba),
    (0x038DC0, i_038dc0),
    (0x038DC4, i_038dc4),
    (0x038DC8, i_038dc8),
    (0x038DCA, i_038dca),
    (0x038DD0, i_038dd0),
    (0x038DD4, i_038dd4),
    (0x038DD8, i_038dd8),
    (0x038DDA, i_038dda),
    (0x038DDE, i_038dde),
    (0x038DE4, i_038de4),
    (0x038DE8, i_038de8),
    (0x038DEC, i_038dec),
    (0x038DEE, i_038dee),
    (0x038DF4, i_038df4),
    (0x038DF8, i_038df8),
    (0x038DFC, i_038dfc),
    (0x038DFE, i_038dfe),
    (0x038E04, i_038e04),
    (0x038E08, i_038e08),
    (0x038E0C, i_038e0c),
    (0x038E0E, i_038e0e),
    (0x038E10, i_038e10),
    (0x038E14, i_038e14),
    (0x038E18, i_038e18),
    (0x038E1C, i_038e1c),
    (0x038E20, i_038e20),
    (0x038E26, i_038e26),
    (0x038E2C, i_038e2c),
    (0x038E30, i_038e30),
    (0x038E32, i_038e32),
    (0x038E38, i_038e38),
    (0x038E3E, i_038e3e),
    (0x038E42, i_038e42),
    (0x038E48, i_038e48),
    (0x038E4A, i_038e4a),
    (0x038E4E, i_038e4e),
    (0x038E54, i_038e54),
    (0x038E5A, i_038e5a),
    (0x038E5E, i_038e5e),
    (0x038E64, i_038e64),
    (0x038E68, i_038e68),
    (0x038E6A, i_038e6a),
    (0x038E70, i_038e70),
    (0x038E76, i_038e76),
    (0x038E7A, i_038e7a),
    (0x038E80, i_038e80),
    (0x038E84, i_038e84),
    (0x038E88, i_038e88),
    (0x038E8E, i_038e8e),
    (0x038E92, i_038e92),
    (0x038E98, i_038e98),
    (0x038E9A, i_038e9a),
    (0x038E9E, i_038e9e),
    (0x038EA2, i_038ea2),
    (0x038EA6, i_038ea6),
    (0x038EAA, i_038eaa),
    (0x038EAE, i_038eae),
    (0x038EB0, i_038eb0),
    (0x038EB2, i_038eb2),
    (0x038EB4, i_038eb4),
    (0x038EB6, i_038eb6),
    (0x038EB8, i_038eb8),
    (0x038EBA, i_038eba),
    (0x038EBE, i_038ebe),
    (0x038EC2, i_038ec2),
    (0x038EC6, i_038ec6),
    (0x038F12, i_038f12),
    (0x038F14, i_038f14),
    (0x038F18, i_038f18),
    (0x038F1E, i_038f1e),
    (0x038F24, i_038f24),
    (0x038F2A, i_038f2a),
    (0x038F2E, i_038f2e),
    (0x038F34, i_038f34),
    (0x038F3A, i_038f3a),
    (0x038F40, i_038f40),
    (0x038F46, i_038f46),
    (0x038F48, i_038f48),
    (0x038F4C, i_038f4c),
    (0x038F52, i_038f52),
    (0x038F58, i_038f58),
    (0x038F5E, i_038f5e),
    (0x038F62, i_038f62),
    (0x038F68, i_038f68),
    (0x038F6E, i_038f6e),
    (0x038F74, i_038f74),
    (0x038F7A, i_038f7a),
    (0x039010, i_039010),
    (0x039014, i_039014),
    (0x039018, i_039018),
    (0x03901E, i_03901e),
    (0x039022, i_039022),
    (0x039028, i_039028),
    (0x03902C, i_03902c),
    (0x039030, i_039030),
    (0x039034, i_039034),
    (0x039038, i_039038),
    (0x03903E, i_03903e),
    (0x039042, i_039042),
    (0x039048, i_039048),
    (0x03904C, i_03904c),
    (0x039050, i_039050),
    (0x039054, i_039054),
    (0x039058, i_039058),
    (0x03905E, i_03905e),
    (0x039062, i_039062),
    (0x039068, i_039068),
    (0x03906C, i_03906c),
    (0x039070, i_039070),
    (0x039074, i_039074),
    (0x039078, i_039078),
    (0x03907E, i_03907e),
    (0x039082, i_039082),
    (0x039088, i_039088),
    (0x03908C, i_03908c),
    (0x039090, i_039090),
    (0x039094, i_039094),
    (0x039098, i_039098),
    (0x03909E, i_03909e),
    (0x0390A2, i_0390a2),
    (0x0390A8, i_0390a8),
    (0x0390AE, i_0390ae),
    (0x0390B4, i_0390b4),
    (0x0390B8, i_0390b8),
    (0x0390BE, i_0390be),
    (0x0390C2, i_0390c2),
    (0x0390C6, i_0390c6),
    (0x0390CC, i_0390cc),
    (0x0390D0, i_0390d0),
    (0x0390D2, i_0390d2),
    (0x0390D6, i_0390d6),
    (0x0390DA, i_0390da),
    (0x0390DC, i_0390dc),
    (0x0390E0, i_0390e0),
    (0x0390E6, i_0390e6),
    (0x0390EA, i_0390ea),
    (0x0390EE, i_0390ee),
    (0x0390F0, i_0390f0),
    (0x0390F6, i_0390f6),
    (0x0390FC, i_0390fc),
    (0x039102, i_039102),
    (0x039106, i_039106),
    (0x03910C, i_03910c),
    (0x039110, i_039110),
    (0x039116, i_039116),
    (0x039118, i_039118),
    (0x03911C, i_03911c),
    (0x039120, i_039120),
    (0x039124, i_039124),
    (0x039126, i_039126),
    (0x03912A, i_03912a),
    (0x03912E, i_03912e),
    (0x039132, i_039132),
    (0x039138, i_039138),
    (0x03913C, i_03913c),
    (0x039140, i_039140),
    (0x039146, i_039146),
    (0x039148, i_039148),
    (0x03914C, i_03914c),
    (0x039152, i_039152),
    (0x039158, i_039158),
    (0x03915E, i_03915e),
    (0x039164, i_039164),
    (0x039168, i_039168),
    (0x03916A, i_03916a),
    (0x039170, i_039170),
    (0x039174, i_039174),
    (0x039178, i_039178),
    (0x03917C, i_03917c),
    (0x039180, i_039180),
    (0x039184, i_039184),
    (0x03918A, i_03918a),
    (0x03918E, i_03918e),
    (0x039192, i_039192),
    (0x039194, i_039194),
    (0x039198, i_039198),
    (0x03919E, i_03919e),
    (0x0391A2, i_0391a2),
    (0x0391A6, i_0391a6),
    (0x0391A8, i_0391a8),
    (0x0391AA, i_0391aa),
    (0x0391B0, i_0391b0),
    (0x0391B6, i_0391b6),
    (0x0391BC, i_0391bc),
    (0x0391C0, i_0391c0),
    (0x0391C2, i_0391c2),
    (0x0391C8, i_0391c8),
    (0x0391CC, i_0391cc),
    (0x0391D0, i_0391d0),
    (0x0391D4, i_0391d4),
    (0x0391D8, i_0391d8),
    (0x0391DC, i_0391dc),
    (0x0391E2, i_0391e2),
    (0x0391E6, i_0391e6),
    (0x0391EC, i_0391ec),
    (0x0391EE, i_0391ee),
    (0x0391F2, i_0391f2),
    (0x0391F8, i_0391f8),
    (0x0391FE, i_0391fe),
    (0x039204, i_039204),
    (0x03920A, i_03920a),
    (0x03920E, i_03920e),
    (0x039210, i_039210),
    (0x039214, i_039214),
    (0x039218, i_039218),
    (0x03921E, i_03921e),
    (0x039224, i_039224),
    (0x03922A, i_03922a),
    (0x03922E, i_03922e),
    (0x039230, i_039230),
    (0x039234, i_039234),
    (0x039238, i_039238),
    (0x03923C, i_03923c),
    (0x039240, i_039240),
    (0x039244, i_039244),
    (0x039248, i_039248),
    (0x03924A, i_03924a),
    (0x03924E, i_03924e),
    (0x0392F8, i_0392f8),
    (0x0392FE, i_0392fe),
    (0x039304, i_039304),
    (0x03930A, i_03930a),
    (0x039310, i_039310),
    (0x039316, i_039316),
    (0x03931A, i_03931a),
    (0x03931E, i_03931e),
    (0x039322, i_039322),
    (0x039326, i_039326),
    (0x03932A, i_03932a),
    (0x039330, i_039330),
    (0x039334, i_039334),
    (0x039338, i_039338),
    (0x03933A, i_03933a),
    (0x03933E, i_03933e),
    (0x039340, i_039340),
    (0x039344, i_039344),
    (0x03934A, i_03934a),
    (0x03934E, i_03934e),
    (0x039350, i_039350),
    (0x039352, i_039352),
    (0x039354, i_039354),
    (0x03935A, i_03935a),
    (0x03935E, i_03935e),
    (0x039362, i_039362),
    (0x039366, i_039366),
    (0x03936A, i_03936a),
    (0x03936C, i_03936c),
    (0x039370, i_039370),
    (0x039376, i_039376),
    (0x03937A, i_03937a),
    (0x03937C, i_03937c),
    (0x039382, i_039382),
    (0x039386, i_039386),
    (0x03938A, i_03938a),
    (0x03938E, i_03938e),
    (0x039392, i_039392),
    (0x039396, i_039396),
    (0x03939A, i_03939a),
    (0x03939E, i_03939e),
    (0x0393A2, i_0393a2),
    (0x0393A6, i_0393a6),
    (0x0393AA, i_0393aa),
    (0x0393B0, i_0393b0),
    (0x0393B4, i_0393b4),
    (0x0393BA, i_0393ba),
    (0x0393C0, i_0393c0),
    (0x0393C6, i_0393c6),
    (0x0393CC, i_0393cc),
    (0x0393CE, i_0393ce),
    (0x0393D4, i_0393d4),
    (0x0393DA, i_0393da),
    (0x0393DE, i_0393de),
    (0x0393E4, i_0393e4),
    (0x0393E8, i_0393e8),
    (0x0393EA, i_0393ea),
    (0x0393F0, i_0393f0),
    (0x0393F4, i_0393f4),
    (0x0393FA, i_0393fa),
    (0x0393FC, i_0393fc),
    (0x039400, i_039400),
    (0x039404, i_039404),
    (0x039408, i_039408),
    (0x03940A, i_03940a),
    (0x03940E, i_03940e),
    (0x039410, i_039410),
    (0x039416, i_039416),
    (0x03941A, i_03941a),
    (0x03941E, i_03941e),
    (0x039422, i_039422),
    (0x039428, i_039428),
    (0x03942C, i_03942c),
    (0x03942E, i_03942e),
    (0x039434, i_039434),
    (0x03943A, i_03943a),
    (0x039440, i_039440),
    (0x039444, i_039444),
    (0x039448, i_039448),
    (0x03944E, i_03944e),
    (0x039450, i_039450),
    (0x039456, i_039456),
    (0x03945A, i_03945a),
    (0x03945E, i_03945e),
    (0x039460, i_039460),
    (0x039466, i_039466),
    (0x03946A, i_03946a),
    (0x03946E, i_03946e),
    (0x039472, i_039472),
    (0x039478, i_039478),
    (0x0394A4, i_0394a4),
    (0x0394A8, i_0394a8),
    (0x0394AC, i_0394ac),
    (0x0394B2, i_0394b2),
    (0x0394B6, i_0394b6),
    (0x0394BC, i_0394bc),
    (0x0394C0, i_0394c0),
    (0x0394C6, i_0394c6),
    (0x0394CC, i_0394cc),
    (0x0394D2, i_0394d2),
    (0x0394D6, i_0394d6),
    (0x0394DC, i_0394dc),
    (0x0394E2, i_0394e2),
    (0x0394E6, i_0394e6),
    (0x0394EA, i_0394ea),
    (0x0394F0, i_0394f0),
    (0x0394F4, i_0394f4),
    (0x0394FA, i_0394fa),
    (0x0394FE, i_0394fe),
    (0x039504, i_039504),
    (0x03950A, i_03950a),
    (0x039510, i_039510),
    (0x039514, i_039514),
    (0x03951A, i_03951a),
    (0x039520, i_039520),
    (0x039526, i_039526),
    (0x03952C, i_03952c),
    (0x039530, i_039530),
    (0x039532, i_039532),
    (0x039536, i_039536),
    (0x03953A, i_03953a),
    (0x03953E, i_03953e),
    (0x039542, i_039542),
    (0x039546, i_039546),
    (0x03954A, i_03954a),
    (0x03954E, i_03954e),
    (0x039552, i_039552),
    (0x039556, i_039556),
    (0x03955A, i_03955a),
    (0x03955E, i_03955e),
    (0x039562, i_039562),
    (0x039566, i_039566),
    (0x03956A, i_03956a),
    (0x03956E, i_03956e),
    (0x039572, i_039572),
    (0x039576, i_039576),
    (0x03957A, i_03957a),
    (0x03957E, i_03957e),
    (0x039582, i_039582),
    (0x039586, i_039586),
    (0x03958A, i_03958a),
    (0x03958E, i_03958e),
    (0x039592, i_039592),
    (0x039596, i_039596),
    (0x03959A, i_03959a),
    (0x03959E, i_03959e),
    (0x0395A2, i_0395a2),
    (0x0395A6, i_0395a6),
    (0x0395AA, i_0395aa),
    (0x0395AE, i_0395ae),
    (0x0395B2, i_0395b2),
    (0x0395B6, i_0395b6),
    (0x0395BA, i_0395ba),
    (0x0395BE, i_0395be),
    (0x0395C2, i_0395c2),
    (0x0395C6, i_0395c6),
    (0x0395CA, i_0395ca),
    (0x0395CE, i_0395ce),
    (0x0395D2, i_0395d2),
    (0x0395D6, i_0395d6),
    (0x0395DA, i_0395da),
    (0x0395DE, i_0395de),
    (0x0395E0, i_0395e0),
    (0x0395E2, i_0395e2),
    (0x0395E6, i_0395e6),
    (0x0395E8, i_0395e8),
    (0x0395EA, i_0395ea),
    (0x0395EE, i_0395ee),
    (0x0395F2, i_0395f2),
    (0x0395F8, i_0395f8),
    (0x0395FC, i_0395fc),
    (0x039600, i_039600),
    (0x039602, i_039602),
    (0x039606, i_039606),
    (0x03960A, i_03960a),
    (0x03960E, i_03960e),
    (0x039612, i_039612),
    (0x039616, i_039616),
    (0x03961A, i_03961a),
    (0x03961C, i_03961c),
    (0x039620, i_039620),
    (0x039626, i_039626),
    (0x03962A, i_03962a),
];
