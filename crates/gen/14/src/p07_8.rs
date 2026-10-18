// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p07_8) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $07FADE: lea.l $29c77c.l, a0
pub fn i_07fade(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAE4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29C77C;
    20
}
/// $07FAE4: jsr $28cd4.l
pub fn i_07fae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAEA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FAEA);
    cpu.pc = 0x28CD4;
    16
}
/// $07FAEA: lea.l $7faf0(pc), a1
pub fn i_07faea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FAF0;
    16
}
/// $07FAEE: move.l a1, (a6)
pub fn i_07faee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FAF0: jsr $7fe66(pc)
pub fn i_07faf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAF4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FAF4);
    cpu.pc = 0x7FE66;
    16
}
/// $07FAF4: jsr $28d70.l
pub fn i_07faf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FAFA);
    cpu.pc = 0x28D70;
    16
}
/// $07FAFA: bcc.w $7fb14
pub fn i_07fafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FAFE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FB14; return 10; }
    8
}
/// $07FAFE: movea.l $c(a6), a0
pub fn i_07fafe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB02;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FB02: move.w $24(a0), d0
pub fn i_07fb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB06;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FB06: subi.w #$c, d0
pub fn i_07fb06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB0A;
    cpu.ir = 0x440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0xC, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FB0A: move.w d0, $70(a6)
pub fn i_07fb0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB0E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FB0E: lea.l $7f33e(pc), a1
pub fn i_07fb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB12;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F33E;
    16
}
/// $07FB12: move.l a1, (a6)
pub fn i_07fb12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB14;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB14: jsr $80054(pc)
pub fn i_07fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB18;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB18);
    cpu.pc = 0x80054;
    16
}
/// $07FB18: jsr $7fde0(pc)
pub fn i_07fb18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB1C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB1C);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FB1C: bcc.w $7fb26
pub fn i_07fb1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB20;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FB26; return 10; }
    8
}
// ===== SetTaskHandler_07fb20  ($07FB20, 8 B, kernel/task_handlers.c)
/// $07FB20: lea.l $7fdbc(pc), a1
pub fn i_07fb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB24;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FB24: move.l a1, (a6)
pub fn i_07fb24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB26;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB26: rts
pub fn i_07fb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB28;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Idle_07fb28  ($07FB28, 62 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07FB28: jsr $7ed6a(pc)
pub fn i_07fb28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB2C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB2C);
    cpu.pc = 0x7ED6A;
    16
}
/// $07FB2C: lea.l $29b816.l, a0
pub fn i_07fb2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB32;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29B816;
    20
}
/// $07FB32: jsr $28cd4.l
pub fn i_07fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FB38);
    cpu.pc = 0x28CD4;
    16
}
/// $07FB38: lea.l $7fb3e(pc), a1
pub fn i_07fb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB3C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FB3E;
    16
}
/// $07FB3C: move.l a1, (a6)
pub fn i_07fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB3E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB3E: jsr $7fe66(pc)
pub fn i_07fb3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB42;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB42);
    cpu.pc = 0x7FE66;
    16
}
/// $07FB42: jsr $28d70.l
pub fn i_07fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB48;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FB48);
    cpu.pc = 0x28D70;
    16
}
/// $07FB48: tst.w $90(a6)
pub fn i_07fb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB4C;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FB4C: bne.w $7fb56
pub fn i_07fb4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB50;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FB56; return 10; }
    8
}
/// $07FB50: lea.l $7fb6e(pc), a1
pub fn i_07fb50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB54;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FB6E;
    16
}
/// $07FB54: move.l a1, (a6)
pub fn i_07fb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB56;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB56: subq.w #$1, $90(a6)
pub fn i_07fb56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB5A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x90);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FB5A: jsr $80054(pc)
pub fn i_07fb5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB5E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB5E);
    cpu.pc = 0x80054;
    16
}
/// $07FB5E: jsr $7fde0(pc)
pub fn i_07fb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB62;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FB62);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FB62: bcc.w $7fb6c
pub fn i_07fb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB66;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FB6C; return 10; }
    8
}
// ===== SetTaskHandler_07fb66  ($07FB66, 8 B, kernel/task_handlers.c)
/// $07FB66: lea.l $7fdbc(pc), a1
pub fn i_07fb66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB6A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FB6A: move.l a1, (a6)
pub fn i_07fb6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB6C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FB6C: rts
pub fn i_07fb6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB6E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Carrier_Gunner_Decide_07fb6e  ($07FB6E, 92 B, enemies/bosses/boss2_crab_carrier_07a00a.s)
/// $07FB6E: jsr $5e0d4.l
pub fn i_07fb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FB74);
    cpu.pc = 0x5E0D4;
    16
}
/// $07FB74: move.w $22(a0), d0
pub fn i_07fb74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB78;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FB78: cmp.w $22(a6), d0
pub fn i_07fb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB7C;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07FB7C: ble.w $7fb8e
pub fn i_07fb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB80;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7FB8E; return 10; }
    8
}
/// $07FB80: btst.b #$0, $3a(a6)
pub fn i_07fb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB86;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07FB86: bne.w $7fbd2
pub fn i_07fb86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB8A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FBD2; return 10; }
    8
}
/// $07FB8A: bra.w $7fb98
pub fn i_07fb8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB8E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7FB98;
    10
}
/// $07FB8E: btst.b #$0, $3a(a6)
pub fn i_07fb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB94;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $07FB94: beq.w $7fbd2
pub fn i_07fb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FBD2; return 10; }
    8
}
/// $07FB98: lea.l $29bf34.l, a0
pub fn i_07fb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FB9E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BF34;
    20
}
/// $07FB9E: jsr $28cd4.l
pub fn i_07fb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBA4);
    cpu.pc = 0x28CD4;
    16
}
/// $07FBA4: lea.l $7fbaa(pc), a1
pub fn i_07fba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBA8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FBAA;
    16
}
/// $07FBA8: move.l a1, (a6)
pub fn i_07fba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBAA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FBAA: jsr $7fe66(pc)
pub fn i_07fbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBAE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FBAE);
    cpu.pc = 0x7FE66;
    16
}
/// $07FBAE: jsr $28d70.l
pub fn i_07fbae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBB4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBB4);
    cpu.pc = 0x28D70;
    16
}
/// $07FBB4: bcc.w $7fbbe
pub fn i_07fbb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FBBE; return 10; }
    8
}
/// $07FBB8: lea.l $7fbd2(pc), a1
pub fn i_07fbb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBBC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FBD2;
    16
}
/// $07FBBC: move.l a1, (a6)
pub fn i_07fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBBE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FBBE: jsr $80054(pc)
pub fn i_07fbbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBC2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FBC2);
    cpu.pc = 0x80054;
    16
}
/// $07FBC2: jsr $7fde0(pc)
pub fn i_07fbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FBC6);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FBC6: bcc.w $7fbd0
pub fn i_07fbc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBCA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FBD0; return 10; }
    8
}
// ===== SetTaskHandler_07fbca  ($07FBCA, 8 B, kernel/task_handlers.c)
/// $07FBCA: lea.l $7fdbc(pc), a1
pub fn i_07fbca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FBCE: move.l a1, (a6)
pub fn i_07fbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FBD0: rts
pub fn i_07fbd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBD2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_EscortInit_07fbd2  ($07FBD2, 64 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FBD2: jsr $5e9b6.l
pub fn i_07fbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBD8);
    cpu.pc = 0x5E9B6;
    16
}
/// $07FBD8: andi.w #$3, d0
pub fn i_07fbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBDC;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FBDC: move.b d0, $5c(a6)
pub fn i_07fbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBE0;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07FBE0: lea.l $29ba70.l, a0
pub fn i_07fbe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBE6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x29BA70;
    20
}
/// $07FBE6: jsr $28cd4.l
pub fn i_07fbe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBEC);
    cpu.pc = 0x28CD4;
    16
}
/// $07FBEC: lea.l $7fbf2(pc), a1
pub fn i_07fbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FBF2;
    16
}
/// $07FBF0: move.l a1, (a6)
pub fn i_07fbf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FBF2: jsr $7fe66(pc)
pub fn i_07fbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBF6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FBF6);
    cpu.pc = 0x7FE66;
    16
}
/// $07FBF6: jsr $28d70.l
pub fn i_07fbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FBFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FBFC);
    cpu.pc = 0x28D70;
    16
}
/// $07FBFC: bcc.w $7fc06
pub fn i_07fbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FC06; return 10; }
    8
}
/// $07FC00: lea.l $7fb28(pc), a1
pub fn i_07fc00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FB28;
    16
}
/// $07FC04: move.l a1, (a6)
pub fn i_07fc04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FC06: jsr $80054(pc)
pub fn i_07fc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC0A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FC0A);
    cpu.pc = 0x80054;
    16
}
/// $07FC0A: jsr $7fde0(pc)
pub fn i_07fc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC0E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FC0E);
    cpu.pc = 0x7FDE0;
    16
}
/// $07FC0E: bcc.w $7fc18
pub fn i_07fc0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FC18; return 10; }
    8
}
// ===== SetTaskHandler_07fc12  ($07FC12, 8 B, kernel/task_handlers.c)
/// $07FC12: lea.l $7fdbc(pc), a1
pub fn i_07fc12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FC16: move.l a1, (a6)
pub fn i_07fc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FC18: rts
pub fn i_07fc18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_HatchRow3Spawn_07fc1a  ($07FC1A, 294 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FC1A: lea.l $77fd6.l, a1
pub fn i_07fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC20;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07FC20: jsr $4ae.l
pub fn i_07fc20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FC26);
    cpu.pc = 0x4AE;
    16
}
/// $07FC26: jsr $5dd02.l
pub fn i_07fc26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC2C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FC2C);
    cpu.pc = 0x5DD02;
    16
}
/// $07FC2C: addi.w #$28, $38(a0)
pub fn i_07fc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC32;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FC32: subi.w #$30, $22(a0)
pub fn i_07fc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC38;
    cpu.ir = 0x468;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x30, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FC38: move.w #$ffb8, $70(a6)
pub fn i_07fc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFB8;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FC3E: lea.l $2e1dca.l, a0
pub fn i_07fc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC44;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E1DCA;
    20
}
/// $07FC44: jsr $28cd4.l
pub fn i_07fc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FC4A);
    cpu.pc = 0x28CD4;
    16
}
/// $07FC4A: lea.l $7fcbe(pc), a1
pub fn i_07fc4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC4E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FCBE;
    16
}
/// $07FC4E: move.l a1, (a6)
pub fn i_07fc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC50;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FC50: bra.w $7fcbe
pub fn i_07fc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FC54;
    cpu.ir = 0x6000;
    cpu.pc = 0x7FCBE;
    10
}
/// $07FCBE: movea.l $c(a6), a0
pub fn i_07fcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCC2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FCC2: move.w $22(a0), $22(a6)
pub fn i_07fcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCC8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FCC8: move.w $24(a0), $24(a6)
pub fn i_07fcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCCE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FCCE: jsr $28d70.l
pub fn i_07fcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCD4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FCD4);
    cpu.pc = 0x28D70;
    16
}
/// $07FCD4: bclr.b #$3, $13(a6)
pub fn i_07fcd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCDA;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $07FCDA: movea.l $c(a6), a0
pub fn i_07fcda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCDE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FCDE: cmpi.b #$3, $20(a0)
pub fn i_07fcde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCE4;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $07FCE4: bne.w $7fd38
pub fn i_07fce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCE8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FD38; return 10; }
    8
}
/// $07FCE8: tst.w $70(a6)
pub fn i_07fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCEC;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FCEC: bne.w $7fd2a
pub fn i_07fcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCF0;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FD2A; return 10; }
    8
}
/// $07FCF0: lea.l $2e2ae8.l, a1
pub fn i_07fcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCF6;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E2AE8;
    20
}
/// $07FCF6: jsr $77c7e.l
pub fn i_07fcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FCFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FCFC);
    cpu.pc = 0x77C7E;
    16
}
/// $07FCFC: addi.w #$28, $38(a0)
pub fn i_07fcfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD02;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FD02: lea.l $2e2afa.l, a1
pub fn i_07fd02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD08;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E2AFA;
    20
}
/// $07FD08: jsr $77c7e.l
pub fn i_07fd08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD0E);
    cpu.pc = 0x77C7E;
    16
}
/// $07FD0E: addi.w #$28, $38(a0)
pub fn i_07fd0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD14;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FD14: lea.l $7f22a(pc), a1
pub fn i_07fd14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD18;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7F22A;
    16
}
/// $07FD18: jsr $4ae.l
pub fn i_07fd18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD1E);
    cpu.pc = 0x4AE;
    16
}
/// $07FD1E: jsr $5dd02.l
pub fn i_07fd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD24;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD24);
    cpu.pc = 0x5DD02;
    16
}
/// $07FD24: addi.w #$c000, $38(a0)
pub fn i_07fd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD2A;
    cpu.ir = 0x668;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0xC000, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FD2A: lea.l $ffff.w, a0
pub fn i_07fd2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD2E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07FD2E: move.l a0, $48(a6)
pub fn i_07fd2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD32;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07FD32: lea.l $7fd48(pc), a1
pub fn i_07fd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FD48;
    16
}
/// $07FD36: move.l a1, (a6)
pub fn i_07fd36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FD38: jsr $7fdc4(pc)
pub fn i_07fd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD3C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FD3C);
    cpu.pc = 0x7FDC4;
    16
}
/// $07FD3C: bcc.w $7fd46
pub fn i_07fd3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD40;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FD46; return 10; }
    8
}
// ===== SetTaskHandler_07fd40  ($07FD40, 8 B, kernel/task_handlers.c)
/// $07FD40: lea.l $7fdbc(pc), a1
pub fn i_07fd40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD44;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FD44: move.l a1, (a6)
pub fn i_07fd44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD46;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FD46: rts
pub fn i_07fd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD48;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_HatchDeployed_07fd48  ($07FD48, 88 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FD48: movea.l $c(a6), a0
pub fn i_07fd48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD4C;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FD4C: move.w $22(a0), $22(a6)
pub fn i_07fd4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD52;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FD52: move.w $24(a0), $24(a6)
pub fn i_07fd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD58;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FD58: jsr $28d70.l
pub fn i_07fd58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD5E);
    cpu.pc = 0x28D70;
    16
}
/// $07FD5E: movea.l $c(a6), a0
pub fn i_07fd5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD62;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FD62: cmpi.b #$1, $21(a0)
pub fn i_07fd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD68;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $07FD68: bne.w $7fd98
pub fn i_07fd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD6C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FD98; return 10; }
    8
}
/// $07FD6C: move.w $70(a6), d0
pub fn i_07fd6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD70;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FD70: add.w d0, $22(a6)
pub fn i_07fd70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD74;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FD74: addi.w #$28, $38(a6)
pub fn i_07fd74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD7A;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FD7A: lea.l $ffff.w, a0
pub fn i_07fd7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD7E;
    cpu.ir = 0x41F8;
    cpu.a[0] = 0xFFFFFFFF;
    16
}
/// $07FD7E: move.l a0, $48(a6)
pub fn i_07fd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD82;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07FD82: lea.l $77fd6.l, a1
pub fn i_07fd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD88;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $07FD88: move.l a1, (a6)
pub fn i_07fd88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FD8A: tst.w $70(a6)
pub fn i_07fd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD8E;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FD8E: bne.w $7fd98
pub fn i_07fd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD92;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FD98; return 10; }
    8
}
/// $07FD92: jsr $434dc.l
pub fn i_07fd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FD98);
    cpu.pc = 0x434DC;
    16
}
/// $07FD98: jsr $7fdc4(pc)
pub fn i_07fd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FD9C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x7FD9C);
    cpu.pc = 0x7FDC4;
    16
}
/// $07FD9C: bcc.w $7fda6
pub fn i_07fd9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDA0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FDA6; return 10; }
    8
}
// ===== SetTaskHandler_07fda0  ($07FDA0, 8 B, kernel/task_handlers.c)
/// $07FDA0: lea.l $7fdbc(pc), a1
pub fn i_07fda0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDA4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x7FDBC;
    16
}
/// $07FDA4: move.l a1, (a6)
pub fn i_07fda4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDA6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $07FDA6: rts
pub fn i_07fda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDA8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_LeaderNotify_07fda8  ($07FDA8, 44 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FDA8: movea.l $c(a6), a0
pub fn i_07fda8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDAC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FDAC: move.b #$5, $20(a0)
pub fn i_07fdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDB2;
    cpu.ir = 0x117C;
    let v1: u32 = 0x5;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $07FDB2: bra.w $7fdbc
pub fn i_07fdb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDB6;
    cpu.ir = 0x6000;
    cpu.pc = 0x7FDBC;
    10
}
/// $07FDBC: jmp $518.l
pub fn i_07fdbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDC2;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $07FDC4: lea.l $2e2ad8.l, a0
pub fn i_07fdc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDCA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2AD8;
    20
}
/// $07FDCA: jsr $5dd5c.l
pub fn i_07fdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FDD0);
    cpu.pc = 0x5DD5C;
    16
}
/// $07FDD0: bcc.w $7fdda
pub fn i_07fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDD4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x7FDDA; return 10; }
    8
}
// ===== SetC_07fdd4  ($07FDD4, 6 B, thunks/ccr_helpers.c)
/// $07FDD4: ori.b #$1, ccr
pub fn i_07fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDD8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $07FDD8: rts
pub fn i_07fdd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDDA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_07fdda  ($07FDDA, 6 B, thunks/ccr_helpers.c)
/// $07FDDA: andi.b #$fe, ccr
pub fn i_07fdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDDE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $07FDDE: rts
pub fn i_07fdde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_LeaderDeadGate_07fde0  ($07FDE0, 14 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FDE0: movea.l $c(a6), a0
pub fn i_07fde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDE4;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FDE4: cmpi.b #$ff, $21(a0)
pub fn i_07fde4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDEA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $07FDEA: bne.w $7fdf4
pub fn i_07fdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDEE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x7FDF4; return 10; }
    8
}
// ===== SetC_07fdee  ($07FDEE, 6 B, thunks/ccr_helpers.c)
/// $07FDEE: ori.b #$1, ccr
pub fn i_07fdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDF2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $07FDF2: rts
pub fn i_07fdf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDF4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearC_07fdf4  ($07FDF4, 6 B, thunks/ccr_helpers.c)
/// $07FDF4: andi.b #$fe, ccr
pub fn i_07fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDF8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $07FDF8: rts
pub fn i_07fdf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_TrackArc_07fdfa  ($07FDFA, 232 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FDFA: movea.l $c(a6), a0
pub fn i_07fdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FDFE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FDFE: move.w $22(a0), $22(a6)
pub fn i_07fdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE04;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FE04: move.w $24(a0), $24(a6)
pub fn i_07fe04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE0A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FE0A: move.w $74(a6), d0
pub fn i_07fe0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE0E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FE0E: asr.w #$8, d0
pub fn i_07fe0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE10;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07FE10: add.w $22(a6), d0
pub fn i_07fe10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE14;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FE14: move.w d0, $22(a6)
pub fn i_07fe14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE18;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FE18: addi.w #$8, $24(a6)
pub fn i_07fe18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE1E;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FE1E: lea.l $2e22fa.l, a1
pub fn i_07fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE24;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2E22FA;
    20
}
/// $07FE24: move.w $74(a6), d1
pub fn i_07fe24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE28;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FE28: asr.w #$8, d1
pub fn i_07fe28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE2A;
    cpu.ir = 0xE041;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07FE2A: addi.w #$50, d1
pub fn i_07fe2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE2E;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x50, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FE2E: tst.w d1
pub fn i_07fe2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE30;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07FE30: bpl.w $7fe3c
pub fn i_07fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE34;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FE3C; return 10; }
    8
}
/// $07FE34: move.w #$b800, $74(a6)
pub fn i_07fe34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE3A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FE3A: clr.w d1
pub fn i_07fe3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE3C;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07FE3C: cmpi.w #$90, d1
pub fn i_07fe3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE40;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x90, Size::Word, false);
    8
}
/// $07FE40: ble.w $7fe4e
pub fn i_07fe40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE44;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7FE4E; return 10; }
    8
}
/// $07FE44: move.w #$4800, $74(a6)
pub fn i_07fe44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE4A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x4800;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FE4A: move.w #$90, d1
pub fn i_07fe4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE4E;
    cpu.ir = 0x323C;
    let v1: u32 = 0x90;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FE4E: clr.w d0
pub fn i_07fe4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE50;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07FE50: lsr.w #$2, d1
pub fn i_07fe50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE52;
    cpu.ir = 0xE449;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $07FE52: move.b (a1, d1.w), d0
pub fn i_07fe52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE56;
    cpu.ir = 0x1031;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $07FE56: cmpi.w #$1, $72(a6)
pub fn i_07fe56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE5C;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Word, false);
    16
}
/// $07FE5C: beq.w $7fe64
pub fn i_07fe5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE60;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FE64; return 10; }
    8
}
/// $07FE60: add.w d0, $24(a6)
pub fn i_07fe60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE64;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FE64: rts
pub fn i_07fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FE66: movea.l $c(a6), a0
pub fn i_07fe66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE6A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FE6A: movea.l $c(a0), a0
pub fn i_07fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE6E;
    cpu.ir = 0x2068;
    let ad2 = cpu.a[0].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $07FE6E: move.w $78(a0), $22(a6)
pub fn i_07fe6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE74;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x78);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FE74: move.w $7a(a0), $24(a6)
pub fn i_07fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE7A;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FE7A: subq.w #$8, $22(a6)
pub fn i_07fe7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE7E;
    cpu.ir = 0x516E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 8, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FE7E: move.w $74(a6), d0
pub fn i_07fe7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE82;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FE82: add.w $28(a6), d0
pub fn i_07fe82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE86;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FE86: move.w d0, $74(a6)
pub fn i_07fe86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE8A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FE8A: asr.w #$8, d0
pub fn i_07fe8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE8C;
    cpu.ir = 0xE040;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07FE8C: add.w d0, $22(a6)
pub fn i_07fe8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE90;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FE90: tst.w $76(a6)
pub fn i_07fe90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE94;
    cpu.ir = 0x4A6E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $07FE94: beq.w $7feb2
pub fn i_07fe94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE98;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FEB2; return 10; }
    8
}
/// $07FE98: move.w $74(a6), d0
pub fn i_07fe98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE9C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FE9C: lsr.w #$8, d0
pub fn i_07fe9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FE9E;
    cpu.ir = 0xE048;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    22
}
/// $07FE9E: lsr.w #$1, d0
pub fn i_07fe9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEA0;
    cpu.ir = 0xE248;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $07FEA0: lea.l $2e2320.l, a0
pub fn i_07fea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEA6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E2320;
    20
}
/// $07FEA6: move.b (a0, d0.w), d0
pub fn i_07fea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEAA;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $07FEAA: andi.w #$ff, d0
pub fn i_07feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEAE;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FEAE: add.w d0, $24(a6)
pub fn i_07feae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEB2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FEB2: rts
pub fn i_07feb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEB4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FEB4: addi.w #$2, $74(a6)
pub fn i_07feb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEBA;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FEBA: andi.w #$7f, $74(a6)
pub fn i_07feba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEC0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x74);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x7F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $07FEC0: move.w $76(a6), d0
pub fn i_07fec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEC4;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FEC4: sub.w d0, $24(a6)
pub fn i_07fec4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEC8;
    cpu.ir = 0x916E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = sub_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $07FEC8: move.w $74(a6), d0
pub fn i_07fec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FECC;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x74);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FECC: add.w d0, d0
pub fn i_07fecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FECE;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $07FECE: lea.l $2c072c.l, a0
pub fn i_07fece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FED4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C072C;
    20
}
/// $07FED4: move.w (a0, d0.w), d0
pub fn i_07fed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FED8;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $07FED8: muls.w #$4, d0
pub fn i_07fed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEDC;
    cpu.ir = 0xC1FC;
    let v1: u32 = 0x4;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $07FEDC: asr.l #$8, d0
pub fn i_07fedc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEDE;
    cpu.ir = 0xE080;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[0] = r;
    22
}
/// $07FEDE: add.w d0, $24(a6)
pub fn i_07fede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEE2;
    cpu.ir = 0xD16E;
    let ad1 = cpu.a[6].wrapping_add(0x24);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
// ===== SetTaskW_07fee2  ($07FEE2, 6 B, kernel/task_setters_w.c)
/// $07FEE2: move.w d0, $76(a6)
pub fn i_07fee2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEE6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FEE6: rts
pub fn i_07fee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Squad_AIDecide_07fee8  ($07FEE8, 326 B, enemies/squad/deploy_module_07fbd2.s)
/// $07FEE8: cmpi.w #$ffff, $80(a6)
pub fn i_07fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEEE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $07FEEE: beq.w $7ff28
pub fn i_07feee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEF2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FF28; return 10; }
    8
}
/// $07FEF2: move.w $80(a6), $66(a6)
pub fn i_07fef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEF8;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    20
}
/// $07FEF8: tst.b $9d(a6)
pub fn i_07fef8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FEFC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $07FEFC: beq.w $7ff0e
pub fn i_07fefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF00;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x7FF0E; return 10; }
    8
}
/// $07FF00: cmpi.w #$50, $22(a6)
pub fn i_07ff00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF06;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x50, Size::Word, false);
    16
}
/// $07FF06: blt.w $7ff96
pub fn i_07ff06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF0A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7FF96; return 10; }
    8
}
/// $07FF0A: bra.w $7ff18
pub fn i_07ff0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF0E;
    cpu.ir = 0x6000;
    cpu.pc = 0x7FF18;
    10
}
/// $07FF0E: cmpi.w #$f0, $22(a6)
pub fn i_07ff0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xF0, Size::Word, false);
    16
}
/// $07FF14: bgt.w $7ff5e
pub fn i_07ff14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF18;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7FF5E; return 10; }
    8
}
/// $07FF18: move.w #$ffff, $80(a6)
pub fn i_07ff18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF1E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FF1E: lea.l $2e251c.l, a0
pub fn i_07ff1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF24;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2E251C;
    20
}
/// $07FF24: move.l a0, $48(a6)
pub fn i_07ff24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF28;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $07FF28: jsr $5e0d4.l
pub fn i_07ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF2E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FF2E);
    cpu.pc = 0x5E0D4;
    16
}
/// $07FF2E: bcs.w $7ffbc
pub fn i_07ff2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF32;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x7FFBC; return 10; }
    8
}
/// $07FF32: clr.b d2
pub fn i_07ff32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF34;
    cpu.ir = 0x4202;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $07FF34: move.w $22(a6), d0
pub fn i_07ff34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF38;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FF38: sub.w $22(a0), d0
pub fn i_07ff38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF3C;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FF3C: cmpi.w #$60, d0
pub fn i_07ff3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF40;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x60, Size::Word, false);
    8
}
/// $07FF40: blt.w $7ff84
pub fn i_07ff40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF44;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7FF84; return 10; }
    8
}
/// $07FF44: cmpi.w #$90, d0
pub fn i_07ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF48;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x90, Size::Word, false);
    8
}
/// $07FF48: ble.w $7ffbc
pub fn i_07ff48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF4C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7FFBC; return 10; }
    8
}
/// $07FF4C: moveq #$0, d0
pub fn i_07ff4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF4E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07FF4E: move.b $98(a6), d0
pub fn i_07ff4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF52;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07FF52: asl.w #$4, d0
pub fn i_07ff52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF54;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07FF54: cmp.w $22(a6), d0
pub fn i_07ff54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF58;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07FF58: bge.w $7ffbc
pub fn i_07ff58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF5C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x7FFBC; return 10; }
    8
}
/// $07FF5C: moveq #$0, d0
pub fn i_07ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF5E;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07FF5E: move.w #$fffe, d1
pub fn i_07ff5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF62;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFFFE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FF62: add.w $28(a6), d1
pub fn i_07ff62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF66;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FF66: cmpi.w #$fee0, d1
pub fn i_07ff66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF6A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFEE0, Size::Word, false);
    8
}
/// $07FF6A: bgt.w $7ff72
pub fn i_07ff6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF6E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x7FF72; return 10; }
    8
}
/// $07FF6E: move.w #$fee0, d1
pub fn i_07ff6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF72;
    cpu.ir = 0x323C;
    let v1: u32 = 0xFEE0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FF72: move.w d1, $28(a6)
pub fn i_07ff72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF76;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FF76: move.w #$18, $70(a6)
pub fn i_07ff76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF7C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x18;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FF7C: move.w #$1, $7c(a6)
pub fn i_07ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF82;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FF82: rts
pub fn i_07ff82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF84;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FF84: moveq #$0, d0
pub fn i_07ff84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF86;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07FF86: move.b $99(a6), d0
pub fn i_07ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF8A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $07FF8A: asl.w #$4, d0
pub fn i_07ff8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF8C;
    cpu.ir = 0xE940;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $07FF8C: cmp.w $22(a6), d0
pub fn i_07ff8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF90;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $07FF90: ble.w $7ffbc
pub fn i_07ff90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF94;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x7FFBC; return 10; }
    8
}
/// $07FF94: moveq #$0, d0
pub fn i_07ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF96;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $07FF96: move.w #$2, d1
pub fn i_07ff96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF9A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FF9A: add.w $28(a6), d1
pub fn i_07ff9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FF9E;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $07FF9E: cmpi.w #$120, d1
pub fn i_07ff9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFA2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x120, Size::Word, false);
    8
}
/// $07FFA2: blt.w $7ffaa
pub fn i_07ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFA6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x7FFAA; return 10; }
    8
}
/// $07FFA6: move.w #$120, d1
pub fn i_07ffa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFAA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $07FFAA: move.w d1, $28(a6)
pub fn i_07ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFAE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFAE: move.w #$30, $70(a6)
pub fn i_07ffae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFB4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFB4: move.w #$2, $7c(a6)
pub fn i_07ffb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFBA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFBA: rts
pub fn i_07ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFBC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FFBC: move.w #$24, $70(a6)
pub fn i_07ffbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFC2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x24;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFC2: move.w $28(a6), d1
pub fn i_07ffc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFC6;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $07FFC6: tst.w d1
pub fn i_07ffc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFC8;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07FFC8: bpl.w $7ffe0
pub fn i_07ffc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFCC;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FFE0; return 10; }
    8
}
/// $07FFCC: move.w #$1, $7c(a6)
pub fn i_07ffcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFD2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFD2: addq.w #$6, d1
pub fn i_07ffd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFD4;
    cpu.ir = 0x5C41;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 6, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $07FFD4: tst.w d1
pub fn i_07ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFD6;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07FFD6: bpl.w $7ffee
pub fn i_07ffd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFDA;
    cpu.ir = 0x6A00;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FFEE; return 10; }
    8
}
/// $07FFDA: move.w d1, $28(a6)
pub fn i_07ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFDE;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFDE: rts
pub fn i_07ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFE0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FFE0: move.w #$2, $7c(a6)
pub fn i_07ffe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFE6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFE6: addi.w #$fffa, d1
pub fn i_07ffe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFEA;
    cpu.ir = 0x641;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0xFFFA, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $07FFEA: tst.w d1
pub fn i_07ffea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFEC;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $07FFEC: bpl.b $7ffda
pub fn i_07ffec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFEE;
    cpu.ir = 0x6AEC;
    if cpu.sr.evaluate(Condition::Pl) { cpu.pc = 0x7FFDA; return 10; }
    8
}
/// $07FFEE: jsr $267e2.l
pub fn i_07ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFF4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x7FFF4);
    cpu.pc = 0x267E2;
    16
}
/// $07FFF4: move.w #$0, $7c(a6)
pub fn i_07fff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFFA;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $07FFFA: rts
pub fn i_07fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x7FFFC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $07FFFC: move.w $72(a6), d0
pub fn i_07fffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x80000;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x07FADE, i_07fade),
    (0x07FAE4, i_07fae4),
    (0x07FAEA, i_07faea),
    (0x07FAEE, i_07faee),
    (0x07FAF0, i_07faf0),
    (0x07FAF4, i_07faf4),
    (0x07FAFA, i_07fafa),
    (0x07FAFE, i_07fafe),
    (0x07FB02, i_07fb02),
    (0x07FB06, i_07fb06),
    (0x07FB0A, i_07fb0a),
    (0x07FB0E, i_07fb0e),
    (0x07FB12, i_07fb12),
    (0x07FB14, i_07fb14),
    (0x07FB18, i_07fb18),
    (0x07FB1C, i_07fb1c),
    (0x07FB20, i_07fb20),
    (0x07FB24, i_07fb24),
    (0x07FB26, i_07fb26),
    (0x07FB28, i_07fb28),
    (0x07FB2C, i_07fb2c),
    (0x07FB32, i_07fb32),
    (0x07FB38, i_07fb38),
    (0x07FB3C, i_07fb3c),
    (0x07FB3E, i_07fb3e),
    (0x07FB42, i_07fb42),
    (0x07FB48, i_07fb48),
    (0x07FB4C, i_07fb4c),
    (0x07FB50, i_07fb50),
    (0x07FB54, i_07fb54),
    (0x07FB56, i_07fb56),
    (0x07FB5A, i_07fb5a),
    (0x07FB5E, i_07fb5e),
    (0x07FB62, i_07fb62),
    (0x07FB66, i_07fb66),
    (0x07FB6A, i_07fb6a),
    (0x07FB6C, i_07fb6c),
    (0x07FB6E, i_07fb6e),
    (0x07FB74, i_07fb74),
    (0x07FB78, i_07fb78),
    (0x07FB7C, i_07fb7c),
    (0x07FB80, i_07fb80),
    (0x07FB86, i_07fb86),
    (0x07FB8A, i_07fb8a),
    (0x07FB8E, i_07fb8e),
    (0x07FB94, i_07fb94),
    (0x07FB98, i_07fb98),
    (0x07FB9E, i_07fb9e),
    (0x07FBA4, i_07fba4),
    (0x07FBA8, i_07fba8),
    (0x07FBAA, i_07fbaa),
    (0x07FBAE, i_07fbae),
    (0x07FBB4, i_07fbb4),
    (0x07FBB8, i_07fbb8),
    (0x07FBBC, i_07fbbc),
    (0x07FBBE, i_07fbbe),
    (0x07FBC2, i_07fbc2),
    (0x07FBC6, i_07fbc6),
    (0x07FBCA, i_07fbca),
    (0x07FBCE, i_07fbce),
    (0x07FBD0, i_07fbd0),
    (0x07FBD2, i_07fbd2),
    (0x07FBD8, i_07fbd8),
    (0x07FBDC, i_07fbdc),
    (0x07FBE0, i_07fbe0),
    (0x07FBE6, i_07fbe6),
    (0x07FBEC, i_07fbec),
    (0x07FBF0, i_07fbf0),
    (0x07FBF2, i_07fbf2),
    (0x07FBF6, i_07fbf6),
    (0x07FBFC, i_07fbfc),
    (0x07FC00, i_07fc00),
    (0x07FC04, i_07fc04),
    (0x07FC06, i_07fc06),
    (0x07FC0A, i_07fc0a),
    (0x07FC0E, i_07fc0e),
    (0x07FC12, i_07fc12),
    (0x07FC16, i_07fc16),
    (0x07FC18, i_07fc18),
    (0x07FC1A, i_07fc1a),
    (0x07FC20, i_07fc20),
    (0x07FC26, i_07fc26),
    (0x07FC2C, i_07fc2c),
    (0x07FC32, i_07fc32),
    (0x07FC38, i_07fc38),
    (0x07FC3E, i_07fc3e),
    (0x07FC44, i_07fc44),
    (0x07FC4A, i_07fc4a),
    (0x07FC4E, i_07fc4e),
    (0x07FC50, i_07fc50),
    (0x07FCBE, i_07fcbe),
    (0x07FCC2, i_07fcc2),
    (0x07FCC8, i_07fcc8),
    (0x07FCCE, i_07fcce),
    (0x07FCD4, i_07fcd4),
    (0x07FCDA, i_07fcda),
    (0x07FCDE, i_07fcde),
    (0x07FCE4, i_07fce4),
    (0x07FCE8, i_07fce8),
    (0x07FCEC, i_07fcec),
    (0x07FCF0, i_07fcf0),
    (0x07FCF6, i_07fcf6),
    (0x07FCFC, i_07fcfc),
    (0x07FD02, i_07fd02),
    (0x07FD08, i_07fd08),
    (0x07FD0E, i_07fd0e),
    (0x07FD14, i_07fd14),
    (0x07FD18, i_07fd18),
    (0x07FD1E, i_07fd1e),
    (0x07FD24, i_07fd24),
    (0x07FD2A, i_07fd2a),
    (0x07FD2E, i_07fd2e),
    (0x07FD32, i_07fd32),
    (0x07FD36, i_07fd36),
    (0x07FD38, i_07fd38),
    (0x07FD3C, i_07fd3c),
    (0x07FD40, i_07fd40),
    (0x07FD44, i_07fd44),
    (0x07FD46, i_07fd46),
    (0x07FD48, i_07fd48),
    (0x07FD4C, i_07fd4c),
    (0x07FD52, i_07fd52),
    (0x07FD58, i_07fd58),
    (0x07FD5E, i_07fd5e),
    (0x07FD62, i_07fd62),
    (0x07FD68, i_07fd68),
    (0x07FD6C, i_07fd6c),
    (0x07FD70, i_07fd70),
    (0x07FD74, i_07fd74),
    (0x07FD7A, i_07fd7a),
    (0x07FD7E, i_07fd7e),
    (0x07FD82, i_07fd82),
    (0x07FD88, i_07fd88),
    (0x07FD8A, i_07fd8a),
    (0x07FD8E, i_07fd8e),
    (0x07FD92, i_07fd92),
    (0x07FD98, i_07fd98),
    (0x07FD9C, i_07fd9c),
    (0x07FDA0, i_07fda0),
    (0x07FDA4, i_07fda4),
    (0x07FDA6, i_07fda6),
    (0x07FDA8, i_07fda8),
    (0x07FDAC, i_07fdac),
    (0x07FDB2, i_07fdb2),
    (0x07FDBC, i_07fdbc),
    (0x07FDC4, i_07fdc4),
    (0x07FDCA, i_07fdca),
    (0x07FDD0, i_07fdd0),
    (0x07FDD4, i_07fdd4),
    (0x07FDD8, i_07fdd8),
    (0x07FDDA, i_07fdda),
    (0x07FDDE, i_07fdde),
    (0x07FDE0, i_07fde0),
    (0x07FDE4, i_07fde4),
    (0x07FDEA, i_07fdea),
    (0x07FDEE, i_07fdee),
    (0x07FDF2, i_07fdf2),
    (0x07FDF4, i_07fdf4),
    (0x07FDF8, i_07fdf8),
    (0x07FDFA, i_07fdfa),
    (0x07FDFE, i_07fdfe),
    (0x07FE04, i_07fe04),
    (0x07FE0A, i_07fe0a),
    (0x07FE0E, i_07fe0e),
    (0x07FE10, i_07fe10),
    (0x07FE14, i_07fe14),
    (0x07FE18, i_07fe18),
    (0x07FE1E, i_07fe1e),
    (0x07FE24, i_07fe24),
    (0x07FE28, i_07fe28),
    (0x07FE2A, i_07fe2a),
    (0x07FE2E, i_07fe2e),
    (0x07FE30, i_07fe30),
    (0x07FE34, i_07fe34),
    (0x07FE3A, i_07fe3a),
    (0x07FE3C, i_07fe3c),
    (0x07FE40, i_07fe40),
    (0x07FE44, i_07fe44),
    (0x07FE4A, i_07fe4a),
    (0x07FE4E, i_07fe4e),
    (0x07FE50, i_07fe50),
    (0x07FE52, i_07fe52),
    (0x07FE56, i_07fe56),
    (0x07FE5C, i_07fe5c),
    (0x07FE60, i_07fe60),
    (0x07FE64, i_07fe64),
    (0x07FE66, i_07fe66),
    (0x07FE6A, i_07fe6a),
    (0x07FE6E, i_07fe6e),
    (0x07FE74, i_07fe74),
    (0x07FE7A, i_07fe7a),
    (0x07FE7E, i_07fe7e),
    (0x07FE82, i_07fe82),
    (0x07FE86, i_07fe86),
    (0x07FE8A, i_07fe8a),
    (0x07FE8C, i_07fe8c),
    (0x07FE90, i_07fe90),
    (0x07FE94, i_07fe94),
    (0x07FE98, i_07fe98),
    (0x07FE9C, i_07fe9c),
    (0x07FE9E, i_07fe9e),
    (0x07FEA0, i_07fea0),
    (0x07FEA6, i_07fea6),
    (0x07FEAA, i_07feaa),
    (0x07FEAE, i_07feae),
    (0x07FEB2, i_07feb2),
    (0x07FEB4, i_07feb4),
    (0x07FEBA, i_07feba),
    (0x07FEC0, i_07fec0),
    (0x07FEC4, i_07fec4),
    (0x07FEC8, i_07fec8),
    (0x07FECC, i_07fecc),
    (0x07FECE, i_07fece),
    (0x07FED4, i_07fed4),
    (0x07FED8, i_07fed8),
    (0x07FEDC, i_07fedc),
    (0x07FEDE, i_07fede),
    (0x07FEE2, i_07fee2),
    (0x07FEE6, i_07fee6),
    (0x07FEE8, i_07fee8),
    (0x07FEEE, i_07feee),
    (0x07FEF2, i_07fef2),
    (0x07FEF8, i_07fef8),
    (0x07FEFC, i_07fefc),
    (0x07FF00, i_07ff00),
    (0x07FF06, i_07ff06),
    (0x07FF0A, i_07ff0a),
    (0x07FF0E, i_07ff0e),
    (0x07FF14, i_07ff14),
    (0x07FF18, i_07ff18),
    (0x07FF1E, i_07ff1e),
    (0x07FF24, i_07ff24),
    (0x07FF28, i_07ff28),
    (0x07FF2E, i_07ff2e),
    (0x07FF32, i_07ff32),
    (0x07FF34, i_07ff34),
    (0x07FF38, i_07ff38),
    (0x07FF3C, i_07ff3c),
    (0x07FF40, i_07ff40),
    (0x07FF44, i_07ff44),
    (0x07FF48, i_07ff48),
    (0x07FF4C, i_07ff4c),
    (0x07FF4E, i_07ff4e),
    (0x07FF52, i_07ff52),
    (0x07FF54, i_07ff54),
    (0x07FF58, i_07ff58),
    (0x07FF5C, i_07ff5c),
    (0x07FF5E, i_07ff5e),
    (0x07FF62, i_07ff62),
    (0x07FF66, i_07ff66),
    (0x07FF6A, i_07ff6a),
    (0x07FF6E, i_07ff6e),
    (0x07FF72, i_07ff72),
    (0x07FF76, i_07ff76),
    (0x07FF7C, i_07ff7c),
    (0x07FF82, i_07ff82),
    (0x07FF84, i_07ff84),
    (0x07FF86, i_07ff86),
    (0x07FF8A, i_07ff8a),
    (0x07FF8C, i_07ff8c),
    (0x07FF90, i_07ff90),
    (0x07FF94, i_07ff94),
    (0x07FF96, i_07ff96),
    (0x07FF9A, i_07ff9a),
    (0x07FF9E, i_07ff9e),
    (0x07FFA2, i_07ffa2),
    (0x07FFA6, i_07ffa6),
    (0x07FFAA, i_07ffaa),
    (0x07FFAE, i_07ffae),
    (0x07FFB4, i_07ffb4),
    (0x07FFBA, i_07ffba),
    (0x07FFBC, i_07ffbc),
    (0x07FFC2, i_07ffc2),
    (0x07FFC6, i_07ffc6),
    (0x07FFC8, i_07ffc8),
    (0x07FFCC, i_07ffcc),
    (0x07FFD2, i_07ffd2),
    (0x07FFD4, i_07ffd4),
    (0x07FFD6, i_07ffd6),
    (0x07FFDA, i_07ffda),
    (0x07FFDE, i_07ffde),
    (0x07FFE0, i_07ffe0),
    (0x07FFE6, i_07ffe6),
    (0x07FFEA, i_07ffea),
    (0x07FFEC, i_07ffec),
    (0x07FFEE, i_07ffee),
    (0x07FFF4, i_07fff4),
    (0x07FFFA, i_07fffa),
    (0x07FFFC, i_07fffc),
];
