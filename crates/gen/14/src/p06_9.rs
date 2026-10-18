// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p06_9) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


/// $06EF86: subq.w #$8, d2
pub fn i_06ef86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF88;
    cpu.ir = 0x5142;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 8, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06EF88: cmpi.w #$100, d2
pub fn i_06ef88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF8C;
    cpu.ir = 0xC42;
    let v1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x100, Size::Word, false);
    8
}
/// $06EF8C: blt.w $6efb6
pub fn i_06ef8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF90;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6EFB6; return 10; }
    8
}
/// $06EF90: move.w $22(a6), d1
pub fn i_06ef90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF94;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EF94: movem.w d2, -(a7)
pub fn i_06ef94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF98;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[2] as u16);
    cpu.a[7] = a;
    20
}
/// $06EF98: jsr $280c6.l
pub fn i_06ef98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EF9E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6EF9E);
    cpu.pc = 0x280C6;
    16
}
/// $06EF9E: movem.w (a7)+, d2
pub fn i_06ef9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFA2;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[2] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
/// $06EFA2: tst.b d0
pub fn i_06efa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFA4;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $06EFA4: beq.b $6ef86
pub fn i_06efa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFA6;
    cpu.ir = 0x67E0;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EF86; return 10; }
    8
}
/// $06EFA6: andi.b #$30, d0
pub fn i_06efa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFAA;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x30) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06EFAA: tst.b d0
pub fn i_06efaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFAC;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $06EFAC: beq.w $6efb6
pub fn i_06efac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFB0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EFB6; return 10; }
    8
}
/// $06EFB0: cmpi.b #$20, d0
pub fn i_06efb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFB4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $06EFB4: bne.b $6ef86
pub fn i_06efb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFB6;
    cpu.ir = 0x66D0;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6EF86; return 10; }
    8
}
/// $06EFB6: move.w $24(a6), d1
pub fn i_06efb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFBA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06EFBA: sub.w d2, d1
pub fn i_06efba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFBC;
    cpu.ir = 0x9242;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06EFBC: cmp.w $82(a6), d1
pub fn i_06efbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFC0;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06EFC0: bgt.w $6efca
pub fn i_06efc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFC4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6EFCA; return 10; }
    8
}
// ===== SetXN_06efc4  ($06EFC4, 6 B, thunks/ccr_helpers.c)
/// $06EFC4: ori.b #$11, ccr
pub fn i_06efc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFC8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06EFC8: rts
pub fn i_06efc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFCA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_06efca  ($06EFCA, 6 B, thunks/ccr_helpers.c)
/// $06EFCA: andi.b #$ee, ccr
pub fn i_06efca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFCE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06EFCE: rts
pub fn i_06efce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFD0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_BelowY_06efd0  ($06EFD0, 20 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EFD0: cmpi.b #$1, $9b(a6)
pub fn i_06efd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFD6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06EFD6: beq.w $6efdc
pub fn i_06efd6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFDA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6EFDC; return 10; }
    8
}
/// $06EFDA: move.w d3, d2
pub fn i_06efda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFDC;
    cpu.ir = 0x3403;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EFDC: cmp.w $24(a6), d2
pub fn i_06efdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFE0;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06EFE0: bgt.w $6efea
pub fn i_06efe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFE4;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6EFEA; return 10; }
    8
}
// ===== ClearXN_06efe4  ($06EFE4, 6 B, thunks/ccr_helpers.c)
/// $06EFE4: andi.b #$ee, ccr
pub fn i_06efe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFE8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06EFE8: rts
pub fn i_06efe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFEA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06efea  ($06EFEA, 6 B, thunks/ccr_helpers.c)
/// $06EFEA: ori.b #$11, ccr
pub fn i_06efea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFEE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06EFEE: rts
pub fn i_06efee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_InScreenByMode_06eff0  ($06EFF0, 48 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06EFF0: move.w #$44, d1
pub fn i_06eff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFF4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x44;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EFF4: move.w #$190, d2
pub fn i_06eff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFF8;
    cpu.ir = 0x343C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EFF8: move.w #$180, d3
pub fn i_06eff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6EFFC;
    cpu.ir = 0x363C;
    let v1: u32 = 0x180;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06EFFC: cmpi.b #$0, $99(a6)
pub fn i_06effc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F002;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06F002: beq.w $6f012
pub fn i_06f002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F006;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F012; return 10; }
    8
}
/// $06F006: move.w #$28, d1
pub fn i_06f006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F00A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x28;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F00A: move.w #$168, d2
pub fn i_06f00a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F00E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F00E: move.w #$158, d3
pub fn i_06f00e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F012;
    cpu.ir = 0x363C;
    let v1: u32 = 0x158;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F012: tst.b $9b(a6)
pub fn i_06f012(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F016;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06F016: bne.b $6efd0
pub fn i_06f016(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F018;
    cpu.ir = 0x66B8;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6EFD0; return 10; }
    8
}
/// $06F018: move.w d1, $82(a6)
pub fn i_06f018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F01C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F01C: bra.w $6ef80
pub fn i_06f01c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F020;
    cpu.ir = 0x6000;
    cpu.pc = 0x6EF80;
    10
}
// ===== Gunship_InScreenByModeB_06f020  ($06F020, 48 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F020: move.w #$2c, d1
pub fn i_06f020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F024;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F024: move.w #$178, d2
pub fn i_06f024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F028;
    cpu.ir = 0x343C;
    let v1: u32 = 0x178;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F028: move.w #$168, d3
pub fn i_06f028(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F02C;
    cpu.ir = 0x363C;
    let v1: u32 = 0x168;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F02C: cmpi.b #$0, $99(a6)
pub fn i_06f02c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F032;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x99);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06F032: beq.w $6f042
pub fn i_06f032(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F036;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F042; return 10; }
    8
}
/// $06F036: move.w #$10, d1
pub fn i_06f036(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F03A;
    cpu.ir = 0x323C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F03A: move.w #$150, d2
pub fn i_06f03a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F03E;
    cpu.ir = 0x343C;
    let v1: u32 = 0x150;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F03E: move.w #$140, d3
pub fn i_06f03e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F042;
    cpu.ir = 0x363C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F042: tst.b $9b(a6)
pub fn i_06f042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F046;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06F046: bne.b $6efd0
pub fn i_06f046(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F048;
    cpu.ir = 0x6688;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6EFD0; return 10; }
    8
}
/// $06F048: move.w d1, $82(a6)
pub fn i_06f048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F04C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F04C: bra.w $6ef80
pub fn i_06f04c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F050;
    cpu.ir = 0x6000;
    cpu.pc = 0x6EF80;
    10
}
// ===== Gunship_SpeedReached_06f050  ($06F050, 68 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F050: move.w $28(a6), d0
pub fn i_06f050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F054;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06F054: btst.b #$0, $3a(a6)
pub fn i_06f054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F05A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06F05A: beq.w $6f060
pub fn i_06f05a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F05E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F060; return 10; }
    8
}
/// $06F05E: neg.w d0
pub fn i_06f05e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F060;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06F060: cmp.w $36(a6), d0
pub fn i_06f060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F064;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06F064: bcs.w $6f080
pub fn i_06f064(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F068;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6F080; return 10; }
    8
}
/// $06F068: move.w $36(a6), d0
pub fn i_06f068(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F06C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06F06C: btst.b #$0, $3a(a6)
pub fn i_06f06c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F072;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06F072: bne.w $6f078
pub fn i_06f072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F076;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F078; return 10; }
    8
}
/// $06F076: neg.w d0
pub fn i_06f076(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F078;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06F078: move.w d0, $28(a6)
pub fn i_06f078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F07C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F07C: clr.w $2c(a6)
pub fn i_06f07c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F080;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2C);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F080: cmpi.w #$0, $2a(a6)
pub fn i_06f080(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F086;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F086: blt.w $6f092
pub fn i_06f086(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F08A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6F092; return 10; }
    8
}
/// $06F08A: clr.w $2e(a6)
pub fn i_06f08a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F08E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2E);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F08E: clr.w $2a(a6)
pub fn i_06f08e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F092;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F092: rts
pub fn i_06f092(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F094;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpeedByAngle_06f094  ($06F094, 30 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F094: lea.l $2d4aa6.l, a0
pub fn i_06f094(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F09A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4AA6;
    20
}
/// $06F09A: move.b $34(a6), d0
pub fn i_06f09a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F09E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06F09E: andi.w #$ff, d0
pub fn i_06f09e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0A2;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06F0A2: lsr.w #$3, d0
pub fn i_06f0a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0A4;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06F0A4: lsl.w #$1, d0
pub fn i_06f0a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0A6;
    cpu.ir = 0xE348;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $06F0A6: move.w (a0, d0.w), d0
pub fn i_06f0a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0AA;
    cpu.ir = 0x3030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06F0AA: move.w $72(a6), d1
pub fn i_06f0aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0AE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06F0AE: muls.w d1, d0
pub fn i_06f0ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0B0;
    cpu.ir = 0xC1C1;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = ((v1 as u16 as i16 as i32).wrapping_mul(cpu.d[0] as u16 as i16 as i32)) as u32;
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $06F0B0: asr.l #$8, d0
pub fn i_06f0b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0B2;
    cpu.ir = 0xE080;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 8, Size::Long, 0, false);
    cpu.d[0] = r;
    22
}
// ===== SetTaskW_06f0b2  ($06F0B2, 6 B, kernel/task_setters_w.c)
/// $06F0B2: move.w d0, $36(a6)
pub fn i_06f0b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0B6;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F0B6: rts
pub fn i_06f0b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpriteA_06f0b8  ($06F0B8, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F0B8: lea.l $2d4c42.l, a0
pub fn i_06f0b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0BE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4C42;
    20
}
// ===== JsrAbsThunk_06f0be  ($06F0BE, 8 B, thunks/jsr_abs_thunks.c)
/// $06F0BE: jsr $28cd4.l
pub fn i_06f0be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0C4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F0C4);
    cpu.pc = 0x28CD4;
    16
}
/// $06F0C4: rts
pub fn i_06f0c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpriteB_06f0c6  ($06F0C6, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F0C6: lea.l $2d4c52.l, a0
pub fn i_06f0c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0CC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4C52;
    20
}
// ===== JsrAbsThunk_06f0cc  ($06F0CC, 8 B, thunks/jsr_abs_thunks.c)
/// $06F0CC: jsr $28cd4.l
pub fn i_06f0cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F0D2);
    cpu.pc = 0x28CD4;
    16
}
/// $06F0D2: rts
pub fn i_06f0d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0D4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpriteC_06f0d4  ($06F0D4, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F0D4: lea.l $2d4c7e.l, a0
pub fn i_06f0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0DA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4C7E;
    20
}
// ===== JsrAbsThunk_06f0da  ($06F0DA, 8 B, thunks/jsr_abs_thunks.c)
/// $06F0DA: jsr $28cd4.l
pub fn i_06f0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0E0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F0E0);
    cpu.pc = 0x28CD4;
    16
}
/// $06F0E0: rts
pub fn i_06f0e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0E2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpriteD_06f0e2  ($06F0E2, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F0E2: lea.l $2d4c9c.l, a0
pub fn i_06f0e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0E8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4C9C;
    20
}
// ===== JsrAbsThunk_06f0e8  ($06F0E8, 8 B, thunks/jsr_abs_thunks.c)
/// $06F0E8: jsr $28cd4.l
pub fn i_06f0e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F0EE);
    cpu.pc = 0x28CD4;
    16
}
/// $06F0EE: rts
pub fn i_06f0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpriteE_06f0f0  ($06F0F0, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F0F0: lea.l $2d4cc8.l, a0
pub fn i_06f0f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0F6;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4CC8;
    20
}
// ===== JsrAbsThunk_06f0f6  ($06F0F6, 8 B, thunks/jsr_abs_thunks.c)
/// $06F0F6: jsr $28cd4.l
pub fn i_06f0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0FC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F0FC);
    cpu.pc = 0x28CD4;
    16
}
/// $06F0FC: rts
pub fn i_06f0fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F0FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpriteF_06f0fe  ($06F0FE, 6 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F0FE: lea.l $2d4ce6.l, a0
pub fn i_06f0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F104;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D4CE6;
    20
}
// ===== JsrAbsThunk_06f104  ($06F104, 8 B, thunks/jsr_abs_thunks.c)
/// $06F104: jsr $28cd4.l
pub fn i_06f104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F10A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F10A);
    cpu.pc = 0x28CD4;
    16
}
/// $06F10A: rts
pub fn i_06f10a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F10C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpeedDiv128_06f10c  ($06F10C, 18 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F10C: move.w $36(a6), d0
pub fn i_06f10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F110;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06F110: asr.w #$7, d0
pub fn i_06f110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F112;
    cpu.ir = 0xEE40;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 7, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    20
}
/// $06F112: btst.b #$0, $3a(a6)
pub fn i_06f112(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F118;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06F118: bne.w $6f11e
pub fn i_06f118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F11C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F11E; return 10; }
    8
}
/// $06F11C: neg.w d0
pub fn i_06f11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F11E;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_06f11e  ($06F11E, 6 B, kernel/task_setters_w.c)
/// $06F11E: move.w d0, $2c(a6)
pub fn i_06f11e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F122;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F122: rts
pub fn i_06f122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F124;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpeedDiv16_06f124  ($06F124, 18 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F124: move.w $36(a6), d0
pub fn i_06f124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F128;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06F128: asr.w #$4, d0
pub fn i_06f128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F12A;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06F12A: btst.b #$0, $3a(a6)
pub fn i_06f12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F130;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06F130: bne.w $6f136
pub fn i_06f130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F134;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F136; return 10; }
    8
}
/// $06F134: neg.w d0
pub fn i_06f134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F136;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== SetTaskW_06f136  ($06F136, 6 B, kernel/task_setters_w.c)
/// $06F136: move.w d0, $2c(a6)
pub fn i_06f136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F13A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F13A: rts
pub fn i_06f13a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F13C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_SpawnShell_06f13c  ($06F13C, 38 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F13C: lea.l $6ee4e(pc), a1
pub fn i_06f13c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F140;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6EE4E;
    16
}
/// $06F140: jsr $4ae.l
pub fn i_06f140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F146;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F146);
    cpu.pc = 0x4AE;
    16
}
/// $06F146: jsr $5dd02.l
pub fn i_06f146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F14C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F14C);
    cpu.pc = 0x5DD02;
    16
}
/// $06F14C: move.w #$10, d0
pub fn i_06f14c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F150;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F150: btst.b #$0, $3a(a6)
pub fn i_06f150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F156;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06F156: beq.w $6f15c
pub fn i_06f156(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F15A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F15C; return 10; }
    8
}
/// $06F15A: neg.w d0
pub fn i_06f15a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F15C;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06F15C: add.w d0, $22(a0)
pub fn i_06f15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F160;
    cpu.ir = 0xD168;
    let ad1 = cpu.a[0].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r3 = add_with_flags(cpu, v2, h1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F160: rts
pub fn i_06f160(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F162;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Gunship_CmpSlotWithParent_06f162  ($06F162, 16 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F162: movea.l $8(a6), a1
pub fn i_06f162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F166;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $06F166: move.b $10(a6), d0
pub fn i_06f166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F16A;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06F16A: cmp.b $10(a1), d0
pub fn i_06f16a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F16E;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06F16E: bcs.w $6f178
pub fn i_06f16e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F172;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6F178; return 10; }
    8
}
// ===== ClearXN_06f172  ($06F172, 6 B, thunks/ccr_helpers.c)
/// $06F172: andi.b #$ee, ccr
pub fn i_06f172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F176;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $06F176: rts
pub fn i_06f176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F178;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_06f178  ($06F178, 6 B, thunks/ccr_helpers.c)
/// $06F178: ori.b #$11, ccr
pub fn i_06f178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F17C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $06F17C: rts
pub fn i_06f17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F17E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Intro_Tmpl70_06f17e  ($06F17E, 112 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F17E: move.w #$10b5, d0
pub fn i_06f17e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F182;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B5;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F182: jsr $2352.l
pub fn i_06f182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F188;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F188);
    cpu.pc = 0x2352;
    16
}
/// $06F188: ori.b #$1, $3a(a6)
pub fn i_06f188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F18E;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x1) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06F18E: move.w #$fc00, $28(a6)
pub fn i_06f18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F194;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFC00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F194: bra.w $6f1ac
pub fn i_06f194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F198;
    cpu.ir = 0x6000;
    cpu.pc = 0x6F1AC;
    10
}
/// $06F198: move.w #$10c3, d0
pub fn i_06f198(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F19C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10C3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F19C: jsr $2352.l
pub fn i_06f19c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1A2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F1A2);
    cpu.pc = 0x2352;
    16
}
/// $06F1A2: clr.b $3a(a6)
pub fn i_06f1a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1A6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06F1A6: move.w $400.w, $28(a6)
pub fn i_06f1a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1AC;
    cpu.ir = 0x3D78;
    let v2 = bus.read16(0x400) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    let ad3 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad3, v2 as u16);
    20
}
/// $06F1AC: clr.w $38(a6)
pub fn i_06f1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1B0;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F1B0: move.w #$12b, d1
pub fn i_06f1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x12B;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F1B4: jsr $236e.l
pub fn i_06f1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F1BA);
    cpu.pc = 0x236E;
    16
}
/// $06F1BA: lea.l $2d889e.l, a0
pub fn i_06f1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D889E;
    20
}
/// $06F1C0: jsr $28cd4.l
pub fn i_06f1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F1C6);
    cpu.pc = 0x28CD4;
    16
}
/// $06F1C6: lea.l $6f1cc(pc), a1
pub fn i_06f1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F1CC;
    16
}
/// $06F1CA: move.l a1, (a6)
pub fn i_06f1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F1CC: jsr $27cee.l
pub fn i_06f1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F1D2);
    cpu.pc = 0x27CEE;
    16
}
/// $06F1D2: jsr $28d70.l
pub fn i_06f1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F1D8);
    cpu.pc = 0x28D70;
    16
}
/// $06F1D8: movea.l #$ffffffff, a0
pub fn i_06f1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1DE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06F1DE: lea.l $2d531e.l, a0
pub fn i_06f1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D531E;
    20
}
/// $06F1E4: jsr $5dd5c.l
pub fn i_06f1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F1EA);
    cpu.pc = 0x5DD5C;
    16
}
/// $06F1EA: bcc.w $6f1f4
pub fn i_06f1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1EE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F1F4; return 10; }
    8
}
// ===== SetTaskHandler_06f1ee  ($06F1EE, 8 B, kernel/task_handlers.c)
/// $06F1EE: lea.l $6fb02(pc), a1
pub fn i_06f1ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1F2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $06F1F2: move.l a1, (a6)
pub fn i_06f1f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1F4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F1F4: rts
pub fn i_06f1f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Tmpl6F_06f1f6  ($06F1F6, 322 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F1F6: move.w #$59, d1
pub fn i_06f1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F1FA;
    cpu.ir = 0x323C;
    let v1: u32 = 0x59;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F1FA: jsr $236e.l
pub fn i_06f1fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F200;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F200);
    cpu.pc = 0x236E;
    16
}
/// $06F200: move.w #$b, $1c(a6)
pub fn i_06f200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F206;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F206: jsr $138fe.l
pub fn i_06f206(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F20C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F20C);
    cpu.pc = 0x138FE;
    16
}
/// $06F20C: move.l #$2d50c6, $48(a6)
pub fn i_06f20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F214;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D50C6;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06F214: move.w #$4000, d0
pub fn i_06f214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F218;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F218: jsr $28134.l
pub fn i_06f218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F21E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F21E);
    cpu.pc = 0x28134;
    16
}
/// $06F21E: andi.w #$ffe3, $38(a6)
pub fn i_06f21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F224;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06F224: ori.w #$14, $38(a6)
pub fn i_06f224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F22A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x14) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06F22A: lea.l $2b9b22.l, a0
pub fn i_06f22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F230;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9B22;
    20
}
/// $06F230: jsr $799de.l
pub fn i_06f230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F236;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F236);
    cpu.pc = 0x799DE;
    16
}
/// $06F236: lea.l $2b9ba4.l, a0
pub fn i_06f236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F23C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9BA4;
    20
}
/// $06F23C: lsl.w #$3, d0
pub fn i_06f23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F23E;
    cpu.ir = 0xE748;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06F23E: move.w (a0, d0.w), $66(a6)
pub fn i_06f23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F244;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06F244: move.w $2(a0, d0.w), $7a(a6)
pub fn i_06f244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F24A;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06F24A: move.w $4(a0, d0.w), $7c(a6)
pub fn i_06f24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F250;
    cpu.ir = 0x3D70;
    let ad2 = cpu.a[0].wrapping_add(0x4).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06F250: clr.b $73(a6)
pub fn i_06f250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F254;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06F254: move.b #$ff, $8e(a6)
pub fn i_06f254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F25A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8E);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F25A: move.b #$0, $8b(a6)
pub fn i_06f25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F260;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F260: lea.l $6fba0(pc), a1
pub fn i_06f260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F264;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FBA0;
    16
}
/// $06F264: jsr $4ae.l
pub fn i_06f264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F26A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F26A);
    cpu.pc = 0x4AE;
    16
}
/// $06F26A: jsr $5dd02.l
pub fn i_06f26a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F270;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F270);
    cpu.pc = 0x5DD02;
    16
}
/// $06F270: lea.l $6fb10(pc), a1
pub fn i_06f270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F274;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB10;
    16
}
/// $06F274: jsr $4ae.l
pub fn i_06f274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F27A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F27A);
    cpu.pc = 0x4AE;
    16
}
/// $06F27A: jsr $5dd02.l
pub fn i_06f27a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F280;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F280);
    cpu.pc = 0x5DD02;
    16
}
/// $06F280: clr.b $98(a0)
pub fn i_06f280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F284;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x98);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06F284: lea.l $6fb10(pc), a1
pub fn i_06f284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F288;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB10;
    16
}
/// $06F288: jsr $4ae.l
pub fn i_06f288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F28E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F28E);
    cpu.pc = 0x4AE;
    16
}
/// $06F28E: jsr $5dd02.l
pub fn i_06f28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F294;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F294);
    cpu.pc = 0x5DD02;
    16
}
/// $06F294: move.b #$1, $98(a0)
pub fn i_06f294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F29A;
    cpu.ir = 0x117C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F29A: lea.l $6fb10(pc), a1
pub fn i_06f29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F29E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB10;
    16
}
/// $06F29E: jsr $4ae.l
pub fn i_06f29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2A4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F2A4);
    cpu.pc = 0x4AE;
    16
}
/// $06F2A4: jsr $5dd02.l
pub fn i_06f2a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2AA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F2AA);
    cpu.pc = 0x5DD02;
    16
}
/// $06F2AA: move.b #$2, $98(a0)
pub fn i_06f2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2B0;
    cpu.ir = 0x117C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x98);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F2B0: clr.b $9b(a6)
pub fn i_06f2b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2B4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06F2B4: lea.l $682d0.l, a1
pub fn i_06f2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x682D0;
    20
}
/// $06F2BA: jsr $4ae.l
pub fn i_06f2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2C0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F2C0);
    cpu.pc = 0x4AE;
    16
}
/// $06F2C0: jsr $5dd02.l
pub fn i_06f2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F2C6);
    cpu.pc = 0x5DD02;
    16
}
/// $06F2C6: lea.l $683f0.l, a1
pub fn i_06f2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x683F0;
    20
}
/// $06F2CC: jsr $4ae.l
pub fn i_06f2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F2D2);
    cpu.pc = 0x4AE;
    16
}
/// $06F2D2: jsr $5dd02.l
pub fn i_06f2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2D8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F2D8);
    cpu.pc = 0x5DD02;
    16
}
/// $06F2D8: move.w #$200, $36(a6)
pub fn i_06f2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2DE;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F2DE: clr.b $21(a6)
pub fn i_06f2de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2E2;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x21);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06F2E2: clr.w $22(a6)
pub fn i_06f2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2E6;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x22);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F2E6: move.w #$160, $24(a6)
pub fn i_06f2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2EC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x160;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F2EC: move.b #$1, $8b(a6)
pub fn i_06f2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2F2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F2F2: move.w #$600, $28(a6)
pub fn i_06f2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x600;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F2F8: move.b #$ff, $9a(a6)
pub fn i_06f2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F2FE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F2FE: move.w #$10b9, d0
pub fn i_06f2fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F302;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10B9;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F302: jsr $2352.l
pub fn i_06f302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F308;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F308);
    cpu.pc = 0x2352;
    16
}
/// $06F308: jsr $70cbe(pc)
pub fn i_06f308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F30C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F30C);
    cpu.pc = 0x70CBE;
    16
}
/// $06F30C: lea.l $2d5e02.l, a0
pub fn i_06f30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F312;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5E02;
    20
}
/// $06F312: jsr $28cd4.l
pub fn i_06f312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F318;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F318);
    cpu.pc = 0x28CD4;
    16
}
/// $06F318: lea.l $6f31e(pc), a1
pub fn i_06f318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F31C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F31E;
    16
}
/// $06F31C: move.l a1, (a6)
pub fn i_06f31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F31E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F31E: jsr $27cee.l
pub fn i_06f31e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F324;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F324);
    cpu.pc = 0x27CEE;
    16
}
/// $06F324: jsr $27cee.l
pub fn i_06f324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F32A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F32A);
    cpu.pc = 0x27CEE;
    16
}
/// $06F32A: jsr $70ab0(pc)
pub fn i_06f32a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F32E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F32E);
    cpu.pc = 0x70AB0;
    16
}
/// $06F32E: cmpi.w #$198, $22(a6)
pub fn i_06f32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F334;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x198, Size::Word, false);
    16
}
/// $06F334: blt.w $6f33e
pub fn i_06f334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F338;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6F33E; return 10; }
    8
}
// ===== SetTaskHandler_06f338  ($06F338, 8 B, kernel/task_handlers.c)
/// $06F338: lea.l $6f340(pc), a1
pub fn i_06f338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F33C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F340;
    16
}
/// $06F33C: move.l a1, (a6)
pub fn i_06f33c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F33E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F33E: rts
pub fn i_06f33e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F340;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_WaitScroll_06f340  ($06F340, 68 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F340: clr.b $9a(a6)
pub fn i_06f340(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F344;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06F344: move.w #$78, $70(a6)
pub fn i_06f344(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F34A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x78;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F34A: move.w #$198, $22(a6)
pub fn i_06f34a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F350;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x198;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F350: move.w #$140, $24(a6)
pub fn i_06f350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F356;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F356: move.b #$1, $8b(a6)
pub fn i_06f356(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F35C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F35C: jsr $267e2.l
pub fn i_06f35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F362;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F362);
    cpu.pc = 0x267E2;
    16
}
/// $06F362: move.b #$2, $8a(a6)
pub fn i_06f362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F368;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F368: lea.l $6f36e(pc), a1
pub fn i_06f368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F36C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F36E;
    16
}
/// $06F36C: move.l a1, (a6)
pub fn i_06f36c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F36E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F36E: tst.b $21(a6)
pub fn i_06f36e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F372;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x21);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06F372: beq.w $6f38a
pub fn i_06f372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F376;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F38A; return 10; }
    8
}
/// $06F376: subq.w #$1, $70(a6)
pub fn i_06f376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F37A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F37A: cmpi.w #$0, $70(a6)
pub fn i_06f37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F380;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F380: bgt.w $6f38a
pub fn i_06f380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F384;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F38A; return 10; }
    8
}
// ===== SetTaskHandler_06f384  ($06F384, 8 B, kernel/task_handlers.c)
/// $06F384: lea.l $6f38c(pc), a1
pub fn i_06f384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F388;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F38C;
    16
}
/// $06F388: move.l a1, (a6)
pub fn i_06f388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F38A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F38A: rts
pub fn i_06f38a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F38C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_PlayMusic10BA_06f38c  ($06F38C, 10 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F38C: move.w #$10ba, d0
pub fn i_06f38c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F390;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10BA;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F390: jsr $2352.l
pub fn i_06f390(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F396;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F396);
    cpu.pc = 0x2352;
    16
}
// ===== M5Boss_Idle_06f396  ($06F396, 128 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F396: move.w #$a0, $80(a6)
pub fn i_06f396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F39C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F39C: move.w #$190, $82(a6)
pub fn i_06f39c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3A2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x190;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F3A2: lea.l $2d53f2.l, a0
pub fn i_06f3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3A8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D53F2;
    20
}
/// $06F3A8: move.b $73(a6), d0
pub fn i_06f3a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3AC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06F3AC: andi.w #$3, d0
pub fn i_06f3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3B0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06F3B0: lsl.w #$2, d0
pub fn i_06f3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3B2;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06F3B2: movea.l (a0, d0.w), a0
pub fn i_06f3b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3B6;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06F3B6: jsr $799de.l
pub fn i_06f3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3BC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F3BC);
    cpu.pc = 0x799DE;
    16
}
/// $06F3BC: move.w d0, $70(a6)
pub fn i_06f3bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F3C0: move.w #$200, $36(a6)
pub fn i_06f3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3C6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F3C6: move.b #$0, $8c(a6)
pub fn i_06f3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3CC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F3CC: move.w #$258, $94(a6)
pub fn i_06f3cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3D2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F3D2: lea.l $2d55ce.l, a0
pub fn i_06f3d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3D8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D55CE;
    20
}
/// $06F3D8: jsr $28cd4.l
pub fn i_06f3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3DE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F3DE);
    cpu.pc = 0x28CD4;
    16
}
/// $06F3DE: lea.l $6f3e4(pc), a1
pub fn i_06f3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3E2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F3E4;
    16
}
/// $06F3E2: move.l a1, (a6)
pub fn i_06f3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3E4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F3E4: jsr $28998.l
pub fn i_06f3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F3EA);
    cpu.pc = 0x28998;
    16
}
/// $06F3EA: jsr $70ad8(pc)
pub fn i_06f3ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3EE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F3EE);
    cpu.pc = 0x70AD8;
    16
}
/// $06F3EE: bcc.w $6f3f8
pub fn i_06f3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F3F8; return 10; }
    8
}
/// $06F3F2: move.b #$4, $8c(a6)
pub fn i_06f3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3F8;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F3F8: jsr $70ab0(pc)
pub fn i_06f3f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F3FC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F3FC);
    cpu.pc = 0x70AB0;
    16
}
/// $06F3FC: jsr $70c02(pc)
pub fn i_06f3fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F400;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F400);
    cpu.pc = 0x70C02;
    16
}
/// $06F400: lea.l $2d53d2.l, a0
pub fn i_06f400(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F406;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D53D2;
    20
}
/// $06F406: movea.l #$ffffffff, a1
pub fn i_06f406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F40C;
    cpu.ir = 0x227C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $06F40C: jsr $772.l
pub fn i_06f40c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F412;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F412);
    cpu.pc = 0x772;
    16
}
/// $06F412: bra.w $6fac2
pub fn i_06f412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F416;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_PickAttack_06f416  ($06F416, 264 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F416: jsr $5e9b6.l
pub fn i_06f416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F41C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F41C);
    cpu.pc = 0x5E9B6;
    16
}
/// $06F41C: andi.b #$3, d0
pub fn i_06f41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F420;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06F420: move.b d0, $88(a6)
pub fn i_06f420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F424;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F424: jsr $5e1ea.l
pub fn i_06f424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F42A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F42A);
    cpu.pc = 0x5E1EA;
    16
}
/// $06F42A: move.l a0, $9c(a6)
pub fn i_06f42a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F42E;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x9C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06F42E: clr.b d0
pub fn i_06f42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F430;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $06F430: lea.l $2d547e.l, a1
pub fn i_06f430(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F436;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D547E;
    20
}
/// $06F436: move.w $22(a0), d3
pub fn i_06f436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F43A;
    cpu.ir = 0x3628;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06F43A: cmp.w $22(a6), d3
pub fn i_06f43a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F43E;
    cpu.ir = 0xB66E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06F43E: blt.w $6f44c
pub fn i_06f43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F442;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6F44C; return 10; }
    8
}
/// $06F442: move.b #$1, d0
pub fn i_06f442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F446;
    cpu.ir = 0x103C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v1 & 0xFF);
    4
}
/// $06F446: lea.l $2d549e.l, a1
pub fn i_06f446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F44C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D549E;
    20
}
/// $06F44C: move.b d0, $87(a6)
pub fn i_06f44c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F450;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F450: move.b $88(a6), d0
pub fn i_06f450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F454;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06F454: cmpi.b #$3, d0
pub fn i_06f454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F458;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $06F458: beq.w $6f472
pub fn i_06f458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F45C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F472; return 10; }
    8
}
/// $06F45C: cmpi.b #$4, d0
pub fn i_06f45c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F460;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $06F460: beq.w $6f472
pub fn i_06f460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F464;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F472; return 10; }
    8
}
/// $06F464: cmpi.w #$164, $24(a0)
pub fn i_06f464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F46A;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x164, Size::Word, false);
    16
}
/// $06F46A: blt.w $6f472
pub fn i_06f46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F46E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6F472; return 10; }
    8
}
/// $06F46E: addi.b #$5, d0
pub fn i_06f46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F472;
    cpu.ir = 0x600;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 0x5, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $06F472: andi.w #$7, d0
pub fn i_06f472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F476;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06F476: lsl.w #$2, d0
pub fn i_06f476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F478;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06F478: move.w (a1, d0.w), $80(a6)
pub fn i_06f478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F47E;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x80);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06F47E: move.w $2(a1, d0.w), $82(a6)
pub fn i_06f47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F484;
    cpu.ir = 0x3D71;
    let ad2 = cpu.a[1].wrapping_add(0x2).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad4, v3 as u16);
    22
}
/// $06F484: move.b $88(a6), d0
pub fn i_06f484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F488;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06F488: cmpi.b #$3, d0
pub fn i_06f488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F48C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x3, Size::Byte, false);
    8
}
/// $06F48C: beq.w $6f4ac
pub fn i_06f48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F490;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F4AC; return 10; }
    8
}
/// $06F490: cmpi.b #$4, d0
pub fn i_06f490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F494;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x4, Size::Byte, false);
    8
}
/// $06F494: beq.w $6f4ac
pub fn i_06f494(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F498;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F4AC; return 10; }
    8
}
/// $06F498: jsr $5e9b6.l
pub fn i_06f498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F49E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F49E);
    cpu.pc = 0x5E9B6;
    16
}
/// $06F49E: btst.b #$0, d0
pub fn i_06f49e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4A2;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $06F4A2: beq.w $6f4ac
pub fn i_06f4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4A6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F4AC; return 10; }
    8
}
/// $06F4A6: addi.w #$28, $82(a6)
pub fn i_06f4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4AC;
    cpu.ir = 0x66E;
    let ad1 = cpu.a[6].wrapping_add(0x82);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 0x28, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    16
}
/// $06F4AC: move.w #$1074, d0
pub fn i_06f4ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4B0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1074;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F4B0: jsr $2352.l
pub fn i_06f4b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4B6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F4B6);
    cpu.pc = 0x2352;
    16
}
/// $06F4B6: move.b #$1, $8c(a6)
pub fn i_06f4b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4BC;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F4BC: move.w #$258, $94(a6)
pub fn i_06f4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4C2;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F4C2: move.w #$200, $36(a6)
pub fn i_06f4c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4C8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F4C8: lea.l $2d55da.l, a0
pub fn i_06f4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4CE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D55DA;
    20
}
/// $06F4CE: jsr $28cd4.l
pub fn i_06f4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4D4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F4D4);
    cpu.pc = 0x28CD4;
    16
}
/// $06F4D4: tst.b $9b(a6)
pub fn i_06f4d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4D8;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06F4D8: beq.w $6f4f2
pub fn i_06f4d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F4F2; return 10; }
    8
}
/// $06F4DC: addq.b #$2, $87(a6)
pub fn i_06f4dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4E0;
    cpu.ir = 0x542E;
    let ad1 = cpu.a[6].wrapping_add(0x87);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 2, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06F4E0: move.w #$400, $36(a6)
pub fn i_06f4e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4E6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x400;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F4E6: lea.l $2d5666.l, a0
pub fn i_06f4e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5666;
    20
}
/// $06F4EC: jsr $28cd4.l
pub fn i_06f4ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F4F2);
    cpu.pc = 0x28CD4;
    16
}
/// $06F4F2: lea.l $6f4f8(pc), a1
pub fn i_06f4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4F6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F4F8;
    16
}
/// $06F4F6: move.l a1, (a6)
pub fn i_06f4f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4F8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F4F8: jsr $28998.l
pub fn i_06f4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F4FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F4FE);
    cpu.pc = 0x28998;
    16
}
/// $06F4FE: jsr $70ad8(pc)
pub fn i_06f4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F502;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F502);
    cpu.pc = 0x70AD8;
    16
}
/// $06F502: bcc.w $6f50c
pub fn i_06f502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F506;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F50C; return 10; }
    8
}
/// $06F506: move.b #$3, $8c(a6)
pub fn i_06f506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F50C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F50C: jsr $70ab0(pc)
pub fn i_06f50c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F510;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F510);
    cpu.pc = 0x70AB0;
    16
}
/// $06F510: bcc.w $6f51a
pub fn i_06f510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F514;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F51A; return 10; }
    8
}
/// $06F514: lea.l $6f51e(pc), a1
pub fn i_06f514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F518;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F51E;
    16
}
/// $06F518: move.l a1, (a6)
pub fn i_06f518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F51A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F51A: bra.w $6fac2
pub fn i_06f51a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F51E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_Hover_06f51e  ($06F51E, 124 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F51E: move.w #$100, $36(a6)
pub fn i_06f51e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F524;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F524: move.b #$2, $8c(a6)
pub fn i_06f524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F52A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F52A: move.w #$258, $94(a6)
pub fn i_06f52a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F530;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F530: move.b $87(a6), d0
pub fn i_06f530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F534;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06F534: andi.w #$1, d0
pub fn i_06f534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F538;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06F538: tst.b $9b(a6)
pub fn i_06f538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F53C;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06F53C: beq.w $6f542
pub fn i_06f53c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F540;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F542; return 10; }
    8
}
/// $06F540: addq.w #$2, d0
pub fn i_06f540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F542;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06F542: movea.l #$2d53b2, a0
pub fn i_06f542(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F548;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D53B2;
    cpu.a[0] = v1;
    4
}
/// $06F548: lsl.w #$2, d0
pub fn i_06f548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F54A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06F54A: movea.l (a0, d0.w), a0
pub fn i_06f54a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F54E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06F54E: cmpa.l #$ffffffff, a0
pub fn i_06f54e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F554;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06F554: beq.w $6f55e
pub fn i_06f554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F558;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F55E; return 10; }
    8
}
/// $06F558: jsr $28cd4.l
pub fn i_06f558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F55E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F55E);
    cpu.pc = 0x28CD4;
    16
}
/// $06F55E: lea.l $6f564(pc), a1
pub fn i_06f55e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F562;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F564;
    16
}
/// $06F562: move.l a1, (a6)
pub fn i_06f562(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F564;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F564: jsr $28998.l
pub fn i_06f564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F56A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F56A);
    cpu.pc = 0x28998;
    16
}
/// $06F56A: jsr $70ad8(pc)
pub fn i_06f56a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F56E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F56E);
    cpu.pc = 0x70AD8;
    16
}
/// $06F56E: bcc.w $6f578
pub fn i_06f56e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F572;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F578; return 10; }
    8
}
/// $06F572: move.b #$3, $8c(a6)
pub fn i_06f572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F578;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F578: jsr $70ab0(pc)
pub fn i_06f578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F57C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F57C);
    cpu.pc = 0x70AB0;
    16
}
/// $06F57C: bcc.w $6f58c
pub fn i_06f57c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F580;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F58C; return 10; }
    8
}
/// $06F580: move.b #$1, $9b(a6)
pub fn i_06f580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F586;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F586: lea.l $6f396(pc), a1
pub fn i_06f586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F58A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F396;
    16
}
/// $06F58A: move.l a1, (a6)
pub fn i_06f58a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F58C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F58C: bra.w $6fac2
pub fn i_06f58c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F590;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
/// $06F590: move.b #$4, $88(a6)
pub fn i_06f590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F596;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F596: bra.w $6f424
pub fn i_06f596(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F59A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6F424;
    10
}
// ===== M5Boss_AttackA_06f59a  ($06F59A, 88 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F59A: move.b #$ff, $20(a6)
pub fn i_06f59a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5A0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F5A0: jsr $70cfe(pc)
pub fn i_06f5a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5A4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F5A4);
    cpu.pc = 0x70CFE;
    16
}
/// $06F5A4: move.b #$0, $8c(a6)
pub fn i_06f5a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5AA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F5AA: move.w #$258, $94(a6)
pub fn i_06f5aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5B0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F5B0: move.w #$1075, d0
pub fn i_06f5b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5B4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F5B4: jsr $2352.l
pub fn i_06f5b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F5BA);
    cpu.pc = 0x2352;
    16
}
/// $06F5BA: lea.l $2d5e7a.l, a0
pub fn i_06f5ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5E7A;
    20
}
/// $06F5C0: jsr $28cd4.l
pub fn i_06f5c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F5C6);
    cpu.pc = 0x28CD4;
    16
}
/// $06F5C6: lea.l $6f5cc(pc), a1
pub fn i_06f5c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5CA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F5CC;
    16
}
/// $06F5CA: move.l a1, (a6)
pub fn i_06f5ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5CC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F5CC: jsr $28998.l
pub fn i_06f5cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5D2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F5D2);
    cpu.pc = 0x28998;
    16
}
/// $06F5D2: jsr $70ad8(pc)
pub fn i_06f5d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5D6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F5D6);
    cpu.pc = 0x70AD8;
    16
}
/// $06F5D6: bcc.w $6f5e0
pub fn i_06f5d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5DA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F5E0; return 10; }
    8
}
/// $06F5DA: move.b #$4, $8c(a6)
pub fn i_06f5da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5E0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F5E0: jsr $70ab0(pc)
pub fn i_06f5e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F5E4);
    cpu.pc = 0x70AB0;
    16
}
/// $06F5E4: bcc.w $6f5ee
pub fn i_06f5e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5E8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F5EE; return 10; }
    8
}
/// $06F5E8: lea.l $6f5f2(pc), a1
pub fn i_06f5e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5EC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F5F2;
    16
}
/// $06F5EC: move.l a1, (a6)
pub fn i_06f5ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5EE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F5EE: bra.w $6fac2
pub fn i_06f5ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackAHold_06f5f2  ($06F5F2, 74 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F5F2: move.w #$b4, $70(a6)
pub fn i_06f5f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5F8;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F5F8: lea.l $6f5fe(pc), a1
pub fn i_06f5f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F5FE;
    16
}
/// $06F5FC: move.l a1, (a6)
pub fn i_06f5fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F5FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F5FE: jsr $28998.l
pub fn i_06f5fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F604;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F604);
    cpu.pc = 0x28998;
    16
}
/// $06F604: jsr $70ad8(pc)
pub fn i_06f604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F608;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F608);
    cpu.pc = 0x70AD8;
    16
}
/// $06F608: bcc.w $6f612
pub fn i_06f608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F60C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F612; return 10; }
    8
}
/// $06F60C: move.b #$4, $8c(a6)
pub fn i_06f60c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F612;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F612: jsr $70ab0(pc)
pub fn i_06f612(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F616;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F616);
    cpu.pc = 0x70AB0;
    16
}
/// $06F616: subq.w #$1, $70(a6)
pub fn i_06f616(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F61A;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F61A: cmpi.w #$0, $70(a6)
pub fn i_06f61a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F620;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F620: bgt.w $6f62a
pub fn i_06f620(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F624;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F62A; return 10; }
    8
}
/// $06F624: lea.l $6f63c(pc), a1
pub fn i_06f624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F628;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F63C;
    16
}
/// $06F628: move.l a1, (a6)
pub fn i_06f628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F62A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F62A: tst.b $20(a6)
pub fn i_06f62a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F62E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x20);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06F62E: bne.w $6f638
pub fn i_06f62e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F632;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F638; return 10; }
    8
}
/// $06F632: lea.l $6f63c(pc), a1
pub fn i_06f632(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F636;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F63C;
    16
}
/// $06F636: move.l a1, (a6)
pub fn i_06f636(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F638;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F638: bra.w $6fac2
pub fn i_06f638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F63C;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackAEnd_06f63c  ($06F63C, 66 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F63C: move.w #$1075, d0
pub fn i_06f63c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F640;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F640: jsr $2352.l
pub fn i_06f640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F646;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F646);
    cpu.pc = 0x2352;
    16
}
/// $06F646: lea.l $2d6002.l, a0
pub fn i_06f646(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F64C;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D6002;
    20
}
/// $06F64C: jsr $28cd4.l
pub fn i_06f64c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F652;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F652);
    cpu.pc = 0x28CD4;
    16
}
/// $06F652: lea.l $6f658(pc), a1
pub fn i_06f652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F656;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F658;
    16
}
/// $06F656: move.l a1, (a6)
pub fn i_06f656(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F658;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F658: jsr $28998.l
pub fn i_06f658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F65E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F65E);
    cpu.pc = 0x28998;
    16
}
/// $06F65E: jsr $70ad8(pc)
pub fn i_06f65e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F662;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F662);
    cpu.pc = 0x70AD8;
    16
}
/// $06F662: bcc.w $6f66c
pub fn i_06f662(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F666;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F66C; return 10; }
    8
}
/// $06F666: move.b #$4, $8c(a6)
pub fn i_06f666(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F66C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F66C: jsr $70ab0(pc)
pub fn i_06f66c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F670;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F670);
    cpu.pc = 0x70AB0;
    16
}
/// $06F670: bcc.w $6f67a
pub fn i_06f670(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F674;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F67A; return 10; }
    8
}
/// $06F674: lea.l $6f396(pc), a1
pub fn i_06f674(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F678;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F396;
    16
}
/// $06F678: move.l a1, (a6)
pub fn i_06f678(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F67A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F67A: bra.w $6fac2
pub fn i_06f67a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F67E;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_Charge_06f67e  ($06F67E, 94 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F67E: jsr $70cde(pc)
pub fn i_06f67e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F682;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F682);
    cpu.pc = 0x70CDE;
    16
}
/// $06F682: move.w #$12c, $70(a6)
pub fn i_06f682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F688;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x12C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F688: move.w #$200, $36(a6)
pub fn i_06f688(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F68E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F68E: move.b #$0, $8c(a6)
pub fn i_06f68e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F694;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F694: move.w #$258, $94(a6)
pub fn i_06f694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F69A;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F69A: lea.l $2d55ce.l, a0
pub fn i_06f69a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6A0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D55CE;
    20
}
/// $06F6A0: jsr $28cd4.l
pub fn i_06f6a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6A6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F6A6);
    cpu.pc = 0x28CD4;
    16
}
/// $06F6A6: lea.l $6f6ac(pc), a1
pub fn i_06f6a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F6AC;
    16
}
/// $06F6AA: move.l a1, (a6)
pub fn i_06f6aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F6AC: jsr $28998.l
pub fn i_06f6ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6B2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F6B2);
    cpu.pc = 0x28998;
    16
}
/// $06F6B2: jsr $70ad8(pc)
pub fn i_06f6b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6B6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F6B6);
    cpu.pc = 0x70AD8;
    16
}
/// $06F6B6: bcc.w $6f6c0
pub fn i_06f6b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6BA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F6C0; return 10; }
    8
}
/// $06F6BA: lea.l $6f6dc(pc), a1
pub fn i_06f6ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6BE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F6DC;
    16
}
/// $06F6BE: move.l a1, (a6)
pub fn i_06f6be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6C0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F6C0: jsr $70ab0(pc)
pub fn i_06f6c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6C4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F6C4);
    cpu.pc = 0x70AB0;
    16
}
/// $06F6C4: subq.w #$1, $70(a6)
pub fn i_06f6c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6C8;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F6C8: cmpi.w #$0, $70(a6)
pub fn i_06f6c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6CE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F6CE: bgt.w $6f6d8
pub fn i_06f6ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6D2;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F6D8; return 10; }
    8
}
/// $06F6D2: lea.l $6f6dc(pc), a1
pub fn i_06f6d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6D6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F6DC;
    16
}
/// $06F6D6: move.l a1, (a6)
pub fn i_06f6d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6D8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F6D8: bra.w $6fac2
pub fn i_06f6d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6DC;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_Barrage_06f6dc  ($06F6DC, 220 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F6DC: jsr $5e896.l
pub fn i_06f6dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F6E2);
    cpu.pc = 0x5E896;
    16
}
/// $06F6E2: bcs.w $6f70a
pub fn i_06f6e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6E6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6F70A; return 10; }
    8
}
/// $06F6E6: lea.l $2ba37e.l, a0
pub fn i_06f6e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6EC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA37E;
    20
}
/// $06F6EC: jsr $799de.l
pub fn i_06f6ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6F2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F6F2);
    cpu.pc = 0x799DE;
    16
}
/// $06F6F2: move.b d0, $72(a6)
pub fn i_06f6f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6F6;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F6F6: lea.l $2ba410.l, a0
pub fn i_06f6f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F6FC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA410;
    20
}
/// $06F6FC: jsr $799de.l
pub fn i_06f6fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F702;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F702);
    cpu.pc = 0x799DE;
    16
}
/// $06F702: move.w d0, $70(a6)
pub fn i_06f702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F706;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F706: bra.w $6f72a
pub fn i_06f706(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F70A;
    cpu.ir = 0x6000;
    cpu.pc = 0x6F72A;
    10
}
/// $06F70A: lea.l $2ba492.l, a0
pub fn i_06f70a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F710;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA492;
    20
}
/// $06F710: jsr $799de.l
pub fn i_06f710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F716;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F716);
    cpu.pc = 0x799DE;
    16
}
/// $06F716: move.b d0, $72(a6)
pub fn i_06f716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F71A;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F71A: lea.l $2ba514.l, a0
pub fn i_06f71a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F720;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA514;
    20
}
/// $06F720: jsr $799de.l
pub fn i_06f720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F726;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F726);
    cpu.pc = 0x799DE;
    16
}
/// $06F726: move.w d0, $70(a6)
pub fn i_06f726(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F72A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F72A: andi.b #$7f, $72(a6)
pub fn i_06f72a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F730;
    cpu.ir = 0x22E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & 0x7F) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06F730: bne.w $6f73a
pub fn i_06f730(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F734;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F73A; return 10; }
    8
}
/// $06F734: move.b #$1, $72(a6)
pub fn i_06f734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F73A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F73A: move.b #$3, $8c(a6)
pub fn i_06f73a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F740;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F740: lea.l $6f746(pc), a1
pub fn i_06f740(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F744;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F746;
    16
}
/// $06F744: move.l a1, (a6)
pub fn i_06f744(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F746;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F746: jsr $28998.l
pub fn i_06f746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F74C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F74C);
    cpu.pc = 0x28998;
    16
}
/// $06F74C: jsr $70ad8(pc)
pub fn i_06f74c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F750;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F750);
    cpu.pc = 0x70AD8;
    16
}
/// $06F750: jsr $70ab0(pc)
pub fn i_06f750(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F754;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F754);
    cpu.pc = 0x70AB0;
    16
}
/// $06F754: subq.w #$1, $70(a6)
pub fn i_06f754(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F758;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F758: cmpi.w #$0, $70(a6)
pub fn i_06f758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F75E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F75E: bgt.w $6f7a4
pub fn i_06f75e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F762;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F7A4; return 10; }
    8
}
/// $06F762: lea.l $2ba410.l, a0
pub fn i_06f762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F768;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA410;
    20
}
/// $06F768: jsr $799de.l
pub fn i_06f768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F76E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F76E);
    cpu.pc = 0x799DE;
    16
}
/// $06F76E: move.w d0, $70(a6)
pub fn i_06f76e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F772;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F772: jsr $5e896.l
pub fn i_06f772(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F778;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F778);
    cpu.pc = 0x5E896;
    16
}
/// $06F778: bcc.w $6f78c
pub fn i_06f778(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F77C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F78C; return 10; }
    8
}
/// $06F77C: lea.l $2ba514.l, a0
pub fn i_06f77c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F782;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2BA514;
    20
}
/// $06F782: jsr $799de.l
pub fn i_06f782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F788;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F788);
    cpu.pc = 0x799DE;
    16
}
/// $06F788: move.w d0, $70(a6)
pub fn i_06f788(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F78C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F78C: jsr $70ea2(pc)
pub fn i_06f78c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F790;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F790);
    cpu.pc = 0x70EA2;
    16
}
/// $06F790: subq.b #$1, $72(a6)
pub fn i_06f790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F794;
    cpu.ir = 0x532E;
    let ad1 = cpu.a[6].wrapping_add(0x72);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06F794: cmpi.b #$0, $72(a6)
pub fn i_06f794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F79A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06F79A: bgt.w $6f7a4
pub fn i_06f79a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F79E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F7A4; return 10; }
    8
}
/// $06F79E: lea.l $6f7b8(pc), a1
pub fn i_06f79e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7A2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F7B8;
    16
}
/// $06F7A2: move.l a1, (a6)
pub fn i_06f7a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7A4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F7A4: btst.b #$0, $13(a6)
pub fn i_06f7a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7AA;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06F7AA: beq.w $6f7b4
pub fn i_06f7aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7AE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F7B4; return 10; }
    8
}
/// $06F7AE: lea.l $6f7b8(pc), a1
pub fn i_06f7ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7B2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F7B8;
    16
}
/// $06F7B2: move.l a1, (a6)
pub fn i_06f7b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7B4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F7B4: bra.w $6fac2
pub fn i_06f7b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7B8;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_BarrageEnd_06f7b8  ($06F7B8, 56 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F7B8: move.b #$4, $8c(a6)
pub fn i_06f7b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7BE;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F7BE: move.w #$1e, $70(a6)
pub fn i_06f7be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7C4;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F7C4: lea.l $6f7ca(pc), a1
pub fn i_06f7c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7C8;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F7CA;
    16
}
/// $06F7C8: move.l a1, (a6)
pub fn i_06f7c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7CA;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F7CA: jsr $28998.l
pub fn i_06f7ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F7D0);
    cpu.pc = 0x28998;
    16
}
/// $06F7D0: jsr $70ad8(pc)
pub fn i_06f7d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7D4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F7D4);
    cpu.pc = 0x70AD8;
    16
}
/// $06F7D4: jsr $70ab0(pc)
pub fn i_06f7d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F7D8);
    cpu.pc = 0x70AB0;
    16
}
/// $06F7D8: subq.w #$1, $70(a6)
pub fn i_06f7d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7DC;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F7DC: cmpi.w #$0, $70(a6)
pub fn i_06f7dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7E2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F7E2: bgt.w $6f7ec
pub fn i_06f7e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7E6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F7EC; return 10; }
    8
}
/// $06F7E6: lea.l $6f396(pc), a1
pub fn i_06f7e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7EA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F396;
    16
}
/// $06F7EA: move.l a1, (a6)
pub fn i_06f7ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7EC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F7EC: bra.w $6fac2
pub fn i_06f7ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackB_06f7f0  ($06F7F0, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F7F0: jsr $70cfe(pc)
pub fn i_06f7f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F7F4);
    cpu.pc = 0x70CFE;
    16
}
/// $06F7F4: move.b #$0, $8c(a6)
pub fn i_06f7f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F7FA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F7FA: move.w #$258, $94(a6)
pub fn i_06f7fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F800;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F800: move.w #$1075, d0
pub fn i_06f800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F804;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F804: jsr $2352.l
pub fn i_06f804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F80A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F80A);
    cpu.pc = 0x2352;
    16
}
/// $06F80A: lea.l $2d5a78.l, a0
pub fn i_06f80a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F810;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5A78;
    20
}
/// $06F810: jsr $28cd4.l
pub fn i_06f810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F816;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F816);
    cpu.pc = 0x28CD4;
    16
}
/// $06F816: lea.l $6f81c(pc), a1
pub fn i_06f816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F81A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F81C;
    16
}
/// $06F81A: move.l a1, (a6)
pub fn i_06f81a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F81C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F81C: jsr $28998.l
pub fn i_06f81c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F822;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F822);
    cpu.pc = 0x28998;
    16
}
/// $06F822: jsr $70ad8(pc)
pub fn i_06f822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F826;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F826);
    cpu.pc = 0x70AD8;
    16
}
/// $06F826: bcc.w $6f830
pub fn i_06f826(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F82A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F830; return 10; }
    8
}
/// $06F82A: move.b #$4, $8c(a6)
pub fn i_06f82a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F830;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F830: jsr $70ab0(pc)
pub fn i_06f830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F834;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F834);
    cpu.pc = 0x70AB0;
    16
}
/// $06F834: bcc.w $6f83e
pub fn i_06f834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F838;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F83E; return 10; }
    8
}
/// $06F838: lea.l $6f842(pc), a1
pub fn i_06f838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F83C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F842;
    16
}
/// $06F83C: move.l a1, (a6)
pub fn i_06f83c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F83E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F83E: bra.w $6fac2
pub fn i_06f83e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F842;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackBLoop_06f842  ($06F842, 126 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F842: lea.l $2b9d28.l, a0
pub fn i_06f842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F848;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9D28;
    20
}
/// $06F848: jsr $799de.l
pub fn i_06f848(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F84E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F84E);
    cpu.pc = 0x799DE;
    16
}
/// $06F84E: move.b d0, $72(a6)
pub fn i_06f84e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F852;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F852: move.w #$2d, $70(a6)
pub fn i_06f852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F858;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F858: lea.l $6f85e(pc), a1
pub fn i_06f858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F85C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F85E;
    16
}
/// $06F85C: move.l a1, (a6)
pub fn i_06f85c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F85E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F85E: jsr $28998.l
pub fn i_06f85e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F864;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F864);
    cpu.pc = 0x28998;
    16
}
/// $06F864: jsr $70ad8(pc)
pub fn i_06f864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F868;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F868);
    cpu.pc = 0x70AD8;
    16
}
/// $06F868: bcc.w $6f872
pub fn i_06f868(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F86C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F872; return 10; }
    8
}
/// $06F86C: move.b #$4, $8c(a6)
pub fn i_06f86c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F872;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F872: jsr $70ab0(pc)
pub fn i_06f872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F876;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F876);
    cpu.pc = 0x70AB0;
    16
}
/// $06F876: cmpi.b #$1, $8a(a6)
pub fn i_06f876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F87C;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x8A);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06F87C: bne.w $6f88e
pub fn i_06f87c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F880;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F88E; return 10; }
    8
}
/// $06F880: cmpi.b #$0, $72(a6)
pub fn i_06f880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F886;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06F886: ble.w $6f88e
pub fn i_06f886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F88A;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x6F88E; return 10; }
    8
}
/// $06F88A: jsr $70e5a(pc)
pub fn i_06f88a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F88E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F88E);
    cpu.pc = 0x70E5A;
    16
}
/// $06F88E: cmpi.b #$0, $72(a6)
pub fn i_06f88e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F894;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06F894: bgt.w $6f8ac
pub fn i_06f894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F898;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F8AC; return 10; }
    8
}
/// $06F898: subq.w #$1, $70(a6)
pub fn i_06f898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F89C;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F89C: cmpi.w #$0, $70(a6)
pub fn i_06f89c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8A2;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F8A2: bgt.w $6f8ac
pub fn i_06f8a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8A6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F8AC; return 10; }
    8
}
/// $06F8A6: lea.l $6f8c0(pc), a1
pub fn i_06f8a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8AA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F8C0;
    16
}
/// $06F8AA: move.l a1, (a6)
pub fn i_06f8aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8AC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F8AC: btst.b #$0, $13(a6)
pub fn i_06f8ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8B2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06F8B2: beq.w $6f8bc
pub fn i_06f8b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8B6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6F8BC; return 10; }
    8
}
/// $06F8B6: lea.l $6f8c0(pc), a1
pub fn i_06f8b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8BA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F8C0;
    16
}
/// $06F8BA: move.l a1, (a6)
pub fn i_06f8ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8BC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F8BC: bra.w $6fac2
pub fn i_06f8bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackBEnd_06f8c0  ($06F8C0, 66 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F8C0: move.w #$1075, d0
pub fn i_06f8c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8C4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F8C4: jsr $2352.l
pub fn i_06f8c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8CA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F8CA);
    cpu.pc = 0x2352;
    16
}
/// $06F8CA: lea.l $2d5c00.l, a0
pub fn i_06f8ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8D0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5C00;
    20
}
/// $06F8D0: jsr $28cd4.l
pub fn i_06f8d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8D6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F8D6);
    cpu.pc = 0x28CD4;
    16
}
/// $06F8D6: lea.l $6f8dc(pc), a1
pub fn i_06f8d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8DA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F8DC;
    16
}
/// $06F8DA: move.l a1, (a6)
pub fn i_06f8da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8DC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F8DC: jsr $28998.l
pub fn i_06f8dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8E2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F8E2);
    cpu.pc = 0x28998;
    16
}
/// $06F8E2: jsr $70ad8(pc)
pub fn i_06f8e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F8E6);
    cpu.pc = 0x70AD8;
    16
}
/// $06F8E6: bcc.w $6f8f0
pub fn i_06f8e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8EA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F8F0; return 10; }
    8
}
/// $06F8EA: move.b #$4, $8c(a6)
pub fn i_06f8ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F8F0: jsr $70ab0(pc)
pub fn i_06f8f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8F4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F8F4);
    cpu.pc = 0x70AB0;
    16
}
/// $06F8F4: bcc.w $6f8fe
pub fn i_06f8f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8F8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F8FE; return 10; }
    8
}
/// $06F8F8: lea.l $6f396(pc), a1
pub fn i_06f8f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8FC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F396;
    16
}
/// $06F8FC: move.l a1, (a6)
pub fn i_06f8fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F8FE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F8FE: bra.w $6fac2
pub fn i_06f8fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F902;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackC_06f902  ($06F902, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F902: jsr $70cbe(pc)
pub fn i_06f902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F906;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F906);
    cpu.pc = 0x70CBE;
    16
}
/// $06F906: move.b #$0, $8c(a6)
pub fn i_06f906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F90C;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F90C: move.w #$258, $94(a6)
pub fn i_06f90c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F912;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F912: move.w #$1075, d0
pub fn i_06f912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F916;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1075;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06F916: jsr $2352.l
pub fn i_06f916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F91C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F91C);
    cpu.pc = 0x2352;
    16
}
/// $06F91C: lea.l $2d5cf2.l, a0
pub fn i_06f91c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F922;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5CF2;
    20
}
/// $06F922: jsr $28cd4.l
pub fn i_06f922(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F928;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F928);
    cpu.pc = 0x28CD4;
    16
}
/// $06F928: lea.l $6f92e(pc), a1
pub fn i_06f928(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F92C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F92E;
    16
}
/// $06F92C: move.l a1, (a6)
pub fn i_06f92c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F92E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F92E: jsr $28998.l
pub fn i_06f92e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F934;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F934);
    cpu.pc = 0x28998;
    16
}
/// $06F934: jsr $70ad8(pc)
pub fn i_06f934(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F938;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F938);
    cpu.pc = 0x70AD8;
    16
}
/// $06F938: bcc.w $6f942
pub fn i_06f938(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F93C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F942; return 10; }
    8
}
/// $06F93C: move.b #$4, $8c(a6)
pub fn i_06f93c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F942;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F942: jsr $70ab0(pc)
pub fn i_06f942(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F946;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F946);
    cpu.pc = 0x70AB0;
    16
}
/// $06F946: bcc.w $6f950
pub fn i_06f946(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F94A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F950; return 10; }
    8
}
/// $06F94A: lea.l $6f954(pc), a1
pub fn i_06f94a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F94E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F954;
    16
}
/// $06F94E: move.l a1, (a6)
pub fn i_06f94e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F950;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F950: bra.w $6fac2
pub fn i_06f950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F954;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAC2;
    10
}
// ===== M5Boss_AttackCLoop_06f954  ($06F954, 118 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F954: move.b #$7f, $72(a6)
pub fn i_06f954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F95A;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x7F;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F95A: move.w #$2d, $70(a6)
pub fn i_06f95a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F960;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x2D;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F960: move.b #$0, $8c(a6)
pub fn i_06f960(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F966;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F966: move.w #$258, $94(a6)
pub fn i_06f966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F96C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F96C: lea.l $6f972(pc), a1
pub fn i_06f96c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F970;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F972;
    16
}
/// $06F970: move.l a1, (a6)
pub fn i_06f970(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F972;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F972: jsr $28998.l
pub fn i_06f972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F978;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F978);
    cpu.pc = 0x28998;
    16
}
/// $06F978: jsr $70ad8(pc)
pub fn i_06f978(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F97C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F97C);
    cpu.pc = 0x70AD8;
    16
}
/// $06F97C: bcc.w $6f990
pub fn i_06f97c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F980;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6F990; return 10; }
    8
}
/// $06F980: move.b #$0, $8c(a6)
pub fn i_06f980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F986;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x8C);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F986: move.w #$258, $94(a6)
pub fn i_06f986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F98C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x258;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F98C: jsr $70cbe(pc)
pub fn i_06f98c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F990;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F990);
    cpu.pc = 0x70CBE;
    16
}
/// $06F990: jsr $70ab0(pc)
pub fn i_06f990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F994;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6F994);
    cpu.pc = 0x70AB0;
    16
}
/// $06F994: cmpi.b #$2, $73(a6)
pub fn i_06f994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F99A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $06F99A: blt.w $6f9aa
pub fn i_06f99a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F99E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6F9AA; return 10; }
    8
}
/// $06F99E: move.l #$2d50c6, $48(a6)
pub fn i_06f99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9A6;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x2D50C6;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $06F9A6: bra.w $6f9c6
pub fn i_06f9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9AA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6F9C6;
    10
}
/// $06F9AA: tst.b $72(a6)
pub fn i_06f9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9AE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x72);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06F9AE: bne.w $6f9c6
pub fn i_06f9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9B2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6F9C6; return 10; }
    8
}
/// $06F9B2: subq.w #$1, $70(a6)
pub fn i_06f9b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9B6;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06F9B6: cmpi.w #$0, $70(a6)
pub fn i_06f9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9BC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06F9BC: bgt.w $6f9c6
pub fn i_06f9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9C0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6F9C6; return 10; }
    8
}
/// $06F9C0: lea.l $6f8c0(pc), a1
pub fn i_06f9c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9C4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F8C0;
    16
}
/// $06F9C4: move.l a1, (a6)
pub fn i_06f9c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9C6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06F9C6: bra.w $6fae4
pub fn i_06f9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9CA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FAE4;
    10
}
// ===== M5Boss_Die_06f9ca  ($06F9CA, 142 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06F9CA: clr.b $106ed3.l
pub fn i_06f9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9D0;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106ED3) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106ED3, r2 as u8);
    16
}
/// $06F9D0: move.w #$200, $28(a6)
pub fn i_06f9d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9D6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x200;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F9D6: move.w #$ff80, $2a(a6)
pub fn i_06f9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9DC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F9DC: move.b #$3, $73(a6)
pub fn i_06f9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9E2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06F9E2: lea.l $2d544c.l, a1
pub fn i_06f9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9E8;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x2D544C;
    20
}
/// $06F9E8: jsr $77c7e.l
pub fn i_06f9e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9EE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6F9EE);
    cpu.pc = 0x77C7E;
    16
}
/// $06F9EE: move.w #$c000, $38(a0)
pub fn i_06f9ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9F4;
    cpu.ir = 0x317C;
    let v1: u32 = 0xC000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06F9F4: lea.l $2d60f6.l, a0
pub fn i_06f9f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6F9FA;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D60F6;
    20
}
/// $06F9FA: jsr $28cd4.l
pub fn i_06f9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA00;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA00);
    cpu.pc = 0x28CD4;
    16
}
/// $06FA00: lea.l $6fa06(pc), a1
pub fn i_06fa00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA04;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FA06;
    16
}
/// $06FA04: move.l a1, (a6)
pub fn i_06fa04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA06;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FA06: bset.b #$6, $13(a6)
pub fn i_06fa06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA0C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FA0C: jsr $27cee.l
pub fn i_06fa0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA12;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA12);
    cpu.pc = 0x27CEE;
    16
}
/// $06FA12: bcc.w $6fa1a
pub fn i_06fa12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA16;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FA1A; return 10; }
    8
}
/// $06FA16: clr.w $2a(a6)
pub fn i_06fa16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA1A;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x2A);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FA1A: cmpi.w #$160, $22(a6)
pub fn i_06fa1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA20;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x160, Size::Word, false);
    16
}
/// $06FA20: blt.w $6fa30
pub fn i_06fa20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA24;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6FA30; return 10; }
    8
}
/// $06FA24: jsr $267e2.l
pub fn i_06fa24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA2A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA2A);
    cpu.pc = 0x267E2;
    16
}
/// $06FA2A: lea.l $6fa60(pc), a1
pub fn i_06fa2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA2E;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FA60;
    16
}
/// $06FA2E: move.l a1, (a6)
pub fn i_06fa2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA30;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FA30: cmpi.w #$140, $24(a6)
pub fn i_06fa30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA36;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x140, Size::Word, false);
    16
}
/// $06FA36: bgt.w $6fa40
pub fn i_06fa36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA3A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FA40; return 10; }
    8
}
/// $06FA3A: move.w #$40, $2a(a6)
pub fn i_06fa3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA40;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x40;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FA40: cmpi.w #$170, $24(a6)
pub fn i_06fa40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA46;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x170, Size::Word, false);
    16
}
/// $06FA46: blt.w $6fa50
pub fn i_06fa46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA4A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6FA50; return 10; }
    8
}
/// $06FA4A: move.w #$ffc0, $2a(a6)
pub fn i_06fa4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA50;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFC0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FA50: jsr $70ab0(pc)
pub fn i_06fa50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA54;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FA54);
    cpu.pc = 0x70AB0;
    16
}
/// $06FA54: bcc.w $6fa5e
pub fn i_06fa54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA58;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FA5E; return 10; }
    8
}
// ===== SetTaskHandler_06fa58  ($06FA58, 8 B, kernel/task_handlers.c)
/// $06FA58: lea.l $6fa60(pc), a1
pub fn i_06fa58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA5C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FA60;
    16
}
/// $06FA5C: move.l a1, (a6)
pub fn i_06fa5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA5E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FA5E: rts
pub fn i_06fa5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA60;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Explode_06fa60  ($06FA60, 84 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FA60: move.w $22(a6), $5c(a6)
pub fn i_06fa60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA66;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x5C);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06FA66: move.w #$120, $22(a6)
pub fn i_06fa66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA6C;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x120;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FA6C: move.w #$1033, d0
pub fn i_06fa6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA70;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1033;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FA70: jsr $2352.l
pub fn i_06fa70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA76;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA76);
    cpu.pc = 0x2352;
    16
}
/// $06FA76: move.b #$4, $10a2d0.l
pub fn i_06fa76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA7E;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D0, v1 as u8);
    16
}
/// $06FA7E: move.b #$0, $10a2d1.l
pub fn i_06fa7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA86;
    cpu.ir = 0x13FC;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x10A2D1, v1 as u8);
    16
}
/// $06FA86: lea.l $77fd6.l, a1
pub fn i_06fa86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA8C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x77FD6;
    20
}
/// $06FA8C: jsr $4ae.l
pub fn i_06fa8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA92;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA92);
    cpu.pc = 0x4AE;
    16
}
/// $06FA92: jsr $5dd02.l
pub fn i_06fa92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA98;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FA98);
    cpu.pc = 0x5DD02;
    16
}
/// $06FA98: addq.w #$1, $38(a0)
pub fn i_06fa98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FA9C;
    cpu.ir = 0x5268;
    let ad1 = cpu.a[0].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FA9C: jsr $628ea.l
pub fn i_06fa9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAA2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FAA2);
    cpu.pc = 0x628EA;
    16
}
/// $06FAA2: jsr $628ea.l
pub fn i_06faa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAA8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FAA8);
    cpu.pc = 0x628EA;
    16
}
/// $06FAA8: move.w $5c(a6), $22(a6)
pub fn i_06faa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAAE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x5C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $06FAAE: lea.l $6faba(pc), a1
pub fn i_06faae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FABA;
    16
}
/// $06FAB2: move.l a1, (a6)
pub fn i_06fab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
// ===== JsrPcThunk_06fab4  ($06FAB4, 6 B, thunks/jsr_pc_thunks.c)
/// $06FAB4: jsr $70ab0(pc)
pub fn i_06fab4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FAB8);
    cpu.pc = 0x70AB0;
    16
}
/// $06FAB8: rts
pub fn i_06fab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FABA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06faba  ($06FABA, 8 B, kernel/jmp_to_scheduler.c)
/// $06FABA: jmp $518.l
pub fn i_06faba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAC0;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06FAC0: rts
pub fn i_06fac0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAC2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_HitTest_06fac2  ($06FAC2, 34 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FAC2: jsr $70c4e(pc)
pub fn i_06fac2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAC6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FAC6);
    cpu.pc = 0x70C4E;
    16
}
/// $06FAC6: jsr $28758.l
pub fn i_06fac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FACC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FACC);
    cpu.pc = 0x28758;
    16
}
/// $06FACC: bcc.w $6fae2
pub fn i_06facc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAD0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FAE2; return 10; }
    8
}
/// $06FAD0: bclr.b #$0, $13(a6)
pub fn i_06fad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAD6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FAD6: move.w #$1, $66(a6)
pub fn i_06fad6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FADC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FADC: move.b #$2, $73(a6)
pub fn i_06fadc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAE2;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06FAE2: rts
pub fn i_06fae2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAE4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_HitTestB_06fae4  ($06FAE4, 14 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FAE4: jsr $70c4e(pc)
pub fn i_06fae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAE8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FAE8);
    cpu.pc = 0x70C4E;
    16
}
/// $06FAE8: jsr $28758.l
pub fn i_06fae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FAEE);
    cpu.pc = 0x28758;
    16
}
/// $06FAEE: bcc.w $6faf8
pub fn i_06faee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAF2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FAF8; return 10; }
    8
}
// ===== SetTaskHandler_06faf2  ($06FAF2, 8 B, kernel/task_handlers.c)
/// $06FAF2: lea.l $6f9ca(pc), a1
pub fn i_06faf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAF6;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6F9CA;
    16
}
/// $06FAF6: move.l a1, (a6)
pub fn i_06faf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAF8;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FAF8: rts
pub fn i_06faf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FAFA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JmpToScheduler_06fafa  ($06FAFA, 8 B, kernel/jmp_to_scheduler.c)
/// $06FAFA: jmp $518.l
pub fn i_06fafa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB00;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06FB00: rts
pub fn i_06fb00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB02;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Jsr5B6ThenJmpScheduler_06fb02  ($06FB02, 14 B, kernel/jsr5b6_jmp518.c)
/// $06FB02: jsr $5b6.l
pub fn i_06fb02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB08;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB08);
    cpu.pc = 0x5B6;
    16
}
/// $06FB08: jmp $518.l
pub fn i_06fb08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB0E;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
/// $06FB0E: rts
pub fn i_06fb0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB10;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Part_06fb10  ($06FB10, 30 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FB10: move.w #$59, d1
pub fn i_06fb10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB14;
    cpu.ir = 0x323C;
    let v1: u32 = 0x59;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FB14: jsr $236e.l
pub fn i_06fb14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB1A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB1A);
    cpu.pc = 0x236E;
    16
}
/// $06FB1A: move.w #$b, $1c(a6)
pub fn i_06fb1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB20;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FB20: jsr $138fe.l
pub fn i_06fb20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB26;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB26);
    cpu.pc = 0x138FE;
    16
}
/// $06FB26: clr.b $73(a6)
pub fn i_06fb26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB2A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FB2A: clr.b $9b(a6)
pub fn i_06fb2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB2E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x9B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
// ===== M5Boss_PartFollow_06fb2e  ($06FB2E, 106 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FB2E: move.b $98(a6), d0
pub fn i_06fb2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB32;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FB32: andi.w #$3, d0
pub fn i_06fb32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB36;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FB36: lsl.w #$2, d0
pub fn i_06fb36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB38;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FB38: move.b $73(a6), d1
pub fn i_06fb38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB3C;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FB3C: andi.w #$3, d1
pub fn i_06fb3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB40;
    cpu.ir = 0x241;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FB40: add.w d1, d0
pub fn i_06fb40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB42;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FB42: movea.l #$2d5372, a0
pub fn i_06fb42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB48;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D5372;
    cpu.a[0] = v1;
    4
}
/// $06FB48: lsl.w #$2, d0
pub fn i_06fb48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB4A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FB4A: movea.l (a0, d0.w), a0
pub fn i_06fb4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB4E;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06FB4E: cmpa.l #$ffffffff, a0
pub fn i_06fb4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB54;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06FB54: beq.w $6fb5e
pub fn i_06fb54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB58;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FB5E; return 10; }
    8
}
/// $06FB58: jsr $28cd4.l
pub fn i_06fb58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB5E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB5E);
    cpu.pc = 0x28CD4;
    16
}
/// $06FB5E: lea.l $6fb64(pc), a1
pub fn i_06fb5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB62;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB64;
    16
}
/// $06FB62: move.l a1, (a6)
pub fn i_06fb62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB64;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FB64: jsr $70d5a(pc)
pub fn i_06fb64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB68;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FB68);
    cpu.pc = 0x70D5A;
    16
}
/// $06FB68: jsr $28d70.l
pub fn i_06fb68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB6E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB6E);
    cpu.pc = 0x28D70;
    16
}
/// $06FB6E: movea.l $c(a6), a0
pub fn i_06fb6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB72;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FB72: move.b $9b(a0), $9b(a6)
pub fn i_06fb72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB78;
    cpu.ir = 0x1D68;
    let ad2 = cpu.a[0].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x9B);
    bus.write8(ad4, v3 as u8);
    20
}
/// $06FB78: move.b $73(a0), d0
pub fn i_06fb78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB7C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FB7C: cmp.b $73(a6), d0
pub fn i_06fb7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB80;
    cpu.ir = 0xB02E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $06FB80: beq.w $6fb8e
pub fn i_06fb80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB84;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FB8E; return 10; }
    8
}
/// $06FB84: move.b d0, $73(a6)
pub fn i_06fb84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB88;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    bus.write8(ad2, v1 as u8);
    12
}
/// $06FB88: lea.l $6fb2e(pc), a1
pub fn i_06fb88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB8C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB2E;
    16
}
/// $06FB8C: move.l a1, (a6)
pub fn i_06fb8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB8E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FB8E: jsr $5e45a.l
pub fn i_06fb8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB94;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FB94);
    cpu.pc = 0x5E45A;
    16
}
/// $06FB94: bcc.w $6fb9e
pub fn i_06fb94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB98;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FB9E; return 10; }
    8
}
// ===== SetTaskHandler_06fb98  ($06FB98, 8 B, kernel/task_handlers.c)
/// $06FB98: lea.l $6fb02(pc), a1
pub fn i_06fb98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB9C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $06FB9C: move.l a1, (a6)
pub fn i_06fb9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FB9E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FB9E: rts
pub fn i_06fb9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBA0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_Rotor_06fba0  ($06FBA0, 168 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FBA0: move.w #$59, d1
pub fn i_06fba0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBA4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x59;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FBA4: jsr $236e.l
pub fn i_06fba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBAA);
    cpu.pc = 0x236E;
    16
}
/// $06FBAA: move.w #$b, $1c(a6)
pub fn i_06fbaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBB0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FBB0: jsr $138fe.l
pub fn i_06fbb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBB6);
    cpu.pc = 0x138FE;
    16
}
/// $06FBB6: move.w #$3c, $70(a6)
pub fn i_06fbb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBBC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FBBC: lea.l $2d61a8.l, a0
pub fn i_06fbbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBC2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D61A8;
    20
}
/// $06FBC2: jsr $28cd4.l
pub fn i_06fbc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBC8);
    cpu.pc = 0x28CD4;
    16
}
/// $06FBC8: lea.l $6fbce(pc), a1
pub fn i_06fbc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBCC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FBCE;
    16
}
/// $06FBCC: move.l a1, (a6)
pub fn i_06fbcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBCE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FBCE: jsr $70d5a(pc)
pub fn i_06fbce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBD2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FBD2);
    cpu.pc = 0x70D5A;
    16
}
/// $06FBD2: jsr $28d70.l
pub fn i_06fbd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBD8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBD8);
    cpu.pc = 0x28D70;
    16
}
/// $06FBD8: bcc.w $6fbe8
pub fn i_06fbd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBDC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FBE8; return 10; }
    8
}
/// $06FBDC: lea.l $2d61a8.l, a0
pub fn i_06fbdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBE2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D61A8;
    20
}
/// $06FBE2: jsr $28cd4.l
pub fn i_06fbe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBE8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FBE8);
    cpu.pc = 0x28CD4;
    16
}
/// $06FBE8: movea.l $c(a6), a0
pub fn i_06fbe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBEC;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FBEC: cmpi.b #$2, $8b(a0)
pub fn i_06fbec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBF2;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $06FBF2: bne.w $6fbfc
pub fn i_06fbf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FBFC; return 10; }
    8
}
/// $06FBF6: lea.l $6fc50(pc), a1
pub fn i_06fbf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBFA;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FC50;
    16
}
/// $06FBFA: move.l a1, (a6)
pub fn i_06fbfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FBFC;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FBFC: cmpi.b #$3, $8b(a0)
pub fn i_06fbfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC02;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $06FC02: bne.w $6fc0c
pub fn i_06fc02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC06;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FC0C; return 10; }
    8
}
/// $06FC06: lea.l $6fca2(pc), a1
pub fn i_06fc06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC0A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FCA2;
    16
}
/// $06FC0A: move.l a1, (a6)
pub fn i_06fc0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC0C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC0C: movea.l $c(a6), a0
pub fn i_06fc0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC10;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FC10: cmpi.b #$0, $8b(a0)
pub fn i_06fc10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC16;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $06FC16: beq.w $6fc3e
pub fn i_06fc16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC1A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FC3E; return 10; }
    8
}
/// $06FC1A: subq.w #$1, $70(a6)
pub fn i_06fc1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC1E;
    cpu.ir = 0x536E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 1, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FC1E: cmpi.w #$0, $70(a6)
pub fn i_06fc1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Word, false);
    16
}
/// $06FC24: bgt.w $6fc3e
pub fn i_06fc24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC28;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FC3E; return 10; }
    8
}
/// $06FC28: lea.l $708c0(pc), a1
pub fn i_06fc28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x708C0;
    16
}
/// $06FC2C: jsr $4ae.l
pub fn i_06fc2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC32);
    cpu.pc = 0x4AE;
    16
}
/// $06FC32: jsr $5dd02.l
pub fn i_06fc32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC38;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC38);
    cpu.pc = 0x5DD02;
    16
}
/// $06FC38: move.w #$3c, $70(a6)
pub fn i_06fc38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC3E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3C;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FC3E: jsr $5e45a.l
pub fn i_06fc3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC44);
    cpu.pc = 0x5E45A;
    16
}
/// $06FC44: bcc.w $6fc4e
pub fn i_06fc44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC48;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FC4E; return 10; }
    8
}
// ===== SetTaskHandler_06fc48  ($06FC48, 8 B, kernel/task_handlers.c)
/// $06FC48: lea.l $6fb02(pc), a1
pub fn i_06fc48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC4C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FB02;
    16
}
/// $06FC4C: move.l a1, (a6)
pub fn i_06fc4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC4E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC4E: rts
pub fn i_06fc4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC50;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_RotorA_06fc50  ($06FC50, 82 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FC50: lea.l $2d638a.l, a0
pub fn i_06fc50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC56;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D638A;
    20
}
/// $06FC56: jsr $28cd4.l
pub fn i_06fc56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC5C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC5C);
    cpu.pc = 0x28CD4;
    16
}
/// $06FC5C: lea.l $6fc62(pc), a1
pub fn i_06fc5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC60;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FC62;
    16
}
/// $06FC60: move.l a1, (a6)
pub fn i_06fc60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC62;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC62: jsr $70d5a(pc)
pub fn i_06fc62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC66;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FC66);
    cpu.pc = 0x70D5A;
    16
}
/// $06FC66: jsr $28d70.l
pub fn i_06fc66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC6C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FC6C);
    cpu.pc = 0x28D70;
    16
}
/// $06FC6C: bcc.w $6fc76
pub fn i_06fc6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC70;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FC76; return 10; }
    8
}
/// $06FC70: lea.l $6fc50(pc), a1
pub fn i_06fc70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC74;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FC50;
    16
}
/// $06FC74: move.l a1, (a6)
pub fn i_06fc74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC76;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC76: movea.l $c(a6), a0
pub fn i_06fc76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC7A;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FC7A: cmpi.b #$1, $8b(a0)
pub fn i_06fc7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC80;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06FC80: bne.w $6fc8a
pub fn i_06fc80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC84;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FC8A; return 10; }
    8
}
/// $06FC84: lea.l $6fbb6(pc), a1
pub fn i_06fc84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC88;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FBB6;
    16
}
/// $06FC88: move.l a1, (a6)
pub fn i_06fc88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC8A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC8A: cmpi.b #$3, $8b(a0)
pub fn i_06fc8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC90;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x3, Size::Byte, false);
    16
}
/// $06FC90: bne.w $6fc9a
pub fn i_06fc90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC94;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FC9A; return 10; }
    8
}
/// $06FC94: lea.l $6fca2(pc), a1
pub fn i_06fc94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC98;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FCA2;
    16
}
/// $06FC98: move.l a1, (a6)
pub fn i_06fc98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC9A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FC9A: subq.w #$3, $70(a6)
pub fn i_06fc9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FC9E;
    cpu.ir = 0x576E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 3, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FC9E: bra.w $6fc0c
pub fn i_06fc9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FC0C;
    10
}
// ===== M5Boss_RotorB_06fca2  ($06FCA2, 88 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FCA2: lea.l $2d638a.l, a0
pub fn i_06fca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCA8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D638A;
    20
}
/// $06FCA8: jsr $28cd4.l
pub fn i_06fca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCAE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FCAE);
    cpu.pc = 0x28CD4;
    16
}
/// $06FCAE: lea.l $6fcb4(pc), a1
pub fn i_06fcae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCB2;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FCB4;
    16
}
/// $06FCB2: move.l a1, (a6)
pub fn i_06fcb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCB4;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FCB4: jsr $70d5a(pc)
pub fn i_06fcb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCB8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x6FCB8);
    cpu.pc = 0x70D5A;
    16
}
/// $06FCB8: jsr $28d70.l
pub fn i_06fcb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCBE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FCBE);
    cpu.pc = 0x28D70;
    16
}
/// $06FCBE: bcc.w $6fcce
pub fn i_06fcbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCC2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FCCE; return 10; }
    8
}
/// $06FCC2: lea.l $2d638a.l, a0
pub fn i_06fcc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCC8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D638A;
    20
}
/// $06FCC8: jsr $28cd4.l
pub fn i_06fcc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCCE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FCCE);
    cpu.pc = 0x28CD4;
    16
}
/// $06FCCE: movea.l $c(a6), a0
pub fn i_06fcce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCD2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FCD2: cmpi.b #$1, $8b(a0)
pub fn i_06fcd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCD8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x1, Size::Byte, false);
    16
}
/// $06FCD8: bne.w $6fce2
pub fn i_06fcd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FCE2; return 10; }
    8
}
/// $06FCDC: lea.l $6fbb6(pc), a1
pub fn i_06fcdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCE0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FBB6;
    16
}
/// $06FCE0: move.l a1, (a6)
pub fn i_06fce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCE2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FCE2: cmpi.b #$2, $8b(a0)
pub fn i_06fce2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCE8;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $06FCE8: bne.w $6fcf2
pub fn i_06fce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCEC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x6FCF2; return 10; }
    8
}
/// $06FCEC: lea.l $6fc50(pc), a1
pub fn i_06fcec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCF0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FC50;
    16
}
/// $06FCF0: move.l a1, (a6)
pub fn i_06fcf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCF2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FCF2: subq.w #$7, $70(a6)
pub fn i_06fcf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCF6;
    cpu.ir = 0x5F6E;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read16(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 7, Size::Word, true);
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FCF6: bra.w $6fc0c
pub fn i_06fcf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCFA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FC0C;
    10
}
// ===== M5Boss_Bomb_06fcfa  ($06FCFA, 238 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FCFA: move.w #$10a3, d0
pub fn i_06fcfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FCFE;
    cpu.ir = 0x303C;
    let v1: u32 = 0x10A3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FCFE: jsr $2352.l
pub fn i_06fcfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD04;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD04);
    cpu.pc = 0x2352;
    16
}
/// $06FD04: bset.b #$4, $6b(a6)
pub fn i_06fd04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD0A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FD0A: lea.l $2b9c24.l, a0
pub fn i_06fd0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD10;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9C24;
    20
}
/// $06FD10: jsr $799de.l
pub fn i_06fd10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD16;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD16);
    cpu.pc = 0x799DE;
    16
}
/// $06FD16: move.w d0, $66(a6)
pub fn i_06fd16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD1A;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FD1A: move.w d0, d1
pub fn i_06fd1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD1C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FD1C: lsr.w #$2, d1
pub fn i_06fd1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD1E;
    cpu.ir = 0xE449;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FD1E: move.w d1, $7c(a6)
pub fn i_06fd1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD22;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FD22: sub.w d1, d0
pub fn i_06fd22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD24;
    cpu.ir = 0x9041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FD24: move.w d0, $7a(a6)
pub fn i_06fd24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD28;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FD28: move.w #$5a, d1
pub fn i_06fd28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD2C;
    cpu.ir = 0x323C;
    let v1: u32 = 0x5A;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FD2C: jsr $236e.l
pub fn i_06fd2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD32;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD32);
    cpu.pc = 0x236E;
    16
}
/// $06FD32: move.w #$b, $1c(a6)
pub fn i_06fd32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD38;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xB;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FD38: jsr $138fe.l
pub fn i_06fd38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD3E);
    cpu.pc = 0x138FE;
    16
}
/// $06FD3E: clr.b $84(a6)
pub fn i_06fd3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD42;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FD42: clr.b $85(a6)
pub fn i_06fd42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD46;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x85);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FD46: clr.b $89(a6)
pub fn i_06fd46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD4A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x89);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FD4A: move.b $87(a6), d0
pub fn i_06fd4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD4E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x87);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FD4E: andi.w #$1, d0
pub fn i_06fd4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD52;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FD52: tst.b $9b(a6)
pub fn i_06fd52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD56;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x9B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06FD56: beq.w $6fd5c
pub fn i_06fd56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FD5C; return 10; }
    8
}
/// $06FD5A: addq.w #$2, d0
pub fn i_06fd5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD5C;
    cpu.ir = 0x5440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06FD5C: movea.l #$2d53c2, a0
pub fn i_06fd5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD62;
    cpu.ir = 0x207C;
    let v1: u32 = 0x2D53C2;
    cpu.a[0] = v1;
    4
}
/// $06FD62: lsl.w #$2, d0
pub fn i_06fd62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD64;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FD64: movea.l (a0, d0.w), a0
pub fn i_06fd64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD68;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $06FD68: cmpa.l #$ffffffff, a0
pub fn i_06fd68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD6E;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $06FD6E: beq.w $6fd78
pub fn i_06fd6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD72;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FD78; return 10; }
    8
}
/// $06FD72: jsr $28cd4.l
pub fn i_06fd72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD78;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FD78);
    cpu.pc = 0x28CD4;
    16
}
/// $06FD78: lea.l $6fd7e(pc), a1
pub fn i_06fd78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FD7E;
    16
}
/// $06FD7C: move.l a1, (a6)
pub fn i_06fd7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FD7E: movea.l $c(a6), a0
pub fn i_06fd7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD82;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $06FD82: move.w $22(a0), d1
pub fn i_06fd82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD86;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FD86: move.w $24(a0), d2
pub fn i_06fd86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD8A;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FD8A: move.b $84(a6), d3
pub fn i_06fd8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD8E;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FD8E: move.b $85(a6), d4
pub fn i_06fd8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD92;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x85);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FD92: ext.w d3
pub fn i_06fd92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD94;
    cpu.ir = 0x4883;
    let v = cpu.d[3] as i8 as i16 as u16 as u32; cpu.d[3] = (cpu.d[3] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06FD94: ext.w d4
pub fn i_06fd94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD96;
    cpu.ir = 0x4884;
    let v = cpu.d[4] as i8 as i16 as u16 as u32; cpu.d[4] = (cpu.d[4] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $06FD96: add.w d3, d1
pub fn i_06fd96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD98;
    cpu.ir = 0xD243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FD98: add.w d4, d2
pub fn i_06fd98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD9A;
    cpu.ir = 0xD444;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FD9A: addi.w #$2c, d2
pub fn i_06fd9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FD9E;
    cpu.ir = 0x642;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2C, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FD9E: move.w d1, $22(a6)
pub fn i_06fd9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDA2;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FDA2: move.w d2, $24(a6)
pub fn i_06fda2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDA6;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FDA6: move.w $38(a0), d0
pub fn i_06fda6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDAA;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FDAA: subq.w #$2, d0
pub fn i_06fdaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDAC;
    cpu.ir = 0x5540;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06FDAC: move.w d0, $38(a6)
pub fn i_06fdac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDB0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FDB0: jsr $28d70.l
pub fn i_06fdb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FDB6);
    cpu.pc = 0x28D70;
    16
}
/// $06FDB6: bcc.w $6fde6
pub fn i_06fdb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDBA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FDE6; return 10; }
    8
}
/// $06FDBA: lea.l $6fde8(pc), a1
pub fn i_06fdba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDBE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FDE8;
    16
}
/// $06FDBE: move.l a1, (a6)
pub fn i_06fdbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDC0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FDC0: cmpi.b #$4, $88(a6)
pub fn i_06fdc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDC6;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x4, Size::Byte, false);
    16
}
/// $06FDC6: bcs.w $6fdd0
pub fn i_06fdc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDCA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6FDD0; return 10; }
    8
}
/// $06FDCA: lea.l $6ffca(pc), a1
pub fn i_06fdca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDCE;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FFCA;
    16
}
/// $06FDCE: move.l a1, (a6)
pub fn i_06fdce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDD0;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FDD0: move.w #$8000, d0
pub fn i_06fdd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDD4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FDD4: jsr $28134.l
pub fn i_06fdd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDDA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FDDA);
    cpu.pc = 0x28134;
    16
}
/// $06FDDA: andi.w #$ffe3, $38(a6)
pub fn i_06fdda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDE0;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06FDE0: ori.w #$1c, $38(a6)
pub fn i_06fde0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDE6;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0x1C) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $06FDE6: rts
pub fn i_06fde6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDE8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_BombFall_06fde8  ($06FDE8, 256 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FDE8: jsr $267e2.l
pub fn i_06fde8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDEE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FDEE);
    cpu.pc = 0x267E2;
    16
}
/// $06FDEE: lea.l $2b9ca6.l, a0
pub fn i_06fdee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDF4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B9CA6;
    20
}
/// $06FDF4: jsr $799de.l
pub fn i_06fdf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDFA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FDFA);
    cpu.pc = 0x799DE;
    16
}
/// $06FDFA: move.w d0, $36(a6)
pub fn i_06fdfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FDFE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x36);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FDFE: tst.b $34(a6)
pub fn i_06fdfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE02;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06FE02: beq.w $6fe08
pub fn i_06fe02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE06;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FE08; return 10; }
    8
}
/// $06FE06: neg.w d0
pub fn i_06fe06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE08;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $06FE08: move.w d0, $28(a6)
pub fn i_06fe08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE0C;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FE0C: move.b $88(a6), d0
pub fn i_06fe0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE10;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x88);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FE10: andi.w #$3, d0
pub fn i_06fe10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE14;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FE14: lea.l $2d545e.l, a0
pub fn i_06fe14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE1A;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D545E;
    20
}
/// $06FE1A: tst.b $34(a6)
pub fn i_06fe1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE1E;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06FE1E: beq.w $6fe28
pub fn i_06fe1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE22;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FE28; return 10; }
    8
}
/// $06FE22: lea.l $2d546e.l, a0
pub fn i_06fe22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE28;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D546E;
    20
}
/// $06FE28: lsl.w #$2, d0
pub fn i_06fe28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE2A;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $06FE2A: move.l (a0, d0.w), $90(a6)
pub fn i_06fe2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE30;
    cpu.ir = 0x2D70;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x90);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $06FE30: clr.w $94(a6)
pub fn i_06fe30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE34;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $06FE34: clr.b $96(a6)
pub fn i_06fe34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE38;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x96);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FE38: lea.l $2d6d70.l, a0
pub fn i_06fe38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE3E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D6D70;
    20
}
/// $06FE3E: jsr $28cd4.l
pub fn i_06fe3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE44);
    cpu.pc = 0x28CD4;
    16
}
/// $06FE44: lea.l $6fe4a(pc), a1
pub fn i_06fe44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE48;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FE4A;
    16
}
/// $06FE48: move.l a1, (a6)
pub fn i_06fe48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE4A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FE4A: jsr $78f90.l
pub fn i_06fe4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE50);
    cpu.pc = 0x78F90;
    16
}
/// $06FE50: move.b $34(a6), d0
pub fn i_06fe50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE54;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FE54: andi.w #$ff, d0
pub fn i_06fe54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE58;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FE58: lsr.w #$3, d0
pub fn i_06fe58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE5A;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FE5A: move.w d0, $76(a6)
pub fn i_06fe5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE5E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FE5E: jsr $28d70.l
pub fn i_06fe5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE64;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE64);
    cpu.pc = 0x28D70;
    16
}
/// $06FE64: jsr $283d8.l
pub fn i_06fe64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE6A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE6A);
    cpu.pc = 0x283D8;
    16
}
/// $06FE6A: btst.b #$1, $13(a6)
pub fn i_06fe6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE70;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $06FE70: beq.w $6fe7a
pub fn i_06fe70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE74;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FE7A; return 10; }
    8
}
/// $06FE74: lea.l $70074(pc), a1
pub fn i_06fe74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE78;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FE78: move.l a1, (a6)
pub fn i_06fe78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE7A;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FE7A: jsr $2870a.l
pub fn i_06fe7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE80;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FE80);
    cpu.pc = 0x2870A;
    16
}
/// $06FE80: bcc.w $6fec2
pub fn i_06fe80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE84;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FEC2; return 10; }
    8
}
/// $06FE84: bclr.b #$3, $13(a6)
pub fn i_06fe84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE8A;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FE8A: move.w $7a(a6), d0
pub fn i_06fe8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE8E;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FE8E: cmp.w $66(a6), d0
pub fn i_06fe8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE92;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $06FE92: bcs.w $6fec2
pub fn i_06fe92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE96;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x6FEC2; return 10; }
    8
}
/// $06FE96: sub.w $7c(a6), d0
pub fn i_06fe96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE9A;
    cpu.ir = 0x906E;
    let ad2 = cpu.a[6].wrapping_add(0x7C);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FE9A: move.w d0, $7a(a6)
pub fn i_06fe9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FE9E;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x7A);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FE9E: jsr $13600.l
pub fn i_06fe9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEA4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEA4);
    cpu.pc = 0x13600;
    16
}
/// $06FEA4: addq.b #$1, $73(a6)
pub fn i_06fea4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEA8;
    cpu.ir = 0x522E;
    let ad1 = cpu.a[6].wrapping_add(0x73);
    let v2 = bus.read8(ad1) as u32;
    let r3 = add_with_flags(cpu, v2, 1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    12
}
/// $06FEA8: move.b $73(a6), d0
pub fn i_06fea8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEAC;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x73);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FEAC: andi.w #$7, d0
pub fn i_06feac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEB0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x7) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FEB0: add.w d0, d0
pub fn i_06feb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEB2;
    cpu.ir = 0xD040;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $06FEB2: lea.l $2d552e.l, a0
pub fn i_06feb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEB8;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D552E;
    20
}
/// $06FEB8: move.w (a0, d0.w), d1
pub fn i_06feb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEBC;
    cpu.ir = 0x3230;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $06FEBC: jsr $236e.l
pub fn i_06febc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEC2;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEC2);
    cpu.pc = 0x236E;
    16
}
/// $06FEC2: jsr $28758.l
pub fn i_06fec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEC8;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEC8);
    cpu.pc = 0x28758;
    16
}
/// $06FEC8: bcc.w $6fed2
pub fn i_06fec8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FECC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FED2; return 10; }
    8
}
/// $06FECC: lea.l $70074(pc), a1
pub fn i_06fecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FED0;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FED0: move.l a1, (a6)
pub fn i_06fed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FED2;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FED2: movea.l #$ffffffff, a0
pub fn i_06fed2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FED8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $06FED8: lea.l $2d5316.l, a0
pub fn i_06fed8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEDE;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2D5316;
    20
}
/// $06FEDE: jsr $5dd56.l
pub fn i_06fede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEE4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEE4);
    cpu.pc = 0x5DD56;
    16
}
/// $06FEE4: bcc.w $6feee
pub fn i_06fee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEE8;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FEEE; return 10; }
    8
}
// ===== SetTaskHandler_06fee8  ($06FEE8, 8 B, kernel/task_handlers.c)
/// $06FEE8: lea.l $700a6(pc), a1
pub fn i_06fee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEEC;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x700A6;
    16
}
/// $06FEEC: move.l a1, (a6)
pub fn i_06feec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEEE;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FEEE: rts
pub fn i_06feee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== M5Boss_BombBounce_06fef0  ($06FEF0, 66 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FEF0: jsr $267e2.l
pub fn i_06fef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEF6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FEF6);
    cpu.pc = 0x267E2;
    16
}
/// $06FEF6: move.w #$ffa0, $2e(a6)
pub fn i_06fef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FEFC;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFA0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FEFC: lea.l $6ff02(pc), a1
pub fn i_06fefc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF00;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FF02;
    16
}
/// $06FF00: move.l a1, (a6)
pub fn i_06ff00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF02;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF02: bset.b #$6, $13(a6)
pub fn i_06ff02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF08;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FF08: jsr $27d50.l
pub fn i_06ff08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF0E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF0E);
    cpu.pc = 0x27D50;
    16
}
/// $06FF0E: bcc.w $6ff18
pub fn i_06ff0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF12;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FF18; return 10; }
    8
}
/// $06FF12: lea.l $70074(pc), a1
pub fn i_06ff12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF16;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FF16: move.l a1, (a6)
pub fn i_06ff16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF18;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF18: jsr $28d70.l
pub fn i_06ff18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF1E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF1E);
    cpu.pc = 0x28D70;
    16
}
/// $06FF1E: cmpi.w #$150, $24(a6)
pub fn i_06ff1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF24;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $06FF24: bgt.w $6ff2e
pub fn i_06ff24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF28;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FF2E; return 10; }
    8
}
/// $06FF28: lea.l $70074(pc), a1
pub fn i_06ff28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF2C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FF2C: move.l a1, (a6)
pub fn i_06ff2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF2E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF2E: bra.w $6fe64
pub fn i_06ff2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF32;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FE64;
    10
}
// ===== M5Boss_Missile_06ff32  ($06FF32, 64 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FF32: lea.l $6ff38(pc), a1
pub fn i_06ff32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF36;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FF38;
    16
}
/// $06FF36: move.l a1, (a6)
pub fn i_06ff36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF38;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF38: bset.b #$6, $13(a6)
pub fn i_06ff38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF3E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FF3E: jsr $27cee.l
pub fn i_06ff3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF44;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF44);
    cpu.pc = 0x27CEE;
    16
}
/// $06FF44: jsr $28d70.l
pub fn i_06ff44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF4A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF4A);
    cpu.pc = 0x28D70;
    16
}
/// $06FF4A: jsr $5e1ea.l
pub fn i_06ff4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF50;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF50);
    cpu.pc = 0x5E1EA;
    16
}
/// $06FF50: move.w $22(a6), d0
pub fn i_06ff50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF54;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FF54: sub.w $22(a0), d0
pub fn i_06ff54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF58;
    cpu.ir = 0x9068;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FF58: cmpi.w #$40, d0
pub fn i_06ff58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF5C;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x40, Size::Word, false);
    8
}
/// $06FF5C: bgt.w $6ff6e
pub fn i_06ff5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF60;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FF6E; return 10; }
    8
}
/// $06FF60: cmpi.w #$ffc0, d0
pub fn i_06ff60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF64;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFC0, Size::Word, false);
    8
}
/// $06FF64: blt.w $6ff6e
pub fn i_06ff64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF68;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x6FF6E; return 10; }
    8
}
/// $06FF68: lea.l $6ff72(pc), a1
pub fn i_06ff68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF6C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FF72;
    16
}
/// $06FF6C: move.l a1, (a6)
pub fn i_06ff6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF6E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF6E: bra.w $6fe64
pub fn i_06ff6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF72;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FE64;
    10
}
// ===== M5Boss_MissileHome_06ff72  ($06FF72, 88 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FF72: move.w #$ff80, $2e(a6)
pub fn i_06ff72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF78;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2E);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FF78: lea.l $6ff7e(pc), a1
pub fn i_06ff78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF7C;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x6FF7E;
    16
}
/// $06FF7C: move.l a1, (a6)
pub fn i_06ff7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF7E;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF7E: bset.b #$6, $13(a6)
pub fn i_06ff7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF84;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $06FF84: jsr $27d50.l
pub fn i_06ff84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF8A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FF8A);
    cpu.pc = 0x27D50;
    16
}
/// $06FF8A: bcc.w $6ff98
pub fn i_06ff8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF8E;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x6FF98; return 10; }
    8
}
/// $06FF8E: lea.l $70074(pc), a1
pub fn i_06ff8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF92;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FF92: move.l a1, (a6)
pub fn i_06ff92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF94;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FF94: bra.w $6ffb0
pub fn i_06ff94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF98;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FFB0;
    10
}
/// $06FF98: move.w $28(a6), d0
pub fn i_06ff98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FF9C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FF9C: move.w $2a(a6), d1
pub fn i_06ff9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFA0;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $06FFA0: asr.w #$4, d0
pub fn i_06ffa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFA2;
    cpu.ir = 0xE840;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06FFA2: asr.w #$4, d1
pub fn i_06ffa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFA4;
    cpu.ir = 0xE841;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    14
}
/// $06FFA4: jsr $5e018.l
pub fn i_06ffa4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFAA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FFAA);
    cpu.pc = 0x5E018;
    16
}
/// $06FFAA: lsr.w #$3, d0
pub fn i_06ffaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFAC;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FFAC: move.w d0, $76(a6)
pub fn i_06ffac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFB0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FFB0: jsr $28d70.l
pub fn i_06ffb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFB6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FFB6);
    cpu.pc = 0x28D70;
    16
}
/// $06FFB6: cmpi.w #$150, $24(a6)
pub fn i_06ffb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFBC;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x150, Size::Word, false);
    16
}
/// $06FFBC: bgt.w $6ffc6
pub fn i_06ffbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFC0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x6FFC6; return 10; }
    8
}
/// $06FFC0: lea.l $70074(pc), a1
pub fn i_06ffc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFC4;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x70074;
    16
}
/// $06FFC4: move.l a1, (a6)
pub fn i_06ffc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFC6;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $06FFC6: bra.w $6fe64
pub fn i_06ffc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFCA;
    cpu.ir = 0x6000;
    cpu.pc = 0x6FE64;
    10
}
// ===== M5Boss_Shell_06ffca  ($06FFCA, 124 B, enemies/bosses/gunship_m5boss_finalboss_06dfe8.s)
/// $06FFCA: jsr $5e9b6.l
pub fn i_06ffca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFD0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x6FFD0);
    cpu.pc = 0x5E9B6;
    16
}
/// $06FFD0: andi.w #$20, d0
pub fn i_06ffd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFD4;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x20) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FFD4: addi.w #$10, d0
pub fn i_06ffd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFD8;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x10, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FFD8: move.w d0, $82(a6)
pub fn i_06ffd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFDC;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x82);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FFDC: move.w #$80, d0
pub fn i_06ffdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFE0;
    cpu.ir = 0x303C;
    let v1: u32 = 0x80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FFE0: tst.b $34(a6)
pub fn i_06ffe0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFE4;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $06FFE4: beq.w $6ffec
pub fn i_06ffe4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x6FFEC; return 10; }
    8
}
/// $06FFE8: move.w #$ff80, d0
pub fn i_06ffe8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFEC;
    cpu.ir = 0x303C;
    let v1: u32 = 0xFF80;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $06FFEC: move.w d0, $28(a6)
pub fn i_06ffec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFF0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FFF0: move.b $34(a6), d0
pub fn i_06fff0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFF4;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $06FFF4: andi.w #$ff, d0
pub fn i_06fff4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFF8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $06FFF8: lsr.w #$3, d0
pub fn i_06fff8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFFA;
    cpu.ir = 0xE648;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 3, Size::Word, 1, false);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $06FFFA: move.w d0, $76(a6)
pub fn i_06fffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x6FFFE;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x76);
    bus.write16(ad2, v1 as u16);
    12
}
/// $06FFFE: move.w #$fa00, $2a(a6)
pub fn i_06fffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x70004;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFA00;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    bus.write16(ad2, v1 as u16);
    12
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x06EF86, i_06ef86),
    (0x06EF88, i_06ef88),
    (0x06EF8C, i_06ef8c),
    (0x06EF90, i_06ef90),
    (0x06EF94, i_06ef94),
    (0x06EF98, i_06ef98),
    (0x06EF9E, i_06ef9e),
    (0x06EFA2, i_06efa2),
    (0x06EFA4, i_06efa4),
    (0x06EFA6, i_06efa6),
    (0x06EFAA, i_06efaa),
    (0x06EFAC, i_06efac),
    (0x06EFB0, i_06efb0),
    (0x06EFB4, i_06efb4),
    (0x06EFB6, i_06efb6),
    (0x06EFBA, i_06efba),
    (0x06EFBC, i_06efbc),
    (0x06EFC0, i_06efc0),
    (0x06EFC4, i_06efc4),
    (0x06EFC8, i_06efc8),
    (0x06EFCA, i_06efca),
    (0x06EFCE, i_06efce),
    (0x06EFD0, i_06efd0),
    (0x06EFD6, i_06efd6),
    (0x06EFDA, i_06efda),
    (0x06EFDC, i_06efdc),
    (0x06EFE0, i_06efe0),
    (0x06EFE4, i_06efe4),
    (0x06EFE8, i_06efe8),
    (0x06EFEA, i_06efea),
    (0x06EFEE, i_06efee),
    (0x06EFF0, i_06eff0),
    (0x06EFF4, i_06eff4),
    (0x06EFF8, i_06eff8),
    (0x06EFFC, i_06effc),
    (0x06F002, i_06f002),
    (0x06F006, i_06f006),
    (0x06F00A, i_06f00a),
    (0x06F00E, i_06f00e),
    (0x06F012, i_06f012),
    (0x06F016, i_06f016),
    (0x06F018, i_06f018),
    (0x06F01C, i_06f01c),
    (0x06F020, i_06f020),
    (0x06F024, i_06f024),
    (0x06F028, i_06f028),
    (0x06F02C, i_06f02c),
    (0x06F032, i_06f032),
    (0x06F036, i_06f036),
    (0x06F03A, i_06f03a),
    (0x06F03E, i_06f03e),
    (0x06F042, i_06f042),
    (0x06F046, i_06f046),
    (0x06F048, i_06f048),
    (0x06F04C, i_06f04c),
    (0x06F050, i_06f050),
    (0x06F054, i_06f054),
    (0x06F05A, i_06f05a),
    (0x06F05E, i_06f05e),
    (0x06F060, i_06f060),
    (0x06F064, i_06f064),
    (0x06F068, i_06f068),
    (0x06F06C, i_06f06c),
    (0x06F072, i_06f072),
    (0x06F076, i_06f076),
    (0x06F078, i_06f078),
    (0x06F07C, i_06f07c),
    (0x06F080, i_06f080),
    (0x06F086, i_06f086),
    (0x06F08A, i_06f08a),
    (0x06F08E, i_06f08e),
    (0x06F092, i_06f092),
    (0x06F094, i_06f094),
    (0x06F09A, i_06f09a),
    (0x06F09E, i_06f09e),
    (0x06F0A2, i_06f0a2),
    (0x06F0A4, i_06f0a4),
    (0x06F0A6, i_06f0a6),
    (0x06F0AA, i_06f0aa),
    (0x06F0AE, i_06f0ae),
    (0x06F0B0, i_06f0b0),
    (0x06F0B2, i_06f0b2),
    (0x06F0B6, i_06f0b6),
    (0x06F0B8, i_06f0b8),
    (0x06F0BE, i_06f0be),
    (0x06F0C4, i_06f0c4),
    (0x06F0C6, i_06f0c6),
    (0x06F0CC, i_06f0cc),
    (0x06F0D2, i_06f0d2),
    (0x06F0D4, i_06f0d4),
    (0x06F0DA, i_06f0da),
    (0x06F0E0, i_06f0e0),
    (0x06F0E2, i_06f0e2),
    (0x06F0E8, i_06f0e8),
    (0x06F0EE, i_06f0ee),
    (0x06F0F0, i_06f0f0),
    (0x06F0F6, i_06f0f6),
    (0x06F0FC, i_06f0fc),
    (0x06F0FE, i_06f0fe),
    (0x06F104, i_06f104),
    (0x06F10A, i_06f10a),
    (0x06F10C, i_06f10c),
    (0x06F110, i_06f110),
    (0x06F112, i_06f112),
    (0x06F118, i_06f118),
    (0x06F11C, i_06f11c),
    (0x06F11E, i_06f11e),
    (0x06F122, i_06f122),
    (0x06F124, i_06f124),
    (0x06F128, i_06f128),
    (0x06F12A, i_06f12a),
    (0x06F130, i_06f130),
    (0x06F134, i_06f134),
    (0x06F136, i_06f136),
    (0x06F13A, i_06f13a),
    (0x06F13C, i_06f13c),
    (0x06F140, i_06f140),
    (0x06F146, i_06f146),
    (0x06F14C, i_06f14c),
    (0x06F150, i_06f150),
    (0x06F156, i_06f156),
    (0x06F15A, i_06f15a),
    (0x06F15C, i_06f15c),
    (0x06F160, i_06f160),
    (0x06F162, i_06f162),
    (0x06F166, i_06f166),
    (0x06F16A, i_06f16a),
    (0x06F16E, i_06f16e),
    (0x06F172, i_06f172),
    (0x06F176, i_06f176),
    (0x06F178, i_06f178),
    (0x06F17C, i_06f17c),
    (0x06F17E, i_06f17e),
    (0x06F182, i_06f182),
    (0x06F188, i_06f188),
    (0x06F18E, i_06f18e),
    (0x06F194, i_06f194),
    (0x06F198, i_06f198),
    (0x06F19C, i_06f19c),
    (0x06F1A2, i_06f1a2),
    (0x06F1A6, i_06f1a6),
    (0x06F1AC, i_06f1ac),
    (0x06F1B0, i_06f1b0),
    (0x06F1B4, i_06f1b4),
    (0x06F1BA, i_06f1ba),
    (0x06F1C0, i_06f1c0),
    (0x06F1C6, i_06f1c6),
    (0x06F1CA, i_06f1ca),
    (0x06F1CC, i_06f1cc),
    (0x06F1D2, i_06f1d2),
    (0x06F1D8, i_06f1d8),
    (0x06F1DE, i_06f1de),
    (0x06F1E4, i_06f1e4),
    (0x06F1EA, i_06f1ea),
    (0x06F1EE, i_06f1ee),
    (0x06F1F2, i_06f1f2),
    (0x06F1F4, i_06f1f4),
    (0x06F1F6, i_06f1f6),
    (0x06F1FA, i_06f1fa),
    (0x06F200, i_06f200),
    (0x06F206, i_06f206),
    (0x06F20C, i_06f20c),
    (0x06F214, i_06f214),
    (0x06F218, i_06f218),
    (0x06F21E, i_06f21e),
    (0x06F224, i_06f224),
    (0x06F22A, i_06f22a),
    (0x06F230, i_06f230),
    (0x06F236, i_06f236),
    (0x06F23C, i_06f23c),
    (0x06F23E, i_06f23e),
    (0x06F244, i_06f244),
    (0x06F24A, i_06f24a),
    (0x06F250, i_06f250),
    (0x06F254, i_06f254),
    (0x06F25A, i_06f25a),
    (0x06F260, i_06f260),
    (0x06F264, i_06f264),
    (0x06F26A, i_06f26a),
    (0x06F270, i_06f270),
    (0x06F274, i_06f274),
    (0x06F27A, i_06f27a),
    (0x06F280, i_06f280),
    (0x06F284, i_06f284),
    (0x06F288, i_06f288),
    (0x06F28E, i_06f28e),
    (0x06F294, i_06f294),
    (0x06F29A, i_06f29a),
    (0x06F29E, i_06f29e),
    (0x06F2A4, i_06f2a4),
    (0x06F2AA, i_06f2aa),
    (0x06F2B0, i_06f2b0),
    (0x06F2B4, i_06f2b4),
    (0x06F2BA, i_06f2ba),
    (0x06F2C0, i_06f2c0),
    (0x06F2C6, i_06f2c6),
    (0x06F2CC, i_06f2cc),
    (0x06F2D2, i_06f2d2),
    (0x06F2D8, i_06f2d8),
    (0x06F2DE, i_06f2de),
    (0x06F2E2, i_06f2e2),
    (0x06F2E6, i_06f2e6),
    (0x06F2EC, i_06f2ec),
    (0x06F2F2, i_06f2f2),
    (0x06F2F8, i_06f2f8),
    (0x06F2FE, i_06f2fe),
    (0x06F302, i_06f302),
    (0x06F308, i_06f308),
    (0x06F30C, i_06f30c),
    (0x06F312, i_06f312),
    (0x06F318, i_06f318),
    (0x06F31C, i_06f31c),
    (0x06F31E, i_06f31e),
    (0x06F324, i_06f324),
    (0x06F32A, i_06f32a),
    (0x06F32E, i_06f32e),
    (0x06F334, i_06f334),
    (0x06F338, i_06f338),
    (0x06F33C, i_06f33c),
    (0x06F33E, i_06f33e),
    (0x06F340, i_06f340),
    (0x06F344, i_06f344),
    (0x06F34A, i_06f34a),
    (0x06F350, i_06f350),
    (0x06F356, i_06f356),
    (0x06F35C, i_06f35c),
    (0x06F362, i_06f362),
    (0x06F368, i_06f368),
    (0x06F36C, i_06f36c),
    (0x06F36E, i_06f36e),
    (0x06F372, i_06f372),
    (0x06F376, i_06f376),
    (0x06F37A, i_06f37a),
    (0x06F380, i_06f380),
    (0x06F384, i_06f384),
    (0x06F388, i_06f388),
    (0x06F38A, i_06f38a),
    (0x06F38C, i_06f38c),
    (0x06F390, i_06f390),
    (0x06F396, i_06f396),
    (0x06F39C, i_06f39c),
    (0x06F3A2, i_06f3a2),
    (0x06F3A8, i_06f3a8),
    (0x06F3AC, i_06f3ac),
    (0x06F3B0, i_06f3b0),
    (0x06F3B2, i_06f3b2),
    (0x06F3B6, i_06f3b6),
    (0x06F3BC, i_06f3bc),
    (0x06F3C0, i_06f3c0),
    (0x06F3C6, i_06f3c6),
    (0x06F3CC, i_06f3cc),
    (0x06F3D2, i_06f3d2),
    (0x06F3D8, i_06f3d8),
    (0x06F3DE, i_06f3de),
    (0x06F3E2, i_06f3e2),
    (0x06F3E4, i_06f3e4),
    (0x06F3EA, i_06f3ea),
    (0x06F3EE, i_06f3ee),
    (0x06F3F2, i_06f3f2),
    (0x06F3F8, i_06f3f8),
    (0x06F3FC, i_06f3fc),
    (0x06F400, i_06f400),
    (0x06F406, i_06f406),
    (0x06F40C, i_06f40c),
    (0x06F412, i_06f412),
    (0x06F416, i_06f416),
    (0x06F41C, i_06f41c),
    (0x06F420, i_06f420),
    (0x06F424, i_06f424),
    (0x06F42A, i_06f42a),
    (0x06F42E, i_06f42e),
    (0x06F430, i_06f430),
    (0x06F436, i_06f436),
    (0x06F43A, i_06f43a),
    (0x06F43E, i_06f43e),
    (0x06F442, i_06f442),
    (0x06F446, i_06f446),
    (0x06F44C, i_06f44c),
    (0x06F450, i_06f450),
    (0x06F454, i_06f454),
    (0x06F458, i_06f458),
    (0x06F45C, i_06f45c),
    (0x06F460, i_06f460),
    (0x06F464, i_06f464),
    (0x06F46A, i_06f46a),
    (0x06F46E, i_06f46e),
    (0x06F472, i_06f472),
    (0x06F476, i_06f476),
    (0x06F478, i_06f478),
    (0x06F47E, i_06f47e),
    (0x06F484, i_06f484),
    (0x06F488, i_06f488),
    (0x06F48C, i_06f48c),
    (0x06F490, i_06f490),
    (0x06F494, i_06f494),
    (0x06F498, i_06f498),
    (0x06F49E, i_06f49e),
    (0x06F4A2, i_06f4a2),
    (0x06F4A6, i_06f4a6),
    (0x06F4AC, i_06f4ac),
    (0x06F4B0, i_06f4b0),
    (0x06F4B6, i_06f4b6),
    (0x06F4BC, i_06f4bc),
    (0x06F4C2, i_06f4c2),
    (0x06F4C8, i_06f4c8),
    (0x06F4CE, i_06f4ce),
    (0x06F4D4, i_06f4d4),
    (0x06F4D8, i_06f4d8),
    (0x06F4DC, i_06f4dc),
    (0x06F4E0, i_06f4e0),
    (0x06F4E6, i_06f4e6),
    (0x06F4EC, i_06f4ec),
    (0x06F4F2, i_06f4f2),
    (0x06F4F6, i_06f4f6),
    (0x06F4F8, i_06f4f8),
    (0x06F4FE, i_06f4fe),
    (0x06F502, i_06f502),
    (0x06F506, i_06f506),
    (0x06F50C, i_06f50c),
    (0x06F510, i_06f510),
    (0x06F514, i_06f514),
    (0x06F518, i_06f518),
    (0x06F51A, i_06f51a),
    (0x06F51E, i_06f51e),
    (0x06F524, i_06f524),
    (0x06F52A, i_06f52a),
    (0x06F530, i_06f530),
    (0x06F534, i_06f534),
    (0x06F538, i_06f538),
    (0x06F53C, i_06f53c),
    (0x06F540, i_06f540),
    (0x06F542, i_06f542),
    (0x06F548, i_06f548),
    (0x06F54A, i_06f54a),
    (0x06F54E, i_06f54e),
    (0x06F554, i_06f554),
    (0x06F558, i_06f558),
    (0x06F55E, i_06f55e),
    (0x06F562, i_06f562),
    (0x06F564, i_06f564),
    (0x06F56A, i_06f56a),
    (0x06F56E, i_06f56e),
    (0x06F572, i_06f572),
    (0x06F578, i_06f578),
    (0x06F57C, i_06f57c),
    (0x06F580, i_06f580),
    (0x06F586, i_06f586),
    (0x06F58A, i_06f58a),
    (0x06F58C, i_06f58c),
    (0x06F590, i_06f590),
    (0x06F596, i_06f596),
    (0x06F59A, i_06f59a),
    (0x06F5A0, i_06f5a0),
    (0x06F5A4, i_06f5a4),
    (0x06F5AA, i_06f5aa),
    (0x06F5B0, i_06f5b0),
    (0x06F5B4, i_06f5b4),
    (0x06F5BA, i_06f5ba),
    (0x06F5C0, i_06f5c0),
    (0x06F5C6, i_06f5c6),
    (0x06F5CA, i_06f5ca),
    (0x06F5CC, i_06f5cc),
    (0x06F5D2, i_06f5d2),
    (0x06F5D6, i_06f5d6),
    (0x06F5DA, i_06f5da),
    (0x06F5E0, i_06f5e0),
    (0x06F5E4, i_06f5e4),
    (0x06F5E8, i_06f5e8),
    (0x06F5EC, i_06f5ec),
    (0x06F5EE, i_06f5ee),
    (0x06F5F2, i_06f5f2),
    (0x06F5F8, i_06f5f8),
    (0x06F5FC, i_06f5fc),
    (0x06F5FE, i_06f5fe),
    (0x06F604, i_06f604),
    (0x06F608, i_06f608),
    (0x06F60C, i_06f60c),
    (0x06F612, i_06f612),
    (0x06F616, i_06f616),
    (0x06F61A, i_06f61a),
    (0x06F620, i_06f620),
    (0x06F624, i_06f624),
    (0x06F628, i_06f628),
    (0x06F62A, i_06f62a),
    (0x06F62E, i_06f62e),
    (0x06F632, i_06f632),
    (0x06F636, i_06f636),
    (0x06F638, i_06f638),
    (0x06F63C, i_06f63c),
    (0x06F640, i_06f640),
    (0x06F646, i_06f646),
    (0x06F64C, i_06f64c),
    (0x06F652, i_06f652),
    (0x06F656, i_06f656),
    (0x06F658, i_06f658),
    (0x06F65E, i_06f65e),
    (0x06F662, i_06f662),
    (0x06F666, i_06f666),
    (0x06F66C, i_06f66c),
    (0x06F670, i_06f670),
    (0x06F674, i_06f674),
    (0x06F678, i_06f678),
    (0x06F67A, i_06f67a),
    (0x06F67E, i_06f67e),
    (0x06F682, i_06f682),
    (0x06F688, i_06f688),
    (0x06F68E, i_06f68e),
    (0x06F694, i_06f694),
    (0x06F69A, i_06f69a),
    (0x06F6A0, i_06f6a0),
    (0x06F6A6, i_06f6a6),
    (0x06F6AA, i_06f6aa),
    (0x06F6AC, i_06f6ac),
    (0x06F6B2, i_06f6b2),
    (0x06F6B6, i_06f6b6),
    (0x06F6BA, i_06f6ba),
    (0x06F6BE, i_06f6be),
    (0x06F6C0, i_06f6c0),
    (0x06F6C4, i_06f6c4),
    (0x06F6C8, i_06f6c8),
    (0x06F6CE, i_06f6ce),
    (0x06F6D2, i_06f6d2),
    (0x06F6D6, i_06f6d6),
    (0x06F6D8, i_06f6d8),
    (0x06F6DC, i_06f6dc),
    (0x06F6E2, i_06f6e2),
    (0x06F6E6, i_06f6e6),
    (0x06F6EC, i_06f6ec),
    (0x06F6F2, i_06f6f2),
    (0x06F6F6, i_06f6f6),
    (0x06F6FC, i_06f6fc),
    (0x06F702, i_06f702),
    (0x06F706, i_06f706),
    (0x06F70A, i_06f70a),
    (0x06F710, i_06f710),
    (0x06F716, i_06f716),
    (0x06F71A, i_06f71a),
    (0x06F720, i_06f720),
    (0x06F726, i_06f726),
    (0x06F72A, i_06f72a),
    (0x06F730, i_06f730),
    (0x06F734, i_06f734),
    (0x06F73A, i_06f73a),
    (0x06F740, i_06f740),
    (0x06F744, i_06f744),
    (0x06F746, i_06f746),
    (0x06F74C, i_06f74c),
    (0x06F750, i_06f750),
    (0x06F754, i_06f754),
    (0x06F758, i_06f758),
    (0x06F75E, i_06f75e),
    (0x06F762, i_06f762),
    (0x06F768, i_06f768),
    (0x06F76E, i_06f76e),
    (0x06F772, i_06f772),
    (0x06F778, i_06f778),
    (0x06F77C, i_06f77c),
    (0x06F782, i_06f782),
    (0x06F788, i_06f788),
    (0x06F78C, i_06f78c),
    (0x06F790, i_06f790),
    (0x06F794, i_06f794),
    (0x06F79A, i_06f79a),
    (0x06F79E, i_06f79e),
    (0x06F7A2, i_06f7a2),
    (0x06F7A4, i_06f7a4),
    (0x06F7AA, i_06f7aa),
    (0x06F7AE, i_06f7ae),
    (0x06F7B2, i_06f7b2),
    (0x06F7B4, i_06f7b4),
    (0x06F7B8, i_06f7b8),
    (0x06F7BE, i_06f7be),
    (0x06F7C4, i_06f7c4),
    (0x06F7C8, i_06f7c8),
    (0x06F7CA, i_06f7ca),
    (0x06F7D0, i_06f7d0),
    (0x06F7D4, i_06f7d4),
    (0x06F7D8, i_06f7d8),
    (0x06F7DC, i_06f7dc),
    (0x06F7E2, i_06f7e2),
    (0x06F7E6, i_06f7e6),
    (0x06F7EA, i_06f7ea),
    (0x06F7EC, i_06f7ec),
    (0x06F7F0, i_06f7f0),
    (0x06F7F4, i_06f7f4),
    (0x06F7FA, i_06f7fa),
    (0x06F800, i_06f800),
    (0x06F804, i_06f804),
    (0x06F80A, i_06f80a),
    (0x06F810, i_06f810),
    (0x06F816, i_06f816),
    (0x06F81A, i_06f81a),
    (0x06F81C, i_06f81c),
    (0x06F822, i_06f822),
    (0x06F826, i_06f826),
    (0x06F82A, i_06f82a),
    (0x06F830, i_06f830),
    (0x06F834, i_06f834),
    (0x06F838, i_06f838),
    (0x06F83C, i_06f83c),
    (0x06F83E, i_06f83e),
    (0x06F842, i_06f842),
    (0x06F848, i_06f848),
    (0x06F84E, i_06f84e),
    (0x06F852, i_06f852),
    (0x06F858, i_06f858),
    (0x06F85C, i_06f85c),
    (0x06F85E, i_06f85e),
    (0x06F864, i_06f864),
    (0x06F868, i_06f868),
    (0x06F86C, i_06f86c),
    (0x06F872, i_06f872),
    (0x06F876, i_06f876),
    (0x06F87C, i_06f87c),
    (0x06F880, i_06f880),
    (0x06F886, i_06f886),
    (0x06F88A, i_06f88a),
    (0x06F88E, i_06f88e),
    (0x06F894, i_06f894),
    (0x06F898, i_06f898),
    (0x06F89C, i_06f89c),
    (0x06F8A2, i_06f8a2),
    (0x06F8A6, i_06f8a6),
    (0x06F8AA, i_06f8aa),
    (0x06F8AC, i_06f8ac),
    (0x06F8B2, i_06f8b2),
    (0x06F8B6, i_06f8b6),
    (0x06F8BA, i_06f8ba),
    (0x06F8BC, i_06f8bc),
    (0x06F8C0, i_06f8c0),
    (0x06F8C4, i_06f8c4),
    (0x06F8CA, i_06f8ca),
    (0x06F8D0, i_06f8d0),
    (0x06F8D6, i_06f8d6),
    (0x06F8DA, i_06f8da),
    (0x06F8DC, i_06f8dc),
    (0x06F8E2, i_06f8e2),
    (0x06F8E6, i_06f8e6),
    (0x06F8EA, i_06f8ea),
    (0x06F8F0, i_06f8f0),
    (0x06F8F4, i_06f8f4),
    (0x06F8F8, i_06f8f8),
    (0x06F8FC, i_06f8fc),
    (0x06F8FE, i_06f8fe),
    (0x06F902, i_06f902),
    (0x06F906, i_06f906),
    (0x06F90C, i_06f90c),
    (0x06F912, i_06f912),
    (0x06F916, i_06f916),
    (0x06F91C, i_06f91c),
    (0x06F922, i_06f922),
    (0x06F928, i_06f928),
    (0x06F92C, i_06f92c),
    (0x06F92E, i_06f92e),
    (0x06F934, i_06f934),
    (0x06F938, i_06f938),
    (0x06F93C, i_06f93c),
    (0x06F942, i_06f942),
    (0x06F946, i_06f946),
    (0x06F94A, i_06f94a),
    (0x06F94E, i_06f94e),
    (0x06F950, i_06f950),
    (0x06F954, i_06f954),
    (0x06F95A, i_06f95a),
    (0x06F960, i_06f960),
    (0x06F966, i_06f966),
    (0x06F96C, i_06f96c),
    (0x06F970, i_06f970),
    (0x06F972, i_06f972),
    (0x06F978, i_06f978),
    (0x06F97C, i_06f97c),
    (0x06F980, i_06f980),
    (0x06F986, i_06f986),
    (0x06F98C, i_06f98c),
    (0x06F990, i_06f990),
    (0x06F994, i_06f994),
    (0x06F99A, i_06f99a),
    (0x06F99E, i_06f99e),
    (0x06F9A6, i_06f9a6),
    (0x06F9AA, i_06f9aa),
    (0x06F9AE, i_06f9ae),
    (0x06F9B2, i_06f9b2),
    (0x06F9B6, i_06f9b6),
    (0x06F9BC, i_06f9bc),
    (0x06F9C0, i_06f9c0),
    (0x06F9C4, i_06f9c4),
    (0x06F9C6, i_06f9c6),
    (0x06F9CA, i_06f9ca),
    (0x06F9D0, i_06f9d0),
    (0x06F9D6, i_06f9d6),
    (0x06F9DC, i_06f9dc),
    (0x06F9E2, i_06f9e2),
    (0x06F9E8, i_06f9e8),
    (0x06F9EE, i_06f9ee),
    (0x06F9F4, i_06f9f4),
    (0x06F9FA, i_06f9fa),
    (0x06FA00, i_06fa00),
    (0x06FA04, i_06fa04),
    (0x06FA06, i_06fa06),
    (0x06FA0C, i_06fa0c),
    (0x06FA12, i_06fa12),
    (0x06FA16, i_06fa16),
    (0x06FA1A, i_06fa1a),
    (0x06FA20, i_06fa20),
    (0x06FA24, i_06fa24),
    (0x06FA2A, i_06fa2a),
    (0x06FA2E, i_06fa2e),
    (0x06FA30, i_06fa30),
    (0x06FA36, i_06fa36),
    (0x06FA3A, i_06fa3a),
    (0x06FA40, i_06fa40),
    (0x06FA46, i_06fa46),
    (0x06FA4A, i_06fa4a),
    (0x06FA50, i_06fa50),
    (0x06FA54, i_06fa54),
    (0x06FA58, i_06fa58),
    (0x06FA5C, i_06fa5c),
    (0x06FA5E, i_06fa5e),
    (0x06FA60, i_06fa60),
    (0x06FA66, i_06fa66),
    (0x06FA6C, i_06fa6c),
    (0x06FA70, i_06fa70),
    (0x06FA76, i_06fa76),
    (0x06FA7E, i_06fa7e),
    (0x06FA86, i_06fa86),
    (0x06FA8C, i_06fa8c),
    (0x06FA92, i_06fa92),
    (0x06FA98, i_06fa98),
    (0x06FA9C, i_06fa9c),
    (0x06FAA2, i_06faa2),
    (0x06FAA8, i_06faa8),
    (0x06FAAE, i_06faae),
    (0x06FAB2, i_06fab2),
    (0x06FAB4, i_06fab4),
    (0x06FAB8, i_06fab8),
    (0x06FABA, i_06faba),
    (0x06FAC0, i_06fac0),
    (0x06FAC2, i_06fac2),
    (0x06FAC6, i_06fac6),
    (0x06FACC, i_06facc),
    (0x06FAD0, i_06fad0),
    (0x06FAD6, i_06fad6),
    (0x06FADC, i_06fadc),
    (0x06FAE2, i_06fae2),
    (0x06FAE4, i_06fae4),
    (0x06FAE8, i_06fae8),
    (0x06FAEE, i_06faee),
    (0x06FAF2, i_06faf2),
    (0x06FAF6, i_06faf6),
    (0x06FAF8, i_06faf8),
    (0x06FAFA, i_06fafa),
    (0x06FB00, i_06fb00),
    (0x06FB02, i_06fb02),
    (0x06FB08, i_06fb08),
    (0x06FB0E, i_06fb0e),
    (0x06FB10, i_06fb10),
    (0x06FB14, i_06fb14),
    (0x06FB1A, i_06fb1a),
    (0x06FB20, i_06fb20),
    (0x06FB26, i_06fb26),
    (0x06FB2A, i_06fb2a),
    (0x06FB2E, i_06fb2e),
    (0x06FB32, i_06fb32),
    (0x06FB36, i_06fb36),
    (0x06FB38, i_06fb38),
    (0x06FB3C, i_06fb3c),
    (0x06FB40, i_06fb40),
    (0x06FB42, i_06fb42),
    (0x06FB48, i_06fb48),
    (0x06FB4A, i_06fb4a),
    (0x06FB4E, i_06fb4e),
    (0x06FB54, i_06fb54),
    (0x06FB58, i_06fb58),
    (0x06FB5E, i_06fb5e),
    (0x06FB62, i_06fb62),
    (0x06FB64, i_06fb64),
    (0x06FB68, i_06fb68),
    (0x06FB6E, i_06fb6e),
    (0x06FB72, i_06fb72),
    (0x06FB78, i_06fb78),
    (0x06FB7C, i_06fb7c),
    (0x06FB80, i_06fb80),
    (0x06FB84, i_06fb84),
    (0x06FB88, i_06fb88),
    (0x06FB8C, i_06fb8c),
    (0x06FB8E, i_06fb8e),
    (0x06FB94, i_06fb94),
    (0x06FB98, i_06fb98),
    (0x06FB9C, i_06fb9c),
    (0x06FB9E, i_06fb9e),
    (0x06FBA0, i_06fba0),
    (0x06FBA4, i_06fba4),
    (0x06FBAA, i_06fbaa),
    (0x06FBB0, i_06fbb0),
    (0x06FBB6, i_06fbb6),
    (0x06FBBC, i_06fbbc),
    (0x06FBC2, i_06fbc2),
    (0x06FBC8, i_06fbc8),
    (0x06FBCC, i_06fbcc),
    (0x06FBCE, i_06fbce),
    (0x06FBD2, i_06fbd2),
    (0x06FBD8, i_06fbd8),
    (0x06FBDC, i_06fbdc),
    (0x06FBE2, i_06fbe2),
    (0x06FBE8, i_06fbe8),
    (0x06FBEC, i_06fbec),
    (0x06FBF2, i_06fbf2),
    (0x06FBF6, i_06fbf6),
    (0x06FBFA, i_06fbfa),
    (0x06FBFC, i_06fbfc),
    (0x06FC02, i_06fc02),
    (0x06FC06, i_06fc06),
    (0x06FC0A, i_06fc0a),
    (0x06FC0C, i_06fc0c),
    (0x06FC10, i_06fc10),
    (0x06FC16, i_06fc16),
    (0x06FC1A, i_06fc1a),
    (0x06FC1E, i_06fc1e),
    (0x06FC24, i_06fc24),
    (0x06FC28, i_06fc28),
    (0x06FC2C, i_06fc2c),
    (0x06FC32, i_06fc32),
    (0x06FC38, i_06fc38),
    (0x06FC3E, i_06fc3e),
    (0x06FC44, i_06fc44),
    (0x06FC48, i_06fc48),
    (0x06FC4C, i_06fc4c),
    (0x06FC4E, i_06fc4e),
    (0x06FC50, i_06fc50),
    (0x06FC56, i_06fc56),
    (0x06FC5C, i_06fc5c),
    (0x06FC60, i_06fc60),
    (0x06FC62, i_06fc62),
    (0x06FC66, i_06fc66),
    (0x06FC6C, i_06fc6c),
    (0x06FC70, i_06fc70),
    (0x06FC74, i_06fc74),
    (0x06FC76, i_06fc76),
    (0x06FC7A, i_06fc7a),
    (0x06FC80, i_06fc80),
    (0x06FC84, i_06fc84),
    (0x06FC88, i_06fc88),
    (0x06FC8A, i_06fc8a),
    (0x06FC90, i_06fc90),
    (0x06FC94, i_06fc94),
    (0x06FC98, i_06fc98),
    (0x06FC9A, i_06fc9a),
    (0x06FC9E, i_06fc9e),
    (0x06FCA2, i_06fca2),
    (0x06FCA8, i_06fca8),
    (0x06FCAE, i_06fcae),
    (0x06FCB2, i_06fcb2),
    (0x06FCB4, i_06fcb4),
    (0x06FCB8, i_06fcb8),
    (0x06FCBE, i_06fcbe),
    (0x06FCC2, i_06fcc2),
    (0x06FCC8, i_06fcc8),
    (0x06FCCE, i_06fcce),
    (0x06FCD2, i_06fcd2),
    (0x06FCD8, i_06fcd8),
    (0x06FCDC, i_06fcdc),
    (0x06FCE0, i_06fce0),
    (0x06FCE2, i_06fce2),
    (0x06FCE8, i_06fce8),
    (0x06FCEC, i_06fcec),
    (0x06FCF0, i_06fcf0),
    (0x06FCF2, i_06fcf2),
    (0x06FCF6, i_06fcf6),
    (0x06FCFA, i_06fcfa),
    (0x06FCFE, i_06fcfe),
    (0x06FD04, i_06fd04),
    (0x06FD0A, i_06fd0a),
    (0x06FD10, i_06fd10),
    (0x06FD16, i_06fd16),
    (0x06FD1A, i_06fd1a),
    (0x06FD1C, i_06fd1c),
    (0x06FD1E, i_06fd1e),
    (0x06FD22, i_06fd22),
    (0x06FD24, i_06fd24),
    (0x06FD28, i_06fd28),
    (0x06FD2C, i_06fd2c),
    (0x06FD32, i_06fd32),
    (0x06FD38, i_06fd38),
    (0x06FD3E, i_06fd3e),
    (0x06FD42, i_06fd42),
    (0x06FD46, i_06fd46),
    (0x06FD4A, i_06fd4a),
    (0x06FD4E, i_06fd4e),
    (0x06FD52, i_06fd52),
    (0x06FD56, i_06fd56),
    (0x06FD5A, i_06fd5a),
    (0x06FD5C, i_06fd5c),
    (0x06FD62, i_06fd62),
    (0x06FD64, i_06fd64),
    (0x06FD68, i_06fd68),
    (0x06FD6E, i_06fd6e),
    (0x06FD72, i_06fd72),
    (0x06FD78, i_06fd78),
    (0x06FD7C, i_06fd7c),
    (0x06FD7E, i_06fd7e),
    (0x06FD82, i_06fd82),
    (0x06FD86, i_06fd86),
    (0x06FD8A, i_06fd8a),
    (0x06FD8E, i_06fd8e),
    (0x06FD92, i_06fd92),
    (0x06FD94, i_06fd94),
    (0x06FD96, i_06fd96),
    (0x06FD98, i_06fd98),
    (0x06FD9A, i_06fd9a),
    (0x06FD9E, i_06fd9e),
    (0x06FDA2, i_06fda2),
    (0x06FDA6, i_06fda6),
    (0x06FDAA, i_06fdaa),
    (0x06FDAC, i_06fdac),
    (0x06FDB0, i_06fdb0),
    (0x06FDB6, i_06fdb6),
    (0x06FDBA, i_06fdba),
    (0x06FDBE, i_06fdbe),
    (0x06FDC0, i_06fdc0),
    (0x06FDC6, i_06fdc6),
    (0x06FDCA, i_06fdca),
    (0x06FDCE, i_06fdce),
    (0x06FDD0, i_06fdd0),
    (0x06FDD4, i_06fdd4),
    (0x06FDDA, i_06fdda),
    (0x06FDE0, i_06fde0),
    (0x06FDE6, i_06fde6),
    (0x06FDE8, i_06fde8),
    (0x06FDEE, i_06fdee),
    (0x06FDF4, i_06fdf4),
    (0x06FDFA, i_06fdfa),
    (0x06FDFE, i_06fdfe),
    (0x06FE02, i_06fe02),
    (0x06FE06, i_06fe06),
    (0x06FE08, i_06fe08),
    (0x06FE0C, i_06fe0c),
    (0x06FE10, i_06fe10),
    (0x06FE14, i_06fe14),
    (0x06FE1A, i_06fe1a),
    (0x06FE1E, i_06fe1e),
    (0x06FE22, i_06fe22),
    (0x06FE28, i_06fe28),
    (0x06FE2A, i_06fe2a),
    (0x06FE30, i_06fe30),
    (0x06FE34, i_06fe34),
    (0x06FE38, i_06fe38),
    (0x06FE3E, i_06fe3e),
    (0x06FE44, i_06fe44),
    (0x06FE48, i_06fe48),
    (0x06FE4A, i_06fe4a),
    (0x06FE50, i_06fe50),
    (0x06FE54, i_06fe54),
    (0x06FE58, i_06fe58),
    (0x06FE5A, i_06fe5a),
    (0x06FE5E, i_06fe5e),
    (0x06FE64, i_06fe64),
    (0x06FE6A, i_06fe6a),
    (0x06FE70, i_06fe70),
    (0x06FE74, i_06fe74),
    (0x06FE78, i_06fe78),
    (0x06FE7A, i_06fe7a),
    (0x06FE80, i_06fe80),
    (0x06FE84, i_06fe84),
    (0x06FE8A, i_06fe8a),
    (0x06FE8E, i_06fe8e),
    (0x06FE92, i_06fe92),
    (0x06FE96, i_06fe96),
    (0x06FE9A, i_06fe9a),
    (0x06FE9E, i_06fe9e),
    (0x06FEA4, i_06fea4),
    (0x06FEA8, i_06fea8),
    (0x06FEAC, i_06feac),
    (0x06FEB0, i_06feb0),
    (0x06FEB2, i_06feb2),
    (0x06FEB8, i_06feb8),
    (0x06FEBC, i_06febc),
    (0x06FEC2, i_06fec2),
    (0x06FEC8, i_06fec8),
    (0x06FECC, i_06fecc),
    (0x06FED0, i_06fed0),
    (0x06FED2, i_06fed2),
    (0x06FED8, i_06fed8),
    (0x06FEDE, i_06fede),
    (0x06FEE4, i_06fee4),
    (0x06FEE8, i_06fee8),
    (0x06FEEC, i_06feec),
    (0x06FEEE, i_06feee),
    (0x06FEF0, i_06fef0),
    (0x06FEF6, i_06fef6),
    (0x06FEFC, i_06fefc),
    (0x06FF00, i_06ff00),
    (0x06FF02, i_06ff02),
    (0x06FF08, i_06ff08),
    (0x06FF0E, i_06ff0e),
    (0x06FF12, i_06ff12),
    (0x06FF16, i_06ff16),
    (0x06FF18, i_06ff18),
    (0x06FF1E, i_06ff1e),
    (0x06FF24, i_06ff24),
    (0x06FF28, i_06ff28),
    (0x06FF2C, i_06ff2c),
    (0x06FF2E, i_06ff2e),
    (0x06FF32, i_06ff32),
    (0x06FF36, i_06ff36),
    (0x06FF38, i_06ff38),
    (0x06FF3E, i_06ff3e),
    (0x06FF44, i_06ff44),
    (0x06FF4A, i_06ff4a),
    (0x06FF50, i_06ff50),
    (0x06FF54, i_06ff54),
    (0x06FF58, i_06ff58),
    (0x06FF5C, i_06ff5c),
    (0x06FF60, i_06ff60),
    (0x06FF64, i_06ff64),
    (0x06FF68, i_06ff68),
    (0x06FF6C, i_06ff6c),
    (0x06FF6E, i_06ff6e),
    (0x06FF72, i_06ff72),
    (0x06FF78, i_06ff78),
    (0x06FF7C, i_06ff7c),
    (0x06FF7E, i_06ff7e),
    (0x06FF84, i_06ff84),
    (0x06FF8A, i_06ff8a),
    (0x06FF8E, i_06ff8e),
    (0x06FF92, i_06ff92),
    (0x06FF94, i_06ff94),
    (0x06FF98, i_06ff98),
    (0x06FF9C, i_06ff9c),
    (0x06FFA0, i_06ffa0),
    (0x06FFA2, i_06ffa2),
    (0x06FFA4, i_06ffa4),
    (0x06FFAA, i_06ffaa),
    (0x06FFAC, i_06ffac),
    (0x06FFB0, i_06ffb0),
    (0x06FFB6, i_06ffb6),
    (0x06FFBC, i_06ffbc),
    (0x06FFC0, i_06ffc0),
    (0x06FFC4, i_06ffc4),
    (0x06FFC6, i_06ffc6),
    (0x06FFCA, i_06ffca),
    (0x06FFD0, i_06ffd0),
    (0x06FFD4, i_06ffd4),
    (0x06FFD8, i_06ffd8),
    (0x06FFDC, i_06ffdc),
    (0x06FFE0, i_06ffe0),
    (0x06FFE4, i_06ffe4),
    (0x06FFE8, i_06ffe8),
    (0x06FFEC, i_06ffec),
    (0x06FFF0, i_06fff0),
    (0x06FFF4, i_06fff4),
    (0x06FFF8, i_06fff8),
    (0x06FFFA, i_06fffa),
    (0x06FFFE, i_06fffe),
];
