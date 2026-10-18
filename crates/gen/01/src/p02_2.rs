// GENERADO por tools/recomp/m68k_recomp.py — no editar a mano.
// Programa 68000 de Metal Slug (región p02_2) traducido a Rust,
// una función por instrucción: misma semántica y mismos ciclos que la CPU.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments, unused_imports, clippy::all)]
use neogeo_hw::cpu::m68k::{Bus, Condition, Cpu, Exception, Size, StatusRegister};
use neogeo_hw::cpu::m68k::exec::*;
use neogeo_hw::NeoGeoBus as B;


// ===== SetXN_0280c0  ($0280C0, 6 B, thunks/ccr_helpers.c)
/// $0280C0: ori.b #$11, ccr
pub fn i_0280c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280C4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0280C4: rts
pub fn i_0280c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280C6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_TileUnderToShade_0280c6  ($0280C6, 54 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0280C6: jsr $27db2(pc)
pub fn i_0280c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280CA;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x280CA);
    cpu.pc = 0x27DB2;
    16
}
/// $0280CA: jsr $9993c.l
pub fn i_0280ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280D0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x280D0);
    cpu.pc = 0x9993C;
    16
}
/// $0280D0: move.b d6, $106f44.l
pub fn i_0280d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280D6;
    cpu.ir = 0x13C6;
    let v1 = cpu.d[6] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F44, v1 as u8);
    16
}
/// $0280D6: cmpi.b #$f, d6
pub fn i_0280d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280DA;
    cpu.ir = 0xC06;
    let v1 = cpu.d[6] & 0xFF;
    sub_with_flags(cpu, v1, 0xF, Size::Byte, false);
    8
}
/// $0280DA: beq.w $280f0
pub fn i_0280da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280DE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x280F0; return 10; }
    8
}
/// $0280DE: moveq #$0, d0
pub fn i_0280de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280E0;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0280E0: moveq #$0, d3
pub fn i_0280e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280E2;
    cpu.ir = 0x7600;
    cpu.d[3] = 0x0;
    cpu.sr.set_nz(cpu.d[3], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0280E2: move.w d5, d4
pub fn i_0280e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280E4;
    cpu.ir = 0x3805;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0280E4: moveq #$11, d0
pub fn i_0280e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280E6;
    cpu.ir = 0x7011;
    cpu.d[0] = 0x11;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0280E6: lea.l $278ba8.l, a1
pub fn i_0280e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280EC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x278BA8;
    20
}
/// $0280EC: bra.w $280f0
pub fn i_0280ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280F0;
    cpu.ir = 0x6000;
    cpu.pc = 0x280F0;
    10
}
/// $0280F0: andi.b #$3f, d0
pub fn i_0280f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280F4;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x3F) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0280F4: cmpi.b #$0, d0
pub fn i_0280f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280F8;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $0280F8: beq.w $28102
pub fn i_0280f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x280FC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28102; return 10; }
    8
}
// ===== SetXN_0280fc  ($0280FC, 6 B, thunks/ccr_helpers.c)
/// $0280FC: ori.b #$11, ccr
pub fn i_0280fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28100;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028100: rts
pub fn i_028100(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28102;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_028102  ($028102, 6 B, thunks/ccr_helpers.c)
/// $028102: andi.b #$ee, ccr
pub fn i_028102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28106;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028106: rts
pub fn i_028106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28108;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ApplyFadeShade_028108  ($028108, 44 B, entity/apply_fade_shade_028108.s)
/// $028108: move.w $24(a6), d6
pub fn i_028108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2810C;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02810C: move.w $38(a6), d5
pub fn i_02810c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28110;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028110: subi.w #$4000, d5
pub fn i_028110(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28114;
    cpu.ir = 0x445;
    let v1 = cpu.d[5] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x4000, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $028114: cmpi.w #$7fff, d5
pub fn i_028114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28118;
    cpu.ir = 0xC45;
    let v1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x7FFF, Size::Word, false);
    8
}
/// $028118: bcc.w $28132
pub fn i_028118(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2811C;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28132; return 10; }
    8
}
/// $02811C: andi.w #$ff, d6
pub fn i_02811c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28120;
    cpu.ir = 0x246;
    let v1 = cpu.d[6] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $028120: move.w #$ff, d5
pub fn i_028120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28124;
    cpu.ir = 0x3A3C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028124: sub.w d6, d5
pub fn i_028124(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28126;
    cpu.ir = 0x9A46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028126: asl.w #$5, d5
pub fn i_028126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28128;
    cpu.ir = 0xEB45;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 5, Size::Word, 0, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    16
}
/// $028128: andi.w #$e01f, $38(a6)
pub fn i_028128(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2812E;
    cpu.ir = 0x26E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0xE01F) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02812E: or.w d5, $38(a6)
pub fn i_02812e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28132;
    cpu.ir = 0x8B6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | cpu.d[5]) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $028132: rts
pub fn i_028132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28134;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntitySetField38AndUpdate  ($028134, 8 B, entity/entity_setters.c)
/// $028134: move.w d0, $38(a6)
pub fn i_028134(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28138;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $028138: jmp $28108(pc)
pub fn i_028138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2813C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28108;
    8
}
// ===== Entity_SetField38FromTile_02813c  ($02813C, 28 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02813C: andi.w #$ffe3, d0
pub fn i_02813c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28140;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFE3) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $028140: ori.w #$0, d0
pub fn i_028140(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28144;
    cpu.ir = 0x40;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 | 0x0) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $028144: move.w d0, d5
pub fn i_028144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28146;
    cpu.ir = 0x3A00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028146: clr.w d0
pub fn i_028146(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28148;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028148: jsr $440d0.l
pub fn i_028148(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2814E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2814E);
    cpu.pc = 0x440D0;
    16
}
/// $02814E: move.w d1, d6
pub fn i_02814e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28150;
    cpu.ir = 0x3C01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028150: move.w d5, $38(a6)
pub fn i_028150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28154;
    cpu.ir = 0x3D45;
    let v1 = cpu.d[5] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $028154: jmp $28110(pc)
pub fn i_028154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28158;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28110;
    8
}
// ===== Entity_GravityToIndex_028158  ($028158, 68 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028158: clr.w d0
pub fn i_028158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2815A;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02815A: move.w $34(a6), d7
pub fn i_02815a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2815E;
    cpu.ir = 0x3E2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02815E: cmpi.w #$400, d7
pub fn i_02815e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28162;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x400, Size::Word, false);
    8
}
/// $028162: beq.w $28184
pub fn i_028162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28166;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28184; return 10; }
    8
}
/// $028166: cmpi.w #$fc00, d7
pub fn i_028166(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2816A;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFC00, Size::Word, false);
    8
}
/// $02816A: beq.w $2818a
pub fn i_02816a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2816E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2818A; return 10; }
    8
}
/// $02816E: cmpi.w #$1000, d7
pub fn i_02816e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28172;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1000, Size::Word, false);
    8
}
/// $028172: beq.w $28190
pub fn i_028172(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28176;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28190; return 10; }
    8
}
/// $028176: cmpi.w #$f000, d7
pub fn i_028176(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2817A;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF000, Size::Word, false);
    8
}
/// $02817A: beq.w $28196
pub fn i_02817a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2817E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28196; return 10; }
    8
}
/// $02817E: bra.w $28182
pub fn i_02817e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28182;
    cpu.ir = 0x6000;
    cpu.pc = 0x28182;
    10
}
/// $028182: rts
pub fn i_028182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28184;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $028184: move.w #$3, d0
pub fn i_028184(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28188;
    cpu.ir = 0x303C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028188: bra.b $28182
pub fn i_028188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2818A;
    cpu.ir = 0x60F8;
    cpu.pc = 0x28182;
    10
}
/// $02818A: move.w #$4, d0
pub fn i_02818a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2818E;
    cpu.ir = 0x303C;
    let v1: u32 = 0x4;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02818E: bra.b $28182
pub fn i_02818e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28190;
    cpu.ir = 0x60F2;
    cpu.pc = 0x28182;
    10
}
/// $028190: move.w #$1, d0
pub fn i_028190(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28194;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028194: bra.b $28182
pub fn i_028194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28196;
    cpu.ir = 0x60EC;
    cpu.pc = 0x28182;
    10
}
/// $028196: move.w #$2, d0
pub fn i_028196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2819A;
    cpu.ir = 0x303C;
    let v1: u32 = 0x2;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02819A: bra.b $28182
pub fn i_02819a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2819C;
    cpu.ir = 0x60E6;
    cpu.pc = 0x28182;
    10
}
// ===== Entity_ClearVelLatch_02819c  ($02819C, 20 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02819C: movea.l #$108080, a5
pub fn i_02819c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281A2;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $0281A2: move.w #$ffff, -$1142(a5)
pub fn i_0281a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281A8;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0281A8: move.w #$ffff, -$1140(a5)
pub fn i_0281a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281AE;
    cpu.ir = 0x3B7C;
    let v1: u32 = 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEC0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0281AE: rts
pub fn i_0281ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SetVelLatchX_0281b0  ($0281B0, 12 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0281B0: movea.l #$108080, a5
pub fn i_0281b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281B6;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $0281B6: move.w d0, -$1142(a5)
pub fn i_0281b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281BA;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBE);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0281BA: rts
pub fn i_0281ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281BC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SetVelLatchY_0281bc  ($0281BC, 12 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0281BC: movea.l #$108080, a5
pub fn i_0281bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281C2;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $0281C2: move.w d0, -$1140(a5)
pub fn i_0281c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281C6;
    cpu.ir = 0x3B40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEC0);
    bus.write16(ad2, v1 as u16);
    12
}
/// $0281C6: rts
pub fn i_0281c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281C8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ApplyVelLatch_0281c8  ($0281C8, 100 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0281C8: movea.l #$108080, a5
pub fn i_0281c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281CE;
    cpu.ir = 0x2A7C;
    let v1: u32 = 0x108080;
    cpu.a[5] = v1;
    4
}
/// $0281CE: tst.w -$1142(a5)
pub fn i_0281ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281D2;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $0281D2: beq.w $281fc
pub fn i_0281d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281D6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x281FC; return 10; }
    8
}
/// $0281D6: move.w $28(a6), d1
pub fn i_0281d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281DA;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0281DA: clr.w d2
pub fn i_0281da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281DC;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0281DC: tst.w d1
pub fn i_0281dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281DE;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0281DE: beq.w $281e6
pub fn i_0281de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x281E6; return 10; }
    8
}
/// $0281E2: move.w #$1, d2
pub fn i_0281e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281E6;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0281E6: ext.l d1
pub fn i_0281e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281E8;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0281E8: swap d1
pub fn i_0281e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281EA;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0281EA: or.w d2, d1
pub fn i_0281ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281EC;
    cpu.ir = 0x8242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0281EC: cmp.w -$1142(a5), d1
pub fn i_0281ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281F0;
    cpu.ir = 0xB26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEBE);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $0281F0: bne.w $281fc
pub fn i_0281f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281F4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x281FC; return 10; }
    8
}
/// $0281F4: clr.w -$1154(a5)
pub fn i_0281f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281F8;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEAC);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $0281F8: clr.b -$1150(a5)
pub fn i_0281f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x281FC;
    cpu.ir = 0x422D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEB0);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0281FC: tst.w -$1140(a5)
pub fn i_0281fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28200;
    cpu.ir = 0x4A6D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEC0);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    12
}
/// $028200: beq.w $28222
pub fn i_028200(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28204;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28222; return 10; }
    8
}
/// $028204: move.w $2a(a6), d1
pub fn i_028204(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28208;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028208: clr.w d2
pub fn i_028208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2820A;
    cpu.ir = 0x4242;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02820A: tst.w d1
pub fn i_02820a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2820C;
    cpu.ir = 0x4A41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02820C: beq.w $28214
pub fn i_02820c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28210;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28214; return 10; }
    8
}
/// $028210: move.w #$1, d2
pub fn i_028210(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28214;
    cpu.ir = 0x343C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028214: ext.l d1
pub fn i_028214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28216;
    cpu.ir = 0x48C1;
    cpu.d[1] = cpu.d[1] as i16 as i32 as u32;
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028216: swap d1
pub fn i_028216(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28218;
    cpu.ir = 0x4841;
    cpu.d[1] = cpu.d[1].rotate_left(16);
    cpu.sr.set_nz(cpu.d[1], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028218: or.w d2, d1
pub fn i_028218(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2821A;
    cpu.ir = 0x8242;
    let v1 = cpu.d[2] & 0xFFFF;
    let r = (v1 | cpu.d[1]) & 0xFFFF;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    cpu.sr.set_nz(r, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $02821A: cmp.w -$1140(a5), d1
pub fn i_02821a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2821E;
    cpu.ir = 0xB26D;
    let ad2 = cpu.a[5].wrapping_add(0xFFFFEEC0);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02821E: bne.w $28222
pub fn i_02821e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28222;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28222; return 10; }
    8
}
/// $028222: clr.w -$1142(a5)
pub fn i_028222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28226;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEBE);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $028226: clr.w -$1140(a5)
pub fn i_028226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2822A;
    cpu.ir = 0x426D;
    let ad1 = cpu.a[5].wrapping_add(0xFFFFEEC0);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02822A: rts
pub fn i_02822a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2822C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SlopeDirTest_02822c  ($02822C, 86 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02822C: movem.w d5-d7, -(a7)
pub fn i_02822c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28230;
    cpu.ir = 0x48A7;
    let mut a = cpu.a[7];
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[7] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[6] as u16);
    a = a.wrapping_sub(2); bus.write16(a, cpu.d[5] as u16);
    cpu.a[7] = a;
    20
}
/// $028230: move.w $28(a6), d5
pub fn i_028230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28234;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x28);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028234: move.w $2a(a6), d6
pub fn i_028234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28238;
    cpu.ir = 0x3C2E;
    let ad2 = cpu.a[6].wrapping_add(0x2A);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028238: move.w (a3), d7
pub fn i_028238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2823A;
    cpu.ir = 0x3E13;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $02823A: bne.w $28248
pub fn i_02823a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2823E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28248; return 10; }
    8
}
/// $02823E: tst.w d6
pub fn i_02823e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28240;
    cpu.ir = 0x4A46;
    let v1 = cpu.d[6] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $028240: bgt.w $28288
pub fn i_028240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28244;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28288; return 10; }
    8
}
/// $028244: bra.w $2827e
pub fn i_028244(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28248;
    cpu.ir = 0x6000;
    cpu.pc = 0x2827E;
    10
}
/// $028248: cmpi.w #$ffff, d7
pub fn i_028248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2824C;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02824C: beq.w $28288
pub fn i_02824c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28250;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28288; return 10; }
    8
}
/// $028250: tst.w d7
pub fn i_028250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28252;
    cpu.ir = 0x4A47;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $028252: bmi.w $2826c
pub fn i_028252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28256;
    cpu.ir = 0x6B00;
    if cpu.sr.evaluate(Condition::Mi) { cpu.pc = 0x2826C; return 10; }
    8
}
/// $028256: cmpi.w #$1000, d7
pub fn i_028256(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2825A;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x1000, Size::Word, false);
    8
}
/// $02825A: bne.w $28262
pub fn i_02825a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2825E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28262; return 10; }
    8
}
/// $02825E: add.w d6, d6
pub fn i_02825e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28260;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028260: add.w d6, d6
pub fn i_028260(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28262;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028262: sub.w d5, d6
pub fn i_028262(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28264;
    cpu.ir = 0x9C45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028264: bgt.w $28288
pub fn i_028264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28268;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28288; return 10; }
    8
}
/// $028268: bra.w $2827e
pub fn i_028268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2826C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2827E;
    10
}
/// $02826C: cmpi.w #$f000, d7
pub fn i_02826c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28270;
    cpu.ir = 0xC47;
    let v1 = cpu.d[7] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xF000, Size::Word, false);
    8
}
/// $028270: bne.w $28278
pub fn i_028270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28274;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28278; return 10; }
    8
}
/// $028274: add.w d6, d6
pub fn i_028274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28276;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028276: add.w d6, d6
pub fn i_028276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28278;
    cpu.ir = 0xDC46;
    let v1 = cpu.d[6] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028278: add.w d5, d6
pub fn i_028278(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2827A;
    cpu.ir = 0xDC45;
    let v1 = cpu.d[5] & 0xFFFF;
    let h1 = cpu.d[6] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[6] = (cpu.d[6] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02827A: bgt.w $28288
pub fn i_02827a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2827E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28288; return 10; }
    8
}
/// $02827E: movem.w (a7)+, d5-d7
pub fn i_02827e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28282;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[7] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== ClearC_028282  ($028282, 6 B, thunks/ccr_helpers.c)
/// $028282: andi.b #$fe, ccr
pub fn i_028282(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28286;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $028286: rts
pub fn i_028286(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28288;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SlopeDirTest_Tail_028288  ($028288, 4 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028288: movem.w (a7)+, d5-d7
pub fn i_028288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2828C;
    cpu.ir = 0x4C9F;
    let mut a = cpu.a[7];
    cpu.d[5] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[6] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.d[7] = bus.read16(a) as i16 as i32 as u32; a = a.wrapping_add(2);
    cpu.a[7] = a;
    20
}
// ===== SetC_02828c  ($02828C, 6 B, thunks/ccr_helpers.c)
/// $02828C: ori.b #$1, ccr
pub fn i_02828c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28290;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $028290: rts
pub fn i_028290(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28292;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_SwapProbeCommit_028292  ($028292, 70 B, entity/swap_probe_028292.s)
/// $028292: cmpi.w #$ffff, $60(a6)
pub fn i_028292(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28298;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $028298: beq.w $282a4
pub fn i_028298(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2829C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x282A4; return 10; }
    8
}
/// $02829C: move.l #$ffffffff, $60(a6)
pub fn i_02829c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282A4;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0282A4: move.w $34(a6), -(a7)
pub fn i_0282a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282A8;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $0282A8: move.w $62(a6), $34(a6)
pub fn i_0282a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282AE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x62);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0282AE: jsr $27a92(pc)
pub fn i_0282ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282B2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x282B2);
    cpu.pc = 0x27A92;
    16
}
/// $0282B2: bcc.w $282c8
pub fn i_0282b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282B6;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x282C8; return 10; }
    8
}
/// $0282B6: move.w $34(a6), $62(a6)
pub fn i_0282b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282BC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x62);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0282BC: move.w (a7)+, $34(a6)
pub fn i_0282bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282C0;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    16
}
/// $0282C0: ori.b #$11, ccr
pub fn i_0282c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282C4;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0282C4: bra.w $282d6
pub fn i_0282c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282C8;
    cpu.ir = 0x6000;
    cpu.pc = 0x282D6;
    10
}
/// $0282C8: move.w $34(a6), $62(a6)
pub fn i_0282c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282CE;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x62);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0282CE: move.w (a7)+, $34(a6)
pub fn i_0282ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282D2;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    16
}
/// $0282D2: andi.b #$ee, ccr
pub fn i_0282d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282D6;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0282D6: rts
pub fn i_0282d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeSwapGravity_A_0282d8  ($0282D8, 64 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0282D8: cmpi.w #$ffff, $60(a6)
pub fn i_0282d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282DE;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $0282DE: beq.w $282ea
pub fn i_0282de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282E2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x282EA; return 10; }
    8
}
/// $0282E2: move.l #$ffffffff, $60(a6)
pub fn i_0282e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282EA;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0282EA: move.w $34(a6), -(a7)
pub fn i_0282ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282EE;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $0282EE: move.w $62(a6), $34(a6)
pub fn i_0282ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282F4;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x62);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0282F4: jsr $27afc(pc)
pub fn i_0282f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282F8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x282F8);
    cpu.pc = 0x27AFC;
    16
}
/// $0282F8: bcc.w $2830e
pub fn i_0282f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x282FC;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2830E; return 10; }
    8
}
/// $0282FC: move.w $34(a6), $62(a6)
pub fn i_0282fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28302;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x62);
    bus.write16(ad4, v3 as u16);
    20
}
/// $028302: move.w (a7)+, $34(a6)
pub fn i_028302(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28306;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    16
}
/// $028306: ori.b #$11, ccr
pub fn i_028306(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2830A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02830A: bra.w $2831c
pub fn i_02830a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2830E;
    cpu.ir = 0x6000;
    cpu.pc = 0x2831C;
    10
}
/// $02830E: move.w $34(a6), $62(a6)
pub fn i_02830e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28314;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x62);
    bus.write16(ad4, v3 as u16);
    20
}
/// $028314: move.w (a7)+, $34(a6)
pub fn i_028314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28318;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    16
}
// ===== ClearXN_028318  ($028318, 6 B, thunks/ccr_helpers.c)
/// $028318: andi.b #$ee, ccr
pub fn i_028318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2831C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02831C: rts
pub fn i_02831c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2831E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeSwapGravity_B_02831e  ($02831E, 64 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02831E: cmpi.w #$ffff, $60(a6)
pub fn i_02831e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28324;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $028324: beq.w $28330
pub fn i_028324(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28328;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28330; return 10; }
    8
}
/// $028328: move.l #$ffffffff, $60(a6)
pub fn i_028328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28330;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028330: move.w $34(a6), -(a7)
pub fn i_028330(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28334;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $028334: move.w $62(a6), $34(a6)
pub fn i_028334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2833A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x62);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02833A: jsr $27bc8(pc)
pub fn i_02833a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2833E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2833E);
    cpu.pc = 0x27BC8;
    16
}
/// $02833E: bcc.w $28354
pub fn i_02833e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28342;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28354; return 10; }
    8
}
/// $028342: move.w $34(a6), $62(a6)
pub fn i_028342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28348;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x62);
    bus.write16(ad4, v3 as u16);
    20
}
/// $028348: move.w (a7)+, $34(a6)
pub fn i_028348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2834C;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    16
}
/// $02834C: ori.b #$11, ccr
pub fn i_02834c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28350;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028350: bra.w $28362
pub fn i_028350(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28354;
    cpu.ir = 0x6000;
    cpu.pc = 0x28362;
    10
}
/// $028354: move.w $34(a6), $62(a6)
pub fn i_028354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2835A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x62);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02835A: move.w (a7)+, $34(a6)
pub fn i_02835a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2835E;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    16
}
// ===== ClearXN_02835e  ($02835E, 6 B, thunks/ccr_helpers.c)
/// $02835E: andi.b #$ee, ccr
pub fn i_02835e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28362;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028362: rts
pub fn i_028362(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28364;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeSwapGravity_C_028364  ($028364, 64 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028364: cmpi.w #$ffff, $60(a6)
pub fn i_028364(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2836A;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $02836A: beq.w $28376
pub fn i_02836a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2836E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28376; return 10; }
    8
}
/// $02836E: move.l #$ffffffff, $60(a6)
pub fn i_02836e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28376;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028376: move.w $34(a6), -(a7)
pub fn i_028376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2837A;
    cpu.ir = 0x3F2E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[7].wrapping_sub(2); cpu.a[7] = ad4;
    bus.write16(ad4, v3 as u16);
    18
}
/// $02837A: move.w $62(a6), $34(a6)
pub fn i_02837a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28380;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x62);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    20
}
/// $028380: jsr $27c8c(pc)
pub fn i_028380(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28384;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28384);
    cpu.pc = 0x27C8C;
    16
}
/// $028384: bcc.w $2839a
pub fn i_028384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28388;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2839A; return 10; }
    8
}
/// $028388: move.w $34(a6), $62(a6)
pub fn i_028388(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2838E;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x62);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02838E: move.w (a7)+, $34(a6)
pub fn i_02838e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28392;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    16
}
/// $028392: ori.b #$11, ccr
pub fn i_028392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28396;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028396: bra.w $283a8
pub fn i_028396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2839A;
    cpu.ir = 0x6000;
    cpu.pc = 0x283A8;
    10
}
/// $02839A: move.w $34(a6), $62(a6)
pub fn i_02839a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283A0;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x34);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x62);
    bus.write16(ad4, v3 as u16);
    20
}
/// $0283A0: move.w (a7)+, $34(a6)
pub fn i_0283a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283A4;
    cpu.ir = 0x3D5F;
    let ad2 = cpu.a[7]; cpu.a[7] = ad2.wrapping_add(2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x34);
    bus.write16(ad4, v3 as u16);
    16
}
// ===== ClearXN_0283a4  ($0283A4, 6 B, thunks/ccr_helpers.c)
/// $0283A4: andi.b #$ee, ccr
pub fn i_0283a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283A8;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0283A8: rts
pub fn i_0283a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearD3Rts_0283aa  ($0283AA, 4 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0283AA: clr.b d3
pub fn i_0283aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283AC;
    cpu.ir = 0x4203;
    let v1 = cpu.d[3] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (r2 & 0xFF);
    4
}
/// $0283AC: rts
pub fn i_0283ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283AE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_0283ae  ($0283AE, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0283AE: movea.l $8(a6), a1
pub fn i_0283ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283B2;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $0283B2: move.b $10(a6), d0
pub fn i_0283b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283B6;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0283B6: cmp.b $10(a1), d0
pub fn i_0283b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283BA;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0283BA: bcs.w $283c4
pub fn i_0283ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283BE;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x283C4; return 10; }
    8
}
// ===== ClearXN_0283be  ($0283BE, 6 B, thunks/ccr_helpers.c)
/// $0283BE: andi.b #$ee, ccr
pub fn i_0283be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283C2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0283C2: rts
pub fn i_0283c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_0283c4  ($0283C4, 6 B, thunks/ccr_helpers.c)
/// $0283C4: ori.b #$11, ccr
pub fn i_0283c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283C8;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0283C8: rts
pub fn i_0283c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283CA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ClearFlags13Bits12  ($0283CA, 14 B, entity/clear_flags13_bits12_0283ca.s)
/// $0283CA: bclr.b #$1, $13(a6)
pub fn i_0283ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283D0;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0283D0: bclr.b #$2, $13(a6)
pub fn i_0283d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283D6;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0283D6: rts
pub fn i_0283d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283D8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_ProbeSlot4c_0283D8  ($0283D8, 20 B, entity/probe_slot4c_0283d8.s)
/// $0283D8: moveq #$ff, d7
pub fn i_0283d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283DA;
    cpu.ir = 0x7EFF;
    cpu.d[7] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0283DA: movea.l $4c(a6), a0
pub fn i_0283da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283DE;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0283DE: cmpi.l #$ffffffff, $4c(a6)
pub fn i_0283de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283E6;
    cpu.ir = 0xCAE;
    let ad2 = cpu.a[6].wrapping_add(0x4C);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0283E6: bne.w $283ec
pub fn i_0283e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283EA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x283EC; return 10; }
    8
}
/// $0283EA: rts
pub fn i_0283ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283EC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_RunList_0283ec  ($0283EC, 60 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0283EC: move.b (a0), d0
pub fn i_0283ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283EE;
    cpu.ir = 0x1010;
    let v2 = bus.read8(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    8
}
/// $0283EE: cmpi.b #$ff, d0
pub fn i_0283ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283F2;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xFF, Size::Byte, false);
    8
}
/// $0283F2: bne.w $283fe
pub fn i_0283f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283F6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x283FE; return 10; }
    8
}
/// $0283F6: suba.l #$50, a0
pub fn i_0283f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283FC;
    cpu.ir = 0x91FC;
    let v1: u32 = 0x50;
    cpu.a[0] = cpu.a[0].wrapping_sub(v1);
    8
}
/// $0283FC: rts
pub fn i_0283fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x283FE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0283FE: cmpi.b #$5, d0
pub fn i_0283fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28402;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $028402: bcs.w $28412
pub fn i_028402(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28406;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28412; return 10; }
    8
}
/// $028406: nop
pub fn i_028406(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28408;
    cpu.ir = 0x4E71;
    4
}
/// $028408: nop
pub fn i_028408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2840A;
    cpu.ir = 0x4E71;
    4
}
/// $02840A: cmpi.b #$5, d0
pub fn i_02840a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2840E;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x5, Size::Byte, false);
    8
}
/// $02840E: nop
pub fn i_02840e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28410;
    cpu.ir = 0x4E71;
    4
}
/// $028410: trap #$f
pub fn i_028410(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28412;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028412: ext.w d0
pub fn i_028412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28414;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $028414: lsl.w #$2, d0
pub fn i_028414(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28416;
    cpu.ir = 0xE548;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 1, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $028416: lea.l $28428(pc), a4
pub fn i_028416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2841A;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x28428;
    16
}
/// $02841A: movea.l (a4, d0.w), a5
pub fn i_02841a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2841E;
    cpu.ir = 0x2A74;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[5] = v3;
    18
}
/// $02841E: jsr (a5)
pub fn i_02841e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28420;
    cpu.ir = 0x4E95;
    cpu.push32(bus, 0x28420);
    cpu.jump_to(cpu.a[5]);
    16
}
/// $028420: adda.l #$50, a0
pub fn i_028420(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28426;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0x50;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $028426: bra.b $283ec
pub fn i_028426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28428;
    cpu.ir = 0x60C4;
    cpu.pc = 0x283EC;
    10
}
// ===== Hitbox_Op0_AllPlayers_02843c  ($02843C, 34 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02843C: lea.l $100440.l, a1
pub fn i_02843c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28442;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $028442: jsr $28576(pc)
pub fn i_028442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28446;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28446);
    cpu.pc = 0x28576;
    16
}
/// $028446: lea.l $1004e0.l, a1
pub fn i_028446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2844C;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $02844C: jsr $28576(pc)
pub fn i_02844c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28450;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28450);
    cpu.pc = 0x28576;
    16
}
/// $028450: lea.l $100580.l, a1
pub fn i_028450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28456;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $028456: jsr $28576(pc)
pub fn i_028456(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2845A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2845A);
    cpu.pc = 0x28576;
    16
}
/// $02845A: moveq #$ff, d7
pub fn i_02845a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2845C;
    cpu.ir = 0x7EFF;
    cpu.d[7] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02845C: rts
pub fn i_02845c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2845E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_Op1_P1AndSlug_02845e  ($02845E, 50 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02845E: lea.l $100440.l, a1
pub fn i_02845e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28464;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $028464: jsr $28576(pc)
pub fn i_028464(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28468;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28468);
    cpu.pc = 0x28576;
    16
}
/// $028468: btst.b #$1, $5a(a6)
pub fn i_028468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2846E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02846E: bne.w $28482
pub fn i_02846e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28472;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28482; return 10; }
    8
}
/// $028472: lea.l $100580.l, a1
pub fn i_028472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28478;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $028478: jsr $28576(pc)
pub fn i_028478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2847C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2847C);
    cpu.pc = 0x28576;
    16
}
/// $02847C: moveq #$ff, d7
pub fn i_02847c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2847E;
    cpu.ir = 0x7EFF;
    cpu.d[7] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02847E: bra.w $2848e
pub fn i_02847e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28482;
    cpu.ir = 0x6000;
    cpu.pc = 0x2848E;
    10
}
/// $028482: lea.l $100580.l, a1
pub fn i_028482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28488;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $028488: jsr $28576(pc)
pub fn i_028488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2848C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2848C);
    cpu.pc = 0x28576;
    16
}
/// $02848C: moveq #$0, d7
pub fn i_02848c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2848E;
    cpu.ir = 0x7E00;
    cpu.d[7] = 0x0;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02848E: rts
pub fn i_02848e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28490;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_Op2_P2AndSlug_028490  ($028490, 50 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028490: lea.l $1004e0.l, a1
pub fn i_028490(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28496;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $028496: jsr $28576(pc)
pub fn i_028496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2849A;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2849A);
    cpu.pc = 0x28576;
    16
}
/// $02849A: btst.b #$1, $5a(a6)
pub fn i_02849a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284A0;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0284A0: bne.w $284b4
pub fn i_0284a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284A4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x284B4; return 10; }
    8
}
/// $0284A4: lea.l $100580.l, a1
pub fn i_0284a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284AA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $0284AA: jsr $28576(pc)
pub fn i_0284aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x284AE);
    cpu.pc = 0x28576;
    16
}
/// $0284AE: moveq #$ff, d7
pub fn i_0284ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284B0;
    cpu.ir = 0x7EFF;
    cpu.d[7] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0284B0: bra.w $284c0
pub fn i_0284b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284B4;
    cpu.ir = 0x6000;
    cpu.pc = 0x284C0;
    10
}
/// $0284B4: lea.l $100580.l, a1
pub fn i_0284b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284BA;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $0284BA: jsr $28576(pc)
pub fn i_0284ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284BE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x284BE);
    cpu.pc = 0x28576;
    16
}
/// $0284BE: moveq #$1, d7
pub fn i_0284be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284C0;
    cpu.ir = 0x7E01;
    cpu.d[7] = 0x1;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0284C0: rts
pub fn i_0284c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284C2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_Op3_EnemyPool800_0284c2  ($0284C2, 86 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0284C2: movea.l #$ffffffff, a4
pub fn i_0284c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284C8;
    cpu.ir = 0x287C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $0284C8: jsr $28930(pc)
pub fn i_0284c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284CC;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x284CC);
    cpu.pc = 0x28930;
    16
}
/// $0284CC: bcs.w $28514
pub fn i_0284cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284D0;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28514; return 10; }
    8
}
/// $0284D0: btst.b #$1, $13(a6)
pub fn i_0284d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284D6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0284D6: bne.w $28514
pub fn i_0284d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284DA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28514; return 10; }
    8
}
/// $0284DA: lea.l $100800.l, a1
pub fn i_0284da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284E0;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100800;
    20
}
/// $0284E0: movea.l $8(a1), a2
pub fn i_0284e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284E4;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0284E4: move.b $100810.l, d0
pub fn i_0284e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284EA;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x100810) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $0284EA: cmp.b $10(a2), d0
pub fn i_0284ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284EE;
    cpu.ir = 0xB02A;
    let ad2 = cpu.a[2].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $0284EE: bcc.w $28514
pub fn i_0284ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284F2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28514; return 10; }
    8
}
/// $0284F2: movea.l a2, a1
pub fn i_0284f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284F4;
    cpu.ir = 0x224A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $0284F4: movea.l $48(a1), a2
pub fn i_0284f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284F8;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $0284F8: cmpa.l #$ffffffff, a2
pub fn i_0284f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x284FE;
    cpu.ir = 0xB5FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0284FE: beq.b $284e0
pub fn i_0284fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28500;
    cpu.ir = 0x67E0;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x284E0; return 10; }
    8
}
/// $028500: cmpa.l a6, a1
pub fn i_028500(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28502;
    cpu.ir = 0xB3CE;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028502: beq.b $284e0
pub fn i_028502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28504;
    cpu.ir = 0x67DC;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x284E0; return 10; }
    8
}
/// $028504: jsr $28594(pc)
pub fn i_028504(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28508;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28508);
    cpu.pc = 0x28594;
    16
}
/// $028508: btst.b #$1, $13(a6)
pub fn i_028508(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2850E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02850E: bne.w $28514
pub fn i_02850e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28512;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28514; return 10; }
    8
}
/// $028512: bra.b $284e0
pub fn i_028512(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28514;
    cpu.ir = 0x60CC;
    cpu.pc = 0x284E0;
    10
}
/// $028514: moveq #$ff, d7
pub fn i_028514(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28516;
    cpu.ir = 0x7EFF;
    cpu.d[7] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028516: rts
pub fn i_028516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28518;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_Op4_EnemyPool8A0_028518  ($028518, 86 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028518: movea.l #$ffffffff, a4
pub fn i_028518(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2851E;
    cpu.ir = 0x287C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $02851E: jsr $28930(pc)
pub fn i_02851e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28522;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28522);
    cpu.pc = 0x28930;
    16
}
/// $028522: bcs.w $2856a
pub fn i_028522(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28526;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2856A; return 10; }
    8
}
/// $028526: btst.b #$1, $13(a6)
pub fn i_028526(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2852C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02852C: bne.w $2856a
pub fn i_02852c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28530;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2856A; return 10; }
    8
}
/// $028530: lea.l $1008a0.l, a1
pub fn i_028530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28536;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1008A0;
    20
}
/// $028536: movea.l $8(a1), a2
pub fn i_028536(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2853A;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02853A: move.b $1008b0.l, d0
pub fn i_02853a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28540;
    cpu.ir = 0x1039;
    let v2 = bus.read8(0x1008B0) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v2 & 0xFF);
    16
}
/// $028540: cmp.b $10(a2), d0
pub fn i_028540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28544;
    cpu.ir = 0xB02A;
    let ad2 = cpu.a[2].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $028544: bcc.w $2856a
pub fn i_028544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28548;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2856A; return 10; }
    8
}
/// $028548: movea.l a2, a1
pub fn i_028548(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2854A;
    cpu.ir = 0x224A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[1] = v1;
    4
}
/// $02854A: movea.l $48(a1), a2
pub fn i_02854a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2854E;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $02854E: cmpa.l #$ffffffff, a2
pub fn i_02854e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28554;
    cpu.ir = 0xB5FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028554: beq.b $28536
pub fn i_028554(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28556;
    cpu.ir = 0x67E0;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28536; return 10; }
    8
}
/// $028556: cmpa.l a6, a1
pub fn i_028556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28558;
    cpu.ir = 0xB3CE;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028558: beq.b $28536
pub fn i_028558(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2855A;
    cpu.ir = 0x67DC;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28536; return 10; }
    8
}
/// $02855A: jsr $28594(pc)
pub fn i_02855a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2855E;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2855E);
    cpu.pc = 0x28594;
    16
}
/// $02855E: btst.b #$1, $13(a6)
pub fn i_02855e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28564;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028564: bne.w $2856a
pub fn i_028564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28568;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2856A; return 10; }
    8
}
/// $028568: bra.b $28536
pub fn i_028568(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2856A;
    cpu.ir = 0x60CC;
    cpu.pc = 0x28536;
    10
}
/// $02856A: moveq #$ff, d7
pub fn i_02856a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2856C;
    cpu.ir = 0x7EFF;
    cpu.d[7] = 0xFFFFFFFF;
    cpu.sr.set_nz(cpu.d[7], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02856C: rts
pub fn i_02856c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2856E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_TestEntityPair_028576  ($028576, 30 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028576: cmpa.l a6, a1
pub fn i_028576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28578;
    cpu.ir = 0xB3CE;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028578: beq.w $28686
pub fn i_028578(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2857C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28686; return 10; }
    8
}
/// $02857C: movea.l $48(a1), a2
pub fn i_02857c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28580;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $028580: cmpa.l #$ffffffff, a2
pub fn i_028580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28586;
    cpu.ir = 0xB5FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028586: beq.w $28686
pub fn i_028586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2858A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28686; return 10; }
    8
}
/// $02858A: btst.b #$1, $13(a6)
pub fn i_02858a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28590;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028590: bne.w $28686
pub fn i_028590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28594;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28686; return 10; }
    8
}
// ===== Hitbox_TestBoxes_028594  ($028594, 246 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028594: cmpi.l #$ffffffff, (a2)
pub fn i_028594(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2859A;
    cpu.ir = 0xC92;
    let v2 = bus.read32(cpu.a[2]);
    sub_with_flags(cpu, v2, 0xFFFFFFFF, Size::Long, false);
    16
}
/// $02859A: beq.w $28686
pub fn i_02859a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2859E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28686; return 10; }
    8
}
/// $02859E: btst.b #$3, $13(a1)
pub fn i_02859e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285A4;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0285A4: bne.w $28686
pub fn i_0285a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285A8;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28686; return 10; }
    8
}
/// $0285A8: lea.l $2856e(pc), a4
pub fn i_0285a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285AC;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x2856E;
    16
}
/// $0285AC: lea.l $27928c.l, a5
pub fn i_0285ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285B2;
    cpu.ir = 0x4BF9;
    cpu.a[5] = 0x27928C;
    20
}
/// $0285B2: cmpi.l #$ffffffff, $2(a2)
pub fn i_0285b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285BA;
    cpu.ir = 0xCAA;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    sub_with_flags(cpu, v3, 0xFFFFFFFF, Size::Long, false);
    20
}
/// $0285BA: beq.w $285c2
pub fn i_0285ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285BE;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x285C2; return 10; }
    8
}
/// $0285BE: movea.l $2(a2), a5
pub fn i_0285be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285C2;
    cpu.ir = 0x2A6A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[5] = v3;
    16
}
/// $0285C2: adda.l #$a, a2
pub fn i_0285c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285C8;
    cpu.ir = 0xD5FC;
    let v1: u32 = 0xA;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $0285C8: move.b $1(a0), d0
pub fn i_0285c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285CC;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0285CC: andi.w #$ff, d0
pub fn i_0285cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285D0;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0285D0: move.b (a5, d0.w), d1
pub fn i_0285d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285D4;
    cpu.ir = 0x1235;
    let ad2 = cpu.a[5].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0285D4: cmpi.b #$1, d1
pub fn i_0285d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285D8;
    cpu.ir = 0xC01;
    let v1 = cpu.d[1] & 0xFF;
    sub_with_flags(cpu, v1, 0x1, Size::Byte, false);
    8
}
/// $0285D8: beq.w $2860e
pub fn i_0285d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285DC;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2860E; return 10; }
    8
}
/// $0285DC: move.b $5(a0), d0
pub fn i_0285dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285E0;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x5);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0285E0: cmpi.b #$8, d0
pub fn i_0285e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285E4;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $0285E4: bcs.w $285f4
pub fn i_0285e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x285F4; return 10; }
    8
}
/// $0285E8: nop
pub fn i_0285e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285EA;
    cpu.ir = 0x4E71;
    4
}
/// $0285EA: nop
pub fn i_0285ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285EC;
    cpu.ir = 0x4E71;
    4
}
/// $0285EC: cmpi.b #$8, d0
pub fn i_0285ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285F0;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x8, Size::Byte, false);
    8
}
/// $0285F0: nop
pub fn i_0285f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285F2;
    cpu.ir = 0x4E71;
    4
}
/// $0285F2: trap #$f
pub fn i_0285f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285F4;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0285F4: andi.w #$ff, d0
pub fn i_0285f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285F8;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0285F8: movea.l (a4, d0.w), a3
pub fn i_0285f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x285FC;
    cpu.ir = 0x2674;
    let ad2 = cpu.a[4].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $0285FC: adda.l #$a, a0
pub fn i_0285fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28602;
    cpu.ir = 0xD1FC;
    let v1: u32 = 0xA;
    cpu.a[0] = cpu.a[0].wrapping_add(v1);
    8
}
/// $028602: jsr (a3)
pub fn i_028602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28604;
    cpu.ir = 0x4E93;
    cpu.push32(bus, 0x28604);
    cpu.jump_to(cpu.a[3]);
    16
}
/// $028604: bcs.w $28618
pub fn i_028604(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28608;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28618; return 10; }
    8
}
/// $028608: suba.l #$a, a0
pub fn i_028608(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2860E;
    cpu.ir = 0x91FC;
    let v1: u32 = 0xA;
    cpu.a[0] = cpu.a[0].wrapping_sub(v1);
    8
}
/// $02860E: adda.l #$46, a2
pub fn i_02860e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28614;
    cpu.ir = 0xD5FC;
    let v1: u32 = 0x46;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $028614: bra.w $28594
pub fn i_028614(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28618;
    cpu.ir = 0x6000;
    cpu.pc = 0x28594;
    10
}
/// $028618: suba.l #$a, a0
pub fn i_028618(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2861E;
    cpu.ir = 0x91FC;
    let v1: u32 = 0xA;
    cpu.a[0] = cpu.a[0].wrapping_sub(v1);
    8
}
/// $02861E: suba.l #$a, a2
pub fn i_02861e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28624;
    cpu.ir = 0x95FC;
    let v1: u32 = 0xA;
    cpu.a[2] = cpu.a[2].wrapping_sub(v1);
    8
}
/// $028624: tst.b $45(a1)
pub fn i_028624(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28628;
    cpu.ir = 0x4A29;
    let ad2 = cpu.a[1].wrapping_add(0x45);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $028628: bne.w $286d4
pub fn i_028628(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2862C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x286D4; return 10; }
    8
}
/// $02862C: move.b $1(a0), d0
pub fn i_02862c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28630;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028630: move.b d0, $58(a1)
pub fn i_028630(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28634;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x58);
    bus.write8(ad2, v1 as u8);
    12
}
/// $028634: move.l a6, $50(a1)
pub fn i_028634(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28638;
    cpu.ir = 0x234E;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let ad2 = cpu.a[1].wrapping_add(0x50);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028638: move.b $4(a0), d0
pub fn i_028638(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2863C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02863C: cmpi.b #$0, d0
pub fn i_02863c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28640;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $028640: beq.w $286d4
pub fn i_028640(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28644;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x286D4; return 10; }
    8
}
/// $028644: move.b $1(a0), d0
pub fn i_028644(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28648;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028648: cmpi.b #$22, d0
pub fn i_028648(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2864C;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x22, Size::Byte, false);
    8
}
/// $02864C: bls.w $2865c
pub fn i_02864c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28650;
    cpu.ir = 0x6300;
    if cpu.sr.evaluate(Condition::Ls) { cpu.pc = 0x2865C; return 10; }
    8
}
/// $028650: nop
pub fn i_028650(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28652;
    cpu.ir = 0x4E71;
    4
}
/// $028652: nop
pub fn i_028652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28654;
    cpu.ir = 0x4E71;
    4
}
/// $028654: cmpi.b #$22, d0
pub fn i_028654(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28658;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x22, Size::Byte, false);
    8
}
/// $028658: nop
pub fn i_028658(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2865A;
    cpu.ir = 0x4E71;
    4
}
/// $02865A: trap #$f
pub fn i_02865a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2865C;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02865C: cmpi.b #$0, d0
pub fn i_02865c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28660;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0x0, Size::Byte, false);
    8
}
/// $028660: beq.w $286e4
pub fn i_028660(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28664;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x286E4; return 10; }
    8
}
/// $028664: btst.b #$0, $6b(a1)
pub fn i_028664(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2866A;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02866A: beq.w $28676
pub fn i_02866a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2866E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28676; return 10; }
    8
}
/// $02866E: cmpi.b #$b, d0
pub fn i_02866e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28672;
    cpu.ir = 0xC00;
    let v1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, v1, 0xB, Size::Byte, false);
    8
}
/// $028672: beq.w $286e4
pub fn i_028672(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28676;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x286E4; return 10; }
    8
}
/// $028676: bset.b #$3, $13(a1)
pub fn i_028676(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2867C;
    cpu.ir = 0x8E9;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02867C: bset.b #$0, $5a(a1)
pub fn i_02867c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28682;
    cpu.ir = 0x8E9;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028682: bra.w $28690
pub fn i_028682(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28686;
    cpu.ir = 0x6000;
    cpu.pc = 0x28690;
    10
}
/// $028686: move.b #$ff, d1
pub fn i_028686(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2868A;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
// ===== ClearXN_02868a  ($02868A, 6 B, thunks/ccr_helpers.c)
/// $02868A: andi.b #$ee, ccr
pub fn i_02868a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2868E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02868E: rts
pub fn i_02868e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28690;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_ApplyDamage_028690  ($028690, 46 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028690: move.w $2(a0), d0
pub fn i_028690(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28694;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028694: jsr $286ec(pc)
pub fn i_028694(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28698;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28698);
    cpu.pc = 0x286EC;
    16
}
/// $028698: move.b $4(a0), d0
pub fn i_028698(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2869C;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02869C: move.b $1(a2), d1
pub fn i_02869c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286A0;
    cpu.ir = 0x122A;
    let ad2 = cpu.a[2].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0286A0: cmp.b d1, d0
pub fn i_0286a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286A2;
    cpu.ir = 0xB001;
    let v1 = cpu.d[1] & 0xFF;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v1, Size::Byte, false);
    4
}
/// $0286A2: bgt.w $286c4
pub fn i_0286a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286A6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x286C4; return 10; }
    8
}
/// $0286A6: bset.b #$2, $13(a6)
pub fn i_0286a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286AC;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0286AC: bset.b #$1, $13(a6)
pub fn i_0286ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286B2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0286B2: bset.b #$2, $5a(a6)
pub fn i_0286b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286B8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0286B8: bset.b #$1, $5a(a6)
pub fn i_0286b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286BE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
// ===== SetXN_0286be  ($0286BE, 6 B, thunks/ccr_helpers.c)
/// $0286BE: ori.b #$11, ccr
pub fn i_0286be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286C2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0286C2: rts
pub fn i_0286c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286C4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_MarkHitOnly_0286c4  ($0286C4, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0286C4: bset.b #$2, $13(a6)
pub fn i_0286c4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286CA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0286CA: bset.b #$2, $5a(a6)
pub fn i_0286ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286D0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0286D0: jmp $2868a(pc)
pub fn i_0286d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286D4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2868A;
    8
}
// ===== Hitbox_MarkBlocked_0286d4  ($0286D4, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0286D4: bset.b #$1, $13(a6)
pub fn i_0286d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286DA;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0286DA: bset.b #$1, $5a(a6)
pub fn i_0286da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286E0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0286E0: jmp $28686(pc)
pub fn i_0286e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286E4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28686;
    8
}
// ===== Hitbox_MarkHitNoDamage_0286e4  ($0286E4, 8 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0286E4: move.b #$ff, d1
pub fn i_0286e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286E8;
    cpu.ir = 0x123C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v1 & 0xFF);
    4
}
/// $0286E8: jmp $286a6(pc)
pub fn i_0286e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286EC;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x286A6;
    8
}
// ===== Hitbox_SubtractHP_0286ec  ($0286EC, 22 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0286EC: move.w $66(a1), d1
pub fn i_0286ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286F0;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0286F0: sub.w d0, d1
pub fn i_0286f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286F2;
    cpu.ir = 0x9240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0286F2: bgt.w $286fe
pub fn i_0286f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286F6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x286FE; return 10; }
    8
}
/// $0286F6: bset.b #$0, $13(a1)
pub fn i_0286f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286FC;
    cpu.ir = 0x8E9;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $0286FC: eor.w d1, d1
pub fn i_0286fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x286FE;
    cpu.ir = 0xB341;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0286FE: move.w d1, $66(a1)
pub fn i_0286fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28702;
    cpu.ir = 0x3341;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== JsrAbsThunk_028702  ($028702, 8 B, thunks/jsr_abs_thunks.c)
/// $028702: jsr $51914.l
pub fn i_028702(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28708;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28708);
    cpu.pc = 0x51914;
    16
}
/// $028708: rts
pub fn i_028708(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2870A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_SideOfImpact_02870a  ($02870A, 12 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02870A: btst.b #$3, $13(a6)
pub fn i_02870a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28710;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028710: bne.w $2871c
pub fn i_028710(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28714;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2871C; return 10; }
    8
}
/// $028714: clr.w d0
pub fn i_028714(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28716;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
// ===== ClearXN_028716  ($028716, 6 B, thunks/ccr_helpers.c)
/// $028716: andi.b #$ee, ccr
pub fn i_028716(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2871A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02871A: rts
pub fn i_02871a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2871C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_SideOfImpact_Calc_02871c  ($02871C, 54 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02871C: movea.l $48(a6), a2
pub fn i_02871c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28720;
    cpu.ir = 0x246E;
    let ad2 = cpu.a[6].wrapping_add(0x48);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $028720: move.w #$1, d0
pub fn i_028720(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28724;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028724: cmpa.l #$ffffffff, a2
pub fn i_028724(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2872A;
    cpu.ir = 0xB5FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $02872A: beq.w $28752
pub fn i_02872a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2872E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28752; return 10; }
    8
}
/// $02872E: adda.l #$a, a2
pub fn i_02872e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28734;
    cpu.ir = 0xD5FC;
    let v1: u32 = 0xA;
    cpu.a[2] = cpu.a[2].wrapping_add(v1);
    8
}
/// $028734: move.w $54(a6), d0
pub fn i_028734(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28738;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x54);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028738: move.w (a2), d2
pub fn i_028738(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2873A;
    cpu.ir = 0x3412;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $02873A: move.w $2(a2), d3
pub fn i_02873a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2873E;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02873E: add.w $22(a6), d2
pub fn i_02873e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28742;
    cpu.ir = 0xD46E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028742: add.w $22(a6), d3
pub fn i_028742(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28746;
    cpu.ir = 0xD66E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028746: add.w d2, d3
pub fn i_028746(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28748;
    cpu.ir = 0xD642;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028748: asr.w #$1, d3
pub fn i_028748(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2874A;
    cpu.ir = 0xE243;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02874A: sub.w d3, d0
pub fn i_02874a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2874C;
    cpu.ir = 0x9043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02874C: subx.w d0, d0
pub fn i_02874c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2874E;
    cpu.ir = 0x9140;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let h2 = cpu.d[0] & 0xFFFF;
    let r = subx_with_flags(cpu, h1, h2, x, Size::Word);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02874E: addi.w #$2, d0
pub fn i_02874e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28752;
    cpu.ir = 0x640;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = add_with_flags(cpu, v1, 0x2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
// ===== SetXN_028752  ($028752, 6 B, thunks/ccr_helpers.c)
/// $028752: ori.b #$11, ccr
pub fn i_028752(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28756;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028756: rts
pub fn i_028756(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28758;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_CheckBit0_028758  ($028758, 10 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028758: btst.b #$0, $13(a6)
pub fn i_028758(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2875E;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02875E: bne.w $28768
pub fn i_02875e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28762;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28768; return 10; }
    8
}
// ===== ClearXN_028762  ($028762, 6 B, thunks/ccr_helpers.c)
/// $028762: andi.b #$ee, ccr
pub fn i_028762(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28766;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028766: rts
pub fn i_028766(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28768;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_028768  ($028768, 6 B, thunks/ccr_helpers.c)
/// $028768: ori.b #$11, ccr
pub fn i_028768(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2876C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02876C: rts
pub fn i_02876c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2876E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapRect_02876e  ($02876E, 104 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02876E: move.w (a0), d0
pub fn i_02876e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28770;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028770: move.w $2(a0), d1
pub fn i_028770(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28774;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028774: btst.b #$0, $3a(a6)
pub fn i_028774(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2877A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02877A: beq.w $28784
pub fn i_02877a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2877E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28784; return 10; }
    8
}
/// $02877E: neg.w d0
pub fn i_02877e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28780;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028780: neg.w d1
pub fn i_028780(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28782;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028782: exg.l d0, d1
pub fn i_028782(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28784;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $028784: cmp.w d1, d0
pub fn i_028784(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28786;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028786: blt.w $28794
pub fn i_028786(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2878A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28794; return 10; }
    8
}
/// $02878A: nop
pub fn i_02878a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2878C;
    cpu.ir = 0x4E71;
    4
}
/// $02878C: nop
pub fn i_02878c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2878E;
    cpu.ir = 0x4E71;
    4
}
/// $02878E: cmp.w d1, d0
pub fn i_02878e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28790;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028790: nop
pub fn i_028790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28792;
    cpu.ir = 0x4E71;
    4
}
/// $028792: trap #$f
pub fn i_028792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28794;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028794: move.w (a2), d2
pub fn i_028794(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28796;
    cpu.ir = 0x3412;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028796: move.w $2(a2), d3
pub fn i_028796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2879A;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02879A: btst.b #$0, $3a(a1)
pub fn i_02879a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A0;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0287A0: beq.w $287aa
pub fn i_0287a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x287AA; return 10; }
    8
}
/// $0287A4: neg.w d2
pub fn i_0287a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A6;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0287A6: neg.w d3
pub fn i_0287a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287A8;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0287A8: exg.l d2, d3
pub fn i_0287a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287AA;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $0287AA: cmp.w d3, d2
pub fn i_0287aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287AC;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287AC: blt.w $287ba
pub fn i_0287ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x287BA; return 10; }
    8
}
/// $0287B0: nop
pub fn i_0287b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B2;
    cpu.ir = 0x4E71;
    4
}
/// $0287B2: nop
pub fn i_0287b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B4;
    cpu.ir = 0x4E71;
    4
}
/// $0287B4: cmp.w d3, d2
pub fn i_0287b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B6;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287B6: nop
pub fn i_0287b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287B8;
    cpu.ir = 0x4E71;
    4
}
/// $0287B8: trap #$f
pub fn i_0287b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287BA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0287BA: add.w $22(a6), d0
pub fn i_0287ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287BE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0287BE: add.w $22(a6), d1
pub fn i_0287be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287C2;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0287C2: add.w $22(a1), d2
pub fn i_0287c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287C6;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0287C6: add.w $22(a1), d3
pub fn i_0287c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287CA;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0287CA: cmp.w d1, d2
pub fn i_0287ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287CC;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287CC: bgt.w $287d6
pub fn i_0287cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287D0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x287D6; return 10; }
    8
}
/// $0287D0: cmp.w d3, d0
pub fn i_0287d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287D2;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287D2: ble.w $287dc
pub fn i_0287d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287D6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x287DC; return 10; }
    8
}
// ===== ClearXN_0287d6  ($0287D6, 6 B, thunks/ccr_helpers.c)
/// $0287D6: andi.b #$ee, ccr
pub fn i_0287d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287DA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0287DA: rts
pub fn i_0287da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287DC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapRect_Y_0287dc  ($0287DC, 130 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0287DC: cmp.w d2, d0
pub fn i_0287dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287DE;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287DE: bcc.w $287e4
pub fn i_0287de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287E2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x287E4; return 10; }
    8
}
/// $0287E2: move.w d2, d0
pub fn i_0287e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287E4;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0287E4: cmp.w d3, d1
pub fn i_0287e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287E6;
    cpu.ir = 0xB243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0287E6: bcs.w $287ec
pub fn i_0287e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287EA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x287EC; return 10; }
    8
}
/// $0287EA: move.w d3, d1
pub fn i_0287ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287EC;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0287EC: add.w d0, d1
pub fn i_0287ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287EE;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $0287EE: asr.w #$1, d1
pub fn i_0287ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287F0;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $0287F0: move.w d1, d7
pub fn i_0287f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287F2;
    cpu.ir = 0x3E01;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0287F2: move.w $4(a0), d0
pub fn i_0287f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287F6;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0287F6: move.w $6(a0), d1
pub fn i_0287f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x287FA;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0287FA: btst.b #$1, $3a(a6)
pub fn i_0287fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28800;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028800: beq.w $2880a
pub fn i_028800(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28804;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2880A; return 10; }
    8
}
/// $028804: neg.w d0
pub fn i_028804(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28806;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028806: neg.w d1
pub fn i_028806(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28808;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028808: exg.l d0, d1
pub fn i_028808(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2880A;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $02880A: cmp.w d1, d0
pub fn i_02880a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2880C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02880C: blt.w $2881a
pub fn i_02880c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28810;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2881A; return 10; }
    8
}
/// $028810: nop
pub fn i_028810(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28812;
    cpu.ir = 0x4E71;
    4
}
/// $028812: nop
pub fn i_028812(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28814;
    cpu.ir = 0x4E71;
    4
}
/// $028814: cmp.w d1, d0
pub fn i_028814(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28816;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028816: nop
pub fn i_028816(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28818;
    cpu.ir = 0x4E71;
    4
}
/// $028818: trap #$f
pub fn i_028818(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2881A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02881A: move.w $4(a2), d2
pub fn i_02881a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2881E;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02881E: move.w $6(a2), d3
pub fn i_02881e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28822;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028822: btst.b #$1, $3a(a1)
pub fn i_028822(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28828;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028828: beq.w $28832
pub fn i_028828(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2882C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28832; return 10; }
    8
}
/// $02882C: neg.w d2
pub fn i_02882c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2882E;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02882E: neg.w d3
pub fn i_02882e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28830;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028830: exg.l d2, d3
pub fn i_028830(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28832;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $028832: cmp.w d3, d2
pub fn i_028832(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28834;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028834: blt.w $28842
pub fn i_028834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28838;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28842; return 10; }
    8
}
/// $028838: nop
pub fn i_028838(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2883A;
    cpu.ir = 0x4E71;
    4
}
/// $02883A: nop
pub fn i_02883a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2883C;
    cpu.ir = 0x4E71;
    4
}
/// $02883C: cmp.w d3, d2
pub fn i_02883c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2883E;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02883E: nop
pub fn i_02883e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28840;
    cpu.ir = 0x4E71;
    4
}
/// $028840: trap #$f
pub fn i_028840(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28842;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028842: add.w $24(a6), d0
pub fn i_028842(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28846;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028846: add.w $24(a6), d1
pub fn i_028846(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2884A;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02884A: add.w $24(a1), d2
pub fn i_02884a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2884E;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02884E: add.w $24(a1), d3
pub fn i_02884e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28852;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028852: cmp.w d1, d2
pub fn i_028852(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28854;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028854: bgt.w $2885e
pub fn i_028854(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28858;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2885E; return 10; }
    8
}
/// $028858: cmp.w d3, d0
pub fn i_028858(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2885A;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02885A: ble.w $28864
pub fn i_02885a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2885E;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28864; return 10; }
    8
}
// ===== ClearXN_02885e  ($02885E, 6 B, thunks/ccr_helpers.c)
/// $02885E: andi.b #$ee, ccr
pub fn i_02885e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28862;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028862: rts
pub fn i_028862(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28864;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapRect_Store_028864  ($028864, 28 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028864: cmp.w d2, d0
pub fn i_028864(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28866;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028866: bcc.w $2886c
pub fn i_028866(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2886A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2886C; return 10; }
    8
}
/// $02886A: move.w d2, d0
pub fn i_02886a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2886C;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02886C: cmp.w d3, d1
pub fn i_02886c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2886E;
    cpu.ir = 0xB243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02886E: bcs.w $28874
pub fn i_02886e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28872;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28874; return 10; }
    8
}
/// $028872: move.w d3, d1
pub fn i_028872(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28874;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028874: add.w d0, d1
pub fn i_028874(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28876;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $028876: asr.w #$1, d1
pub fn i_028876(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28878;
    cpu.ir = 0xE241;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 1, Size::Word, 0, false);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $028878: move.w d1, $56(a1)
pub fn i_028878(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2887C;
    cpu.ir = 0x3341;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x56);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02887C: move.w d7, $54(a1)
pub fn i_02887c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28880;
    cpu.ir = 0x3347;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x54);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_028880  ($028880, 6 B, thunks/ccr_helpers.c)
/// $028880: ori.b #$11, ccr
pub fn i_028880(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28884;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028884: rts
pub fn i_028884(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28886;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapPoint_028886  ($028886, 64 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028886: move.w (a0), d0
pub fn i_028886(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28888;
    cpu.ir = 0x3010;
    let v2 = bus.read16(cpu.a[0]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028888: move.w (a2), d2
pub fn i_028888(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2888A;
    cpu.ir = 0x3412;
    let v2 = bus.read16(cpu.a[2]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $02888A: move.w $2(a2), d3
pub fn i_02888a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2888E;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02888E: btst.b #$0, $3a(a1)
pub fn i_02888e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28894;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028894: beq.w $2889e
pub fn i_028894(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28898;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2889E; return 10; }
    8
}
/// $028898: neg.w d2
pub fn i_028898(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2889A;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02889A: neg.w d3
pub fn i_02889a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2889C;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $02889C: exg.l d2, d3
pub fn i_02889c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2889E;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $02889E: cmp.w d3, d2
pub fn i_02889e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288A0;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0288A0: blt.w $288ae
pub fn i_0288a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288A4;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x288AE; return 10; }
    8
}
/// $0288A4: nop
pub fn i_0288a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288A6;
    cpu.ir = 0x4E71;
    4
}
/// $0288A6: nop
pub fn i_0288a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288A8;
    cpu.ir = 0x4E71;
    4
}
/// $0288A8: cmp.w d3, d2
pub fn i_0288a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288AA;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0288AA: nop
pub fn i_0288aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288AC;
    cpu.ir = 0x4E71;
    4
}
/// $0288AC: trap #$f
pub fn i_0288ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288AE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0288AE: add.w $22(a6), d0
pub fn i_0288ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288B2;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0288B2: add.w $22(a1), d2
pub fn i_0288b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288B6;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0288B6: add.w $22(a1), d3
pub fn i_0288b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288BA;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0288BA: cmp.w d0, d2
pub fn i_0288ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288BC;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0288BC: bgt.w $288c6
pub fn i_0288bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288C0;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x288C6; return 10; }
    8
}
/// $0288C0: cmp.w d3, d0
pub fn i_0288c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288C2;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0288C2: ble.w $288cc
pub fn i_0288c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288C6;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x288CC; return 10; }
    8
}
// ===== ClearXN_0288c6  ($0288C6, 6 B, thunks/ccr_helpers.c)
/// $0288C6: andi.b #$ee, ccr
pub fn i_0288c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288CA;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0288CA: rts
pub fn i_0288ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288CC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapPoint_Y_0288cc  ($0288CC, 70 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0288CC: move.w d0, d7
pub fn i_0288cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288CE;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0288CE: move.w $4(a0), d0
pub fn i_0288ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288D2;
    cpu.ir = 0x3028;
    let ad2 = cpu.a[0].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0288D2: move.w $4(a2), d2
pub fn i_0288d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288D6;
    cpu.ir = 0x342A;
    let ad2 = cpu.a[2].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0288D6: move.w $6(a2), d3
pub fn i_0288d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288DA;
    cpu.ir = 0x362A;
    let ad2 = cpu.a[2].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0288DA: btst.b #$1, $3a(a1)
pub fn i_0288da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288E0;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0288E0: beq.w $288ea
pub fn i_0288e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288E4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x288EA; return 10; }
    8
}
/// $0288E4: neg.w d2
pub fn i_0288e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288E6;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0288E6: neg.w d3
pub fn i_0288e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288E8;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0288E8: exg.l d2, d3
pub fn i_0288e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288EA;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $0288EA: cmp.w d3, d2
pub fn i_0288ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288EC;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0288EC: blt.w $288fa
pub fn i_0288ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288F0;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x288FA; return 10; }
    8
}
/// $0288F0: nop
pub fn i_0288f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288F2;
    cpu.ir = 0x4E71;
    4
}
/// $0288F2: nop
pub fn i_0288f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288F4;
    cpu.ir = 0x4E71;
    4
}
/// $0288F4: cmp.w d3, d2
pub fn i_0288f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288F6;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $0288F6: nop
pub fn i_0288f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288F8;
    cpu.ir = 0x4E71;
    4
}
/// $0288F8: trap #$f
pub fn i_0288f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288FA;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0288FA: add.w $24(a6), d0
pub fn i_0288fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x288FE;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $0288FE: add.w $24(a1), d2
pub fn i_0288fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28902;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028902: add.w $24(a1), d3
pub fn i_028902(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28906;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028906: cmp.w d0, d2
pub fn i_028906(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28908;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028908: bgt.w $28912
pub fn i_028908(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2890C;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28912; return 10; }
    8
}
/// $02890C: cmp.w d3, d0
pub fn i_02890c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2890E;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $02890E: ble.w $28918
pub fn i_02890e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28912;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28918; return 10; }
    8
}
// ===== ClearXN_028912  ($028912, 6 B, thunks/ccr_helpers.c)
/// $028912: andi.b #$ee, ccr
pub fn i_028912(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28916;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028916: rts
pub fn i_028916(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28918;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapPoint_Store_028918  ($028918, 8 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028918: move.w d7, $54(a1)
pub fn i_028918(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2891C;
    cpu.ir = 0x3347;
    let v1 = cpu.d[7] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x54);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02891C: move.w d0, $56(a1)
pub fn i_02891c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28920;
    cpu.ir = 0x3340;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x56);
    bus.write16(ad2, v1 as u16);
    12
}
// ===== SetXN_028920  ($028920, 6 B, thunks/ccr_helpers.c)
/// $028920: ori.b #$11, ccr
pub fn i_028920(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28924;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028924: rts
pub fn i_028924(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28926;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_ScreenMargin_028926  ($028926, 96 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028926: ori.b #$1, d0
pub fn i_028926(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2892A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02892A: ori.b #$1, d0
pub fn i_02892a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2892E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02892E: dc.w $ffff
pub fn i_02892e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28930;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
/// $028930: cmpi.b #$0, $6(a0)
pub fn i_028930(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28936;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x6);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x0, Size::Byte, false);
    16
}
/// $028936: bne.w $28986
pub fn i_028936(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2893A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28986; return 10; }
    8
}
/// $02893A: cmpa.l #$ffffffff, a4
pub fn i_02893a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28940;
    cpu.ir = 0xB9FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[4];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028940: bne.w $28948
pub fn i_028940(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28944;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28948; return 10; }
    8
}
/// $028944: lea.l $28926(pc), a4
pub fn i_028944(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28948;
    cpu.ir = 0x49FA;
    cpu.a[4] = 0x28926;
    16
}
/// $028948: move.w #$0, d0
pub fn i_028948(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2894C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02894C: move.w #$140, d1
pub fn i_02894c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28950;
    cpu.ir = 0x323C;
    let v1: u32 = 0x140;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028950: move.w #$100, d2
pub fn i_028950(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28954;
    cpu.ir = 0x343C;
    let v1: u32 = 0x100;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028954: move.w #$1f0, d3
pub fn i_028954(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28958;
    cpu.ir = 0x363C;
    let v1: u32 = 0x1F0;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028958: add.w (a4), d0
pub fn i_028958(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2895A;
    cpu.ir = 0xD054;
    let v2 = bus.read16(cpu.a[4]) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v2, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    8
}
/// $02895A: add.w $2(a4), d1
pub fn i_02895a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2895E;
    cpu.ir = 0xD26C;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $02895E: sub.w $6(a4), d2
pub fn i_02895e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28962;
    cpu.ir = 0x946C;
    let ad2 = cpu.a[4].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028962: sub.w $4(a4), d3
pub fn i_028962(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28966;
    cpu.ir = 0x966C;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = sub_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028966: cmp.w $22(a6), d0
pub fn i_028966(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2896A;
    cpu.ir = 0xB06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02896A: bgt.w $2898c
pub fn i_02896a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2896E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2898C; return 10; }
    8
}
/// $02896E: cmp.w $22(a6), d1
pub fn i_02896e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28972;
    cpu.ir = 0xB26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $028972: blt.w $2898c
pub fn i_028972(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28976;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2898C; return 10; }
    8
}
/// $028976: cmp.w $24(a6), d2
pub fn i_028976(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2897A;
    cpu.ir = 0xB46E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $02897A: bgt.w $2898c
pub fn i_02897a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2897E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x2898C; return 10; }
    8
}
/// $02897E: cmp.w $24(a6), d3
pub fn i_02897e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28982;
    cpu.ir = 0xB66E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    sub_with_flags(cpu, h1, v3, Size::Word, false);
    12
}
/// $028982: blt.w $2898c
pub fn i_028982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28986;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2898C; return 10; }
    8
}
// ===== ClearC_028986  ($028986, 6 B, thunks/ccr_helpers.c)
/// $028986: andi.b #$fe, ccr
pub fn i_028986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2898A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0x1E);
    20
}
/// $02898A: rts
pub fn i_02898a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2898C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetC_02898c  ($02898C, 6 B, thunks/ccr_helpers.c)
/// $02898C: ori.b #$1, ccr
pub fn i_02898c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28990;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x1);
    20
}
/// $028990: rts
pub fn i_028990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28992;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_028992  ($028992, 6 B, thunks/ccr_helpers.c)
/// $028992: andi.b #$ee, ccr
pub fn i_028992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28996;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028996: rts
pub fn i_028996(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28998;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Player_Dispatch3Slots_028998  ($028998, 94 B, player/dispatch_3slots_028998.s)
/// $028998: cmpi.w #$ffff, $60(a6)
pub fn i_028998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2899E;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $02899E: beq.w $289f0
pub fn i_02899e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289A2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x289F0; return 10; }
    8
}
/// $0289A2: movea.l $60(a6), a0
pub fn i_0289a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289A6;
    cpu.ir = 0x206E;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0289A6: cmpi.b #$80, (a0)
pub fn i_0289a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289AA;
    cpu.ir = 0xC10;
    let v2 = bus.read8(cpu.a[0]) as u32;
    sub_with_flags(cpu, v2, 0x80, Size::Byte, false);
    12
}
/// $0289AA: bne.w $289f0
pub fn i_0289aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289AE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x289F0; return 10; }
    8
}
/// $0289AE: lea.l $100580.l, a1
pub fn i_0289ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289B4;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $0289B4: jsr $28a96(pc)
pub fn i_0289b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289B8;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x289B8);
    cpu.pc = 0x28A96;
    16
}
/// $0289B8: lea.l $100440.l, a1
pub fn i_0289b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289BE;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100440;
    20
}
/// $0289BE: jsr $28a96(pc)
pub fn i_0289be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289C2;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x289C2);
    cpu.pc = 0x28A96;
    16
}
/// $0289C2: bcs.w $289d8
pub fn i_0289c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289C6;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x289D8; return 10; }
    8
}
/// $0289C6: lea.l $1004e0.l, a1
pub fn i_0289c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289CC;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $0289CC: jsr $28a96(pc)
pub fn i_0289cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289D0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x289D0);
    cpu.pc = 0x28A96;
    16
}
/// $0289D0: bcc.w $289f0
pub fn i_0289d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289D4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x289F0; return 10; }
    8
}
/// $0289D4: bra.w $289ea
pub fn i_0289d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289D8;
    cpu.ir = 0x6000;
    cpu.pc = 0x289EA;
    10
}
/// $0289D8: movem.l a1, -(a7)
pub fn i_0289d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289DC;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $0289DC: lea.l $1004e0.l, a1
pub fn i_0289dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289E2;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x1004E0;
    20
}
/// $0289E2: jsr $28a96(pc)
pub fn i_0289e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289E6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x289E6);
    cpu.pc = 0x28A96;
    16
}
/// $0289E6: movem.l (a7)+, a1
pub fn i_0289e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289EA;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $0289EA: ori.b #$11, ccr
pub fn i_0289ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289EE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $0289EE: rts
pub fn i_0289ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289F0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $0289F0: andi.b #$ee, ccr
pub fn i_0289f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289F4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $0289F4: rts
pub fn i_0289f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289F6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_DefaultShapes_0289f6  ($0289F6, 160 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0289F6: or.b d1, d0
pub fn i_0289f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289F8;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0289F8: ori.b #$0, d0
pub fn i_0289f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289FC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0289FC: dc.w $ffe0
pub fn i_0289fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x289FE;
    cpu.ir = 0xFFE0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Entity_HitboxCollide_028A96  ($028A96, 114 B, entity/hitbox_collide_028a96.s)
/// $028A96: cmpa.l a6, a1
pub fn i_028a96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28A98;
    cpu.ir = 0xB3CE;
    let v1 = cpu.a[6] & 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028A98: beq.w $28b08
pub fn i_028a98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28A9C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28B08; return 10; }
    8
}
/// $028A9C: movea.l $60(a1), a2
pub fn i_028a9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AA0;
    cpu.ir = 0x2469;
    let ad2 = cpu.a[1].wrapping_add(0x60);
    let v3 = bus.read32(ad2);
    cpu.a[2] = v3;
    16
}
/// $028AA0: cmpa.l #$ffffffff, a2
pub fn i_028aa0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AA6;
    cpu.ir = 0xB5FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[2];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028AA6: beq.w $28b08
pub fn i_028aa6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AAA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28B08; return 10; }
    8
}
/// $028AAA: cmpi.b #$80, (a2)
pub fn i_028aaa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AAE;
    cpu.ir = 0xC12;
    let v2 = bus.read8(cpu.a[2]) as u32;
    sub_with_flags(cpu, v2, 0x80, Size::Byte, false);
    12
}
/// $028AAE: bne.w $28b08
pub fn i_028aae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AB2;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28B08; return 10; }
    8
}
/// $028AB2: btst.b #$3, $69(a1)
pub fn i_028ab2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AB8;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x69);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028AB8: bne.w $28b08
pub fn i_028ab8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ABC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28B08; return 10; }
    8
}
/// $028ABC: movea.l a2, a3
pub fn i_028abc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ABE;
    cpu.ir = 0x264A;
    let v1 = cpu.a[2] & 0xFFFFFFFF;
    cpu.a[3] = v1;
    4
}
/// $028ABE: adda.l #$6, a3
pub fn i_028abe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AC4;
    cpu.ir = 0xD7FC;
    let v1: u32 = 0x6;
    cpu.a[3] = cpu.a[3].wrapping_add(v1);
    8
}
/// $028AC4: movea.l a0, a4
pub fn i_028ac4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AC6;
    cpu.ir = 0x2848;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    cpu.a[4] = v1;
    4
}
/// $028AC6: adda.l #$6, a4
pub fn i_028ac6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ACC;
    cpu.ir = 0xD9FC;
    let v1: u32 = 0x6;
    cpu.a[4] = cpu.a[4].wrapping_add(v1);
    8
}
/// $028ACC: jsr $28b14(pc)
pub fn i_028acc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AD0;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x28AD0);
    cpu.pc = 0x28B14;
    16
}
/// $028AD0: bcs.w $28ad8
pub fn i_028ad0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AD4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28AD8; return 10; }
    8
}
/// $028AD4: bra.w $28b08
pub fn i_028ad4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AD8;
    cpu.ir = 0x6000;
    cpu.pc = 0x28B08;
    10
}
/// $028AD8: addq.b #$1, d7
pub fn i_028ad8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ADA;
    cpu.ir = 0x5207;
    let v1 = cpu.d[7] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[7] = (cpu.d[7] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028ADA: btst.b #$6, $6b(a1)
pub fn i_028ada(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AE0;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028AE0: bne.w $28ae8
pub fn i_028ae0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AE4;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28AE8; return 10; }
    8
}
/// $028AE4: bset.b d7, $69(a6)
pub fn i_028ae4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AE8;
    cpu.ir = 0xFEE;
    let mk: u32 = 1u32 << ((cpu.d[7]) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028AE8: addq.b #$1, d6
pub fn i_028ae8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AEA;
    cpu.ir = 0x5206;
    let v1 = cpu.d[6] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[6] = (cpu.d[6] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028AEA: bset.b d6, $69(a1)
pub fn i_028aea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AEE;
    cpu.ir = 0xDE9;
    let mk: u32 = 1u32 << ((cpu.d[6]) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028AEE: bclr.b #$2, $69(a1)
pub fn i_028aee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AF4;
    cpu.ir = 0x8A9;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028AF4: cmpi.b #$2, $1(a0)
pub fn i_028af4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AFA;
    cpu.ir = 0xC28;
    let ad2 = cpu.a[0].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x2, Size::Byte, false);
    16
}
/// $028AFA: bne.w $28b04
pub fn i_028afa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28AFE;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28B04; return 10; }
    8
}
/// $028AFE: bset.b #$2, $69(a1)
pub fn i_028afe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B04;
    cpu.ir = 0x8E9;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[1].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028B04: bra.w $28b0e
pub fn i_028b04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B08;
    cpu.ir = 0x6000;
    cpu.pc = 0x28B0E;
    10
}
// ===== ClearXN_028b08  ($028B08, 6 B, thunks/ccr_helpers.c)
/// $028B08: andi.b #$ee, ccr
pub fn i_028b08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B0C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028B0C: rts
pub fn i_028b0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B0E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_028b0e  ($028B0E, 6 B, thunks/ccr_helpers.c)
/// $028B0E: ori.b #$11, ccr
pub fn i_028b0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B12;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028B12: rts
pub fn i_028b12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B14;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapTestXY_028B14  ($028B14, 268 B, entity/hitbox_collide_028a96.s)
/// $028B14: move.w (a4), d0
pub fn i_028b14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B16;
    cpu.ir = 0x3014;
    let v2 = bus.read16(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028B16: move.w $2(a4), d1
pub fn i_028b16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B1A;
    cpu.ir = 0x322C;
    let ad2 = cpu.a[4].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028B1A: btst.b #$0, $3a(a6)
pub fn i_028b1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B20;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028B20: beq.w $28b2a
pub fn i_028b20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B24;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28B2A; return 10; }
    8
}
/// $028B24: neg.w d0
pub fn i_028b24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B26;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028B26: neg.w d1
pub fn i_028b26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B28;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028B28: exg.l d0, d1
pub fn i_028b28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B2A;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $028B2A: cmp.w d1, d0
pub fn i_028b2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B2C;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B2C: blt.w $28b3a
pub fn i_028b2c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B30;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28B3A; return 10; }
    8
}
/// $028B30: nop
pub fn i_028b30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B32;
    cpu.ir = 0x4E71;
    4
}
/// $028B32: nop
pub fn i_028b32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B34;
    cpu.ir = 0x4E71;
    4
}
/// $028B34: cmp.w d1, d0
pub fn i_028b34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B36;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B36: nop
pub fn i_028b36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B38;
    cpu.ir = 0x4E71;
    4
}
/// $028B38: trap #$f
pub fn i_028b38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B3A;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028B3A: move.w (a3), d2
pub fn i_028b3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B3C;
    cpu.ir = 0x3413;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028B3C: move.w $2(a3), d3
pub fn i_028b3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B40;
    cpu.ir = 0x362B;
    let ad2 = cpu.a[3].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028B40: btst.b #$0, $3a(a1)
pub fn i_028b40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B46;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028B46: beq.w $28b50
pub fn i_028b46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B4A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28B50; return 10; }
    8
}
/// $028B4A: neg.w d2
pub fn i_028b4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B4C;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028B4C: neg.w d3
pub fn i_028b4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B4E;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028B4E: exg.l d2, d3
pub fn i_028b4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B50;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $028B50: cmp.w d3, d2
pub fn i_028b50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B52;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B52: blt.w $28b60
pub fn i_028b52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B56;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28B60; return 10; }
    8
}
/// $028B56: nop
pub fn i_028b56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B58;
    cpu.ir = 0x4E71;
    4
}
/// $028B58: nop
pub fn i_028b58(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B5A;
    cpu.ir = 0x4E71;
    4
}
/// $028B5A: cmp.w d3, d2
pub fn i_028b5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B5C;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B5C: nop
pub fn i_028b5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B5E;
    cpu.ir = 0x4E71;
    4
}
/// $028B5E: trap #$f
pub fn i_028b5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B60;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028B60: add.w $22(a6), d0
pub fn i_028b60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B64;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B64: add.w $22(a6), d1
pub fn i_028b64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B68;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B68: add.w $22(a1), d2
pub fn i_028b68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B6C;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B6C: add.w $22(a1), d3
pub fn i_028b6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B70;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B70: cmp.w d1, d2
pub fn i_028b70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B72;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B72: bgt.w $28b7c
pub fn i_028b72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B76;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28B7C; return 10; }
    8
}
/// $028B76: cmp.w d3, d0
pub fn i_028b76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B78;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B78: ble.w $28b82
pub fn i_028b78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B7C;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28B82; return 10; }
    8
}
/// $028B7C: andi.b #$ee, ccr
pub fn i_028b7c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B80;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028B80: rts
pub fn i_028b80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B82;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $028B82: cmp.w d2, d0
pub fn i_028b82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B84;
    cpu.ir = 0xB042;
    let v1 = cpu.d[2] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B84: bcc.w $28b8a
pub fn i_028b84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B88;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x28B8A; return 10; }
    8
}
/// $028B88: move.w d2, d0
pub fn i_028b88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B8A;
    cpu.ir = 0x3002;
    let v1 = cpu.d[2] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028B8A: cmp.w d3, d1
pub fn i_028b8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B8C;
    cpu.ir = 0xB243;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028B8C: bcs.w $28b92
pub fn i_028b8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B90;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28B92; return 10; }
    8
}
/// $028B90: move.w d3, d1
pub fn i_028b90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B92;
    cpu.ir = 0x3203;
    let v1 = cpu.d[3] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028B92: move.w $22(a1), d4
pub fn i_028b92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B96;
    cpu.ir = 0x3829;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028B96: move.w $22(a6), d5
pub fn i_028b96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B9A;
    cpu.ir = 0x3A2E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028B9A: add.w $2(a2), d4
pub fn i_028b9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28B9E;
    cpu.ir = 0xD86A;
    let ad2 = cpu.a[2].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[4] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[4] = (cpu.d[4] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028B9E: add.w $2(a0), d5
pub fn i_028b9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA2;
    cpu.ir = 0xDA68;
    let ad2 = cpu.a[0].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[5] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[5] = (cpu.d[5] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028BA2: cmp.w d4, d5
pub fn i_028ba2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA4;
    cpu.ir = 0xBA44;
    let v1 = cpu.d[4] & 0xFFFF;
    let h1 = cpu.d[5] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BA4: slt.b d7
pub fn i_028ba4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA6;
    cpu.ir = 0x5DC7;
    let c1: u32 = if cpu.sr.evaluate(Condition::Lt) { 0xFF } else { 0 };
    let v2 = cpu.d[7] & 0xFF;
    cpu.d[7] = (cpu.d[7] & !0xFF) | (c1 & 0xFF);
    4
}
/// $028BA6: sge.b d6
pub fn i_028ba6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BA8;
    cpu.ir = 0x5CC6;
    let c1: u32 = if cpu.sr.evaluate(Condition::Ge) { 0xFF } else { 0 };
    let v2 = cpu.d[6] & 0xFF;
    cpu.d[6] = (cpu.d[6] & !0xFF) | (c1 & 0xFF);
    4
}
/// $028BA8: move.w $4(a4), d0
pub fn i_028ba8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BAC;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028BAC: move.w $6(a4), d1
pub fn i_028bac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BB0;
    cpu.ir = 0x322C;
    let ad2 = cpu.a[4].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028BB0: btst.b #$1, $3a(a6)
pub fn i_028bb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BB6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028BB6: beq.w $28bc0
pub fn i_028bb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BBA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28BC0; return 10; }
    8
}
/// $028BBA: neg.w d0
pub fn i_028bba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BBC;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028BBC: neg.w d1
pub fn i_028bbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BBE;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028BBE: exg.l d0, d1
pub fn i_028bbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC0;
    cpu.ir = 0xC141;
    cpu.d.swap(0, 1);
    6
}
/// $028BC0: cmp.w d1, d0
pub fn i_028bc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC2;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BC2: blt.w $28bd0
pub fn i_028bc2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28BD0; return 10; }
    8
}
/// $028BC6: nop
pub fn i_028bc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BC8;
    cpu.ir = 0x4E71;
    4
}
/// $028BC8: nop
pub fn i_028bc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BCA;
    cpu.ir = 0x4E71;
    4
}
/// $028BCA: cmp.w d1, d0
pub fn i_028bca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BCC;
    cpu.ir = 0xB041;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BCC: nop
pub fn i_028bcc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BCE;
    cpu.ir = 0x4E71;
    4
}
/// $028BCE: trap #$f
pub fn i_028bce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BD0;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028BD0: move.w $4(a3), d2
pub fn i_028bd0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BD4;
    cpu.ir = 0x342B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028BD4: move.w $6(a3), d3
pub fn i_028bd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BD8;
    cpu.ir = 0x362B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028BD8: btst.b #$1, $3a(a1)
pub fn i_028bd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BDE;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028BDE: beq.w $28be8
pub fn i_028bde(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BE2;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28BE8; return 10; }
    8
}
/// $028BE2: neg.w d2
pub fn i_028be2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BE4;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028BE4: neg.w d3
pub fn i_028be4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BE6;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028BE6: exg.l d2, d3
pub fn i_028be6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BE8;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $028BE8: cmp.w d3, d2
pub fn i_028be8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BEA;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BEA: blt.w $28bf8
pub fn i_028bea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BEE;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28BF8; return 10; }
    8
}
/// $028BEE: nop
pub fn i_028bee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF0;
    cpu.ir = 0x4E71;
    4
}
/// $028BF0: nop
pub fn i_028bf0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF2;
    cpu.ir = 0x4E71;
    4
}
/// $028BF2: cmp.w d3, d2
pub fn i_028bf2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF4;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028BF4: nop
pub fn i_028bf4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF6;
    cpu.ir = 0x4E71;
    4
}
/// $028BF6: trap #$f
pub fn i_028bf6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BF8;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028BF8: add.w $24(a6), d0
pub fn i_028bf8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28BFC;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028BFC: add.w $24(a6), d1
pub fn i_028bfc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C00;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C00: add.w $24(a1), d2
pub fn i_028c00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C04;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C04: add.w $24(a1), d3
pub fn i_028c04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C08;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C08: cmp.w d1, d2
pub fn i_028c08(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C0A;
    cpu.ir = 0xB441;
    let v1 = cpu.d[1] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C0A: bgt.w $28c14
pub fn i_028c0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C0E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28C14; return 10; }
    8
}
/// $028C0E: cmp.w d3, d0
pub fn i_028c0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C10;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C10: ble.w $28c1a
pub fn i_028c10(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C14;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28C1A; return 10; }
    8
}
/// $028C14: andi.b #$ee, ccr
pub fn i_028c14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C18;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028C18: rts
pub fn i_028c18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C1A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $028C1A: ori.b #$11, ccr
pub fn i_028c1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C1E;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028C1E: rts
pub fn i_028c1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C20;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapPointA4_028c20  ($028C20, 64 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028C20: move.w (a4), d0
pub fn i_028c20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C22;
    cpu.ir = 0x3014;
    let v2 = bus.read16(cpu.a[4]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028C22: move.w (a3), d2
pub fn i_028c22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C24;
    cpu.ir = 0x3413;
    let v2 = bus.read16(cpu.a[3]) as u32;
    cpu.sr.set_nz(v2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v2 & 0xFFFF);
    8
}
/// $028C24: move.w $2(a3), d3
pub fn i_028c24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C28;
    cpu.ir = 0x362B;
    let ad2 = cpu.a[3].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028C28: btst.b #$0, $3a(a1)
pub fn i_028c28(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C2E;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028C2E: beq.w $28c38
pub fn i_028c2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C32;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28C38; return 10; }
    8
}
/// $028C32: neg.w d2
pub fn i_028c32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C34;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028C34: neg.w d3
pub fn i_028c34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C36;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028C36: exg.l d2, d3
pub fn i_028c36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C38;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $028C38: cmp.w d3, d2
pub fn i_028c38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C3A;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C3A: blt.w $28c48
pub fn i_028c3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C3E;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28C48; return 10; }
    8
}
/// $028C3E: nop
pub fn i_028c3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C40;
    cpu.ir = 0x4E71;
    4
}
/// $028C40: nop
pub fn i_028c40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C42;
    cpu.ir = 0x4E71;
    4
}
/// $028C42: cmp.w d3, d2
pub fn i_028c42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C44;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C44: nop
pub fn i_028c44(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C46;
    cpu.ir = 0x4E71;
    4
}
/// $028C46: trap #$f
pub fn i_028c46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C48;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028C48: add.w $22(a6), d0
pub fn i_028c48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C4C;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C4C: add.w $22(a1), d2
pub fn i_028c4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C50;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C50: add.w $22(a1), d3
pub fn i_028c50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C54;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C54: cmp.w d0, d2
pub fn i_028c54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C56;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C56: bgt.w $28c60
pub fn i_028c56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C5A;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28C60; return 10; }
    8
}
/// $028C5A: cmp.w d3, d0
pub fn i_028c5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C5C;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C5C: ble.w $28c66
pub fn i_028c5c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C60;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28C66; return 10; }
    8
}
// ===== ClearXN_028c60  ($028C60, 6 B, thunks/ccr_helpers.c)
/// $028C60: andi.b #$ee, ccr
pub fn i_028c60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C64;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028C64: rts
pub fn i_028c64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C66;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Hitbox_OverlapPointA4_Y_028c66  ($028C66, 70 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028C66: move.w d0, d7
pub fn i_028c66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C68;
    cpu.ir = 0x3E00;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[7] = (cpu.d[7] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028C68: move.w $4(a4), d0
pub fn i_028c68(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C6C;
    cpu.ir = 0x302C;
    let ad2 = cpu.a[4].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028C6C: move.w $4(a3), d2
pub fn i_028c6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C70;
    cpu.ir = 0x342B;
    let ad2 = cpu.a[3].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028C70: move.w $6(a3), d3
pub fn i_028c70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C74;
    cpu.ir = 0x362B;
    let ad2 = cpu.a[3].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028C74: btst.b #$1, $3a(a1)
pub fn i_028c74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C7A;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((1) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028C7A: beq.w $28c84
pub fn i_028c7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28C84; return 10; }
    8
}
/// $028C7E: neg.w d2
pub fn i_028c7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C80;
    cpu.ir = 0x4442;
    let v1 = cpu.d[2] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028C80: neg.w d3
pub fn i_028c80(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C82;
    cpu.ir = 0x4443;
    let v1 = cpu.d[3] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028C82: exg.l d2, d3
pub fn i_028c82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C84;
    cpu.ir = 0xC543;
    cpu.d.swap(2, 3);
    6
}
/// $028C84: cmp.w d3, d2
pub fn i_028c84(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C86;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C86: blt.w $28c94
pub fn i_028c86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C8A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28C94; return 10; }
    8
}
/// $028C8A: nop
pub fn i_028c8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C8C;
    cpu.ir = 0x4E71;
    4
}
/// $028C8C: nop
pub fn i_028c8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C8E;
    cpu.ir = 0x4E71;
    4
}
/// $028C8E: cmp.w d3, d2
pub fn i_028c8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C90;
    cpu.ir = 0xB443;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028C90: nop
pub fn i_028c90(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C92;
    cpu.ir = 0x4E71;
    4
}
/// $028C92: trap #$f
pub fn i_028c92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C94;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028C94: add.w $24(a6), d0
pub fn i_028c94(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C98;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C98: add.w $24(a1), d2
pub fn i_028c98(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28C9C;
    cpu.ir = 0xD469;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[2] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028C9C: add.w $24(a1), d3
pub fn i_028c9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CA0;
    cpu.ir = 0xD669;
    let ad2 = cpu.a[1].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[3] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[3] = (cpu.d[3] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028CA0: cmp.w d0, d2
pub fn i_028ca0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CA2;
    cpu.ir = 0xB440;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[2] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028CA2: bgt.w $28cac
pub fn i_028ca2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CA6;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28CAC; return 10; }
    8
}
/// $028CA6: cmp.w d3, d0
pub fn i_028ca6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CA8;
    cpu.ir = 0xB043;
    let v1 = cpu.d[3] & 0xFFFF;
    let h1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, h1, v1, Size::Word, false);
    4
}
/// $028CA8: ble.w $28cb2
pub fn i_028ca8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CAC;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x28CB2; return 10; }
    8
}
// ===== ClearXN_028cac  ($028CAC, 6 B, thunks/ccr_helpers.c)
/// $028CAC: andi.b #$ee, ccr
pub fn i_028cac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CB0;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028CB0: rts
pub fn i_028cb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CB2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_028cb2  ($028CB2, 6 B, thunks/ccr_helpers.c)
/// $028CB2: ori.b #$11, ccr
pub fn i_028cb2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CB6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028CB6: rts
pub fn i_028cb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CB8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_028cb8  ($028CB8, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028CB8: movea.l $8(a6), a1
pub fn i_028cb8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CBC;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028CBC: move.b $10(a6), d0
pub fn i_028cbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CC0;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028CC0: cmp.b $10(a1), d0
pub fn i_028cc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CC4;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $028CC4: bcs.w $28cce
pub fn i_028cc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CC8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28CCE; return 10; }
    8
}
// ===== ClearXN_028cc8  ($028CC8, 6 B, thunks/ccr_helpers.c)
/// $028CC8: andi.b #$ee, ccr
pub fn i_028cc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CCC;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $028CCC: rts
pub fn i_028ccc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CCE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_028cce  ($028CCE, 6 B, thunks/ccr_helpers.c)
/// $028CCE: ori.b #$11, ccr
pub fn i_028cce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CD2;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $028CD2: rts
pub fn i_028cd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CD4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== EntitySetSpriteMap  ($028CD4, 28 B, entity/entity_setters.c)
/// $028CD4: move.l a0, $3c(a6)
pub fn i_028cd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CD8;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028CD8: move.l a0, $40(a6)
pub fn i_028cd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CDC;
    cpu.ir = 0x2D48;
    let v1 = cpu.a[0] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028CDC: clr.b $3b(a6)
pub fn i_028cdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CE0;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x3B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $028CE0: clr.b $46(a6)
pub fn i_028ce0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CE4;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $028CE4: clr.b $47(a6)
pub fn i_028ce4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CE8;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x47);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $028CE8: bset.b #$5, $69(a6)
pub fn i_028ce8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CEE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x69);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028CEE: rts
pub fn i_028cee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28CF0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Entity_HasLinkedSlots  ($028D70, 30 B, entity/has_linked_028d70.s)
/// $028D70: movea.l $40(a6), a1
pub fn i_028d70(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D74;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028D74: cmpa.l #$ffffffff, a1
pub fn i_028d74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7A;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028D7A: beq.w $28d8c
pub fn i_028d7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D7E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28D8C; return 10; }
    8
}
/// $028D7E: movea.l $3c(a6), a1
pub fn i_028d7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D82;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028D82: cmpa.l #$ffffffff, a1
pub fn i_028d82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D88;
    cpu.ir = 0xB3FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $028D88: bne.w $28d8e
pub fn i_028d88(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28D8E; return 10; }
    8
}
/// $028D8C: rts
pub fn i_028d8c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D8E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Script_DispatchOpcode  ($028D8E, 70 B, entity/script_dispatch_028d8e.s)
/// $028D8E: movea.l $3c(a6), a1
pub fn i_028d8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D92;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028D92: cmpa.l $40(a6), a1
pub fn i_028d92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D96;
    cpu.ir = 0xB3EE;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    let h1 = cpu.a[1];
    sub_with_flags(cpu, h1, v3, Size::Long, false);
    18
}
/// $028D96: beq.w $28da8
pub fn i_028d96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28DA8; return 10; }
    8
}
/// $028D9A: move.b $3b(a6), d1
pub fn i_028d9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28D9E;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028D9E: addq.b #$1, d1
pub fn i_028d9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA0;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028DA0: beq.w $28da8
pub fn i_028da0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA4;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28DA8; return 10; }
    8
}
/// $028DA4: move.b d1, $3b(a6)
pub fn i_028da4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DA8;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    bus.write8(ad2, v1 as u8);
    12
}
/// $028DA8: moveq #$0, d2
pub fn i_028da8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAA;
    cpu.ir = 0x7400;
    cpu.d[2] = 0x0;
    cpu.sr.set_nz(cpu.d[2], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028DAA: move.b (a1), d2
pub fn i_028daa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DAC;
    cpu.ir = 0x1411;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v2 & 0xFF);
    8
}
/// $028DAC: cmpi.b #$20, d2
pub fn i_028dac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB0;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $028DB0: bcs.w $28dc0
pub fn i_028db0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB4;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x28DC0; return 10; }
    8
}
/// $028DB4: nop
pub fn i_028db4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB6;
    cpu.ir = 0x4E71;
    4
}
/// $028DB6: nop
pub fn i_028db6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DB8;
    cpu.ir = 0x4E71;
    4
}
/// $028DB8: cmpi.b #$20, d2
pub fn i_028db8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DBC;
    cpu.ir = 0xC02;
    let v1 = cpu.d[2] & 0xFF;
    sub_with_flags(cpu, v1, 0x20, Size::Byte, false);
    8
}
/// $028DBC: nop
pub fn i_028dbc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DBE;
    cpu.ir = 0x4E71;
    4
}
/// $028DBE: trap #$f
pub fn i_028dbe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DC0;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $028DC0: andi.l #$ff, d2
pub fn i_028dc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DC6;
    cpu.ir = 0x282;
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFFFFFF;
    cpu.sr.set_nz(r2, Size::Long); cpu.sr.0 &= !0x3;
    cpu.d[2] = r2;
    8
}
/// $028DC6: lsl.l #$2, d2
pub fn i_028dc6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DC8;
    cpu.ir = 0xE58A;
    let h1 = cpu.d[2] & 0xFFFFFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Long, 1, true);
    cpu.d[2] = r;
    10
}
/// $028DC8: movea.l #$28cf0, a2
pub fn i_028dc8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DCE;
    cpu.ir = 0x247C;
    let v1: u32 = 0x28CF0;
    cpu.a[2] = v1;
    4
}
/// $028DCE: movea.l (a2, d2.w), a3
pub fn i_028dce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD2;
    cpu.ir = 0x2672;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[2] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[3] = v3;
    18
}
/// $028DD2: jmp (a3)
pub fn i_028dd2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD4;
    cpu.ir = 0x4ED3;
    cpu.jump_to(cpu.a[3]);
    8
}
// ===== ScriptOp00_Wait_028dd4  ($028DD4, 30 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028DD4: move.b $46(a6), d2
pub fn i_028dd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DD8;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028DD8: bne.w $28de4
pub fn i_028dd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28DE4; return 10; }
    8
}
/// $028DDC: move.b $1(a1), d2
pub fn i_028ddc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DE0;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028DE0: move.b d2, $46(a6)
pub fn i_028de0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DE4;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write8(ad2, v1 as u8);
    12
}
/// $028DE4: adda.l #$2, a1
pub fn i_028de4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DEA;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $028DEA: move.l a1, $3c(a6)
pub fn i_028dea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DEE;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028DEE: jmp $28da8(pc)
pub fn i_028dee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DF2;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp01_Jump_028df2  ($028DF2, 12 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028DF2: movea.l $2(a1), a1
pub fn i_028df2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DF6;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $028DF6: move.l a1, $3c(a6)
pub fn i_028df6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DFA;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028DFA: jmp $28da8(pc)
pub fn i_028dfa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28DFE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp02_Sprite_028dfe  ($028DFE, 546 B, entity/collmap_hitbox_script_ops_027400.s)
/// $028DFE: move.l a1, $40(a6)
pub fn i_028dfe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E02;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $028E02: move.b $1(a1), d1
pub fn i_028e02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E06;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028E06: move.b $3a(a6), d2
pub fn i_028e06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E0A;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028E0A: eor.b d1, d2
pub fn i_028e0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E0C;
    cpu.ir = 0xB302;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 ^ cpu.d[1]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028E0C: btst.b #$3, $1(a1)
pub fn i_028e0c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E12;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028E12: beq.w $28e22
pub fn i_028e12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E16;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E22; return 10; }
    8
}
/// $028E16: move.w $22(a6), d0
pub fn i_028e16(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E1A;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028E1A: move.w $24(a6), d1
pub fn i_028e1a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E1E;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028E1E: bra.w $28e46
pub fn i_028e1e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E22;
    cpu.ir = 0x6000;
    cpu.pc = 0x28E46;
    10
}
/// $028E22: move.w $8(a1), d0
pub fn i_028e22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E26;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x8);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028E26: btst.b #$0, d2
pub fn i_028e26(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E2A;
    cpu.ir = 0x802;
    let mk: u32 = 1u32 << ((0) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $028E2A: beq.w $28e30
pub fn i_028e2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E30; return 10; }
    8
}
/// $028E2E: neg.w d0
pub fn i_028e2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E30;
    cpu.ir = 0x4440;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028E30: add.w $22(a6), d0
pub fn i_028e30(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E34;
    cpu.ir = 0xD06E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028E34: move.w $a(a1), d1
pub fn i_028e34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E38;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028E38: btst.b #$1, d2
pub fn i_028e38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E3C;
    cpu.ir = 0x802;
    let mk: u32 = 1u32 << ((1) % 32);
    let v1 = cpu.d[2] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $028E3C: beq.w $28e42
pub fn i_028e3c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E40;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E42; return 10; }
    8
}
/// $028E40: neg.w d1
pub fn i_028e40(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E42;
    cpu.ir = 0x4441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = 0u32.wrapping_sub(v1) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.set(StatusRegister::V, v1 == 0x8000); cpu.sr.set(StatusRegister::C, v1 != 0); cpu.sr.set(StatusRegister::X, v1 != 0);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $028E42: add.w $24(a6), d1
pub fn i_028e42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E46;
    cpu.ir = 0xD26E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v3, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    12
}
/// $028E46: swap d0
pub fn i_028e46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E48;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028E48: subi.w #$1, d1
pub fn i_028e48(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E4C;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $028E4C: move.w d1, d0
pub fn i_028e4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E4E;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028E4E: move.b $59(a6), d1
pub fn i_028e4e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E52;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x59);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028E52: andi.b #$1, d1
pub fn i_028e52(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E56;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028E56: beq.w $28e9c
pub fn i_028e56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E5A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E9C; return 10; }
    8
}
/// $028E5A: btst.b #$6, $6b(a6)
pub fn i_028e5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E60;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028E60: beq.w $28e6e
pub fn i_028e60(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E64;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E6E; return 10; }
    8
}
/// $028E64: bset.b #$7, $5a(a6)
pub fn i_028e64(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E6A;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028E6A: bra.w $28e9c
pub fn i_028e6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E6E;
    cpu.ir = 0x6000;
    cpu.pc = 0x28E9C;
    10
}
/// $028E6E: btst.b #$0, $6b(a6)
pub fn i_028e6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E74;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028E74: beq.w $28e82
pub fn i_028e74(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E78;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28E82; return 10; }
    8
}
/// $028E78: bset.b #$7, $5a(a6)
pub fn i_028e78(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E7E;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028E7E: bra.w $28e9c
pub fn i_028e7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E82;
    cpu.ir = 0x6000;
    cpu.pc = 0x28E9C;
    10
}
/// $028E82: btst.b #$1, $100001.l
pub fn i_028e82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E8A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((1) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $028E8A: bne.w $28e96
pub fn i_028e8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E8E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28E96; return 10; }
    8
}
/// $028E8E: jmp $28fca(pc)
pub fn i_028e8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E92;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28FCA;
    8
}
/// $028E96: bset.b #$7, $5a(a6)
pub fn i_028e96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28E9C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028E9C: btst.b #$5, $12(a6)
pub fn i_028e9c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EA2;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028EA2: beq.w $28ec6
pub fn i_028ea2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EA6;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28EC6; return 10; }
    8
}
/// $028EA6: bclr.b #$5, $12(a6)
pub fn i_028ea6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EAC;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028EAC: move.b $12(a6), d1
pub fn i_028eac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EB0;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028EB0: andi.b #$10, d1
pub fn i_028eb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EB4;
    cpu.ir = 0x201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028EB4: ror.b #$4, d1
pub fn i_028eb4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EB6;
    cpu.ir = 0xE819;
    let h1 = cpu.d[1] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 3, false);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    14
}
/// $028EB6: move.b $1(a1), d2
pub fn i_028eb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EBA;
    cpu.ir = 0x1429;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028EBA: andi.b #$10, d2
pub fn i_028eba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EBE;
    cpu.ir = 0x202;
    let v1 = cpu.d[2] & 0xFF;
    let r2 = (v1 & 0x10) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028EBE: ror.b #$4, d2
pub fn i_028ebe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EC0;
    cpu.ir = 0xE81A;
    let h1 = cpu.d[2] & 0xFF;
    let r = do_shift_rotate(cpu, h1, 4, Size::Byte, 3, false);
    cpu.d[2] = (cpu.d[2] & !0xFF) | (r & 0xFF);
    14
}
/// $028EC0: eor.b d2, d1
pub fn i_028ec0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EC2;
    cpu.ir = 0xB501;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 ^ cpu.d[2]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028EC2: bne.w $28fca
pub fn i_028ec2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EC6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FCA; return 10; }
    8
}
/// $028EC6: btst.b #$2, $1(a1)
pub fn i_028ec6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ECC;
    cpu.ir = 0x829;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028ECC: beq.w $28eda
pub fn i_028ecc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ED0;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28EDA; return 10; }
    8
}
/// $028ED0: btst.b #$3, $12(a6)
pub fn i_028ed0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28ED6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028ED6: bne.w $28fca
pub fn i_028ed6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EDA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FCA; return 10; }
    8
}
/// $028EDA: move.w $38(a6), d1
pub fn i_028eda(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EDE;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028EDE: cmpi.w #$ffff, $6(a1)
pub fn i_028ede(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EE4;
    cpu.ir = 0xC69;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $028EE4: beq.w $28eec
pub fn i_028ee4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EE8;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28EEC; return 10; }
    8
}
/// $028EE8: move.w $6(a1), d1
pub fn i_028ee8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EEC;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028EEC: move.w $14(a6), d2
pub fn i_028eec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EF0;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028EF0: btst.b #$7, $5a(a6)
pub fn i_028ef0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EF6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028EF6: bne.w $28f04
pub fn i_028ef6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28EFA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28F04; return 10; }
    8
}
/// $028EFA: btst.b #$0, $5a(a6)
pub fn i_028efa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F00;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x5A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028F00: beq.w $28f22
pub fn i_028f00(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F04;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28F22; return 10; }
    8
}
/// $028F04: btst.b #$0, $13(a6)
pub fn i_028f04(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F0A;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x13);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028F0A: bne.w $28f22
pub fn i_028f0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F0E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28F22; return 10; }
    8
}
/// $028F0E: cmpi.w #$ffff, $1c(a6)
pub fn i_028f0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F14;
    cpu.ir = 0xC6E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFFFF, Size::Word, false);
    16
}
/// $028F14: beq.w $28f22
pub fn i_028f14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F18;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28F22; return 10; }
    8
}
/// $028F18: move.w $1c(a6), d2
pub fn i_028f18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F1C;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $028F1C: bset.b #$7, $5a(a6)
pub fn i_028f1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F22;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((7) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5A);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $028F22: btst.b #$4, $100001.l
pub fn i_028f22(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F2A;
    cpu.ir = 0x839;
    let mk: u32 = 1u32 << ((4) % 8);
    let v2 = bus.read8(0x100001) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    20
}
/// $028F2A: beq.w $28f3e
pub fn i_028f2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F2E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28F3E; return 10; }
    8
}
/// $028F2E: btst.b #$0, $6b(a6)
pub fn i_028f2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F34;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x6B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028F34: beq.w $28f3e
pub fn i_028f34(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F38;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x28F3E; return 10; }
    8
}
/// $028F38: jsr $2f84a.l
pub fn i_028f38(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F3E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28F3E);
    cpu.pc = 0x2F84A;
    16
}
/// $028F3E: move.b $3a(a6), d5
pub fn i_028f3e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F42;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F42: move.b $12(a6), d4
pub fn i_028f42(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F46;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F46: andi.b #$4, d4
pub fn i_028f46(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F4A;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028F4A: or.b d4, d5
pub fn i_028f4a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F4C;
    cpu.ir = 0x8A04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $028F4C: move.b $1(a1), d4
pub fn i_028f4c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F50;
    cpu.ir = 0x1829;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F50: andi.b #$3, d4
pub fn i_028f50(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F54;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x3) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $028F54: eor.b d4, d5
pub fn i_028f54(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F56;
    cpu.ir = 0xB905;
    let v1 = cpu.d[5] & 0xFF;
    let r2 = (v1 ^ cpu.d[4]) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r2 & 0xFF);
    4
}
/// $028F56: move.b $33(a6), d3
pub fn i_028f56(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F5A;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F5A: move.b $32(a6), d4
pub fn i_028f5a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F5E;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028F5E: movea.l $2(a1), a0
pub fn i_028f5e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F62;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $028F62: movem.l d0, -(a7)
pub fn i_028f62(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F66;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[0]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $028F66: movem.l d1, -(a7)
pub fn i_028f66(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F6A;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.d[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $028F6A: move.w d0, d1
pub fn i_028f6a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F6C;
    cpu.ir = 0x3200;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $028F6C: swap d0
pub fn i_028f6c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F6E;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $028F6E: cmpi.w #$ff00, d0
pub fn i_028f6e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F72;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFF00, Size::Word, false);
    8
}
/// $028F72: bgt.w $28f7a
pub fn i_028f72(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F76;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28F7A; return 10; }
    8
}
/// $028F76: bra.w $28fa2
pub fn i_028f76(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F7A;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA2;
    10
}
/// $028F7A: cmpi.w #$200, d0
pub fn i_028f7a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F7E;
    cpu.ir = 0xC40;
    let v1 = cpu.d[0] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x200, Size::Word, false);
    8
}
/// $028F7E: blt.w $28f86
pub fn i_028f7e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F82;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28F86; return 10; }
    8
}
/// $028F82: bra.w $28fa2
pub fn i_028f82(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F86;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA2;
    10
}
/// $028F86: cmpi.w #$0, d1
pub fn i_028f86(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F8A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x0, Size::Word, false);
    8
}
/// $028F8A: bgt.w $28f92
pub fn i_028f8a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F8E;
    cpu.ir = 0x6E00;
    if cpu.sr.evaluate(Condition::Gt) { cpu.pc = 0x28F92; return 10; }
    8
}
/// $028F8E: bra.w $28fa2
pub fn i_028f8e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F92;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA2;
    10
}
/// $028F92: cmpi.w #$300, d1
pub fn i_028f92(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F96;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x300, Size::Word, false);
    8
}
/// $028F96: blt.w $28f9e
pub fn i_028f96(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F9A;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x28F9E; return 10; }
    8
}
/// $028F9A: bra.w $28fa2
pub fn i_028f9a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28F9E;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA2;
    10
}
/// $028F9E: bra.w $28fa8
pub fn i_028f9e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FA2;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FA8;
    10
}
/// $028FA2: movea.l #$ffffffff, a0
pub fn i_028fa2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FA8;
    cpu.ir = 0x207C;
    let v1: u32 = 0xFFFFFFFF;
    cpu.a[0] = v1;
    4
}
/// $028FA8: movem.l (a7)+, d1
pub fn i_028fa8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FAC;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $028FAC: movem.l (a7)+, d0
pub fn i_028fac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FB0;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.d[0] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $028FB0: btst.b #$6, $12(a6)
pub fn i_028fb0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FB6;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $028FB6: bne.w $28fc4
pub fn i_028fb6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FBA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FC4; return 10; }
    8
}
/// $028FBA: jsr $5a9e2.l
pub fn i_028fba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FC0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28FC0);
    cpu.pc = 0x5A9E2;
    16
}
/// $028FC0: bra.w $28fca
pub fn i_028fc0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FC4;
    cpu.ir = 0x6000;
    cpu.pc = 0x28FCA;
    10
}
/// $028FC4: jsr $5a9d6.l
pub fn i_028fc4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FCA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x28FCA);
    cpu.pc = 0x5A9D6;
    16
}
/// $028FCA: move.b $1(a1), d0
pub fn i_028fca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FCE;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $028FCE: adda.l #$8, a1
pub fn i_028fce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FD4;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x8;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $028FD4: btst.b #$3, d0
pub fn i_028fd4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FD8;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $028FD8: bne.w $28fe2
pub fn i_028fd8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FDC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FE2; return 10; }
    8
}
/// $028FDC: adda.l #$4, a1
pub fn i_028fdc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FE2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $028FE2: cmpi.b #$18, (a1)
pub fn i_028fe2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FE6;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x18, Size::Byte, false);
    12
}
/// $028FE6: bne.w $28fee
pub fn i_028fe6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FEA;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FEE; return 10; }
    8
}
/// $028FEA: jmp $2934c(pc)
pub fn i_028fea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FEE;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2934C;
    8
}
/// $028FEE: cmpi.b #$2, (a1)
pub fn i_028fee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FF2;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $028FF2: bne.w $28ffa
pub fn i_028ff2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FF6;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x28FFA; return 10; }
    8
}
/// $028FF6: jmp $28e02(pc)
pub fn i_028ff6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FFA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28E02;
    8
}
/// $028FFA: tst.b $46(a6)
pub fn i_028ffa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x28FFE;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $028FFE: beq.w $29026
pub fn i_028ffe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29002;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29026; return 10; }
    8
}
/// $029002: tst.b $44(a6)
pub fn i_029002(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29006;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $029006: bne.w $29020
pub fn i_029006(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2900A;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29020; return 10; }
    8
}
/// $02900A: subi.b #$1, $46(a6)
pub fn i_02900a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29010;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $029010: bne.w $29020
pub fn i_029010(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29014;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29020; return 10; }
    8
}
/// $029014: cmpi.b #$16, (a1)
pub fn i_029014(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29018;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x16, Size::Byte, false);
    12
}
/// $029018: beq.w $29026
pub fn i_029018(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2901C;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29026; return 10; }
    8
}
/// $02901C: move.l a1, $3c(a6)
pub fn i_02901c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29020;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== ClearXN_029020  ($029020, 6 B, thunks/ccr_helpers.c)
/// $029020: andi.b #$ee, ccr
pub fn i_029020(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29024;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $029024: rts
pub fn i_029024(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29026;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_029026  ($029026, 6 B, thunks/ccr_helpers.c)
/// $029026: ori.b #$11, ccr
pub fn i_029026(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2902A;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02902A: rts
pub fn i_02902a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2902C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptOp03_Loop_02902c  ($02902C, 66 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02902C: tst.b $47(a6)
pub fn i_02902c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29030;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $029030: bne.w $2903e
pub fn i_029030(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29034;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2903E; return 10; }
    8
}
/// $029034: move.b $1(a1), d1
pub fn i_029034(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29038;
    cpu.ir = 0x1229;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $029038: addq.b #$1, d1
pub fn i_029038(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2903A;
    cpu.ir = 0x5201;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = add_with_flags(cpu, v1, 1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    4
}
/// $02903A: move.b d1, $47(a6)
pub fn i_02903a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2903E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02903E: move.b $47(a6), d1
pub fn i_02903e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29042;
    cpu.ir = 0x122E;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (v3 & 0xFF);
    12
}
/// $029042: clr.b d0
pub fn i_029042(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29044;
    cpu.ir = 0x4200;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    4
}
/// $029044: subi.b #$1, d1
pub fn i_029044(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29048;
    cpu.ir = 0x401;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Byte, true);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029048: addx.b d0, d1
pub fn i_029048(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2904A;
    cpu.ir = 0xD300;
    let x = cpu.sr.get(StatusRegister::X) as u32;
    let h1 = cpu.d[1] & 0xFF;
    let h2 = cpu.d[0] & 0xFF;
    let r = addx_with_flags(cpu, h1, h2, x, Size::Byte);
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r & 0xFF);
    4
}
/// $02904A: move.b d1, $47(a6)
pub fn i_02904a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2904E;
    cpu.ir = 0x1D41;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x47);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02904E: tst.b d1
pub fn i_02904e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29050;
    cpu.ir = 0x4A01;
    let v1 = cpu.d[1] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $029050: beq.w $29060
pub fn i_029050(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29054;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29060; return 10; }
    8
}
/// $029054: movea.l $2(a1), a1
pub fn i_029054(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29058;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $029058: move.l a1, $3c(a6)
pub fn i_029058(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2905C;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02905C: jmp $28da8(pc)
pub fn i_02905c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29060;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
/// $029060: adda.l #$6, a1
pub fn i_029060(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29066;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029066: move.l a1, $3c(a6)
pub fn i_029066(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2906A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02906A: jmp $28da8(pc)
pub fn i_02906a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2906E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp04_Sound_02906e  ($02906E, 24 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02906E: move.w $2(a1), d0
pub fn i_02906e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29072;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $029072: jsr $2352.l
pub fn i_029072(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29078;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29078);
    cpu.pc = 0x2352;
    16
}
/// $029078: adda.l #$4, a1
pub fn i_029078(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2907E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02907E: move.l a1, $3c(a6)
pub fn i_02907e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29082;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $029082: jmp $28da8(pc)
pub fn i_029082(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29086;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp05_SetByte_02909e  ($02909E, 44 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02909E: move.w $2(a1), d1
pub fn i_02909e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290A2;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0290A2: cmpi.w #$a0, d1
pub fn i_0290a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290A6;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290A6: blt.w $290b6
pub fn i_0290a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290AA;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x290B6; return 10; }
    8
}
/// $0290AA: nop
pub fn i_0290aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290AC;
    cpu.ir = 0x4E71;
    4
}
/// $0290AC: nop
pub fn i_0290ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290AE;
    cpu.ir = 0x4E71;
    4
}
/// $0290AE: cmpi.w #$a0, d1
pub fn i_0290ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290B2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290B2: nop
pub fn i_0290b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290B4;
    cpu.ir = 0x4E71;
    4
}
/// $0290B4: trap #$f
pub fn i_0290b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290B6;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0290B6: move.b $1(a1), (a6, d1.w)
pub fn i_0290b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290BC;
    cpu.ir = 0x1DA9;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write8(ad4, v3 as u8);
    22
}
/// $0290BC: adda.l #$4, a1
pub fn i_0290bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290C2;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0290C2: move.l a1, $3c(a6)
pub fn i_0290c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290C6;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0290C6: jmp $28da8(pc)
pub fn i_0290c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290CA;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp06_SetWord_0290ca  ($0290CA, 44 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0290CA: move.w $4(a1), d1
pub fn i_0290ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290CE;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0290CE: cmpi.w #$a0, d1
pub fn i_0290ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290D2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290D2: blt.w $290e2
pub fn i_0290d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290D6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x290E2; return 10; }
    8
}
/// $0290D6: nop
pub fn i_0290d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290D8;
    cpu.ir = 0x4E71;
    4
}
/// $0290D8: nop
pub fn i_0290d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290DA;
    cpu.ir = 0x4E71;
    4
}
/// $0290DA: cmpi.w #$a0, d1
pub fn i_0290da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290DE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290DE: nop
pub fn i_0290de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290E0;
    cpu.ir = 0x4E71;
    4
}
/// $0290E0: trap #$f
pub fn i_0290e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290E2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0290E2: move.w $2(a1), (a6, d1.w)
pub fn i_0290e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290E8;
    cpu.ir = 0x3DA9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write16(ad4, v3 as u16);
    22
}
/// $0290E8: adda.l #$6, a1
pub fn i_0290e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290EE;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0290EE: move.l a1, $3c(a6)
pub fn i_0290ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290F2;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0290F2: jmp $28da8(pc)
pub fn i_0290f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290F6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp07_SetLong_0290f6  ($0290F6, 44 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0290F6: move.w $6(a1), d1
pub fn i_0290f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290FA;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x6);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0290FA: cmpi.w #$a0, d1
pub fn i_0290fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x290FE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0290FE: blt.w $2910e
pub fn i_0290fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29102;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x2910E; return 10; }
    8
}
/// $029102: nop
pub fn i_029102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29104;
    cpu.ir = 0x4E71;
    4
}
/// $029104: nop
pub fn i_029104(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29106;
    cpu.ir = 0x4E71;
    4
}
/// $029106: cmpi.w #$a0, d1
pub fn i_029106(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2910A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $02910A: nop
pub fn i_02910a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2910C;
    cpu.ir = 0x4E71;
    4
}
/// $02910C: trap #$f
pub fn i_02910c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2910E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02910E: move.l $2(a1), (a6, d1.w)
pub fn i_02910e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29114;
    cpu.ir = 0x2DA9;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    30
}
/// $029114: adda.l #$8, a1
pub fn i_029114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2911A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x8;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02911A: move.l a1, $3c(a6)
pub fn i_02911a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2911E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02911E: jmp $28da8(pc)
pub fn i_02911e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29122;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp08_Call_029122  ($029122, 28 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029122: movem.l a1, -(a7)
pub fn i_029122(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29126;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $029126: movea.l $2(a1), a1
pub fn i_029126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2912A;
    cpu.ir = 0x2269;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $02912A: jsr (a1)
pub fn i_02912a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2912C;
    cpu.ir = 0x4E91;
    cpu.push32(bus, 0x2912C);
    cpu.jump_to(cpu.a[1]);
    16
}
/// $02912C: movem.l (a7)+, a1
pub fn i_02912c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29130;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $029130: adda.l #$6, a1
pub fn i_029130(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29136;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029136: move.l a1, $3c(a6)
pub fn i_029136(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2913A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02913A: jmp $28da8(pc)
pub fn i_02913a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2913E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp09_SetHitbox48_02913e  ($02913E, 20 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02913E: move.l $2(a1), $48(a6)
pub fn i_02913e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29144;
    cpu.ir = 0x2D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x48);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $029144: adda.l #$6, a1
pub fn i_029144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2914A;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02914A: move.l a1, $3c(a6)
pub fn i_02914a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2914E;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02914E: jmp $28da8(pc)
pub fn i_02914e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29152;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp0A_SetField4C_029152  ($029152, 20 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029152: move.l $2(a1), $4c(a6)
pub fn i_029152(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29158;
    cpu.ir = 0x2D69;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    let ad4 = cpu.a[6].wrapping_add(0x4C);
    bus.write32(ad4, v3);
    cpu.sr.set_nz(v3, Size::Long); cpu.sr.0 &= !0x3;
    28
}
/// $029158: adda.l #$6, a1
pub fn i_029158(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2915E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x6;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02915E: move.l a1, $3c(a6)
pub fn i_02915e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29162;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $029162: jmp $28da8(pc)
pub fn i_029162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29166;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp0D_AndByte_029196  ($029196, 46 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029196: move.w $2(a1), d1
pub fn i_029196(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2919A;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02919A: cmpi.w #$a0, d1
pub fn i_02919a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2919E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $02919E: blt.w $291ae
pub fn i_02919e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291A2;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x291AE; return 10; }
    8
}
/// $0291A2: nop
pub fn i_0291a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291A4;
    cpu.ir = 0x4E71;
    4
}
/// $0291A4: nop
pub fn i_0291a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291A6;
    cpu.ir = 0x4E71;
    4
}
/// $0291A6: cmpi.w #$a0, d1
pub fn i_0291a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291AA;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0291AA: nop
pub fn i_0291aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291AC;
    cpu.ir = 0x4E71;
    4
}
/// $0291AC: trap #$f
pub fn i_0291ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291AE;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0291AE: move.b $1(a1), d0
pub fn i_0291ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291B2;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0291B2: and.b d0, (a6, d1.w)
pub fn i_0291b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291B6;
    cpu.ir = 0xC136;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 & cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $0291B6: adda.l #$4, a1
pub fn i_0291b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291BC;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0291BC: move.l a1, $3c(a6)
pub fn i_0291bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291C0;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0291C0: jmp $28da8(pc)
pub fn i_0291c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x291C4;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp10_OrByte_029220  ($029220, 46 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029220: move.w $2(a1), d1
pub fn i_029220(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29224;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $029224: cmpi.w #$a0, d1
pub fn i_029224(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29228;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $029228: blt.w $29238
pub fn i_029228(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2922C;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x29238; return 10; }
    8
}
/// $02922C: nop
pub fn i_02922c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2922E;
    cpu.ir = 0x4E71;
    4
}
/// $02922E: nop
pub fn i_02922e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29230;
    cpu.ir = 0x4E71;
    4
}
/// $029230: cmpi.w #$a0, d1
pub fn i_029230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29234;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $029234: nop
pub fn i_029234(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29236;
    cpu.ir = 0x4E71;
    4
}
/// $029236: trap #$f
pub fn i_029236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29238;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $029238: move.b $1(a1), d0
pub fn i_029238(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2923C;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02923C: or.b d0, (a6, d1.w)
pub fn i_02923c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29240;
    cpu.ir = 0x8136;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $029240: adda.l #$4, a1
pub fn i_029240(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29246;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029246: move.l a1, $3c(a6)
pub fn i_029246(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2924A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02924A: jmp $28da8(pc)
pub fn i_02924a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2924E;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp13_EorByte_0292aa  ($0292AA, 46 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0292AA: move.w $2(a1), d1
pub fn i_0292aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292AE;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0292AE: cmpi.w #$a0, d1
pub fn i_0292ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292B2;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0292B2: blt.w $292c2
pub fn i_0292b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292B6;
    cpu.ir = 0x6D00;
    if cpu.sr.evaluate(Condition::Lt) { cpu.pc = 0x292C2; return 10; }
    8
}
/// $0292B6: nop
pub fn i_0292b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292B8;
    cpu.ir = 0x4E71;
    4
}
/// $0292B8: nop
pub fn i_0292b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292BA;
    cpu.ir = 0x4E71;
    4
}
/// $0292BA: cmpi.w #$a0, d1
pub fn i_0292ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292BE;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xA0, Size::Word, false);
    8
}
/// $0292BE: nop
pub fn i_0292be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292C0;
    cpu.ir = 0x4E71;
    4
}
/// $0292C0: trap #$f
pub fn i_0292c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292C2;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $0292C2: move.b $1(a1), d0
pub fn i_0292c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292C6;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0292C6: eor.b d0, (a6, d1.w)
pub fn i_0292c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292CA;
    cpu.ir = 0xB136;
    let ad1 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[1] as i16 as i32 as u32));
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 ^ cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $0292CA: adda.l #$4, a1
pub fn i_0292ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292D0;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0292D0: move.l a1, $3c(a6)
pub fn i_0292d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292D4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0292D4: jmp $28da8(pc)
pub fn i_0292d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x292D8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp16_RepeatSprite_029334  ($029334, 8 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029334: movea.l $40(a6), a1
pub fn i_029334(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29338;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $029338: jmp $28dfe(pc)
pub fn i_029338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2933C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DFE;
    8
}
// ===== ScriptOp17_SpriteFlag5_02933c  ($02933C, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02933C: bset.b #$5, $12(a6)
pub fn i_02933c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29342;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $029342: adda.l #$2, a1
pub fn i_029342(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29348;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029348: jmp $28dfe(pc)
pub fn i_029348(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2934C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DFE;
    8
}
// ===== ScriptOp18_SpriteFlag5Cont_02934c  ($02934C, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02934C: bset.b #$5, $12(a6)
pub fn i_02934c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29352;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $029352: adda.l #$2, a1
pub fn i_029352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29358;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029358: jmp $28e02(pc)
pub fn i_029358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2935C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28E02;
    8
}
// ===== ScriptOp19_SetField70_02935c  ($02935C, 26 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02935C: move.l a1, $70(a6)
pub fn i_02935c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29360;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x70);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $029360: addi.l #$2, $70(a6)
pub fn i_029360(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29368;
    cpu.ir = 0x6AE;
    let ad1 = cpu.a[6].wrapping_add(0x70);
    let v2 = bus.read32(ad1);
    let r3 = add_with_flags(cpu, v2, 0x2, Size::Long, true);
    bus.write32(ad1, r3);
    20
}
/// $029368: adda.l #$a, a1
pub fn i_029368(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2936E;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0xA;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $02936E: move.l a1, $3c(a6)
pub fn i_02936e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29372;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $029372: jmp $28da8(pc)
pub fn i_029372(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29376;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp1A_WaitFromTable_029376  ($029376, 50 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029376: move.b $46(a6), d2
pub fn i_029376(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2937A;
    cpu.ir = 0x142E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02937A: bne.w $2939a
pub fn i_02937a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2937E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2939A; return 10; }
    8
}
/// $02937E: move.b $1(a1), d0
pub fn i_02937e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29382;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $029382: andi.w #$ff, d0
pub fn i_029382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29386;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $029386: movea.l (a6, d0.w), a0
pub fn i_029386(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2938A;
    cpu.ir = 0x2076;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02938A: move.b $3b(a6), d0
pub fn i_02938a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2938E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x3B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02938E: andi.w #$ff, d0
pub fn i_02938e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29392;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $029392: move.b (a0, d0.w), d2
pub fn i_029392(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29396;
    cpu.ir = 0x1430;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFF) | (v3 & 0xFF);
    14
}
/// $029396: move.b d2, $46(a6)
pub fn i_029396(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2939A;
    cpu.ir = 0x1D42;
    let v1 = cpu.d[2] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02939A: adda.l #$2, a1
pub fn i_02939a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293A0;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0293A0: move.l a1, $3c(a6)
pub fn i_0293a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293A4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0293A4: jmp $28da8(pc)
pub fn i_0293a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293A8;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== ScriptOp1B_IndexStride18_0293a8  ($0293A8, 30 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0293A8: clr.w d0
pub fn i_0293a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293AA;
    cpu.ir = 0x4240;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    4
}
/// $0293AA: move.b $1(a1), d0
pub fn i_0293aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293AE;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0293AE: move.w (a6, d0.w), d1
pub fn i_0293ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293B2;
    cpu.ir = 0x3236;
    let ad2 = cpu.a[6].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    14
}
/// $0293B2: mulu.w #$12, d1
pub fn i_0293b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293B6;
    cpu.ir = 0xC2FC;
    let v1: u32 = 0x12;
    let r = (v1 & 0xFFFF).wrapping_mul(cpu.d[1] & 0xFFFF);
    cpu.d[1] = r;
    cpu.sr.set_nz(r, Size::Long); cpu.sr.0 &= !0x3;
    70
}
/// $0293B6: adda.l #$2, a1
pub fn i_0293b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293BC;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x2;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0293BC: adda.l d1, a1
pub fn i_0293bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293BE;
    cpu.ir = 0xD3C1;
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $0293BE: move.l a1, $3c(a6)
pub fn i_0293be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293C2;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0293C2: jmp $28da8(pc)
pub fn i_0293c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x293C6;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x28DA8;
    8
}
// ===== SetTaskB_029424  ($029424, 6 B, kernel/task_setters_b.c)
/// $029424: move.b d0, $44(a6)
pub fn i_029424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29428;
    cpu.ir = 0x1D40;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x44);
    bus.write8(ad2, v1 as u8);
    12
}
/// $029428: rts
pub fn i_029428(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2942A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptOp1E_SpawnChild_02942a  ($02942A, 128 B, entity/collmap_hitbox_script_ops_027400.s)
/// $02942A: move.w $2(a1), d0
pub fn i_02942a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2942E;
    cpu.ir = 0x3029;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02942E: cmpi.b #$b, $106ece.l
pub fn i_02942e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29436;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xB, Size::Byte, false);
    20
}
/// $029436: beq.w $29446
pub fn i_029436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2943A;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29446; return 10; }
    8
}
/// $02943A: cmpi.b #$c, $106ece.l
pub fn i_02943a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29442;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ECE) as u32;
    sub_with_flags(cpu, v2, 0xC, Size::Byte, false);
    20
}
/// $029442: bne.w $2944a
pub fn i_029442(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29446;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2944A; return 10; }
    8
}
/// $029446: andi.w #$fff, d0
pub fn i_029446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2944A;
    cpu.ir = 0x240;
    let v1 = cpu.d[0] & 0xFFFF;
    let r2 = (v1 & 0xFFF) & 0xFFFF;
    cpu.sr.set_nz(r2, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $02944A: move.w $4(a1), d1
pub fn i_02944a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2944E;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0x4);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02944E: jsr $440d0.l
pub fn i_02944e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29454;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29454);
    cpu.pc = 0x440D0;
    16
}
/// $029454: move.w d0, $22(a6)
pub fn i_029454(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29458;
    cpu.ir = 0x3D40;
    let v1 = cpu.d[0] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    bus.write16(ad2, v1 as u16);
    12
}
/// $029458: move.w d1, $24(a6)
pub fn i_029458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2945C;
    cpu.ir = 0x3D41;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02945C: move.w $c(a1), $38(a6)
pub fn i_02945c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29462;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0xC);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad4, v3 as u16);
    20
}
/// $029462: move.w $a(a1), $70(a6)
pub fn i_029462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29468;
    cpu.ir = 0x3D69;
    let ad2 = cpu.a[1].wrapping_add(0xA);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x70);
    bus.write16(ad4, v3 as u16);
    20
}
/// $029468: move.b $1(a1), d0
pub fn i_029468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2946C;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02946C: or.b d0, $12(a6)
pub fn i_02946c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29470;
    cpu.ir = 0x812E;
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | cpu.d[0]) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $029470: tst.b (a1)
pub fn i_029470(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29472;
    cpu.ir = 0x4A11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    cpu.sr.set_nz(v2, Size::Byte); cpu.sr.0 &= !0x3;
    8
}
/// $029472: bne.w $294ae
pub fn i_029472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29476;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x294AE; return 10; }
    8
}
/// $029476: move.w $e(a1), d1
pub fn i_029476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2947A;
    cpu.ir = 0x3229;
    let ad2 = cpu.a[1].wrapping_add(0xE);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02947A: cmpi.w #$ffff, d1
pub fn i_02947a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2947E;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02947E: bne.w $2948e
pub fn i_02947e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29482;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2948E; return 10; }
    8
}
/// $029482: nop
pub fn i_029482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29484;
    cpu.ir = 0x4E71;
    4
}
/// $029484: nop
pub fn i_029484(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29486;
    cpu.ir = 0x4E71;
    4
}
/// $029486: cmpi.w #$ffff, d1
pub fn i_029486(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2948A;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0xFFFF, Size::Word, false);
    8
}
/// $02948A: nop
pub fn i_02948a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2948C;
    cpu.ir = 0x4E71;
    4
}
/// $02948C: trap #$f
pub fn i_02948c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2948E;
    cpu.ir = 0x4E4F;
    let was = cpu.sr.supervisor(); if was { cpu.ssp = cpu.a[7]; } else { cpu.usp = cpu.a[7]; cpu.a[7] = cpu.ssp; }
    let old = cpu.sr.0; cpu.sr.set(StatusRegister::S, true); cpu.sr.set(StatusRegister::T, false);
    cpu.push32(bus, cpu.pc); cpu.push16(bus, old); cpu.pc = bus.read32(0xBC);
    34
}
/// $02948E: movem.l a1, -(a7)
pub fn i_02948e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29492;
    cpu.ir = 0x48E7;
    let mut a = cpu.a[7];
    let v = cpu.a[1]; bus.write16(a.wrapping_sub(2), v as u16); bus.write16(a.wrapping_sub(4), (v >> 16) as u16); a = a.wrapping_sub(4);
    cpu.a[7] = a;
    20
}
/// $029492: jsr $236e.l
pub fn i_029492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29498;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29498);
    cpu.pc = 0x236E;
    16
}
/// $029498: movem.l (a7)+, a1
pub fn i_029498(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2949C;
    cpu.ir = 0x4CDF;
    let mut a = cpu.a[7];
    cpu.a[1] = bus.read32(a); a = a.wrapping_add(4);
    cpu.a[7] = a;
    20
}
/// $02949C: movea.l $10(a1), a0
pub fn i_02949c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294A0;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0294A0: cmpa.l #$ffffffff, a0
pub fn i_0294a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294A6;
    cpu.ir = 0xB1FC;
    let v1: u32 = 0xFFFFFFFF;
    let h1 = cpu.a[0];
    sub_with_flags(cpu, h1, v1, Size::Long, false);
    6
}
/// $0294A6: beq.w $294ae
pub fn i_0294a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294AA;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x294AE; return 10; }
    8
}
// ===== JsrPcThunk_0294aa  ($0294AA, 6 B, thunks/jsr_pc_thunks.c)
/// $0294AA: jsr $28cd4(pc)
pub fn i_0294aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294AE;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x294AE);
    cpu.pc = 0x28CD4;
    16
}
/// $0294AE: rts
pub fn i_0294ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294B0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ScriptOp1E_SpawnChild_Sprite_0294b0  ($0294B0, 170 B, entity/collmap_hitbox_script_ops_027400.s)
/// $0294B0: move.l a1, $40(a6)
pub fn i_0294b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294B4;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x40);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $0294B4: move.w $22(a6), d0
pub fn i_0294b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294B8;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x22);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0294B8: move.w $24(a6), d1
pub fn i_0294b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294BC;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x24);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0294BC: bra.w $294c8
pub fn i_0294bc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294C0;
    cpu.ir = 0x6000;
    cpu.pc = 0x294C8;
    10
}
/// $0294C8: swap d0
pub fn i_0294c8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294CA;
    cpu.ir = 0x4840;
    cpu.d[0] = cpu.d[0].rotate_left(16);
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $0294CA: subi.w #$1, d1
pub fn i_0294ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294CE;
    cpu.ir = 0x441;
    let v1 = cpu.d[1] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, 0x1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r2 & 0xFFFF);
    8
}
/// $0294CE: move.w d1, d0
pub fn i_0294ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294D0;
    cpu.ir = 0x3001;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $0294D0: move.w $38(a6), d1
pub fn i_0294d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294D4;
    cpu.ir = 0x322E;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0294D4: move.w $14(a6), d2
pub fn i_0294d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294D8;
    cpu.ir = 0x342E;
    let ad2 = cpu.a[6].wrapping_add(0x14);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[2] = (cpu.d[2] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $0294D8: move.b $3a(a6), d5
pub fn i_0294d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294DC;
    cpu.ir = 0x1A2E;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0294DC: move.b $12(a6), d4
pub fn i_0294dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294E0;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0294E0: andi.b #$4, d4
pub fn i_0294e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294E4;
    cpu.ir = 0x204;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 & 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0294E4: or.b d4, d5
pub fn i_0294e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294E6;
    cpu.ir = 0x8A04;
    let v1 = cpu.d[4] & 0xFF;
    let r = (v1 | cpu.d[5]) & 0xFF;
    cpu.d[5] = (cpu.d[5] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0294E6: move.b $33(a6), d3
pub fn i_0294e6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294EA;
    cpu.ir = 0x162E;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[3] = (cpu.d[3] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0294EA: move.b $32(a6), d4
pub fn i_0294ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294EE;
    cpu.ir = 0x182E;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (v3 & 0xFF);
    12
}
/// $0294EE: movea.l $2(a1), a0
pub fn i_0294ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294F2;
    cpu.ir = 0x2069;
    let ad2 = cpu.a[1].wrapping_add(0x2);
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    16
}
/// $0294F2: btst.b #$6, $12(a6)
pub fn i_0294f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294F8;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((6) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x12);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $0294F8: bne.w $29506
pub fn i_0294f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x294FC;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29506; return 10; }
    8
}
/// $0294FC: jsr $5a9e2.l
pub fn i_0294fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29502;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x29502);
    cpu.pc = 0x5A9E2;
    16
}
/// $029502: bra.w $2950c
pub fn i_029502(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29506;
    cpu.ir = 0x6000;
    cpu.pc = 0x2950C;
    10
}
/// $029506: jsr $5a9d6.l
pub fn i_029506(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2950C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2950C);
    cpu.pc = 0x5A9D6;
    16
}
/// $02950C: move.b $1(a1), d0
pub fn i_02950c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29510;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $029510: adda.l #$8, a1
pub fn i_029510(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29516;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x8;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029516: btst.b #$3, d0
pub fn i_029516(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2951A;
    cpu.ir = 0x800;
    let mk: u32 = 1u32 << ((3) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $02951A: bne.w $29524
pub fn i_02951a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2951E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29524; return 10; }
    8
}
/// $02951E: adda.l #$4, a1
pub fn i_02951e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29524;
    cpu.ir = 0xD3FC;
    let v1: u32 = 0x4;
    cpu.a[1] = cpu.a[1].wrapping_add(v1);
    8
}
/// $029524: cmpi.b #$18, (a1)
pub fn i_029524(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29528;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x18, Size::Byte, false);
    12
}
/// $029528: bne.w $29530
pub fn i_029528(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2952C;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x29530; return 10; }
    8
}
/// $02952C: jmp $2934c(pc)
pub fn i_02952c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29530;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x2934C;
    8
}
/// $029530: cmpi.b #$2, (a1)
pub fn i_029530(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29534;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x2, Size::Byte, false);
    12
}
/// $029534: bne.w $2953c
pub fn i_029534(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29538;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2953C; return 10; }
    8
}
/// $029538: jmp $294b4(pc)
pub fn i_029538(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2953C;
    cpu.ir = 0x4EFA;
    cpu.pc = 0x294B4;
    8
}
/// $02953C: tst.b $46(a6)
pub fn i_02953c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29540;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x46);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $029540: beq.w $29560
pub fn i_029540(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29544;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29560; return 10; }
    8
}
/// $029544: subi.b #$1, $46(a6)
pub fn i_029544(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2954A;
    cpu.ir = 0x42E;
    let ad1 = cpu.a[6].wrapping_add(0x46);
    let v2 = bus.read8(ad1) as u32;
    let r3 = sub_with_flags(cpu, v2, 0x1, Size::Byte, true);
    bus.write8(ad1, r3 as u8);
    16
}
/// $02954A: bne.w $2955a
pub fn i_02954a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2954E;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2955A; return 10; }
    8
}
/// $02954E: cmpi.b #$16, (a1)
pub fn i_02954e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29552;
    cpu.ir = 0xC11;
    let v2 = bus.read8(cpu.a[1]) as u32;
    sub_with_flags(cpu, v2, 0x16, Size::Byte, false);
    12
}
/// $029552: beq.w $29560
pub fn i_029552(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29556;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x29560; return 10; }
    8
}
/// $029556: move.l a1, $3c(a6)
pub fn i_029556(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2955A;
    cpu.ir = 0x2D49;
    let v1 = cpu.a[1] & 0xFFFFFFFF;
    let ad2 = cpu.a[6].wrapping_add(0x3C);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
// ===== ClearXN_02955a  ($02955A, 6 B, thunks/ccr_helpers.c)
/// $02955A: andi.b #$ee, ccr
pub fn i_02955a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2955E;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02955E: rts
pub fn i_02955e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29560;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_029560  ($029560, 6 B, thunks/ccr_helpers.c)
/// $029560: ori.b #$11, ccr
pub fn i_029560(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29564;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $029564: rts
pub fn i_029564(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29566;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ChildRank_CmpByte10_029566  ($029566, 16 B, entity/collmap_hitbox_script_ops_027400.s)
/// $029566: movea.l $8(a6), a1
pub fn i_029566(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2956A;
    cpu.ir = 0x226E;
    let ad2 = cpu.a[6].wrapping_add(0x8);
    let v3 = bus.read32(ad2);
    cpu.a[1] = v3;
    16
}
/// $02956A: move.b $10(a6), d0
pub fn i_02956a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2956E;
    cpu.ir = 0x102E;
    let ad2 = cpu.a[6].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02956E: cmp.b $10(a1), d0
pub fn i_02956e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29572;
    cpu.ir = 0xB029;
    let ad2 = cpu.a[1].wrapping_add(0x10);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    sub_with_flags(cpu, h1, v3, Size::Byte, false);
    12
}
/// $029572: bcs.w $2957c
pub fn i_029572(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29576;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2957C; return 10; }
    8
}
// ===== ClearXN_029576  ($029576, 6 B, thunks/ccr_helpers.c)
/// $029576: andi.b #$ee, ccr
pub fn i_029576(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2957A;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02957A: rts
pub fn i_02957a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2957C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetXN_02957c  ($02957C, 6 B, thunks/ccr_helpers.c)
/// $02957C: ori.b #$11, ccr
pub fn i_02957c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29580;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $029580: rts
pub fn i_029580(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29582;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== JsrPcThunk_029582  ($029582, 6 B, thunks/jsr_pc_thunks.c)
/// $029582: jsr $2a46c(pc)
pub fn i_029582(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29586;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x29586);
    cpu.pc = 0x2A46C;
    16
}
/// $029586: rts
pub fn i_029586(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29588;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== SetGlobalFlagFF_029588  ($029588, 16 B, kernel/scheduler_flag_setters.c)
/// $029588: move.b #$ff, $106f4a.l
pub fn i_029588(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29590;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F4A, v1 as u8);
    16
}
/// $029590: jmp $518.l
pub fn i_029590(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29596;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== ClearGlobalFlag_029598  ($029598, 14 B, kernel/scheduler_flag_setters.c)
/// $029598: clr.b $106f4a.l
pub fn i_029598(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2959E;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106F4A) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106F4A, r2 as u8);
    16
}
/// $02959E: jmp $518.l
pub fn i_02959e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295A4;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x518;
    8
}
// ===== Slug_AngleToSpriteIdx_0295a6  ($0295A6, 14 B, slug/helpers_0295a6.s)
/// $0295A6: lea.l $2b0bee.l, a0
pub fn i_0295a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295AC;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x2B0BEE;
    20
}
/// $0295AC: move.b (a0, d0.w), d0
pub fn i_0295ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295B0;
    cpu.ir = 0x1030;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    14
}
/// $0295B0: ext.w d0
pub fn i_0295b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295B2;
    cpu.ir = 0x4880;
    let v = cpu.d[0] as i8 as i16 as u16 as u32; cpu.d[0] = (cpu.d[0] & 0xFFFF_0000) | v;
    cpu.sr.set_nz(v, Size::Word); cpu.sr.0 &= !0x3;
    4
}
/// $0295B2: rts
pub fn i_0295b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295B4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_HitboxIdle_0295b4  ($0295B4, 76 B, slug/helpers_0295a6.s)
/// $0295B4: or.b d1, d0
pub fn i_0295b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295B6;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $0295B6: ori.b #$0, d0
pub fn i_0295b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295BA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0295BA: dc.w $ffec
pub fn i_0295ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x295BC;
    cpu.ir = 0xFFEC;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Slug_HitboxIdleB_029600  ($029600, 76 B, slug/helpers_0295a6.s)
/// $029600: or.b d1, d0
pub fn i_029600(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29602;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $029602: ori.b #$0, d0
pub fn i_029602(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29606;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029606: dc.w $ffec
pub fn i_029606(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29608;
    cpu.ir = 0xFFEC;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Slug_HitboxDestroyed_02964c  ($02964C, 324 B, slug/helpers_0295a6.s)
/// $02964C: or.b d1, d0
pub fn i_02964c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2964E;
    cpu.ir = 0x8001;
    let v1 = cpu.d[1] & 0xFF;
    let r = (v1 | cpu.d[0]) & 0xFF;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    cpu.sr.set_nz(r, Size::Byte); cpu.sr.0 &= !0x3;
    4
}
/// $02964E: ori.b #$0, d0
pub fn i_02964e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29652;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029652: dc.w $fff6
pub fn i_029652(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29654;
    cpu.ir = 0xFFF6;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Slug_HitboxA_029790  ($029790, 164 B, slug/helpers_0295a6.s)
/// $029790: btst.l d1, (a4)+
pub fn i_029790(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29792;
    cpu.ir = 0x31C;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 8);
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    12
}
/// $029792: andi.w #$404, (a0)+
pub fn i_029792(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29796;
    cpu.ir = 0x258;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(2);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 & 0x404) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    12
}
/// $029796: ori.b #$0, d0
pub fn i_029796(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2979A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02979A: dc.w $ffc0
pub fn i_02979a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2979C;
    cpu.ir = 0xFFC0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Slug_HitboxB_029834  ($029834, 164 B, slug/helpers_0295a6.s)
/// $029834: btst.l d1, (a6)+
pub fn i_029834(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29836;
    cpu.ir = 0x31E;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 8);
    let ad2 = cpu.a[6]; cpu.a[6] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    12
}
/// $029836: ori.b #$4, d1
pub fn i_029836(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2983A;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02983A: ori.b #$0, d0
pub fn i_02983a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2983E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $02983E: dc.w $ffd0
pub fn i_02983e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29840;
    cpu.ir = 0xFFD0;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Slug_HitboxC_0298d8  ($0298D8, 164 B, slug/helpers_0295a6.s)
/// $0298D8: btst.l d1, (a4)+
pub fn i_0298d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x298DA;
    cpu.ir = 0x31C;
    let mk: u32 = 1u32 << ((cpu.d[1]) % 8);
    let ad2 = cpu.a[4]; cpu.a[4] = ad2.wrapping_add(1);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    12
}
/// $0298DA: ori.b #$4, (a4)
pub fn i_0298da(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x298DE;
    cpu.ir = 0x14;
    let ad1 = cpu.a[4];
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x4) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0298DE: ori.b #$0, d0
pub fn i_0298de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x298E2;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0298E2: dc.w $ff50
pub fn i_0298e2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x298E4;
    cpu.ir = 0xFF50;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Slug_HitboxD_02997c  ($02997C, 34 B, slug/helpers_0295a6.s)
/// $02997C: btst.l d0, d1
pub fn i_02997c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2997E;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $02997E: btst.l d0, d1
pub fn i_02997e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29980;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $029980: btst.l d0, d1
pub fn i_029980(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29982;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $029982: btst.l d0, d1
pub fn i_029982(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29984;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $029984: btst.l d0, d1
pub fn i_029984(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29986;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $029986: btst.l d0, d1
pub fn i_029986(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29988;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $029988: btst.l d0, d1
pub fn i_029988(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2998A;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $02998A: btst.l d0, d1
pub fn i_02998a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2998C;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $02998C: ori.b #$1, d1
pub fn i_02998c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29990;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029990: btst.l d0, d1
pub fn i_029990(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29992;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $029992: btst.l d0, d1
pub fn i_029992(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29994;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $029994: ori.b #$1, d1
pub fn i_029994(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29998;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029998: btst.l d0, d1
pub fn i_029998(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2999A;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $02999A: btst.l d0, d1
pub fn i_02999a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2999C;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $02999C: btst.l d0, d1
pub fn i_02999c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2999E;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
// ===== Slug_HitboxCb_02999e  ($02999E, 84 B, slug/helpers_0295a6.s)
/// $02999E: ori.b #$2, d4
pub fn i_02999e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299A2;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0299A2: ?
pub fn i_0299a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299A6;
    cpu.ir = 0x997C;
    let v1: u32 = 0x0;
    let h1 = cpu.d[4] & 0xFFFF;
    let r2 = sub_with_flags(cpu, v1, h1, Size::Word, true);
    4
}
/// $0299A6: ori.b #$f4, d0
pub fn i_0299a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299AA;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xF4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0299AA: ?
pub fn i_0299aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299AE;
    cpu.ir = 0xC;
    let v1 = cpu.a[4] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[4] = r2;
    8
}
/// $0299AE: ori.b #$0, (a0)+
pub fn i_0299ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299B2;
    cpu.ir = 0x18;
    let ad1 = cpu.a[0]; cpu.a[0] = ad1.wrapping_add(1);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    12
}
/// $0299B2: ori.b #$6c, -(a4)
pub fn i_0299b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299B6;
    cpu.ir = 0x24;
    let ad1 = cpu.a[4].wrapping_sub(1); cpu.a[4] = ad1;
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x6C) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    14
}
/// $0299B6: dc.w $ffff
pub fn i_0299b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299B8;
    cpu.ir = 0xFFFF;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Slug_HitboxCbB_0299f2  ($0299F2, 34 B, slug/helpers_0295a6.s)
/// $0299F2: ori.b #$0, d0
pub fn i_0299f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299F6;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0299F6: btst.l d0, d1
pub fn i_0299f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299F8;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0299F8: ori.b #$0, d0
pub fn i_0299f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299FC;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $0299FC: btst.l d0, d0
pub fn i_0299fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x299FE;
    cpu.ir = 0x100;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[0] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
/// $0299FE: ori.b #$0, d0
pub fn i_0299fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A02;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A02: ori.b #$0, d0
pub fn i_029a02(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A06;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A06: ori.b #$0, d0
pub fn i_029a06(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A0A;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A0A: ori.b #$0, d0
pub fn i_029a0a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A0E;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A0E: ori.b #$1, d1
pub fn i_029a0e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A12;
    cpu.ir = 0x1;
    let v1 = cpu.d[1] & 0xFF;
    let r2 = (v1 | 0x1) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A12: btst.l d0, d1
pub fn i_029a12(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A14;
    cpu.ir = 0x101;
    let mk: u32 = 1u32 << ((cpu.d[0]) % 32);
    let v1 = cpu.d[1] & 0xFFFFFFFF;
    cpu.sr.set(StatusRegister::Z, v1 & mk == 0);
    8
}
// ===== Slug_HitboxCbC_029a14  ($029A14, 84 B, slug/helpers_0295a6.s)
/// $029A14: ori.b #$2, d4
pub fn i_029a14(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A18;
    cpu.ir = 0x4;
    let v1 = cpu.d[4] & 0xFF;
    let r2 = (v1 | 0x2) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[4] = (cpu.d[4] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A18: suba.l (a2, d0.w), a4
pub fn i_029a18(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A1C;
    cpu.ir = 0x99F2;
    let ad2 = cpu.a[2].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[4] = cpu.a[4].wrapping_sub(v3);
    22
}
/// $029A1C: ori.b #$f4, d0
pub fn i_029a1c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A20;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0xF4) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A20: ?
pub fn i_029a20(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A24;
    cpu.ir = 0xC;
    let v1 = cpu.a[4] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.a[4] = r2;
    8
}
/// $029A24: ori.b #$0, $24(a0)
pub fn i_029a24(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A2A;
    cpu.ir = 0x28;
    let ad1 = cpu.a[0].wrapping_add(0x24);
    let v2 = bus.read8(ad1) as u32;
    let r3 = (v2 | 0x0) & 0xFF;
    cpu.sr.set_nz(r3, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(ad1, r3 as u8);
    16
}
/// $029A2A: movea.w -$1(a4), a3
pub fn i_029a2a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A2E;
    cpu.ir = 0x366C;
    let ad2 = cpu.a[4].wrapping_add(0xFFFFFFFF);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[3] = v3 as i16 as i32 as u32;
    12
}
/// $029A2E: ori.b #$0, d0
pub fn i_029a2e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A32;
    cpu.ir = 0x0;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 | 0x0) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A32: andi.b #$24, d0
pub fn i_029a32(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A36;
    cpu.ir = 0x200;
    let v1 = cpu.d[0] & 0xFF;
    let r2 = (v1 & 0x24) & 0xFF;
    cpu.sr.set_nz(r2, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r2 & 0xFF);
    8
}
/// $029A36: movea.w ([$fff40028]), a3
pub fn i_029a36(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A3A;
    cpu.ir = 0x3676;
    let ad2 = cpu.a[6].wrapping_add(0xFFFFFFFF).wrapping_add(cpu.a[7]);
    let v3 = bus.read16(ad2) as u32;
    cpu.a[3] = v3 as i16 as i32 as u32;
    14
}
/// $029A3A: dc.w $fff4
pub fn i_029a3a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x29A3C;
    cpu.ir = 0xFFF4;
    cpu.pc = cpu.instr_pc;
    cpu.enter_exception(bus, Exception::LineF);
    return 34;
}
// ===== Slug_Init_02a0f8  ($02A0F8, 178 B, slug/helpers_0295a6.s)
/// $02A0F8: jsr $236e.l
pub fn i_02a0f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A0FE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A0FE);
    cpu.pc = 0x236E;
    16
}
/// $02A0FE: move.w #$1, d1
pub fn i_02a0fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A102;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A102: jsr $236e.l
pub fn i_02a102(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A108;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A108);
    cpu.pc = 0x236E;
    16
}
/// $02A108: move.w #$1e, $1c(a6)
pub fn i_02a108(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A10E;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x1E;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A10E: jsr $138fe.l
pub fn i_02a10e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A114;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A114);
    cpu.pc = 0x138FE;
    16
}
/// $02A114: move.w $16(a6), $14(a6)
pub fn i_02a114(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A11A;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02A11A: bset.b #$4, $12(a6)
pub fn i_02a11a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A120;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A120: bset.b #$2, $5b(a6)
pub fn i_02a120(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A126;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A126: bset.b #$0, $6b(a6)
pub fn i_02a126(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A12C;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A12C: move.b #$ff, $32(a6)
pub fn i_02a12c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A132;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A132: move.b #$ff, $33(a6)
pub fn i_02a132(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A138;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A138: move.b #$0, $3a(a6)
pub fn i_02a138(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A13E;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A13E: move.w #$8000, $38(a6)
pub fn i_02a13e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A144;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A144: ori.w #$c, $38(a6)
pub fn i_02a144(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A14A;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A14A: jsr $5e98a.l
pub fn i_02a14a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A150;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A150);
    cpu.pc = 0x5E98A;
    16
}
/// $02A150: clr.b $8c(a6)
pub fn i_02a150(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A154;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A154: move.l #$295b4, $60(a6)
pub fn i_02a154(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A15C;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02A15C: jsr $8f6d2.l
pub fn i_02a15c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A162;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A162);
    cpu.pc = 0x8F6D2;
    16
}
/// $02A162: move.w #$30, $66(a6)
pub fn i_02a162(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A168;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A168: jsr $2fada(pc)
pub fn i_02a168(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A16C;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A16C);
    cpu.pc = 0x2FADA;
    16
}
/// $02A16C: clr.w $94(a6)
pub fn i_02a16c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A170;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02A170: clr.w $36(a6)
pub fn i_02a170(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A174;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02A174: clr.b $92(a6)
pub fn i_02a174(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A178;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A178: clr.b $8f(a6)
pub fn i_02a178(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A17C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A17C: move.b #$a, $90(a6)
pub fn i_02a17c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A182;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xA;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A182: bclr.b #$3, $13(a6)
pub fn i_02a182(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A188;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A188: lea.l $7773e.l, a1
pub fn i_02a188(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A18E;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7773E;
    20
}
/// $02A18E: jsr $4ae.l
pub fn i_02a18e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A194;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A194);
    cpu.pc = 0x4AE;
    16
}
/// $02A194: jsr $5dd02.l
pub fn i_02a194(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A19A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A19A);
    cpu.pc = 0x5DD02;
    16
}
/// $02A19A: jsr $3060a.l
pub fn i_02a19a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A1A0);
    cpu.pc = 0x3060A;
    16
}
/// $02A1A0: move.l #$10000, $98(a6)
pub fn i_02a1a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1A8;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x10000;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02A1A8: rts
pub fn i_02a1a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1AA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_InitBoss_02a1aa  ($02A1AA, 152 B, slug/helpers_0295a6.s)
/// $02A1AA: jsr $236e.l
pub fn i_02a1aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1B0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A1B0);
    cpu.pc = 0x236E;
    16
}
/// $02A1B0: move.w #$1, d1
pub fn i_02a1b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1B4;
    cpu.ir = 0x323C;
    let v1: u32 = 0x1;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A1B4: jsr $236e.l
pub fn i_02a1b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1BA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A1BA);
    cpu.pc = 0x236E;
    16
}
/// $02A1BA: move.w #$3, $1c(a6)
pub fn i_02a1ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1C0;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x3;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x1C);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A1C0: jsr $138fe.l
pub fn i_02a1c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1C6;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A1C6);
    cpu.pc = 0x138FE;
    16
}
/// $02A1C6: move.w $16(a6), $14(a6)
pub fn i_02a1c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1CC;
    cpu.ir = 0x3D6E;
    let ad2 = cpu.a[6].wrapping_add(0x16);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    let ad4 = cpu.a[6].wrapping_add(0x14);
    bus.write16(ad4, v3 as u16);
    20
}
/// $02A1CC: bset.b #$4, $12(a6)
pub fn i_02a1cc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1D2;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A1D2: bset.b #$2, $5b(a6)
pub fn i_02a1d2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1D8;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((2) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x5B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A1D8: bset.b #$0, $6b(a6)
pub fn i_02a1d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1DE;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((0) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x6B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A1DE: move.b #$ff, $32(a6)
pub fn i_02a1de(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1E4;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x32);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A1E4: move.b #$ff, $33(a6)
pub fn i_02a1e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1EA;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x33);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A1EA: move.b #$0, $3a(a6)
pub fn i_02a1ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1F0;
    cpu.ir = 0x1D7C;
    let v1: u32 = 0x0;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x3A);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A1F0: move.w #$8000, $38(a6)
pub fn i_02a1f0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1F6;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x8000;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x38);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A1F6: ori.w #$c, $38(a6)
pub fn i_02a1f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A1FC;
    cpu.ir = 0x6E;
    let ad1 = cpu.a[6].wrapping_add(0x38);
    let v2 = bus.read16(ad1) as u32;
    let r3 = (v2 | 0xC) & 0xFFFF;
    cpu.sr.set_nz(r3, Size::Word); cpu.sr.0 &= !0x3;
    bus.write16(ad1, r3 as u16);
    16
}
/// $02A1FC: jsr $5e98a.l
pub fn i_02a1fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A202;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A202);
    cpu.pc = 0x5E98A;
    16
}
/// $02A202: move.w #$30, $66(a6)
pub fn i_02a202(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A208;
    cpu.ir = 0x3D7C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[6].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A208: clr.b $8c(a6)
pub fn i_02a208(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A20C;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8C);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A20C: move.l #$295b4, $60(a6)
pub fn i_02a20c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A214;
    cpu.ir = 0x2D7C;
    let v1: u32 = 0x295B4;
    let ad2 = cpu.a[6].wrapping_add(0x60);
    bus.write32(ad2, v1);
    cpu.sr.set_nz(v1, Size::Long); cpu.sr.0 &= !0x3;
    16
}
/// $02A214: jsr $8f6d2.l
pub fn i_02a214(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A21A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A21A);
    cpu.pc = 0x8F6D2;
    16
}
/// $02A21A: clr.w $94(a6)
pub fn i_02a21a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A21E;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x94);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02A21E: clr.w $36(a6)
pub fn i_02a21e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A222;
    cpu.ir = 0x426E;
    let ad1 = cpu.a[6].wrapping_add(0x36);
    let v2 = bus.read16(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write16(ad1, r3 as u16);
    12
}
/// $02A222: clr.b $92(a6)
pub fn i_02a222(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A226;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x92);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A226: clr.b $8f(a6)
pub fn i_02a226(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A22A;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8F);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A22A: bclr.b #$3, $13(a6)
pub fn i_02a22a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A230;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((3) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x13);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A230: lea.l $7773e.l, a1
pub fn i_02a230(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A236;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x7773E;
    20
}
/// $02A236: jsr $4ae.l
pub fn i_02a236(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A23C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A23C);
    cpu.pc = 0x4AE;
    16
}
/// $02A23C: jsr $5dd02.l
pub fn i_02a23c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A242;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A242);
    cpu.pc = 0x5DD02;
    16
}
// ===== JsrAbsThunk_02a242  ($02A242, 8 B, thunks/jsr_abs_thunks.c)
/// $02A242: jsr $3060a.l
pub fn i_02a242(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A248;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A248);
    cpu.pc = 0x3060A;
    16
}
/// $02A248: rts
pub fn i_02a248(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A24A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClrRamWord_02a24a  ($02A24A, 8 B, thunks/clr_ram_word.c)
/// $02A24A: clr.b $106f4b.l
pub fn i_02a24a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A250;
    cpu.ir = 0x4239;
    let v1 = bus.read8(0x106F4B) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r2: u32 = 0;
    bus.write8(0x106F4B, r2 as u8);
    16
}
/// $02A250: rts
pub fn i_02a250(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A252;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_MarkRidden_02a252  ($02A252, 10 B, slug/helpers_0295a6.s)
/// $02A252: move.b #$ff, $106f4b.l
pub fn i_02a252(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A25A;
    cpu.ir = 0x13FC;
    let v1: u32 = 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    bus.write8(0x106F4B, v1 as u8);
    16
}
/// $02A25A: rts
pub fn i_02a25a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A25C;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_IsIdleFlagClear_02a25c  ($02A25C, 20 B, slug/helpers_0295a6.s)
/// $02A25C: cmpi.b #$0, $106ed3.l
pub fn i_02a25c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A264;
    cpu.ir = 0xC39;
    let v2 = bus.read8(0x106ED3) as u32;
    sub_with_flags(cpu, v2, 0x0, Size::Byte, false);
    20
}
/// $02A264: bne.w $2a270
pub fn i_02a264(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A268;
    cpu.ir = 0x6600;
    if cpu.sr.evaluate(Condition::Ne) { cpu.pc = 0x2A270; return 10; }
    8
}
/// $02A268: ori.b #$11, ccr
pub fn i_02a268(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A26C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A26C: bra.w $2a274
pub fn i_02a26c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A270;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A274;
    10
}
// ===== ClearXN_02a270  ($02A270, 6 B, thunks/ccr_helpers.c)
/// $02A270: andi.b #$ee, ccr
pub fn i_02a270(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A274;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A274: rts
pub fn i_02a274(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A276;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TestBit5Field8D_02a276  ($02A276, 18 B, slug/helpers_0295a6.s)
/// $02A276: btst.b #$5, $8d(a6)
pub fn i_02a276(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A27C;
    cpu.ir = 0x82E;
    let mk: u32 = 1u32 << ((5) % 8);
    let ad2 = cpu.a[6].wrapping_add(0x8D);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set(StatusRegister::Z, v3 & mk == 0);
    16
}
/// $02A27C: beq.w $2a288
pub fn i_02a27c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A280;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A288; return 10; }
    8
}
/// $02A280: andi.b #$ee, ccr
pub fn i_02a280(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A284;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A284: bra.w $2a28c
pub fn i_02a284(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A288;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A28C;
    10
}
// ===== SetXN_02a288  ($02A288, 6 B, thunks/ccr_helpers.c)
/// $02A288: ori.b #$11, ccr
pub fn i_02a288(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A28C;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A28C: rts
pub fn i_02a28c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A28E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_AddGaugeFromField98_02a28e  ($02A28E, 30 B, slug/helpers_0295a6.s)
/// $02A28E: lea.l $100580.l, a1
pub fn i_02a28e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A294;
    cpu.ir = 0x43F9;
    cpu.a[1] = 0x100580;
    20
}
/// $02A294: cmpi.b #$ff, $98(a6)
pub fn i_02a294(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A29A;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0xFF, Size::Byte, false);
    16
}
/// $02A29A: beq.w $2a2aa
pub fn i_02a29a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A29E;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A2AA; return 10; }
    8
}
/// $02A29E: move.b $90(a1), d0
pub fn i_02a29e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2A2;
    cpu.ir = 0x1029;
    let ad2 = cpu.a[1].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02A2A2: add.b $98(a6), d0
pub fn i_02a2a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2A6;
    cpu.ir = 0xD02E;
    let ad2 = cpu.a[6].wrapping_add(0x98);
    let v3 = bus.read8(ad2) as u32;
    let h1 = cpu.d[0] & 0xFF;
    let r = add_with_flags(cpu, h1, v3, Size::Byte, true);
    cpu.d[0] = (cpu.d[0] & !0xFF) | (r & 0xFF);
    12
}
/// $02A2A6: move.b d0, $90(a1)
pub fn i_02a2a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2AA;
    cpu.ir = 0x1340;
    let v1 = cpu.d[0] & 0xFF;
    cpu.sr.set_nz(v1, Size::Byte); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[1].wrapping_add(0x90);
    bus.write8(ad2, v1 as u8);
    12
}
/// $02A2AA: rts
pub fn i_02a2aa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2AC;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_GetGauge_02a2ac  ($02A2AC, 14 B, slug/helpers_0295a6.s)
/// $02A2AC: lea.l $100580.l, a0
pub fn i_02a2ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2B2;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02A2B2: moveq #$0, d0
pub fn i_02a2b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2B4;
    cpu.ir = 0x7000;
    cpu.d[0] = 0x0;
    cpu.sr.set_nz(cpu.d[0], Size::Long); cpu.sr.0 &= !0x3;
    4
}
/// $02A2B4: move.b $90(a0), d0
pub fn i_02a2b4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2B8;
    cpu.ir = 0x1028;
    let ad2 = cpu.a[0].wrapping_add(0x90);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFF) | (v3 & 0xFF);
    12
}
/// $02A2B8: rts
pub fn i_02a2b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2BA;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_AddHP_02a2ba  ($02A2BA, 56 B, slug/helpers_0295a6.s)
/// $02A2BA: lea.l $100580.l, a0
pub fn i_02a2ba(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2C0;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02A2C0: cmpi.w #$30, $66(a0)
pub fn i_02a2c0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2C6;
    cpu.ir = 0xC68;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    sub_with_flags(cpu, v3, 0x30, Size::Word, false);
    16
}
/// $02A2C6: bge.w $2a2f2
pub fn i_02a2c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2CA;
    cpu.ir = 0x6C00;
    if cpu.sr.evaluate(Condition::Ge) { cpu.pc = 0x2A2F2; return 10; }
    8
}
/// $02A2CA: move.w $66(a0), d1
pub fn i_02a2ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2CE;
    cpu.ir = 0x3228;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A2CE: add.w d0, d1
pub fn i_02a2ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2D0;
    cpu.ir = 0xD240;
    let v1 = cpu.d[0] & 0xFFFF;
    let h1 = cpu.d[1] & 0xFFFF;
    let r = add_with_flags(cpu, h1, v1, Size::Word, true);
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (r & 0xFFFF);
    4
}
/// $02A2D0: cmpi.w #$30, d1
pub fn i_02a2d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2D4;
    cpu.ir = 0xC41;
    let v1 = cpu.d[1] & 0xFFFF;
    sub_with_flags(cpu, v1, 0x30, Size::Word, false);
    8
}
/// $02A2D4: ble.w $2a2dc
pub fn i_02a2d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2D8;
    cpu.ir = 0x6F00;
    if cpu.sr.evaluate(Condition::Le) { cpu.pc = 0x2A2DC; return 10; }
    8
}
/// $02A2D8: move.w #$30, d1
pub fn i_02a2d8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2DC;
    cpu.ir = 0x323C;
    let v1: u32 = 0x30;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[1] = (cpu.d[1] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A2DC: move.w d1, $66(a0)
pub fn i_02a2dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2E0;
    cpu.ir = 0x3141;
    let v1 = cpu.d[1] & 0xFFFF;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    let ad2 = cpu.a[0].wrapping_add(0x66);
    bus.write16(ad2, v1 as u16);
    12
}
/// $02A2E0: move.w #$1083, d0
pub fn i_02a2e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2E4;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1083;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A2E4: jsr $2352.l
pub fn i_02a2e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2EA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A2EA);
    cpu.pc = 0x2352;
    16
}
/// $02A2EA: andi.b #$ee, ccr
pub fn i_02a2ea(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2EE;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A2EE: bra.w $2a2f6
pub fn i_02a2ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2F2;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A2F6;
    10
}
// ===== SetXN_02a2f2  ($02A2F2, 6 B, thunks/ccr_helpers.c)
/// $02A2F2: ori.b #$11, ccr
pub fn i_02a2f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2F6;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A2F6: rts
pub fn i_02a2f6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2F8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_StopMusic108F_02a2f8  ($02A2F8, 24 B, slug/helpers_0295a6.s)
/// $02A2F8: tst.b $8b(a6)
pub fn i_02a2f8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A2FC;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02A2FC: beq.w $2a30e
pub fn i_02a2fc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A300;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A30E; return 10; }
    8
}
/// $02A300: move.w #$108f, d0
pub fn i_02a300(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A304;
    cpu.ir = 0x303C;
    let v1: u32 = 0x108F;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A304: jsr $2352.l
pub fn i_02a304(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A30A;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A30A);
    cpu.pc = 0x2352;
    16
}
/// $02A30A: clr.b $8b(a6)
pub fn i_02a30a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A30E;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A30E: rts
pub fn i_02a30e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A310;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_StopMusic1092_02a310  ($02A310, 24 B, slug/helpers_0295a6.s)
/// $02A310: tst.b $8b(a6)
pub fn i_02a310(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A314;
    cpu.ir = 0x4A2E;
    let ad2 = cpu.a[6].wrapping_add(0x8B);
    let v3 = bus.read8(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Byte); cpu.sr.0 &= !0x3;
    12
}
/// $02A314: beq.w $2a326
pub fn i_02a314(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A318;
    cpu.ir = 0x6700;
    if cpu.sr.evaluate(Condition::Eq) { cpu.pc = 0x2A326; return 10; }
    8
}
/// $02A318: move.w #$1092, d0
pub fn i_02a318(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A31C;
    cpu.ir = 0x303C;
    let v1: u32 = 0x1092;
    cpu.sr.set_nz(v1, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v1 & 0xFFFF);
    4
}
/// $02A31C: jsr $2352.l
pub fn i_02a31c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A322;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A322);
    cpu.pc = 0x2352;
    16
}
/// $02A322: clr.b $8b(a6)
pub fn i_02a322(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A326;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x8B);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A326: rts
pub fn i_02a326(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A328;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CallGroundProbeA_02a328  ($02A328, 18 B, slug/helpers_0295a6.s)
/// $02A328: move.w $94(a6), d0
pub fn i_02a328(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A32C;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A32C: asl.w #$2, d0
pub fn i_02a32c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A32E;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02A32E: lea.l $2a33a(pc), a0
pub fn i_02a32e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A332;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A33A;
    16
}
/// $02A332: movea.l (a0, d0.w), a0
pub fn i_02a332(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A336;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02A336: jsr (a0)
pub fn i_02a336(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A338;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x2A338);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $02A338: rts
pub fn i_02a338(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A33A;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CallGroundProbeB_02a34e  ($02A34E, 18 B, slug/helpers_0295a6.s)
/// $02A34E: move.w $94(a6), d0
pub fn i_02a34e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A352;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A352: asl.w #$2, d0
pub fn i_02a352(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A354;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02A354: lea.l $2a360(pc), a0
pub fn i_02a354(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A358;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A360;
    16
}
/// $02A358: movea.l (a0, d0.w), a0
pub fn i_02a358(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A35C;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02A35C: jsr (a0)
pub fn i_02a35c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A35E;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x2A35E);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $02A35E: rts
pub fn i_02a35e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A360;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_CallGroundProbeC_02a374  ($02A374, 18 B, slug/helpers_0295a6.s)
/// $02A374: move.w $94(a6), d0
pub fn i_02a374(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A378;
    cpu.ir = 0x302E;
    let ad2 = cpu.a[6].wrapping_add(0x94);
    let v3 = bus.read16(ad2) as u32;
    cpu.sr.set_nz(v3, Size::Word); cpu.sr.0 &= !0x3;
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (v3 & 0xFFFF);
    12
}
/// $02A378: asl.w #$2, d0
pub fn i_02a378(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A37A;
    cpu.ir = 0xE540;
    let h1 = cpu.d[0] & 0xFFFF;
    let r = do_shift_rotate(cpu, h1, 2, Size::Word, 0, true);
    cpu.d[0] = (cpu.d[0] & !0xFFFF) | (r & 0xFFFF);
    10
}
/// $02A37A: lea.l $2a386(pc), a0
pub fn i_02a37a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A37E;
    cpu.ir = 0x41FA;
    cpu.a[0] = 0x2A386;
    16
}
/// $02A37E: movea.l (a0, d0.w), a0
pub fn i_02a37e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A382;
    cpu.ir = 0x2070;
    let ad2 = cpu.a[0].wrapping_add(0x0).wrapping_add((cpu.d[0] as i16 as i32 as u32));
    let v3 = bus.read32(ad2);
    cpu.a[0] = v3;
    18
}
/// $02A382: jsr (a0)
pub fn i_02a382(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A384;
    cpu.ir = 0x4E90;
    cpu.push32(bus, 0x2A384);
    cpu.jump_to(cpu.a[0]);
    16
}
/// $02A384: rts
pub fn i_02a384(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A386;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ProbeFrontThenGround_02a39a  ($02A39A, 30 B, slug/helpers_0295a6.s)
/// $02A39A: jsr $5cf84.l
pub fn i_02a39a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3A0;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A3A0);
    cpu.pc = 0x5CF84;
    16
}
/// $02A3A0: bcc.w $2a3a6
pub fn i_02a3a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3A4;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A3A6; return 10; }
    8
}
/// $02A3A4: rts
pub fn i_02a3a4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3A6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
/// $02A3A6: jsr $5ceec.l
pub fn i_02a3a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3AC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A3AC);
    cpu.pc = 0x5CEEC;
    16
}
/// $02A3AC: bcc.w $2a3b6
pub fn i_02a3ac(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3B0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A3B6; return 10; }
    8
}
/// $02A3B0: jmp $5cf9c.l
pub fn i_02a3b0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3B6;
    cpu.ir = 0x4EF9;
    cpu.pc = 0x5CF9C;
    8
}
/// $02A3B6: rts
pub fn i_02a3b6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3B8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ProbeFrontThenA_02a3b8  ($02A3B8, 18 B, slug/helpers_0295a6.s)
/// $02A3B8: jsr $5cf84.l
pub fn i_02a3b8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3BE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A3BE);
    cpu.pc = 0x5CF84;
    16
}
/// $02A3BE: bcc.w $2a3d0
pub fn i_02a3be(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3C2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A3D0; return 10; }
    8
}
/// $02A3C2: jsr $2a328(pc)
pub fn i_02a3c2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3C6;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A3C6);
    cpu.pc = 0x2A328;
    16
}
/// $02A3C6: bcs.w $2a3d0
pub fn i_02a3c6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3CA;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A3D0; return 10; }
    8
}
// ===== SetXN_02a3ca  ($02A3CA, 6 B, thunks/ccr_helpers.c)
/// $02A3CA: ori.b #$11, ccr
pub fn i_02a3ca(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3CE;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A3CE: rts
pub fn i_02a3ce(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3D0;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a3d0  ($02A3D0, 6 B, thunks/ccr_helpers.c)
/// $02A3D0: andi.b #$ee, ccr
pub fn i_02a3d0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3D4;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A3D4: rts
pub fn i_02a3d4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3D6;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ProbeFrontThenB_02a3d6  ($02A3D6, 18 B, slug/helpers_0295a6.s)
/// $02A3D6: jsr $5cf84.l
pub fn i_02a3d6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3DC;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A3DC);
    cpu.pc = 0x5CF84;
    16
}
/// $02A3DC: bcc.w $2a3ee
pub fn i_02a3dc(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3E0;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A3EE; return 10; }
    8
}
/// $02A3E0: jsr $2a34e(pc)
pub fn i_02a3e0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3E4;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A3E4);
    cpu.pc = 0x2A34E;
    16
}
/// $02A3E4: bcs.w $2a3ee
pub fn i_02a3e4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3E8;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A3EE; return 10; }
    8
}
// ===== SetXN_02a3e8  ($02A3E8, 6 B, thunks/ccr_helpers.c)
/// $02A3E8: ori.b #$11, ccr
pub fn i_02a3e8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3EC;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A3EC: rts
pub fn i_02a3ec(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3EE;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a3ee  ($02A3EE, 6 B, thunks/ccr_helpers.c)
/// $02A3EE: andi.b #$ee, ccr
pub fn i_02a3ee(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3F2;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A3F2: rts
pub fn i_02a3f2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3F4;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ProbeLeftWall_02a3f4  ($02A3F4, 30 B, slug/helpers_0295a6.s)
/// $02A3F4: jsr $5cf2c.l
pub fn i_02a3f4(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3FA;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A3FA);
    cpu.pc = 0x5CF2C;
    16
}
/// $02A3FA: bcc.w $2a418
pub fn i_02a3fa(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A3FE;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A3FE: jsr $5cf5c.l
pub fn i_02a3fe(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A404;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A404);
    cpu.pc = 0x5CF5C;
    16
}
/// $02A404: bcs.w $2a418
pub fn i_02a404(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A408;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A408: jsr $5d140.l
pub fn i_02a408(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A40E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A40E);
    cpu.pc = 0x5D140;
    16
}
/// $02A40E: bcs.w $2a418
pub fn i_02a40e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A412;
    cpu.ir = 0x6500;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A418; return 10; }
    8
}
// ===== SetXN_02a412  ($02A412, 6 B, thunks/ccr_helpers.c)
/// $02A412: ori.b #$11, ccr
pub fn i_02a412(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A416;
    cpu.ir = 0x3C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) | 0x11);
    20
}
/// $02A416: rts
pub fn i_02a416(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A418;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== ClearXN_02a418  ($02A418, 6 B, thunks/ccr_helpers.c)
/// $02A418: andi.b #$ee, ccr
pub fn i_02a418(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A41C;
    cpu.ir = 0x23C;
    cpu.sr.0 = (cpu.sr.0 & 0xFFE0) | ((cpu.sr.0 & 0x1F) & 0xE);
    20
}
/// $02A41C: rts
pub fn i_02a41c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A41E;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ProbeRightWall_02a41e  ($02A41E, 26 B, slug/helpers_0295a6.s)
/// $02A41E: jsr $5cf3c.l
pub fn i_02a41e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A424;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A424);
    cpu.pc = 0x5CF3C;
    16
}
/// $02A424: bcc.b $2a418
pub fn i_02a424(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A426;
    cpu.ir = 0x64F2;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A426: jsr $5cf4c.l
pub fn i_02a426(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A42C;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A42C);
    cpu.pc = 0x5CF4C;
    16
}
/// $02A42C: bcs.b $2a418
pub fn i_02a42c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A42E;
    cpu.ir = 0x65EA;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A42E: jsr $5d14c.l
pub fn i_02a42e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A434;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A434);
    cpu.pc = 0x5D14C;
    16
}
/// $02A434: bcs.b $2a418
pub fn i_02a434(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A436;
    cpu.ir = 0x65E2;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A436: bra.b $2a412
pub fn i_02a436(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A438;
    cpu.ir = 0x60DA;
    cpu.pc = 0x2A412;
    10
}
// ===== Slug_ProbeRightWallB_02a438  ($02A438, 26 B, slug/helpers_0295a6.s)
/// $02A438: jsr $5cf4c.l
pub fn i_02a438(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A43E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A43E);
    cpu.pc = 0x5CF4C;
    16
}
/// $02A43E: bcc.b $2a418
pub fn i_02a43e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A440;
    cpu.ir = 0x64D8;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A440: jsr $5cf3c.l
pub fn i_02a440(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A446;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A446);
    cpu.pc = 0x5CF3C;
    16
}
/// $02A446: bcs.b $2a418
pub fn i_02a446(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A448;
    cpu.ir = 0x65D0;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A448: jsr $5d140.l
pub fn i_02a448(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A44E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A44E);
    cpu.pc = 0x5D140;
    16
}
/// $02A44E: bcs.b $2a418
pub fn i_02a44e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A450;
    cpu.ir = 0x65C8;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A450: bra.b $2a412
pub fn i_02a450(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A452;
    cpu.ir = 0x60C0;
    cpu.pc = 0x2A412;
    10
}
// ===== Slug_ProbeLeftWallB_02a452  ($02A452, 26 B, slug/helpers_0295a6.s)
/// $02A452: jsr $5cf5c.l
pub fn i_02a452(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A458;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A458);
    cpu.pc = 0x5CF5C;
    16
}
/// $02A458: bcc.b $2a418
pub fn i_02a458(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A45A;
    cpu.ir = 0x64BE;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A45A: jsr $5cf2c.l
pub fn i_02a45a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A460;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A460);
    cpu.pc = 0x5CF2C;
    16
}
/// $02A460: bcs.b $2a418
pub fn i_02a460(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A462;
    cpu.ir = 0x65B6;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A462: jsr $5d14c.l
pub fn i_02a462(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A468;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A468);
    cpu.pc = 0x5D14C;
    16
}
/// $02A468: bcs.b $2a418
pub fn i_02a468(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A46A;
    cpu.ir = 0x65AE;
    if cpu.sr.evaluate(Condition::Cs) { cpu.pc = 0x2A418; return 10; }
    8
}
/// $02A46A: bra.b $2a412
pub fn i_02a46a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A46C;
    cpu.ir = 0x60A6;
    cpu.pc = 0x2A412;
    10
}
// ===== Slug_ClearFlags8D_02a46c  ($02A46C, 12 B, slug/helpers_0295a6.s)
/// $02A46C: lea.l $100580.l, a0
pub fn i_02a46c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A472;
    cpu.ir = 0x41F9;
    cpu.a[0] = 0x100580;
    20
}
/// $02A472: clr.b $8d(a0)
pub fn i_02a472(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A476;
    cpu.ir = 0x4228;
    let ad1 = cpu.a[0].wrapping_add(0x8D);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A476: rts
pub fn i_02a476(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A478;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_UpdateAirFlag_02a478  ($02A478, 42 B, slug/helpers_0295a6.s)
/// $02A478: jsr $5cf1c.l
pub fn i_02a478(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A47E;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A47E);
    cpu.pc = 0x5CF1C;
    16
}
/// $02A47E: bcc.w $2a48c
pub fn i_02a47e(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A482;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A48C; return 10; }
    8
}
/// $02A482: bset.b #$4, $12(a6)
pub fn i_02a482(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A488;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A488: bra.w $2a4a0
pub fn i_02a488(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A48C;
    cpu.ir = 0x6000;
    cpu.pc = 0x2A4A0;
    10
}
/// $02A48C: bclr.b #$4, $12(a6)
pub fn i_02a48c(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A492;
    cpu.ir = 0x8AE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 & !mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A492: jsr $2abcc(pc)
pub fn i_02a492(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A496;
    cpu.ir = 0x4EBA;
    cpu.push32(bus, 0x2A496);
    cpu.pc = 0x2ABCC;
    16
}
/// $02A496: bcc.w $2a4a0
pub fn i_02a496(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A49A;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A4A0; return 10; }
    8
}
/// $02A49A: bset.b #$4, $12(a6)
pub fn i_02a49a(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A0;
    cpu.ir = 0x8EE;
    let mk: u32 = 1u32 << ((4) % 8);
    let ad1 = cpu.a[6].wrapping_add(0x12);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.set(StatusRegister::Z, v2 & mk == 0);
    let r3 = v2 | mk;
    bus.write8(ad1, r3 as u8);
    16
}
/// $02A4A0: rts
pub fn i_02a4a0(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A2;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_ResetTurnTimer_02a4a2  ($02A4A2, 6 B, slug/helpers_0295a6.s)
/// $02A4A2: clr.b $84(a6)
pub fn i_02a4a2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A6;
    cpu.ir = 0x422E;
    let ad1 = cpu.a[6].wrapping_add(0x84);
    let v2 = bus.read8(ad1) as u32;
    cpu.sr.0 = (cpu.sr.0 & !0xF) | 0x4; let r3: u32 = 0;
    bus.write8(ad1, r3 as u8);
    12
}
/// $02A4A6: rts
pub fn i_02a4a6(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4A8;
    cpu.ir = 0x4E75;
    let r = cpu.pop32(bus); cpu.jump_to(r);
    16
}
// ===== Slug_TurnTimerTick_02a4a8  ($02A4A8, 20 B, slug/helpers_0295a6.s)
/// $02A4A8: jsr $5cd60.l
pub fn i_02a4a8(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4AE;
    cpu.ir = 0x4EB9;
    cpu.push32(bus, 0x2A4AE);
    cpu.pc = 0x5CD60;
    16
}
/// $02A4AE: bcc.w $2a4ce
pub fn i_02a4ae(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4B2;
    cpu.ir = 0x6400;
    if cpu.sr.evaluate(Condition::Cc) { cpu.pc = 0x2A4CE; return 10; }
    8
}
/// $02A4B2: cmpi.b #$14, $84(a6)
pub fn i_02a4b2(cpu: &mut Cpu, bus: &mut B) -> u32 {
    cpu.pc = 0x2A4B8;
    cpu.ir = 0xC2E;
    let ad2 = cpu.a[6].wrapping_add(0x84);
    let v3 = bus.read8(ad2) as u32;
    sub_with_flags(cpu, v3, 0x14, Size::Byte, false);
    16
}

pub const TABLE: &[(u32, crate::InsFn)] = &[
    (0x0280C0, i_0280c0),
    (0x0280C4, i_0280c4),
    (0x0280C6, i_0280c6),
    (0x0280CA, i_0280ca),
    (0x0280D0, i_0280d0),
    (0x0280D6, i_0280d6),
    (0x0280DA, i_0280da),
    (0x0280DE, i_0280de),
    (0x0280E0, i_0280e0),
    (0x0280E2, i_0280e2),
    (0x0280E4, i_0280e4),
    (0x0280E6, i_0280e6),
    (0x0280EC, i_0280ec),
    (0x0280F0, i_0280f0),
    (0x0280F4, i_0280f4),
    (0x0280F8, i_0280f8),
    (0x0280FC, i_0280fc),
    (0x028100, i_028100),
    (0x028102, i_028102),
    (0x028106, i_028106),
    (0x028108, i_028108),
    (0x02810C, i_02810c),
    (0x028110, i_028110),
    (0x028114, i_028114),
    (0x028118, i_028118),
    (0x02811C, i_02811c),
    (0x028120, i_028120),
    (0x028124, i_028124),
    (0x028126, i_028126),
    (0x028128, i_028128),
    (0x02812E, i_02812e),
    (0x028132, i_028132),
    (0x028134, i_028134),
    (0x028138, i_028138),
    (0x02813C, i_02813c),
    (0x028140, i_028140),
    (0x028144, i_028144),
    (0x028146, i_028146),
    (0x028148, i_028148),
    (0x02814E, i_02814e),
    (0x028150, i_028150),
    (0x028154, i_028154),
    (0x028158, i_028158),
    (0x02815A, i_02815a),
    (0x02815E, i_02815e),
    (0x028162, i_028162),
    (0x028166, i_028166),
    (0x02816A, i_02816a),
    (0x02816E, i_02816e),
    (0x028172, i_028172),
    (0x028176, i_028176),
    (0x02817A, i_02817a),
    (0x02817E, i_02817e),
    (0x028182, i_028182),
    (0x028184, i_028184),
    (0x028188, i_028188),
    (0x02818A, i_02818a),
    (0x02818E, i_02818e),
    (0x028190, i_028190),
    (0x028194, i_028194),
    (0x028196, i_028196),
    (0x02819A, i_02819a),
    (0x02819C, i_02819c),
    (0x0281A2, i_0281a2),
    (0x0281A8, i_0281a8),
    (0x0281AE, i_0281ae),
    (0x0281B0, i_0281b0),
    (0x0281B6, i_0281b6),
    (0x0281BA, i_0281ba),
    (0x0281BC, i_0281bc),
    (0x0281C2, i_0281c2),
    (0x0281C6, i_0281c6),
    (0x0281C8, i_0281c8),
    (0x0281CE, i_0281ce),
    (0x0281D2, i_0281d2),
    (0x0281D6, i_0281d6),
    (0x0281DA, i_0281da),
    (0x0281DC, i_0281dc),
    (0x0281DE, i_0281de),
    (0x0281E2, i_0281e2),
    (0x0281E6, i_0281e6),
    (0x0281E8, i_0281e8),
    (0x0281EA, i_0281ea),
    (0x0281EC, i_0281ec),
    (0x0281F0, i_0281f0),
    (0x0281F4, i_0281f4),
    (0x0281F8, i_0281f8),
    (0x0281FC, i_0281fc),
    (0x028200, i_028200),
    (0x028204, i_028204),
    (0x028208, i_028208),
    (0x02820A, i_02820a),
    (0x02820C, i_02820c),
    (0x028210, i_028210),
    (0x028214, i_028214),
    (0x028216, i_028216),
    (0x028218, i_028218),
    (0x02821A, i_02821a),
    (0x02821E, i_02821e),
    (0x028222, i_028222),
    (0x028226, i_028226),
    (0x02822A, i_02822a),
    (0x02822C, i_02822c),
    (0x028230, i_028230),
    (0x028234, i_028234),
    (0x028238, i_028238),
    (0x02823A, i_02823a),
    (0x02823E, i_02823e),
    (0x028240, i_028240),
    (0x028244, i_028244),
    (0x028248, i_028248),
    (0x02824C, i_02824c),
    (0x028250, i_028250),
    (0x028252, i_028252),
    (0x028256, i_028256),
    (0x02825A, i_02825a),
    (0x02825E, i_02825e),
    (0x028260, i_028260),
    (0x028262, i_028262),
    (0x028264, i_028264),
    (0x028268, i_028268),
    (0x02826C, i_02826c),
    (0x028270, i_028270),
    (0x028274, i_028274),
    (0x028276, i_028276),
    (0x028278, i_028278),
    (0x02827A, i_02827a),
    (0x02827E, i_02827e),
    (0x028282, i_028282),
    (0x028286, i_028286),
    (0x028288, i_028288),
    (0x02828C, i_02828c),
    (0x028290, i_028290),
    (0x028292, i_028292),
    (0x028298, i_028298),
    (0x02829C, i_02829c),
    (0x0282A4, i_0282a4),
    (0x0282A8, i_0282a8),
    (0x0282AE, i_0282ae),
    (0x0282B2, i_0282b2),
    (0x0282B6, i_0282b6),
    (0x0282BC, i_0282bc),
    (0x0282C0, i_0282c0),
    (0x0282C4, i_0282c4),
    (0x0282C8, i_0282c8),
    (0x0282CE, i_0282ce),
    (0x0282D2, i_0282d2),
    (0x0282D6, i_0282d6),
    (0x0282D8, i_0282d8),
    (0x0282DE, i_0282de),
    (0x0282E2, i_0282e2),
    (0x0282EA, i_0282ea),
    (0x0282EE, i_0282ee),
    (0x0282F4, i_0282f4),
    (0x0282F8, i_0282f8),
    (0x0282FC, i_0282fc),
    (0x028302, i_028302),
    (0x028306, i_028306),
    (0x02830A, i_02830a),
    (0x02830E, i_02830e),
    (0x028314, i_028314),
    (0x028318, i_028318),
    (0x02831C, i_02831c),
    (0x02831E, i_02831e),
    (0x028324, i_028324),
    (0x028328, i_028328),
    (0x028330, i_028330),
    (0x028334, i_028334),
    (0x02833A, i_02833a),
    (0x02833E, i_02833e),
    (0x028342, i_028342),
    (0x028348, i_028348),
    (0x02834C, i_02834c),
    (0x028350, i_028350),
    (0x028354, i_028354),
    (0x02835A, i_02835a),
    (0x02835E, i_02835e),
    (0x028362, i_028362),
    (0x028364, i_028364),
    (0x02836A, i_02836a),
    (0x02836E, i_02836e),
    (0x028376, i_028376),
    (0x02837A, i_02837a),
    (0x028380, i_028380),
    (0x028384, i_028384),
    (0x028388, i_028388),
    (0x02838E, i_02838e),
    (0x028392, i_028392),
    (0x028396, i_028396),
    (0x02839A, i_02839a),
    (0x0283A0, i_0283a0),
    (0x0283A4, i_0283a4),
    (0x0283A8, i_0283a8),
    (0x0283AA, i_0283aa),
    (0x0283AC, i_0283ac),
    (0x0283AE, i_0283ae),
    (0x0283B2, i_0283b2),
    (0x0283B6, i_0283b6),
    (0x0283BA, i_0283ba),
    (0x0283BE, i_0283be),
    (0x0283C2, i_0283c2),
    (0x0283C4, i_0283c4),
    (0x0283C8, i_0283c8),
    (0x0283CA, i_0283ca),
    (0x0283D0, i_0283d0),
    (0x0283D6, i_0283d6),
    (0x0283D8, i_0283d8),
    (0x0283DA, i_0283da),
    (0x0283DE, i_0283de),
    (0x0283E6, i_0283e6),
    (0x0283EA, i_0283ea),
    (0x0283EC, i_0283ec),
    (0x0283EE, i_0283ee),
    (0x0283F2, i_0283f2),
    (0x0283F6, i_0283f6),
    (0x0283FC, i_0283fc),
    (0x0283FE, i_0283fe),
    (0x028402, i_028402),
    (0x028406, i_028406),
    (0x028408, i_028408),
    (0x02840A, i_02840a),
    (0x02840E, i_02840e),
    (0x028410, i_028410),
    (0x028412, i_028412),
    (0x028414, i_028414),
    (0x028416, i_028416),
    (0x02841A, i_02841a),
    (0x02841E, i_02841e),
    (0x028420, i_028420),
    (0x028426, i_028426),
    (0x02843C, i_02843c),
    (0x028442, i_028442),
    (0x028446, i_028446),
    (0x02844C, i_02844c),
    (0x028450, i_028450),
    (0x028456, i_028456),
    (0x02845A, i_02845a),
    (0x02845C, i_02845c),
    (0x02845E, i_02845e),
    (0x028464, i_028464),
    (0x028468, i_028468),
    (0x02846E, i_02846e),
    (0x028472, i_028472),
    (0x028478, i_028478),
    (0x02847C, i_02847c),
    (0x02847E, i_02847e),
    (0x028482, i_028482),
    (0x028488, i_028488),
    (0x02848C, i_02848c),
    (0x02848E, i_02848e),
    (0x028490, i_028490),
    (0x028496, i_028496),
    (0x02849A, i_02849a),
    (0x0284A0, i_0284a0),
    (0x0284A4, i_0284a4),
    (0x0284AA, i_0284aa),
    (0x0284AE, i_0284ae),
    (0x0284B0, i_0284b0),
    (0x0284B4, i_0284b4),
    (0x0284BA, i_0284ba),
    (0x0284BE, i_0284be),
    (0x0284C0, i_0284c0),
    (0x0284C2, i_0284c2),
    (0x0284C8, i_0284c8),
    (0x0284CC, i_0284cc),
    (0x0284D0, i_0284d0),
    (0x0284D6, i_0284d6),
    (0x0284DA, i_0284da),
    (0x0284E0, i_0284e0),
    (0x0284E4, i_0284e4),
    (0x0284EA, i_0284ea),
    (0x0284EE, i_0284ee),
    (0x0284F2, i_0284f2),
    (0x0284F4, i_0284f4),
    (0x0284F8, i_0284f8),
    (0x0284FE, i_0284fe),
    (0x028500, i_028500),
    (0x028502, i_028502),
    (0x028504, i_028504),
    (0x028508, i_028508),
    (0x02850E, i_02850e),
    (0x028512, i_028512),
    (0x028514, i_028514),
    (0x028516, i_028516),
    (0x028518, i_028518),
    (0x02851E, i_02851e),
    (0x028522, i_028522),
    (0x028526, i_028526),
    (0x02852C, i_02852c),
    (0x028530, i_028530),
    (0x028536, i_028536),
    (0x02853A, i_02853a),
    (0x028540, i_028540),
    (0x028544, i_028544),
    (0x028548, i_028548),
    (0x02854A, i_02854a),
    (0x02854E, i_02854e),
    (0x028554, i_028554),
    (0x028556, i_028556),
    (0x028558, i_028558),
    (0x02855A, i_02855a),
    (0x02855E, i_02855e),
    (0x028564, i_028564),
    (0x028568, i_028568),
    (0x02856A, i_02856a),
    (0x02856C, i_02856c),
    (0x028576, i_028576),
    (0x028578, i_028578),
    (0x02857C, i_02857c),
    (0x028580, i_028580),
    (0x028586, i_028586),
    (0x02858A, i_02858a),
    (0x028590, i_028590),
    (0x028594, i_028594),
    (0x02859A, i_02859a),
    (0x02859E, i_02859e),
    (0x0285A4, i_0285a4),
    (0x0285A8, i_0285a8),
    (0x0285AC, i_0285ac),
    (0x0285B2, i_0285b2),
    (0x0285BA, i_0285ba),
    (0x0285BE, i_0285be),
    (0x0285C2, i_0285c2),
    (0x0285C8, i_0285c8),
    (0x0285CC, i_0285cc),
    (0x0285D0, i_0285d0),
    (0x0285D4, i_0285d4),
    (0x0285D8, i_0285d8),
    (0x0285DC, i_0285dc),
    (0x0285E0, i_0285e0),
    (0x0285E4, i_0285e4),
    (0x0285E8, i_0285e8),
    (0x0285EA, i_0285ea),
    (0x0285EC, i_0285ec),
    (0x0285F0, i_0285f0),
    (0x0285F2, i_0285f2),
    (0x0285F4, i_0285f4),
    (0x0285F8, i_0285f8),
    (0x0285FC, i_0285fc),
    (0x028602, i_028602),
    (0x028604, i_028604),
    (0x028608, i_028608),
    (0x02860E, i_02860e),
    (0x028614, i_028614),
    (0x028618, i_028618),
    (0x02861E, i_02861e),
    (0x028624, i_028624),
    (0x028628, i_028628),
    (0x02862C, i_02862c),
    (0x028630, i_028630),
    (0x028634, i_028634),
    (0x028638, i_028638),
    (0x02863C, i_02863c),
    (0x028640, i_028640),
    (0x028644, i_028644),
    (0x028648, i_028648),
    (0x02864C, i_02864c),
    (0x028650, i_028650),
    (0x028652, i_028652),
    (0x028654, i_028654),
    (0x028658, i_028658),
    (0x02865A, i_02865a),
    (0x02865C, i_02865c),
    (0x028660, i_028660),
    (0x028664, i_028664),
    (0x02866A, i_02866a),
    (0x02866E, i_02866e),
    (0x028672, i_028672),
    (0x028676, i_028676),
    (0x02867C, i_02867c),
    (0x028682, i_028682),
    (0x028686, i_028686),
    (0x02868A, i_02868a),
    (0x02868E, i_02868e),
    (0x028690, i_028690),
    (0x028694, i_028694),
    (0x028698, i_028698),
    (0x02869C, i_02869c),
    (0x0286A0, i_0286a0),
    (0x0286A2, i_0286a2),
    (0x0286A6, i_0286a6),
    (0x0286AC, i_0286ac),
    (0x0286B2, i_0286b2),
    (0x0286B8, i_0286b8),
    (0x0286BE, i_0286be),
    (0x0286C2, i_0286c2),
    (0x0286C4, i_0286c4),
    (0x0286CA, i_0286ca),
    (0x0286D0, i_0286d0),
    (0x0286D4, i_0286d4),
    (0x0286DA, i_0286da),
    (0x0286E0, i_0286e0),
    (0x0286E4, i_0286e4),
    (0x0286E8, i_0286e8),
    (0x0286EC, i_0286ec),
    (0x0286F0, i_0286f0),
    (0x0286F2, i_0286f2),
    (0x0286F6, i_0286f6),
    (0x0286FC, i_0286fc),
    (0x0286FE, i_0286fe),
    (0x028702, i_028702),
    (0x028708, i_028708),
    (0x02870A, i_02870a),
    (0x028710, i_028710),
    (0x028714, i_028714),
    (0x028716, i_028716),
    (0x02871A, i_02871a),
    (0x02871C, i_02871c),
    (0x028720, i_028720),
    (0x028724, i_028724),
    (0x02872A, i_02872a),
    (0x02872E, i_02872e),
    (0x028734, i_028734),
    (0x028738, i_028738),
    (0x02873A, i_02873a),
    (0x02873E, i_02873e),
    (0x028742, i_028742),
    (0x028746, i_028746),
    (0x028748, i_028748),
    (0x02874A, i_02874a),
    (0x02874C, i_02874c),
    (0x02874E, i_02874e),
    (0x028752, i_028752),
    (0x028756, i_028756),
    (0x028758, i_028758),
    (0x02875E, i_02875e),
    (0x028762, i_028762),
    (0x028766, i_028766),
    (0x028768, i_028768),
    (0x02876C, i_02876c),
    (0x02876E, i_02876e),
    (0x028770, i_028770),
    (0x028774, i_028774),
    (0x02877A, i_02877a),
    (0x02877E, i_02877e),
    (0x028780, i_028780),
    (0x028782, i_028782),
    (0x028784, i_028784),
    (0x028786, i_028786),
    (0x02878A, i_02878a),
    (0x02878C, i_02878c),
    (0x02878E, i_02878e),
    (0x028790, i_028790),
    (0x028792, i_028792),
    (0x028794, i_028794),
    (0x028796, i_028796),
    (0x02879A, i_02879a),
    (0x0287A0, i_0287a0),
    (0x0287A4, i_0287a4),
    (0x0287A6, i_0287a6),
    (0x0287A8, i_0287a8),
    (0x0287AA, i_0287aa),
    (0x0287AC, i_0287ac),
    (0x0287B0, i_0287b0),
    (0x0287B2, i_0287b2),
    (0x0287B4, i_0287b4),
    (0x0287B6, i_0287b6),
    (0x0287B8, i_0287b8),
    (0x0287BA, i_0287ba),
    (0x0287BE, i_0287be),
    (0x0287C2, i_0287c2),
    (0x0287C6, i_0287c6),
    (0x0287CA, i_0287ca),
    (0x0287CC, i_0287cc),
    (0x0287D0, i_0287d0),
    (0x0287D2, i_0287d2),
    (0x0287D6, i_0287d6),
    (0x0287DA, i_0287da),
    (0x0287DC, i_0287dc),
    (0x0287DE, i_0287de),
    (0x0287E2, i_0287e2),
    (0x0287E4, i_0287e4),
    (0x0287E6, i_0287e6),
    (0x0287EA, i_0287ea),
    (0x0287EC, i_0287ec),
    (0x0287EE, i_0287ee),
    (0x0287F0, i_0287f0),
    (0x0287F2, i_0287f2),
    (0x0287F6, i_0287f6),
    (0x0287FA, i_0287fa),
    (0x028800, i_028800),
    (0x028804, i_028804),
    (0x028806, i_028806),
    (0x028808, i_028808),
    (0x02880A, i_02880a),
    (0x02880C, i_02880c),
    (0x028810, i_028810),
    (0x028812, i_028812),
    (0x028814, i_028814),
    (0x028816, i_028816),
    (0x028818, i_028818),
    (0x02881A, i_02881a),
    (0x02881E, i_02881e),
    (0x028822, i_028822),
    (0x028828, i_028828),
    (0x02882C, i_02882c),
    (0x02882E, i_02882e),
    (0x028830, i_028830),
    (0x028832, i_028832),
    (0x028834, i_028834),
    (0x028838, i_028838),
    (0x02883A, i_02883a),
    (0x02883C, i_02883c),
    (0x02883E, i_02883e),
    (0x028840, i_028840),
    (0x028842, i_028842),
    (0x028846, i_028846),
    (0x02884A, i_02884a),
    (0x02884E, i_02884e),
    (0x028852, i_028852),
    (0x028854, i_028854),
    (0x028858, i_028858),
    (0x02885A, i_02885a),
    (0x02885E, i_02885e),
    (0x028862, i_028862),
    (0x028864, i_028864),
    (0x028866, i_028866),
    (0x02886A, i_02886a),
    (0x02886C, i_02886c),
    (0x02886E, i_02886e),
    (0x028872, i_028872),
    (0x028874, i_028874),
    (0x028876, i_028876),
    (0x028878, i_028878),
    (0x02887C, i_02887c),
    (0x028880, i_028880),
    (0x028884, i_028884),
    (0x028886, i_028886),
    (0x028888, i_028888),
    (0x02888A, i_02888a),
    (0x02888E, i_02888e),
    (0x028894, i_028894),
    (0x028898, i_028898),
    (0x02889A, i_02889a),
    (0x02889C, i_02889c),
    (0x02889E, i_02889e),
    (0x0288A0, i_0288a0),
    (0x0288A4, i_0288a4),
    (0x0288A6, i_0288a6),
    (0x0288A8, i_0288a8),
    (0x0288AA, i_0288aa),
    (0x0288AC, i_0288ac),
    (0x0288AE, i_0288ae),
    (0x0288B2, i_0288b2),
    (0x0288B6, i_0288b6),
    (0x0288BA, i_0288ba),
    (0x0288BC, i_0288bc),
    (0x0288C0, i_0288c0),
    (0x0288C2, i_0288c2),
    (0x0288C6, i_0288c6),
    (0x0288CA, i_0288ca),
    (0x0288CC, i_0288cc),
    (0x0288CE, i_0288ce),
    (0x0288D2, i_0288d2),
    (0x0288D6, i_0288d6),
    (0x0288DA, i_0288da),
    (0x0288E0, i_0288e0),
    (0x0288E4, i_0288e4),
    (0x0288E6, i_0288e6),
    (0x0288E8, i_0288e8),
    (0x0288EA, i_0288ea),
    (0x0288EC, i_0288ec),
    (0x0288F0, i_0288f0),
    (0x0288F2, i_0288f2),
    (0x0288F4, i_0288f4),
    (0x0288F6, i_0288f6),
    (0x0288F8, i_0288f8),
    (0x0288FA, i_0288fa),
    (0x0288FE, i_0288fe),
    (0x028902, i_028902),
    (0x028906, i_028906),
    (0x028908, i_028908),
    (0x02890C, i_02890c),
    (0x02890E, i_02890e),
    (0x028912, i_028912),
    (0x028916, i_028916),
    (0x028918, i_028918),
    (0x02891C, i_02891c),
    (0x028920, i_028920),
    (0x028924, i_028924),
    (0x028926, i_028926),
    (0x02892A, i_02892a),
    (0x02892E, i_02892e),
    (0x028930, i_028930),
    (0x028936, i_028936),
    (0x02893A, i_02893a),
    (0x028940, i_028940),
    (0x028944, i_028944),
    (0x028948, i_028948),
    (0x02894C, i_02894c),
    (0x028950, i_028950),
    (0x028954, i_028954),
    (0x028958, i_028958),
    (0x02895A, i_02895a),
    (0x02895E, i_02895e),
    (0x028962, i_028962),
    (0x028966, i_028966),
    (0x02896A, i_02896a),
    (0x02896E, i_02896e),
    (0x028972, i_028972),
    (0x028976, i_028976),
    (0x02897A, i_02897a),
    (0x02897E, i_02897e),
    (0x028982, i_028982),
    (0x028986, i_028986),
    (0x02898A, i_02898a),
    (0x02898C, i_02898c),
    (0x028990, i_028990),
    (0x028992, i_028992),
    (0x028996, i_028996),
    (0x028998, i_028998),
    (0x02899E, i_02899e),
    (0x0289A2, i_0289a2),
    (0x0289A6, i_0289a6),
    (0x0289AA, i_0289aa),
    (0x0289AE, i_0289ae),
    (0x0289B4, i_0289b4),
    (0x0289B8, i_0289b8),
    (0x0289BE, i_0289be),
    (0x0289C2, i_0289c2),
    (0x0289C6, i_0289c6),
    (0x0289CC, i_0289cc),
    (0x0289D0, i_0289d0),
    (0x0289D4, i_0289d4),
    (0x0289D8, i_0289d8),
    (0x0289DC, i_0289dc),
    (0x0289E2, i_0289e2),
    (0x0289E6, i_0289e6),
    (0x0289EA, i_0289ea),
    (0x0289EE, i_0289ee),
    (0x0289F0, i_0289f0),
    (0x0289F4, i_0289f4),
    (0x0289F6, i_0289f6),
    (0x0289F8, i_0289f8),
    (0x0289FC, i_0289fc),
    (0x028A96, i_028a96),
    (0x028A98, i_028a98),
    (0x028A9C, i_028a9c),
    (0x028AA0, i_028aa0),
    (0x028AA6, i_028aa6),
    (0x028AAA, i_028aaa),
    (0x028AAE, i_028aae),
    (0x028AB2, i_028ab2),
    (0x028AB8, i_028ab8),
    (0x028ABC, i_028abc),
    (0x028ABE, i_028abe),
    (0x028AC4, i_028ac4),
    (0x028AC6, i_028ac6),
    (0x028ACC, i_028acc),
    (0x028AD0, i_028ad0),
    (0x028AD4, i_028ad4),
    (0x028AD8, i_028ad8),
    (0x028ADA, i_028ada),
    (0x028AE0, i_028ae0),
    (0x028AE4, i_028ae4),
    (0x028AE8, i_028ae8),
    (0x028AEA, i_028aea),
    (0x028AEE, i_028aee),
    (0x028AF4, i_028af4),
    (0x028AFA, i_028afa),
    (0x028AFE, i_028afe),
    (0x028B04, i_028b04),
    (0x028B08, i_028b08),
    (0x028B0C, i_028b0c),
    (0x028B0E, i_028b0e),
    (0x028B12, i_028b12),
    (0x028B14, i_028b14),
    (0x028B16, i_028b16),
    (0x028B1A, i_028b1a),
    (0x028B20, i_028b20),
    (0x028B24, i_028b24),
    (0x028B26, i_028b26),
    (0x028B28, i_028b28),
    (0x028B2A, i_028b2a),
    (0x028B2C, i_028b2c),
    (0x028B30, i_028b30),
    (0x028B32, i_028b32),
    (0x028B34, i_028b34),
    (0x028B36, i_028b36),
    (0x028B38, i_028b38),
    (0x028B3A, i_028b3a),
    (0x028B3C, i_028b3c),
    (0x028B40, i_028b40),
    (0x028B46, i_028b46),
    (0x028B4A, i_028b4a),
    (0x028B4C, i_028b4c),
    (0x028B4E, i_028b4e),
    (0x028B50, i_028b50),
    (0x028B52, i_028b52),
    (0x028B56, i_028b56),
    (0x028B58, i_028b58),
    (0x028B5A, i_028b5a),
    (0x028B5C, i_028b5c),
    (0x028B5E, i_028b5e),
    (0x028B60, i_028b60),
    (0x028B64, i_028b64),
    (0x028B68, i_028b68),
    (0x028B6C, i_028b6c),
    (0x028B70, i_028b70),
    (0x028B72, i_028b72),
    (0x028B76, i_028b76),
    (0x028B78, i_028b78),
    (0x028B7C, i_028b7c),
    (0x028B80, i_028b80),
    (0x028B82, i_028b82),
    (0x028B84, i_028b84),
    (0x028B88, i_028b88),
    (0x028B8A, i_028b8a),
    (0x028B8C, i_028b8c),
    (0x028B90, i_028b90),
    (0x028B92, i_028b92),
    (0x028B96, i_028b96),
    (0x028B9A, i_028b9a),
    (0x028B9E, i_028b9e),
    (0x028BA2, i_028ba2),
    (0x028BA4, i_028ba4),
    (0x028BA6, i_028ba6),
    (0x028BA8, i_028ba8),
    (0x028BAC, i_028bac),
    (0x028BB0, i_028bb0),
    (0x028BB6, i_028bb6),
    (0x028BBA, i_028bba),
    (0x028BBC, i_028bbc),
    (0x028BBE, i_028bbe),
    (0x028BC0, i_028bc0),
    (0x028BC2, i_028bc2),
    (0x028BC6, i_028bc6),
    (0x028BC8, i_028bc8),
    (0x028BCA, i_028bca),
    (0x028BCC, i_028bcc),
    (0x028BCE, i_028bce),
    (0x028BD0, i_028bd0),
    (0x028BD4, i_028bd4),
    (0x028BD8, i_028bd8),
    (0x028BDE, i_028bde),
    (0x028BE2, i_028be2),
    (0x028BE4, i_028be4),
    (0x028BE6, i_028be6),
    (0x028BE8, i_028be8),
    (0x028BEA, i_028bea),
    (0x028BEE, i_028bee),
    (0x028BF0, i_028bf0),
    (0x028BF2, i_028bf2),
    (0x028BF4, i_028bf4),
    (0x028BF6, i_028bf6),
    (0x028BF8, i_028bf8),
    (0x028BFC, i_028bfc),
    (0x028C00, i_028c00),
    (0x028C04, i_028c04),
    (0x028C08, i_028c08),
    (0x028C0A, i_028c0a),
    (0x028C0E, i_028c0e),
    (0x028C10, i_028c10),
    (0x028C14, i_028c14),
    (0x028C18, i_028c18),
    (0x028C1A, i_028c1a),
    (0x028C1E, i_028c1e),
    (0x028C20, i_028c20),
    (0x028C22, i_028c22),
    (0x028C24, i_028c24),
    (0x028C28, i_028c28),
    (0x028C2E, i_028c2e),
    (0x028C32, i_028c32),
    (0x028C34, i_028c34),
    (0x028C36, i_028c36),
    (0x028C38, i_028c38),
    (0x028C3A, i_028c3a),
    (0x028C3E, i_028c3e),
    (0x028C40, i_028c40),
    (0x028C42, i_028c42),
    (0x028C44, i_028c44),
    (0x028C46, i_028c46),
    (0x028C48, i_028c48),
    (0x028C4C, i_028c4c),
    (0x028C50, i_028c50),
    (0x028C54, i_028c54),
    (0x028C56, i_028c56),
    (0x028C5A, i_028c5a),
    (0x028C5C, i_028c5c),
    (0x028C60, i_028c60),
    (0x028C64, i_028c64),
    (0x028C66, i_028c66),
    (0x028C68, i_028c68),
    (0x028C6C, i_028c6c),
    (0x028C70, i_028c70),
    (0x028C74, i_028c74),
    (0x028C7A, i_028c7a),
    (0x028C7E, i_028c7e),
    (0x028C80, i_028c80),
    (0x028C82, i_028c82),
    (0x028C84, i_028c84),
    (0x028C86, i_028c86),
    (0x028C8A, i_028c8a),
    (0x028C8C, i_028c8c),
    (0x028C8E, i_028c8e),
    (0x028C90, i_028c90),
    (0x028C92, i_028c92),
    (0x028C94, i_028c94),
    (0x028C98, i_028c98),
    (0x028C9C, i_028c9c),
    (0x028CA0, i_028ca0),
    (0x028CA2, i_028ca2),
    (0x028CA6, i_028ca6),
    (0x028CA8, i_028ca8),
    (0x028CAC, i_028cac),
    (0x028CB0, i_028cb0),
    (0x028CB2, i_028cb2),
    (0x028CB6, i_028cb6),
    (0x028CB8, i_028cb8),
    (0x028CBC, i_028cbc),
    (0x028CC0, i_028cc0),
    (0x028CC4, i_028cc4),
    (0x028CC8, i_028cc8),
    (0x028CCC, i_028ccc),
    (0x028CCE, i_028cce),
    (0x028CD2, i_028cd2),
    (0x028CD4, i_028cd4),
    (0x028CD8, i_028cd8),
    (0x028CDC, i_028cdc),
    (0x028CE0, i_028ce0),
    (0x028CE4, i_028ce4),
    (0x028CE8, i_028ce8),
    (0x028CEE, i_028cee),
    (0x028D70, i_028d70),
    (0x028D74, i_028d74),
    (0x028D7A, i_028d7a),
    (0x028D7E, i_028d7e),
    (0x028D82, i_028d82),
    (0x028D88, i_028d88),
    (0x028D8C, i_028d8c),
    (0x028D8E, i_028d8e),
    (0x028D92, i_028d92),
    (0x028D96, i_028d96),
    (0x028D9A, i_028d9a),
    (0x028D9E, i_028d9e),
    (0x028DA0, i_028da0),
    (0x028DA4, i_028da4),
    (0x028DA8, i_028da8),
    (0x028DAA, i_028daa),
    (0x028DAC, i_028dac),
    (0x028DB0, i_028db0),
    (0x028DB4, i_028db4),
    (0x028DB6, i_028db6),
    (0x028DB8, i_028db8),
    (0x028DBC, i_028dbc),
    (0x028DBE, i_028dbe),
    (0x028DC0, i_028dc0),
    (0x028DC6, i_028dc6),
    (0x028DC8, i_028dc8),
    (0x028DCE, i_028dce),
    (0x028DD2, i_028dd2),
    (0x028DD4, i_028dd4),
    (0x028DD8, i_028dd8),
    (0x028DDC, i_028ddc),
    (0x028DE0, i_028de0),
    (0x028DE4, i_028de4),
    (0x028DEA, i_028dea),
    (0x028DEE, i_028dee),
    (0x028DF2, i_028df2),
    (0x028DF6, i_028df6),
    (0x028DFA, i_028dfa),
    (0x028DFE, i_028dfe),
    (0x028E02, i_028e02),
    (0x028E06, i_028e06),
    (0x028E0A, i_028e0a),
    (0x028E0C, i_028e0c),
    (0x028E12, i_028e12),
    (0x028E16, i_028e16),
    (0x028E1A, i_028e1a),
    (0x028E1E, i_028e1e),
    (0x028E22, i_028e22),
    (0x028E26, i_028e26),
    (0x028E2A, i_028e2a),
    (0x028E2E, i_028e2e),
    (0x028E30, i_028e30),
    (0x028E34, i_028e34),
    (0x028E38, i_028e38),
    (0x028E3C, i_028e3c),
    (0x028E40, i_028e40),
    (0x028E42, i_028e42),
    (0x028E46, i_028e46),
    (0x028E48, i_028e48),
    (0x028E4C, i_028e4c),
    (0x028E4E, i_028e4e),
    (0x028E52, i_028e52),
    (0x028E56, i_028e56),
    (0x028E5A, i_028e5a),
    (0x028E60, i_028e60),
    (0x028E64, i_028e64),
    (0x028E6A, i_028e6a),
    (0x028E6E, i_028e6e),
    (0x028E74, i_028e74),
    (0x028E78, i_028e78),
    (0x028E7E, i_028e7e),
    (0x028E82, i_028e82),
    (0x028E8A, i_028e8a),
    (0x028E8E, i_028e8e),
    (0x028E96, i_028e96),
    (0x028E9C, i_028e9c),
    (0x028EA2, i_028ea2),
    (0x028EA6, i_028ea6),
    (0x028EAC, i_028eac),
    (0x028EB0, i_028eb0),
    (0x028EB4, i_028eb4),
    (0x028EB6, i_028eb6),
    (0x028EBA, i_028eba),
    (0x028EBE, i_028ebe),
    (0x028EC0, i_028ec0),
    (0x028EC2, i_028ec2),
    (0x028EC6, i_028ec6),
    (0x028ECC, i_028ecc),
    (0x028ED0, i_028ed0),
    (0x028ED6, i_028ed6),
    (0x028EDA, i_028eda),
    (0x028EDE, i_028ede),
    (0x028EE4, i_028ee4),
    (0x028EE8, i_028ee8),
    (0x028EEC, i_028eec),
    (0x028EF0, i_028ef0),
    (0x028EF6, i_028ef6),
    (0x028EFA, i_028efa),
    (0x028F00, i_028f00),
    (0x028F04, i_028f04),
    (0x028F0A, i_028f0a),
    (0x028F0E, i_028f0e),
    (0x028F14, i_028f14),
    (0x028F18, i_028f18),
    (0x028F1C, i_028f1c),
    (0x028F22, i_028f22),
    (0x028F2A, i_028f2a),
    (0x028F2E, i_028f2e),
    (0x028F34, i_028f34),
    (0x028F38, i_028f38),
    (0x028F3E, i_028f3e),
    (0x028F42, i_028f42),
    (0x028F46, i_028f46),
    (0x028F4A, i_028f4a),
    (0x028F4C, i_028f4c),
    (0x028F50, i_028f50),
    (0x028F54, i_028f54),
    (0x028F56, i_028f56),
    (0x028F5A, i_028f5a),
    (0x028F5E, i_028f5e),
    (0x028F62, i_028f62),
    (0x028F66, i_028f66),
    (0x028F6A, i_028f6a),
    (0x028F6C, i_028f6c),
    (0x028F6E, i_028f6e),
    (0x028F72, i_028f72),
    (0x028F76, i_028f76),
    (0x028F7A, i_028f7a),
    (0x028F7E, i_028f7e),
    (0x028F82, i_028f82),
    (0x028F86, i_028f86),
    (0x028F8A, i_028f8a),
    (0x028F8E, i_028f8e),
    (0x028F92, i_028f92),
    (0x028F96, i_028f96),
    (0x028F9A, i_028f9a),
    (0x028F9E, i_028f9e),
    (0x028FA2, i_028fa2),
    (0x028FA8, i_028fa8),
    (0x028FAC, i_028fac),
    (0x028FB0, i_028fb0),
    (0x028FB6, i_028fb6),
    (0x028FBA, i_028fba),
    (0x028FC0, i_028fc0),
    (0x028FC4, i_028fc4),
    (0x028FCA, i_028fca),
    (0x028FCE, i_028fce),
    (0x028FD4, i_028fd4),
    (0x028FD8, i_028fd8),
    (0x028FDC, i_028fdc),
    (0x028FE2, i_028fe2),
    (0x028FE6, i_028fe6),
    (0x028FEA, i_028fea),
    (0x028FEE, i_028fee),
    (0x028FF2, i_028ff2),
    (0x028FF6, i_028ff6),
    (0x028FFA, i_028ffa),
    (0x028FFE, i_028ffe),
    (0x029002, i_029002),
    (0x029006, i_029006),
    (0x02900A, i_02900a),
    (0x029010, i_029010),
    (0x029014, i_029014),
    (0x029018, i_029018),
    (0x02901C, i_02901c),
    (0x029020, i_029020),
    (0x029024, i_029024),
    (0x029026, i_029026),
    (0x02902A, i_02902a),
    (0x02902C, i_02902c),
    (0x029030, i_029030),
    (0x029034, i_029034),
    (0x029038, i_029038),
    (0x02903A, i_02903a),
    (0x02903E, i_02903e),
    (0x029042, i_029042),
    (0x029044, i_029044),
    (0x029048, i_029048),
    (0x02904A, i_02904a),
    (0x02904E, i_02904e),
    (0x029050, i_029050),
    (0x029054, i_029054),
    (0x029058, i_029058),
    (0x02905C, i_02905c),
    (0x029060, i_029060),
    (0x029066, i_029066),
    (0x02906A, i_02906a),
    (0x02906E, i_02906e),
    (0x029072, i_029072),
    (0x029078, i_029078),
    (0x02907E, i_02907e),
    (0x029082, i_029082),
    (0x02909E, i_02909e),
    (0x0290A2, i_0290a2),
    (0x0290A6, i_0290a6),
    (0x0290AA, i_0290aa),
    (0x0290AC, i_0290ac),
    (0x0290AE, i_0290ae),
    (0x0290B2, i_0290b2),
    (0x0290B4, i_0290b4),
    (0x0290B6, i_0290b6),
    (0x0290BC, i_0290bc),
    (0x0290C2, i_0290c2),
    (0x0290C6, i_0290c6),
    (0x0290CA, i_0290ca),
    (0x0290CE, i_0290ce),
    (0x0290D2, i_0290d2),
    (0x0290D6, i_0290d6),
    (0x0290D8, i_0290d8),
    (0x0290DA, i_0290da),
    (0x0290DE, i_0290de),
    (0x0290E0, i_0290e0),
    (0x0290E2, i_0290e2),
    (0x0290E8, i_0290e8),
    (0x0290EE, i_0290ee),
    (0x0290F2, i_0290f2),
    (0x0290F6, i_0290f6),
    (0x0290FA, i_0290fa),
    (0x0290FE, i_0290fe),
    (0x029102, i_029102),
    (0x029104, i_029104),
    (0x029106, i_029106),
    (0x02910A, i_02910a),
    (0x02910C, i_02910c),
    (0x02910E, i_02910e),
    (0x029114, i_029114),
    (0x02911A, i_02911a),
    (0x02911E, i_02911e),
    (0x029122, i_029122),
    (0x029126, i_029126),
    (0x02912A, i_02912a),
    (0x02912C, i_02912c),
    (0x029130, i_029130),
    (0x029136, i_029136),
    (0x02913A, i_02913a),
    (0x02913E, i_02913e),
    (0x029144, i_029144),
    (0x02914A, i_02914a),
    (0x02914E, i_02914e),
    (0x029152, i_029152),
    (0x029158, i_029158),
    (0x02915E, i_02915e),
    (0x029162, i_029162),
    (0x029196, i_029196),
    (0x02919A, i_02919a),
    (0x02919E, i_02919e),
    (0x0291A2, i_0291a2),
    (0x0291A4, i_0291a4),
    (0x0291A6, i_0291a6),
    (0x0291AA, i_0291aa),
    (0x0291AC, i_0291ac),
    (0x0291AE, i_0291ae),
    (0x0291B2, i_0291b2),
    (0x0291B6, i_0291b6),
    (0x0291BC, i_0291bc),
    (0x0291C0, i_0291c0),
    (0x029220, i_029220),
    (0x029224, i_029224),
    (0x029228, i_029228),
    (0x02922C, i_02922c),
    (0x02922E, i_02922e),
    (0x029230, i_029230),
    (0x029234, i_029234),
    (0x029236, i_029236),
    (0x029238, i_029238),
    (0x02923C, i_02923c),
    (0x029240, i_029240),
    (0x029246, i_029246),
    (0x02924A, i_02924a),
    (0x0292AA, i_0292aa),
    (0x0292AE, i_0292ae),
    (0x0292B2, i_0292b2),
    (0x0292B6, i_0292b6),
    (0x0292B8, i_0292b8),
    (0x0292BA, i_0292ba),
    (0x0292BE, i_0292be),
    (0x0292C0, i_0292c0),
    (0x0292C2, i_0292c2),
    (0x0292C6, i_0292c6),
    (0x0292CA, i_0292ca),
    (0x0292D0, i_0292d0),
    (0x0292D4, i_0292d4),
    (0x029334, i_029334),
    (0x029338, i_029338),
    (0x02933C, i_02933c),
    (0x029342, i_029342),
    (0x029348, i_029348),
    (0x02934C, i_02934c),
    (0x029352, i_029352),
    (0x029358, i_029358),
    (0x02935C, i_02935c),
    (0x029360, i_029360),
    (0x029368, i_029368),
    (0x02936E, i_02936e),
    (0x029372, i_029372),
    (0x029376, i_029376),
    (0x02937A, i_02937a),
    (0x02937E, i_02937e),
    (0x029382, i_029382),
    (0x029386, i_029386),
    (0x02938A, i_02938a),
    (0x02938E, i_02938e),
    (0x029392, i_029392),
    (0x029396, i_029396),
    (0x02939A, i_02939a),
    (0x0293A0, i_0293a0),
    (0x0293A4, i_0293a4),
    (0x0293A8, i_0293a8),
    (0x0293AA, i_0293aa),
    (0x0293AE, i_0293ae),
    (0x0293B2, i_0293b2),
    (0x0293B6, i_0293b6),
    (0x0293BC, i_0293bc),
    (0x0293BE, i_0293be),
    (0x0293C2, i_0293c2),
    (0x029424, i_029424),
    (0x029428, i_029428),
    (0x02942A, i_02942a),
    (0x02942E, i_02942e),
    (0x029436, i_029436),
    (0x02943A, i_02943a),
    (0x029442, i_029442),
    (0x029446, i_029446),
    (0x02944A, i_02944a),
    (0x02944E, i_02944e),
    (0x029454, i_029454),
    (0x029458, i_029458),
    (0x02945C, i_02945c),
    (0x029462, i_029462),
    (0x029468, i_029468),
    (0x02946C, i_02946c),
    (0x029470, i_029470),
    (0x029472, i_029472),
    (0x029476, i_029476),
    (0x02947A, i_02947a),
    (0x02947E, i_02947e),
    (0x029482, i_029482),
    (0x029484, i_029484),
    (0x029486, i_029486),
    (0x02948A, i_02948a),
    (0x02948C, i_02948c),
    (0x02948E, i_02948e),
    (0x029492, i_029492),
    (0x029498, i_029498),
    (0x02949C, i_02949c),
    (0x0294A0, i_0294a0),
    (0x0294A6, i_0294a6),
    (0x0294AA, i_0294aa),
    (0x0294AE, i_0294ae),
    (0x0294B0, i_0294b0),
    (0x0294B4, i_0294b4),
    (0x0294B8, i_0294b8),
    (0x0294BC, i_0294bc),
    (0x0294C8, i_0294c8),
    (0x0294CA, i_0294ca),
    (0x0294CE, i_0294ce),
    (0x0294D0, i_0294d0),
    (0x0294D4, i_0294d4),
    (0x0294D8, i_0294d8),
    (0x0294DC, i_0294dc),
    (0x0294E0, i_0294e0),
    (0x0294E4, i_0294e4),
    (0x0294E6, i_0294e6),
    (0x0294EA, i_0294ea),
    (0x0294EE, i_0294ee),
    (0x0294F2, i_0294f2),
    (0x0294F8, i_0294f8),
    (0x0294FC, i_0294fc),
    (0x029502, i_029502),
    (0x029506, i_029506),
    (0x02950C, i_02950c),
    (0x029510, i_029510),
    (0x029516, i_029516),
    (0x02951A, i_02951a),
    (0x02951E, i_02951e),
    (0x029524, i_029524),
    (0x029528, i_029528),
    (0x02952C, i_02952c),
    (0x029530, i_029530),
    (0x029534, i_029534),
    (0x029538, i_029538),
    (0x02953C, i_02953c),
    (0x029540, i_029540),
    (0x029544, i_029544),
    (0x02954A, i_02954a),
    (0x02954E, i_02954e),
    (0x029552, i_029552),
    (0x029556, i_029556),
    (0x02955A, i_02955a),
    (0x02955E, i_02955e),
    (0x029560, i_029560),
    (0x029564, i_029564),
    (0x029566, i_029566),
    (0x02956A, i_02956a),
    (0x02956E, i_02956e),
    (0x029572, i_029572),
    (0x029576, i_029576),
    (0x02957A, i_02957a),
    (0x02957C, i_02957c),
    (0x029580, i_029580),
    (0x029582, i_029582),
    (0x029586, i_029586),
    (0x029588, i_029588),
    (0x029590, i_029590),
    (0x029598, i_029598),
    (0x02959E, i_02959e),
    (0x0295A6, i_0295a6),
    (0x0295AC, i_0295ac),
    (0x0295B0, i_0295b0),
    (0x0295B2, i_0295b2),
    (0x0295B4, i_0295b4),
    (0x0295B6, i_0295b6),
    (0x0295BA, i_0295ba),
    (0x029600, i_029600),
    (0x029602, i_029602),
    (0x029606, i_029606),
    (0x02964C, i_02964c),
    (0x02964E, i_02964e),
    (0x029652, i_029652),
    (0x029790, i_029790),
    (0x029792, i_029792),
    (0x029796, i_029796),
    (0x02979A, i_02979a),
    (0x029834, i_029834),
    (0x029836, i_029836),
    (0x02983A, i_02983a),
    (0x02983E, i_02983e),
    (0x0298D8, i_0298d8),
    (0x0298DA, i_0298da),
    (0x0298DE, i_0298de),
    (0x0298E2, i_0298e2),
    (0x02997C, i_02997c),
    (0x02997E, i_02997e),
    (0x029980, i_029980),
    (0x029982, i_029982),
    (0x029984, i_029984),
    (0x029986, i_029986),
    (0x029988, i_029988),
    (0x02998A, i_02998a),
    (0x02998C, i_02998c),
    (0x029990, i_029990),
    (0x029992, i_029992),
    (0x029994, i_029994),
    (0x029998, i_029998),
    (0x02999A, i_02999a),
    (0x02999C, i_02999c),
    (0x02999E, i_02999e),
    (0x0299A2, i_0299a2),
    (0x0299A6, i_0299a6),
    (0x0299AA, i_0299aa),
    (0x0299AE, i_0299ae),
    (0x0299B2, i_0299b2),
    (0x0299B6, i_0299b6),
    (0x0299F2, i_0299f2),
    (0x0299F6, i_0299f6),
    (0x0299F8, i_0299f8),
    (0x0299FC, i_0299fc),
    (0x0299FE, i_0299fe),
    (0x029A02, i_029a02),
    (0x029A06, i_029a06),
    (0x029A0A, i_029a0a),
    (0x029A0E, i_029a0e),
    (0x029A12, i_029a12),
    (0x029A14, i_029a14),
    (0x029A18, i_029a18),
    (0x029A1C, i_029a1c),
    (0x029A20, i_029a20),
    (0x029A24, i_029a24),
    (0x029A2A, i_029a2a),
    (0x029A2E, i_029a2e),
    (0x029A32, i_029a32),
    (0x029A36, i_029a36),
    (0x029A3A, i_029a3a),
    (0x02A0F8, i_02a0f8),
    (0x02A0FE, i_02a0fe),
    (0x02A102, i_02a102),
    (0x02A108, i_02a108),
    (0x02A10E, i_02a10e),
    (0x02A114, i_02a114),
    (0x02A11A, i_02a11a),
    (0x02A120, i_02a120),
    (0x02A126, i_02a126),
    (0x02A12C, i_02a12c),
    (0x02A132, i_02a132),
    (0x02A138, i_02a138),
    (0x02A13E, i_02a13e),
    (0x02A144, i_02a144),
    (0x02A14A, i_02a14a),
    (0x02A150, i_02a150),
    (0x02A154, i_02a154),
    (0x02A15C, i_02a15c),
    (0x02A162, i_02a162),
    (0x02A168, i_02a168),
    (0x02A16C, i_02a16c),
    (0x02A170, i_02a170),
    (0x02A174, i_02a174),
    (0x02A178, i_02a178),
    (0x02A17C, i_02a17c),
    (0x02A182, i_02a182),
    (0x02A188, i_02a188),
    (0x02A18E, i_02a18e),
    (0x02A194, i_02a194),
    (0x02A19A, i_02a19a),
    (0x02A1A0, i_02a1a0),
    (0x02A1A8, i_02a1a8),
    (0x02A1AA, i_02a1aa),
    (0x02A1B0, i_02a1b0),
    (0x02A1B4, i_02a1b4),
    (0x02A1BA, i_02a1ba),
    (0x02A1C0, i_02a1c0),
    (0x02A1C6, i_02a1c6),
    (0x02A1CC, i_02a1cc),
    (0x02A1D2, i_02a1d2),
    (0x02A1D8, i_02a1d8),
    (0x02A1DE, i_02a1de),
    (0x02A1E4, i_02a1e4),
    (0x02A1EA, i_02a1ea),
    (0x02A1F0, i_02a1f0),
    (0x02A1F6, i_02a1f6),
    (0x02A1FC, i_02a1fc),
    (0x02A202, i_02a202),
    (0x02A208, i_02a208),
    (0x02A20C, i_02a20c),
    (0x02A214, i_02a214),
    (0x02A21A, i_02a21a),
    (0x02A21E, i_02a21e),
    (0x02A222, i_02a222),
    (0x02A226, i_02a226),
    (0x02A22A, i_02a22a),
    (0x02A230, i_02a230),
    (0x02A236, i_02a236),
    (0x02A23C, i_02a23c),
    (0x02A242, i_02a242),
    (0x02A248, i_02a248),
    (0x02A24A, i_02a24a),
    (0x02A250, i_02a250),
    (0x02A252, i_02a252),
    (0x02A25A, i_02a25a),
    (0x02A25C, i_02a25c),
    (0x02A264, i_02a264),
    (0x02A268, i_02a268),
    (0x02A26C, i_02a26c),
    (0x02A270, i_02a270),
    (0x02A274, i_02a274),
    (0x02A276, i_02a276),
    (0x02A27C, i_02a27c),
    (0x02A280, i_02a280),
    (0x02A284, i_02a284),
    (0x02A288, i_02a288),
    (0x02A28C, i_02a28c),
    (0x02A28E, i_02a28e),
    (0x02A294, i_02a294),
    (0x02A29A, i_02a29a),
    (0x02A29E, i_02a29e),
    (0x02A2A2, i_02a2a2),
    (0x02A2A6, i_02a2a6),
    (0x02A2AA, i_02a2aa),
    (0x02A2AC, i_02a2ac),
    (0x02A2B2, i_02a2b2),
    (0x02A2B4, i_02a2b4),
    (0x02A2B8, i_02a2b8),
    (0x02A2BA, i_02a2ba),
    (0x02A2C0, i_02a2c0),
    (0x02A2C6, i_02a2c6),
    (0x02A2CA, i_02a2ca),
    (0x02A2CE, i_02a2ce),
    (0x02A2D0, i_02a2d0),
    (0x02A2D4, i_02a2d4),
    (0x02A2D8, i_02a2d8),
    (0x02A2DC, i_02a2dc),
    (0x02A2E0, i_02a2e0),
    (0x02A2E4, i_02a2e4),
    (0x02A2EA, i_02a2ea),
    (0x02A2EE, i_02a2ee),
    (0x02A2F2, i_02a2f2),
    (0x02A2F6, i_02a2f6),
    (0x02A2F8, i_02a2f8),
    (0x02A2FC, i_02a2fc),
    (0x02A300, i_02a300),
    (0x02A304, i_02a304),
    (0x02A30A, i_02a30a),
    (0x02A30E, i_02a30e),
    (0x02A310, i_02a310),
    (0x02A314, i_02a314),
    (0x02A318, i_02a318),
    (0x02A31C, i_02a31c),
    (0x02A322, i_02a322),
    (0x02A326, i_02a326),
    (0x02A328, i_02a328),
    (0x02A32C, i_02a32c),
    (0x02A32E, i_02a32e),
    (0x02A332, i_02a332),
    (0x02A336, i_02a336),
    (0x02A338, i_02a338),
    (0x02A34E, i_02a34e),
    (0x02A352, i_02a352),
    (0x02A354, i_02a354),
    (0x02A358, i_02a358),
    (0x02A35C, i_02a35c),
    (0x02A35E, i_02a35e),
    (0x02A374, i_02a374),
    (0x02A378, i_02a378),
    (0x02A37A, i_02a37a),
    (0x02A37E, i_02a37e),
    (0x02A382, i_02a382),
    (0x02A384, i_02a384),
    (0x02A39A, i_02a39a),
    (0x02A3A0, i_02a3a0),
    (0x02A3A4, i_02a3a4),
    (0x02A3A6, i_02a3a6),
    (0x02A3AC, i_02a3ac),
    (0x02A3B0, i_02a3b0),
    (0x02A3B6, i_02a3b6),
    (0x02A3B8, i_02a3b8),
    (0x02A3BE, i_02a3be),
    (0x02A3C2, i_02a3c2),
    (0x02A3C6, i_02a3c6),
    (0x02A3CA, i_02a3ca),
    (0x02A3CE, i_02a3ce),
    (0x02A3D0, i_02a3d0),
    (0x02A3D4, i_02a3d4),
    (0x02A3D6, i_02a3d6),
    (0x02A3DC, i_02a3dc),
    (0x02A3E0, i_02a3e0),
    (0x02A3E4, i_02a3e4),
    (0x02A3E8, i_02a3e8),
    (0x02A3EC, i_02a3ec),
    (0x02A3EE, i_02a3ee),
    (0x02A3F2, i_02a3f2),
    (0x02A3F4, i_02a3f4),
    (0x02A3FA, i_02a3fa),
    (0x02A3FE, i_02a3fe),
    (0x02A404, i_02a404),
    (0x02A408, i_02a408),
    (0x02A40E, i_02a40e),
    (0x02A412, i_02a412),
    (0x02A416, i_02a416),
    (0x02A418, i_02a418),
    (0x02A41C, i_02a41c),
    (0x02A41E, i_02a41e),
    (0x02A424, i_02a424),
    (0x02A426, i_02a426),
    (0x02A42C, i_02a42c),
    (0x02A42E, i_02a42e),
    (0x02A434, i_02a434),
    (0x02A436, i_02a436),
    (0x02A438, i_02a438),
    (0x02A43E, i_02a43e),
    (0x02A440, i_02a440),
    (0x02A446, i_02a446),
    (0x02A448, i_02a448),
    (0x02A44E, i_02a44e),
    (0x02A450, i_02a450),
    (0x02A452, i_02a452),
    (0x02A458, i_02a458),
    (0x02A45A, i_02a45a),
    (0x02A460, i_02a460),
    (0x02A462, i_02a462),
    (0x02A468, i_02a468),
    (0x02A46A, i_02a46a),
    (0x02A46C, i_02a46c),
    (0x02A472, i_02a472),
    (0x02A476, i_02a476),
    (0x02A478, i_02a478),
    (0x02A47E, i_02a47e),
    (0x02A482, i_02a482),
    (0x02A488, i_02a488),
    (0x02A48C, i_02a48c),
    (0x02A492, i_02a492),
    (0x02A496, i_02a496),
    (0x02A49A, i_02a49a),
    (0x02A4A0, i_02a4a0),
    (0x02A4A2, i_02a4a2),
    (0x02A4A6, i_02a4a6),
    (0x02A4A8, i_02a4a8),
    (0x02A4AE, i_02a4ae),
    (0x02A4B2, i_02a4b2),
];
