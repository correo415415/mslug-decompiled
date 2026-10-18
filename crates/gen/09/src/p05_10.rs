// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_10) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $05FDEA: lea.l $5fdf0(pc), a1
pub fn i_05fdea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDEE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FDF0;
    16
}
/// $05FDEE: move.l a1, (a6)
pub fn i_05fdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDF0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FDF0: jsr $2783a.l
pub fn i_05fdf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FDF6);
    cpu.pc = 0x2783A;
    16
}
/// $05FDF6: jsr $28d70.l
pub fn i_05fdf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FDFC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FDFC);
    cpu.pc = 0x28D70;
    16
}
/// $05FDFC: bcc.w $5fe06
pub fn i_05fdfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE00;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FE06; return 10; }
    8
}
/// $05FE00: lea.l $5fe26(pc), a1
pub fn i_05fe00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $05FE04: move.l a1, (a6)
pub fn i_05fe04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FE06: jsr $2870a.l
pub fn i_05fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE0C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE0C);
    cpu.pc = 0x2870A;
    16
}
/// $05FE0C: bcc.w $5fe16
pub fn i_05fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE10;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FE16; return 10; }
    8
}
/// $05FE10: lea.l $60210(pc), a1
pub fn i_05fe10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE14;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FE14: move.l a1, (a6)
pub fn i_05fe14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE16;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FE16: jsr $6037c(pc)
pub fn i_05fe16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE1A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FE1A);
    cpu.pc = 0x6037C;
    16
}
/// $05FE1A: bcc.w $5fe24
pub fn i_05fe1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE1E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FE24; return 10; }
    8
}
/// $05FE1E: jmp $518.l
pub fn i_05fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE24;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FE24: rts
pub fn i_05fe24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE26;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Idle_05fe26  ($05FE26, 154 B, props/late_props_turrets_05e000.s)
/// $05FE26: lea.l $2b79a0.l, a0
pub fn i_05fe26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE2C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B79A0;
    20
}
/// $05FE2C: jsr $799de.l
pub fn i_05fe2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE32);
    cpu.pc = 0x799DE;
    16
}
/// $05FE32: move.w d0, $70(a6)
pub fn i_05fe32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE36;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FE36: lea.l $2c196e.l, a0
pub fn i_05fe36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE3C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C196E;
    20
}
/// $05FE3C: jsr $28cd4.l
pub fn i_05fe3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE42;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE42);
    cpu.pc = 0x28CD4;
    16
}
/// $05FE42: lea.l $5fe48(pc), a1
pub fn i_05fe42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE46;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE48;
    16
}
/// $05FE46: move.l a1, (a6)
pub fn i_05fe46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE48;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FE48: jsr $2783a.l
pub fn i_05fe48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE4E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE4E);
    cpu.pc = 0x2783A;
    16
}
/// $05FE4E: jsr $28d70.l
pub fn i_05fe4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE54;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE54);
    cpu.pc = 0x28D70;
    16
}
/// $05FE54: cmpi.w #$ff, $70(a6)
pub fn i_05fe54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE5A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Word, false);
    16
}
/// $05FE5A: beq.w $5fe78
pub fn i_05fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE5E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5FE78; return 10; }
    8
}
/// $05FE5E: subi.w #$1, $70(a6)
pub fn i_05fe5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE64;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05FE64: beq.w $5fe72
pub fn i_05fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE68;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5FE72; return 10; }
    8
}
/// $05FE68: subi.w #$1, $70(a6)
pub fn i_05fe68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE6E;
    cpu.ir = 0x46E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $05FE6E: bne.w $5fe78
pub fn i_05fe6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE72;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FE78; return 10; }
    8
}
/// $05FE72: lea.l $5fec0(pc), a1
pub fn i_05fe72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE76;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FEC0;
    16
}
/// $05FE76: move.l a1, (a6)
pub fn i_05fe76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE78;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FE78: jsr $5e0d4.l
pub fn i_05fe78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE7E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FE7E);
    cpu.pc = 0x5E0D4;
    16
}
/// $05FE7E: move.w $22(a0), d0
pub fn i_05fe7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE82;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05FE82: sub.w $22(a6), d0
pub fn i_05fe82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE86;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05FE86: rol.w #$1, d0
pub fn i_05fe86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE88;
    cpu.ir = 0xE358;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05FE88: andi.w #$1, d0
pub fn i_05fe88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE8C;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05FE8C: move.b $3a(a6), d1
pub fn i_05fe8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE90;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05FE90: andi.b #$1, d1
pub fn i_05fe90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE94;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05FE94: cmp.b d0, d1
pub fn i_05fe94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE96;
    cpu.ir = 0xB200;
    let v1 = cpu.d[0] & 0xFF;
    let h1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $05FE96: bne.w $5fea0
pub fn i_05fe96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE9A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5FEA0; return 10; }
    8
}
/// $05FE9A: lea.l $5ff1c(pc), a1
pub fn i_05fe9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FE9E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FF1C;
    16
}
/// $05FE9E: move.l a1, (a6)
pub fn i_05fe9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEA0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FEA0: jsr $2870a.l
pub fn i_05fea0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEA6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEA6);
    cpu.pc = 0x2870A;
    16
}
/// $05FEA6: bcc.w $5feb0
pub fn i_05fea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEAA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FEB0; return 10; }
    8
}
/// $05FEAA: lea.l $60210(pc), a1
pub fn i_05feaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEAE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FEAE: move.l a1, (a6)
pub fn i_05feae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEB0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FEB0: jsr $6037c(pc)
pub fn i_05feb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEB4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FEB4);
    cpu.pc = 0x6037C;
    16
}
/// $05FEB4: bcc.w $5febe
pub fn i_05feb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEB8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FEBE; return 10; }
    8
}
/// $05FEB8: jmp $518.l
pub fn i_05feb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEBE;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FEBE: rts
pub fn i_05febe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEC0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Turn_05fec0  ($05FEC0, 92 B, props/late_props_turrets_05e000.s)
/// $05FEC0: jsr $5e0d4.l
pub fn i_05fec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEC6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEC6);
    cpu.pc = 0x5E0D4;
    16
}
/// $05FEC6: move.w $22(a0), d0
pub fn i_05fec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FECA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05FECA: sub.w $22(a6), d0
pub fn i_05feca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FECE;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05FECE: asl.w #$1, d0
pub fn i_05fece(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FED0;
    cpu.ir = 0xE340;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05FED0: move.w d0, $28(a6)
pub fn i_05fed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FED4;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FED4: lea.l $2c19ce.l, a0
pub fn i_05fed4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEDA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C19CE;
    20
}
/// $05FEDA: jsr $28cd4.l
pub fn i_05feda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEE0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEE0);
    cpu.pc = 0x28CD4;
    16
}
/// $05FEE0: lea.l $5fee6(pc), a1
pub fn i_05fee0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEE4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FEE6;
    16
}
/// $05FEE4: move.l a1, (a6)
pub fn i_05fee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEE6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FEE6: jsr $2783a.l
pub fn i_05fee6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEEC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEEC);
    cpu.pc = 0x2783A;
    16
}
/// $05FEEC: jsr $28d70.l
pub fn i_05feec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEF2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FEF2);
    cpu.pc = 0x28D70;
    16
}
/// $05FEF2: bcc.w $5fefc
pub fn i_05fef2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEF6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FEFC; return 10; }
    8
}
/// $05FEF6: lea.l $5fe26(pc), a1
pub fn i_05fef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $05FEFA: move.l a1, (a6)
pub fn i_05fefa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FEFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FEFC: jsr $2870a.l
pub fn i_05fefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF02;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF02);
    cpu.pc = 0x2870A;
    16
}
/// $05FF02: bcc.w $5ff0c
pub fn i_05ff02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF06;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF0C; return 10; }
    8
}
/// $05FF06: lea.l $60210(pc), a1
pub fn i_05ff06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FF0A: move.l a1, (a6)
pub fn i_05ff0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FF0C: jsr $6037c(pc)
pub fn i_05ff0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF10;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FF10);
    cpu.pc = 0x6037C;
    16
}
/// $05FF10: bcc.w $5ff1a
pub fn i_05ff10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF14;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF1A; return 10; }
    8
}
/// $05FF14: jmp $518.l
pub fn i_05ff14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF1A;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FF1A: rts
pub fn i_05ff1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Destroyed_05ff1c  ($05FF1C, 72 B, props/late_props_turrets_05e000.s)
/// $05FF1C: lea.l $2c1b86.l, a0
pub fn i_05ff1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF22;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1B86;
    20
}
/// $05FF22: jsr $28cd4.l
pub fn i_05ff22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF28;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF28);
    cpu.pc = 0x28CD4;
    16
}
/// $05FF28: lea.l $5ff2e(pc), a1
pub fn i_05ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FF2E;
    16
}
/// $05FF2C: move.l a1, (a6)
pub fn i_05ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FF2E: jsr $2783a.l
pub fn i_05ff2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF34;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF34);
    cpu.pc = 0x2783A;
    16
}
/// $05FF34: jsr $28d70.l
pub fn i_05ff34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF3A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF3A);
    cpu.pc = 0x28D70;
    16
}
/// $05FF3A: bcc.w $5ff44
pub fn i_05ff3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF3E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF44; return 10; }
    8
}
/// $05FF3E: lea.l $5fe26(pc), a1
pub fn i_05ff3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF42;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FE26;
    16
}
/// $05FF42: move.l a1, (a6)
pub fn i_05ff42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF44;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FF44: jsr $2870a.l
pub fn i_05ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF4A);
    cpu.pc = 0x2870A;
    16
}
/// $05FF4A: bcc.w $5ff54
pub fn i_05ff4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF4E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF54; return 10; }
    8
}
/// $05FF4E: lea.l $60210(pc), a1
pub fn i_05ff4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF52;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FF52: move.l a1, (a6)
pub fn i_05ff52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF54;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FF54: jsr $6037c(pc)
pub fn i_05ff54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF58;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FF58);
    cpu.pc = 0x6037C;
    16
}
/// $05FF58: bcc.w $5ff62
pub fn i_05ff58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF5C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FF62; return 10; }
    8
}
/// $05FF5C: jmp $518.l
pub fn i_05ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF62;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $05FF62: rts
pub fn i_05ff62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF64;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable_Tmpl22_05ff64  ($05FF64, 10 B, props/late_props_turrets_05e000.s)
/// $05FF64: cmpi.w #$140, $22(a6)
pub fn i_05ff64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF6A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $05FF6A: blt.w $5ff76
pub fn i_05ff6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF6E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5FF76; return 10; }
    8
}
// ===== JsrAbsThunk_05ff6e  ($05FF6E, 8 B, thunks/jsr_abs_thunks.c)
/// $05FF6E: jsr $2783a.l
pub fn i_05ff6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF74;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF74);
    cpu.pc = 0x2783A;
    16
}
/// $05FF74: rts
pub fn i_05ff74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF76;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Breakable22_Init_05ff76  ($05FF76, 146 B, props/late_props_turrets_05e000.s)
/// $05FF76: move.w #$e, d1
pub fn i_05ff76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF7A;
    cpu.ir = 0x323C;
    let v1: u32 = 0xE;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05FF7A: jsr $236e.l
pub fn i_05ff7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FF80);
    cpu.pc = 0x236E;
    16
}
/// $05FF80: move.b #$ff, $32(a6)
pub fn i_05ff80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF86;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FF86: move.b #$ff, $33(a6)
pub fn i_05ff86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF8C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FF8C: move.b $98(a6), d0
pub fn i_05ff8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF90;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $05FF90: andi.b #$1, d0
pub fn i_05ff90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF94;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $05FF94: move.b d0, $3a(a6)
pub fn i_05ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF98;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $05FF98: move.w #$8000, $38(a6)
pub fn i_05ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FF9E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FF9E: move.w #$1, $66(a6)
pub fn i_05ff9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFA4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05FFA4: jsr $267e2.l
pub fn i_05ffa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFAA);
    cpu.pc = 0x267E2;
    16
}
/// $05FFAA: lea.l $2c1b02.l, a0
pub fn i_05ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFB0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2C1B02;
    20
}
/// $05FFB0: jsr $28cd4.l
pub fn i_05ffb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFB6);
    cpu.pc = 0x28CD4;
    16
}
/// $05FFB6: lea.l $5ffbc(pc), a1
pub fn i_05ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFBA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5FFBC;
    16
}
/// $05FFBA: move.l a1, (a6)
pub fn i_05ffba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFBC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FFBC: jsr $2783a.l
pub fn i_05ffbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFC2);
    cpu.pc = 0x2783A;
    16
}
/// $05FFC2: jsr $28d70.l
pub fn i_05ffc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFC8);
    cpu.pc = 0x28D70;
    16
}
/// $05FFC8: jsr $5e0d4.l
pub fn i_05ffc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFCE);
    cpu.pc = 0x5E0D4;
    16
}
/// $05FFCE: move.w $22(a0), d0
pub fn i_05ffce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFD2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05FFD2: sub.w $22(a6), d0
pub fn i_05ffd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFD6;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05FFD6: jsr $603b4(pc)
pub fn i_05ffd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFDA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FFDA);
    cpu.pc = 0x603B4;
    16
}
/// $05FFDA: cmpi.w #$20, d0
pub fn i_05ffda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFDE;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x20, Size::Word, false);
    8
}
/// $05FFDE: bgt.w $5ffe8
pub fn i_05ffde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFE2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5FFE8; return 10; }
    8
}
/// $05FFE2: lea.l $60008(pc), a1
pub fn i_05ffe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFE6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60008;
    16
}
/// $05FFE6: move.l a1, (a6)
pub fn i_05ffe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFE8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FFE8: jsr $2870a.l
pub fn i_05ffe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5FFEE);
    cpu.pc = 0x2870A;
    16
}
/// $05FFEE: bcc.w $5fff8
pub fn i_05ffee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5FFF8; return 10; }
    8
}
/// $05FFF2: lea.l $60210(pc), a1
pub fn i_05fff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x60210;
    16
}
/// $05FFF6: move.l a1, (a6)
pub fn i_05fff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05FFF8: jsr $6037c(pc)
pub fn i_05fff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5FFFC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5FFFC);
    cpu.pc = 0x6037C;
    16
}
/// $05FFFC: bcc.w $60006
pub fn i_05fffc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x60000;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x60006; return 10; }
    8
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x05FDEA, i_05fdea),
    (0x05FDEE, i_05fdee),
    (0x05FDF0, i_05fdf0),
    (0x05FDF6, i_05fdf6),
    (0x05FDFC, i_05fdfc),
    (0x05FE00, i_05fe00),
    (0x05FE04, i_05fe04),
    (0x05FE06, i_05fe06),
    (0x05FE0C, i_05fe0c),
    (0x05FE10, i_05fe10),
    (0x05FE14, i_05fe14),
    (0x05FE16, i_05fe16),
    (0x05FE1A, i_05fe1a),
    (0x05FE1E, i_05fe1e),
    (0x05FE24, i_05fe24),
    (0x05FE26, i_05fe26),
    (0x05FE2C, i_05fe2c),
    (0x05FE32, i_05fe32),
    (0x05FE36, i_05fe36),
    (0x05FE3C, i_05fe3c),
    (0x05FE42, i_05fe42),
    (0x05FE46, i_05fe46),
    (0x05FE48, i_05fe48),
    (0x05FE4E, i_05fe4e),
    (0x05FE54, i_05fe54),
    (0x05FE5A, i_05fe5a),
    (0x05FE5E, i_05fe5e),
    (0x05FE64, i_05fe64),
    (0x05FE68, i_05fe68),
    (0x05FE6E, i_05fe6e),
    (0x05FE72, i_05fe72),
    (0x05FE76, i_05fe76),
    (0x05FE78, i_05fe78),
    (0x05FE7E, i_05fe7e),
    (0x05FE82, i_05fe82),
    (0x05FE86, i_05fe86),
    (0x05FE88, i_05fe88),
    (0x05FE8C, i_05fe8c),
    (0x05FE90, i_05fe90),
    (0x05FE94, i_05fe94),
    (0x05FE96, i_05fe96),
    (0x05FE9A, i_05fe9a),
    (0x05FE9E, i_05fe9e),
    (0x05FEA0, i_05fea0),
    (0x05FEA6, i_05fea6),
    (0x05FEAA, i_05feaa),
    (0x05FEAE, i_05feae),
    (0x05FEB0, i_05feb0),
    (0x05FEB4, i_05feb4),
    (0x05FEB8, i_05feb8),
    (0x05FEBE, i_05febe),
    (0x05FEC0, i_05fec0),
    (0x05FEC6, i_05fec6),
    (0x05FECA, i_05feca),
    (0x05FECE, i_05fece),
    (0x05FED0, i_05fed0),
    (0x05FED4, i_05fed4),
    (0x05FEDA, i_05feda),
    (0x05FEE0, i_05fee0),
    (0x05FEE4, i_05fee4),
    (0x05FEE6, i_05fee6),
    (0x05FEEC, i_05feec),
    (0x05FEF2, i_05fef2),
    (0x05FEF6, i_05fef6),
    (0x05FEFA, i_05fefa),
    (0x05FEFC, i_05fefc),
    (0x05FF02, i_05ff02),
    (0x05FF06, i_05ff06),
    (0x05FF0A, i_05ff0a),
    (0x05FF0C, i_05ff0c),
    (0x05FF10, i_05ff10),
    (0x05FF14, i_05ff14),
    (0x05FF1A, i_05ff1a),
    (0x05FF1C, i_05ff1c),
    (0x05FF22, i_05ff22),
    (0x05FF28, i_05ff28),
    (0x05FF2C, i_05ff2c),
    (0x05FF2E, i_05ff2e),
    (0x05FF34, i_05ff34),
    (0x05FF3A, i_05ff3a),
    (0x05FF3E, i_05ff3e),
    (0x05FF42, i_05ff42),
    (0x05FF44, i_05ff44),
    (0x05FF4A, i_05ff4a),
    (0x05FF4E, i_05ff4e),
    (0x05FF52, i_05ff52),
    (0x05FF54, i_05ff54),
    (0x05FF58, i_05ff58),
    (0x05FF5C, i_05ff5c),
    (0x05FF62, i_05ff62),
    (0x05FF64, i_05ff64),
    (0x05FF6A, i_05ff6a),
    (0x05FF6E, i_05ff6e),
    (0x05FF74, i_05ff74),
    (0x05FF76, i_05ff76),
    (0x05FF7A, i_05ff7a),
    (0x05FF80, i_05ff80),
    (0x05FF86, i_05ff86),
    (0x05FF8C, i_05ff8c),
    (0x05FF90, i_05ff90),
    (0x05FF94, i_05ff94),
    (0x05FF98, i_05ff98),
    (0x05FF9E, i_05ff9e),
    (0x05FFA4, i_05ffa4),
    (0x05FFAA, i_05ffaa),
    (0x05FFB0, i_05ffb0),
    (0x05FFB6, i_05ffb6),
    (0x05FFBA, i_05ffba),
    (0x05FFBC, i_05ffbc),
    (0x05FFC2, i_05ffc2),
    (0x05FFC8, i_05ffc8),
    (0x05FFCE, i_05ffce),
    (0x05FFD2, i_05ffd2),
    (0x05FFD6, i_05ffd6),
    (0x05FFDA, i_05ffda),
    (0x05FFDE, i_05ffde),
    (0x05FFE2, i_05ffe2),
    (0x05FFE6, i_05ffe6),
    (0x05FFE8, i_05ffe8),
    (0x05FFEE, i_05ffee),
    (0x05FFF2, i_05fff2),
    (0x05FFF6, i_05fff6),
    (0x05FFF8, i_05fff8),
    (0x05FFFC, i_05fffc),
];
