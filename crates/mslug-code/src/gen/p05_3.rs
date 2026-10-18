// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p05_3) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


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
/// $05E0CE: movea.l (a7)+, a1
pub fn i_05e0ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0D0;
    cpu.ir = 0x225F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    12
}
/// $05E0D0: bra.w $5e0d8
pub fn i_05e0d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0D4;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E0D8;
    10
}
/// $05E0D4: jsr $5e1aa(pc)
pub fn i_05e0d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0D8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E0D8);
    cpu.pc = 0x5E1AA;
    16
}
/// $05E0D8: tst.b d0
pub fn i_05e0d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0DA;
    cpu.ir = 0x4A00;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $05E0DA: beq.w $5e12a
pub fn i_05e0da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E12A; return 10; }
    8
}
/// $05E0DE: lea.l $100440.l, a0
pub fn i_05e0de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0E4;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100440;
    20
}
/// $05E0E4: lea.l $1004e0.l, a1
pub fn i_05e0e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0EA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $05E0EA: cmpi.b #$1, d0
pub fn i_05e0ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0EE;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $05E0EE: beq.w $5e11c
pub fn i_05e0ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0F2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E11C; return 10; }
    8
}
/// $05E0F2: cmpi.b #$2, d0
pub fn i_05e0f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0F6;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x2, Size::Byte, false);
    8
}
/// $05E0F6: beq.w $5e122
pub fn i_05e0f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E122; return 10; }
    8
}
/// $05E0FA: move.w $22(a6), d0
pub fn i_05e0fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E0FE;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E0FE: move.w $22(a0), d1
pub fn i_05e0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E102;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E102: move.w $22(a1), d2
pub fn i_05e102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E106;
    cpu.ir = 0x3429;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E106: sub.w d0, d1
pub fn i_05e106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E108;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E108: bge.w $5e10e
pub fn i_05e108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E10C;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E10E; return 10; }
    8
}
/// $05E10C: neg.w d1
pub fn i_05e10c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E10E;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E10E: sub.w d0, d2
pub fn i_05e10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E110;
    cpu.ir = 0x9440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E110: bge.w $5e116
pub fn i_05e110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E114;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E116; return 10; }
    8
}
/// $05E114: neg.w d2
pub fn i_05e114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E116;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E116: cmp.w d1, d2
pub fn i_05e116(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E118;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05E118: blt.w $5e122
pub fn i_05e118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E11C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E122; return 10; }
    8
}
// ===== ClearXN_05e11c  ($05E11C, 6 B, thunks/ccr_helpers.c)
/// $05E11C: andi.b #$ee, ccr
pub fn i_05e11c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E120;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E120: rts
pub fn i_05e120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E122;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_UseP2_05e122  ($05E122, 2 B, props/late_props_turrets_05e000.s)
/// $05E122: movea.l a1, a0
pub fn i_05e122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E124;
    cpu.ir = 0x2049;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
// ===== ClearXN_05e124  ($05E124, 6 B, thunks/ccr_helpers.c)
/// $05E124: andi.b #$ee, ccr
pub fn i_05e124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E128;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E128: rts
pub fn i_05e128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E12A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_DefaultP2_05e12a  ($05E12A, 6 B, props/late_props_turrets_05e000.s)
/// $05E12A: lea.l $1004e0.l, a0
pub fn i_05e12a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E130;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x1004E0;
    20
}
// ===== SetXN_05e130  ($05E130, 6 B, thunks/ccr_helpers.c)
/// $05E130: ori.b #$11, ccr
pub fn i_05e130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E134;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E134: rts
pub fn i_05e134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E136;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Players_AliveMask_05e1aa  ($05E1AA, 52 B, props/late_props_turrets_05e000.s)
/// $05E1AA: clr.l d0
pub fn i_05e1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1AC;
    cpu.ir = 0x4280;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = r2;
    4
}
/// $05E1AC: jsr $5e3a2(pc)
pub fn i_05e1ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1B0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E1B0);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E1B0: bcc.w $5e1bc
pub fn i_05e1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1B4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E1BC; return 10; }
    8
}
/// $05E1B4: move.w #$1, d0
pub fn i_05e1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1B8;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E1B8: bra.w $5e1be
pub fn i_05e1b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1BC;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E1BE;
    10
}
/// $05E1BC: clr.w d0
pub fn i_05e1bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1BE;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E1BE: move.l d0, -(a7)
pub fn i_05e1be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1C0;
    cpu.ir = 0x2F00;
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05E1C0: moveq #$1, d0
pub fn i_05e1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1C2;
    cpu.ir = 0x7001;
    cpu.d[0] = 0x1;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E1C2: jsr $5e3a2(pc)
pub fn i_05e1c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E1C6);
    cpu.pc = 0x5E3A2;
    16
}
/// $05E1C6: bcc.w $5e1d2
pub fn i_05e1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1CA;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x5E1D2; return 10; }
    8
}
/// $05E1CA: move.w #$2, d1
pub fn i_05e1ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1CE;
    cpu.ir = 0x323C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E1CE: bra.w $5e1d4
pub fn i_05e1ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1D2;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E1D4;
    10
}
/// $05E1D2: clr.w d1
pub fn i_05e1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1D4;
    cpu.ir = 0x4241;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E1D4: move.l (a7)+, d0
pub fn i_05e1d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1D6;
    cpu.ir = 0x201F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.d[0] = v3;
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05E1D6: or.w d1, d0
pub fn i_05e1d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1D8;
    cpu.ir = 0x8041;
    let v1 = cpu.d[1] & 0xFFFF;
    let r = (v1 | cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E1D8: tst.w d0
pub fn i_05e1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1DA;
    cpu.ir = 0x4A40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E1DA: beq.w $5e1e4
pub fn i_05e1da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E1E4; return 10; }
    8
}
// ===== ClearXN_05e1de  ($05E1DE, 6 B, thunks/ccr_helpers.c)
/// $05E1DE: andi.b #$ee, ccr
pub fn i_05e1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1E2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E1E2: rts
pub fn i_05e1e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e1e4  ($05E1E4, 6 B, thunks/ccr_helpers.c)
/// $05E1E4: ori.b #$11, ccr
pub fn i_05e1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1E8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E1E8: rts
pub fn i_05e1e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E1EA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $05E23A: sub.w d2, d0
pub fn i_05e23a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E23C;
    cpu.ir = 0x9042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E23C: sub.w d3, d1
pub fn i_05e23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E23E;
    cpu.ir = 0x9243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E23E: cmpi.w #$0, d0
pub fn i_05e23e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E242;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05E242: bge.w $5e248
pub fn i_05e242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E246;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E248; return 10; }
    8
}
/// $05E246: neg.w d0
pub fn i_05e246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E248;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E248: cmpi.w #$0, d1
pub fn i_05e248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E24C;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $05E24C: bge.w $5e252
pub fn i_05e24c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E250;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x5E252; return 10; }
    8
}
/// $05E250: neg.w d1
pub fn i_05e250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E252;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E252: cmp.w d1, d0
pub fn i_05e252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E254;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $05E254: bgt.w $5e25a
pub fn i_05e254(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E258;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E25A; return 10; }
    8
}
/// $05E258: exg.l d0, d1
pub fn i_05e258(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E25A;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $05E25A: lsr.w #$1, d1
pub fn i_05e25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E25C;
    cpu.ir = 0xE249;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 1, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05E25C: add.w d1, d0
pub fn i_05e25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E25E;
    cpu.ir = 0xD041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $05E25E: rts
pub fn i_05e25e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E260;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Target_InRangeBox_05e260  ($05E260, 108 B, props/late_props_turrets_05e000.s)
/// $05E260: move.w $22(a0), d1
pub fn i_05e260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E264;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E264: sub.w $22(a6), d1
pub fn i_05e264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E268;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E268: btst.b #$0, $3a(a6)
pub fn i_05e268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E26E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E26E: bne.w $5e284
pub fn i_05e26e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E272;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E284; return 10; }
    8
}
/// $05E272: cmp.w (a1), d1
pub fn i_05e272(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E274;
    cpu.ir = 0xB251;
    let v2 = bus.read16(cpu.a[1]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    8
}
/// $05E274: blt.w $5e2d2
pub fn i_05e274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E278;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E278: cmp.w $2(a1), d1
pub fn i_05e278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E27C;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E27C: bgt.w $5e2d2
pub fn i_05e27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E280;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E280: bra.w $5e294
pub fn i_05e280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E284;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E294;
    10
}
/// $05E284: neg.w d1
pub fn i_05e284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E286;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E286: cmp.w (a1), d1
pub fn i_05e286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E288;
    cpu.ir = 0xB251;
    let v2 = bus.read16(cpu.a[1]) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v2, Size::Word, false);
    8
}
/// $05E288: blt.w $5e2d2
pub fn i_05e288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E28C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E28C: cmp.w $2(a1), d1
pub fn i_05e28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E290;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E290: bgt.w $5e2d2
pub fn i_05e290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E294;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E294: move.w $24(a0), d1
pub fn i_05e294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E298;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E298: sub.w $24(a6), d1
pub fn i_05e298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E29C;
    cpu.ir = 0x926E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $05E29C: btst.b #$1, $3a(a6)
pub fn i_05e29c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2A2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E2A2: bne.w $5e2ba
pub fn i_05e2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2A6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x5E2BA; return 10; }
    8
}
/// $05E2A6: cmp.w $4(a1), d1
pub fn i_05e2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2AA;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E2AA: blt.w $5e2d2
pub fn i_05e2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2AE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E2AE: cmp.w $6(a1), d1
pub fn i_05e2ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2B2;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E2B2: bgt.w $5e2d2
pub fn i_05e2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2B6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E2B6: bra.w $5e2cc
pub fn i_05e2b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2BA;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E2CC;
    10
}
/// $05E2BA: neg.w d1
pub fn i_05e2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2BC;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $05E2BC: cmp.w $4(a1), d1
pub fn i_05e2bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2C0;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E2C0: blt.w $5e2d2
pub fn i_05e2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2C4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
/// $05E2C4: cmp.w $6(a1), d1
pub fn i_05e2c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2C8;
    cpu.ir = 0xB269;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $05E2C8: bgt.w $5e2d2
pub fn i_05e2c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2CC;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x5E2D2; return 10; }
    8
}
// ===== ClearXN_05e2cc  ($05E2CC, 6 B, thunks/ccr_helpers.c)
/// $05E2CC: andi.b #$ee, ccr
pub fn i_05e2cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2D0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E2D0: rts
pub fn i_05e2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2D2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e2d2  ($05E2D2, 6 B, thunks/ccr_helpers.c)
/// $05E2D2: ori.b #$11, ccr
pub fn i_05e2d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2D6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E2D6: rts
pub fn i_05e2d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SlotExtractCoords_05E2D8  ($05E2D8, 96 B, video/fix_layer_backends_05db1a.s)
/// $05E2D8: moveq #$ff, d0
pub fn i_05e2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2DA;
    cpu.ir = 0x70FF;
    cpu.d[0] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E2DA: moveq #$ff, d1
pub fn i_05e2da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2DC;
    cpu.ir = 0x72FF;
    cpu.d[1] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E2DC: cmpi.l #$ffffffff, (a0)
pub fn i_05e2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2E2;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $05E2E2: beq.w $5e336
pub fn i_05e2e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2E6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E2E6: cmpi.l #$52a, (a0)
pub fn i_05e2e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2EC;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $05E2EC: beq.w $5e336
pub fn i_05e2ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2F0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E2F0: cmpi.l #$400, (a0)
pub fn i_05e2f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2F6;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $05E2F6: beq.w $5e336
pub fn i_05e2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E2FA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E2FA: cmpi.b #$3, $106ece.l
pub fn i_05e2fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E302;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0x3, Size::Byte, false);
    20
}
/// $05E302: beq.w $5e320
pub fn i_05e302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E306;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E320; return 10; }
    8
}
/// $05E306: moveq #$0, d0
pub fn i_05e306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E308;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E308: move.w $22(a0), d0
pub fn i_05e308(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E30C;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E30C: btst.b #$7, $5b(a0)
pub fn i_05e30c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E312;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E312: beq.w $5e336
pub fn i_05e312(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E316;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E316: moveq #$0, d1
pub fn i_05e316(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E318;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E318: move.w $24(a0), d1
pub fn i_05e318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E31C;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E31C: bra.w $5e336
pub fn i_05e31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E320;
    cpu.ir = 0x6000;
    cpu.pc = 0x5E336;
    10
}
/// $05E320: moveq #$0, d0
pub fn i_05e320(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E322;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E322: move.w $22(a0), d0
pub fn i_05e322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E326;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E326: btst.b #$7, $5b(a0)
pub fn i_05e326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E32C;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E32C: beq.w $5e336
pub fn i_05e32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E330;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E336; return 10; }
    8
}
/// $05E330: moveq #$0, d1
pub fn i_05e330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E332;
    cpu.ir = 0x7200;
    cpu.d[1] = 0x0;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E332: move.w $24(a0), d1
pub fn i_05e332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E336;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E336: rts
pub fn i_05e336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E338;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_GetEntity_05e3a2  ($05E3A2, 64 B, props/late_props_turrets_05e000.s)
/// $05E3A2: andi.w #$1, d0
pub fn i_05e3a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3A6;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0x1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E3A6: lea.l $5e3f4(pc), a0
pub fn i_05e3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3AA;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x5E3F4;
    16
}
/// $05E3AA: asl.l #$2, d0
pub fn i_05e3aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3AC;
    cpu.ir = 0xE580;
    let h1 = cpu.d[0] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 0, true);
    cpu.d[0] = r;
    10
}
/// $05E3AC: movea.l (a0, d0.w), a0
pub fn i_05e3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3B0;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $05E3B0: btst.b #$7, $5b(a0)
pub fn i_05e3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3B6;
    cpu.ir = 0x828;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[0].wrapping_add(0x5B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $05E3B6: beq.w $5e3e8
pub fn i_05e3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3BA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
/// $05E3BA: cmpi.l #$ffffffff, (a0)
pub fn i_05e3ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3C0;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $05E3C0: beq.w $5e3e8
pub fn i_05e3c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3C4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
/// $05E3C4: cmpi.l #$52a, (a0)
pub fn i_05e3c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3CA;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $05E3CA: beq.w $5e3e8
pub fn i_05e3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3CE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
/// $05E3CE: cmpi.l #$400, (a0)
pub fn i_05e3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3D4;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $05E3D4: beq.w $5e3e8
pub fn i_05e3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3D8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
/// $05E3D8: cmpi.l #$2ae3e, (a0)
pub fn i_05e3d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3DE;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x2AE3E, Size::Long, false);
    16
}
/// $05E3DE: beq.w $5e3e8
pub fn i_05e3de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E3E8; return 10; }
    8
}
// ===== SetXN_05e3e2  ($05E3E2, 6 B, thunks/ccr_helpers.c)
/// $05E3E2: ori.b #$11, ccr
pub fn i_05e3e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3E6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E3E6: rts
pub fn i_05e3e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3E8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_GetEntityFail_05e3e8  ($05E3E8, 6 B, props/late_props_turrets_05e000.s)
/// $05E3E8: movea.l #$ffffffff, a0
pub fn i_05e3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3EE;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
// ===== ClearXN_05e3ee  ($05E3EE, 6 B, thunks/ccr_helpers.c)
/// $05E3EE: andi.b #$ee, ccr
pub fn i_05e3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3F2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E3F2: rts
pub fn i_05e3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E3F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parent_IsLiveHandler_05e452  ($05E452, 8 B, props/late_props_turrets_05e000.s)
/// $05E452: movea.l $c(a6), a0
pub fn i_05e452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E456;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E456: jmp $5e3ba(pc)
pub fn i_05e456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E45A;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x5E3BA;
    8
}
// ===== Parent_IsFreed_05e45a  ($05E45A, 34 B, props/late_props_turrets_05e000.s)
/// $05E45A: movea.l $c(a6), a0
pub fn i_05e45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E45E;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E45E: cmpi.l #$ffffffff, (a0)
pub fn i_05e45e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E464;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $05E464: beq.w $5e482
pub fn i_05e464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E468;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E482; return 10; }
    8
}
/// $05E468: cmpi.l #$52a, (a0)
pub fn i_05e468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E46E;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x52A, Size::Long, false);
    16
}
/// $05E46E: beq.w $5e482
pub fn i_05e46e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E472;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E482; return 10; }
    8
}
/// $05E472: cmpi.l #$400, (a0)
pub fn i_05e472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E478;
    cpu.ir = 0xC90;
    let v2 = bus.read32(cpu.a[0]);
    sub_with_flags(cpu, v2, 0x400, Size::Long, false);
    16
}
/// $05E478: beq.w $5e482
pub fn i_05e478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E47C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x5E482; return 10; }
    8
}
// ===== ClearXN_05e47c  ($05E47C, 6 B, thunks/ccr_helpers.c)
/// $05E47C: andi.b #$ee, ccr
pub fn i_05e47c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E480;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $05E480: rts
pub fn i_05e480(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E482;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_05e482  ($05E482, 6 B, thunks/ccr_helpers.c)
/// $05E482: ori.b #$11, ccr
pub fn i_05e482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E486;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $05E486: rts
pub fn i_05e486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E488;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ReserveAndSetPos_05E4B2  ($05E4B2, 24 B, entity/reserve_and_setpos_05e4b2.s)
/// $05E4B2: bset.b #$6, $13(a6)
pub fn i_05e4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4B8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05E4B8: jsr $5e4ca(pc)
pub fn i_05e4b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4BC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x5E4BC);
    cpu.pc = 0x5E4CA;
    16
}
/// $05E4BC: move.w d0, $38(a6)
pub fn i_05e4bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4C0;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E4C0: move.w d1, $22(a6)
pub fn i_05e4c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4C4;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E4C4: move.w d2, $24(a6)
pub fn i_05e4c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4C8;
    cpu.ir = 0x3D42;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E4C8: rts
pub fn i_05e4c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parent_GetPrioPos_05e4ca  ($05E4CA, 18 B, props/late_props_turrets_05e000.s)
/// $05E4CA: movea.l $c(a6), a0
pub fn i_05e4ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4CE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E4CE: move.w $38(a0), d0
pub fn i_05e4ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4D2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E4D2: move.w $22(a0), d1
pub fn i_05e4d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4D6;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E4D6: move.w $24(a0), d2
pub fn i_05e4d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4DA;
    cpu.ir = 0x3428;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $05E4DA: rts
pub fn i_05e4da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Parent_CopyPosPrio_05e4ee  ($05E4EE, 24 B, props/late_props_turrets_05e000.s)
/// $05E4EE: movea.l $c(a6), a0
pub fn i_05e4ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4F2;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0xC);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $05E4F2: move.w $22(a0), $22(a6)
pub fn i_05e4f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4F8;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E4F8: move.w $24(a0), $24(a6)
pub fn i_05e4f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E4FE;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E4FE: move.w $38(a0), $38(a6)
pub fn i_05e4fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E504;
    cpu.ir = 0x3D68;
    let ad2 = cpu.a[0].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $05E504: rts
pub fn i_05e504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E506;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_MarkFlag2TimerMax_05e98a  ($05E98A, 14 B, props/late_props_turrets_05e000.s)
/// $05E98A: ori.b #$4, $12(a6)
pub fn i_05e98a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E990;
    cpu.ir = 0x2E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $05E990: move.w #$ffff, $30(a6)
pub fn i_05e990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E996;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x30);
    bus.write16(ad2, v1 as u16);
    12
}
/// $05E996: rts
pub fn i_05e996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E998;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rng_Seed_05e998  ($05E998, 30 B, props/late_props_turrets_05e000.s)
/// $05E998: lea.l $10e230.l, a0
pub fn i_05e998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E99E;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x10E230;
    20
}
/// $05E99E: moveq #$1f, d7
pub fn i_05e99e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A0;
    cpu.ir = 0x7E1F;
    cpu.d[7] = 0x1F;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05E9A0: move.w d0, (a0)+
pub fn i_05e9a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A2;
    cpu.ir = 0x30C0;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0]; cpu.a[0] = ad2.wrapping_add(2);
    bus.write16(ad2, v1 as u16);
    8
}
/// $05E9A2: mulu.w #$b7c7, d0
pub fn i_05e9a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9A6;
    cpu.ir = 0xC0FC;
    let v1: u32 = 0xB7C7;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05E9A6: addi.w #$81f5, d0
pub fn i_05e9a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9AA;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x81F5, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9AA: dbra d7, $5e9a0
pub fn i_05e9aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9AE;
    cpu.ir = 0x51CF;
    if cpu.sr.evaluate(Condition::False) { cpu.pc = 0x5E9AE; return 12; }
    let n = (cpu.d[7] as u16).wrapping_sub(1);
    cpu.d[7] = (cpu.d[7] & 0xFFFF_0000) | n as u32;
    if n == 0xFFFF { cpu.pc = 0x5E9AE; return 14; }
    cpu.pc = 0x5E9A0;
    10
}
/// $05E9AE: clr.w $10e270.l
pub fn i_05e9ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9B4;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x10E270) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x10E270, r2 as u16);
    16
}
/// $05E9B4: rts
pub fn i_05e9b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9B6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RNG_LFSRStep_SelfSeed_05E9B6  ($05E9B6, 46 B, kernel/rng_lfsr_self_seed_05e9b6.s)
/// $05E9B6: lea.l $10e230.l, a4
pub fn i_05e9b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9BC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E230;
    20
}
/// $05E9BC: move.w $10e270.l, d7
pub fn i_05e9bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9C2;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E270) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05E9C2: addi.w #$2, d7
pub fn i_05e9c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9C6;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9C6: move.w #$3e, d6
pub fn i_05e9c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9CA;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E9CA: and.w d6, d7
pub fn i_05e9ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9CC;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E9CC: move.w d7, $10e270.l
pub fn i_05e9cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9D2;
    cpu.ir = 0x33C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E270, v1 as u16);
    16
}
/// $05E9D2: move.w (a4, d7.w), d0
pub fn i_05e9d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9D6;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05E9D6: subi.w #$15, d7
pub fn i_05e9d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9DA;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x15, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9DA: and.w d6, d7
pub fn i_05e9da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9DC;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E9DC: adda.w d7, a4
pub fn i_05e9dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9DE;
    cpu.ir = 0xD8C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.a[4] = cpu.a[4].wrapping_add((v1 as u16 as i16 as i32 as u32));
    8
}
/// $05E9DE: eor.w d0, (a4)
pub fn i_05e9de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9E0;
    cpu.ir = 0xB154;
    let ad1 = cpu.a[4];
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    8
}
/// $05E9E0: move.w (a4), d0
pub fn i_05e9e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9E2;
    cpu.ir = 0x3014;
    let v2 = bus.read16(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $05E9E2: rts
pub fn i_05e9e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9E4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== RNG_LFSRStep_05E9E4  ($05E9E4, 56 B, kernel/rng_lfsr_step_05e9e4.s)
/// $05E9E4: move.w d0, d5
pub fn i_05e9e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9E6;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E9E6: lea.l $10e230.l, a4
pub fn i_05e9e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9EC;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E230;
    20
}
/// $05E9EC: move.w $10e270.l, d7
pub fn i_05e9ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9F2;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E270) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05E9F2: addi.w #$2, d7
pub fn i_05e9f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9F6;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05E9F6: move.w #$3e, d6
pub fn i_05e9f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9FA;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05E9FA: and.w d6, d7
pub fn i_05e9fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5E9FC;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05E9FC: move.w d7, $10e270.l
pub fn i_05e9fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA02;
    cpu.ir = 0x33C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E270, v1 as u16);
    16
}
/// $05EA02: move.w (a4, d7.w), d0
pub fn i_05ea02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA06;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05EA06: rol.w #$1, d0
pub fn i_05ea06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA08;
    cpu.ir = 0xE358;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05EA08: subi.w #$15, d7
pub fn i_05ea08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA0C;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x15, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05EA0C: and.w d6, d7
pub fn i_05ea0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA0E;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05EA0E: eor.w d0, (a4, d7.w)
pub fn i_05ea0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA12;
    cpu.ir = 0xB174;
    let ad1 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    14
}
/// $05EA12: move.w (a4, d7.w), d0
pub fn i_05ea12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA16;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05EA16: mulu.w d5, d0
pub fn i_05ea16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA18;
    cpu.ir = 0xC0C5;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[0] & 0xFFFF);
    cpu.d[0] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $05EA18: swap d0
pub fn i_05ea18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA1A;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $05EA1A: rts
pub fn i_05ea1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA1C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Rng_Mask_05ea1c  ($05EA1C, 54 B, props/late_props_turrets_05e000.s)
/// $05EA1C: move.w d0, d5
pub fn i_05ea1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA1E;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EA1E: lea.l $10e230.l, a4
pub fn i_05ea1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA24;
    cpu.ir = 0x49F9;
    cpu.a[4] = 0x10E230;
    20
}
/// $05EA24: move.w $10e270.l, d7
pub fn i_05ea24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA2A;
    cpu.ir = 0x3E39;
    let v2 = bus.read16(0x10E270) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    16
}
/// $05EA2A: addi.w #$2, d7
pub fn i_05ea2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA2E;
    cpu.ir = 0x647;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05EA2E: move.w #$3e, d6
pub fn i_05ea2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA32;
    cpu.ir = 0x3C3C;
    let v1: u32 = 0x3E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $05EA32: and.w d6, d7
pub fn i_05ea32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA34;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05EA34: move.w d7, $10e270.l
pub fn i_05ea34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA3A;
    cpu.ir = 0x33C7;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(0x10E270, v1 as u16);
    16
}
/// $05EA3A: move.w (a4, d7.w), d0
pub fn i_05ea3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA3E;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05EA3E: rol.w #$1, d0
pub fn i_05ea3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA40;
    cpu.ir = 0xE358;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 3, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $05EA40: subi.w #$15, d7
pub fn i_05ea40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA44;
    cpu.ir = 0x447;
    let v1 = cpu.d[7] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x15, Size::Word, true);
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $05EA44: and.w d6, d7
pub fn i_05ea44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA46;
    cpu.ir = 0xCE46;
    let v1 = cpu.d[6] & 0xFFFF;
    let r = (v1 & cpu.d[7]) & 0xFFFF;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05EA46: eor.w d0, (a4, d7.w)
pub fn i_05ea46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA4A;
    cpu.ir = 0xB174;
    let ad1 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    14
}
/// $05EA4A: move.w (a4, d7.w), d0
pub fn i_05ea4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA4E;
    cpu.ir = 0x3034;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[7] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $05EA4E: and.w d5, d0
pub fn i_05ea4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA50;
    cpu.ir = 0xC045;
    let v1 = cpu.d[5] & 0xFFFF;
    let r = (v1 & cpu.d[0]) & 0xFFFF;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $05EA50: rts
pub fn i_05ea50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA52;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Task_Clear106F42_05ea96  ($05EA96, 14 B, props/late_props_turrets_05e000.s)
/// $05EA96: lea.l $5ea9c(pc), a1
pub fn i_05ea96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA9A;
    cpu.ir = 0x43FA;
    cpu.a[1] = 0x5EA9C;
    16
}
/// $05EA9A: move.l a1, (a6)
pub fn i_05ea9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EA9C;
    cpu.ir = 0x2C89;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    bus.write32(cpu.a[6], v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    12
}
/// $05EA9C: clr.w $106f42.l
pub fn i_05ea9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAA2;
    cpu.ir = 0x4279;
    let v1 = bus.read16(0x106F42) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write16(0x106F42, r2 as u16);
    16
}
/// $05EAA2: rts
pub fn i_05eaa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAA4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slot1008A0_Alloc6FE_05eab6  ($05EAB6, 18 B, props/late_props_turrets_05e000.s)
/// $05EAB6: move.l a6, -(a7)
pub fn i_05eab6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAB8;
    cpu.ir = 0x2F0E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let o2 = cpu.a[7];
    bus.write16(o2.wrapping_sub(2), v1 as u16);
    cpu.a[7] = o2.wrapping_sub(4);
    bus.write16(o2.wrapping_sub(4), (v1 >> 16) as u16);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    14
}
/// $05EAB8: lea.l $1008a0.l, a6
pub fn i_05eab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EABE;
    cpu.ir = 0x4DF9;
    cpu.a[6] = 0x1008A0;
    20
}
/// $05EABE: jsr $6fe.l
pub fn i_05eabe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAC4;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x5EAC4);
    cpu.pc = 0x6FE;
    16
}
/// $05EAC4: movea.l (a7)+, a6
pub fn i_05eac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAC6;
    cpu.ir = 0x2C5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(4);
    let v3 = bus.read32(ad2);
    cpu.a[6] = v3;
    12
}
/// $05EAC6: rts
pub fn i_05eac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x5EAC8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
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
    (0x05E0CE, i_05e0ce),
    (0x05E0D0, i_05e0d0),
    (0x05E0D4, i_05e0d4),
    (0x05E0D8, i_05e0d8),
    (0x05E0DA, i_05e0da),
    (0x05E0DE, i_05e0de),
    (0x05E0E4, i_05e0e4),
    (0x05E0EA, i_05e0ea),
    (0x05E0EE, i_05e0ee),
    (0x05E0F2, i_05e0f2),
    (0x05E0F6, i_05e0f6),
    (0x05E0FA, i_05e0fa),
    (0x05E0FE, i_05e0fe),
    (0x05E102, i_05e102),
    (0x05E106, i_05e106),
    (0x05E108, i_05e108),
    (0x05E10C, i_05e10c),
    (0x05E10E, i_05e10e),
    (0x05E110, i_05e110),
    (0x05E114, i_05e114),
    (0x05E116, i_05e116),
    (0x05E118, i_05e118),
    (0x05E11C, i_05e11c),
    (0x05E120, i_05e120),
    (0x05E122, i_05e122),
    (0x05E124, i_05e124),
    (0x05E128, i_05e128),
    (0x05E12A, i_05e12a),
    (0x05E130, i_05e130),
    (0x05E134, i_05e134),
    (0x05E1AA, i_05e1aa),
    (0x05E1AC, i_05e1ac),
    (0x05E1B0, i_05e1b0),
    (0x05E1B4, i_05e1b4),
    (0x05E1B8, i_05e1b8),
    (0x05E1BC, i_05e1bc),
    (0x05E1BE, i_05e1be),
    (0x05E1C0, i_05e1c0),
    (0x05E1C2, i_05e1c2),
    (0x05E1C6, i_05e1c6),
    (0x05E1CA, i_05e1ca),
    (0x05E1CE, i_05e1ce),
    (0x05E1D2, i_05e1d2),
    (0x05E1D4, i_05e1d4),
    (0x05E1D6, i_05e1d6),
    (0x05E1D8, i_05e1d8),
    (0x05E1DA, i_05e1da),
    (0x05E1DE, i_05e1de),
    (0x05E1E2, i_05e1e2),
    (0x05E1E4, i_05e1e4),
    (0x05E1E8, i_05e1e8),
    (0x05E23A, i_05e23a),
    (0x05E23C, i_05e23c),
    (0x05E23E, i_05e23e),
    (0x05E242, i_05e242),
    (0x05E246, i_05e246),
    (0x05E248, i_05e248),
    (0x05E24C, i_05e24c),
    (0x05E250, i_05e250),
    (0x05E252, i_05e252),
    (0x05E254, i_05e254),
    (0x05E258, i_05e258),
    (0x05E25A, i_05e25a),
    (0x05E25C, i_05e25c),
    (0x05E25E, i_05e25e),
    (0x05E260, i_05e260),
    (0x05E264, i_05e264),
    (0x05E268, i_05e268),
    (0x05E26E, i_05e26e),
    (0x05E272, i_05e272),
    (0x05E274, i_05e274),
    (0x05E278, i_05e278),
    (0x05E27C, i_05e27c),
    (0x05E280, i_05e280),
    (0x05E284, i_05e284),
    (0x05E286, i_05e286),
    (0x05E288, i_05e288),
    (0x05E28C, i_05e28c),
    (0x05E290, i_05e290),
    (0x05E294, i_05e294),
    (0x05E298, i_05e298),
    (0x05E29C, i_05e29c),
    (0x05E2A2, i_05e2a2),
    (0x05E2A6, i_05e2a6),
    (0x05E2AA, i_05e2aa),
    (0x05E2AE, i_05e2ae),
    (0x05E2B2, i_05e2b2),
    (0x05E2B6, i_05e2b6),
    (0x05E2BA, i_05e2ba),
    (0x05E2BC, i_05e2bc),
    (0x05E2C0, i_05e2c0),
    (0x05E2C4, i_05e2c4),
    (0x05E2C8, i_05e2c8),
    (0x05E2CC, i_05e2cc),
    (0x05E2D0, i_05e2d0),
    (0x05E2D2, i_05e2d2),
    (0x05E2D6, i_05e2d6),
    (0x05E2D8, i_05e2d8),
    (0x05E2DA, i_05e2da),
    (0x05E2DC, i_05e2dc),
    (0x05E2E2, i_05e2e2),
    (0x05E2E6, i_05e2e6),
    (0x05E2EC, i_05e2ec),
    (0x05E2F0, i_05e2f0),
    (0x05E2F6, i_05e2f6),
    (0x05E2FA, i_05e2fa),
    (0x05E302, i_05e302),
    (0x05E306, i_05e306),
    (0x05E308, i_05e308),
    (0x05E30C, i_05e30c),
    (0x05E312, i_05e312),
    (0x05E316, i_05e316),
    (0x05E318, i_05e318),
    (0x05E31C, i_05e31c),
    (0x05E320, i_05e320),
    (0x05E322, i_05e322),
    (0x05E326, i_05e326),
    (0x05E32C, i_05e32c),
    (0x05E330, i_05e330),
    (0x05E332, i_05e332),
    (0x05E336, i_05e336),
    (0x05E3A2, i_05e3a2),
    (0x05E3A6, i_05e3a6),
    (0x05E3AA, i_05e3aa),
    (0x05E3AC, i_05e3ac),
    (0x05E3B0, i_05e3b0),
    (0x05E3B6, i_05e3b6),
    (0x05E3BA, i_05e3ba),
    (0x05E3C0, i_05e3c0),
    (0x05E3C4, i_05e3c4),
    (0x05E3CA, i_05e3ca),
    (0x05E3CE, i_05e3ce),
    (0x05E3D4, i_05e3d4),
    (0x05E3D8, i_05e3d8),
    (0x05E3DE, i_05e3de),
    (0x05E3E2, i_05e3e2),
    (0x05E3E6, i_05e3e6),
    (0x05E3E8, i_05e3e8),
    (0x05E3EE, i_05e3ee),
    (0x05E3F2, i_05e3f2),
    (0x05E452, i_05e452),
    (0x05E456, i_05e456),
    (0x05E45A, i_05e45a),
    (0x05E45E, i_05e45e),
    (0x05E464, i_05e464),
    (0x05E468, i_05e468),
    (0x05E46E, i_05e46e),
    (0x05E472, i_05e472),
    (0x05E478, i_05e478),
    (0x05E47C, i_05e47c),
    (0x05E480, i_05e480),
    (0x05E482, i_05e482),
    (0x05E486, i_05e486),
    (0x05E4B2, i_05e4b2),
    (0x05E4B8, i_05e4b8),
    (0x05E4BC, i_05e4bc),
    (0x05E4C0, i_05e4c0),
    (0x05E4C4, i_05e4c4),
    (0x05E4C8, i_05e4c8),
    (0x05E4CA, i_05e4ca),
    (0x05E4CE, i_05e4ce),
    (0x05E4D2, i_05e4d2),
    (0x05E4D6, i_05e4d6),
    (0x05E4DA, i_05e4da),
    (0x05E4EE, i_05e4ee),
    (0x05E4F2, i_05e4f2),
    (0x05E4F8, i_05e4f8),
    (0x05E4FE, i_05e4fe),
    (0x05E504, i_05e504),
    (0x05E98A, i_05e98a),
    (0x05E990, i_05e990),
    (0x05E996, i_05e996),
    (0x05E998, i_05e998),
    (0x05E99E, i_05e99e),
    (0x05E9A0, i_05e9a0),
    (0x05E9A2, i_05e9a2),
    (0x05E9A6, i_05e9a6),
    (0x05E9AA, i_05e9aa),
    (0x05E9AE, i_05e9ae),
    (0x05E9B4, i_05e9b4),
    (0x05E9B6, i_05e9b6),
    (0x05E9BC, i_05e9bc),
    (0x05E9C2, i_05e9c2),
    (0x05E9C6, i_05e9c6),
    (0x05E9CA, i_05e9ca),
    (0x05E9CC, i_05e9cc),
    (0x05E9D2, i_05e9d2),
    (0x05E9D6, i_05e9d6),
    (0x05E9DA, i_05e9da),
    (0x05E9DC, i_05e9dc),
    (0x05E9DE, i_05e9de),
    (0x05E9E0, i_05e9e0),
    (0x05E9E2, i_05e9e2),
    (0x05E9E4, i_05e9e4),
    (0x05E9E6, i_05e9e6),
    (0x05E9EC, i_05e9ec),
    (0x05E9F2, i_05e9f2),
    (0x05E9F6, i_05e9f6),
    (0x05E9FA, i_05e9fa),
    (0x05E9FC, i_05e9fc),
    (0x05EA02, i_05ea02),
    (0x05EA06, i_05ea06),
    (0x05EA08, i_05ea08),
    (0x05EA0C, i_05ea0c),
    (0x05EA0E, i_05ea0e),
    (0x05EA12, i_05ea12),
    (0x05EA16, i_05ea16),
    (0x05EA18, i_05ea18),
    (0x05EA1A, i_05ea1a),
    (0x05EA1C, i_05ea1c),
    (0x05EA1E, i_05ea1e),
    (0x05EA24, i_05ea24),
    (0x05EA2A, i_05ea2a),
    (0x05EA2E, i_05ea2e),
    (0x05EA32, i_05ea32),
    (0x05EA34, i_05ea34),
    (0x05EA3A, i_05ea3a),
    (0x05EA3E, i_05ea3e),
    (0x05EA40, i_05ea40),
    (0x05EA44, i_05ea44),
    (0x05EA46, i_05ea46),
    (0x05EA4A, i_05ea4a),
    (0x05EA4E, i_05ea4e),
    (0x05EA50, i_05ea50),
    (0x05EA96, i_05ea96),
    (0x05EA9A, i_05ea9a),
    (0x05EA9C, i_05ea9c),
    (0x05EAA2, i_05eaa2),
    (0x05EAB6, i_05eab6),
    (0x05EAB8, i_05eab8),
    (0x05EABE, i_05eabe),
    (0x05EAC4, i_05eac4),
    (0x05EAC6, i_05eac6),
];
